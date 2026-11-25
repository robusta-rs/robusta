use std::{fmt, path::Path};

use robusta_diagnostics::Severity;

use crate::{LintFilter, Rule, RuleCategory, RuleConfiguration, RuleId, RuleSeverityConfig, rules};

use super::{
    Config, LintConfigFile, LintOverride, LintRules, OptionsConfig, RuleConfig, RunOptions,
    config_store::{ResolvedOverride, rule_entry},
};

/// The rules and run-wide options one configuration sets, before
/// command-line filters and overrides.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigStoreBuilder {
    /// Each enabled rule and each `allow` rule with settings, in registry
    /// order, with whether it runs.
    rules: Vec<(RuleConfiguration, bool)>,
    /// The rule values of each `overrides` entry, in configuration order.
    overrides: Vec<ResolvedOverride>,
    options: RunOptions,
}

impl Default for ConfigStoreBuilder {
    fn default() -> Self {
        Self {
            rules: rules::builtin_rules()
                .iter()
                .filter_map(|rule| {
                    default_category_severity(rule.meta().category)
                        .severity()
                        .map(|severity| (RuleConfiguration::new(*rule, severity), true))
                })
                .collect(),
            overrides: Vec::new(),
            options: RunOptions::default(),
        }
    }
}

impl ConfigStoreBuilder {
    /// # Errors
    ///
    /// Returns [`ConfigBuilderError`] for an unknown category or rule, or an
    /// invalid rule value.
    pub fn from_config(config: LintConfigFile) -> Result<Self, ConfigBuilderError> {
        // File selection reads `ignore-patterns` before any linter is built.
        let LintConfigFile {
            categories,
            rules: mut rule_configs,
            overrides,
            options,
            ignore_patterns: _,
        } = config;
        for category in categories.keys() {
            if RuleCategory::from_name(category).is_none() {
                return Err(ConfigBuilderError::UnknownCategory(category.clone()));
            }
        }
        let mut rules = Vec::new();
        for rule in rules::builtin_rules() {
            let config = rule_configs.remove(rule.id().as_str()).unwrap_or_else(|| {
                let category = rule.meta().category;
                RuleConfig::Severity(
                    categories
                        .get(category.as_str())
                        .copied()
                        .unwrap_or_else(|| default_category_severity(category)),
                )
            });
            let (configured, enabled) = configure_rule(*rule, config)?;
            // An `allow` rule without settings is the same as one left out.
            if enabled || configured.rule().to_configuration().is_some() {
                rules.push((configured, enabled));
            }
        }
        reject_unknown_rules(&rule_configs)?;
        let overrides = overrides
            .unwrap_or_default()
            .into_iter()
            .map(resolve_override)
            .collect::<Result<_, _>>()?;
        Ok(Self { rules, overrides, options: options.into() })
    }

    pub fn options(&self) -> RunOptions {
        self.options
    }

    /// Apply each filter in order to the result of the previous one.
    #[must_use]
    pub fn with_filters(mut self, filters: &[LintFilter]) -> Self {
        if filters.is_empty() {
            return self;
        }
        let builtin = rules::builtin_rules();
        let mut entries =
            builtin.iter().map(|rule| rule_entry(&self.rules, *rule).cloned()).collect::<Vec<_>>();

        for filter in filters {
            let severity = filter.severity().severity();
            for (rule, entry) in builtin.iter().zip(&mut entries) {
                if filter.kind().matches(*rule) {
                    let configured = entry.take();
                    *entry = severity.map(|severity| {
                        let configured = configured.map_or_else(
                            || RuleConfiguration::new(*rule, severity),
                            |(configured, _)| configured.with_severity(severity),
                        );
                        (configured, true)
                    });
                }
            }
        }

        self.rules = entries.into_iter().flatten().collect();
        self
    }

    /// Build the linters for the files under `base_dir`, which `overrides`
    /// patterns are relative to.
    pub fn build(self, base_dir: &Path) -> Config {
        let overrides =
            self.overrides.into_iter().filter(ResolvedOverride::changes_rules).collect();
        Config::new(self.rules, overrides, base_dir)
    }

    /// The resolved options and rules, at `allow` for every rule that does not run.
    pub fn resolve_final_config_file(&self) -> LintConfigFile {
        let rule_configs = LintRules::new(
            rules::builtin_rules()
                .iter()
                .map(|rule| {
                    let entry = rule_entry(&self.rules, *rule)
                        .map(|(configured, enabled)| (configured, *enabled));
                    (rule.id().as_str().to_string(), rule_config(entry))
                })
                .collect(),
        );
        let RunOptions { deny_warnings, max_warnings, report_unused_suppressions } = self.options;
        let overrides = self
            .overrides
            .iter()
            .map(|resolved| LintOverride {
                files: resolved.files.clone(),
                exclude_files: resolved.exclude_files.clone(),
                rules: LintRules::new(
                    resolved
                        .rules
                        .iter()
                        .map(|(id, value)| {
                            let entry = value.as_ref().map(|configured| (configured, true));
                            (id.as_str().to_string(), rule_config(entry))
                        })
                        .collect(),
                ),
            })
            .collect();
        LintConfigFile {
            rules: rule_configs,
            overrides: Some(overrides),
            options: OptionsConfig {
                deny_warnings: Some(deny_warnings),
                max_warnings,
                report_unused_suppressions: Some(RuleSeverityConfig::from_severity(
                    report_unused_suppressions,
                )),
            },
            ..LintConfigFile::default()
        }
    }
}

fn resolve_override(config: LintOverride) -> Result<ResolvedOverride, ConfigBuilderError> {
    let LintOverride { files, exclude_files, rules: mut rule_configs } = config;
    let mut rules = Vec::new();
    for rule in rules::builtin_rules() {
        if let Some(config) = rule_configs.remove(rule.id().as_str()) {
            let (configured, enabled) = configure_rule(*rule, config)?;
            rules.push((rule.id(), enabled.then_some(configured)));
        }
    }
    reject_unknown_rules(&rule_configs)?;
    Ok(ResolvedOverride { files, exclude_files, rules })
}

/// Reject the first of `rule_configs` left after the built-in rules took theirs.
fn reject_unknown_rules(rule_configs: &LintRules) -> Result<(), ConfigBuilderError> {
    match rule_configs.keys().next() {
        Some(rule) => Err(ConfigBuilderError::UnknownRule(rule.clone())),
        None => Ok(()),
    }
}

/// The rule configured by `config`, and whether it runs.
fn configure_rule(
    rule: &dyn Rule,
    config: RuleConfig,
) -> Result<(RuleConfiguration, bool), ConfigBuilderError> {
    let (severity, settings) = match config {
        RuleConfig::Severity(severity) => (severity, None),
        RuleConfig::WithSettings(severity, settings) => (severity, Some(settings)),
    };
    if settings.is_some() && !rule.has_config() {
        return Err(ConfigBuilderError::UnexpectedSettings(rule.id()));
    }
    let value = settings.map_or(serde_json::Value::Null, serde_json::Value::Object);
    let configured = RuleConfiguration::configured(rule, value, Severity::Warning)
        .map_err(|error| ConfigBuilderError::InvalidSettings { rule: rule.id(), error })?;
    Ok(match severity.severity() {
        Some(severity) => (configured.with_severity(severity), true),
        None => (configured, false),
    })
}

/// A rule's value as `lint.rules` writes it, at `allow` when the rule does not run.
fn rule_config(entry: Option<(&RuleConfiguration, bool)>) -> RuleConfig {
    let severity = entry.and_then(|(configured, enabled)| enabled.then(|| configured.severity()));
    let severity = RuleSeverityConfig::from_severity(severity);
    match entry.and_then(|(configured, _)| configured.rule().to_configuration()) {
        Some(serde_json::Value::Object(settings)) => RuleConfig::WithSettings(severity, settings),
        _ => RuleConfig::Severity(severity),
    }
}

const fn default_category_severity(category: RuleCategory) -> RuleSeverityConfig {
    match category {
        RuleCategory::Correctness => RuleSeverityConfig::Warn,
        _ => RuleSeverityConfig::Allow,
    }
}

#[derive(Debug)]
pub enum ConfigBuilderError {
    UnknownCategory(String),
    UnknownRule(String),
    /// Settings for a rule that takes none.
    UnexpectedSettings(RuleId),
    InvalidSettings {
        rule: RuleId,
        error: serde_json::Error,
    },
}

impl fmt::Display for ConfigBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCategory(category) => write!(f, "unknown category {category:?}"),
            Self::UnknownRule(rule) => write!(f, "unknown rule {rule:?}"),
            Self::UnexpectedSettings(rule) => {
                write!(f, "rule {:?} has no settings", rule.as_str())
            }
            Self::InvalidSettings { rule, error } => write!(f, "rule {:?}: {error}", rule.as_str()),
        }
    }
}

impl std::error::Error for ConfigBuilderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidSettings { error, .. } => Some(error),
            Self::UnknownCategory(_) | Self::UnknownRule(_) | Self::UnexpectedSettings(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use robusta_diagnostics::Severity;
    use serde_json::json;

    use super::ConfigStoreBuilder;
    use crate::{LintFilter, LintFilterKind, RuleCategory, rules};

    fn builder(lint: serde_json::Value) -> ConfigStoreBuilder {
        let lint = serde_json::from_value(lint).expect("lint config should deserialize");
        ConfigStoreBuilder::from_config(lint).expect("lint config should build")
    }

    /// The enabled rules of `lint` after `filters`, as `(id, severity)` pairs
    /// in the order the built config presents them.
    fn filtered(
        lint: &ConfigStoreBuilder,
        filters: &[LintFilter],
    ) -> Vec<(&'static str, Severity)> {
        lint.clone()
            .with_filters(filters)
            .build(std::path::Path::new(""))
            .rules()
            .iter()
            .map(|configured| (configured.rule().id().as_str(), configured.severity()))
            .collect()
    }

    fn kind(name: &str) -> LintFilterKind {
        LintFilterKind::from_name(name).expect("filter should name a rule")
    }

    fn deny(name: &str) -> LintFilter {
        LintFilter::deny(kind(name))
    }

    fn warn(name: &str) -> LintFilter {
        LintFilter::warn(kind(name))
    }

    fn allow(name: &str) -> LintFilter {
        LintFilter::allow(kind(name))
    }

    /// `-W`/`-D` upsert and `-A` removes, so a filter both re-grades a
    /// configured rule and enables one the config never mentioned.
    #[test]
    fn filters_upsert_on_warn_and_deny_and_remove_on_allow() {
        let lint = &builder(json!({
            "categories": { "correctness": "allow" },
            "rules": { "uppercase-long-suffix": "warn" },
        }));

        assert_eq!(filtered(lint, &[]), [("uppercase-long-suffix", Severity::Warning)]);
        // Re-grade a configured rule.
        assert_eq!(
            filtered(lint, &[deny("uppercase-long-suffix")]),
            [("uppercase-long-suffix", Severity::Error)]
        );
        // Enable one the config never mentioned, keeping the configured one.
        assert_eq!(
            filtered(lint, &[deny("modifier-order")]),
            [("uppercase-long-suffix", Severity::Warning), ("modifier-order", Severity::Error)]
        );
        // Remove a configured rule.
        assert_eq!(filtered(lint, &[allow("uppercase-long-suffix")]).len(), 0);
        // Allowing a rule that was never enabled is a no-op, not an error.
        assert_eq!(
            filtered(lint, &[allow("modifier-order")]),
            [("uppercase-long-suffix", Severity::Warning)]
        );
    }

    /// Each filter sees the previous one's result, which is the only thing
    /// that makes these two orderings mean different runs.
    #[test]
    fn filters_apply_in_order_so_a_later_one_overrides_an_earlier_one() {
        let lint = ConfigStoreBuilder::default();

        let style_except_one = filtered(&lint, &[deny("style"), allow("uppercase-long-suffix")]);
        assert_ne!(style_except_one.len(), 0);
        assert!(!style_except_one.iter().any(|(id, _)| *id == "uppercase-long-suffix"));

        let reversed = filtered(&lint, &[allow("uppercase-long-suffix"), deny("style")]);
        assert_eq!(
            reversed.len(),
            style_except_one.len() + 1,
            "reversing the flags should leave the allowed rule enabled"
        );
        assert!(reversed.contains(&("uppercase-long-suffix", Severity::Error)));

        // `-A all` clears the set, so what follows it is the whole rule set.
        assert_eq!(
            filtered(&lint, &[deny("all"), allow("all"), deny("uppercase-long-suffix")]),
            [("uppercase-long-suffix", Severity::Error)]
        );
    }

    #[test]
    fn a_category_filter_selects_exactly_its_category_and_all_selects_everything() {
        let lint = ConfigStoreBuilder::default();
        let builtin = rules::builtin_rules();

        assert_eq!(filtered(&lint, &[warn("all")]).len(), builtin.len());
        for category in RuleCategory::ALL {
            let selected = filtered(&lint, &[allow("all"), warn(category.as_str())]);
            let expected = builtin
                .iter()
                .filter(|rule| rule.meta().category == category)
                .map(|rule| (rule.id().as_str(), Severity::Warning))
                .collect::<Vec<_>>();
            assert_eq!(selected, expected, "{} selected the wrong rules", category.as_str());
        }
    }

    /// Whatever order the filters arrive in, the enabled set stays in the
    /// registry's own order so diagnostics and `config` stay stable.
    #[test]
    fn filters_preserve_the_registry_order_of_the_enabled_set() {
        let lint = ConfigStoreBuilder::default();
        let registry =
            rules::builtin_rules().iter().map(|rule| rule.id().as_str()).collect::<Vec<_>>();

        let forward = filtered(&lint, &[deny("all")]);
        let piecemeal =
            filtered(&lint, &registry.iter().rev().map(|id| deny(id)).collect::<Vec<_>>());

        assert_eq!(forward.iter().map(|(id, _)| *id).collect::<Vec<_>>(), registry);
        assert_eq!(piecemeal, forward);
    }
}
