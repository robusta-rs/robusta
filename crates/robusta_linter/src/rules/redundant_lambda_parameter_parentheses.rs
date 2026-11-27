use robusta_ast::AstKind;
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;
use robusta_semantic::AstNode;

use crate::utils::keep_interior;
use crate::{FixKind, LintContext, Rule, RuleFixMeta};

declare_robusta_lint! {
    /// Reports parentheses around one concise lambda parameter: `(value) ->
    /// value` is `value -> value`.
    ///
    /// JLS §15.27.1 defines the singleton concise forms `(x)` and `x` as
    /// interchangeable. Typed, `var`, annotated, final, varargs, empty, and
    /// multi-parameter lists stay parenthesized because they use different
    /// grammar productions.
    RedundantLambdaParameterParentheses {
        id: REDUNDANT_LAMBDA_PARAMETER_PARENTHESES_ID,
        rule: REDUNDANT_LAMBDA_PARAMETER_PARENTHESES,
        summary: "Reports parentheses around one concise lambda parameter.",
        category: Style,
        fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
        node_types: [LambdaExpression],
    }
}

impl Rule for RedundantLambdaParameterParentheses {
    fn run(&self, node: &AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let AstKind::LambdaExpression(lambda) = node.kind() else {
            return;
        };
        if !lambda.params.has_parentheses() {
            return;
        }
        let [parameter] = &lambda.params.items[..] else {
            return;
        };
        if parameter.ty.is_some()
            || !parameter.modifiers.is_empty()
            || parameter.varargs
            || !parameter.varargs_annotations.is_empty()
        {
            return;
        }

        let diagnostic = RobustaDiagnostic::warning(
            "parentheses around the concise lambda parameter are redundant",
            lambda.params.span,
        )
        .with_help("Remove the parentheses around this parameter.");
        context.diagnostic_with_fix(diagnostic, |fixer| {
            keep_interior(fixer, lambda.params.span, "(", ")")
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tester::Tester;

    fn tester() -> Tester {
        Tester::new(&REDUNDANT_LAMBDA_PARAMETER_PARENTHESES)
    }

    #[test]
    fn removes_parentheses_from_one_concise_parameter() {
        let report = tester().fixes(
            "class A { void f() { g((value) -> value); g((_)->1); } }",
            "class A { void f() { g(value -> value); g(_->1); } }",
        );

        insta::assert_snapshot!(report, @r#"
        1:24: warning[redundant-lambda-parameter-parentheses]: parentheses around the concise lambda parameter are redundant ("(value)")
        1:45: warning[redundant-lambda-parameter-parentheses]: parentheses around the concise lambda parameter are redundant ("(_)")
        "#);
    }

    #[test]
    fn removes_parentheses_around_a_name_comments_or_an_underscore() {
        tester().fixes(
            "import java.util.function.IntUnaryOperator;

class Example {
    IntUnaryOperator named() {
        return(value) -> value + 1;
    }
    IntUnaryOperator commented = (value /* input
    */) -> value + 1;
    IntUnaryOperator unnamed = (_) -> 1;
    IntUnaryOperator typed = (int value) -> value + 1;
}
",
            "import java.util.function.IntUnaryOperator;

class Example {
    IntUnaryOperator named() {
        return value -> value + 1;
    }
    IntUnaryOperator commented = value /* input
    */ -> value + 1;
    IntUnaryOperator unnamed = _ -> 1;
    IntUnaryOperator typed = (int value) -> value + 1;
}
",
        );
    }

    #[test]
    fn separates_the_parameter_from_a_preceding_keyword() {
        tester().fixes(
            "class A { \
             F f() { return(value)->value; } \
             F g(int i) { return switch (i) { default -> { yield(_)->1; } }; } \
             F h() { return (value) -> value; } \
             F k() { return(/* c */value)->value; } \
             F m() { return(ä)->ä; } \
             }",
            "class A { \
             F f() { return value->value; } \
             F g(int i) { return switch (i) { default -> { yield _->1; } }; } \
             F h() { return value -> value; } \
             F k() { return/* c */value->value; } \
             F m() { return ä->ä; } \
             }",
        );
        tester().fixes(
            r"class A { F f() { \u0072eturn(\u0078)->x; } }",
            r"class A { F f() { \u0072eturn \u0078->x; } }",
        );
        tester().fixes(
            r"class A { F f() { return\u0028x)->x; } F g(int i) { return switch (i) { default -> { yield\u0028_\u0029->1; } }; } }",
            r"class A { F f() { return x->x; } F g(int i) { return switch (i) { default -> { yield _->1; } }; } }",
        );
        tester().fixes(
            r"class A { F f() { \u0072eturn\u0028\u0078\u0029\u002d>x; } }",
            r"class A { F f() { \u0072eturn \u0078\u002d>x; } }",
        );
    }

    #[test]
    fn leaves_required_and_absent_parentheses_alone() {
        tester().passes(
            "class A { void f() { \
             g(value -> value); \
             g(() -> 1); \
             g((left, right) -> left + right); \
             g((String value) -> value); \
             g((var value) -> value); \
             g((@Nullable var value) -> value); \
             g((final String value) -> value); \
             g((String... values) -> values.length); \
             } }",
        );
        tester().passes(
            r"class A { void f() { g(\u0028left, right\u0029 -> left); g(\u0028String value\u0029 -> value); } }",
        );
    }

    #[test]
    fn preserves_every_byte_inside_the_delimiters() {
        let source = "class A { void f() {\n\
            g(( /* input */ value /* retained */ ) -> value);\n\
            g((// leading\n\
            value) -> value);\n\
            g((value // trailing\n\
            ) -> value);\n\
            } }";
        let expected = concat!(
            "class A { void f() {\n",
            "g( /* input */ value /* retained */  -> value);\n",
            "g(// leading\n",
            "value -> value);\n",
            "g(value // trailing\n",
            " -> value);\n",
            "} }",
        );

        let report = tester().fixes(source, expected);

        insta::assert_snapshot!(report, @r#"
        2:3: warning[redundant-lambda-parameter-parentheses]: parentheses around the concise lambda parameter are redundant ("( /* input */ value /* retained */ )")
        3:3: warning[redundant-lambda-parameter-parentheses]: parentheses around the concise lambda parameter are redundant ("(// leading\nvalue)")
        5:3: warning[redundant-lambda-parameter-parentheses]: parentheses around the concise lambda parameter are redundant ("(value // trailing\n)")
        "#);
    }

    #[test]
    fn removes_unicode_escaped_parentheses_and_keeps_the_interior() {
        let class =
            |parameters: &str| format!("class A {{ void f() {{ g({parameters} -> x); }} }}");
        for (parameter_list, interior) in [
            (r"\u0028x)", "x"),
            (r"(x\u0029", "x"),
            (r"\u0028x\u0029", "x"),
            (r"\uuuu0028x)", "x"),
            (r"(x\uuuu0029", "x"),
            (
                r"\uu0028 /* before */ \u0078 /* after */ \uuu0029",
                r" /* before */ \u0078 /* after */ ",
            ),
            ("\\u0028// before\r\nx // after\r\n\\u0029", "// before\r\nx // after\r\n"),
        ] {
            tester().fixes(&class(parameter_list), &class(interior));
        }

        let report = tester().fixes(
            r"class A { void f() { g(\u0028x\u0029 -> x); } }",
            "class A { void f() { g(x -> x); } }",
        );

        insta::assert_snapshot!(report, @r#"1:24: warning[redundant-lambda-parameter-parentheses]: parentheses around the concise lambda parameter are redundant ("\\u0028x\\u0029")"#);
    }

    #[test]
    fn preserves_escaped_parameter_spelling_inside_literal_parentheses() {
        tester().fixes(
            r"class A { void f() { g(( /* before */ \u0078 /* after */ ) -> x); } }",
            r"class A { void f() { g( /* before */ \u0078 /* after */  -> x); } }",
        );
    }
}
