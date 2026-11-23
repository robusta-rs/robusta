// Adapted from https://github.com/oxc-project/oxc/blob/5b4787fb0d4d6d6f26fbbb1477e4643a67d7659f/crates/oxc_allocator/src/allocator_api2.rs

use std::{alloc::Layout, ptr::NonNull};

use allocator_api2::alloc::{AllocError, Allocator as AllocatorApi};

use crate::Allocator;

// SAFETY: every method delegates to the backing arena's implementation of the
// same trait, whose preconditions the caller already upholds. The borrow keeps
// the arena alive while collections hold its allocations.
unsafe impl AllocatorApi for &Allocator {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        AllocatorApi::allocate(&self.arena(), layout)
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // SAFETY: the caller supplies a live allocation and its original layout.
        unsafe { AllocatorApi::deallocate(&self.arena(), ptr, layout) };
    }

    #[inline]
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        // SAFETY: the caller upholds `shrink`'s contract for this allocation.
        unsafe { AllocatorApi::shrink(&self.arena(), ptr, old_layout, new_layout) }
    }

    #[inline]
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        // SAFETY: the caller upholds `grow`'s contract for this allocation.
        unsafe { AllocatorApi::grow(&self.arena(), ptr, old_layout, new_layout) }
    }

    #[inline]
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        // SAFETY: the caller upholds `grow_zeroed`'s contract for this allocation.
        unsafe { AllocatorApi::grow_zeroed(&self.arena(), ptr, old_layout, new_layout) }
    }
}
