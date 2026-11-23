// Copied from https://github.com/oxc-project/oxc/blob/36009ddf684d73dedaf10685544dcca924e1cfbb/crates/oxc_allocator/src/accessor.rs

use crate::Allocator;

/// Accessor for getting the underlying allocator.
pub trait GetAllocator<'a> {
    /// Get the underlying allocator.
    fn allocator(&self) -> &'a Allocator;
}

impl<'a> GetAllocator<'a> for &'a Allocator {
    #[inline]
    fn allocator(&self) -> &'a Allocator {
        self
    }
}
