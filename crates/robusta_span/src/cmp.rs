// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_span/src/cmp.rs

//! Specialized comparison traits

use robusta_allocator::{Box, Vec};

/// This trait works similarly to [PartialEq] but it gives the liberty of checking the equality of the
/// content loosely.
///
/// This would mean the implementor can skip some parts of the content while doing equality checks.
/// As an example, AST types ignore fields such as [crate::Span] and semantic IDs. They compare what
/// they store: names after escape translation, and literals as written.
///
/// One should always prefer using the [PartialEq] over this since implementations of this trait
/// inherently are slower or in the best-case scenario as fast as the [PartialEq] comparison.
pub trait ContentEq {
    /// This method tests for contents of `self` and `other` to be equal.
    #[must_use]
    fn content_eq(&self, other: &Self) -> bool;

    /// This method tests for contents of `self` and `other` not to be equal.
    /// The default implementation is almost always
    /// sufficient, and should not be overridden without very good reason.
    #[inline]
    #[must_use]
    fn content_ne(&self, other: &Self) -> bool {
        !self.content_eq(other)
    }
}

/// Blanket implementation for [Option] types
impl<T: ContentEq> ContentEq for Option<T> {
    #[inline]
    fn content_eq(&self, other: &Self) -> bool {
        // NOTE: based on the standard library
        // Spelling out the cases explicitly optimizes better than
        // `_ => false`
        #[expect(clippy::match_same_arms)]
        match (self, other) {
            (Some(lhs), Some(rhs)) => lhs.content_eq(rhs),
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }
}

/// Blanket implementation for [`Box`] types
impl<T: ContentEq> ContentEq for Box<'_, T> {
    #[inline]
    fn content_eq(&self, other: &Self) -> bool {
        self.as_ref().content_eq(other.as_ref())
    }
}

/// Blanket implementation for [`Vec`] types.
///
/// # Warning
/// This implementation is slow compared to [PartialEq] for native types which are [Copy] (e.g. `u32`).
/// Prefer comparing the 2 vectors using `==` if they contain such native types (e.g. `Vec<u32>`).
/// <https://godbolt.org/z/54on5sMWc>
impl<T: ContentEq> ContentEq for Vec<'_, T> {
    #[inline]
    fn content_eq(&self, other: &Self) -> bool {
        if self.len() == other.len() {
            !self.iter().zip(other.iter()).any(|(lhs, rhs)| lhs.content_ne(rhs))
        } else {
            false
        }
    }
}

mod content_eq_auto_impls {
    use super::ContentEq;

    macro_rules! content_eq_impl {
        ($($t:ty)*) => ($(
            impl ContentEq for $t {
                #[inline]
                fn content_eq(&self, other: &$t) -> bool { (*self) == (*other) }
                #[inline]
                fn content_ne(&self, other: &$t) -> bool { (*self) != (*other) }
            }
        )*)
    }

    content_eq_impl! {
        char &str
        bool isize usize
        u8 u16 u32 u64 u128
        i8 i16 i32 i64 i128
    }
}
