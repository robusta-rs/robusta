use robusta_allocator::Allocator;
use robusta_diagnostics::Severity;
use robusta_linter::{FixKind, LintDiagnostic, Linter, RuleConfiguration, apply_fixes, rules};
use robusta_semantic::SemanticBuilder;

/// The outcome of applying fixes at one ceiling.
#[derive(Debug, Clone)]
pub struct FixOutput {
    /// The source after applying every permitted fix.
    pub code: String,
    /// How many edits were applied.
    pub applied: usize,
}

/// Run every built-in rule over `source`.
///
/// # Errors
///
/// Returns a message when the source does not parse cleanly.
pub fn lint(source: &str) -> Result<Vec<LintDiagnostic>, String> {
    let allocator = Allocator::new();
    let result = robusta_parser::parse(&allocator, source);
    if result.status != robusta_parser::ParseStatus::Clean {
        return Err(format!("source did not parse cleanly ({} errors)", result.errors.len()));
    }

    let semantic = SemanticBuilder::new().build(&result.unit);
    let linter = Linter::new(
        rules::builtin_rules().iter().map(|rule| RuleConfiguration::new(*rule, Severity::Warning)),
    );
    Ok(linter.lint(&semantic))
}

/// Apply the fixes in `diagnostics` permitted by `ceiling`, leaving the result
/// unformatted.
///
/// # Errors
///
/// Returns a message when an emitted fix edit falls outside the source.
pub fn fix(
    source: &str,
    diagnostics: &[LintDiagnostic],
    ceiling: FixKind,
) -> Result<FixOutput, String> {
    let fixed = apply_fixes(source, diagnostics, ceiling).map_err(|error| error.to_string())?;
    Ok(FixOutput { code: fixed.code.into_owned(), applied: fixed.applied })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lint_and_fix(source: &str, ceiling: FixKind) -> FixOutput {
        fix(source, &lint(source).unwrap(), ceiling).unwrap()
    }

    #[test]
    fn safe_ceiling_deletes_an_uninitialized_unused_local() {
        let output = lint_and_fix("class A { void m() { int ignored; } }", FixKind::SAFE_FIX);
        assert_eq!(output.applied, 1);
        assert_eq!(output.code, "class A { void m() { } }");
    }

    #[test]
    fn safe_ceiling_leaves_an_unsafe_fix_untouched() {
        let output =
            lint_and_fix("class A { void m() { int ignored = 1 + 2; } }", FixKind::SAFE_FIX);
        assert_eq!(output.applied, 0);
    }

    #[test]
    fn unsafe_ceiling_applies_an_unsafe_fix() {
        let output = lint_and_fix("class A { void m() { int ignored = 1 + 2; } }", FixKind::ALL);
        assert_eq!(output.applied, 1);
        assert_eq!(output.code, "class A { void m() { } }");
    }

    #[test]
    fn unparsable_source_is_reported() {
        assert!(lint("class A { void m(").is_err());
    }
}
