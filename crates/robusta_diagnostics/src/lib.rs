//! Diagnostics, source positions, and report output.
//!
//! [`RobustaDiagnostic`] supplies severity, messages, source labels, help, and
//! notes to the graphical renderer. [`Reporter`] supports streaming and buffered
//! reports. [`Output`] writes stdout and stderr. Concrete report formats live
//! in the CLI.

use std::borrow::Cow;
use std::fmt;

pub use robusta_span::LabeledSpan;
use robusta_span::Span;

mod handlers;
mod named_source;
mod output;
mod protocol;
mod reporter;
mod source_impls;

pub use handlers::{GraphicalReportHandler, GraphicalTheme};
pub use named_source::NamedSource;
pub use output::Output;
pub use protocol::{Diagnostic, Severity, SourceCode};
pub use reporter::{DiagnosticSummary, FileReport, Finding, Reporter, RuleInfo};

/// A source-backed diagnostic, ready to render.
pub type Error = Box<dyn Diagnostic + Send + Sync>;

/// The owned labels attached to a diagnostic. The common one- and two-label
/// cases are stored inline.
pub type Labels = smallvec::SmallVec<[LabeledSpan; 2]>;

/// The result type used across robusta: a value, or a fatal diagnostic.
pub type Result<T> = std::result::Result<T, RobustaDiagnostic>;

/// Describes one problem robusta found: what is wrong, how severe it is, and
/// where in the source it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RobustaDiagnostic {
    pub severity: Severity,
    pub message: String,
    pub labels: Labels,
    pub help: Option<String>,
    pub note: Option<String>,
}

impl RobustaDiagnostic {
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self::new(Severity::Error, message, span)
    }

    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self::new(Severity::Warning, message, span)
    }

    fn new(severity: Severity, message: impl Into<String>, span: Span) -> Self {
        Self {
            severity,
            message: message.into(),
            labels: std::iter::once(unlabeled_span(span)).collect(),
            help: None,
            note: None,
        }
    }

    #[must_use]
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    #[must_use]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Replace the diagnostic's labels with one label.
    #[must_use]
    pub fn with_label(mut self, label: LabeledSpan) -> Self {
        self.labels = std::iter::once(label).collect();
        self
    }

    /// Replace the diagnostic's labels with `labels`.
    #[must_use]
    pub fn with_labels(mut self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self {
        self.labels = labels.into_iter().collect();
        self
    }

    /// Append one label without replacing existing labels.
    #[must_use]
    pub fn and_label(mut self, label: LabeledSpan) -> Self {
        self.labels.push(label);
        self
    }

    /// Append labels without replacing existing labels.
    #[must_use]
    pub fn and_labels(mut self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self {
        self.labels.extend(labels);
        self
    }

    /// The location used by fixes, suppressions, positions, and report formats
    /// that support only one source range.
    pub fn primary_span(&self) -> Span {
        self.labels
            .iter()
            .find(|label| label.primary())
            .or_else(|| self.labels.first())
            .map_or_else(|| Span::empty(0), LabeledSpan::span)
    }

    /// Remap every label through a source-coordinate translation.
    pub fn map_spans(&mut self, mut map: impl FnMut(Span) -> Span) {
        for label in &mut self.labels {
            label.set_span(map(label.span()));
        }
    }

    /// Add source code to this diagnostic so a renderer can quote it. Use
    /// [`NamedSource`] when a presentation path is available.
    pub fn with_source_code<T: SourceCode + 'static>(self, source: T) -> Error {
        Box::new(DiagnosticWithSource { diagnostic: self, source_code: source })
    }
}

impl fmt::Display for RobustaDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(formatter)
    }
}

impl std::error::Error for RobustaDiagnostic {}

impl Diagnostic for RobustaDiagnostic {
    fn help(&self) -> Option<Cow<'_, str>> {
        self.help.as_deref().map(Cow::Borrowed)
    }

    fn note(&self) -> Option<Cow<'_, str>> {
        self.note.as_deref().map(Cow::Borrowed)
    }

    fn severity(&self) -> Option<Severity> {
        Some(self.severity)
    }

    fn labels(&self) -> &[LabeledSpan] {
        &self.labels
    }
}

/// A diagnostic paired with the source its labels point into.
pub(crate) struct DiagnosticWithSource<D, S> {
    pub(crate) diagnostic: D,
    pub(crate) source_code: S,
}

impl<D: fmt::Debug, S> fmt::Debug for DiagnosticWithSource<D, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.diagnostic, f)
    }
}

impl<D: fmt::Display, S> fmt::Display for DiagnosticWithSource<D, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.diagnostic, f)
    }
}

impl<D: fmt::Debug + fmt::Display, S> std::error::Error for DiagnosticWithSource<D, S> {}

impl<D: Diagnostic, S: SourceCode> Diagnostic for DiagnosticWithSource<D, S> {
    fn code(&self) -> Option<Cow<'_, str>> {
        self.diagnostic.code()
    }

    fn severity(&self) -> Option<Severity> {
        self.diagnostic.severity()
    }

    fn help(&self) -> Option<Cow<'_, str>> {
        self.diagnostic.help()
    }

    fn note(&self) -> Option<Cow<'_, str>> {
        self.diagnostic.note()
    }

    fn labels(&self) -> &[LabeledSpan] {
        self.diagnostic.labels()
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.source_code)
    }
}

/// Create an unlabeled source span for a diagnostic.
pub fn unlabeled_span(span: Span) -> LabeledSpan {
    LabeledSpan::underline(span)
}

/// Create a labeled secondary source span for a diagnostic.
pub fn labeled_span(span: Span, message: impl Into<String>) -> LabeledSpan {
    LabeledSpan::at(span, message)
}

/// Create the primary labeled source span for a diagnostic.
pub fn primary_labeled_span(span: Span, message: impl Into<String>) -> LabeledSpan {
    LabeledSpan::new_primary_with_span(Some(message.into()), span)
}

/// A one-based source position suitable for diagnostic output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

/// A finding's one-based start and exclusive end positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl SourceRange {
    /// A range covering one position, for a finding whose producer reports a
    /// location without a width.
    pub const fn point(position: SourcePosition) -> Self {
        Self { start: position, end: position }
    }
}

/// Maps byte offsets to diagnostic positions with a reusable line-start index.
pub struct SourceLocator<'source> {
    source: &'source str,
    line_starts: Vec<usize>,
}

impl<'source> SourceLocator<'source> {
    pub fn new(source: &'source str) -> Self {
        let bytes = source.as_bytes();
        let mut line_starts = vec![0];
        for index in memchr::memchr2_iter(b'\r', b'\n', bytes) {
            // A `\r\n` pair's line start was pushed at the `\r`.
            if bytes[index] == b'\n' && index > 0 && bytes[index - 1] == b'\r' {
                continue;
            }
            line_starts
                .push(index + robusta_syntax::line_terminator::terminator_len(source, index));
        }
        Self { source, line_starts }
    }

    /// Convert a byte offset into a one-based line and Unicode-scalar column.
    ///
    /// Clamps to the source end, then back to a character boundary.
    pub fn position(&self, offset: u32) -> SourcePosition {
        let mut offset = (offset as usize).min(self.source.len());
        while !self.source.is_char_boundary(offset) {
            offset -= 1;
        }

        let line = self.line_starts.partition_point(|&start| start <= offset);
        let line_start = self.line_starts[line - 1];
        let column = self.source[line_start..offset].chars().count() + 1;
        SourcePosition { line, column }
    }

    /// Convert a byte span into a one-based range with an exclusive end.
    pub fn range(&self, span: Span) -> SourceRange {
        SourceRange { start: self.position(span.start), end: self.position(span.end) }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RobustaDiagnostic, SourceLocator, SourcePosition, SourceRange, labeled_span,
        primary_labeled_span,
    };
    use robusta_span::Span;

    #[test]
    fn source_position_counts_java_line_terminators() {
        let source = "a\nb\r\nc\rd";
        let locator = SourceLocator::new(source);

        assert_eq!(locator.position(0), SourcePosition { line: 1, column: 1 });
        assert_eq!(locator.position(2), SourcePosition { line: 2, column: 1 });
        assert_eq!(locator.position(5), SourcePosition { line: 3, column: 1 });
        assert_eq!(locator.position(7), SourcePosition { line: 4, column: 1 });
    }

    #[test]
    fn source_position_separates_adjacent_terminators() {
        // `\r\n`: one break. `\n\r` and `\n\r\n`: two breaks.
        let locator = SourceLocator::new("a\r\n\r\nb");
        assert_eq!(locator.position(5), SourcePosition { line: 3, column: 1 });

        let locator = SourceLocator::new("a\n\rb");
        assert_eq!(locator.position(3), SourcePosition { line: 3, column: 1 });

        let locator = SourceLocator::new("a\n\r\nb");
        assert_eq!(locator.position(4), SourcePosition { line: 3, column: 1 });

        let locator = SourceLocator::new("\n\n");
        assert_eq!(locator.position(2), SourcePosition { line: 3, column: 1 });
    }

    #[test]
    fn source_range_resolves_both_ends_of_a_span() {
        let source = "class A {\n    int b;\n}\n";
        let locator = SourceLocator::new(source);

        let range = locator.range(Span::new(14, 20));

        assert_eq!(range.start, SourcePosition { line: 2, column: 5 });
        assert_eq!(range.end, SourcePosition { line: 2, column: 11 });
        assert_eq!(locator.range(Span::new(0, 20)).end, range.end);
        assert_eq!(locator.range(Span::empty(14)), SourceRange::point(range.start));
    }

    #[test]
    fn source_position_counts_unicode_scalars_as_columns() {
        let source = "αβx";
        let locator = SourceLocator::new(source);

        assert_eq!(locator.position(4), SourcePosition { line: 1, column: 3 });
    }

    #[test]
    fn source_position_clamps_invalid_offsets() {
        let source = "é";
        let locator = SourceLocator::new(source);

        assert_eq!(locator.position(1), SourcePosition { line: 1, column: 1 });
        assert_eq!(locator.position(u32::MAX), SourcePosition { line: 1, column: 2 });
    }

    #[test]
    fn primary_span_prefers_the_primary_label_then_the_first_label() {
        let first = Span::new(2, 4);
        let primary = Span::new(8, 10);
        let diagnostic = RobustaDiagnostic::error("problem", first).with_labels([
            labeled_span(first, "related"),
            primary_labeled_span(primary, "problem here"),
        ]);

        assert_eq!(diagnostic.primary_span(), primary);
        assert_eq!(RobustaDiagnostic::error("problem", first).primary_span(), first);
    }

    #[test]
    fn mapping_spans_preserves_label_text_and_primary_state() {
        let mut diagnostic = RobustaDiagnostic::error("problem", Span::new(2, 4)).with_labels([
            labeled_span(Span::new(2, 4), "related"),
            primary_labeled_span(Span::new(8, 10), "problem here"),
        ]);

        diagnostic.map_spans(|span| Span::new(span.start + 100, span.end + 200));

        let labels = &diagnostic.labels;
        assert_eq!(labels[0].span(), Span::new(102, 204));
        assert_eq!(labels[0].label(), Some("related"));
        assert!(!labels[0].primary());
        assert_eq!(labels[1].span(), Span::new(108, 210));
        assert_eq!(labels[1].label(), Some("problem here"));
        assert!(labels[1].primary());
        assert_eq!(diagnostic.primary_span(), Span::new(108, 210));
    }
}
