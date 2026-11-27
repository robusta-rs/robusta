use robusta_ast::{AstKind, KeywordModifier, Modifier, ModifierKind};
use robusta_diagnostics::{RobustaDiagnostic, labeled_span, primary_labeled_span};
use robusta_macros::declare_robusta_lint;
use robusta_span::Span;

use crate::{FixKind, LintContext, Rule, RuleFixMeta};

declare_robusta_lint! {
    /// Reorders contiguous keyword modifiers into Java's customary order.
    ///
    /// Annotations end a run. This keeps their source position stable and mirrors
    /// google-java-format's discipline of reordering modifier tokens without
    /// moving annotations or the trivia between modifier positions.
    ModifierOrder {
        id: MODIFIER_ORDER_ID,
        rule: MODIFIER_ORDER,
        summary: "Reports keyword modifiers that are not in Java's customary order.",
        category: Style,
        fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
        node_types: [
            ClassDeclaration,
            InterfaceDeclaration,
            EnumDeclaration,
            EnumConstant,
            RecordDeclaration,
            RecordComponent,
            AnnotationInterfaceDeclaration,
            AnnotationInterfaceElementDeclaration,
            FieldDeclaration,
            MethodDeclaration,
            ConstructorDeclaration,
            CompactConstructorDeclaration,
            FormalParameter,
            EnhancedForStatement,
            ResourceDeclaration,
            CatchClause,
            LocalVariableDeclaration,
            TypePattern,
        ],
    }
}

impl Rule for ModifierOrder {
    fn run(&self, node: &robusta_semantic::AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let Some(modifiers) = modifier_list(node.kind()) else {
            return;
        };

        for run in modifiers.split(|modifier| matches!(modifier, Modifier::Annotation(_))) {
            check_run(run, context);
        }
    }
}

fn check_run(modifiers: &[Modifier<'_>], context: &mut LintContext<'_, '_, '_>) {
    if modifiers.len() < 2
        || modifiers.windows(2).all(|pair| {
            modifier_rank(keyword_modifier(&pair[0]).kind)
                <= modifier_rank(keyword_modifier(&pair[1]).kind)
        })
    {
        return;
    }

    let inversion = modifiers
        .windows(2)
        .find(|pair| {
            modifier_rank(keyword_modifier(&pair[0]).kind)
                > modifier_rank(keyword_modifier(&pair[1]).kind)
        })
        .expect("an unordered modifier run has an adjacent inversion");
    let before = keyword_modifier(&inversion[0]);
    let should_precede = keyword_modifier(&inversion[1]);

    let mut sorted = modifiers.iter().map(keyword_modifier).collect::<Vec<_>>();
    sorted.sort_by_key(|modifier| modifier_rank(modifier.kind));

    let first = keyword_modifier(&modifiers[0]).span;
    let last = keyword_modifier(&modifiers[modifiers.len() - 1]).span;
    let span = Span::new(first.start, last.end);
    let mut replacement = String::with_capacity(span.len() as usize);
    for (index, modifier) in sorted.iter().enumerate() {
        replacement.push_str(modifier.span.source_text(context.source_text()));
        if let Some(next) = modifiers.get(index + 1) {
            let current = keyword_modifier(&modifiers[index]);
            let trivia = Span::new(current.span.end, keyword_modifier(next).span.start);
            replacement.push_str(trivia.source_text(context.source_text()));
        }
    }

    let source = context.source_text();
    let diagnostic = RobustaDiagnostic::warning("modifiers are not in the canonical order", span)
        .with_labels([
            primary_labeled_span(span, "reorder this modifier run"),
            labeled_span(
                before.span,
                format!("`{}` currently appears first", before.span.source_text(source)),
            ),
            labeled_span(
                should_precede.span,
                format!("`{}` should appear earlier", should_precede.span.source_text(source)),
            ),
        ])
        .with_help("Reorder the modifiers into Java's customary sequence.");
    context.diagnostic_with_fix(diagnostic, |fixer| fixer.replace(span, replacement));
}

fn keyword_modifier<'modifier>(modifier: &'modifier Modifier<'_>) -> &'modifier KeywordModifier {
    match modifier {
        Modifier::Keyword(keyword) => keyword,
        Modifier::Annotation(_) => unreachable!("annotations split keyword modifier runs"),
    }
}

const fn modifier_rank(kind: ModifierKind) -> u8 {
    match kind {
        ModifierKind::Public => 0,
        ModifierKind::Protected => 1,
        ModifierKind::Private => 2,
        ModifierKind::Abstract => 3,
        ModifierKind::Default => 4,
        ModifierKind::Static => 5,
        ModifierKind::Final => 6,
        ModifierKind::Sealed => 7,
        ModifierKind::NonSealed => 8,
        ModifierKind::Transient => 9,
        ModifierKind::Volatile => 10,
        ModifierKind::Synchronized => 11,
        ModifierKind::Native => 12,
        ModifierKind::Strictfp => 13,
    }
}

fn modifier_list(kind: AstKind<'_>) -> Option<&[Modifier<'_>]> {
    if let Some(declaration) = kind.as_type_declaration_kind() {
        return Some(declaration.modifiers());
    }
    Some(match kind {
        AstKind::EnumConstant(node) => &node.modifiers,
        AstKind::RecordComponent(node) => &node.modifiers,
        AstKind::AnnotationInterfaceElementDeclaration(node) => &node.modifiers,
        AstKind::FieldDeclaration(node) => &node.modifiers,
        AstKind::MethodDeclaration(node) => &node.modifiers,
        AstKind::ConstructorDeclaration(node) => &node.modifiers,
        AstKind::CompactConstructorDeclaration(node) => &node.modifiers,
        AstKind::FormalParameter(node) => &node.modifiers,
        AstKind::EnhancedForStatement(node) => &node.modifiers,
        AstKind::ResourceDeclaration(node) => &node.modifiers,
        AstKind::CatchClause(node) => &node.modifiers,
        AstKind::LocalVariableDeclaration(node) => &node.modifiers,
        AstKind::TypePattern(node) => &node.modifiers,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tester::Tester;

    #[test]
    fn reorders_java_modifiers_and_preserves_trivia() {
        // Invalid Java: it combines every modifier.
        let source = "strictfp native synchronized volatile transient non-sealed sealed final \
                      static default abstract private protected public class A { \
                      static/* first */abstract/* second */public int value; }";

        let report = Tester::new(&MODIFIER_ORDER).fixes(
            source,
            "public protected private abstract default static final sealed non-sealed transient \
             volatile synchronized native strictfp class A { \
             public/* first */abstract/* second */static int value; }",
        );

        insta::assert_snapshot!(report, @r#"
        1:1: warning[modifier-order]: modifiers are not in the canonical order ("strictfp native synchronized volatile transient non-sealed sealed final static default abstract private protected public")
        1:132: warning[modifier-order]: modifiers are not in the canonical order ("static/* first */abstract/* second */public")
        "#);
    }

    #[test]
    fn reorders_each_run_in_place_around_comments_and_annotations() {
        let report = Tester::new(&MODIFIER_ORDER).fixes(
            "class Example {
    static /* shared */ public final int LIMIT = 1;
    static @Deprecated final public int oldLimit = 2;
}
",
            "class Example {
    public /* shared */ static final int LIMIT = 1;
    static @Deprecated public final int oldLimit = 2;
}
",
        );

        insta::assert_snapshot!(report, @r#"
        2:5: warning[modifier-order]: modifiers are not in the canonical order ("static /* shared */ public final")
        3:24: warning[modifier-order]: modifiers are not in the canonical order ("final public")
        "#);
    }

    #[test]
    fn orders_final_before_sealed_and_non_sealed() {
        // Invalid Java: final, sealed, and non-sealed exclude each other.
        let tester = Tester::new(&MODIFIER_ORDER);
        tester.passes("final sealed class A {}");
        tester.passes("final non-sealed class A {}");
        tester.passes("sealed non-sealed class A {}");

        let report = tester.fixes("sealed final class A {}", "final sealed class A {}");
        insta::assert_snapshot!(report, @r#"1:1: warning[modifier-order]: modifiers are not in the canonical order ("sealed final")"#);

        let report = tester.fixes("non-sealed final class A {}", "final non-sealed class A {}");
        insta::assert_snapshot!(report, @r#"1:1: warning[modifier-order]: modifiers are not in the canonical order ("non-sealed final")"#);
    }

    #[test]
    fn annotations_bound_independent_keyword_runs() {
        let report = Tester::new(&MODIFIER_ORDER).fixes(
            "static final @Deprecated private public class A {}",
            "static final @Deprecated public private class A {}",
        );

        insta::assert_snapshot!(report, @r#"1:26: warning[modifier-order]: modifiers are not in the canonical order ("private public")"#);
    }

    #[test]
    fn preserves_original_modifier_spelling_when_reordering() {
        let report = Tester::new(&MODIFIER_ORDER)
            .fixes(r"static p\u0075blic class A {}", r"p\u0075blic static class A {}");

        insta::assert_snapshot!(report, @r#"1:1: warning[modifier-order]: modifiers are not in the canonical order ("static p\\u0075blic")"#);
    }

    #[test]
    fn applies_to_nested_declarations_and_variable_positions() {
        let source = r"public class A {
            static public int field;
            final public A() {}
            synchronized public void method(final @Deprecated String value) {
                final @Deprecated String local = value;
                for (final @Deprecated String item : values) {}
            }
        }";

        let report = Tester::new(&MODIFIER_ORDER).fixes(
            source,
            r"public class A {
            public static int field;
            public final A() {}
            public synchronized void method(final @Deprecated String value) {
                final @Deprecated String local = value;
                for (final @Deprecated String item : values) {}
            }
        }",
        );

        insta::assert_snapshot!(report, @r#"
        2:13: warning[modifier-order]: modifiers are not in the canonical order ("static public")
        3:13: warning[modifier-order]: modifiers are not in the canonical order ("final public")
        4:13: warning[modifier-order]: modifiers are not in the canonical order ("synchronized public")
        "#);
    }
}
