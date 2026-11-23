use std::fmt;
use std::sync::Arc;

use robusta_diagnostics::{RobustaDiagnostic, Severity};
use robusta_semantic::{AstNode, AstTypesBitset};
use robusta_span::Span;

use crate::context::LintContext;
use crate::fixer::{Fix, FixKind};

/// Stable, user-facing identity of one lint rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleId(&'static str);

impl RuleId {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for RuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// One rule-owned diagnostic and its optional source edit.
#[derive(Debug, Clone)]
pub struct LintDiagnostic {
    pub(crate) rule_id: RuleId,
    pub(crate) diagnostic: RobustaDiagnostic,
    pub(crate) span: Span,
    pub(crate) fix: Option<Fix>,
}

impl LintDiagnostic {
    pub const fn rule_id(&self) -> RuleId {
        self.rule_id
    }

    pub const fn diagnostic(&self) -> &RobustaDiagnostic {
        &self.diagnostic
    }

    /// The diagnostic's primary span, resolved once for suppressions and reporters.
    pub const fn span(&self) -> Span {
        self.span
    }

    pub const fn fix(&self) -> Option<&Fix> {
        self.fix.as_ref()
    }
}

/// Broad grouping of a rule's concern. Variants are added when a rule first
/// needs them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleCategory {
    /// Code that is wrong or useless.
    Correctness,
    /// Consistency and readability concerns with no behavioral risk.
    Style,
    /// Constructs that are legal but commonly indicate stale or misleading code.
    Suspicious,
}

impl RuleCategory {
    /// Every category, in declaration order, for frontends that list them.
    pub const ALL: [Self; 3] = [Self::Correctness, Self::Style, Self::Suspicious];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Correctness => "correctness",
            Self::Style => "style",
            Self::Suspicious => "suspicious",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Correctness => "Code that is wrong or useless.",
            Self::Style => "Consistency and readability concerns with no behavioral risk.",
            Self::Suspicious => "Constructs that commonly indicate stale or misleading code.",
        }
    }

    /// The category a user-facing name selects, or `None` when none does.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|category| category.as_str() == name)
    }
}

// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_linter/src/rule.rs
impl fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Correctness => "Correctness",
            Self::Style => "Style",
            Self::Suspicious => "Suspicious",
        })
    }
}

/// The fixes a rule declares in its `declare_robusta_lint!` block.
///
/// Its [`FixKind`] is an inventory, not a ceiling: it names every kind the rule
/// emits, and [`LintContext`] checks each emitted edit against it.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleFixMeta {
    /// The rule reports only.
    #[default]
    None,
    /// A fix is available for some violations, but not all.
    Conditional(FixKind),
    /// A fix is available for every violation the rule reports.
    Fixable(FixKind),
}

impl RuleFixMeta {
    /// The kinds this rule may emit, or [`FixKind::NONE`].
    ///
    /// # Panics
    ///
    /// In debug builds, when a rule declares a fix with no kind.
    #[must_use]
    pub fn fix_kind(self) -> FixKind {
        match self {
            Self::Conditional(kind) | Self::Fixable(kind) => {
                debug_assert!(
                    kind.is_some(),
                    "a rule declared a fix with no kind; use `RuleFixMeta::None` if it has none"
                );
                kind
            }
            Self::None => FixKind::NONE,
        }
    }

    /// Whether the rule offers any edit, unsafe fixes included.
    #[must_use]
    pub const fn has_fix(self) -> bool {
        matches!(self, Self::Fixable(_) | Self::Conditional(_))
    }

    // Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_linter/src/rule.rs
    pub fn emoji(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Conditional(kind) | Self::Fixable(kind) => Some(kind.emoji()),
        }
    }

    /// Whether this rule declared that it may emit an edit of `kind`.
    #[must_use]
    pub fn supports_fix(self, kind: FixKind) -> bool {
        self.fix_kind().contains(kind)
    }

    /// Stable user-facing name used by configuration docs and CLI metadata.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Conditional(kind) => match kind {
                FixKind::SAFE_FIX => "conditional_fix",
                FixKind::UNSAFE_FIX => "conditional_unsafe_fix",
                FixKind::ALL => "conditional_safe_fix_or_unsafe_fix",
                _ => "conditional_unknown",
            },
            Self::Fixable(kind) => match kind {
                FixKind::SAFE_FIX => "fixable_fix",
                FixKind::UNSAFE_FIX => "fixable_unsafe_fix",
                FixKind::ALL => "fixable_safe_fix_or_unsafe_fix",
                _ => "fixable_unknown",
            },
        }
    }
}

impl fmt::Display for RuleFixMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Declaration-time metadata for one lint rule.
#[derive(Debug, Clone, Copy)]
pub struct RuleMeta {
    /// Stable, user-facing identity used in config keys and diagnostics.
    pub id: RuleId,
    /// One-sentence, user-facing description of what the rule reports.
    pub summary: &'static str,
    pub category: RuleCategory,
    /// What fixes the rule can attach.
    pub fix: RuleFixMeta,
}

/// The metadata half of a rule, implemented by its `declare_robusta_lint!` block.
pub trait DeclaredRule {
    fn meta(&self) -> &'static RuleMeta;

    fn documentation(&self) -> Option<&'static str> {
        None
    }

    /// The node kinds `run` acts on, or `None` to run on every node.
    ///
    /// The [`Linter`](crate::Linter) never hands the rule a kind missing from
    /// the set, so the rule's tests must cover each kind it matches.
    fn node_types(&self) -> Option<&'static AstTypesBitset>;

    /// Whether the rule takes settings.
    fn has_config(&self) -> bool;

    fn settings_schema(
        &self,
        _generator: &mut schemars::SchemaGenerator,
    ) -> Option<schemars::schema::Schema> {
        None
    }

    /// A new instance of the rule, initialized by [`Rule::from_configuration`].
    ///
    /// # Errors
    ///
    /// When `value` is not a valid configuration for the rule.
    fn configure(&self, value: serde_json::Value) -> Result<Arc<dyn Rule>, serde_json::Error>;
}

/// A file-local lint rule: a `declare_robusta_lint!` block for its
/// [`DeclaredRule`] metadata, and an `impl Rule` with `run`.
///
/// `Send + Sync` lets one registry serve files linted in parallel.
pub trait Rule: DeclaredRule + fmt::Debug + Send + Sync {
    /// Initialize from the rule's settings, or `Value::Null` for the defaults.
    ///
    /// # Errors
    ///
    /// When `value` is not a valid configuration for the rule.
    fn from_configuration(_value: serde_json::Value) -> Result<Self, serde_json::Error>
    where
        Self: Sized + Default,
    {
        Ok(Self::default())
    }

    /// The settings that differ from the defaults, as the configuration writes them.
    fn to_configuration(&self) -> Option<serde_json::Value> {
        None
    }

    fn run(&self, node: &AstNode<'_>, context: &mut LintContext<'_, '_, '_>);

    /// The declared rule ID. Do not override.
    fn id(&self) -> RuleId {
        self.meta().id
    }
}

/// One enabled rule, configured, and its severity.
#[derive(Clone)]
pub struct RuleConfiguration {
    rule: Arc<dyn Rule>,
    severity: Severity,
}

impl RuleConfiguration {
    /// `rule` with its default settings.
    ///
    /// # Panics
    ///
    /// When the rule rejects `Value::Null`, which every rule accepts.
    pub fn new(rule: &dyn Rule, severity: Severity) -> Self {
        let rule = rule.configure(serde_json::Value::Null).expect("every rule has defaults");
        Self { rule, severity }
    }

    /// `rule` configured by `value`.
    ///
    /// # Errors
    ///
    /// When `value` is not a valid configuration for the rule.
    pub fn configured(
        rule: &dyn Rule,
        value: serde_json::Value,
        severity: Severity,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self { rule: rule.configure(value)?, severity })
    }

    pub fn rule(&self) -> &dyn Rule {
        &*self.rule
    }

    pub const fn severity(&self) -> Severity {
        self.severity
    }

    /// The same configured rule at `severity`.
    #[must_use]
    pub fn with_severity(&self, severity: Severity) -> Self {
        Self { rule: Arc::clone(&self.rule), severity }
    }
}

impl fmt::Debug for RuleConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuleConfiguration")
            .field("rule_id", &self.rule.id())
            .field("severity", &self.severity)
            .field("settings", &self.rule.to_configuration())
            .finish()
    }
}

impl PartialEq for RuleConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.rule.id() == other.rule.id()
            && self.severity == other.severity
            && self.rule.to_configuration() == other.rule.to_configuration()
    }
}

impl Eq for RuleConfiguration {}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "ruledocs")]
    fn every_registered_rule_has_documentation() {
        for rule in crate::rules::builtin_rules() {
            assert!(
                rule.documentation().is_some_and(|docs| !docs.trim().is_empty()),
                "Rule '{}' is missing documentation.",
                rule.id()
            );
        }
    }
}
