use robusta_diagnostics::Severity;

use crate::RuleTimingStore;

mod allow_warn_deny;
mod filter;

pub use allow_warn_deny::RuleSeverityConfig;
pub use filter::{LintFilter, LintFilterKind};

/// Optional lint-run behavior that does not belong to individual rules.
#[derive(Debug, Default, Clone, Copy)]
pub struct LintOptions<'run> {
    /// The severity of unused-suppression findings; `None` skips the audit.
    pub report_unused_suppressions: Option<Severity>,
    /// Where per-rule measurements go. `Some` selects the timed dispatch loop;
    /// `None` — the default — selects one that reads no clock at all.
    pub timings: Option<&'run RuleTimingStore>,
}
