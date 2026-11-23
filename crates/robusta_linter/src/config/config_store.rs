use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use robusta_config::GlobSet;
use rustc_hash::FxHashMap;

use crate::{Linter, Rule, RuleConfiguration, RuleId, rules};

/// One `overrides` entry, with its rule values resolved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedOverride {
    pub files: GlobSet,
    pub exclude_files: GlobSet,
    /// Rule values in registry order; `None` turns the rule off.
    pub rules: Vec<(RuleId, Option<RuleConfiguration>)>,
}

impl ResolvedOverride {
    pub fn changes_rules(&self) -> bool {
        !self.rules.is_empty()
    }

    fn is_match(&self, path: &str) -> bool {
        self.files.is_match(path) && !self.exclude_files.is_match(path)
    }
}

/// Linters for the files one configuration governs.
#[derive(Debug)]
pub struct Config {
    /// The rules after the command-line filters, with whether each runs.
    base_rules: Vec<(RuleConfiguration, bool)>,
    /// The linter for files that no `overrides` entry matches.
    base: Arc<Linter>,
    /// The configuration's directory, which `overrides` patterns are relative to.
    base_dir: PathBuf,
    /// The `overrides` entries that change rules.
    overrides: Vec<ResolvedOverride>,
    /// Linters for files that `overrides` entries match, keyed by those entries.
    overridden: FxHashMap<Vec<usize>, Arc<Linter>>,
}

impl Config {
    pub fn new(
        rules: Vec<(RuleConfiguration, bool)>,
        overrides: Vec<ResolvedOverride>,
        base_dir: &Path,
    ) -> Self {
        let enabled =
            rules.iter().filter_map(|(configured, enabled)| enabled.then_some(configured));
        Self {
            base: Arc::new(Linter::new(enabled.cloned())),
            base_rules: rules,
            base_dir: base_dir.to_path_buf(),
            overrides,
            overridden: FxHashMap::default(),
        }
    }

    /// The enabled rules for files that no `overrides` entry matches.
    pub fn rules(&self) -> &[RuleConfiguration] {
        self.base.rules()
    }

    pub fn apply_overrides(&mut self, path: &Path) -> Arc<Linter> {
        if self.overrides.is_empty() {
            return Arc::clone(&self.base);
        }
        let relative = path.strip_prefix(&self.base_dir).unwrap_or(path).to_string_lossy();
        let matching = (0..self.overrides.len())
            .filter(|&index| self.overrides[index].is_match(&relative))
            .collect::<Vec<_>>();
        if matching.is_empty() {
            return Arc::clone(&self.base);
        }
        let Self { base_rules, overrides, overridden, .. } = self;
        let linter = overridden.entry(matching).or_insert_with_key(|matching| {
            let mut values = rules::builtin_rules()
                .iter()
                .map(|rule| (rule.id(), rule_entry(base_rules, *rule).cloned()))
                .collect::<Vec<_>>();
            for &index in matching {
                for (id, value) in &overrides[index].rules {
                    if let Some((_, configured)) = values.iter_mut().find(|(rule, _)| rule == id) {
                        *configured = value.clone().map(|value| (value, true));
                    }
                }
            }
            let enabled = values.into_iter().filter_map(|(_, configured)| {
                configured.and_then(|(configured, enabled)| enabled.then_some(configured))
            });
            Arc::new(Linter::new(enabled))
        });
        Arc::clone(linter)
    }
}

/// The configured rule for `rule`, and whether it runs.
pub(super) fn rule_entry<'a>(
    rules: &'a [(RuleConfiguration, bool)],
    rule: &dyn Rule,
) -> Option<&'a (RuleConfiguration, bool)> {
    rules.iter().find(|(configured, _)| configured.rule().id() == rule.id())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use robusta_diagnostics::Severity;
    use serde_json::json;

    use crate::{ConfigStoreBuilder, LintConfigFile, LintFilter, LintFilterKind, LintOverride};

    fn lint_override(rules: serde_json::Value) -> LintOverride {
        LintOverride {
            files: serde_json::from_value(json!(["A.java"])).expect("files should deserialize"),
            rules: serde_json::from_value(rules).expect("rules should deserialize"),
            ..LintOverride::default()
        }
    }

    #[test]
    fn override_rule_values_apply_after_filters_and_later_entries_win() {
        let overrides = vec![
            lint_override(json!({ "unused-import": "allow", "uppercase-long-suffix": "warn" })),
            lint_override(json!({ "uppercase-long-suffix": "deny" })),
        ];
        let filters = [
            LintFilter::deny(LintFilterKind::from_name("unused-import").expect("rule")),
            LintFilter::allow(LintFilterKind::from_name("uppercase-long-suffix").expect("rule")),
        ];
        let mut config = ConfigStoreBuilder::from_config(LintConfigFile {
            overrides: Some(overrides),
            ..LintConfigFile::default()
        })
        .expect("config should build")
        .with_filters(&filters)
        .build(Path::new(""));
        let enabled = config
            .apply_overrides(Path::new("A.java"))
            .rules()
            .iter()
            .map(|rule| (rule.rule().id().as_str(), rule.severity()))
            .collect::<Vec<_>>();

        assert_eq!(
            enabled,
            [
                ("uppercase-long-suffix", Severity::Error),
                ("unused-local-variable", Severity::Warning)
            ]
        );
    }
}
