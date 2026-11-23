// Adapted from https://github.com/oxc-project/oxc/blob/7db7a29aab2d9cf33df279eda41f2ffa4d9799b8/crates/oxc_allocator/src/take_in.rs

use std::mem;

use crate::{Allocator, GetAllocator, Vec};

/// A trait to replace an existing value with an empty dummy, returning the
/// original.
pub trait TakeIn<'a>: Dummy<'a> {
    /// Replace the value with a dummy.
    #[must_use]
    fn take_in<A: GetAllocator<'a>>(&mut self, allocator_accessor: &A) -> Self {
        let allocator = allocator_accessor.allocator();
        let dummy = Dummy::dummy(allocator);
        mem::replace(self, dummy)
    }
}

impl<'a, T> TakeIn<'a> for Vec<'a, T> {}

/// A trait to create a dummy value.
pub trait Dummy<'a>: Sized {
    /// Create a dummy value.
    fn dummy(allocator: &'a Allocator) -> Self;
}

impl<'a, T> Dummy<'a> for Vec<'a, T> {
    /// Create a dummy [`Vec`].
    #[inline]
    fn dummy(allocator: &'a Allocator) -> Self {
        Vec::new_in(&allocator)
    }
}
