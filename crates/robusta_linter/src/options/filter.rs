// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_linter/src/options/filter.rs
use crate::{Rule, RuleCategory, RuleId, RuleSeverityConfig, rules};

/// Enables, disables, and sets the severity of lint rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LintFilter {
    severity: RuleSeverityConfig,
    kind: LintFilterKind,
}

impl LintFilter {
    #[must_use]
    pub const fn allow(kind: LintFilterKind) -> Self {
        Self { severity: RuleSeverityConfig::Allow, kind }
    }

    #[must_use]
    pub const fn warn(kind: LintFilterKind) -> Self {
        Self { severity: RuleSeverityConfig::Warn, kind }
    }

    #[must_use]
    pub const fn deny(kind: LintFilterKind) -> Self {
        Self { severity: RuleSeverityConfig::Deny, kind }
    }

    pub const fn severity(self) -> RuleSeverityConfig {
        self.severity
    }

    pub const fn kind(self) -> LintFilterKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintFilterKind {
    /// Every registered rule.
    All,
    Category(RuleCategory),
    Rule(RuleId),
}

impl LintFilterKind {
    pub const ALL: &'static str = "all";

    /// The rule, category, or `all` that `name` names.
    pub fn from_name(name: &str) -> Option<Self> {
        if name == Self::ALL {
            return Some(Self::All);
        }
        if let Some(category) = RuleCategory::from_name(name) {
            return Some(Self::Category(category));
        }
        rules::builtin_rules()
            .iter()
            .find(|rule| rule.id().as_str() == name)
            .map(|rule| Self::Rule(rule.id()))
    }

    pub(crate) fn matches(self, rule: &dyn Rule) -> bool {
        match self {
            Self::All => true,
            Self::Category(category) => rule.meta().category == category,
            Self::Rule(id) => rule.id() == id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LintFilterKind;
    use crate::{RuleCategory, rules};

    #[test]
    fn every_registered_rule_and_category_is_named() {
        assert_eq!(LintFilterKind::from_name("all"), Some(LintFilterKind::All));
        for rule in rules::builtin_rules() {
            assert_eq!(
                LintFilterKind::from_name(rule.id().as_str()),
                Some(LintFilterKind::Rule(rule.id()))
            );
        }
        for category in RuleCategory::ALL {
            assert_eq!(
                LintFilterKind::from_name(category.as_str()),
                Some(LintFilterKind::Category(category))
            );
        }
        for name in ["uppercase-long-suffixes", "", "no-debugger", "performance", "ALL"] {
            assert_eq!(LintFilterKind::from_name(name), None, "{name:?}");
        }
    }

    #[test]
    fn a_filter_matches_exactly_the_rules_it_names() {
        let rule = *rules::builtin_rules().first().expect("the registry has rules");
        assert!(LintFilterKind::All.matches(rule));
        assert!(LintFilterKind::Rule(rule.id()).matches(rule));
        assert!(LintFilterKind::Category(rule.meta().category).matches(rule));
        assert!(
            !rules::builtin_rules()
                .iter()
                .filter(|other| other.id() != rule.id())
                .any(|other| LintFilterKind::Rule(rule.id()).matches(*other))
        );
    }
}
