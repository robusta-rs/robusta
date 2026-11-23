// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_allocator/src/convert.rs

#![expect(clippy::inline_always)]

use crate::{Allocator, Vec};

/// This trait works similarly to the standard library [`From`] trait.
///
/// It comes with a blanket implementation for [`IntoIn`], and a reflective implementation.
pub trait FromIn<'a, T>: Sized {
    /// Converts to this type from the input type within the given `allocator`.
    fn from_in(value: T, allocator: &'a Allocator) -> Self;
}

/// This trait works similarly to the standard library [`Into`] trait.
///
/// A [`FromIn`] implementation also implicitly implements [`IntoIn`] for the opposite type.
pub trait IntoIn<'a, T>: Sized {
    /// Converts this type into the (usually inferred) input type within the given `allocator`.
    fn into_in(self, allocator: &'a Allocator) -> T;
}

/// `FromIn` is reflective
impl<'a, T> FromIn<'a, T> for T {
    #[inline(always)]
    fn from_in(t: T, _: &'a Allocator) -> T {
        t
    }
}

/// `FromIn` implicitly implements `IntoIn`.
impl<'a, T, U> IntoIn<'a, U> for T
where
    U: FromIn<'a, T>,
{
    #[inline]
    fn into_in(self, allocator: &'a Allocator) -> U {
        U::from_in(self, allocator)
    }
}

impl<'a, T, const N: usize> FromIn<'a, [T; N]> for Vec<'a, T> {
    #[inline(always)]
    fn from_in(array: [T; N], allocator: &'a Allocator) -> Self {
        Vec::from_iter_in(array, &allocator)
    }
}
