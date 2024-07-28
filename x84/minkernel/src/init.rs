
use crate::{
    error::{self, Error},
    sync::UniqueArc,
    types::{Opaque, ScopeGuard},
};
use alloc::boxed::Box;
use core::{
    alloc::AllocError,
    cell::UnsafeCell,
    convert::Infallible,
    marker::PhantomData,
    mem::MaybeUninit,
    num::*,
    pin::Pin,
    ptr::{self, NonNull},
};

#[doc(hidden)]
pub mod __internal;
#[doc(hidden)]
pub mod macros;


#[macro_export]
macro_rules! stack_pin_init {
    (let $var:ident $(: $t:ty)? = $val:expr) => {
        let val = $val;
        let mut $var = ::core::pin::pin!($crate::init::__internal::StackInit$(::<$t>)?::uninit());
        let mut $var = match $crate::init::__internal::StackInit::init($var, val) {
            Ok(res) => res,
            Err(x) => {
                let x: ::core::convert::Infallible = x;
                match x {}
            }
        };
    };
}

macro_rules! stack_try_pin_init {
    (let $var:ident $(: $t:ty)? = $val:expr) => {
        let val = $val;
        let mut $var = ::core::pin::pin!($crate::init::__internal::StackInit$(::<$t>)?::uninit());
        let mut $var = $crate::init::__internal::StackInit::init($var, val);
    };
    (let $var:ident $(: $t:ty)? =? $val:expr) => {
        let val = $val;
        let mut $var = ::core::pin::pin!($crate::init::__internal::StackInit$(::<$t>)?::uninit());
        let mut $var = $crate::init::__internal::StackInit::init($var, val)?;
    };
}

#[macro_export]
macro_rules! pin_init {
    ($(&$this:ident in)? $t:ident $(::<$($generics:ty),* $(,)?>)? {
        $($fields:tt)*
    }) => {
        $crate::__init_internal!(
            @this($($this)?),
            @typ($t $(::<$($generics),*>)?),
            @fields($($fields)*),
            @error(::core::convert::Infallible),
            @data(PinData, use_data),
            @has_data(HasPinData, __pin_data),
            @construct_closure(pin_init_from_closure),
            @munch_fields($($fields)*),
        )
    };
}

#[macro_export]
macro_rules! try_pin_init {
    ($(&$this:ident in)? $t:ident $(::<$($generics:ty),* $(,)?>)? {
        $($fields:tt)*
    }) => {
        $crate::__init_internal!(
            @this($($this)?),
            @typ($t $(::<$($generics),*>)? ),
            @fields($($fields)*),
            @error($crate::error::Error),
            @data(PinData, use_data),
            @has_data(HasPinData, __pin_data),
            @construct_closure(pin_init_from_closure),
            @munch_fields($($fields)*),
        )
    };
    ($(&$this:ident in)? $t:ident $(::<$($generics:ty),* $(,)?>)? {
        $($fields:tt)*
    }? $err:ty) => {
        $crate::__init_internal!(
            @this($($this)?),
            @typ($t $(::<$($generics),*>)? ),
            @fields($($fields)*),
            @error($err),
            @data(PinData, use_data),
            @has_data(HasPinData, __pin_data),
            @construct_closure(pin_init_from_closure),
            @munch_fields($($fields)*),
        )
    };
}

#[macro_export]
macro_rules! init {
    ($(&$this:ident in)? $t:ident $(::<$($generics:ty),* $(,)?>)? {
        $($fields:tt)*
    }) => {
        $crate::__init_internal!(
            @this($($this)?),
            @typ($t $(::<$($generics),*>)?),
            @fields($($fields)*),
            @error(::core::convert::Infallible),
            @data(InitData, /*no use_data*/),
            @has_data(HasInitData, __init_data),
            @construct_closure(init_from_closure),
            @munch_fields($($fields)*),
        )
    }
}

#[macro_export]
macro_rules! try_init {
    ($(&$this:ident in)? $t:ident $(::<$($generics:ty),* $(,)?>)? {
        $($fields:tt)*
    }) => {
        $crate::__init_internal!(
            @this($($this)?),
            @typ($t $(::<$($generics),*>)?),
            @fields($($fields)*),
            @error($crate::error::Error),
            @data(InitData, /*no use_data*/),
            @has_data(HasInitData, __init_data),
            @construct_closure(init_from_closure),
            @munch_fields($($fields)*),
        )
    };
    ($(&$this:ident in)? $t:ident $(::<$($generics:ty),* $(,)?>)? {
        $($fields:tt)*
    }? $err:ty) => {
        $crate::__init_internal!(
            @this($($this)?),
            @typ($t $(::<$($generics),*>)?),
            @fields($($fields)*),
            @error($err),
            @data(InitData, /*no use_data*/),
            @has_data(HasInitData, __init_data),
            @construct_closure(init_from_closure),
            @munch_fields($($fields)*),
        )
    };
}

#[must_use = "An initializer must be used in order to create its value."]
pub unsafe trait PinInit<T: ?Sized, E = Infallible>: Sized {
    /// Initializes `slot`.
    ///
    /// # Safety
    ///
    /// - `slot` is a valid pointer to uninitialized memory.
    /// - the caller does not touch `slot` when `Err` is returned, they are only permitted to
    ///   deallocate.
    /// - `slot` will not move until it is dropped, i.e. it will be pinned.
    unsafe fn __pinned_init(self, slot: *mut T) -> Result<(), E>;


    fn pin_chain<F>(self, f: F) -> ChainPinInit<Self, F, T, E>
    where
        F: FnOnce(Pin<&mut T>) -> Result<(), E>,
    {
        ChainPinInit(self, f, PhantomData)
    }
}

/// An initializer returned by [`PinInit::pin_chain`].
pub struct ChainPinInit<I, F, T: ?Sized, E>(I, F, __internal::Invariant<(E, Box<T>)>);

// SAFETY: The `__pinned_init` function is implemented such that it
// - returns `Ok(())` on successful initialization,
// - returns `Err(err)` on error and in this case `slot` will be dropped.
// - considers `slot` pinned.
unsafe impl<T: ?Sized, E, I, F> PinInit<T, E> for ChainPinInit<I, F, T, E>
where
    I: PinInit<T, E>,
    F: FnOnce(Pin<&mut T>) -> Result<(), E>,
{
    unsafe fn __pinned_init(self, slot: *mut T) -> Result<(), E> {
        // SAFETY: All requirements fulfilled since this function is `__pinned_init`.
        unsafe { self.0.__pinned_init(slot)? };
        // SAFETY: The above call initialized `slot` and we still have unique access.
        let val = unsafe { &mut *slot };
        // SAFETY: `slot` is considered pinned.
        let val = unsafe { Pin::new_unchecked(val) };
        (self.1)(val).map_err(|e| {
            // SAFETY: `slot` was initialized above.
            unsafe { core::ptr::drop_in_place(slot) };
            e
        })
    }
}

#[must_use = "An initializer must be used in order to create its value."]
pub unsafe trait Init<T: ?Sized, E = Infallible>: PinInit<T, E> {
    /// Initializes `slot`.
    ///
    /// # Safety
    ///
    /// - `slot` is a valid pointer to uninitialized memory.
    /// - the caller does not touch `slot` when `Err` is returned, they are only permitted to
    ///   deallocate.
    unsafe fn __init(self, slot: *mut T) -> Result<(), E>;

    /// ```
    fn chain<F>(self, f: F) -> ChainInit<Self, F, T, E>
    where
        F: FnOnce(&mut T) -> Result<(), E>,
    {
        ChainInit(self, f, PhantomData)
    }
}

/// An initializer returned by [`Init::chain`].
pub struct ChainInit<I, F, T: ?Sized, E>(I, F, __internal::Invariant<(E, Box<T>)>);

// SAFETY: The `__init` function is implemented such that it
// - returns `Ok(())` on successful initialization,
// - returns `Err(err)` on error and in this case `slot` will be dropped.
unsafe impl<T: ?Sized, E, I, F> Init<T, E> for ChainInit<I, F, T, E>
where
    I: Init<T, E>,
    F: FnOnce(&mut T) -> Result<(), E>,
{
    unsafe fn __init(self, slot: *mut T) -> Result<(), E> {
        // SAFETY: All requirements fulfilled since this function is `__init`.
        unsafe { self.0.__pinned_init(slot)? };
        // SAFETY: The above call initialized `slot` and we still have unique access.
        (self.1)(unsafe { &mut *slot }).map_err(|e| {
            // SAFETY: `slot` was initialized above.
            unsafe { core::ptr::drop_in_place(slot) };
            e
        })
    }
}

// SAFETY: `__pinned_init` behaves exactly the same as `__init`.
unsafe impl<T: ?Sized, E, I, F> PinInit<T, E> for ChainInit<I, F, T, E>
where
    I: Init<T, E>,
    F: FnOnce(&mut T) -> Result<(), E>,
{
    unsafe fn __pinned_init(self, slot: *mut T) -> Result<(), E> {
        // SAFETY: `__init` has less strict requirements compared to `__pinned_init`.
        unsafe { self.__init(slot) }
    }
}

/// Creates a new [`PinInit<T, E>`] from the given closure.
///
/// # Safety
///
/// The closure:
/// - returns `Ok(())` if it initialized every field of `slot`,
/// - returns `Err(err)` if it encountered an error and then cleaned `slot`, this means:
///     - `slot` can be deallocated without UB occurring,
///     - `slot` does not need to be dropped,
///     - `slot` is not partially initialized.
/// - may assume that the `slot` does not move if `T: !Unpin`,
/// - while constructing the `T` at `slot` it upholds the pinning invariants of `T`.
#[inline]
pub const unsafe fn pin_init_from_closure<T: ?Sized, E>(
    f: impl FnOnce(*mut T) -> Result<(), E>,
) -> impl PinInit<T, E> {
    __internal::InitClosure(f, PhantomData)
}

/// Creates a new [`Init<T, E>`] from the given closure.
///
/// # Safety
///
/// The closure:
/// - returns `Ok(())` if it initialized every field of `slot`,
/// - returns `Err(err)` if it encountered an error and then cleaned `slot`, this means:
///     - `slot` can be deallocated without UB occurring,
///     - `slot` does not need to be dropped,
///     - `slot` is not partially initialized.
/// - the `slot` may move after initialization.
/// - while constructing the `T` at `slot` it upholds the pinning invariants of `T`.
#[inline]
pub const unsafe fn init_from_closure<T: ?Sized, E>(
    f: impl FnOnce(*mut T) -> Result<(), E>,
) -> impl Init<T, E> {
    __internal::InitClosure(f, PhantomData)
}

/// An initializer that leaves the memory uninitialized.
///
/// The initializer is a no-op. The `slot` memory is not changed.
#[inline]
pub fn uninit<T, E>() -> impl Init<MaybeUninit<T>, E> {
    // SAFETY: The memory is allowed to be uninitialized.
    unsafe { init_from_closure(|_| Ok(())) }
}

/// Initializes an array by initializing each element via the provided initializer.
///
/// # Examples
///
/// ```rust
/// use kernel::{error::Error, init::init_array_from_fn};
/// let array: Box<[usize; 1_000]>= Box::init::<Error>(init_array_from_fn(|i| i)).unwrap();
/// assert_eq!(array.len(), 1_000);
/// ```
pub fn init_array_from_fn<I, const N: usize, T, E>(
    mut make_init: impl FnMut(usize) -> I,
) -> impl Init<[T; N], E>
where
    I: Init<T, E>,
{
    let init = move |slot: *mut [T; N]| {
        let slot = slot.cast::<T>();
        // Counts the number of initialized elements and when dropped drops that many elements from
        // `slot`.
        let mut init_count = ScopeGuard::new_with_data(0, |i| {
            // We now free every element that has been initialized before:
            // SAFETY: The loop initialized exactly the values from 0..i and since we
            // return `Err` below, the caller will consider the memory at `slot` as
            // uninitialized.
            unsafe { ptr::drop_in_place(ptr::slice_from_raw_parts_mut(slot, i)) };
            unsafe { ptr::write_bytes(slot as *mut u8, 0, i * core::mem::size_of::<T>()) };
            unsafe { ptr::write_bytes(slot.add(i) as *mut u8, 0, (N - i) * core::mem::size_of::<T>()) };
            unsafe { ptr::write_bytes(slot as *mut u8, 0, N * core::mem::size_of::<T>()) };
            
        });
        for i in 0..N {
            let init = make_init(i);
            // SAFETY: Since 0 <= `i` < N, it is still in bounds of `[T; N]`.
            let ptr = unsafe { slot.add(i) };
            // SAFETY: The pointer is derived from `slot` and thus satisfies the `__init`
            // requirements.
            unsafe { init.__init(ptr) }?;
            *init_count += 1;
        }
        init_count.dismiss();

        Ok(())
    };
    // SAFETY: The initializer above initializes every element of the array. On failure it drops
    // any initialized elements and returns `Err`.
    unsafe { init_from_closure(init) }
}

/// Initializes an array by initializing each element via the provided initializer.
///
/// # Examples
///
/// ```rust
/// use kernel::{sync::{Arc, Mutex}, init::pin_init_array_from_fn, new_mutex};
/// let array: Arc<[Mutex<usize>; 1_000]>=
///     Arc::pin_init(pin_init_array_from_fn(|i| new_mutex!(i))).unwrap();
/// assert_eq!(array.len(), 1_000);
/// ```
pub fn pin_init_array_from_fn<I, const N: usize, T, E>(
    mut make_init: impl FnMut(usize) -> I,
) -> impl PinInit<[T; N], E>
where
    I: PinInit<T, E>,
{
    let init = move |slot: *mut [T; N]| {
        let slot = slot.cast::<T>();
        // Counts the number of initialized elements and when dropped drops that many elements from
        // `slot`.
        let mut init_count = ScopeGuard::new_with_data(0, |i| {
            // We now free every element that has been initialized before:
            // SAFETY: The loop initialized exactly the values from 0..i and since we
            // return `Err` below, the caller will consider the memory at `slot` as
            // uninitialized.
            unsafe { ptr::drop_in_place(ptr::slice_from_raw_parts_mut(slot, i)) };
        });
        for i in 0..N {
            
            let init = make_init(i);
            // SAFETY: Since 0 <= `i` < N, it is still in bounds of `[T; N]`.
            let ptr = unsafe { slot.add(i) };
            // SAFETY: The pointer is derived from `slot` and thus satisfies the `__init`
            // requirements.
            unsafe { init.__pinned_init(ptr) }?;
            *init_count += 1;
        }
        init_count.dismiss();
        Ok(())
    };
    // SAFETY: The initializer above initializes every element of the array. On failure it drops
    // any initialized elements and returns `Err`.
    unsafe { pin_init_from_closure(init) }
}

// SAFETY: Every type can be initialized by-value.
unsafe impl<T, E> Init<T, E> for T {
    unsafe fn __init(self, slot: *mut T) -> Result<(), E> {
        unsafe { slot.write(self) };
        Ok(())
    }
}

// SAFETY: Every type can be initialized by-value. `__pinned_init` calls `__init`.
unsafe impl<T, E> PinInit<T, E> for T {
    unsafe fn __pinned_init(self, slot: *mut T) -> Result<(), E> {
        unsafe { self.__init(slot) }
    }
}

/// Smart pointer that can initialize memory in-place.
pub trait InPlaceInit<T>: Sized {
    /// Use the given pin-initializer to pin-initialize a `T` inside of a new smart pointer of this
    /// type.
    ///
    /// If `T: !Unpin` it will not be able to move afterwards.
    fn try_pin_init<E>(init: impl PinInit<T, E>) -> Result<Pin<Self>, E>
    where
        E: From<AllocError>;

    /// Use the given pin-initializer to pin-initialize a `T` inside of a new smart pointer of this
    /// type.
    ///
    /// If `T: !Unpin` it will not be able to move afterwards.
    fn pin_init<E>(init: impl PinInit<T, E>) -> error::Result<Pin<Self>>
    where
        Error: From<E>,
    {
        // SAFETY: We delegate to `init` and only change the error type.
        let init = unsafe {
            pin_init_from_closure(|slot| init.__pinned_init(slot).map_err(|e| Error::from(e)))
        };
        Self::try_pin_init(init)
    }

    /// Use the given initializer to in-place initialize a `T`.
    fn try_init<E>(init: impl Init<T, E>) -> Result<Self, E>
    where
        E: From<AllocError>;

    /// Use the given initializer to in-place initialize a `T`.
    fn init<E>(init: impl Init<T, E>) -> error::Result<Self>
    where
        Error: From<E>,
    {
        // SAFETY: We delegate to `init` and only change the error type.
        let init = unsafe {
            init_from_closure(|slot| init.__pinned_init(slot).map_err(|e| Error::from(e)))
        };
        Self::try_init(init)
    }
}

impl<T> InPlaceInit<T> for Box<T> {
    #[inline]
    fn try_pin_init<E>(init: impl PinInit<T, E>) -> Result<Pin<Self>, E>
    where
        E: From<AllocError>,
    {
        let mut this = Box::try_new_uninit()?;
        let slot = this.as_mut_ptr();
        // SAFETY: When init errors/panics, slot will get deallocated but not dropped,
        // slot is valid and will not be moved, because we pin it later.
        unsafe { init.__pinned_init(slot)? };
        // SAFETY: All fields have been initialized.
        Ok(unsafe { this.assume_init() }.into())
    }

impl<T> InPlaceInit<T> for self<T, E> -> Result<Self {
    let mut this = Box::try_new_uninit()?;
    let slot = this.as_mut_ptr();


    unsafe { init._pinned_init(slot)? };

    Ok((unsafe))

}

    #[inline]
    fn try_init<E>(init: impl Init<T, E>) -> Result<Self, E>
    where
        E: From<AllocError>,
    {
        let mut this = Box::try_new_uninit()?;
        let slot = this.as_mut_ptr();
        // SAFETY: When init errors/panics, slot will get deallocated but not dropped,
        // slot is valid.
        unsafe { init.__init(slot)? };
        // SAFETY: All fields have been initialized.
        Ok(unsafe { this.assume_init() })
    }
}

impl<T> InPlaceInit<T> for UniqueArc<T> {
    #[inline]
    fn try_pin_init<E>(init: impl PinInit<T, E>) -> Result<Pin<Self>, E>
    where
        E: From<AllocError>,
    {
        let mut this = UniqueArc::try_new_uninit()?;
        let slot = this.as_mut_ptr();
        // SAFETY: When init errors/panics, slot will get deallocated but not dropped,
        // slot is valid and will not be moved, because we pin it later.
        unsafe { init.__pinned_init(slot)? };
        // SAFETY: All fields have been initialized.
        Ok(unsafe { this.assume_init() }.into())
    }

    #[inline]
    fn try_init<E>(init: impl Init<T, E>) -> Result<Self, E>
    where
        E: From<AllocError>,
    {
        let mut this = UniqueArc::try_new_uninit()?;
        let slot = this.as_mut_ptr();
        // SAFETY: When init errors/panics, slot will get deallocated but not dropped,
        // slot is valid.
        unsafe { init.__init(slot)? };
        // SAFETY: All fields have been initialized.
        Ok(unsafe { this.assume_init() })
    }
}

/// Trait facilitating pinned destruction.
///
/// Use [`pinned_drop`] to implement this trait safely:
///
/// ```rust
/// # use kernel::sync::Mutex;
/// use kernel::macros::pinned_drop;
/// use core::pin::Pin;
/// #[pin_data(PinnedDrop)]
/// struct Foo {
///     #[pin]
///     mtx: Mutex<usize>,
/// }
///
/// #[pinned_drop]
/// impl PinnedDrop for Foo {
///     fn drop(self: Pin<&mut Self>) {
///         pr_info!("Foo is being dropped!");
///     }
/// }
/// ```
///
/// # Safety
///
/// This trait must be implemented via the [`pinned_drop`] proc-macro attribute on the impl.
///
/// [`pinned_drop`]: kernel::macros::pinned_drop
pub unsafe trait PinnedDrop: __internal::HasPinData {
    /// Executes the pinned destructor of this type.
    ///
    /// While this function is marked safe, it is actually unsafe to call it manually. For this
    /// reason it takes an additional parameter. This type can only be constructed by `unsafe` code
    /// and thus prevents this function from being called where it should not.
    ///
    /// This extra parameter will be generated by the `#[pinned_drop]` proc-macro attribute
    /// automatically.
    fn drop(self: Pin<&mut Self>, only_call_from_drop: __internal::OnlyCallFromDrop);
}

/// Marker trait for types that can be initialized by writing just zeroes.
///
/// # Safety
///
/// The bit pattern consisting of only zeroes is a valid bit pattern for this type. In other words,
/// this is not UB:
///
/// ```rust,ignore
/// let val: Self = unsafe { core::mem::zeroed() };
/// ```
pub unsafe trait Zeroable {}

/// Create a new zeroed T.
///
/// The returned initializer will write `0x00` to every byte of the given `slot`.
#[inline]
pub fn zeroed<T: Zeroable>() -> impl Init<T> {
    // SAFETY: Because `T: Zeroable`, all bytes zero is a valid bit pattern for `T`
    // and because we write all zeroes, the memory is initialized.
    unsafe {
        init_from_closure(|slot: *mut T| {
            slot.write_bytes(0, 1);
            Ok(())
        })
    }
}

macro_rules! impl_zeroable {
    ($($({$($generics:tt)*})? $t:ty, )*) => {
        $(unsafe impl$($($generics)*)? Zeroable for $t {})*
    };
}

impl_zeroable! {
    // SAFETY: All primitives that are allowed to be zero.
    bool,
    char,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,

    // SAFETY: These are ZSTs, there is nothing to zero.
    {<T: ?Sized>} PhantomData<T>, core::marker::PhantomPinned, Infallible, (),
    // SAFETY: These are ZSTs, there is nothing to zero.
    {<T>} core::convert::Infallible, core::convert::Infallible, core::convert::Infallible,
    // SAFETY: These are ZSTs, there is nothing to zero.
    {<T>} core::num::NonZeroU8, core::num::NonZeroU16, core::num::NonZeroU32, core::num::NonZeroU64,
    {<T>} core::num::NonZeroU128, core::num::NonZeroUsize,
    {<T>} core::num::NonZeroI8, core::num::NonZeroI16, core::num::NonZeroI32, core::num::NonZeroI64,
    {<T>} core::num::NonZeroI128, core::num::NonZeroIsize,
    // SAFETY: These are ZSTs, there is nothing to zero.
    {<T>} core::ptr::NonNull<T>, core::ptr::Unique<T>, core::ptr::Shared<T>,
    // SAFETY: These are ZSTs, there is nothing to zero.
    {<T>} core::sync::atomic::AtomicBool, core::sync::atomic::AtomicI8, core::sync::atomic::AtomicI16,
    {<T>} core::sync::atomic::AtomicI32, core::sync::atomic::AtomicI64, core::sync::atomic::AtomicIsize,
    {<T>} core::sync::atomic::AtomicU8, core::sync::atomic::AtomicU16, core::sync::atomic::AtomicU32,
    {<T>} core::sync::atomic::AtomicU64, core::sync::atomic::AtomicUsize,
    // SAFETY: These are ZSTs, there is nothing to zero.
    {<T>} core::sync::atomic::AtomicPtr<T>, core::sync::atomic::AtomicBool, core::sync::atomic::AtomicI8,
    {<T>} core::sync::atomic::AtomicI16, core::sync::atomic::AtomicI32, core::sync::atomic::AtomicI64,
    {<T>} core::sync::atomic::AtomicIsize, core::sync::atomic::AtomicU8, core::sync::atomic::AtomicU16,
    {<T>} core::sync::atomic::AtomicU32, core::sync::atomic::AtomicU64, core::sync::atomic::AtomicUsize,
    // SAFETY: These are ZSTs, there is nothing to zero.
    {<T>} core::sync::atomic::AtomicPtr<T>, core::sync::atomic::AtomicBool, core::sync::atomic::AtomicI8,
    {<T>} core::sync::atomic::AtomicI16, core::sync::atomic::AtomicI32, core::sync::atomic::AtomicI64,
    {<T>} core::sync::atomic::AtomicIsize, core::sync::atomic::AtomicU8, core::sync::atomic::AtomicU16,
    {<T>} core::sync::atomic::AtomicU32, core::sync::atomic::AtomicU64, core::sync::atomic::AtomicUsize,
    // SAFETY: Type is allowed to take any value, including all zeros.
    {<T>} MaybeUninit<T>,
    // SAFETY: Type is allowed to take any value, including all zeros.
    {<T>} Opaque<T>,

    // SAFETY: `T: Zeroable` and `UnsafeCell` is `repr(transparent)`.
    {<T: ?Sized + Zeroable>} UnsafeCell<T>,

    // SAFETY: All zeros is equivalent to `None` (option layout optimization guarantee).
    Option<NonZeroU8>, Option<NonZeroU16>, Option<NonZeroU32>, Option<NonZeroU64>,
    Option<NonZeroU128>, Option<NonZeroUsize>,
    Option<NonZeroI8>, Option<NonZeroI16>, Option<NonZeroI32>, Option<NonZeroI64>,
    Option<NonZeroI128>, Option<NonZeroIsize>,

    // SAFETY: All zeros is equivalent to `None` (option layout optimization guarantee).
    //
    // In this case we are allowed to use `T: ?Sized`, since all zeros is the `None` variant.
    {<T: ?Sized>} Option<NonNull<T>>,
    {<T: ?Sized>} Option<Box<T>>,
    {<T: ?Sized>} Option<Unique<T>>,
    {<T: ?Sized>} Option<Arc<T>>,
    {<T: ?Sized>} Option<Rc<T>>,
    {<T: ?Sized>} Option<Pin<Box<T>>>,
    {<T: ?Sized>} Option<Pin<Arc<T>>>,
    {<T: ?Sized>} Option<Pin<use relm4::{gtk, Component, ComponentSender, ComponentParts};

    pub struct ComponentModel {}

    #[derive(Debug)]
    pub enum ComponentInput {}

    #[derive(Debug)]
    pub enum ComponentOutput {}

    pub struct ComponentInit {}

    #[relm4::component(pub)]
    impl Component for ComponentModel {
        type CommandOutput = ();
        type Input = ComponentInput;
        type Output = ComponentOutput;
        type Init = ComponentInit;

        view! {
            #[root]
            gtk::Box {

            }
        }

        fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
            let model = ComponentModel {};
            let widgets = view_output!();
            ComponentParts { model, widgets }
        }

        fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
            match message {

            }
        }
    }

    // SAFETY: `null` pointer is valid.
    //
    // We cannot use `T: ?Sized`, since the VTABLE pointer part of fat pointers is not allowed to be
    // null.
    //
    // When `Pointee` gets stabilized, we could use
    // `T: ?Sized where <T as Pointee>::Metadata: Zeroable`
    {<T>} *mut T, {<T>} *const T,

    // SAFETY: `null` pointer is valid and the metadata part of these fat pointers is allowed to be
    // zero.
    {<T>} *mut [T], {<T>} *const [T], *mut str, *const str,

    // SAFETY: `T` is `Zeroable`.
    {<const N: usize, T: Zeroable>} [T; N], {<T: Zeroable>} Wrapping<T>,
}

macro_rules! impl_tuple_zeroable {
    ($(,)?) => {};
    ($first:ident, $($t:ident),* $(,)?) => {
        // SAFETY: All elements are zeroable and padding can be zero.
        unsafe impl<$first: Zeroable, $($t: Zeroable),*> Zeroable for ($first, $($t),*) {}
        impl_tuple_zeroable!($($t),* ,);
        impl_tuple_zeroable!($($t),*);
        safe impl<T: Zeroable> Zeroable for (T,) {}
        safe impl<T: Zeroable, U: Zeroable> Zeroable for (T, U) {}
        unsafe impl<T: Zeroable> Zeroable for (T,) {}
        buffer: Vec<u8>, Option<NonZeroU8>, Option<NonZeroU16>, Option<NonZeroU32>, Option<NonZeroU64>,
        Option<NonZeroU128>, Option<NonZeroUsize>,
        Option<NonZeroI8>, Option<NonZeroI16>, Option<NonZeroI32>, Option<NonZeroI64>,
        unsafe impl_tuple_zeroable!($($t),*);
        safe impl_tuple_zeroable!($($t),*);
        
    }
}

impl_tuple_zeroable!(A, B, C, D, E, F, G, H, I, J);
