use robusta_ast::AstKind;
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;

use crate::utils::follows_documentation;
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
        id: REDUNDANT_SEMICOLONS_ID = "redundant-semicolons",
        rule: REDUNDANT_SEMICOLONS,
        summary: "Reports empty declarations and redundant empty statements.",
        category: Correctness,
        fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
        node_types: [EmptyDeclaration, EmptyStatement],
    }
}

impl Rule for RedundantSemicolons {
    fn run(&self, node: &robusta_semantic::AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let span = match node.kind() {
            AstKind::EmptyDeclaration(declaration) => declaration.span,
            AstKind::EmptyStatement(statement)
                if matches!(
                    context.nodes().parent_kind(node.id()),
                    AstKind::Block(_) | AstKind::SwitchBlockStatementGroup(_)
                ) =>
            {
                statement.span
            }
            _ => return,
        };
        if follows_documentation(context.comments(), span) {
            return;
        }

        context.diagnostic_with_fix(
            RobustaDiagnostic::warning("redundant semicolon", span)
                .with_help("Remove this semicolon."),
            |fixer| fixer.delete_range(span),
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
            " class A {  void f() {  if (ready) ; else {  } while (ready) ; label: ; } } ",
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
    fn keeps_the_semicolon_after_a_documentation_comment() {
        let source = concat!(
            "/** @deprecated */ ;; class A { /** d */ /* c */ ; ; void f() { /// d\n",
            ";; class L {} } } /**/ ; //// c\n; /*** b */ ; ",
            r"\u002f** e */ ;",
        );

        let report = Tester::new(&REDUNDANT_SEMICOLONS).fixes(
            source,
            concat!(
                "/** @deprecated */ ; class A { /** d */ /* c */ ;  void f() { /// d\n",
                "; class L {} } } /**/ ; //// c\n; /*** b */ ; ",
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
                r" /** d */ int x;\u0020 }",
            ),
        );

        insta::assert_snapshot!(report, @r#"
        1:83: warning[redundant-semicolons]: redundant semicolon ("\\u003b")
        1:89: warning[redundant-semicolons]: redundant semicolon (";")
        1:112: warning[redundant-semicolons]: redundant semicolon (";")
        "#);
    }
}
