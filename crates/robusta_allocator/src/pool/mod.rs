// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_allocator/src/pool/mod.rs

use std::{mem::ManuallyDrop, ops::Deref};

use crate::Allocator;

mod standard;
use standard::StandardAllocatorPool;

/// A thread-safe pool for reusing [`Allocator`] instances to reduce allocation overhead.
pub struct AllocatorPool(StandardAllocatorPool);

impl AllocatorPool {
    /// Create a new [`AllocatorPool`] for use across the specified number of threads.
    pub fn new(thread_count: usize) -> AllocatorPool {
        Self(StandardAllocatorPool::new(thread_count))
    }

    /// Retrieve an [`Allocator`] from the pool, or create a new one if the pool is empty.
    ///
    /// Returns an [`AllocatorGuard`] that gives access to the allocator.
    ///
    /// # Panics
    ///
    /// Panics if the underlying mutex is poisoned.
    pub fn get(&self) -> AllocatorGuard<'_> {
        AllocatorGuard { allocator: ManuallyDrop::new(self.0.get()), pool: self }
    }

    /// Add an [`Allocator`] to the pool.
    ///
    /// The `Allocator` is reset by this method, so it's ready to be re-used.
    ///
    /// # Panics
    ///
    /// Panics if the underlying mutex is poisoned.
    fn add(&self, allocator: Allocator) {
        self.0.add(allocator);
    }
}

/// A guard object representing exclusive access to an [`Allocator`] from the pool.
///
/// On drop, the `Allocator` is reset and returned to the pool.
pub struct AllocatorGuard<'alloc_pool> {
    allocator: ManuallyDrop<Allocator>,
    pool: &'alloc_pool AllocatorPool,
}

impl Deref for AllocatorGuard<'_> {
    type Target = Allocator;

    fn deref(&self) -> &Self::Target {
        &self.allocator
    }
}

impl Drop for AllocatorGuard<'_> {
    /// Return [`Allocator`] back to the pool.
    fn drop(&mut self) {
        // SAFETY: After taking ownership of the `Allocator`, we do not touch the `ManuallyDrop` again
        let allocator = unsafe { ManuallyDrop::take(&mut self.allocator) };
        self.pool.add(allocator);
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn a_returned_allocator_is_empty_and_keeps_its_memory() {
        let pool = AllocatorPool::new(1);
        let capacity = {
            let allocator = pool.get();
            allocator.alloc_str("first file");
            assert!(allocator.used_bytes() > 0);
            allocator.capacity()
        };

        let allocator = pool.get();
        assert_eq!(allocator.used_bytes(), 0);
        assert_eq!(allocator.capacity(), capacity);
    }

    #[test]
    fn overlapping_leases_get_separate_allocators() {
        let pool = AllocatorPool::new(1);
        let first = pool.get();
        let second = pool.get();
        let first_text = first.alloc_str("first");
        let second_text = second.alloc_str("second");

        assert_eq!(first_text, "first");
        assert_eq!(second_text, "second");
        assert_eq!(first.used_bytes(), "first".len());
        assert_eq!(second.used_bytes(), "second".len());
    }

    #[test]
    fn concurrent_workers_never_share_or_outgrow_their_leases() {
        const WORKERS: usize = 4;

        let pool = AllocatorPool::new(WORKERS);
        thread::scope(|scope| {
            for worker in 0..WORKERS {
                let pool = &pool;
                scope.spawn(move || {
                    let text = worker.to_string().repeat(1000);
                    for _ in 0..1000 {
                        let allocator = pool.get();
                        assert_eq!(allocator.used_bytes(), 0);
                        let copy = allocator.alloc_str(&text);
                        thread::yield_now();
                        assert_eq!(*copy, *text);
                    }
                });
            }
        });

        assert_eq!(pool.0.len(), WORKERS);
    }
}
