// Adapted from https://github.com/oxc-project/oxc/blob/d8abaa1fdbfa3cfbab83f80ee6b9cccbd9fc02ef/crates/oxc_allocator/src/vec2/raw_vec.rs

// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{alloc::Layout, cmp, ptr::NonNull};

use allocator_api2::alloc::{Allocator, handle_alloc_error};

use crate::arena::Arena;

/// The buffer behind an arena vector: a pointer, a 32-bit length, a 32-bit
/// capacity, and the arena they came from.
///
/// Storing the two counters as `u32` is what keeps the whole header at 24
/// bytes on a 64-bit target instead of 32. Nothing reachable is lost by it: a
/// lexed source is capped at `u32::MAX` bytes, and no list produced from that
/// source can hold more elements than the source has bytes.
///
/// The buffer is never freed. It belongs to the arena and is released with it,
/// which is also why growth abandons the old allocation rather than returning
/// it.
#[repr(C)]
pub struct RawVec<'alloc, T> {
    ptr: NonNull<T>,
    len: u32,
    cap: u32,
    arena: &'alloc Arena,
}

impl<'alloc, T> RawVec<'alloc, T> {
    /// An empty buffer that has not allocated. A capacity of zero means
    /// "unallocated", so the dangling pointer is never read through.
    #[inline]
    pub fn new_in(arena: &'alloc Arena) -> Self {
        Self { ptr: NonNull::dangling(), len: 0, cap: 0, arena }
    }

    /// An empty buffer with room for `capacity` elements already allocated.
    #[inline]
    pub fn with_capacity_in(capacity: usize, arena: &'alloc Arena) -> Self {
        let mut raw = Self::new_in(arena);
        if capacity > 0 {
            raw.allocate_exactly(capacity);
        }
        raw
    }

    /// The start of the buffer. Dangling, but aligned and non-null, while the
    /// capacity is zero or the element type is zero-sized.
    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// The length in the width it is stored in, so the push path neither
    /// widens nor narrows it.
    #[inline]
    pub fn len_u32(&self) -> u32 {
        self.len
    }

    /// Record that the buffer now holds `len` elements.
    ///
    /// # Safety
    ///
    /// `len` must not exceed the capacity, and every element below it must be
    /// initialized.
    #[inline]
    pub unsafe fn set_len(&mut self, len: u32) {
        debug_assert!(len <= self.capacity());
        self.len = len;
    }

    /// The number of elements the buffer can hold.
    ///
    /// A zero-sized element type needs no storage, so its capacity is reported
    /// as the largest length the counter can express. That makes growth the
    /// single place a length overflow is caught, for sized and zero-sized
    /// elements alike.
    #[inline]
    pub fn capacity(&self) -> u32 {
        if size_of::<T>() == 0 { u32::MAX } else { self.cap }
    }

    /// Make room for `additional` elements beyond the current length.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        if additional > (self.capacity() - self.len) as usize {
            self.grow_amortized(additional);
        }
    }

    /// Make room for one more element, given that the buffer is exactly full.
    ///
    /// Kept apart from [`RawVec::reserve`] so that pushing inlines to a
    /// comparison and a cold call rather than a growth calculation.
    #[inline]
    pub fn grow_one(&mut self) {
        self.grow_amortized(1);
    }

    /// Double the capacity, or jump straight to the requested size when that is
    /// larger. Doubling is what makes repeated pushing amortized `O(1)`.
    ///
    /// Growth goes through the arena's reallocation rather than a fresh
    /// allocation and a copy. When the buffer happens to be the arena's most
    /// recent allocation, the arena extends it where it lies, which costs no
    /// copy and abandons no bytes. That case is common in the formatter, which
    /// fills a document buffer without allocating anything else in between.
    #[cold]
    #[inline(never)]
    fn grow_amortized(&mut self, additional: usize) {
        let required = self.len().checked_add(additional).unwrap_or_else(|| capacity_overflow());
        let capacity = cmp::max(self.capacity() as usize * 2, required);

        // A zero-sized element type reports the counter's full range as its
        // capacity, so it can only arrive here by overflowing that range, and
        // it never reaches the allocation below.
        if capacity > u32::MAX as usize {
            capacity_overflow();
        }
        debug_assert!(size_of::<T>() > 0);

        let new_layout = Layout::array::<T>(capacity).unwrap_or_else(|_| capacity_overflow());
        self.ptr = match self.allocated_layout() {
            // SAFETY: the buffer was allocated from this arena with exactly
            // `old_layout`, and `new_layout` is larger because the capacity
            // strictly grew. Both layouts share `T`'s alignment.
            Some(old_layout) => unsafe {
                match Allocator::grow(&self.arena, self.ptr.cast(), old_layout, new_layout) {
                    Ok(buffer) => buffer.cast::<T>(),
                    Err(_) => handle_alloc_error(new_layout),
                }
            },
            None => self.arena.alloc_layout(new_layout).cast::<T>(),
        };

        #[expect(clippy::cast_possible_truncation, reason = "bounded by `u32::MAX` above")]
        {
            self.cap = capacity as u32;
        }
    }

    /// Allocate room for exactly `capacity` elements, for a buffer that has
    /// none yet.
    fn allocate_exactly(&mut self, capacity: usize) {
        // A zero-sized element type never allocates, but its length still has
        // to fit the counter, so this check precedes the size branch.
        if capacity > u32::MAX as usize {
            capacity_overflow();
        }

        if size_of::<T>() == 0 {
            return;
        }

        let layout = Layout::array::<T>(capacity).unwrap_or_else(|_| capacity_overflow());
        self.ptr = self.arena.alloc_layout(layout).cast::<T>();
        #[expect(clippy::cast_possible_truncation, reason = "bounded by `u32::MAX` above")]
        {
            self.cap = capacity as u32;
        }
    }

    /// The layout the current buffer was allocated with, or `None` when nothing
    /// has been allocated for it yet.
    fn allocated_layout(&self) -> Option<Layout> {
        if self.cap == 0 || size_of::<T>() == 0 {
            return None;
        }

        // SAFETY: this size and alignment pair already passed `Layout::array`
        // when the buffer was allocated, so it is still a valid layout.
        Some(unsafe {
            Layout::from_size_align_unchecked(size_of::<T>() * self.cap as usize, align_of::<T>())
        })
    }
}

/// One place to report a capacity that the counters cannot represent, so the
/// growth path carries a single panic site instead of one per check.
#[cold]
#[inline(never)]
fn capacity_overflow() -> ! {
    panic!("arena vector capacity overflow")
}
