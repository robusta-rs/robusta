use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use robusta_diagnostics::Severity;

/// Configured severity for lint findings.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RuleSeverityConfig {
    Allow,
    Warn,
    Deny,
}

impl RuleSeverityConfig {
    /// The severity findings are reported at, or `None` for `allow`.
    pub const fn severity(self) -> Option<Severity> {
        match self {
            Self::Allow => None,
            Self::Warn => Some(Severity::Warning),
            Self::Deny => Some(Severity::Error),
        }
    }

    /// The setting that reports findings at `severity`, or `allow` for `None`.
    ///
    /// # Panics
    ///
    /// Panics on [`Severity::Advice`], which no lint setting reports at.
    pub fn from_severity(severity: Option<Severity>) -> Self {
        match severity {
            None => Self::Allow,
            Some(Severity::Warning) => Self::Warn,
            Some(Severity::Error) => Self::Deny,
            Some(Severity::Advice) => unreachable!("lint severity must be warning or error"),
        }
    }
}

impl TryFrom<&str> for RuleSeverityConfig {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "allow" => Ok(Self::Allow),
            "warn" => Ok(Self::Warn),
            "deny" => Ok(Self::Deny),
            _ => Err("expected `allow`, `warn`, or `deny`"),
        }
    }
}
