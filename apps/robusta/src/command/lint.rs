use std::str::FromStr;

use bpaf::{Bpaf, Parser};
use robusta_diagnostics::Severity;
use robusta_linter::{FixKind, LintFilterKind, RuleCategory, RuleSeverityConfig};

/// Fix options:
#[derive(Debug, Clone, Copy, Bpaf)]
pub struct FixOptions {
    /// Fix as many issues as possible. Only unfixed issues are reported.
    #[bpaf(long)]
    pub fix: bool,
    /// Include unsafe fixes, which may change what the code does or stop it compiling, in what
    /// --fix writes and what --diff and --list-different preview.
    #[bpaf(long)]
    pub unsafe_fixes: bool,
}

impl FixOptions {
    /// The fix kinds a write or a preview applies.
    pub const fn kind(self) -> FixKind {
        if self.unsafe_fixes { FixKind::ALL } else { FixKind::SAFE_FIX }
    }
}

/// One ordered severity override. The variant carries the severity the flag
/// selects, so a `many` over this parser preserves the relative order of
/// different flags, not just repeats of one.
#[derive(Debug, Clone, Copy, Bpaf)]
enum LintFilter {
    Allow(
        /// Disable a rule, a category, or `all` for this run.
        #[bpaf(short('A'), long("allow"), argument::<String>("NAME"), parse(filter_kind))]
        LintFilterKind,
    ),
    Warn(
        /// Set a rule, a category, or `all` to warn for this run.
        #[bpaf(short('W'), long("warn"), argument::<String>("NAME"), parse(filter_kind))]
        LintFilterKind,
    ),
    Deny(
        /// Set a rule, a category, or `all` to deny for this run.
        #[bpaf(short('D'), long("deny"), argument::<String>("NAME"), parse(filter_kind))]
        LintFilterKind,
    ),
}

impl LintFilter {
    const fn into_filter(self) -> robusta_linter::LintFilter {
        match self {
            Self::Allow(kind) => robusta_linter::LintFilter::allow(kind),
            Self::Warn(kind) => robusta_linter::LintFilter::warn(kind),
            Self::Deny(kind) => robusta_linter::LintFilter::deny(kind),
        }
    }
}

/// Names resolve while arguments are parsed, so a filter that matches no rule
/// fails the run instead of quietly linting nothing.
#[expect(clippy::needless_pass_by_value, reason = "bpaf passes each argument by value")]
fn filter_kind(name: String) -> Result<LintFilterKind, String> {
    LintFilterKind::from_name(&name).ok_or_else(|| {
        // bpaf prefixes this with the offending argument, so the message
        // names what would have been accepted instead of repeating it.
        let categories = RuleCategory::ALL.map(RuleCategory::as_str).join(", ");
        format!(
            "not a rule ID, a category ({categories}), or `{}`; see `robusta lint --rules`",
            LintFilterKind::ALL
        )
    })
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(group_help(filter_help()))]
pub struct FilterOptions {
    #[bpaf(external(lint_filter), map(LintFilter::into_filter), many)]
    pub filters: Vec<robusta_linter::LintFilter>,
}

fn filter_help() -> bpaf::Doc {
    let categories = RuleCategory::ALL
        .map(|category| format!("  * `{}` - {}", category.as_str(), category.description()))
        .join("\n");
    bpaf::Doc::from(
        format!(
            "Rule filter options, applied over the configuration, left to right:\n\n\
             For example `-D style -A uppercase-long-suffix`.\n  \
             The categories are:\n{categories}\n  \
             * `all` - All registered rules.\n\n\
             Arguments:"
        )
        .as_str(),
    )
}

// `lint`'s baseline updates. Not a doc comment, which bpaf would render as a help group header:
// `check` lists its own `--prune-suppressions` among its other options, and so does `lint`.
#[derive(Debug, Clone, Copy, Bpaf)]
pub struct SuppressionOptions {
    /// Record each file's error counts per rule in `robusta-suppressions.json`. With `--fix`,
    /// fix first and record the errors that remain.
    #[bpaf(long)]
    pub suppress_all: bool,
    /// Lower or remove obsolete counts in `robusta-suppressions.json`; never raise them.
    #[bpaf(long)]
    pub prune_suppressions: bool,
}

/// Unused-suppression reporting requested on the command line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportUnusedSuppressions {
    /// Neither flag was passed, so the main configuration decides.
    Configured,
    /// `--report-unused-suppressions`: report at warning severity.
    WithoutSeverity,
    /// `--report-unused-suppressions-severity`; `None` turns the audit off.
    WithSeverity(Option<Severity>),
}

impl ReportUnusedSuppressions {
    /// Resolve the audit severity against the main configuration.
    #[must_use]
    pub const fn severity(self, configured: Option<Severity>) -> Option<Severity> {
        match self {
            Self::Configured => configured,
            Self::WithoutSeverity => Some(Severity::Warning),
            Self::WithSeverity(severity) => severity,
        }
    }

    /// The flag that asked for the audit, when one did.
    #[must_use]
    pub const fn requesting_flag(self) -> Option<&'static str> {
        match self {
            Self::WithoutSeverity => Some("--report-unused-suppressions"),
            Self::WithSeverity(Some(_)) => Some("--report-unused-suppressions-severity"),
            Self::Configured | Self::WithSeverity(None) => None,
        }
    }
}

/// The two flags are alternatives, so passing both is a usage error. Neither
/// parser succeeds without its flag, so an invalid severity is reported as
/// such instead of falling back to the configuration.
pub(super) fn report_unused_suppressions() -> impl Parser<ReportUnusedSuppressions> {
    let without_severity = bpaf::long("report-unused-suppressions")
        .help(
            "Report suppressions like `@SuppressWarnings(\"robusta:<rule>\")` and \
             `// robusta-disable-line` that match no finding. --fix removes Robusta's own entries \
             and keeps shared Java warning names.",
        )
        .req_flag(ReportUnusedSuppressions::WithoutSeverity);
    let with_severity = bpaf::long("report-unused-suppressions-severity")
        .help(
            "Same as --report-unused-suppressions, at severity allow, warn, or deny. Overrides \
             the configuration.",
        )
        .argument::<String>("SEVERITY")
        .parse(|severity| {
            RuleSeverityConfig::try_from(severity.as_str()).map(RuleSeverityConfig::severity)
        })
        .map(ReportUnusedSuppressions::WithSeverity);
    bpaf::construct!([without_severity, with_severity])
        .fallback(ReportUnusedSuppressions::Configured)
}

/// Warning options:
#[derive(Debug, Clone, Bpaf)]
pub struct WarningOptions {
    /// Disable reporting on warnings; only errors are reported.
    #[bpaf(long)]
    pub quiet: bool,
    /// Ensure warnings produce a non-zero exit code.
    #[bpaf(long)]
    pub deny_warnings: bool,
    /// Fail when warning-level rule violations exceed this threshold.
    #[bpaf(long, argument("INT"))]
    pub max_warnings: Option<usize>,
    /// Do not display any diagnostics.
    #[bpaf(long)]
    pub silent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugOption {
    /// Report how long each enabled rule spent on the run.
    Timings,
}

impl DebugOption {
    const TIMINGS: &'static str = "timings";
}

impl FromStr for DebugOption {
    type Err = String;

    fn from_str(option: &str) -> Result<Self, Self::Err> {
        match option {
            Self::TIMINGS => Ok(Self::Timings),
            _ => Err(format!("'{option}' is not a known debug option")),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DebugOptions(Vec<DebugOption>);

impl DebugOptions {
    pub fn contains(&self, option: DebugOption) -> bool {
        self.0.contains(&option)
    }
}

impl FromStr for DebugOptions {
    type Err = String;

    fn from_str(options: &str) -> Result<Self, Self::Err> {
        options
            .split(',')
            .filter(|option| !option.is_empty())
            .map(DebugOption::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }
}

pub(super) fn debug_options() -> impl Parser<DebugOptions> {
    bpaf::long("debug")
        .help("Enable comma-separated debug options: timings reports per-rule lint timing.")
        .argument::<DebugOptions>("OPTIONS")
        .fallback(DebugOptions::default())
}

#[cfg(test)]
mod tests {
    use robusta_linter::{FixKind, LintFilter, LintFilterKind, RuleCategory, rules};

    use crate::command::{CheckCommand, Command, LintCommand, command};

    fn lint_filters(args: &[&'static str]) -> Vec<LintFilter> {
        match command().run_inner(args).expect("arguments should parse") {
            Command::Lint(LintCommand { filter_options, .. }) => filter_options.filters,
            command => panic!("expected a lint command, got {command:?}"),
        }
    }

    fn check_filters(args: &[&'static str]) -> Vec<LintFilter> {
        match command().run_inner(args).expect("arguments should parse") {
            Command::Check(CheckCommand { filter_options, .. }) => filter_options.filters,
            command => panic!("expected a check command, got {command:?}"),
        }
    }

    fn rejected_selector(name: &str) -> String {
        super::filter_kind(name.to_string()).expect_err("an unknown selector should be rejected")
    }

    /// The relative order of *different* flags is the whole contract: a `many`
    /// over each flag separately would keep repeats in order but lose whether
    /// a `-D` came before or after an `-A` that overlaps it.
    #[test]
    fn rule_filters_keep_their_relative_order_across_different_flags() {
        assert_eq!(
            lint_filters(&[
                "lint",
                "-D",
                "style",
                "--allow",
                "uppercase-long-suffix",
                "-W",
                "all",
                "."
            ]),
            [
                LintFilter::deny(LintFilterKind::Category(RuleCategory::Style)),
                LintFilter::allow(LintFilterKind::Rule(rules::UPPERCASE_LONG_SUFFIX_ID)),
                LintFilter::warn(LintFilterKind::All),
            ]
        );
        // Reversing the two overlapping filters is a different request, so the
        // parsed order has to reverse with it.
        assert_eq!(
            lint_filters(&["lint", "-A", "uppercase-long-suffix", "-D", "style", "."]),
            [
                LintFilter::allow(LintFilterKind::Rule(rules::UPPERCASE_LONG_SUFFIX_ID)),
                LintFilter::deny(LintFilterKind::Category(RuleCategory::Style)),
            ]
        );
    }

    #[test]
    fn check_accepts_the_same_rule_filters_and_defaults_to_none() {
        assert_eq!(
            check_filters(&["check", "--deny", "suspicious", "."]),
            [LintFilter::deny(LintFilterKind::Category(RuleCategory::Suspicious))]
        );
        assert_eq!(check_filters(&["check", "."]).len(), 0);
        assert_eq!(lint_filters(&["lint", "."]).len(), 0);
    }

    #[test]
    fn an_unknown_selector_is_rejected_and_names_the_accepted_vocabulary() {
        // A near miss of a real rule ID, an empty argument, a rule and a
        // category Robusta does not have, and a case-mismatched `all`.
        for name in ["uppercase-long-suffixes", "", "no-debugger", "performance", "ALL"] {
            let message = rejected_selector(name);
            assert!(
                message.contains("not a rule ID")
                    && message.contains("style")
                    && message.contains("suspicious")
                    && message.contains("lint --rules"),
                "{name:?} produced an unhelpful message: {message}"
            );
        }
        // And the flag surfaces it rather than accepting an unusable filter.
        let failure = command()
            .run_inner(&["lint", "-D", "uppercase-long-suffixes", "."])
            .expect_err("an unknown selector should fail the run");
        assert_ne!(failure.clone().exit_code(), 0);
        assert!(failure.unwrap_stderr().contains("not a rule ID"));
    }

    #[test]
    fn every_registered_rule_and_category_is_selectable_by_name() {
        for rule in rules::builtin_rules() {
            assert_eq!(
                lint_filters(&["lint", "-D", rule.id().as_str(), "."]),
                [LintFilter::deny(LintFilterKind::Rule(rule.id()))]
            );
        }
        for category in RuleCategory::ALL {
            assert_eq!(
                lint_filters(&["lint", "-W", category.as_str(), "."]),
                [LintFilter::warn(LintFilterKind::Category(category))]
            );
        }
    }

    #[test]
    fn unsafe_fixes_widen_the_safe_fixes() {
        let kind = |fix, unsafe_fixes| super::FixOptions { fix, unsafe_fixes }.kind();
        assert_eq!(kind(false, false), FixKind::SAFE_FIX);
        assert_eq!(kind(true, false), FixKind::SAFE_FIX);
        assert_eq!(kind(false, true), FixKind::ALL);
        assert_eq!(kind(true, true), FixKind::ALL);
    }
}
