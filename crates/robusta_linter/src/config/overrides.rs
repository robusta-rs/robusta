use robusta_config::GlobSet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::LintRules;

/// Rule values for the Java files one override matches.
#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct LintOverride {
    /// Glob patterns for the files this override applies to.
    pub files: GlobSet,
    /// Glob patterns for the files this override leaves out.
    #[serde(default, skip_serializing_if = "GlobSet::is_empty")]
    pub exclude_files: GlobSet,
    /// Rule values that replace the ones in effect, as `lint.rules` does.
    #[serde(default)]
    pub rules: LintRules,
}
