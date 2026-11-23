// Adapted from https://github.com/oxc-project/oxc/blob/5b4787fb0d4d6d6f26fbbb1477e4643a67d7659f/crates/oxc_allocator/src/boxed.rs

use std::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::GetAllocator;

/// A pointer to an arena-allocated value that does not run [`Drop`].
///
/// Arena values are released in bulk with their [`Allocator`](crate::Allocator).
/// Accepting a value that owns resources outside that arena would leak those
/// resources, so [`Box::new_in`] rejects types that need dropping.
#[repr(transparent)]
pub struct Box<'alloc, T: ?Sized>(NonNull<T>, PhantomData<(&'alloc (), T)>);

// SAFETY: a `Box` has exclusive access to the `T` it points to and grants access
// to nothing else, so it takes the auto traits of the `&'alloc mut T` it stands
// in for. It holds no reference to its arena, so it cannot allocate from that
// arena on another thread, and it never frees, so sending it cannot release arena
// memory there. `'alloc` keeps the arena alive while the `Box` exists.
unsafe impl<T: Send + ?Sized> Send for Box<'_, T> {}

// SAFETY: sharing a `&Box<T>` shares only a `&T`, so `T: Sync` is sufficient.
unsafe impl<T: Sync + ?Sized> Sync for Box<'_, T> {}

impl<T: ?Sized> Box<'_, T> {
    const ASSERT_T_IS_NOT_DROP: () =
        assert!(!std::mem::needs_drop::<T>(), "cannot arena-allocate a type that needs Drop");
}

impl<'alloc, T> Box<'alloc, T> {
    /// Allocate `value` in the arena without attaching per-value drop glue.
    #[inline]
    pub fn new_in<A: GetAllocator<'alloc>>(value: T, allocator: &A) -> Self {
        const { Self::ASSERT_T_IS_NOT_DROP };
        Self(NonNull::from(allocator.allocator().alloc(value)), PhantomData)
    }
}

impl<T: ?Sized> Deref for Box<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: the pointer came from an arena allocation holding an initialized
        // `T`, `'alloc` keeps that arena alive, and `&self` rules out a live `&mut`.
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for Box<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: as for `deref`, and `&mut self` gives exclusive access to the `T`.
        unsafe { self.0.as_mut() }
    }
}

impl<T: ?Sized> AsRef<T> for Box<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ?Sized> AsMut<T> for Box<'_, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: ?Sized + Debug> Debug for Box<'_, T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&**self, formatter)
    }
}
