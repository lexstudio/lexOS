extern crate syscall;

use core::mem::size_of;

use syscall::{dirent::DirentHeader, RwFlags, EINVAL, SIGKILL};

pub use self::syscall::{
    data, error, flag, io, number, ptrace_event, EnvRegisters, FloatRegisters, IntRegisters,
};

pub use self::{
    driver::*, fs::*, futex::futex, privilege::*, process::*, time::*, usercopy::validate_region,
};

use self::{
    data::{Map, TimeSpec},
    error::{Error, Result, ENOSYS, EOVERFLOW},
    flag::{EventFlags, MapFlags, WaitFlags},
    number::*,
    usercopy::UserSlice,
};

use crate::{interrupt::InterruptStack, percpu::PercpuBlock};

use crate::{
    context::{memory::AddrSpace, process::ProcessId},
    scheme::{memory::MemoryScheme, FileHandle, SchemeNamespace},
};

/// Debug
pub mod debug;

#[cfg(feature = "syscall_debug")]
use self::debug::{debug_end, debug_start};

/// Driver syscalls
pub mod driver;

/// Filesystem syscalls
pub mod fs;

/// Fast userspace mutex
pub mod futex;

/// Privilege syscalls
pub mod privilege;

/// Process syscalls
pub mod process;

/// Time syscalls
pub mod time;

/// Safely copying memory between user and kernel memory
pub mod usercopy;

/// This function is the syscall handler of the kernel, it is composed of an inner function that returns a `Result<usize>`. After the inner function runs, the syscall
/// function calls [`Error::mux`] on it.
pub fn syscall(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    f: usize,
    _stack: &mut InterruptStack,
) -> usize {
    #[inline(always)]
    fn inner(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> Result<usize> {
        let fd = FileHandle::from(b);
        //SYS_* is declared in kernel/syscall/src/number.rs
        match a {
            SYS_WRITE2 => file_op_generic_ext(fd, |scheme, _, desc| {
                let flags = if f == usize::MAX {
                    None
                } else {
                    Some(
                        u32::try_from(f)
                            .ok()
                            .and_then(RwFlags::from_bits)
                            .ok_or(Error::new(EINVAL))?,
                    )
                };
                scheme.kwriteoff(
                    desc.number,
                    UserSlice::ro(c, d)?,
                    e as u64,
                    flags.map_or(desc.flags, |f| desc.rw_flags(f)),
                    desc.flags,
                )
            }),
            SYS_WRITE => sys_write(fd, UserSlice::ro(c, d)?),
            SYS_FMAP => {
                let addrspace = AddrSpace::current()?;
                let map = unsafe { UserSlice::ro(c, d)?.read_exact::<Map>()? };
                if b == !0 {
                    MemoryScheme::fmap_anonymous(&addrspace, &map, false)
                } else {
                    file_op_generic(fd, |scheme, number| {
                        scheme.kfmap(number, &addrspace, &map, false)
                    })
                }
            }
            SYS_GETDENTS => {
                let header_size = u16::try_from(e).map_err(|_| Error::new(EINVAL))?;

                if usize::from(header_size) != size_of::<DirentHeader>() {
                    // TODO: allow? If so, zero_out must be implemented for UserSlice
                    return Err(Error::new(EINVAL));
                }

                file_op_generic(fd, |scheme, number| {
                    scheme.getdents(number, UserSlice::wo(c, d)?, header_size, f as u64)
                })
            }
            SYS_FUTIMENS => file_op_generic(fd, |scheme, number| {
                scheme.kfutimens(number, UserSlice::ro(c, d)?)
            }),

            SYS_READ2 => file_op_generic_ext(fd, |scheme, _, desc| {
                let flags = if f == usize::MAX {
                    None
                } else {
                    Some(
                        u32::try_from(f)
                            .ok()
                            .and_then(RwFlags::from_bits)
                            .ok_or(Error::new(EINVAL))?,
                    )
                };
                scheme.kreadoff(
                    desc.number,
                    UserSlice::wo(c, d)?,
                    e as u64,
                    flags.map_or(desc.flags, |f| desc.rw_flags(f)),
                    desc.flags,
                )
            }),
            SYS_READ => sys_read(fd, UserSlice::wo(c, d)?),
            SYS_FPATH => file_op_generic(fd, |scheme, number| {
                scheme.kfpath(number, UserSlice::wo(c, d)?)
            }),
            SYS_FSTAT => fstat(fd, UserSlice::wo(c, d)?).map(|()| 0),
            SYS_FSTATVFS => file_op_generic(fd, |scheme, number| {
                scheme.kfstatvfs(number, UserSlice::wo(c, d)?).map(|()| 0)
            }),

            SYS_DUP => dup(fd, UserSlice::ro(c, d)?).map(FileHandle::into),
            SYS_DUP2 => dup2(fd, FileHandle::from(c), UserSlice::ro(d, e)?).map(FileHandle::into),

            #[cfg(target_pointer_width = "32")]
            SYS_SENDFD => sendfd(fd, FileHandle::from(c), d, e as u64 | ((f as u64) << 32)),

            #[cfg(target_pointer_width = "64")]
            SYS_SENDFD => sendfd(fd, FileHandle::from(c), d, e as u64),

            SYS_LSEEK => lseek(fd, c as i64, d),
            SYS_FCHMOD => file_op_generic(fd, |scheme, number| {
                scheme.fchmod(number, c as u16).map(|()| 0)
            }),
            SYS_FCHOWN => file_op_generic(fd, |scheme, number| {
                scheme.fchown(number, c as u32, d as u32).map(|()| 0)
            }),
            SYS_FCNTL => fcntl(fd, c, d),
            SYS_FEVENT => file_op_generic(fd, |scheme, number| {
                Ok(scheme
                    .fevent(number, EventFlags::from_bits_truncate(c))?
                    .bits())
            }),
            SYS_FRENAME => frename(fd, UserSlice::ro(c, d)?).map(|()| 0),
            SYS_FUNMAP => funmap(b, c),

            SYS_FSYNC => file_op_generic(fd, |scheme, number| scheme.fsync(number).map(|()| 0)),
            // TODO: 64-bit lengths on 32-bit platforms
            SYS_FTRUNCATE => {
                file_op_generic(fd, |scheme, number| scheme.ftruncate(number, c).map(|()| 0))
            }

            SYS_CLOSE => close(fd).map(|()| 0),

            SYS_OPEN => open(UserSlice::ro(b, c)?, d).map(FileHandle::into),
            SYS_RMDIR => rmdir(UserSlice::ro(b, c)?).map(|()| 0),
            SYS_UNLINK => unlink(UserSlice::ro(b, c)?).map(|()| 0),
            SYS_YIELD => sched_yield().map(|()| 0),
            SYS_NANOSLEEP => nanosleep(
                UserSlice::ro(b, core::mem::size_of::<TimeSpec>())?,
                UserSlice::wo(c, core::mem::size_of::<TimeSpec>())?.none_if_null(),
            )
            .map(|()| 0),
            SYS_CLOCK_GETTIME => {
                clock_gettime(b, UserSlice::wo(c, core::mem::size_of::<TimeSpec>())?).map(|()| 0)
            }
            SYS_FUTEX => futex(b, c, d, e, f),
            SYS_GETPID => getpid().map(ProcessId::into),
            SYS_GETPGID => getpgid(ProcessId::from(b)).map(ProcessId::into),
            SYS_GETPPID => getppid().map(ProcessId::into),

            SYS_EXIT => exit(b),
            SYS_KILL => kill(ProcessId::from(b), c),
            SYS_WAITPID => waitpid(
                ProcessId::from(b),
                if c == 0 {
                    None
                } else {
                    Some(UserSlice::wo(c, core::mem::size_of::<usize>())?)
                },
                WaitFlags::from_bits_truncate(d),
            )
            .map(ProcessId::into),
            SYS_IOPL => iopl(b),
            SYS_GETEGID => getegid(),
            SYS_GETENS => getens(),
            SYS_GETEUID => geteuid(),
            SYS_GETGID => getgid(),
            SYS_GETNS => getns(),
            SYS_GETUID => getuid(),
            SYS_MPROTECT => mprotect(b, c, MapFlags::from_bits_truncate(d)).map(|()| 0),
            SYS_MKNS => mkns(UserSlice::ro(
                b,
                c.checked_mul(core::mem::size_of::<[usize; 2]>())
                    .ok_or(Error::new(EOVERFLOW))?,
            )?),
            SYS_SETPGID => setpgid(ProcessId::from(b), ProcessId::from(c)).map(|()| 0),
            SYS_SETREUID => setreuid(b as u32, c as u32).map(|()| 0),
            SYS_SETRENS => setrens(SchemeNamespace::from(b), SchemeNamespace::from(c)).map(|()| 0),
            SYS_SETREGID => setregid(b as u32, c as u32).map(|()| 0),
            SYS_UMASK => umask(b),
            SYS_VIRTTOPHYS => virttophys(b),

            SYS_MREMAP => mremap(b, c, d, e, f),

            _ => return Err(Error::new(ENOSYS)),
        }
    }

    PercpuBlock::current().inside_syscall.set(true);

    #[cfg(feature = "syscall_debug")]
    debug_start([a, b, c, d, e, f]);

    let result = inner(a, b, c, d, e, f);

    #[cfg(feature = "syscall_debug")]
    debug_end([a, b, c, d, e, f], result);

    let percpu = PercpuBlock::current();
    percpu.inside_syscall.set(false);

    if percpu.switch_internals.being_sigkilled.get() {
        exit(SIGKILL);
    }

    // errormux turns Result<usize> into -errno
    Error::mux(result)
}
use alloc::vec::Vec;

use crate::{
    context::process,
    scheme::{self, SchemeNamespace},
    syscall::error::*,
};

use super::{
    copy_path_to_buf,
    usercopy::{UserSlice, UserSliceRo},
};

pub fn getegid() -> Result<usize> {
    Ok(process::current()?.read().egid as usize)
}

pub fn getens() -> Result<usize> {
    Ok(process::current()?.read().ens.into())
}

pub fn geteuid() -> Result<usize> {
    Ok(process::current()?.read().euid as usize)
}

pub fn getgid() -> Result<usize> {
    Ok(process::current()?.read().rgid as usize)
}

pub fn getens() -> Result<isize> {
    Ok(process::current()?.read().rns.into())
}
pub fn getegid() -> Result<isize> {
    Ok(process::current()?.read().egid as isize)
}
pub fn getuid() -> Result<isize> {
    Ok(process::current()?.read().ruid as isize)
}

pub fn mkns(mut user_buf: UserSliceRo) -> Result<usize> {
    let (uid, from) = match process::current()?.read() {
        ref process => (process.euid, process.ens),
    };

    // TODO: Lift restir in next update
    if uid != 0 {
        return Err(Error::new(EACCES));
    }

    let mut names = Vec::with_capacity(user_buf.len() / core::mem::size_of::<[usize; 2]>());

    while let Some((current_name_ptr_buf, next_part)) =
        user_buf.split_at(core::mem::size_of::<[usize; 2]>())
    {
        let mut iter = current_name_ptr_buf.usizes();
        let ptr = iter.next().ok_or(Error::new(EINVAL))??;
        let len = iter.next().ok_or(Error::new(EINVAL))??;

        let raw_path = UserSlice::new(ptr, len)?;

        // TODO: Max scheme size limit?
        let max_len = 256;

        names.push(copy_path_to_buf(raw_path, max_len)?.into_boxed_str());

        user_buf = next_part;
    }

    let to = scheme::schemes_mut().make_ns(from, names)?;
    Ok(to.into())
}

pub fn setregid(rgid: u32, egid: u32) -> Result<()> {
    let process_lock = process::current()?;
    let mut process = process_lock.write();

    let setrgid = if process.euid == 0 {
        // Allow changing RGID if root
        true
    } else if rgid == process.egid {
        // Allow changing RGID if used for EGID
        true
    } else if rgid == process.rgid {
        // Allow changing RGID if used for RGID
        true
    } else if rgid as i32 == -1 {
        // Ignore RGID if -1 is passed
        false
    } else {
        // Not permitted otherwise
        return Err(Error::new(EPERM));
    };

    let setegid = if process.euid == 0 {
        // Allow changing EGID if root
        true
    } else if egid == process.egid {
        // Allow changing EGID if used for EGID
        true
    } else if egid == process.rgid {
        // Allow changing EGID if used for RGID
        true
    } else if egid as i32 == -1 {
        // Ignore EGID if -1 is passed
        false
    } else {
        // Not permitted otherwise
        return Err(Error::new(EPERM));
    };

    if setrgid {
        process.rgid = rgid;
    }

    if setegid {
        process.egid = egid;
    }

    Ok(())
}

pub fn setrens(rns: SchemeNamespace, ens: SchemeNamespace) -> Result<()> {
    let process_lock = process::current()?;
    let mut process = process_lock.write();

    let setrns = if rns.get() as isize == -1 {
        // Ignore RNS if -1 is passed
        false
    } else if rns.get() == 0 {
        // Allow entering capability mode
        true
    } else if process.rns.get() == 0 {
        // Do not allow leaving capability mode
        return Err(Error::new(EPERM));
    } else if process.euid == 0 {
        // Allow setting RNS if root
        true
    } else if rns == process.ens {
        // Allow setting RNS if used for ENS
        true
    } else if rns == process.rns {
        // Allow setting RNS if used for RNS
        true
    } else {
        // Not permitted otherwise
        return Err(Error::new(EPERM));
    };

    let setens = if ens.get() as isize == -1 {
        // Ignore ENS if -1 is passed
        false
    } else if ens.get() == 0 {
        // Allow entering capability mode
        true
    } else if process.ens.get() == 0 {
        // Do not allow leaving capability mode
        return Err(Error::new(EPERM));
    } else if process.euid == 0 {
        // Allow setting ENS if root
        true
    } else if ens == process.ens {
        // Allow setting ENS if used for ENS
        true
    } else if ens == process.rns {
        // Allow setting ENS if used for RNS
        true
    } else {
        // Not permitted otherwise
        return Err(Error::new(EPERM));
    };

    if setrns {
        assert_ne!(rns.get() as isize, -1);
        process.rns = rns;
    }

    if setens {
        assert_ne!(ens.get() as isize, -1);
        process.ens = ens;
    }

    Ok(())
}

pub fn setreuid(ruid: u32, euid: u32) -> Result<()> {
    let process_lock = process::current()?;
    let mut process = process_lock.write();

    let setruid = if process.euid == 0 {
        // Allow setting RUID if root
        true
    } else if ruid == process.euid {
        // Allow setting RUID if used for EUID
        true
    } else if ruid == process.ruid {
        // Allow setting RUID if used for RUID
        true
    } else if ruid as i32 == -1 {
        // Ignore RUID if -1 is passed
        false
    } else {
        // Not permitted otherwise
        return Err(Error::new(EPERM));
    };

    let seteuid = if process.euid == 0 {
        // Allow setting EUID if root
        true
    } else if euid == process.euid {
        // Allow setting EUID if used for EUID
        true
    } else if euid == process.ruid {
        // Allow setting EUID if used for RUID
        true
    } else if euid as i32 == -1 {
        // Ignore EUID if -1 is passed
        false
    } else {
        // Not permitted otherwise
        return Err(Error::new(EPERM));
    };

    if setruid {
        process.ruid = ruid;
    }

    if seteuid {
        process.euid = euid;
    }

    Ok(())
}
