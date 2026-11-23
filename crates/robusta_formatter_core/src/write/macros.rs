/// Constructs the parameters for the other formatting macros.
///
/// The macro takes a list of values implementing [`Format`](crate::Format) and
/// canonicalizes them into one [`Arguments`](crate::Arguments) value. Each value
/// is stored as a borrowed [`Argument`](crate::Argument) that keeps the concrete
/// type's formatting function, so the list needs no heap allocation and no
/// `dyn Format` vtable.
///
/// Use it wherever several pieces of content have to be passed where one is
/// expected. The builders in [`crate::builders`] take a single `&impl Format`,
/// so nesting content inside one of them goes through this macro:
///
/// ```text
/// write!(f, [group(&format_args!(token("class"), space(), token("A")))]);
/// ```
///
/// [`Arguments`](crate::Arguments) borrows the argument slice the macro builds,
/// and that slice is a temporary of the surrounding expression. Expand the macro
/// directly in the call that consumes it; it cannot be bound to a local first.
///
/// [`write!`](macro@crate::write) is proxied through this macro.
#[macro_export]
macro_rules! format_args {
    ($($value:expr),+ $(,)?) => {
        $crate::Arguments::new(&[
            $(
                $crate::Argument::new(&$value)
            ),+
        ])
    }
}

/// Writes formatted data into a [`Formatter`](crate::Formatter).
///
/// The first argument is the formatter, and the rest is a bracketed list of
/// values implementing [`Format`](crate::Format). Each one is formatted in
/// order and appends its elements to the formatter.
///
/// Writing yields no value: the write path has no error type, and a document
/// that does not hold together is reported later, when it is printed.
///
/// # Examples
///
/// Writing content into a root [`Formatter`](crate::Formatter) across two calls:
///
/// ```text
/// use robusta_allocator::Allocator;
/// use robusta_formatter_core::builders::{space, token};
/// use robusta_formatter_core::{FormatElement, FormatState, Formatter, write};
///
/// let allocator = Allocator::new();
/// let mut state = FormatState::new(context, &allocator);
/// let mut f = Formatter::new(&mut state);
///
/// write!(f, [token("Hello"), space()]);
/// write!(f, [token("World")]);
///
/// assert_eq!(
///     &*f.into_vec(),
///     [
///         FormatElement::token("Hello"),
///         FormatElement::Space,
///         FormatElement::token("World"),
///     ],
/// );
/// ```
///
/// `context` is the language formatter's own
/// [`FormatContext`](crate::FormatContext) implementation; this crate stays
/// language-agnostic and exports no context type of its own.
/// [`Formatter::into_vec`](crate::Formatter::into_vec) returns an arena-backed
/// vector, which is why the comparison goes through a slice.
#[macro_export]
macro_rules! write {
    ($dst:expr, [$($arg:expr),+ $(,)?]) => {{
        let result = $dst.write_fmt($crate::format_args!($($arg),+));
        result
    }};
    ($dst:expr, $arg:expr) => {{
        let result = $dst.write_fmt($crate::format_args!($arg));
        result
    }}
}

/// Prints the first of several variants that fits.
///
/// The variants run from the most flat to the most expanded. The printer
/// measures each variant but the last in [`Flat`] mode, up to its first line
/// break that is not soft, and prints the first that fits in [`Flat`] mode.
/// When none fits, it prints the most expanded variant in [`Expanded`] mode.
/// A variant holding a hard line or an expanded group can therefore still be
/// chosen when its first line fits.
///
/// [`BestFitting`](crate::builders::BestFitting) is an expansion boundary: an
/// expanding element inside a variant does not expand the groups around it.
///
/// [`Flat`]: crate::PrintMode::Flat
/// [`Expanded`]: crate::PrintMode::Expanded
#[macro_export]
macro_rules! best_fitting {
    ($least_expanded:expr, $($tail:expr),+ $(,)?) => {
        $crate::builders::BestFitting::from_arguments_unchecked($crate::format_args!($least_expanded, $($tail),+))
    };
}
