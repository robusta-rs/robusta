use std::{borrow::Cow, cell::Cell};

use rust_lapper::{Interval, Lapper};

use robusta_ast::{
    Annotation, AstKind, Comment, CommentKind, ElementValue, StringLiteral, TypeName,
    parse_lint_disable_directive,
};
use robusta_diagnostics::{RobustaDiagnostic, Severity};
use robusta_semantic::{AstNode, Semantic};
use robusta_span::{GetSpan, Span};
use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::{is_line_terminator, line_content_end, line_span, terminator_len},
};

use crate::{
    FixKind, LintDiagnostic, RuleId, suppression_pruner,
    utils::{names_java_lang_type, removal_span},
};

const SUPPRESS_WARNINGS: &str = "SuppressWarnings";
const ROBUSTA_RULE_PREFIX: &str = "robusta:";

/// Stable diagnostic identity for suppression hygiene findings.
pub const UNUSED_SUPPRESSION_ID: RuleId = RuleId::new("unused-suppression");

/// Source suppressions indexed once for a lint run.
#[derive(Debug)]
pub struct Suppressions<'source> {
    entries: Vec<Suppression<'source>>,
    intervals: Lapper<u32, usize>,
}

impl<'source> Suppressions<'source> {
    pub(crate) fn new(semantic: &Semantic<'source>) -> Self {
        let source = semantic.source_text();
        let mut entries = semantic
            .comments()
            .iter()
            .flat_map(|comment| parse_comment_suppression(source, *comment))
            .collect::<Vec<_>>();
        entries.extend(semantic.nodes().iter().flat_map(|node| match node.kind() {
            AstKind::Annotation(annotation) => {
                parse_annotation_suppression(semantic, node, annotation)
            }
            _ => Vec::new(),
        }));
        Self::from_entries(entries)
    }

    fn from_entries(mut entries: Vec<Suppression<'source>>) -> Self {
        entries.sort_by_key(|suppression| (suppression.directive.start, suppression.directive.end));
        let intervals = Lapper::new(
            entries
                .iter()
                .enumerate()
                .map(|(index, suppression)| Interval {
                    start: suppression.scope.start,
                    stop: suppression.scope.end,
                    val: index,
                })
                .collect(),
        );
        Self { entries, intervals }
    }

    pub(crate) fn contains(&self, rule_id: RuleId, offset: u32) -> bool {
        // No half-open u32 scope contains u32::MAX.
        let Some(end) = offset.checked_add(1) else {
            return false;
        };
        let mut matched = false;
        for interval in self.intervals.find(offset, end) {
            let suppression = &self.entries[interval.val];
            if suppression.rule.matches(rule_id) {
                suppression.used.set(true);
                matched = true;
            }
        }
        matched
    }

    /// Diagnostics for unused suppressions, with removal fixes where possible.
    pub(crate) fn unused_diagnostics(
        &self,
        semantic: &Semantic<'_>,
        severity: Severity,
    ) -> Vec<LintDiagnostic> {
        let unused = self
            .entries
            .iter()
            .filter(|suppression| {
                !suppression.used.get()
                    // `"all"` is shared with other tools.
                    && !matches!(
                        (&suppression.rule, suppression.origin),
                        (SuppressedRule::All, SuppressionOrigin::Annotation(_))
                    )
            })
            .collect::<Vec<_>>();
        let edits = suppression_pruner::removal_edits(semantic, &unused);
        unused
            .into_iter()
            .zip(edits)
            .map(|(suppression, edit)| {
                let message = match &suppression.rule {
                    SuppressedRule::All => "unused suppression".to_string(),
                    SuppressedRule::Rule(rule) => format!("unused suppression for rule `{rule}`"),
                };
                LintDiagnostic {
                    rule_id: UNUSED_SUPPRESSION_ID,
                    diagnostic: RobustaDiagnostic::warning(message, suppression.directive)
                        .with_severity(severity)
                        .with_help("Remove this unused suppression."),
                    span: suppression.directive,
                    fix: edit.map(|fix| fix.with_kind(FixKind::SAFE_FIX)),
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Suppression<'source> {
    scope: Span,
    pub directive: Span,
    rule: SuppressedRule<'source>,
    pub origin: SuppressionOrigin<'source>,
    used: Cell<bool>,
}

#[derive(Debug, Clone, Copy)]
pub enum SuppressionOrigin<'source> {
    Annotation(&'source Annotation<'source>),
    Comment(RuleComment<'source>),
}

/// A suppression comment's removal span and the explanation it keeps.
#[derive(Debug, Clone, Copy)]
pub struct CommentRemoval<'source> {
    pub span: Span,
    pub explanation: &'source str,
    pub kind: CommentKind,
}

#[derive(Debug, Clone, Copy)]
pub struct RuleComment<'source> {
    pub span: Span,
    pub rules: Span,
    pub removal: Option<CommentRemoval<'source>>,
}

impl RuleComment<'_> {
    // Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_linter/src/disable_directives.rs
    #[expect(clippy::cast_possible_truncation)]
    pub fn rule_names<'source>(&self, source: &'source str) -> Vec<(&'source str, Span)> {
        let text = self.rules.source_text(source);
        let mut rules = Vec::new();
        let mut start = None;
        for (index, character) in text.char_indices() {
            if character == ',' || character.is_whitespace() {
                if let Some(start) = start.take() {
                    rules.push((
                        &text[start..index],
                        Span::new(self.rules.start + start as u32, self.rules.start + index as u32),
                    ));
                }
            } else if start.is_none() {
                start = Some(index);
            }
        }
        if let Some(start) = start {
            rules
                .push((&text[start..], Span::new(self.rules.start + start as u32, self.rules.end)));
        }
        rules
    }
}

#[derive(Debug, Clone)]
enum SuppressedRule<'source> {
    All,
    Rule(Cow<'source, str>),
}

impl SuppressedRule<'_> {
    fn matches(&self, rule_id: RuleId) -> bool {
        match self {
            Self::All => true,
            Self::Rule(rule) => rule == rule_id.as_str(),
        }
    }

    fn into_owned(self) -> SuppressedRule<'static> {
        match self {
            Self::All => SuppressedRule::All,
            Self::Rule(rule) => SuppressedRule::Rule(Cow::Owned(rule.into_owned())),
        }
    }
}

#[expect(clippy::cast_possible_truncation)]
fn parse_comment_suppression(source: &str, comment: Comment) -> Vec<Suppression<'_>> {
    let content = comment.content_span(source);
    let text = content.source_text(source).trim_start();
    let Some((next_line, rest)) = parse_lint_disable_directive(text) else {
        return Vec::new();
    };
    let (names, explanation) = rest.split_once("--").unwrap_or((rest, ""));
    let names = names.trim();
    let explanation = explanation.trim();
    let start = content.end - rest.len() as u32;
    let rules_start = start + (rest.len() - rest.trim_start().len()) as u32;
    let rule_comment = RuleComment {
        span: comment.span,
        rules: Span::new(rules_start, rules_start + names.len() as u32),
        removal: if explanation.is_empty() {
            let span = removal_span(source, comment.span);
            // `long /* robusta-disable-line */value` keeps the comment that separates its tokens.
            (!joins_text(source, span)).then_some(CommentRemoval {
                span,
                explanation,
                kind: comment.kind,
            })
        } else if parse_lint_disable_directive(explanation).is_none() {
            Some(CommentRemoval { span: comment.span, explanation, kind: comment.kind })
        } else {
            None
        },
    };
    // A multiline block comment's own line is its first, and its next line follows its last.
    let scope = if next_line {
        let line_end = line_content_end(source, comment.span.end);
        let terminator = terminator_len(source, line_end as usize);
        let next_start = line_end + terminator as u32;
        if terminator == 0 {
            Span::new(next_start, next_start)
        } else {
            line_span(source, next_start)
        }
    } else {
        line_span(source, comment.span.start)
    };
    let rules = rule_comment.rule_names(source);
    if rules.is_empty() {
        vec![Suppression {
            scope,
            directive: comment.span,
            rule: SuppressedRule::All,
            origin: SuppressionOrigin::Comment(rule_comment),
            used: Cell::new(false),
        }]
    } else {
        rules
            .into_iter()
            .map(|(rule, directive)| Suppression {
                scope,
                directive,
                rule: SuppressedRule::Rule(Cow::Borrowed(rule)),
                origin: SuppressionOrigin::Comment(rule_comment),
                used: Cell::new(false),
            })
            .collect()
    }
}

/// Whether deleting `span` would put two non-whitespace characters side by side.
fn joins_text(source: &str, span: Span) -> bool {
    let separates = |byte: u8| is_horizontal_whitespace(byte) || is_line_terminator(byte);
    let bytes = source.as_bytes();
    span.start > 0
        && !separates(bytes[span.start as usize - 1])
        && bytes.get(span.end as usize).is_some_and(|&byte| !separates(byte))
}

fn parse_annotation_suppression<'source>(
    semantic: &Semantic<'source>,
    node: &AstNode<'source>,
    annotation: &'source Annotation<'source>,
) -> Vec<Suppression<'source>> {
    if !names_jdk_annotation(semantic, &annotation.name) {
        return Vec::new();
    }

    let Some(scope) = annotated_declaration_span(semantic.nodes().parent_kind(node.id())) else {
        return Vec::new();
    };
    suppressed_rules(annotation)
        .into_iter()
        .map(|(rule, directive)| Suppression {
            scope,
            directive,
            rule,
            origin: SuppressionOrigin::Annotation(annotation),
            used: Cell::new(false),
        })
        .collect()
}

/// Whether `name` names `java.lang.SuppressWarnings`.
fn names_jdk_annotation<'source>(semantic: &Semantic<'source>, name: &TypeName<'source>) -> bool {
    if name.last() != SUPPRESS_WARNINGS {
        return false;
    }
    let TypeName::Qualified(name) = name else {
        return matches!(name, TypeName::Identifier(name) if names_java_lang_type(semantic, name));
    };
    let TypeName::Qualified(lang) = &name.qualifier else {
        return false;
    };
    // A type named `java` in scope would take the qualifier (JLS 6.5.4.1).
    matches!(&lang.qualifier, TypeName::Identifier(java)
        if lang.name.name == "lang"
            && java.name == "java"
            && semantic.scoping().reference(java.reference_id()).symbol_id().is_none())
}

fn suppressed_rules<'source>(
    annotation: &Annotation<'source>,
) -> Vec<(SuppressedRule<'source>, Span)> {
    let mut rules = Vec::new();
    if let Some(value) = annotation.value() {
        collect_suppressed_rules(value, &mut rules);
    }
    rules
}

fn collect_suppressed_rules<'source>(
    value: &ElementValue<'source>,
    rules: &mut Vec<(SuppressedRule<'source>, Span)>,
) {
    match value {
        ElementValue::StringLiteral(literal) => {
            if let Some(rule) = string_literal_rule(literal) {
                rules.push((rule, literal.span));
            }
        }
        ElementValue::Array(array) => {
            for value in &array.values {
                collect_suppressed_rules(value, rules);
            }
        }
        _ => {}
    }
}

/// The rule a string literal's value names (JLS §3.10.5).
fn string_literal_rule<'source>(
    literal: &StringLiteral<'source>,
) -> Option<SuppressedRule<'source>> {
    match robusta_lexer::string_literal_value(literal.raw)? {
        Cow::Borrowed(value) => value_rule(value),
        Cow::Owned(value) => value_rule(&value).map(SuppressedRule::into_owned),
    }
}

fn value_rule(value: &str) -> Option<SuppressedRule<'_>> {
    if value == "all" {
        return Some(SuppressedRule::All);
    }
    let rule = value.strip_prefix(ROBUSTA_RULE_PREFIX).filter(|rule| !rule.is_empty())?;
    Some(SuppressedRule::Rule(Cow::Borrowed(rule)))
}

fn annotated_declaration_span(kind: AstKind<'_>) -> Option<Span> {
    if kind.is_type_declaration() {
        return Some(kind.span());
    }
    match kind {
        AstKind::ModuleDeclaration(_)
        | AstKind::EnumConstant(_)
        | AstKind::RecordComponent(_)
        | AstKind::AnnotationInterfaceElementDeclaration(_)
        | AstKind::FieldDeclaration(_)
        | AstKind::MethodDeclaration(_)
        | AstKind::ConstructorDeclaration(_)
        | AstKind::CompactConstructorDeclaration(_)
        | AstKind::FormalParameter(_)
        | AstKind::CatchParameter(_)
        | AstKind::LocalVariableDeclaration(_) => Some(kind.span()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use cow_utils::CowUtils;
    use robusta_allocator::Allocator;
    use robusta_diagnostics::Severity;
    use robusta_semantic::SemanticBuilder;
    use robusta_span::Span;

    use super::{RuleComment, SuppressedRule, Suppression, SuppressionOrigin, Suppressions};
    use crate::{
        FixKind, LintOptions, Linter, Rule, RuleConfiguration, RuleId, apply_fixes,
        rules::{self, MODIFIER_ORDER, REDUNDANT_SEMICOLONS, UPPERCASE_LONG_SUFFIX},
    };

    #[test]
    fn indexed_scopes_match_linear_lookup_and_usage() {
        let scopes = [
            Span::new(10, 20),
            Span::new(0, 100),
            Span::new(10, 20),
            Span::new(15, 25),
            Span::new(0, 0),
            Span::new(20, 20),
            Span::new(u32::MAX - 1, u32::MAX),
            Span::new(u32::MAX, u32::MAX),
        ];
        let rules = [
            SuppressedRule::Rule("a".into()),
            SuppressedRule::All,
            SuppressedRule::Rule("b".into()),
        ];
        let entries = scopes
            .into_iter()
            .flat_map(|scope| rules.clone().map(|rule| (scope, rule)))
            .enumerate()
            .map(|(index, (scope, rule))| Suppression {
                scope,
                // Reverse directive order so it differs from scope order.
                directive: Span::new(
                    100 - u32::try_from(index).unwrap(),
                    101 - u32::try_from(index).unwrap(),
                ),
                rule,
                origin: SuppressionOrigin::Comment(RuleComment {
                    span: scope,
                    rules: scope,
                    removal: None,
                }),
                used: Cell::new(false),
            })
            .collect();
        let suppressions = Suppressions::from_entries(entries);
        assert!(
            suppressions
                .entries
                .windows(2)
                .all(|pair| pair[0].directive.start < pair[1].directive.start)
        );
        for offset in (0..=101).chain([u32::MAX - 2, u32::MAX - 1, u32::MAX]) {
            for rule in ["a", "b", "unmatched"] {
                let rule_id = RuleId::new(rule);
                let expected = suppressions
                    .entries
                    .iter()
                    .map(|entry| {
                        entry.used.set(false);
                        entry.scope.start <= offset
                            && offset < entry.scope.end
                            && entry.rule.matches(rule_id)
                    })
                    .collect::<Vec<_>>();
                assert_eq!(suppressions.contains(rule_id, offset), expected.contains(&true));
                assert_eq!(
                    suppressions.entries.iter().map(|entry| entry.used.get()).collect::<Vec<_>>(),
                    expected,
                    "{rule} at {offset}",
                );
            }
        }
        let empty = Suppressions::from_entries(Vec::new());
        assert!(!empty.contains(RuleId::new("a"), 0));
        assert!(!empty.contains(RuleId::new("a"), u32::MAX));
    }

    #[test]
    fn overlapping_annotations_and_line_directives_are_all_used() {
        let source = concat!(
            "@SuppressWarnings(\"robusta:uppercase-long-suffix\") class A {\n",
            "@SuppressWarnings({\"robusta:uppercase-long-suffix\", ",
            "\"robusta:uppercase-long-suffix\", \"robusta:modifier-order\"})\n",
            "long value = 1l; // robusta-disable-line uppercase-long-suffix, uppercase-long-suffix\n",
            "}\n",
        );
        assert!(lint(source).is_empty());
        assert_eq!(messages(&audit_all(source)), ["unused suppression for rule `modifier-order`"]);
        assert_eq!(prune_all(source), source.cow_replace(", \"robusta:modifier-order\"", ""));
    }

    fn lint(source: &str) -> Vec<crate::LintDiagnostic> {
        lint_with_options(source, LintOptions::default())
    }

    #[test]
    fn escaped_annotation_and_element_names_select_the_same_suppression() {
        let source = r#"class A { @SuppressWarn\u0069ngs(v\u0061lue = "robusta:uppercase-long-suffix") long x = 1l; }"#;
        assert!(lint(source).is_empty());
    }

    #[test]
    fn unicode_escaped_values_suppress_like_their_plain_spelling() {
        for value in [
            r#""\u0072obusta:uppercase-long-suffix""#,
            r#""robusta:uppercase-long-suff\u0069x""#,
            r#""robusta\uuu003auppercase-long-suffix""#,
            r"\u0022robusta:uppercase-long-suffix\u0022",
            r#""\u0061ll""#,
            r#"{"unchecked", "robusta:uppercase\u002dlong-suffix"}"#,
        ] {
            let source = format!("class A {{ @SuppressWarnings({value}) long value = 1l; }}");
            assert!(lint(&source).is_empty(), "{source}");
        }
        for value in [
            r#""robusta:uppercase-long-suffix\t""#,
            r#""robusta:uppercase-long-suffix\u005c\u005c""#,
            r#""\u0072obusta:""#,
        ] {
            let source = format!("class A {{ @SuppressWarnings({value}) long value = 1l; }}");
            assert_eq!(rule_ids(&lint(&source)), ["uppercase-long-suffix"], "{source}");
        }
    }

    #[test]
    fn unused_unicode_escaped_values_are_reported_on_their_source_spelling() {
        let source = concat!(
            "class A {\n",
            r#"@SuppressWarnings({"unchecked", "robusta:modifier\u002dorder"})"#,
            "\nint value;\n}\n",
        );
        let diagnostics = audit(source, &RULES);
        assert_eq!(messages(&diagnostics), ["unused suppression for rule `modifier-order`"]);
        assert_eq!(diagnostics[0].span().source_text(source), r#""robusta:modifier\u002dorder""#);
        assert_eq!(prune(source), "class A {\n@SuppressWarnings({\"unchecked\"})\nint value;\n}\n");

        let sole =
            "class A {\n@SuppressWarnings(\"\\u0072obusta:modifier-order\")\nint value;\n}\n";
        assert_eq!(prune(sole), "class A {\nint value;\n}\n");
    }

    #[test]
    fn escape_sequence_values_suppress_like_their_plain_spelling() {
        for value in [
            r#""\162obusta:uppercase-long-suffix""#,
            r#""robusta\072uppercase-long-suffix""#,
            r#""robusta:uppercase-long-suffi\170""#,
            r#""\141ll""#,
            r#"{"unchecked", "robusta:uppercase\55long-suffix"}"#,
        ] {
            let source = format!("class A {{ @SuppressWarnings({value}) long value = 1l; }}");
            assert!(lint(&source).is_empty(), "{source}");
        }
        for value in [
            r#""robusta:uppercase-long-suffix\s""#,
            r#""robusta:uppercase-long-suffix\\""#,
            r#""robusta:uppercase-long-suffix\0""#,
            r#""\162obusta:""#,
        ] {
            let source = format!("class A {{ @SuppressWarnings({value}) long value = 1l; }}");
            assert_eq!(rule_ids(&lint(&source)), ["uppercase-long-suffix"], "{source}");
        }
    }

    #[test]
    fn unused_escape_sequence_values_are_reported_on_their_source_spelling() {
        let source = concat!(
            "class A {\n",
            r#"@SuppressWarnings({"unchecked", "robusta:modifier\055order"})"#,
            "\nint value;\n}\n",
        );
        let diagnostics = audit(source, &RULES);
        assert_eq!(messages(&diagnostics), ["unused suppression for rule `modifier-order`"]);
        assert_eq!(diagnostics[0].span().source_text(source), r#""robusta:modifier\055order""#);
        assert_eq!(prune(source), "class A {\n@SuppressWarnings({\"unchecked\"})\nint value;\n}\n");
    }

    fn lint_with_options(source: &str, options: LintOptions<'_>) -> Vec<crate::LintDiagnostic> {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let semantic = SemanticBuilder::new().build(&parsed.unit);
        Linter::new([
            RuleConfiguration::new(&MODIFIER_ORDER, Severity::Warning),
            RuleConfiguration::new(&REDUNDANT_SEMICOLONS, Severity::Warning),
            RuleConfiguration::new(&UPPERCASE_LONG_SUFFIX, Severity::Warning),
        ])
        .lint_with_options(&semantic, options)
    }

    const RULES: [&dyn Rule; 3] = [&MODIFIER_ORDER, &REDUNDANT_SEMICOLONS, &UPPERCASE_LONG_SUFFIX];

    /// The unused-suppression findings of an audited pass over `rules`.
    fn audit(source: &str, rules: &[&'static dyn Rule]) -> Vec<crate::LintDiagnostic> {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let semantic = SemanticBuilder::new().build(&parsed.unit);
        Linter::new(rules.iter().map(|rule| RuleConfiguration::new(*rule, Severity::Warning)))
            .lint_with_options(
                &semantic,
                LintOptions {
                    report_unused_suppressions: Some(Severity::Warning),
                    ..LintOptions::default()
                },
            )
            .into_iter()
            .filter(|diagnostic| diagnostic.rule_id() == super::UNUSED_SUPPRESSION_ID)
            .collect()
    }

    /// Audit with every built-in rule enabled.
    fn audit_all(source: &str) -> Vec<crate::LintDiagnostic> {
        audit(source, rules::builtin_rules())
    }

    fn apply(source: &str, diagnostics: &[crate::LintDiagnostic], fix_kind: FixKind) -> String {
        apply_fixes(source, diagnostics, fix_kind)
            .expect("removal edits stay inside the source")
            .code
            .into_owned()
    }

    /// `source` after the fixes of [`audit`] over [`RULES`].
    fn prune(source: &str) -> String {
        apply(source, &audit(source, &RULES), FixKind::SAFE_FIX)
    }

    /// `source` after the fixes of [`audit_all`].
    fn prune_all(source: &str) -> String {
        apply(source, &audit_all(source), FixKind::SAFE_FIX)
    }

    fn messages(diagnostics: &[crate::LintDiagnostic]) -> Vec<&str> {
        diagnostics.iter().map(|diagnostic| diagnostic.diagnostic().message.as_str()).collect()
    }

    fn rule_ids(diagnostics: &[crate::LintDiagnostic]) -> Vec<&str> {
        diagnostics.iter().map(|diagnostic| diagnostic.rule_id().as_str()).collect()
    }

    #[test]
    fn targeted_annotation_is_declaration_scoped() {
        let source = "class A {\n\
            @SuppressWarnings(\"robusta:redundant-semicolons\")\n\
            static public void inside() { ; }\n\
            void outside() { ; }\n\
            }";

        assert_eq!(rule_ids(&lint(source)), ["modifier-order", "redundant-semicolons"]);
    }

    #[test]
    fn all_annotation_suppresses_every_rule_only_in_its_declaration() {
        let source = "class A {\n\
            @SuppressWarnings(\"all\") static public long inside = 10l;\n\
            static public long outside = 20l;\n\
            }";

        assert_eq!(rule_ids(&lint(source)), ["modifier-order", "uppercase-long-suffix"]);
    }

    #[test]
    fn annotation_accepts_namespaced_array_values_and_simple_name_matching() {
        let source = "class A {\n\
            @java.lang.SuppressWarnings(value = {\"unchecked\", \
                \"robusta:modifier-order\", \"robusta:uppercase-long-suffix\"})\n\
            static public long value = 10l;\n\
            }";

        assert!(lint(source).is_empty());
    }

    #[test]
    fn annotations_the_file_binds_elsewhere_do_not_suppress() {
        let annotation = "@interface SuppressWarnings { String value(); }";
        let member = "@SuppressWarnings({\"robusta:uppercase-long-suffix\", \
            \"robusta:modifier-order\"}) long value = 1l;";
        for source in [
            format!("{annotation} class A {{ {member} }}"),
            format!("class A {{ {annotation} {member} }}"),
            format!("class A {{ class B {{ {member} }} {annotation} }}"),
            format!("class B {{ {annotation} }} class A extends B {{ {member} }}"),
            format!("class B {{ {annotation} }} class A extends B {{ class C {{ {member} }} }}"),
            format!(
                "class C {{ {annotation} }} class B extends C {{}} class A extends B {{ {member} }}"
            ),
            format!("interface I {{ {annotation} }} class A implements I {{ {member} }}"),
            format!("interface I {{ {annotation} }} enum A implements I {{ X; {member} }}"),
            format!(
                "class O {{ static class B {{ {annotation} }} }} class A extends O.B {{ {member} }}"
            ),
            format!("class B {{ {annotation} }} class A {{ Object o = new B() {{ {member} }}; }}"),
            // The supertype is itself an inherited member type.
            format!(
                "class O {{ static class B {{ {annotation} }} }} \
                class P extends O {{ class A extends B {{ {member} }} }}"
            ),
            format!(
                "class B {{}} class O {{ static class B {{ {annotation} }} }} \
                class P extends O {{ class A extends B {{ {member} }} }}"
            ),
            format!("import a.SuppressWarnings; class A {{ {member} }}"),
            format!("import static a.B.SuppressWarnings; class A {{ {member} }}"),
            format!("class A {{ {} }}", member.cow_replace("@S", "@A.S")),
            format!("class A {{ {} }}", member.cow_replace("@S", "@lang.S")),
            format!("class A {{ class java {{}} {} }}", member.cow_replace("@S", "@java.lang.S")),
        ] {
            assert_eq!(rule_ids(&lint(&source)), ["uppercase-long-suffix"], "{source}");
            assert!(audit(&source, &RULES).is_empty(), "{source}");
        }
    }

    #[test]
    fn simple_jdk_annotation_suppresses_outside_the_scope_of_one_the_file_declares() {
        let annotation = "@interface SuppressWarnings { String value(); }";
        let suppression =
            "@SuppressWarnings({\"robusta:uppercase-long-suffix\", \"robusta:modifier-order\"})";
        let member = format!("{suppression} long value = 1l;");
        for source in [
            format!("class Other {{ {annotation} }} class A {{ {member} }}"),
            format!("class B {{ private {annotation} }} class A extends B {{ {member} }}"),
            // A class's own annotations are outside its body.
            format!(
                "class B {{ {annotation} }} {suppression} class A extends B {{ long value = 1l; }}"
            ),
            // Cyclic inheritance does not compile, but the lookup ends.
            format!("class A extends B {{ {member} }} class B extends A {{}}"),
        ] {
            assert!(lint(&source).is_empty(), "{source}");
            assert_eq!(
                messages(&audit(&source, &RULES)),
                ["unused suppression for rule `modifier-order`"],
                "{source}"
            );
        }

        // Each use of the name resolves where it is written.
        let source = format!("class Other {{ {annotation} {member} }} class A {{ {member} }}");
        assert_eq!(rule_ids(&lint(&source)), ["uppercase-long-suffix"]);
        let unused = audit(&source, &RULES);
        assert_eq!(messages(&unused), ["unused suppression for rule `modifier-order`"]);
        assert!(source[..unused[0].span().start as usize].contains("class A"));
    }

    #[test]
    fn qualified_jdk_annotation_suppresses_beside_one_the_file_declares() {
        let source = "@interface SuppressWarnings { String value(); }\n\
            class A { @java.lang.SuppressWarnings(\"robusta:uppercase-long-suffix\") long value = 1l; }";
        assert!(lint(source).is_empty());
    }

    #[test]
    fn imported_jdk_annotation_suppresses() {
        let source = "import java.lang.SuppressWarnings;\n\
            class A { @SuppressWarnings(\"robusta:uppercase-long-suffix\") long value = 1l; }";
        assert!(lint(source).is_empty());
    }

    #[test]
    fn non_robusta_and_bare_rule_names_are_ignored() {
        for warning in ["unchecked", "modifier-order"] {
            let source = format!(
                "class A {{ @SuppressWarnings(\"{warning}\") static public long value = 10l; }}"
            );
            assert_eq!(rule_ids(&lint(&source)), ["modifier-order", "uppercase-long-suffix"]);
        }
    }

    #[test]
    fn disable_comments_match_named_rules_on_the_selected_physical_line() {
        for ending in ["\n", "\r\n", "\r"] {
            let source = concat!(
                "class A {\n",
                "// robusta-disable-next-line modifier-order, uppercase-long-suffix -- reason\n",
                "static public long first = 1l;\n",
                "static public long second = 2l; // robusta-disable-line uppercase-long-suffix\n",
                "}\n",
            )
            .cow_replace('\n', ending);
            assert_eq!(rule_ids(&lint(&source)), ["modifier-order"], "{ending:?}");
        }
    }

    #[test]
    fn disable_comments_without_a_rule_list_suppress_all_rules() {
        for comment in ["// robusta-disable-line", "// robusta-disable-line -- reason"] {
            let source = format!("class A {{ static public long value = 1l; {comment}\n}}");
            assert!(lint(&source).is_empty(), "{comment}");
        }
        let source =
            "class A {\n// robusta-disable-next-line -- reason\nstatic public long value = 1l;\n}";
        assert!(lint(source).is_empty());
    }

    #[test]
    fn disable_comment_scope_does_not_cross_blank_lines_or_end_of_file() {
        for source in [
            "class A {\n// robusta-disable-next-line\n\nstatic public long value = 1l;\n}",
            "class A {\n// robusta-disable-line\nstatic public long value = 1l;\n}",
            "class A { static public long value = 1l; } // robusta-disable-next-line",
            "class A {\n/* robusta-disable-next-line */\n\nstatic public long value = 1l;\n}",
            "class A { static public long value = 1l; } /* robusta-disable-next-line */",
        ] {
            assert_eq!(
                rule_ids(&lint(source)),
                ["modifier-order", "uppercase-long-suffix"],
                "{source}"
            );
        }
    }

    #[test]
    fn block_comments_match_named_rules_on_the_selected_physical_line() {
        for ending in ["\n", "\r\n", "\r"] {
            let source = concat!(
                "class A {\n",
                "/* robusta-disable-next-line modifier-order, uppercase-long-suffix -- reason */\n",
                "static public long first = 1l;\n",
                "static public long second = 2l; /* robusta-disable-line uppercase-long-suffix */\n",
                "/* robusta-disable-line modifier-order */ static public long third = 3l;\n",
                "/* robusta-disable-next-line */ static public long fourth = 4l;\n",
                "static public long fifth = 5l;\n",
                "}\n",
            )
            .cow_replace('\n', ending);
            assert_eq!(
                rule_ids(&lint(&source)),
                [
                    "modifier-order",
                    "uppercase-long-suffix",
                    "modifier-order",
                    "uppercase-long-suffix"
                ],
                "{ending:?}"
            );
        }
    }

    #[test]
    fn multiline_block_comments_cover_their_first_line_or_the_line_after_their_last() {
        for ending in ["\n", "\r\n", "\r"] {
            let source = concat!(
                "class A {\n",
                "/* robusta-disable-next-line\n",
                "   modifier-order,\n",
                "   uppercase-long-suffix\n",
                "*/\n",
                "static public long first = 1l;\n",
                "static public long second = 2l; /* robusta-disable-line uppercase-long-suffix\n",
                "   -- reason */\n",
                "/* robusta-disable-line\n",
                "*/ static public long third = 3l;\n",
                "/* robusta-disable-next-line\n",
                "*/ static public long fourth = 4l;\n",
                "static public long fifth = 5l;\n",
                "}\n",
            )
            .cow_replace('\n', ending);
            assert_eq!(
                rule_ids(&lint(&source)),
                [
                    "modifier-order",
                    "modifier-order",
                    "uppercase-long-suffix",
                    "modifier-order",
                    "uppercase-long-suffix"
                ],
                "{ending:?}"
            );
        }
    }

    #[test]
    fn block_comment_directives_must_start_the_comment_text() {
        for comment in [
            "/* see robusta-disable-line */",
            "/* robusta-disable-line-typo */",
            "/** robusta-disable-line */",
        ] {
            let source = format!("class A {{ static public long value = 1l; {comment} }}");
            assert_eq!(
                rule_ids(&lint(&source)),
                ["modifier-order", "uppercase-long-suffix"],
                "{comment}"
            );
        }
    }

    #[test]
    fn unused_comment_rules_are_pruned_together_while_preserving_used_rules_and_reason() {
        for (open, close) in [("//", ""), ("/*", " */")] {
            for names in [
                "redundant-semicolons, modifier-order, uppercase-long-suffix, unknown",
                "redundant-semicolons modifier-order uppercase-long-suffix unknown",
            ] {
                let source = format!(
                    "class A {{\n{open} robusta-disable-next-line {names} -- retained reason{close}\nstatic public long value = 1l;\n}}"
                );
                let expected = format!(
                    "class A {{\n{open} robusta-disable-next-line modifier-order, uppercase-long-suffix -- retained reason{close}\nstatic public long value = 1l;\n}}"
                );
                assert_eq!(audit_all(&source).len(), 2);
                assert_eq!(prune_all(&source), expected);
                assert_eq!(prune_all(&expected), expected);
            }
        }
    }

    #[test]
    fn unused_disable_comments_keep_explanations_without_promoting_directives() {
        for comment in [
            "// robusta-disable-next-line unused-import -- retained reason",
            "// robusta-disable-next-line -- retained reason",
        ] {
            let source = format!("class A {{\n{comment}\nint value;\n}}");
            assert_eq!(prune_all(&source), "class A {\n// retained reason\nint value;\n}");
        }
        for comment in [
            "/* robusta-disable-next-line unused-import -- retained reason */",
            "/* robusta-disable-next-line -- retained reason */",
        ] {
            let source = format!("class A {{\n{comment}\nint value;\n}}");
            assert_eq!(prune_all(&source), "class A {\n/* retained reason */\nint value;\n}");
        }
        for comment in [
            "// robusta-disable-line -- robusta-disable-next-line",
            "/* robusta-disable-line -- robusta-disable-next-line */",
        ] {
            let source = format!("class A {{ int value; {comment}\n}}");
            let diagnostics = audit_all(&source);
            assert_eq!(diagnostics.len(), 1);
            assert!(diagnostics[0].fix().is_none());
            assert_eq!(prune_all(&source), source);
        }
    }

    #[test]
    fn unused_block_comments_are_removed_without_joining_adjacent_code() {
        let source = concat!(
            "class A {\n",
            "    /* robusta-disable-next-line */\n",
            "    int first; /* robusta-disable-line -- retained reason */ int second;\n",
            "    /* robusta-disable-line */ int third;\n",
            "    int fourth; /* robusta-disable-line uppercase-long-suffix */ int fifth;\n",
            "    /* robusta-disable-next-line\n",
            "       uppercase-long-suffix\n",
            "    */\n",
            "    int sixth;\n",
            "}\n",
        );
        let expected = concat!(
            "class A {\n",
            "    int first; /* retained reason */ int second;\n",
            "    int third;\n",
            "    int fourth; int fifth;\n",
            "    int sixth;\n",
            "}\n",
        );

        assert_eq!(audit_all(source).len(), 5);
        assert_eq!(prune_all(source), expected);
        assert_eq!(prune_all(expected), expected);
    }

    /// Deleting the comment would join `long` and `value`.
    #[test]
    fn unused_block_comment_separating_tokens_has_no_removal() {
        for comment in ["/* robusta-disable-line */", " /* robusta-disable-line */"] {
            let source = format!("class A {{ long{comment}value; }}");
            let diagnostics = audit_all(&source);
            assert_eq!(messages(&diagnostics), ["unused suppression"], "{source}");
            assert!(diagnostics[0].fix().is_none(), "{source}");
        }
    }

    #[test]
    fn malformed_comment_directives_do_not_suppress() {
        let source = "class A {\n\
            // robusta-disable-next-line-typo\n\
            static public long first = 10l;\n\
            static public long second = 20l; // robusta-disable-line-typo\n\
            }";

        assert_eq!(
            rule_ids(&lint(source)),
            ["modifier-order", "uppercase-long-suffix", "modifier-order", "uppercase-long-suffix"]
        );
    }

    /// Nested declaration scopes with CRLF line endings.
    #[test]
    fn annotations_on_nested_declaration_kinds_cover_their_own_findings() {
        let source = "@SuppressWarnings(\"robusta:modifier-order\")\r\n\
            static public class A {\r\n\
            \t@SuppressWarnings(\"robusta:modifier-order\") final public A() {}\r\n\
            \tvoid f(Iterable<String> xs) {\r\n\
            \t\t{ @SuppressWarnings(\"robusta:uppercase-long-suffix\") long value = 1l; }\r\n\
            \t\tfor (@SuppressWarnings(\"robusta:modifier-order\") final public String x : xs) {}\r\n\
            \t}\r\n\
            }\r\n";

        assert!(lint(source).is_empty());
        assert_eq!(lint(&source.cow_replace("robusta:", "other:")).len(), 4);
    }

    #[test]
    fn annotations_on_header_variables_cover_only_their_declarations() {
        let suppressed = "@SuppressWarnings(\"robusta:modifier-order\") final public";
        let source = format!(
            "class A {{ void f(Object o, Iterable<A> xs) throws Exception {{ \
            for ({suppressed} A x : xs) {{}} for (final public A y : xs) {{}} \
            try ({suppressed} A r = new A(); final public A s = new A()) {{}} \
            if (o instanceof {suppressed} A a && o instanceof final public A b) {{}} \
            try {{}} catch ({suppressed} E e) {{}} try {{}} catch (final public E e) {{}} }} }}"
        );

        assert_eq!(rule_ids(&lint(&source)), ["modifier-order"; 4]);
    }

    #[test]
    fn unused_suppressions_are_reported_per_argument_and_comment() {
        let source = "class A {\n\
            @SuppressWarnings({\"robusta:modifier-order\", \
                \"robusta:uppercase-long-suffix\", \"unchecked\"})\n\
            static public int used;\n\
            long value = 10l; // robusta-disable-line\n\
            int clean; // robusta-disable-line -- explanation\n\
            }";

        let diagnostics = audit_all(source);

        assert_eq!(
            messages(&diagnostics),
            ["unused suppression for rule `uppercase-long-suffix`", "unused suppression"]
        );
    }

    #[test]
    fn unused_suppressions_take_the_requested_severity() {
        let source = "class A { @SuppressWarnings(\"robusta:modifier-order\") int clean; }";

        let diagnostics = lint_with_options(
            source,
            LintOptions {
                report_unused_suppressions: Some(Severity::Error),
                ..LintOptions::default()
            },
        );

        assert_eq!(rule_ids(&diagnostics), ["unused-suppression"]);
        assert_eq!(diagnostics[0].diagnostic().severity, Severity::Error);
        assert!(lint_with_options(source, LintOptions::default()).is_empty());
    }

    #[test]
    fn all_suppression_is_used_when_it_matches_a_finding() {
        let source = "class A { @SuppressWarnings(\"all\") static public long value = 10l; }";

        let diagnostics = audit_all(source);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn shared_all_suppression_is_never_reported_unused() {
        let source = "class A { @SuppressWarnings(\"all\") int value; }";

        let diagnostics = audit_all(source);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disabled_and_unknown_rule_suppressions_are_unused() {
        let source = "class A {\n\
            @SuppressWarnings({\"robusta:modifier-order\", \"robusta:no-such-rule\", \"all\"})\n\
            int value; // robusta-disable-line\n\
            }\n";
        for rules in [&[][..], &[&UPPERCASE_LONG_SUFFIX as &dyn Rule][..], &RULES[..]] {
            assert_eq!(
                messages(&audit(source, rules)),
                [
                    "unused suppression for rule `modifier-order`",
                    "unused suppression for rule `no-such-rule`",
                    "unused suppression",
                ]
            );
        }
        assert_eq!(prune(source), "class A {\n@SuppressWarnings({\"all\"})\nint value;\n}\n");
    }

    #[test]
    fn removal_is_a_safe_fix_that_keeps_shared_warning_names() {
        let source = concat!(
            "class S {\n",
            "    // robusta-disable-next-line uppercase-long-suffix\n",
            "    int n = 1;\n",
            "    @SuppressWarnings({\"unchecked\", \"robusta:modifier-order\"})\n",
            "    void m() {}\n",
            "}\n",
        );
        let pruned = "class S {\n    int n = 1;\n    @SuppressWarnings({\"unchecked\"})\n    void m() {}\n}\n";
        let diagnostics = audit(source, &RULES);

        assert_eq!(apply(source, &diagnostics, FixKind::NONE), source);
        for kind in [FixKind::SAFE_FIX, FixKind::ALL] {
            assert_eq!(apply(source, &diagnostics, kind), pruned, "{kind:?}");
        }
    }

    /// Deleting the annotation would give `m` the comment, deprecating it.
    #[test]
    fn annotation_before_a_documentation_comment_is_reported_without_a_fix() {
        for source in [
            "class A { @SuppressWarnings(\"robusta:modifier-order\") /** @deprecated Old. */ void m() {} }",
            "class A { @SuppressWarnings(value = {\"robusta:modifier-order\"}) /* x */\n/** B */ void m() {} }",
            "class A { /** A */ @SuppressWarnings(\"robusta:modifier-order\") /** B */ void m() {} }",
            "class A { @SuppressWarnings(\"robusta:modifier-order\")\\u000a/** B */ void m() {} }",
            "class A { @SuppressWarnings(\"robusta:modifier-order\")\\u0020/* x */\\uu000d/** B */ void m() {} }",
        ] {
            let diagnostics = audit(source, &RULES);
            assert_eq!(messages(&diagnostics), ["unused suppression for rule `modifier-order`"]);
            assert!(diagnostics[0].fix().is_none(), "{source}");
            assert_eq!(apply(source, &diagnostics, FixKind::ALL), source);
        }
    }

    #[test]
    fn annotation_after_a_documentation_comment_is_removed() {
        let source = concat!(
            "class A {\n",
            "    /** @deprecated Old. */\n",
            "    @SuppressWarnings(\"robusta:modifier-order\")\n",
            "    void m() {}\n",
            "    @SuppressWarnings({\"unchecked\", \"robusta:modifier-order\"}) /** B */ void n() {}\n",
            "    @SuppressWarnings(\"robusta:modifier-order\") void o() {} /** C */ void p() {}\n",
            "    @SuppressWarnings(\"robusta:modifier-order\")\\u0020void q() {} /** D */ void r() {}\n",
            "}\n",
        );

        assert_eq!(
            prune(source),
            concat!(
                "class A {\n",
                "    /** @deprecated Old. */\n",
                "    void m() {}\n",
                "    @SuppressWarnings({\"unchecked\"}) /** B */ void n() {}\n",
                "    void o() {} /** C */ void p() {}\n",
                "    \\u0020void q() {} /** D */ void r() {}\n",
                "}\n",
            )
        );
    }

    #[test]
    fn pruning_removes_a_sole_unused_annotation_but_preserves_its_reason_comment() {
        let source = "class A {\n\
            /* legacy API */\n\
            @SuppressWarnings(\"robusta:uppercase-long-suffix\")\n\
            int value;\n\
            @SuppressWarnings(\"robusta:uppercase-long-suffix\") // legacy API\n\
            int other;\n\
            }";

        assert_eq!(
            prune(source),
            "class A {\n/* legacy API */\nint value;\n// legacy API\nint other;\n}"
        );
    }

    #[test]
    fn pruning_treats_form_feed_as_horizontal_whitespace() {
        let source = concat!(
            "class A {\n",
            "\u{000c}@SuppressWarnings(\"robusta:uppercase-long-suffix\")\u{000c}\n",
            "int value;\n",
            "}",
        );

        assert_eq!(prune(source), "class A {\nint value;\n}");
    }

    #[test]
    fn pruning_mixed_named_array_values_keeps_used_and_unrelated_warnings() {
        let source = "class A {\n\
            @SuppressWarnings(value = {\"robusta:uppercase-long-suffix\", \
                \"unchecked\", \"robusta:modifier-order\"})\n\
            static public int value;\n\
            }";

        assert_eq!(
            prune(source),
            "class A {\n@SuppressWarnings(value = {\"unchecked\", \"robusta:modifier-order\"})\nstatic public int value;\n}"
        );
    }

    #[test]
    fn pruning_reads_the_value_element_among_the_annotations_arguments() {
        let unused = "\"robusta:uppercase-long-suffix\"";
        for (source, pruned) in [
            (
                format!("class A {{\n@SuppressWarnings(value = {unused})\nint value;\n}}"),
                "class A {\nint value;\n}".to_string(),
            ),
            (
                format!("class A {{ @SuppressWarnings(x = 1, value = {unused}) int value; }}"),
                format!("class A {{ @SuppressWarnings(x = 1, value = {unused}) int value; }}"),
            ),
            (
                format!("class A {{ @SuppressWarnings(x = {unused}) int value; }}"),
                format!("class A {{ @SuppressWarnings(x = {unused}) int value; }}"),
            ),
        ] {
            assert_eq!(prune(&source), pruned);
        }
    }

    #[test]
    fn pruning_removes_multiple_and_duplicate_unused_robusta_values() {
        for values in [
            "{\"robusta:uppercase-long-suffix\", \"unchecked\", \"robusta:modifier-order\"}",
            "{\"robusta:uppercase-long-suffix\", \"robusta:uppercase-long-suffix\", \"unchecked\"}",
        ] {
            let source = format!("class A {{ @SuppressWarnings({values}) int value; }}");

            assert_eq!(prune(&source), "class A { @SuppressWarnings({\"unchecked\"}) int value; }");
        }
    }

    #[test]
    fn pruning_leaves_plain_all_unrelated_and_ununderstood_values_unchanged() {
        for source in [
            "class A { @SuppressWarnings(\"all\") int value; }",
            "class A { @SuppressWarnings(\"unchecked\") int value; }",
            "class A { @SuppressWarnings(value = warningName()) int value; }",
            "class A { int value; // robusta-disable-line-typo\n}",
        ] {
            assert_eq!(prune(source), source);
            assert_eq!(prune_all(source), source);
        }
    }

    #[test]
    fn pruning_disable_comments_keeps_their_explanations() {
        let source = "class A {\r\n\
            // robusta-disable-line -- generated\r\n\
            int first; // robusta-disable-line -- generated\r\n\
            @SuppressWarnings(\"robusta:uppercase-long-suffix\") // robusta-disable-line -- generated\r\n\
            int second;\r\n\
            }\r\n";

        let diagnostics = audit_all(source);
        assert_eq!(diagnostics.len(), 4);
        assert_eq!(
            apply(source, &diagnostics, FixKind::SAFE_FIX),
            "class A {\r\n\
             // generated\r\n\
             int first; // generated\r\n\
             // generated\r\n\
             int second;\r\n\
             }\r\n"
        );
    }

    #[test]
    fn pruning_deletes_bare_disable_comments_and_the_lines_they_leave_empty() {
        let source = "class A {\r\n\
            // robusta-disable-line\r\n\
            int first; // robusta-disable-line\r\n\
            @SuppressWarnings(\"robusta:uppercase-long-suffix\") // robusta-disable-line\r\n\
            int second;\r\n\
            }\r\n";

        assert_eq!(prune_all(source), "class A {\r\nint first;\r\nint second;\r\n}\r\n");
    }

    #[test]
    fn pruning_treats_form_feed_around_a_disable_comment_as_horizontal_whitespace() {
        let leading =
            concat!("class A {\n", "\u{000c}// robusta-disable-line\n", "int value;\n", "}\n",);
        let trailing =
            concat!("class A {\n", "int value;\u{000c}// robusta-disable-line\n", "}\n",);

        for source in [leading, trailing] {
            assert_eq!(prune_all(source), "class A {\nint value;\n}\n");
        }
    }

    #[test]
    fn pruning_is_idempotent() {
        let source = "class A {\n\
            @SuppressWarnings({\"robusta:uppercase-long-suffix\", \"unchecked\"}) int value;\n\
            int other; // robusta-disable-line -- generated\n\
            }\n";

        let pruned = prune(source);
        assert_eq!(prune(&pruned), pruned);
        let pruned = prune_all(source);
        assert_eq!(prune_all(&pruned), pruned);
    }

    /// The annotation splits the modifier run, so the next pass reorders the whole run.
    #[test]
    fn removals_and_rule_fixes_reach_a_fixed_point() {
        let source = "class A {\n\
            final @SuppressWarnings(\"robusta:uppercase-long-suffix\") static public int value;\n\
            }\n";
        let fix = |source: &str| {
            let options = LintOptions {
                report_unused_suppressions: Some(Severity::Warning),
                ..LintOptions::default()
            };
            apply(source, &lint_with_options(source, options), FixKind::SAFE_FIX)
        };

        let first = fix(source);
        assert_eq!(first, "class A {\nfinal public static int value;\n}\n");
        let second = fix(&first);
        assert_eq!(second, "class A {\npublic static final int value;\n}\n");
        assert_eq!(fix(&second), second);
    }
}
