// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_allocator/src/pool/standard.rs

use std::{iter, sync::Mutex};

use crate::Allocator;

/// A thread-safe pool for reusing [`Allocator`] instances, that uses standard allocators.
pub struct StandardAllocatorPool {
    /// Allocators currently in the pool.
    allocators: Mutex<Vec<Allocator>>,
}

impl StandardAllocatorPool {
    /// Create a new [`StandardAllocatorPool`] for use across the specified number of threads.
    pub fn new(thread_count: usize) -> StandardAllocatorPool {
        let allocators = iter::repeat_with(Allocator::new).take(thread_count).collect();
        StandardAllocatorPool { allocators: Mutex::new(allocators) }
    }

    /// Retrieve an [`Allocator`] from the pool, or create a new one if the pool is empty.
    ///
    /// # Panics
    ///
    /// Panics if the underlying mutex is poisoned.
    pub fn get(&self) -> Allocator {
        let allocator = {
            let mut allocators = self.allocators.lock().unwrap();
            allocators.pop()
        };
        allocator.unwrap_or_else(Allocator::new)
    }

    /// Add an [`Allocator`] to the pool.
    ///
    /// The `Allocator` is reset by this method, so it's ready to be re-used.
    ///
    /// # Panics
    ///
    /// Panics if the underlying mutex is poisoned.
    pub(super) fn add(&self, mut allocator: Allocator) {
        allocator.reset();
        let mut allocators = self.allocators.lock().unwrap();
        allocators.push(allocator);
    }

    /// The number of allocators currently in the pool.
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.allocators.lock().unwrap().len()
    }
}
