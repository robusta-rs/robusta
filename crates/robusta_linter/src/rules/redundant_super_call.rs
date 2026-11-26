use robusta_ast::{AstKind, ConstructorInvocationKind, Statement};
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;
use robusta_semantic::AstNode;

use crate::utils::follows_documentation;
use crate::{FixKind, LintContext, Rule, RuleFixMeta};

declare_robusta_lint! {
    /// Reports an explicit `super();` that Java would insert implicitly:
    /// `Example() { super(); initialize(); }` is
    /// `Example() { initialize(); }`.
    ///
    /// The invocation must be the first statement directly owned by an
    /// ordinary constructor body, unqualified, free of explicit type
    /// arguments, and called without arguments. JLS §8.8.7 makes that exact
    /// spelling equivalent to the implicit constructor invocation.
    ///
    /// The first-statement condition is semantic. Java 25 permits a constructor
    /// prologue before an explicit invocation, and deleting a later `super();`
    /// would move the implicit call before that prologue and could change
    /// behavior.
    ///
    /// A call that contains a comment is reported without a fix. So is a call
    /// directly after a documentation comment when a local type declaration
    /// could come next, since deleting it could attach the comment to that
    /// declaration.
    RedundantSuperCall {
        id: REDUNDANT_SUPER_CALL_ID = "redundant-super-call",
        rule: REDUNDANT_SUPER_CALL,
        summary: "Reports an explicit leading `super()` equivalent to the implicit constructor invocation.",
        category: Pedantic,
        fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
        node_types: [ConstructorDeclaration],
    }
}

impl Rule for RedundantSuperCall {
    fn run(&self, node: &AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let AstKind::ConstructorDeclaration(constructor) = node.kind() else {
            return;
        };
        let Some(Statement::ConstructorInvocation(invocation)) =
            constructor.body.statements.first()
        else {
            return;
        };
        if invocation.kind != ConstructorInvocationKind::Super
            || invocation.qualifier.is_some()
            || invocation.type_args.is_some()
            || !invocation.args.is_empty()
        {
            return;
        }

        // Comments around the deleted call stay in place.
        let message = "the explicit leading super call is redundant";
        let diagnostic = RobustaDiagnostic::warning(message, invocation.span)
            .with_help("Remove this call; Java inserts it implicitly.");
        let keep = context.has_comments_between(invocation.span)
            || (follows_documentation(context.comments(), invocation.span)
                && could_reach_local_type(&constructor.body.statements[1..]));
        context.diagnostic_with_fix(diagnostic, |fixer| {
            if keep { fixer.noop() } else { fixer.delete_range(invocation.span) }
        });
    }
}

/// Whether a documentation comment before `statements` could reach a local type.
///
/// javac attaches the comment to a local type, but not to a local variable. The
/// fixes of `redundant-semicolons` and `unused-local-variable` can delete empty
/// statements, a local variable, and that variable's writes in the same pass,
/// so none of them separates the comment from a local type after them.
fn could_reach_local_type(statements: &[Statement<'_>]) -> bool {
    let mut rest = statements.iter().filter(|statement| !matches!(statement, Statement::Empty(_)));
    match rest.next() {
        Some(Statement::LocalVariable(_)) => {
            rest.any(Statement::is_local_class_or_interface_declaration)
        }
        next => next.is_some_and(Statement::is_local_class_or_interface_declaration),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply_safe_fixes;
    use crate::rules::{REDUNDANT_SEMICOLONS, UNUSED_LOCAL_VARIABLE};
    use crate::tester::{Tester, lint_together};

    fn tester() -> Tester {
        Tester::new(&REDUNDANT_SUPER_CALL)
    }

    #[test]
    fn removes_leading_no_argument_super_calls() {
        let report = tester().fixes(
            "class A { A() { super(); initialize(); } A(int value) { super(); } }",
            "class A { A() {  initialize(); } A(int value) {  } }",
        );

        insta::assert_snapshot!(report, @r#"
        1:17: warning[redundant-super-call]: the explicit leading super call is redundant ("super();")
        1:57: warning[redundant-super-call]: the explicit leading super call is redundant ("super();")
        "#);
    }

    #[test]
    fn removes_only_a_call_that_comes_first() {
        tester().fixes(
            "class Child {
    Child() {
        super();
        start();
    }

    Child(int value) {
        check(value);
        super();
    }
}
",
            concat!(
                "class Child {\n",
                "    Child() {\n",
                "        \n",
                "        start();\n",
                "    }\n",
                "\n",
                "    Child(int value) {\n",
                "        check(value);\n",
                "        super();\n",
                "    }\n",
                "}\n",
            ),
        );
    }

    #[test]
    fn leaves_every_distinct_constructor_invocation_form_alone() {
        tester().passes(
            "class A { \
             A() { this(1); } \
             A(int value) { super(value); } \
             A(String value) { <String>super(); } \
             A(Object owner) { owner.super(); } \
             A(long value) { mark(\"P\"); super(); } \
             } \
             record R(int value) { R { initialize(); } }",
        );
    }

    #[test]
    fn keeps_the_java_25_post_prologue_counterexample_out_of_scope() {
        // Removing this invocation would make the implicit `super()` run before
        // `mark("P")`, changing the observed order from PS to SP.
        tester().passes(
            "class Parent { Parent() { mark(\"S\"); } static void mark(String value) {} } \
             class Child extends Parent { Child() { mark(\"P\"); super(); } }",
        );
    }

    #[test]
    fn reports_comment_bearing_calls_without_a_fix() {
        tester().reports_without_fix(
            "class A { A() { super /* keep this constructor note */ (); initialize(); } }",
        );
    }

    #[test]
    fn reports_calls_whose_documentation_comment_could_reach_a_declaration_without_a_fix() {
        let tester = tester();
        for next in [
            "class Local {} new Local();",
            "final class Local {}",
            "@Deprecated class Local {}",
            "interface Local {}",
            "enum Local { VALUE }",
            "record Local() {}",
            ";; class Local {}",
            "/** later */ class Local {}",
            "int value = 1; class Local {}",
            "final int value = 1; use(value); interface Local {}",
            "; int value = 1; ; enum Local { VALUE }",
        ] {
            tester.reports_without_fix(&format!(
                "class A {{ A() {{ /** @deprecated Old */ super(); {next} }} }}"
            ));
        }
        tester.reports_without_fix("class A { A() { /// note\n /* c */ super(); class L {} } }");
        tester.reports_without_fix(r"class A { A() { /** d */\u0020super(); class L {} } }");
    }

    #[test]
    fn removes_calls_whose_documentation_comment_stays_detached() {
        let tester = tester();
        for next in [
            "",
            "initialize();",
            "initialize(); class L {}",
            "{ class L {} }",
            "if (ready) {}",
            "; initialize();",
            "int value = 1;",
            "final int value = 1; use(value);",
            "; int value = 1;",
            "int value = 1; { class L {} }",
        ] {
            tester.fixes(
                &format!("class A {{ A() {{ /** @deprecated Old */ super(); {next} }} }}"),
                &format!("class A {{ A() {{ /** @deprecated Old */  {next} }} }}"),
            );
        }
        tester.fixes(
            "class A { A() { /// note\n /* c */ super(); } }",
            "class A { A() { /// note\n /* c */  } }",
        );
        tester.fixes(
            r"class A { A() { /** d */\u0020super(); } }",
            r"class A { A() { /** d */\u0020 } }",
        );
    }

    /// The source after the safe fixes of this rule and of the rules that
    /// delete statements after it.
    fn fix_with_statement_deleting_rules(source: &str) -> String {
        let diagnostics = lint_together(
            &[&REDUNDANT_SUPER_CALL, &REDUNDANT_SEMICOLONS, &UNUSED_LOCAL_VARIABLE],
            source,
        );
        apply_safe_fixes(source, &diagnostics).unwrap().code.into_owned()
    }

    #[test]
    fn keeps_the_comment_off_a_local_class_when_other_fixes_delete_what_separates_them() {
        // Each deletion alone would leave a statement before the class.
        for (between, left) in [
            (";", ""),
            ("int unused = 1;", ""),
            ("; int unused = 1;", " "),
            ("int unused = 1; unused = 2;", " "),
        ] {
            assert_eq!(
                fix_with_statement_deleting_rules(&format!(
                    "class A {{ A() {{ /** @deprecated Old */ super(); {between} class L {{}} }} }}"
                )),
                format!(
                    "class A {{ A() {{ /** @deprecated Old */ super(); {left} class L {{}} }} }}"
                ),
            );
        }
    }

    #[test]
    fn removes_a_detached_call_together_with_statements_other_fixes_delete() {
        for (between, left) in
            [(";", ""), ("int unused = 1;", ""), ("int unused = 1; unused = 2;", " ")]
        {
            assert_eq!(
                fix_with_statement_deleting_rules(&format!(
                    "class A {{ A() {{ /** d */ super(); {between} initialize(); }} }}"
                )),
                format!("class A {{ A() {{ /** d */  {left} initialize(); }} }}"),
            );
        }
    }

    #[test]
    fn removes_calls_between_the_constructor_comment_and_a_later_documentation_comment() {
        tester().fixes(
            "class A { /** Creates. */ A() { super(); /** d */ class L {} } }",
            "class A { /** Creates. */ A() {  /** d */ class L {} } }",
        );
    }

    #[test]
    fn preserves_comments_outside_the_invocation_span() {
        let source = concat!(
            "class A {\r\n",
            "  A() {\r\n",
            "    /* before */ super(); // after\r\n",
            "    initialize();\r\n",
            "  }\r\n",
            "}",
        );
        let expected = concat!(
            "class A {\r\n",
            "  A() {\r\n",
            "    /* before */  // after\r\n",
            "    initialize();\r\n",
            "  }\r\n",
            "}",
        );

        tester().fixes(source, expected);
    }

    #[test]
    fn removes_adjacent_calls_in_separate_constructors_without_a_final_newline() {
        tester().fixes(
            "class A { A() { super(); } A(int value) { super(); work(); } }",
            "class A { A() {  } A(int value) {  work(); } }",
        );
    }
}
