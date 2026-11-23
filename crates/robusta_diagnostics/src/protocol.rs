// Adapted from https://github.com/oxc-project/oxc/blob/82c924496c2e1c79e05b29e40377f6b703a2cd6a/crates/oxc_diagnostics/src/protocol.rs

//! The diagnostic protocol robusta's renderers read.

use std::borrow::Cow;
use std::error::Error;

use robusta_span::LabeledSpan;

/// Rich metadata a renderer turns into a human-friendly report.
pub trait Diagnostic: Error {
    /// Identifier of the rule or producer behind this diagnostic, rendered in
    /// the report title.
    fn code(&self) -> Option<Cow<'_, str>> {
        None
    }

    /// Diagnostic severity. Renderers use it to pick the title style and icon.
    ///
    /// `None` is treated as [`Severity::Error`].
    fn severity(&self) -> Option<Severity> {
        None
    }

    /// Additional help text related to this diagnostic.
    fn help(&self) -> Option<Cow<'_, str>> {
        None
    }

    /// Supplementary context, separate from help text. Notes mirror rustc's
    /// `= note:` lines and carry information rather than guidance.
    fn note(&self) -> Option<Cow<'_, str>> {
        None
    }

    /// Source code this diagnostic's [`labels`](Diagnostic::labels) point into.
    fn source_code(&self) -> Option<&dyn SourceCode> {
        None
    }

    /// Labels applied to this diagnostic's
    /// [`source_code`](Diagnostic::source_code).
    ///
    /// The diagnostic retains ownership; a renderer only borrows them for the
    /// duration of one report.
    fn labels(&self) -> &[LabeledSpan] {
        &[]
    }
}

/// [`Diagnostic`] severity, deciding how a diagnostic is displayed and counted.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub enum Severity {
    /// Advice for improving the reported code.
    Advice,
    /// A non-fatal warning.
    Warning,
    /// An error. This is the default severity.
    #[default]
    Error,
}

/// Contiguous source text a diagnostic renderer reads excerpts from.
pub trait SourceCode: Send + Sync {
    /// The complete source as bytes.
    fn data(&self) -> &[u8];

    /// The name of this source, if any.
    fn name(&self) -> Option<&str> {
        None
    }
}
