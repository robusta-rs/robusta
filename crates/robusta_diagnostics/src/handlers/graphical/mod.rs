// Directory adapted from https://github.com/oxc-project/oxc/tree/ac4785a13909f6db5c986b0a2bc11fc4d8c3cc3c/crates/oxc_diagnostics/src/handlers/graphical

//! The graphical diagnostic renderer.
//!
//! [`GraphicalReportHandler`] turns a [`crate::Diagnostic`] into a richly formatted
//! report: a colored title, the relevant source-code snippets with each label
//! underlined and named, and any help or note text.
//!
//! The rendering pipeline is split across the submodules, roughly in the order
//! output is produced:
//!
//! - [`handler`] — the [`GraphicalReportHandler`] type and its builder API.
//! - [`report`] — the top level: title, help/note, and wrapping.
//! - [`snippet`] — reads the labelled spans and lays out the source snippets.
//! - [`gutter`] — the line-number column and multi-line span gutters.
//! - [`label`] — the underlines and labels drawn under the source text.
//! - [`mod@line`] — the [`Line`](line::Line) model, line splitting, and width math.
//! - [`span`] — [`FancySpan`](span::FancySpan), a styled labelled span.

mod gutter;
mod handler;
mod label;
mod line;
mod report;
mod snippet;
mod span;
mod text;

pub use handler::GraphicalReportHandler;
