use robusta_config::{IgnorePatterns, deserialize_scalar};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::RuleSeverityConfig;

use super::{LintCategories, LintOverride, LintRules};

/// Category severities, rule values, overrides, and run-wide options.
#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct LintConfigFile {
    /// Category severities: correctness defaults to `warn`, other categories to `allow`.
    #[serde(default)]
    pub categories: LintCategories,
    /// Built-in rule IDs mapped to `allow`, `warn`, or `deny`, or to
    /// `[severity, settings]`. Overrides the rule's category severity.
    #[serde(default)]
    pub rules: LintRules,
    /// Rule values for the Java files that match globs. When a file matches
    /// several overrides, the later one wins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<LintOverride>>,
    /// Run-wide options, accepted only in the main configuration.
    #[serde(default, skip_serializing_if = "OptionsConfig::is_empty")]
    pub options: OptionsConfig,
    /// Java files to skip when linting, besides those the top-level
    /// `ignore-patterns` skips.
    ///
    /// A `!` pattern re-includes only a path an earlier entry in this list
    /// skipped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_patterns: Option<IgnorePatterns>,
}

/// Run-wide reporting options. CLI flags take precedence.
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct OptionsConfig {
    /// Fail the run when warnings remain. Equivalent to `--deny-warnings`.
    /// Only the main configuration may set this; defaults to `false`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_scalar"
    )]
    pub deny_warnings: Option<bool>,
    /// Fail the run when more warnings remain than this. `--max-warnings` replaces it.
    /// Only the main configuration may set this; defaults to no limit.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_scalar"
    )]
    pub max_warnings: Option<usize>,
    /// Report suppressions that silence no finding, including those for disabled rules.
    /// Only the main configuration may set this; defaults to `allow`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_scalar"
    )]
    pub report_unused_suppressions: Option<RuleSeverityConfig>,
}

impl OptionsConfig {
    pub fn is_empty(&self) -> bool {
        let Self { deny_warnings, max_warnings, report_unused_suppressions } = self;
        deny_warnings.is_none() && max_warnings.is_none() && report_unused_suppressions.is_none()
    }
}
