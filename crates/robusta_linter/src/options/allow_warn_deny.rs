use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Configured severity for lint findings.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RuleSeverityConfig {
    Allow,
    Warn,
    Deny,
}
