use robusta_semantic::Semantic;

use crate::{LintDiagnostic, disable_directives::Suppressions};

/// Borrowed file state and diagnostic sink for a rule visit.
pub struct ContextHost<'ast, 'nodes, 'diagnostics> {
    pub(crate) semantic: &'nodes Semantic<'ast>,
    pub(crate) suppressions: &'nodes Suppressions<'ast>,
    pub(crate) diagnostics: &'diagnostics mut Vec<LintDiagnostic>,
}
