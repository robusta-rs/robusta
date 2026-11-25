mod categories;
mod config_builder;
mod config_store;
mod lint_config_file;
mod overrides;
mod rules;

use robusta_diagnostics::Severity;

use crate::RuleSeverityConfig;

pub use categories::LintCategories;
pub use config_builder::{ConfigBuilderError, ConfigStoreBuilder};
pub use config_store::Config;
pub use lint_config_file::{LintConfigFile, OptionsConfig};
pub use overrides::LintOverride;
pub use rules::{LintRules, RuleConfig};

/// The run-wide `lint.options` of a configuration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RunOptions {
    pub deny_warnings: bool,
    pub max_warnings: Option<usize>,
    pub report_unused_suppressions: Option<Severity>,
}

impl From<OptionsConfig> for RunOptions {
    fn from(options: OptionsConfig) -> Self {
        let OptionsConfig { deny_warnings, max_warnings, report_unused_suppressions } = options;
        Self {
            deny_warnings: deny_warnings.unwrap_or(false),
            max_warnings,
            report_unused_suppressions: report_unused_suppressions
                .and_then(RuleSeverityConfig::severity),
        }
    }
}
