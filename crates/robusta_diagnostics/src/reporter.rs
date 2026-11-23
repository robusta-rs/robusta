//! Diagnostic findings and the [`Reporter`] lifecycle.

use std::borrow::Cow;
use std::fmt;
use std::path::Path;

use robusta_span::{LabeledSpan, Span};

use crate::{
    Diagnostic, Labels, NamedSource, RobustaDiagnostic, Severity, SourceCode, SourcePosition,
    SourceRange,
};

/// Streaming reporters emit from [`report`](Self::report), buffering reporters
/// from [`finish`](Self::finish).
pub trait Reporter {
    /// Returns text to emit now, or `None` when buffering the report.
    fn report(&mut self, report: FileReport) -> Option<String>;

    /// Returns the buffered document, a run-level line, or `None`.
    fn finish(&mut self, summary: DiagnosticSummary) -> Option<String>;
}

/// Diagnostic totals counted before output filtering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticSummary {
    pub warnings: usize,
    pub errors: usize,
}

impl DiagnosticSummary {
    pub fn merge(&mut self, other: Self) {
        self.warnings += other.warnings;
        self.errors += other.errors;
    }
}

/// Rule ID and optional one-sentence description.
#[derive(Debug, Clone, Copy)]
pub struct RuleInfo {
    pub id: &'static str,
    pub summary: Option<&'static str>,
}

/// A finding with a byte span and one-based source range.
#[derive(Debug)]
pub struct Finding {
    rule: RuleInfo,
    diagnostic: RobustaDiagnostic,
    range: SourceRange,
}

impl Finding {
    pub fn new(
        rule: RuleInfo,
        severity: Severity,
        message: impl Into<String>,
        span: Span,
        range: SourceRange,
    ) -> Self {
        Self {
            rule,
            diagnostic: RobustaDiagnostic::warning(message, span).with_severity(severity),
            range,
        }
    }

    /// Construct a finding without projecting away labels, help, or notes.
    pub fn from_diagnostic(
        rule: RuleInfo,
        diagnostic: RobustaDiagnostic,
        range: SourceRange,
    ) -> Self {
        Self { rule, diagnostic, range }
    }

    /// Construct a finding without a source position or label.
    /// Graphical reports show the message alone.
    pub fn without_position(
        rule: RuleInfo,
        severity: Severity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule,
            diagnostic: RobustaDiagnostic {
                severity,
                message: message.into(),
                labels: Labels::new(),
                help: None,
                note: None,
            },
            range: SourceRange::point(SourcePosition { line: 0, column: 0 }),
        }
    }

    /// The rule that raised this finding.
    pub fn rule(&self) -> RuleInfo {
        self.rule
    }

    /// The identifier of the rule that raised this finding.
    pub fn rule_id(&self) -> &'static str {
        self.rule.id
    }

    pub fn severity(&self) -> Severity {
        self.diagnostic.severity
    }

    pub fn message(&self) -> &str {
        &self.diagnostic.message
    }

    pub fn help(&self) -> Option<&str> {
        self.diagnostic.help.as_deref()
    }

    /// The finding's one-based start and exclusive end positions.
    pub fn range(&self) -> SourceRange {
        self.range
    }

    /// Whether this finding has a resolved one-based source position.
    pub fn has_position(&self) -> bool {
        self.range.start.line != 0 && self.range.start.column != 0
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl std::error::Error for Finding {}

impl Diagnostic for Finding {
    fn help(&self) -> Option<Cow<'_, str>> {
        self.diagnostic.help.as_deref().map(Cow::Borrowed)
    }

    fn note(&self) -> Option<Cow<'_, str>> {
        self.diagnostic.note.as_deref().map(Cow::Borrowed)
    }

    fn severity(&self) -> Option<Severity> {
        Some(self.diagnostic.severity)
    }

    fn labels(&self) -> &[LabeledSpan] {
        &self.diagnostic.labels
    }

    fn code(&self) -> Option<Cow<'_, str>> {
        (!self.rule.id.is_empty()).then_some(Cow::Borrowed(self.rule.id))
    }
}

/// Everything a reporter needs to know about one processed file.
#[derive(Debug)]
pub struct FileReport {
    /// Shared source named with the display path, already normalized to forward slashes.
    source: NamedSource<String>,
    /// `Some(false)` means would reformat. `None` means unchecked.
    formatted: Option<bool>,
    findings: Vec<Finding>,
}

impl FileReport {
    pub fn new(
        display_path: &Path,
        source: impl Into<String>,
        formatted: Option<bool>,
        findings: Vec<Finding>,
    ) -> Self {
        let source = NamedSource::new(display_path.to_string_lossy(), source.into());
        FileReport { source, formatted, findings }
    }

    /// The file's verbatim display path.
    pub fn path(&self) -> &str {
        self.source.name()
    }

    /// The source shared by every finding in this report.
    pub fn source(&self) -> &dyn SourceCode {
        &self.source
    }

    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    /// Whether the file is formatted, or `None` when unchecked.
    pub fn formatted(&self) -> Option<bool> {
        self.formatted
    }

    /// Whether the file would be reformatted by the checked-in policy.
    pub fn would_reformat(&self) -> bool {
        self.formatted == Some(false)
    }

    /// Whether the file has a finding or would be reformatted.
    pub fn is_reportable(&self) -> bool {
        !self.findings.is_empty() || self.would_reformat()
    }
}
