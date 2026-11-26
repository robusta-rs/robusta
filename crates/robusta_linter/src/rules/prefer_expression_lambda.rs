use robusta_ast::{AstKind, Comment, Expression, LambdaBody, ReturnStatement, Statement};
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;
use robusta_semantic::{AstNode, Semantic};
use robusta_span::{GetSpan, Span};
use robusta_syntax::unicode_escape::skip_whitespace;

use crate::fixer::Fix;
use crate::{FixKind, LintContext, Rule, RuleFixMeta};

declare_robusta_lint! {
    /// Reports a lambda block that only returns a value:
    /// `x -> { return x + 1; }` is `x -> x + 1`.
    ///
    /// The fix is unsafe when the returned expression is a JLS §14.8
    /// statement expression: a call, an assignment, an increment or
    /// decrement, or a class instance creation. The block form is
    /// value-compatible but not void-compatible (JLS §15.27.2), while the
    /// expression form of a statement expression is both (JLS §§15.12.2.1,
    /// 15.27.3). The rewrite widens the lambda's target types, which can make
    /// an overloaded call ambiguous or select another overload. Any other
    /// expression keeps the same targets, so its fix is safe.
    PreferExpressionLambda {
        id: PREFER_EXPRESSION_LAMBDA_ID = "prefer-expression-lambda",
        rule: PREFER_EXPRESSION_LAMBDA,
        summary: "Reports lambda blocks that only return a value.",
        category: Style,
        fix: RuleFixMeta::Fixable(FixKind::ALL),
        node_types: [LambdaExpression],
    }
}

impl Rule for PreferExpressionLambda {
    fn run(&self, node: &AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let AstKind::LambdaExpression(lambda) = node.kind() else {
            return;
        };
        let LambdaBody::Block(block) = &lambda.body else {
            return;
        };
        let Some((statement, value)) = returned_value(&block.statements) else {
            return;
        };
        // A statement expression's expression form is also void-compatible, so
        // the rewrite can change which overload the lambda selects.
        let fix_kind =
            if value.is_statement_expression() { FixKind::UNSAFE_FIX } else { FixKind::SAFE_FIX };

        let message = "lambda block only returns a value";
        let diagnostic = RobustaDiagnostic::warning(message, block.span)
            .with_help("Replace the block with its returned expression.");
        let deletions = frame_deletions(context, block.span, statement.span, value.span());
        context.diagnostic_with_fix_of_kind(diagnostic, fix_kind, |fixer| {
            fixer.noop().extend(Vec::from(deletions.map(Fix::delete)))
        });
    }
}

/// A block's lone `return Expression;` and its expression, if that is all it
/// is. A no-value `return` yields none: it is void-compatible only, so no
/// expression body can replace it.
fn returned_value<'a, 'ast>(
    statements: &'a [Statement<'ast>],
) -> Option<(&'a ReturnStatement<'ast>, &'a Expression<'ast>)> {
    match statements {
        [Statement::Return(statement)] => Some((statement, statement.value.as_ref()?)),
        _ => None,
    }
}

/// The spans deleting the `{`, `return`, `;`, and `}` around `value`, each with
/// the whitespace between it and the expression.
///
/// A deletion stops at the comment nearest the expression, so every comment
/// stays in order around the expression. After a `//` comment, the deletion
/// starts at the token itself, keeping the line break that stops the comment
/// from swallowing the tokens after the lambda. The spans come from the
/// statement and expression bounds, so a delimiter spelled with a Unicode
/// escape goes as a whole.
fn frame_deletions(
    semantic: &Semantic<'_>,
    block: Span,
    statement: Span,
    value: Span,
) -> [Span; 4] {
    // Where the comments starting in `start..end` begin, or `end` without any.
    let comments_start = |start, end| {
        semantic.comments_range(start..end).first().map_or(end, |comment| comment.span.start)
    };
    // Where a deletion after the comments starting in `start..end` starts, or
    // `start` without any.
    let comments_end = |start, end| {
        semantic
            .comments_range(start..end)
            .last()
            .map_or(start, |comment| deletion_start(semantic.source_text(), comment))
    };
    [
        Span::new(block.start, comments_start(block.start, statement.start)),
        Span::new(statement.start, comments_start(statement.start, value.start)),
        Span::new(comments_end(value.end, statement.end), statement.end),
        Span::new(comments_end(statement.end, block.end), block.end),
    ]
}

/// Where the deletion after `comment` starts: at its end, or at the next token
/// after a `//` comment, whose line break must stay.
fn deletion_start(source: &str, comment: &Comment) -> u32 {
    if comment.is_line() {
        skip_whitespace(source, comment.span.end, |byte| byte.is_ascii_whitespace())
    } else {
        comment.span.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tester::Tester;

    fn tester() -> Tester {
        Tester::new(&PREFER_EXPRESSION_LAMBDA)
    }

    #[test]
    fn simplifies_value_only_returned_expressions() {
        let source = "class A { void f() { \
             g(x -> { return x + 1; }); \
             g(x -> { return x > 0 ? x : -x; }); \
             g(x -> { return (int) x; }); \
             g(x -> { return x.field; }); \
             g(x -> { return 1; }); \
             g(x -> { return (compute(x)); }); \
             } }";

        let report = tester().fixes(
            source,
            "class A { void f() { \
             g(x -> x + 1); \
             g(x -> x > 0 ? x : -x); \
             g(x -> (int) x); \
             g(x -> x.field); \
             g(x -> 1); \
             g(x -> (compute(x))); \
             } }",
        );

        insta::assert_snapshot!(report, @r#"
        1:29: warning[prefer-expression-lambda]: lambda block only returns a value ("{ return x + 1; }")
        1:56: warning[prefer-expression-lambda]: lambda block only returns a value ("{ return x > 0 ? x : -x; }")
        1:92: warning[prefer-expression-lambda]: lambda block only returns a value ("{ return (int) x; }")
        1:121: warning[prefer-expression-lambda]: lambda block only returns a value ("{ return x.field; }")
        1:150: warning[prefer-expression-lambda]: lambda block only returns a value ("{ return 1; }")
        1:173: warning[prefer-expression-lambda]: lambda block only returns a value ("{ return (compute(x)); }")
        "#);
    }

    #[test]
    fn unsafely_fixes_a_returned_statement_expression() {
        // Each returned expression is a JLS §14.8 statement expression, whose
        // expression form is also void-compatible.
        let source = "class A { void f() { \
             g(x -> { return compute(x); }); \
             g(x -> { return new Wrapper(x); }); \
             g(x -> { return field = x; }); \
             g(x -> { return x++; }); \
             g(x -> { return ++x; }); \
             } }";

        tester().fix_leaves_unchanged(FixKind::SAFE_FIX, source);
        tester().fixes_unsafe(
            source,
            "class A { void f() { \
             g(x -> compute(x)); \
             g(x -> new Wrapper(x)); \
             g(x -> field = x); \
             g(x -> x++); \
             g(x -> ++x); \
             } }",
        );
    }

    #[test]
    fn fixes_the_returned_value_by_kind() {
        tester().fixes(
            "void configure() {\n    choose(x -> { return x + 1; });\n}\n",
            "void configure() {\n    choose(x -> x + 1);\n}\n",
        );

        let source = "void configure() {
    choose(x -> { return compute(x); }); // After the fix, an overload taking a Consumer would make this call ambiguous.
}
";
        tester().fix_leaves_unchanged(FixKind::SAFE_FIX, source);
        tester().fixes_unsafe(
            source,
            "void configure() {
    choose(x -> compute(x)); // After the fix, an overload taking a Consumer would make this call ambiguous.
}
",
        );
    }

    #[test]
    fn leaves_bodies_that_are_not_a_lone_value_return_alone() {
        tester().passes(
            "class A { void f() { \
             g(x -> { return; }); \
             g(x -> { log(x); return x + 1; }); \
             g(x -> { if (x > 0) { return x; } return -x; }); \
             g(x -> { }); \
             g(x -> x + 1); \
             } }",
        );
    }

    #[test]
    fn keeps_comments_written_around_the_deleted_tokens() {
        tester().fixes(
            "class A { IntSupplier f = () -> { return /* units */ 1; }; }",
            "class A { IntSupplier f = () -> /* units */ 1; }",
        );
        tester().fixes(
            "class A { void f() { g(x -> { /* a */ return /* b */ x + 1 /* c */; /* d */ }); } }",
            "class A { void f() { g(x -> /* a */ /* b */ x + 1 /* c */ /* d */); } }",
        );
    }

    #[test]
    fn keeps_the_line_break_that_ends_a_line_comment() {
        tester().fixes(
            "void configure() {
    choose(x -> { // a
        return x + 1; // b
    });
    choose(x -> {
        // c
        return x + 1 // d
        ;
    });
}
",
            "void configure() {
    choose(x -> // a
        x + 1 // b
    );
    choose(x -> // c
        x + 1 // d
        );
}
",
        );
    }

    #[test]
    fn deletes_unicode_escaped_delimiters_whole() {
        tester().fixes(
            r"class A { void f() { g(x -> \u007b \u0072eturn /* c */ x \u003b \uu007d); } }",
            "class A { void f() { g(x -> /* c */ x); } }",
        );
    }

    #[test]
    fn keeps_comments_written_inside_the_returned_expression() {
        let report = tester().fixes(
            "class A { void f() { g(x -> { return x /* keep */ + 1; }); } }",
            "class A { void f() { g(x -> x /* keep */ + 1); } }",
        );

        insta::assert_snapshot!(report, @r#"1:29: warning[prefer-expression-lambda]: lambda block only returns a value ("{ return x /* keep */ + 1; }")"#);
    }

    #[test]
    fn simplifies_a_nested_lambda_to_a_fixpoint() {
        tester().fixes_to_fixpoint(
            FixKind::SAFE_FIX,
            "class A { void f() { g(x -> { return y -> { return x + y; }; }); } }",
            "class A { void f() { g(x -> y -> x + y); } }",
        );
    }
}
