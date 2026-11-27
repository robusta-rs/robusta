use robusta_allocator::Allocator;
use robusta_diagnostics::{Severity, SourceLocator};
use robusta_semantic::SemanticBuilder;

use crate::{
    FixKind, LintDiagnostic, Linter, Rule, RuleConfiguration, apply_fixes, apply_safe_fixes,
};

/// Runs one rule at `Warning` severity against standalone Java sources.
pub struct Tester {
    rule: &'static dyn Rule,
    configuration: serde_json::Value,
}

impl Tester {
    pub(crate) fn new(rule: &'static dyn Rule) -> Self {
        Self { rule, configuration: serde_json::Value::Null }
    }

    /// Change the rule configuration.
    pub(crate) fn configured(self, configuration: serde_json::Value) -> Self {
        Self { configuration, ..self }
    }

    fn lint(&self, source: &str) -> Vec<LintDiagnostic> {
        let configured =
            RuleConfiguration::configured(self.rule, self.configuration.clone(), Severity::Warning)
                .expect("the tested configuration is valid");
        let diagnostics = lint_source([configured], source);
        for diagnostic in &diagnostics {
            assert_eq!(diagnostic.rule_id(), self.rule.id());
            assert_eq!(diagnostic.diagnostic().severity, Severity::Warning);
        }
        diagnostics
    }

    /// Assert the rule reports nothing for `source`.
    pub(crate) fn passes(&self, source: &str) {
        let diagnostics = self.lint(source);
        assert!(diagnostics.is_empty(), "expected no diagnostics, got {diagnostics:?}");
    }

    /// Render the diagnostics for `source`.
    pub(crate) fn reports(&self, source: &str) -> String {
        let diagnostics = self.lint(source);
        assert!(!diagnostics.is_empty(), "expected diagnostics");
        render(source, &diagnostics)
    }

    /// Assert the safe fixes turn `source` into `expected`, and render the
    /// diagnostics for a snapshot.
    ///
    /// `expected` is exact because fixed code can start or end in whitespace
    /// an inline snapshot cannot pin.
    pub(crate) fn fixes(&self, source: &str, expected: &str) -> String {
        let diagnostics = self.lint(source);
        assert!(!diagnostics.is_empty(), "expected diagnostics with fixes");
        let safe_fixes = diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.fix().is_some_and(|fix| fix.kind == FixKind::SAFE_FIX))
            .count();
        let fixed = apply_safe_fixes(source, &diagnostics).expect("rule fixes should be valid");
        assert_eq!(fixed.applied, safe_fixes);
        assert_eq!(fixed.skipped_overlapping, 0);
        assert_eq!(fixed.code, expected);
        assert!(self.lint(&fixed.code).is_empty(), "fixed source must lint clean");
        render(source, &diagnostics)
    }

    /// Assert `source` is reported without a fix.
    pub(crate) fn reports_without_fix(&self, source: &str) {
        let diagnostics = self.lint(source);
        assert!(!diagnostics.is_empty(), "expected diagnostics");
        assert!(
            diagnostics.iter().all(|diagnostic| diagnostic.fix().is_none()),
            "expected no fixes, got {diagnostics:?}"
        );
    }

    /// Returns the applied count, skipped overlaps, and fixed code.
    fn apply_kind(&self, kind: FixKind, source: &str) -> (usize, usize, String) {
        let diagnostics = self.lint(source);
        let fixed = apply_fixes(source, &diagnostics, kind).expect("rule fixes should be valid");
        (fixed.applied, fixed.skipped_overlapping, fixed.code.into_owned())
    }

    /// Assert `--fix --unsafe-fixes` turns `source` into `expected`.
    pub(crate) fn fixes_unsafe(&self, source: &str, expected: &str) {
        let kind = FixKind::ALL;
        let (applied, skipped, fixed) = self.apply_kind(kind, source);
        assert!(applied >= 1, "expected at least one applied fix");
        assert_eq!(skipped, 0, "expected no skipped overlapping edits");
        assert_eq!(fixed, expected);
        let (_, _, again) = self.apply_kind(kind, &fixed);
        assert_eq!(again, fixed, "fix must be idempotent");
    }

    /// Assert no `kind` fix applies to `source`.
    pub(crate) fn fix_leaves_unchanged(&self, kind: FixKind, source: &str) {
        let (applied, _, fixed) = self.apply_kind(kind, source);
        assert_eq!(applied, 0, "expected no fix of this kind");
        assert_eq!(fixed, source);
    }

    /// Assert the fixes of `kind` alone turn `source` into `expected`.
    pub(crate) fn fixes_only(&self, kind: FixKind, source: &str, expected: &str) {
        let (_, skipped, fixed) = self.apply_kind(kind, source);
        assert_eq!(skipped, 0, "expected no skipped overlapping edits");
        assert_eq!(fixed, expected);
    }

    /// Assert repeated `kind` passes turn `source` into `expected`.
    ///
    /// Overlapping edits need more than one pass.
    pub(crate) fn fixes_to_fixpoint(&self, kind: FixKind, source: &str, expected: &str) {
        let mut current = source.to_owned();
        for _ in 0..8 {
            let (_, _, next) = self.apply_kind(kind, &current);
            if next == current {
                break;
            }
            current = next;
        }
        assert_eq!(current, expected);
        let (_, _, again) = self.apply_kind(kind, &current);
        assert_eq!(again, current, "converged source must be a fixpoint");
    }
}

/// Lint `source` with `rules` at `Warning` severity, for fixes that meet in one pass.
pub fn lint_together(rules: &[&'static dyn Rule], source: &str) -> Vec<LintDiagnostic> {
    lint_source(rules.iter().map(|rule| RuleConfiguration::new(*rule, Severity::Warning)), source)
}

/// Lint `source`, which must parse without errors.
fn lint_source(
    rules: impl IntoIterator<Item = RuleConfiguration>,
    source: &str,
) -> Vec<LintDiagnostic> {
    let allocator = Allocator::new();
    let parsed = robusta_parser::parse(&allocator, source);
    assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
    let semantic = SemanticBuilder::new().build(&parsed.unit);
    Linter::new(rules).lint(&semantic)
}

/// Render each diagnostic on one line with its flagged source text.
fn render(source: &str, diagnostics: &[LintDiagnostic]) -> String {
    use std::fmt::Write;

    let locator = SourceLocator::new(source);
    let mut out = String::new();
    for diagnostic in diagnostics {
        let position = locator.position(diagnostic.span().start);
        let _ = writeln!(
            out,
            "{}:{}: warning[{}]: {} ({:?})",
            position.line,
            position.column,
            diagnostic.rule_id(),
            diagnostic.diagnostic().message,
            diagnostic.span().source_text(source),
        );
    }
    out
}
