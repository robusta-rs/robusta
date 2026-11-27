use robusta_ast::AstKind;
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;

use crate::utils::{follows_documentation, removal_span};
use crate::{FixKind, LintContext, Rule, RuleFixMeta};

declare_robusta_lint! {
    /// Removes empty declarations and empty statements in statement lists.
    ///
    /// Empty statements used as a control-flow or labeled-statement body are not
    /// redundant: deleting them would either make the program invalid or change
    /// which statement the construct owns. Neither is a semicolon directly after
    /// a documentation comment, which keeps the comment off the next
    /// declaration, where a `@deprecated` tag would mark it deprecated.
    RedundantSemicolons {
        id: REDUNDANT_SEMICOLONS_ID,
        rule: REDUNDANT_SEMICOLONS,
        summary: "Reports empty declarations and redundant empty statements.",
        category: Correctness,
        fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
        node_types: [EmptyStatement],
    }
}

impl Rule for RedundantSemicolons {
    fn run(&self, node: &robusta_semantic::AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let AstKind::EmptyStatement(statement) = node.kind() else { return };
        // Outside a list of statements or declarations, a `;` is a statement's body.
        if !matches!(
            context.nodes().parent_kind(node.id()),
            AstKind::Block(_)
                | AstKind::SwitchCase(_)
                | AstKind::OrdinaryCompilationUnit(_)
                | AstKind::CompactCompilationUnit(_)
                | AstKind::ClassBody(_)
                | AstKind::InterfaceBody(_)
                | AstKind::EnumBody(_)
                | AstKind::AnnotationInterfaceBody(_)
        ) {
            return;
        }
        let span = statement.span;
        if follows_documentation(context.comments(), span) {
            return;
        }

        context.diagnostic_with_fix(
            RobustaDiagnostic::warning("redundant semicolon", span)
                .with_help("Remove this semicolon."),
            |fixer| fixer.delete_range(removal_span(fixer.source_text(), span)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tester::Tester;

    #[test]
    fn removes_empty_declarations_and_statement_list_semicolons() {
        let source =
            "; class A { ; void f() { ;; if (ready) ; else { ; } while (ready) ; label: ; } } ;";

        let report = Tester::new(&REDUNDANT_SEMICOLONS).fixes(
            source,
            "class A { void f() {if (ready) ; else { } while (ready) ; label: ; } }",
        );

        insta::assert_snapshot!(report, @r#"
        1:1: warning[redundant-semicolons]: redundant semicolon (";")
        1:13: warning[redundant-semicolons]: redundant semicolon (";")
        1:26: warning[redundant-semicolons]: redundant semicolon (";")
        1:27: warning[redundant-semicolons]: redundant semicolon (";")
        1:49: warning[redundant-semicolons]: redundant semicolon (";")
        1:82: warning[redundant-semicolons]: redundant semicolon (";")
        "#);
    }

    #[test]
    fn removes_semicolons_among_the_members_of_every_type_body() {
        let tester = Tester::new(&REDUNDANT_SEMICOLONS);
        tester.fixes(
            "interface I { ; } enum E { A; ; } @interface N { ; } record R() { ; }",
            "interface I { } enum E { A; } @interface N { } record R() { }",
        );
        tester.fixes("void main() {} ;", "void main() {}");
    }

    #[test]
    fn keeps_semicolons_that_are_bodies_or_keep_a_deprecation_detached() {
        Tester::new(&REDUNDANT_SEMICOLONS).fixes(
            "/** @deprecated Old */ ;;
class Example {
    void run(boolean ready) {
        while (ready);
        done: ;
        work();;
    };
    void work() {}
};
",
            "/** @deprecated Old */ ;
class Example {
    void run(boolean ready) {
        while (ready);
        done: ;
        work();
    }
    void work() {}
}
",
        );
    }

    #[test]
    fn deletes_the_lines_of_semicolons_alone_on_them() {
        Tester::new(&REDUNDANT_SEMICOLONS).fixes(
            "class A {\r\n    ;\r\n    void f() {\r\n\t;\r\n        work();\r\n    }\r\n}\r\n  ;",
            "class A {\r\n    void f() {\r\n        work();\r\n    }\r\n}\r\n",
        );
    }

    #[test]
    fn keeps_the_semicolon_after_a_documentation_comment() {
        let source = concat!(
            "/** @deprecated */ ;; class A { /** d */ /* c */ ; ; void f() { /// d\n",
            ";; class L {} } } /**/ ; //// c\n; /*** b */ ; ",
            r"\u002f** e */ ;",
        );

        let report = Tester::new(&REDUNDANT_SEMICOLONS).fixes(
            source,
            concat!(
                "/** @deprecated */ ;class A { /** d */ /* c */ ; void f() { /// d\n",
                ";class L {} } } /**/ ; //// c\n; /*** b */ ; ",
                r"\u002f** e */ ;",
            ),
        );

        insta::assert_snapshot!(report, @r#"
        1:21: warning[redundant-semicolons]: redundant semicolon (";")
        1:52: warning[redundant-semicolons]: redundant semicolon (";")
        2:2: warning[redundant-semicolons]: redundant semicolon (";")
        "#);
    }

    #[test]
    fn keeps_the_semicolon_after_escaped_whitespace() {
        let source = concat!(
            r"/** @deprecated */\u0020\u0009\u000c; class A { /** d */\uu000A/* c */\u000D\u003b",
            r"\u003b; /** d */ int x;\u0020; }",
        );

        let report = Tester::new(&REDUNDANT_SEMICOLONS).fixes(
            source,
            concat!(
                r"/** @deprecated */\u0020\u0009\u000c; class A { /** d */\uu000A/* c */\u000D\u003b",
                r"/** d */ int x;\u0020}",
            ),
        );

        insta::assert_snapshot!(report, @r#"
        1:83: warning[redundant-semicolons]: redundant semicolon ("\\u003b")
        1:89: warning[redundant-semicolons]: redundant semicolon (";")
        1:112: warning[redundant-semicolons]: redundant semicolon (";")
        "#);
    }
}
