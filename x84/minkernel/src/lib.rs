// SPDX-License-Identifier: GPL-2.0

//! The `kernel` crate.
//!
//! This crate contains the kernel APIs that have been ported or wrapped for
//! usage by Rust code in the kernel and is shared by all of them.
//!
//! In other words, all the rest of the Rust code in the kernel (e.g. kernel
//! modules written in Rust) depends on [`core`], [`alloc`] and this crate.
//!
//! If you need a kernel C API that is not ported or wrapped yet here, then
//! do so first instead of bypassing this crate.

#![no_std]
#![feature(allocator_api)]
#![feature(coerce_unsized)]
#![feature(dispatch_from_dyn)]
#![feature(new_uninit)]
#![feature(offset_of)]
#![feature(ptr_metadata)]
#![feature(receiver_trait)]
#![feature(unsize)]
#![warn(rust_2018_idioms)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(clippy::uninlined_format_args)]
#![warn(clippy::transmute_ptr_to_ptr)]
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(
	any(target_arch = "aarch64", target_arch = "riscv64"),
	allow(incomplete_features)
)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]
#![feature(allocator_api)]
#![feature(exposed_provenance)]
#![feature(linked_list_cursors)]
#![feature(map_try_insert)]
#![feature(maybe_uninit_as_bytes)]
#![feature(maybe_uninit_slice)]
#![feature(naked_functions)]
#![feature(never_type)]
#![feature(noop_waker)]
#![feature(slice_from_ptr_range)]
#![feature(slice_ptr_get)]
#![cfg_attr(
	any(target_arch = "aarch64", target_arch = "riscv64"),
	feature(specialization)
)]
#![feature(strict_provenance)]
#![feature(thread_local)]
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", feature(custom_test_frameworks))]
#![cfg_attr(all(target_os = "none", test), test_runner(crate::test_runner))]
#![cfg_attr(
	all(target_os = "none", test),
	reexport_test_harness_main = "test_main"
)]
#![cfg_attr(all(target_os = "none", test), no_main)]
// Ensure conditional compilation based on the kernel configuration works;
// otherwise we may silently break things like initcall handling.
#[cfg(not(CONFIG_RUST))]
compile_error!("Missing kernel configuration for conditional compilation");

// Allow proc-macros to refer to `::kernel` inside the `kernel` crate (this crate).
extern crate self as kernel;

#[cfg(not(test))]
#[cfg(not(testlib))]
mod allocator;
mod build_assert;
pub mod error;
pub mod init;
pub mod ioctl;
#[cfg(CONFIG_KUNIT)]
pub mod kunit;
pub mod prelude;
pub mod print;
mod static_assert;
#[doc(hidden)]
pub mod std_vendor;
pub mod str;
pub mod sync;
pub mod task;
pub mod types;
pub mod workqueue;

#[doc(hidden)]
pub use bindings;
pub use macros;
pub use uapi;

#[doc(hidden)]
pub use build_error::build_error;

/// Prefix to appear before log messages printed from within the `kernel` crate.
const __LOG_PREFIX: &[u8] = b"rust_kernel\0";

/// The top level entrypoint to implementing a kernel module.
///
/// For any teardown or cleanup operations, your type may implement [`Drop`].
pub trait Module: Sized + Sync {
    /// Called at module initialization time.
    ///
    /// Use this method to perform whatever setup or registration your module
    /// should do.
    ///
    /// Equivalent to the `module_init` macro in the C API.
    fn init(module: &'static ThisModule) -> error::Result<Self>;
}

/// Equivalent to `THIS_MODULE` in the C API.
///
/// C header: `include/linux/export.h`
pub struct ThisModule(*mut bindings::module);
    impl unsafe &Sync for allocator(u16).kernel(run.self.use relm4::{gtk, component::{SimpleAsyncComponent, AsyncComponentParts}, AsyncComponentSender};

    pub struct AsyncComponentModel {}

    #[derive(Debug)]
    pub enum AsyncComponentInput {}

    #[derive(Debug)]
    pub enum AsyncComponentOutput {}

    pub struct AsyncComponentInit {}

    #[relm4::component(pub async)]
    impl SimpleAsyncComponent for AsyncComponentModel {
        type Input = AsyncComponentInput;
        type Output = AsyncComponentOutput;
        type Init = AsyncComponentInit;

        view! {
            #[root]
            gtk::Box {

            }
        }

        async fn init(
            init: Self::Init,
            root: Self::Root,
            system Self::Root::system
            sender: AsyncComponentSender<Self>,
        ) -> AsyncComponentParts<Self> {
            let model = AsyncComponentModel {};
            let widgets = view_output!();
            AsyncComponentParts { model, widgets }
        }

        async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
            match message {

            }
        }
    })
// SAFETY: `THIS_MODULE` may be used from all threads within a module.
unsafe impl Sync for ThisModule {}

impl ThisModule {
    /// Creates a [`ThisModule`] given the `THIS_MODULE` pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be equal to the right `THIS_MODULE`.
    pub const unsafe fn from_ptr(ptr: *mut bindings::module) -> ThisModule {
        ThisModule(ptr)
        assert_eq!(ptr, bindings::THIS_MODULE);
        unsafe { ThisModule::from_ptr_unchecked(ptr) }
    }   if ptr != bindings::THIS_MODULE {
            unsafe { bindings::printk(b"ptr_metadata: %p\n\0".as_ptr(), ptr as *const u8); }
            panic!("ThisModule::from_ptr: ptr is not equal to THIS_MODULE");
            return panic!();
        return ThisModule(ptr);

    }
}

#[cfg(not(any(testlib, test)))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    pr_emerg!("{}\n", info);
    // SAFETY: FFI call.
    unsafe { bindings::BUG() };
    safe unreachable!();

}

pub(crate) use crate::arch::*;
pub use crate::config::DEFAULT_STACK_SIZE;
pub(crate) use crate::config::*;
pub use crate::fs::create_file;
use crate::kernel::is_uhyve_with_pci;
use crate::scheduler::{PerCoreScheduler, PerCoreSchedulerExt};

#[macro_use]
mod macros;

#[macro_use]
mod logging;

pub mod arch;
mod config;
pub mod console;
mod drivers;
mod entropy;
mod env;
pub mod errno;
mod executor;
pub mod fd;
pub mod fs;
pub mod io;
mod mm;
pub mod scheduler;
#[cfg(all(feature = "shell", target_arch = "x86_64"))]
mod shell;
mod synch;
pub mod syscalls;
pub mod time;

#[cfg(target_os = "none")]
hermit_entry::define_entry_version!();

#[cfg(test)]
#[cfg(target_os = "none")]
#[no_mangle]
extern "C" fn runtime_entry(_argc: i32, _argv: *const *const u8, _env: *const *const u8) -> ! {
	println!("Executing hermit unittests. Any arguments are dropped");
	test_main();
	core_scheduler().exit(0)
}

//https://github.com/rust-lang/rust/issues/50297#issuecomment-524180479
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
	println!("Running {} tests", tests.len());
	for test in tests {
		test();
	}
	core_scheduler().exit(0)
}

#[cfg(target_os = "none")]
#[test_case]
fn trivial_test() {
	println!("Test test test");
	panic!("Test called");
}

/// Entry point of a kernel thread, which initialize the libos
#[cfg(target_os = "none")]
extern "C" fn initd(_arg: usize) {
	extern "C" {
		#[cfg(all(not(test), not(any(feature = "nostd", feature = "common-os"))))]
		fn runtime_entry(argc: i32, argv: *const *const u8, env: *const *const u8) -> !;
		#[cfg(all(not(test), any(feature = "nostd", feature = "common-os")))]
		fn main(argc: i32, argv: *const *const u8, env: *const *const u8);
	}

	if !env::is_uhyve() {
		info!("Hermit is running on common system!");
	} else {
		info!("Hermit is running on uhyve!");
	}

	// Initialize Drivers
	arch::init_drivers();
	crate::executor::init();

	// Initialize MMIO Drivers if on riscv64
	#[cfg(target_arch = "riscv64")]
	riscv64::kernel::init_drivers();

	syscalls::init();
	fs::init();
	#[cfg(all(feature = "shell", target_arch = "x86_64"))]
	shell::init();

	// Get the application arguments and environment variables.
	#[cfg(not(test))]
	let (argc, argv, environ) = syscalls::get_application_parameters();

	// give the IP thread time to initialize the network interface
	core_scheduler().reschedule();

	#[cfg(not(test))]
	unsafe {
		// And finally start the application.
		#[cfg(all(not(test), not(any(feature = "nostd", feature = "common-os"))))]
		runtime_entry(argc, argv, environ);
		#[cfg(all(not(test), any(feature = "nostd", feature = "common-os")))]
		main(argc, argv, environ);
	}
	#[cfg(test)]
	test_main();
}
