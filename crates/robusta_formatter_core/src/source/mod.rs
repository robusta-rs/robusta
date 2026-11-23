// Directory adapted from https://github.com/oxc-project/oxc/tree/625401da4335f9f8b0a9f6370a46c090ea9475f1/crates/oxc_formatter_core/src/source
//! Source-side access mechanics: data structures over the input source that
//! make no output decision by themselves.
//!
//! - [`SourceText`]: offset-keyed byte and slice access.
//!
//! The structure decides nothing about the output. Interpretation — what
//! counts as a comment, what a gap means, where trivia is placed — stays
//! consumer-owned, because those answers are language-defined.

mod text;

pub use text::SourceText;
