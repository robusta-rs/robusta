// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_allocator/src/allocator.rs

use crate::arena::Arena;

/// Owns the bump arena for one source file at a time.
#[derive(Default)]
pub struct Allocator {
    arena: Arena,
}

impl Allocator {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocate `value` in the arena and return a mutable reference to it.
    #[inline]
    pub fn alloc<T>(&self, value: T) -> &mut T {
        const {
            assert!(!std::mem::needs_drop::<T>(), "cannot arena-allocate a type that needs Drop");
        }
        self.arena.alloc(value)
    }

    /// Move the items of `iter` into the arena and return a mutable slice over
    /// them. Used to relocate a built-up sequence of nodes into arena storage.
    #[inline]
    pub fn alloc_slice_fill_iter<T, I>(&self, iter: I) -> &mut [T]
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        const {
            assert!(!std::mem::needs_drop::<T>(), "cannot arena-allocate a type that needs Drop");
        }
        self.arena.alloc_slice_fill_iter(iter)
    }

    /// Copy `src` into the arena and return an arena-owned string slice.
    #[inline]
    pub fn alloc_str(&self, src: &str) -> &mut str {
        self.arena.alloc_str(src)
    }

    /// Reset this allocator.
    ///
    /// Performs mass deallocation on everything allocated in it by moving the bump cursor back to
    /// the start of its chunk. Keeps only the most recent chunk, which is also the largest, and
    /// returns the others to the system allocator. Does not run any `Drop` implementations.
    #[inline]
    pub fn reset(&mut self) {
        self.arena.reset();
    }

    /// Calculate the total capacity of this [`Allocator`] including all chunks, in bytes.
    ///
    /// This is the memory the allocator owns, not the size of the data allocated in it.
    /// For the latter, use [`used_bytes`](Self::used_bytes).
    #[inline]
    pub fn capacity(&self) -> usize {
        self.arena.allocated_bytes()
    }

    /// Calculate the total size of data used in this [`Allocator`], in bytes.
    ///
    /// This is the memory that has been used, including padding, not the memory the allocator
    /// owns. For the latter, use [`capacity`](Self::capacity).
    #[inline]
    pub fn used_bytes(&self) -> usize {
        self.arena.used_bytes()
    }

    /// The backing arena used by this crate's guarded collection wrappers.
    #[inline]
    pub(crate) fn arena(&self) -> &Arena {
        &self.arena
    }
}
