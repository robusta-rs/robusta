// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_linter/src/context/mod.rs
use std::ops::Deref;

use robusta_diagnostics::{RobustaDiagnostic, Severity};
use robusta_semantic::Semantic;

use crate::fixer::{Fix, FixKind, RuleFix, RuleFixer};
use crate::rule::{LintDiagnostic, RuleFixMeta, RuleId};

mod host;
pub use host::ContextHost;

/// Read-only file and tree state plus the diagnostic sink for one rule visit.
pub struct LintContext<'ast, 'nodes, 'diagnostics> {
    pub(crate) host: ContextHost<'ast, 'nodes, 'diagnostics>,
    pub(crate) rule_id: RuleId,
    /// What the running rule declared it can fix. Every edit reported through
    /// this context is checked against it.
    pub(crate) fix_capabilities: RuleFixMeta,
    pub(crate) severity: Severity,
}

impl<'ast> Deref for LintContext<'ast, '_, '_> {
    type Target = Semantic<'ast>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.host.semantic
    }
}

impl<'ast> LintContext<'ast, '_, '_> {
    pub const fn semantic(&self) -> &Semantic<'ast> {
        self.host.semantic
    }

    /// Report a lint rule violation at the configured severity.
    pub fn diagnostic(&mut self, diagnostic: RobustaDiagnostic) {
        self.push(diagnostic, None);
    }

    /// Report a lint rule violation and provide a safe automatic fix.
    ///
    /// The closure takes a `RuleFixer` and returns the fix, or an empty one
    /// to report without a fix.
    pub fn diagnostic_with_fix<C, F>(&mut self, diagnostic: RobustaDiagnostic, fix: F)
    where
        C: Into<RuleFix>,
        F: FnOnce(RuleFixer<'ast>) -> C,
    {
        self.diagnostic_with_fix_of_kind(diagnostic, FixKind::SAFE_FIX, fix);
    }

    /// Report a lint rule violation and provide a fix of a specific kind.
    ///
    /// # Panics
    ///
    /// In debug builds, when the kind is empty or is not one the rule
    /// declared.
    pub fn diagnostic_with_fix_of_kind<C, F>(
        &mut self,
        diagnostic: RobustaDiagnostic,
        fix_kind: FixKind,
        fix: F,
    ) where
        C: Into<RuleFix>,
        F: FnOnce(RuleFixer<'ast>) -> C,
    {
        let rule_fix = fix(RuleFixer::new(fix_kind, self.source_text())).into();
        self.finish_create_fix(rule_fix, diagnostic);
    }

    /// Non-generic tail of the `diagnostic_with_fix*` family, compiled once.
    fn finish_create_fix(&mut self, rule_fix: RuleFix, diagnostic: RobustaDiagnostic) {
        let kind = rule_fix.kind();
        debug_assert!(
            kind.is_some(),
            "rule `{}` emitted a fix of no kind; report it without one instead",
            self.rule_id,
        );
        debug_assert!(
            self.fix_capabilities.supports_fix(kind),
            "Rule `{}` does not support this fix kind. Did you forget to update `fix:` in \
             declare_robusta_lint?\n\tSupported fix kinds: {}\n\tAttempted fix kind: {}",
            self.rule_id,
            self.fix_capabilities.fix_kind().as_str(),
            kind.as_str(),
        );
        let fix = (!rule_fix.is_empty()).then(|| rule_fix.into_fix(self.source_text()));
        self.push(diagnostic, fix);
    }

    fn push(&mut self, diagnostic: RobustaDiagnostic, fix: Option<Fix>) {
        let diagnostic = diagnostic.with_severity(self.severity);
        let span = diagnostic.primary_span();
        if self.host.suppressions.contains(self.rule_id, span.start) {
            return;
        }
        self.host.diagnostics.push(LintDiagnostic { rule_id: self.rule_id, diagnostic, span, fix });
    }
}

#[cfg(test)]
mod tests {
    use robusta_allocator::Allocator;
    use robusta_semantic::SemanticBuilder;
    use robusta_span::Span;

    use super::*;
    use crate::{LintDiagnostic, disable_directives::Suppressions};

    /// Emit `kind` from a rule that declared `declared`, and return the fixes
    /// that reached the sink.
    fn emit(declared: RuleFixMeta, kind: FixKind) -> Vec<LintDiagnostic> {
        const SOURCE: &str = "class A {}";
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, SOURCE);
        let semantic = SemanticBuilder::new().build(&parsed.unit);
        let suppressions = Suppressions::new(&semantic);
        let mut diagnostics = Vec::new();
        let mut context = LintContext {
            host: ContextHost {
                semantic: &semantic,
                suppressions: &suppressions,
                diagnostics: &mut diagnostics,
            },
            rule_id: RuleId::new("test-rule"),
            fix_capabilities: declared,
            severity: Severity::Warning,
        };
        context.diagnostic_with_fix_of_kind(
            RobustaDiagnostic::warning("test", Span::new(0, 1)),
            kind,
            |fixer| fixer.delete_range(Span::new(0, 1)),
        );
        diagnostics
    }

    #[test]
    #[should_panic(expected = "Attempted fix kind: unsafe_fix")]
    fn emitting_an_undeclared_kind_panics() {
        emit(RuleFixMeta::Fixable(FixKind::SAFE_FIX), FixKind::UNSAFE_FIX);
    }

    #[test]
    #[should_panic(expected = "emitted a fix of no kind")]
    fn an_edit_must_name_a_kind() {
        emit(RuleFixMeta::Fixable(FixKind::SAFE_FIX), FixKind::NONE);
    }

    #[test]
    fn a_declared_kind_passes() {
        let diagnostics = emit(RuleFixMeta::Conditional(FixKind::ALL), FixKind::UNSAFE_FIX);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].fix().map(|fix| fix.kind), Some(FixKind::UNSAFE_FIX));
    }
}
