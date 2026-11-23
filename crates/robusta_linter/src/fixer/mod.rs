use std::{borrow::Cow, fmt};

use robusta_span::Span;

use crate::LintDiagnostic;

mod fix;
pub use fix::{CompositeFix, Fix, FixKind, RuleFix};

// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_linter/src/fixer/mod.rs
/// Produces [`RuleFix`] instances.
#[derive(Clone, Copy)]
#[must_use]
pub struct RuleFixer<'a> {
    /// What kind of fixes the factory methods produce.
    kind: FixKind,
    source_text: &'a str,
}

impl<'a> RuleFixer<'a> {
    pub(crate) const fn new(kind: FixKind, source_text: &'a str) -> Self {
        Self { kind, source_text }
    }

    const fn new_fix(&self, fix: CompositeFix) -> RuleFix {
        RuleFix::new(self.kind, fix)
    }

    pub const fn source_text(&self) -> &'a str {
        self.source_text
    }

    /// The source text covered by `span`.
    pub fn source_range(&self, span: Span) -> &'a str {
        span.source_text(self.source_text)
    }

    /// Delete text covered by a [`Span`].
    pub fn delete_range(&self, span: Span) -> RuleFix {
        self.new_fix(CompositeFix::Single(Fix::delete(span)))
    }

    /// Replace a `target` span with a `replacement` string.
    pub fn replace<S: Into<Cow<'static, str>>>(&self, target: Span, replacement: S) -> RuleFix {
        self.new_fix(CompositeFix::Single(Fix::new(replacement, target)))
    }

    /// A fix that does nothing, reported as no fix.
    pub fn noop(&self) -> RuleFix {
        self.new_fix(CompositeFix::None)
    }
}

/// Source after applying every valid, non-overlapping fix through a selected
/// safety tier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixResult<'source> {
    pub code: Cow<'source, str>,
    pub applied: usize,
    pub skipped_overlapping: usize,
}

/// A rule emitted an edit outside the source or across a UTF-8 boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixError {
    pub span: Span,
    pub source_len: usize,
}

impl fmt::Display for FixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid fix span {:?} for source of {} bytes", self.span, self.source_len)
    }
}

impl std::error::Error for FixError {}

/// Apply fixes permitted by `fix_kind` in source order, skipping any edit that
/// overlaps an edit already applied.
///
/// # Errors
///
/// Returns an error when a fix's edit span is inverted, out of bounds, or not
/// on character boundaries within `source`.
pub fn apply_fixes<'source>(
    source: &'source str,
    diagnostics: &[LintDiagnostic],
    fix_kind: FixKind,
) -> Result<FixResult<'source>, FixError> {
    let mut edits = diagnostics
        .iter()
        .filter_map(LintDiagnostic::fix)
        .filter(|fix| fix_kind.can_apply(fix.kind))
        .collect::<Vec<_>>();
    edits.sort_by_key(|edit| (edit.span.start, edit.span.end));

    if edits.is_empty() {
        return Ok(FixResult { code: Cow::Borrowed(source), applied: 0, skipped_overlapping: 0 });
    }

    let mut output = String::with_capacity(source.len());
    let mut last_applied = None;
    let mut applied = 0usize;
    let mut skipped_overlapping = 0usize;

    for edit in edits {
        validate_span(source, edit.span)?;
        if last_applied.is_some_and(|previous| spans_overlap(previous, edit.span)) {
            skipped_overlapping += 1;
            continue;
        }

        let start = edit.span.start as usize;
        let previous_end = last_applied.map_or(0, |span: Span| span.end as usize);
        output.push_str(&source[previous_end..start]);
        output.push_str(&edit.content);
        last_applied = Some(edit.span);
        applied += 1;
    }

    let previous_end = last_applied.map_or(0, |span| span.end as usize);
    output.push_str(&source[previous_end..]);
    Ok(FixResult { code: Cow::Owned(output), applied, skipped_overlapping })
}

/// Apply only safe fixes.
///
/// # Errors
///
/// Returns an error when a fix's edit span is inverted, out of bounds, or not
/// on character boundaries within `source`.
pub fn apply_safe_fixes<'source>(
    source: &'source str,
    diagnostics: &[LintDiagnostic],
) -> Result<FixResult<'source>, FixError> {
    apply_fixes(source, diagnostics, FixKind::SAFE_FIX)
}

fn validate_span(source: &str, span: Span) -> Result<(), FixError> {
    let start = span.start as usize;
    let end = span.end as usize;
    if start > end
        || end > source.len()
        || !source.is_char_boundary(start)
        || !source.is_char_boundary(end)
    {
        return Err(FixError { span, source_len: source.len() });
    }
    Ok(())
}

// Edits arrive sorted by start, so `right` overlaps when it starts before `left` ends.
#[expect(clippy::suspicious_operation_groupings)]
fn spans_overlap(left: Span, right: Span) -> bool {
    right.start < left.end || (left.is_empty() && right.is_empty() && left.start == right.start)
}

#[cfg(test)]
mod tests {
    use robusta_diagnostics::RobustaDiagnostic;

    use super::*;
    use crate::{LintDiagnostic, RuleId};

    fn lint_diagnostic(fix: Fix) -> LintDiagnostic {
        LintDiagnostic {
            rule_id: RuleId::new("test-rule"),
            diagnostic: RobustaDiagnostic::warning("test", fix.span),
            span: fix.span,
            fix: Some(fix),
        }
    }

    fn safe(fix: Fix) -> Fix {
        fix.with_kind(FixKind::SAFE_FIX)
    }

    fn unsafe_fix(fix: Fix) -> Fix {
        fix.with_kind(FixKind::UNSAFE_FIX)
    }

    #[test]
    fn safe_fixes_are_sorted_and_adjacent_edits_both_apply() {
        let diagnostics = vec![
            lint_diagnostic(safe(Fix::delete(Span::new(2, 3)))),
            lint_diagnostic(safe(Fix::delete(Span::new(1, 2)))),
            lint_diagnostic(unsafe_fix(Fix::new("unsafe", Span::new(3, 4)))),
        ];

        let result = apply_safe_fixes("abcde", &diagnostics).expect("fixes should apply");

        assert_eq!(result.code, "ade");
        assert_eq!(result.applied, 2);
        assert_eq!(result.skipped_overlapping, 0);
    }

    #[test]
    fn a_ceiling_admits_exactly_the_kinds_it_names() {
        let diagnostics = vec![
            lint_diagnostic(safe(Fix::new("S", Span::new(0, 1)))),
            lint_diagnostic(unsafe_fix(Fix::new("U", Span::new(2, 3)))),
        ];

        let safe_only =
            apply_fixes("abcdef", &diagnostics, FixKind::SAFE_FIX).expect("safe fixes apply");
        assert_eq!(safe_only.code, "Sbcdef");
        assert_eq!(safe_only.applied, 1);

        let everything =
            apply_fixes("abcdef", &diagnostics, FixKind::ALL).expect("every kind applies");
        assert_eq!(everything.code, "SbUdef");
        assert_eq!(everything.applied, 2);
    }

    #[test]
    fn overlapping_fixes_keep_the_first_source_order_edit() {
        let diagnostics = vec![
            lint_diagnostic(safe(Fix::new("x", Span::new(2, 5)))),
            lint_diagnostic(safe(Fix::new("y", Span::new(1, 3)))),
        ];

        let result = apply_safe_fixes("abcdef", &diagnostics).expect("fixes should apply");

        assert_eq!(result.code, "aydef");
        assert_eq!(result.applied, 1);
        assert_eq!(result.skipped_overlapping, 1);
    }

    #[test]
    fn invalid_utf8_boundary_is_reported() {
        let diagnostics = vec![lint_diagnostic(safe(Fix::delete(Span::new(1, 2))))];

        let error = apply_safe_fixes("aé", &diagnostics).expect_err("span splits UTF-8");

        assert_eq!(error.span, Span::new(1, 2));
        assert_eq!(error.source_len, 3);
    }
}
