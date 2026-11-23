use robusta_ast::AstType;
use robusta_semantic::Semantic;

mod config;
mod context;
mod disable_directives;
mod fixer;
mod options;
mod rule;
pub mod rules;
mod suppression_pruner;
pub mod table;
#[cfg(test)]
mod tester;
mod timing;
mod utils;

use context::ContextHost;
use disable_directives::Suppressions;
use timing::RuleTimings;

pub use config::{
    Config, ConfigBuilderError, ConfigStoreBuilder, LintCategories, LintConfigFile, LintOverride,
    LintRules, OptionsConfig, RuleConfig, RunOptions,
};
pub use context::LintContext;
pub use disable_directives::UNUSED_SUPPRESSION_ID;
pub use fixer::{Fix, FixError, FixKind, FixResult, apply_fixes, apply_safe_fixes};
pub use options::{LintFilter, LintFilterKind, LintOptions, RuleSeverityConfig};
pub use rule::{
    DeclaredRule, LintDiagnostic, Rule, RuleCategory, RuleConfiguration, RuleFixMeta, RuleId,
    RuleMeta,
};
pub use timing::{RuleTimingRecord, RuleTimingStore};

/// The rule dispatcher for one parsed source file.
#[derive(Debug)]
pub struct Linter {
    rules: Vec<RuleConfiguration>,
    /// `buckets[ty]` = indices into `rules`, in registry order, of the rules
    /// that run on nodes of that [`AstType`].
    buckets: Vec<Vec<usize>>,
}

impl Linter {
    pub fn new(rules: impl IntoIterator<Item = RuleConfiguration>) -> Self {
        let rules: Vec<_> = rules.into_iter().collect();
        let mut buckets = vec![Vec::new(); AstType::COUNT];
        for (index, configured) in rules.iter().enumerate() {
            match configured.rule().node_types() {
                Some(types) => {
                    for ty in AstType::ALL {
                        if types.has(ty) {
                            buckets[ty as usize].push(index);
                        }
                    }
                }
                None => {
                    for bucket in &mut buckets {
                        bucket.push(index);
                    }
                }
            }
        }
        Self { rules, buckets }
    }

    /// Whether no rules are enabled.
    ///
    /// A caller can then skip linting, unless its [`LintOptions`] report unused
    /// suppressions.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// The number of enabled rules.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// The enabled rules and their severities, in registry order.
    pub fn rules(&self) -> &[RuleConfiguration] {
        &self.rules
    }

    /// Run each node, in preorder, through the rules that declared its kind.
    pub fn lint(&self, semantic: &Semantic<'_>) -> Vec<LintDiagnostic> {
        self.lint_with_options(semantic, LintOptions::default())
    }

    /// Run enabled rules and optionally append suppression-hygiene diagnostics.
    pub fn lint_with_options(
        &self,
        semantic: &Semantic<'_>,
        options: LintOptions<'_>,
    ) -> Vec<LintDiagnostic> {
        let suppressions = Suppressions::new(semantic);
        let mut diagnostics = if options.timings.is_some() {
            self.run_rules::<true>(semantic, &suppressions, options.timings)
        } else {
            self.run_rules::<false>(semantic, &suppressions, None)
        };
        if let Some(severity) = options.report_unused_suppressions {
            diagnostics.extend(suppressions.unused_diagnostics(semantic, severity));
        }
        diagnostics
    }

    /// The dispatch loop.
    ///
    /// `TIMINGS` is a const parameter, so an untimed run compiles without the
    /// clock reads, scratch vector, and merge instead of branching per rule call.
    pub(crate) fn run_rules<'ast, const TIMINGS: bool>(
        &self,
        semantic: &Semantic<'ast>,
        suppressions: &Suppressions<'ast>,
        timings: Option<&RuleTimingStore>,
    ) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut measurements = TIMINGS.then(|| RuleTimings::new(self.rules.len()));
        for node in semantic.nodes().iter() {
            for &rule_index in &self.buckets[node.kind().ty() as usize] {
                let configured = &self.rules[rule_index];
                let meta = configured.rule().meta();
                let mut context = LintContext {
                    host: ContextHost { semantic, suppressions, diagnostics: &mut diagnostics },
                    rule_id: meta.id,
                    fix_capabilities: meta.fix,
                    severity: configured.severity(),
                };
                if TIMINGS {
                    measurements
                        .as_mut()
                        .expect("the timed dispatch loop allocates its measurements")
                        .stat(rule_index)
                        .time(|| configured.rule().run(node, &mut context));
                } else {
                    configured.rule().run(node, &mut context);
                }
            }
        }
        if TIMINGS {
            measurements.expect("the timed dispatch loop allocates its measurements").merge_into(
                timings.expect("the timed dispatch loop is selected by a timing store"),
                self.rules.iter().map(|configured| configured.rule().id()),
            );
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use robusta_allocator::Allocator;
    use robusta_diagnostics::Severity;
    use robusta_semantic::SemanticBuilder;

    use super::*;

    /// Triggers every built-in rule, so the two dispatches are compared on real
    /// findings. In `scan`, `unused-local-variable` reports the pattern variable
    /// and must skip the loop variable, resource, and catch parameter, whose
    /// node kinds it does not declare.
    const FIXTURE: &str = "
import java.util.List;
import java.util.Map;
import java.util.Map;

public final class Example {
    static final long COUNT = 10l;;

    Example() {
        super();
        int unused = 1;
        Runnable action = () -> {
            return;
        };
        java.util.function.Function<Integer, Integer> same = (value) -> {
            return value;
        };
    }

    static boolean isBlank(String text) {
        return text == null && text.isEmpty();
    }

    final static void print(List<String> values) {
        for (String value : values) {
            System.out.println(value);
        }
        ;
    }

    static void scan(List<String> values, Object candidate) {
        for (String staleItem : values) {
        }
        try (AutoCloseable staleResource = open()) {
        } catch (Exception failure) {
        }
        if (candidate instanceof String staleText) {
        }
    }
}
";

    /// The pre-bucketing reference dispatch: every rule tests every node.
    fn full_dispatch(linter: &Linter, semantic: &Semantic<'_>) -> Vec<LintDiagnostic> {
        let suppressions = Suppressions::new(semantic);
        let mut diagnostics = Vec::new();
        for node in semantic.nodes().iter() {
            for configured in &linter.rules {
                let meta = configured.rule().meta();
                let mut context = LintContext {
                    host: ContextHost {
                        semantic,
                        suppressions: &suppressions,
                        diagnostics: &mut diagnostics,
                    },
                    rule_id: meta.id,
                    fix_capabilities: meta.fix,
                    severity: configured.severity(),
                };
                configured.rule().run(node, &mut context);
            }
        }
        diagnostics
    }

    #[test]
    fn bucketed_dispatch_matches_full_dispatch() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, FIXTURE);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let unit = &parsed.unit;
        let semantic = SemanticBuilder::new().build(FIXTURE, unit);

        let linter = Linter::new(
            rules::builtin_rules()
                .iter()
                .map(|rule| RuleConfiguration::new(*rule, Severity::Warning)),
        );
        let bucketed = linter.lint(&semantic);
        let full = full_dispatch(&linter, &semantic);

        assert_eq!(format!("{bucketed:#?}"), format!("{full:#?}"));

        let mut reported: Vec<_> =
            bucketed.iter().map(|diagnostic| diagnostic.rule_id().as_str()).collect();
        reported.sort_unstable();
        reported.dedup();
        let mut declared: Vec<_> =
            rules::builtin_rules().iter().map(|rule| rule.id().as_str()).collect();
        declared.sort_unstable();
        assert_eq!(reported, declared, "fixture must trigger every built-in rule");
    }
}
