// Directory adapted from https://github.com/oxc-project/oxc/tree/625401da4335f9f8b0a9f6370a46c090ea9475f1/crates/oxc_formatter_core/src/context

pub mod options;
pub mod simple;

use crate::{IndentStyle, IndentWidth, LineEnding, LineWidth, PrinterOptions};

/// Language-agnostic formatting context trait.
///
/// Each language formatter defines its own context type implementing this trait.
/// The context provides access to language-specific options and source text.
///
/// Note: The allocator is stored separately in `FormatState`, not in the context,
/// so the `'ast` arena lifetime does not leak into the language-agnostic trait
/// interface.
pub trait FormatContext {
    /// The language-specific format options type.
    type Options: FormatOptions;

    /// Returns the formatting options.
    fn options(&self) -> &Self::Options;

    /// Returns the source text being formatted as a raw string slice.
    fn source_code(&self) -> &str;
}

/// Language-agnostic format options trait.
///
/// Provides the subset of options needed by the core IR printer.
/// Language-specific options (e.g., JS quote style, semicolons) are added by each language's
/// concrete options type.
pub trait FormatOptions {
    /// The indent style (tabs or spaces).
    fn indent_style(&self) -> IndentStyle;

    /// The indent width in characters.
    fn indent_width(&self) -> IndentWidth;

    /// The maximum line width.
    fn line_width(&self) -> LineWidth;

    /// The line ending style.
    fn line_ending(&self) -> LineEnding;

    /// Convert to printer options.
    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::default()
            .with_indent_style(self.indent_style())
            .with_indent_width(self.indent_width())
            .with_line_ending(self.line_ending())
            .with_print_width(self.line_width().into())
    }
}
