// Directory adapted from https://github.com/oxc-project/oxc/tree/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter_core/src/write

pub mod arguments;
pub mod buffer;
pub mod builders;
pub mod extensions;
pub mod formatter;
mod macros;
pub mod state;
#[cfg(test)]
mod write_path_tests;

use crate::Formatter;

/// Formatting trait for types that can create a formatted representation.
///
/// This is the formatting-IR counterpart to [`std::fmt::Display`]: an
/// implementation appends [`FormatElement`](crate::FormatElement)s to its
/// [`Formatter`] rather than writing text.
/// `C` is the language formatter's own [`FormatContext`](crate::FormatContext)
/// implementation, and `'ast` is the arena lifetime the written elements
/// borrow from.
///
/// Formatting yields no value: the write path has no error type, and a
/// document that does not hold together is reported later, when it is printed.
///
/// ## Example
///
/// Implementing `Format` for a custom struct, using the
/// [`write!`](macro@crate::write) macro and the builders in
/// [`crate::builders`]:
///
/// ```text
/// use robusta_formatter_core::builders::{hard_line_break, text};
/// use robusta_formatter_core::{Format, FormatContext, Formatter, write};
///
/// struct Paragraph<'ast>(&'ast str);
///
/// impl<'ast, C: FormatContext> Format<'ast, C> for Paragraph<'ast> {
///     fn fmt(&self, f: &mut Formatter<'_, 'ast, C>) {
///         write!(f, [hard_line_break(), text(self.0), hard_line_break()]);
///     }
/// }
/// ```
///
/// [`text`](crate::builders::text) borrows for `'ast`, so content that the
/// implementor owns has to be interned into the arena first; see
/// [`arena_cow_str`](crate::arena_cow_str).
pub trait Format<'ast, C> {
    /// Formats the object using the given formatter.
    fn fmt(&self, f: &mut Formatter<'_, 'ast, C>);
}

impl<'ast, C, T> Format<'ast, C> for &T
where
    T: ?Sized + Format<'ast, C>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_, 'ast, C>) {
        Format::fmt(&**self, f);
    }
}

impl<'ast, C, T> Format<'ast, C> for &mut T
where
    T: ?Sized + Format<'ast, C>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_, 'ast, C>) {
        Format::fmt(&**self, f);
    }
}

impl<'ast, C, T> Format<'ast, C> for Option<T>
where
    T: Format<'ast, C>,
{
    fn fmt(&self, f: &mut Formatter<'_, 'ast, C>) {
        if let Some(value) = self {
            value.fmt(f);
        }
    }
}

impl<C> Format<'_, C> for () {
    #[inline]
    fn fmt(&self, _: &mut Formatter<'_, '_, C>) {
        // Intentionally left empty
    }
}
