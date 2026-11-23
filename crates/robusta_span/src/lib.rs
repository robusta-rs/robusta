//! Source spans and related helpers for robusta.
//!
//! A [`Span`] is a byte range into the original source text. Line and column
//! positions are derived when rendering diagnostics.

mod cmp;
mod labeled_span;
mod span;

pub use cmp::ContentEq;
pub use labeled_span::LabeledSpan;
pub use span::{GetSpan, Span};
