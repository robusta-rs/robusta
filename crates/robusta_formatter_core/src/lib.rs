// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_formatter_core/src/lib.rs
// `inline(always)` is intentional on the write path's dispatch helpers: the
// `Argument`/`write!` layer exists to monomorphize away, and the printer hits
// these calls once per element.
#![allow(clippy::inline_always)]

//! Language-agnostic formatting infrastructure.
//!
//! This crate provides the core IR and printing infrastructure used by
//! robusta's language formatters. A language formatter implements the
//! `Format` contract over its AST, writes a flat stream of
//! [`format_element::FormatElement`]s through a [`Formatter`], and hands the
//! finished document to the printer, which picks line breaks by measuring
//! groups against the configured line width.
//!
mod context;
pub mod format_element;
mod printer;
mod source;
mod write;

pub use write::builders;

pub use context::options::{
    IndentStyle, IndentWidth, IndentWidthFromIntError, LineEnding, LineWidth, LineWidthFromIntError,
};
pub use context::{FormatContext, FormatOptions};
pub use format_element::debug::DisplayDocument;
pub use format_element::document::Document;
pub use format_element::formatted::Formatted;
pub use format_element::group_id::{GroupId, UniqueGroupIdBuilder};
pub use format_element::tag::{
    self, Align, Condition, DedentMode, Group, GroupMode, Label, LabelId, Tag, TagKind,
};
pub use format_element::{
    BestFittingElement, FormatElement, FormatElements, Interned, LINE_TERMINATORS, LineMode,
    PrintMode, normalize_newlines, text_width,
};
pub use printer::error::{ActualStart, InvalidDocumentError, PrintError};
pub use printer::{PrintResult, PrintWidth, Printed, PrinterOptions};
pub use source::SourceText;
pub use write::Format;
pub use write::arguments::{Argument, Arguments};
pub use write::extensions::{MemoizeFormat, Memoized};
pub use write::formatter::{Formatter, arena_cow_str};
pub use write::state::FormatState;

// `Printer` stays crate-internal: `Document::print` / `print_with_indent` are the
// only print entries, which is what makes print-without-finalize unrepresentable
// outside core.
pub(crate) use context::options::CoreFormatOptions;
pub(crate) use context::simple::SimpleFormatContext;
pub(crate) use printer::Printer;
