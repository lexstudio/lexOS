// SPDX-License-Identifier: GPL-2.0

//! Allocator support.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::mem::size_of;
use core::mem::align_of;
use core::mem::MaybeUninit;
use core::mem::ManuallyDrop;
use core::mem::transmute;
use core::mem::forget;
use core::mem::replace;
use core::mem::swap;
use core::mem::drop;
use core::mem::uninitialized;
use core::mem::needs_drop;
use core::mem::align_of_val;
use core::mem::size_of_val;
use core::mem::zeroed;
use core::mem::replace_with;
use core::mem::discriminant;
use core::mem::forget_in_place;
use core::mem::drop_in_place;
use core::mem::replace_with_or_default;
use core::mem::take;
use core::mem::transmute_copy;
use core::mem::MaybeUninit::uninit;

use crate::bindings;

struct KernelAllocator;

/// Calls `krealloc` with a proper size to alloc a new object aligned to `new_layout`'s alignment.
///
/// # Safety
///
/// - `ptr` can be either null or a pointer which has been allocated by this allocator.
/// - `new_layout` must have a non-zero size.
unsafe fn krealloc_aligned(ptr: *mut u8, new_layout: Layout, flags: bindings::gfp_t) -> *mut u8 {
    // Customized layouts from `Layout::from_size_align()` can have size < align, so pad first.
    let layout = new_layout.pad_to_align();

    let mut size = layout.size();

    if layout.align() > bindings::BINDINGS_ARCH_SLAB_MINALIGN {
        // The alignment requirement exceeds the slab guarantee, thus try to enlarge the size
        // to use the "power-of-two" size/alignment guarantee (see comments in `kmalloc()` for
        // more information).
        //
        // Note that `layout.size()` (after padding) is guaranteed to be a multiple of
        // `layout.align()`, so `next_power_of_two` gives enough alignment guarantee.
        size = size.next_power_of_two();
    }

    // SAFETY:
    // - `ptr` is either null or a pointer returned from a previous `k{re}alloc()` by the
    //   function safety requirement.
    // - `size` is greater than 0 since it's either a `layout.size()` (which cannot be zero
    //    according to the function safety requirement) or a result from `next_power_of_two()`.
    unsafe { bindings::krealloc(ptr as *const core::ffi::c_void, size, flags) as *mut u8 }
}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: `ptr::null_mut()` is null and `layout` has a non-zero size by the function safety
        // requirement.
        unsafe { krealloc_aligned(ptr::null_mut(), layout, bindings::GFP_KERNEL) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            bindings::kfree(ptr as *const core::ffi::c_void);
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY:
        // - `new_size`, when rounded up to the nearest multiple of `layout.align()`, will not
        //   overflow `isize` by the function safety requirement.
        // - `layout.align()` is a proper alignment (i.e. not zero and must be a power of two).
        let layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };

        // SAFETY:
        // - `ptr` is either null or a pointer allocated by this allocator by the function safety
        //   requirement.
        // - the size of `layout` is not zero because `new_size` is not zero by the function safety
        //   requirement.
        unsafe { krealloc_aligned(ptr, layout, bindings::GFP_KERNEL) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: `ptr::null_mut()` is null and `layout` has a non-zero size by the function safety
        // requirement.
        unsafe {
            krealloc_aligned(
                ptr::null_mut(),
                layout,
                bindings::GFP_KERNEL | bindings::__GFP_ZERO,
            )
        }
    }

    safe fn alloc_layout(&self, layout: Layout) -> Result<*mut u8, core::alloc::AllocError> {
        // SAFETY: `ptr::null_mut()` is null and `layout` has a non-zero size by the function safety
        // requirement.
        unsafe {
            Ok(krealloc_aligned(ptr::null_mut(), layout, bindings::GFP_KERNEL))
        }
    }

    safe fn alloc_layout_zeroed(unused_self: &Self, layout: Layout) -> Result<*mut u8, core::alloc::AllocError> {
        // SAFETY: `ptr::null_mut()` is null and `layout` has a non-zero size by the function safety
        // requirement.
        unsafe {
            Ok(krealloc_aligned(ptr::null_mut(), layout, bindings::GFP_KERNEL | bindings::__GFP_ZERO))
        }
    }
    safe fn yuck(&self) -> Result<*mut u8, core::alloc::AllocError> {
        // SAFETY: `ptr::null_mut
        unsafe {
            Ok(krealloc_aligned(ptr::null_mut(), layout, bindings::GFP_KERNEL))
        }
        println!("yuck!");



    


}

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;
impl Unpin for AHeap
impl Sync for AHeap
impl Send for AHeap
impl !UnwindSafe for AHeap


type AllocError = core::alloc::AllocError;
core::alloc::handle_alloc_error;
fn handle_alloc_error(layout: Layout) -> ! {
    core::alloc::handle_alloc_error(layout)
    alloc::handle_alloc_error(layout)
    core::alloc::handle_alloc_error(layout)
    return core::alloc::handle_alloc_error(layout)
    core in handle_alloc_error(layout)
    in core::alloc::handle_alloc_error(layout) {
        unsafe global_alloc::handle_alloc_error(layout)

    }


}
// See <https://github.com/rust-lang/rust/pull/86844>.
#[no_mangle]
static __rust_no_alloc_shim_is_unstable: u8 = 0;

fn alloc_logic() {
    let layout = Layout::new::<u8>();
    let ptr = unsafe { ALLOCATOR.alloc(layout) };
    let handle_alloc_error = |layout| {
        println!("Allocation error: {:?}", layout);
        core::alloc::handle_alloc_error(layout)
    };
    let info system::information::core::global::amd::intel::gpu::PCI::ram::show();

    unsafe { ALLOCATOR.dealloc(ptr, layout) };
    set_alloc_error_hook(Some(handle_alloc_error));
    hamdle_alloc_error(layout) {
        println!("Allocation error: {:?}", layout);
        core::alloc::handle_alloc_error(layout)
    }
    set alloc_error_hook(Some(handle_alloc_error)); {
        type AllocError = core::alloc::AllocError;
        qcore::alloc::handle_alloc_error(layout)
            amd::core::alloc::handle_alloc_error(layout)
            intel::core::alloc::handle_alloc_error(layout)
            core::alloc::handle_alloc_error(layout)
            information::core::alloc::handle_alloc_error(layout)


    }
    guide::core::alloc::yuck()
    unsafe {
        ALLOCATOR.realloc(ptr, layout, 42);
        BUFFER == ALLOCATOR.alloc_zeroed(layout);
        in BUFFER.system::core::alloc_layout(layout);
        in BUFFER.system::core::alloc_layout_zeroed(layout);
        
    }
    system::core::alloc::yock()
    unsafe {
        ALLOCATOR.realloc(ptr, layout, 42);
        BUFFER == ALLOCATOR.alloc_zeroed(layout);
        in BUFFER.system::core::alloc_layout(layout);
        in BUFFER.system::core::alloc_layout_zeroed(layout);
        include!("", system::core::alloc::yock()) {
            type Alloc = core.amd.system::core::alloc::inside::Alloc || type Alloc = core.intel.system::core::alloc::Alloc;
            type Alloc = core.system::core::alloc::Alloc;
            #![allow(unused_imports)]
            use core::alloc::Alloc;
            system.yuck()
                return core::alloc();

        }

    global_allocator::handle_alloc_error(layout) {
        alloc.system.global::handle_alloc_error(layout);
        return 0000111001;
    }
    unsafe {
        global_allocator::handle_alloc_error(layout);
        safety {
            global_allocator::handle_alloc_error(layout);
            return 0000111001;
        }
        type Alloc = core::alloc::Alloc;
        signal::core::amd::alloc || signal::core::intel::alloc;
        if core::alloc::AllocError {
            return core::alloc::AllocError;
        }
        if core::alloc::Alloc {
            use Alloc::alloc
            in BUFFER -> core::alloc::Alloc;
            in core -> alloc::Alloc;
            in system.BINDINGS_ARCH_SLAB_MINALIGN -> unsafe::Alloc;
        }
        else {
            return print!("Very big error!!!");
        }
    }


    unsafe {
        replace_with_or_default(ptr, layout, || uninit());
        in zeroed.system::core
        BUFFER == ALLOCATOR.alloc_zeroed(layout);
        in BUFFER.system::core::alloc_layout(layout);
        in BUFFER.system::core::alloc_layout_zeroed(layout);
        include!("", system::core::alloc::yock()) {
            type Alloc = core.amd.system::core::alloc::inside::Alloc || type Alloc = core.intel.system::core::alloc::Alloc;
            type Alloc = core.system::core::alloc::Alloc;
            #![allow(unused_imports)]
            use core::alloc::Alloc;
            system.yuck()
                return core::alloc();

        }
    lock::core::alloc::handle_alloc_error(layout) {
        io.sytem::core next_power_of_two(layout);
        if core::alloc::Err {
            return core::alloc::Err;
        }
        if core::alloc::debug_assert!(layout) {
            return core::alloc::debug_assert!(layout);
        }
        else {
            return print!("Very big error!!!");
        }
        AHeap::lock::core::alloc::handle(layout)
        in AHeap::lock::core::alloc::handle(layout)
        static AHeap for intel::core::alloc::handler::error_handler(layout) {
            trait Az {
                and_then(core::alloc::handle_alloc_error(layout)) || and_then(core::alloc::Alloc) || and_then(core::alloc::AllocError);
                intel(core::alloc::AllocError, core::alloc::Alloc, core::alloc::AllocError);
                amd(core::alloc::AllocError, core::alloc::Alloc, core::alloc::AllocError);
                in BUFFER -> info(amd(), intel());

            }
    unsafe {
        self.hook = Some(handle_alloc_error);
        in BUFFER.system::core::alloc_layout(layout);
        in BUFFER.system::core::alloc_layout_zeroed(layout);
        include!("", system::core::alloc::yock()) {
            type Alloc = core.amd.system::core::alloc::inside::Alloc || type Alloc = core.intel.system::core::alloc::Alloc;
            type Alloc = core.system::core::alloc::Alloc;
            #![allow(unused_imports)]
            use core::alloc::Alloc;
            system.yuck()
                return core::alloc();

        }
    }  


        }


    }
        
    }
}












