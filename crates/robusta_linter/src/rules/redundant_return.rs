use robusta_ast::{AstKind, MethodResult};
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;
use robusta_semantic::{AstNode, AstNodes, NodeId};
use robusta_span::{GetSpan, Span};

use crate::fixer::{RuleFix, RuleFixer};
use crate::utils::replace_delimiters;
use crate::{FixKind, LintContext, Rule, RuleFixMeta};

declare_robusta_lint! {
    /// Reports a `return;` that ends its method, constructor, or lambda body.
    ///
    /// Falling through from the return leaves every enclosing statement
    /// without running anything it skips:
    /// `void f() { if (ready) { g(); return; } }` is `void f() { if (ready) { g(); } }`.
    ///
    /// The walk outward stops at a following statement, at a statement group
    /// that falls into the next one, at a loop, whose condition or update
    /// falling through would run, and at a `finally` block, where a return
    /// discards a pending exception. A return that a `break` follows is kept.
    ///
    /// The removal is target-preserving. JLS §14.17 makes the return's target
    /// the enclosing declaration or lambda, so the return transfers control
    /// exactly where falling off the end of the body does. For a lambda, JLS
    /// §15.27.2 makes a block that can complete normally void-compatible and
    /// not value-compatible, with or without the return. Only a `finally`
    /// block that cannot complete normally could stop the body completing
    /// once the return is gone, so in a lambda the walk also stops at a `try`
    /// statement with a `finally` block.
    RedundantReturn {
        id: REDUNDANT_RETURN_ID = "redundant-return",
        rule: REDUNDANT_RETURN,
        summary: "Reports a no-value return that ends a void method, constructor, or void-compatible lambda block.",
        category: Pedantic,
        fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
        node_types: [ReturnStatement],
    }
}

impl Rule for RedundantReturn {
    fn run(&self, node: &AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let AstKind::ReturnStatement(statement) = node.kind() else {
            return;
        };
        if statement.value.is_some() || !ends_body(context.nodes(), node.id(), statement.span) {
            return;
        }

        // An `if` branch or a labeled statement needs a statement in its place.
        let needs_block = !matches!(
            context.nodes().parent_kind(node.id()),
            AstKind::Block(_) | AstKind::SwitchBlockStatementGroup(_)
        );
        let message = "the trailing return is redundant";
        let diagnostic = RobustaDiagnostic::warning(message, statement.span)
            .with_help("Remove the trailing `return` statement.");
        context
            .diagnostic_with_fix(diagnostic, |fixer| build_fix(fixer, statement.span, needs_block));
    }
}

// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_linter/src/rules/eslint/no_useless_return.rs
/// Whether falling through from the statement at `span` leaves every
/// enclosing statement, and then the body of a void method, constructor, or
/// lambda, without running anything.
fn ends_body(nodes: &AstNodes<'_>, id: NodeId, span: Span) -> bool {
    let mut child = span;
    let mut through_finally = false;
    for (ancestor_id, ancestor) in nodes.ancestors_enumerated(id) {
        let ends_with = |statements: &[_]| statements.last().map(GetSpan::span) == Some(child);
        match ancestor.kind() {
            AstKind::Block(block) => {
                if let AstKind::TryStatement(statement) = nodes.parent_kind(ancestor_id)
                    && statement.finally_block.as_ref().is_some_and(|f| f.span == block.span)
                {
                    return false;
                }
                if !ends_with(&block.statements) {
                    return false;
                }
            }
            AstKind::SwitchBlockStatementGroup(group) => {
                // A return cannot leave a switch expression (JLS §15.28.1).
                let AstKind::SwitchStatement(switch) = nodes.parent_kind(ancestor_id) else {
                    return false;
                };
                if !ends_with(&group.statements)
                    || switch.cases.last().map(GetSpan::span) != Some(group.span)
                {
                    return false;
                }
            }
            AstKind::TryStatement(statement) => {
                through_finally |= statement.finally_block.is_some();
            }
            AstKind::IfStatement(_)
            | AstKind::LabeledStatement(_)
            | AstKind::SynchronizedStatement(_)
            | AstKind::CatchClause(_)
            | AstKind::SwitchRule(_)
            | AstKind::SwitchStatement(_) => {}
            // A no-value return in a non-void method is invalid Java, so the
            // declared type decides the case without resolving anything.
            AstKind::MethodDeclaration(method) => {
                return matches!(method.return_type, MethodResult::Void(_));
            }
            AstKind::ConstructorDeclaration(_) => return true,
            AstKind::LambdaExpression(_) => return !through_finally,
            _ => return false,
        }
        child = ancestor.kind().span();
    }
    false
}

/// Drop the `return` and its `;`, keeping whatever was written between them,
/// in a block when `needs_block`.
///
/// A no-value return holds no expression, so its span runs from the keyword
/// through the semicolon and deleting it whole would delete any comment inside
/// it. Splicing the interior back instead keeps those bytes exactly as written
/// — including the line terminator that ends a trailing `//` comment.
fn build_fix(fixer: RuleFixer<'_>, statement: Span, needs_block: bool) -> RuleFix {
    let replacements = if needs_block { ["{", "}"] } else { ["", ""] };
    replace_delimiters(fixer, statement, ["return", ";"], replacements)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tester::Tester;

    fn tester() -> Tester {
        Tester::new(&REDUNDANT_RETURN)
    }

    #[test]
    fn removes_a_trailing_return_from_every_owner() {
        let source = "class A { \
             A() { init(); return; } \
             void f() { g(); return; } \
             void empty() { return; } \
             void lambda() { run(() -> { g(); return; }); } \
             }";

        let report = tester().fixes(
            source,
            "class A { \
             A() { init();  } \
             void f() { g();  } \
             void empty() {  } \
             void lambda() { run(() -> { g();  }); } \
             }",
        );

        insta::assert_snapshot!(report, @r#"
        1:25: warning[redundant-return]: the trailing return is redundant ("return;")
        1:51: warning[redundant-return]: the trailing return is redundant ("return;")
        1:76: warning[redundant-return]: the trailing return is redundant ("return;")
        1:119: warning[redundant-return]: the trailing return is redundant ("return;")
        "#);
    }

    #[test]
    fn removes_a_nested_return_that_ends_the_body() {
        let source = "class A { \
             void branch(boolean dirty) { if (dirty) { flush(); return; } } \
             void otherwise(boolean dirty) { if (dirty) { flush(); } else { return; } } \
             void group(int mode) { switch (mode) { case 1: flush(); break; default: reload(); return; } } \
             void rule(int mode) { switch (mode) { case 1 -> { return; } default -> flush(); } } \
             void attempt() { try { flush(); return; } catch (Exception e) { log(e); } } \
             void caught() { try { flush(); } catch (Exception e) { log(e); return; } } \
             void cleanup() { try (Lock lock = lock()) { flush(); return; } finally { unlock(); } } \
             void guarded() { synchronized (this) { flush(); return; } } \
             void labeled() { done: { flush(); return; } } \
             void nested() { { { return; } } } \
             }";

        tester().fixes(
            source,
            "class A { \
             void branch(boolean dirty) { if (dirty) { flush();  } } \
             void otherwise(boolean dirty) { if (dirty) { flush(); } else {  } } \
             void group(int mode) { switch (mode) { case 1: flush(); break; default: reload();  } } \
             void rule(int mode) { switch (mode) { case 1 -> {  } default -> flush(); } } \
             void attempt() { try { flush();  } catch (Exception e) { log(e); } } \
             void caught() { try { flush(); } catch (Exception e) { log(e);  } } \
             void cleanup() { try (Lock lock = lock()) { flush();  } finally { unlock(); } } \
             void guarded() { synchronized (this) { flush();  } } \
             void labeled() { done: { flush();  } } \
             void nested() { { {  } } } \
             }",
        );
    }

    #[test]
    fn wraps_the_kept_text_where_java_requires_a_statement() {
        tester().fixes(
            "class A { \
             void branch(boolean ready) { if (ready) return; else return /* why */; } \
             void labeled() { done: return; } \
             }",
            "class A { \
             void branch(boolean ready) { if (ready) {} else { /* why */} } \
             void labeled() { done: {} } \
             }",
        );
    }

    #[test]
    fn removes_only_the_returns_that_end_their_body() {
        let source = "class A {
void refresh(int mode, boolean ready, boolean dirty) {
    if (mode == 0) {
        pick(() -> {
            reload();
            return; // Falling through still limits the lambda to void-returning targets.
        });
        flush();
        return; // Falling through skips the other branches and ends the method.
    } else if (dirty) {
        switch (mode) {
            case 1:
                if (ready) {
                    return; // Falling through would reach the break.
                }
                break;
            default:
                reload();
                return; // Falling through leaves the switch and ends the method.
        }
    } else {
        if (!ready) {
            return; // Falling through would reach the declaration.
        }
        int attempts;
    }
}
}
";
        let expected = "class A {
void refresh(int mode, boolean ready, boolean dirty) {
    if (mode == 0) {
        pick(() -> {
            reload();
             // Falling through still limits the lambda to void-returning targets.
        });
        flush();
         // Falling through skips the other branches and ends the method.
    } else if (dirty) {
        switch (mode) {
            case 1:
                if (ready) {
                    return; // Falling through would reach the break.
                }
                break;
            default:
                reload();
                 // Falling through leaves the switch and ends the method.
        }
    } else {
        if (!ready) {
            return; // Falling through would reach the declaration.
        }
        int attempts;
    }
}
}
";
        tester().fixes(source, expected);
    }

    #[test]
    fn leaves_returns_that_falling_through_would_not_end_with() {
        tester().passes(
            "class A { \
             void guard(boolean ready) { if (!ready) { return; } g(); } \
             void declared(boolean ready) { if (!ready) { return; } int attempts; } \
             void breaks(int mode, boolean ready) { switch (mode) { case 1: if (ready) { flush(); return; } break; } } \
             void falls(int mode) { switch (mode) { case 1: return; default: g(); } } \
             void trailing(int mode) { switch (mode) { case 1: return; default: } } \
             void forever(boolean ready) { while (true) { if (ready) { return; } break; } } \
             void next(boolean ready) { while (next()) { if (ready) { return; } break; } } \
             void loop(boolean ready) { do { return; } while (ready); } \
             void each(int[] values) { for (int value : values) { return; } } \
             void cleanup() { try { g(); } finally { return; } } \
             void deeper() { try { g(); } finally { if (ready()) { return; } } } \
             void inner(boolean ready) { class B { void f() { if (ready) { return; } g(); } } g(); } \
             void lambda(boolean ready) { run(() -> { if (ready) { return; } g(); }); } \
             void finished() { run(() -> { try { g(); return; } finally { h(); } }); } \
             }",
        );
    }

    #[test]
    fn leaves_value_returns_and_bodyless_declarations_alone() {
        tester().passes(
            "abstract class A { \
             abstract void f(); \
             int value() { return 1; } \
             void lambda() { run(() -> { return get(); }); } \
             void expression() { run(() -> g()); } \
             void none() { g(); } \
             }",
        );
    }

    #[test]
    fn leaves_a_no_value_return_in_a_non_void_method_alone() {
        // `int f() { return; }` does not compile; reporting it would claim the
        // fix keeps the program valid, and it would not.
        tester().passes("class A { int f() { return; } }");
    }

    #[test]
    fn leaves_a_compact_constructor_alone() {
        tester().passes("record R(int x) { R { return; } }");
    }

    #[test]
    fn keeps_comments_written_inside_the_return() {
        // CRLF throughout: the line comment's own terminator is spliced back
        // from the source, never synthesized.
        let source = concat!(
            "class A {\r\n",
            "  void block() { g(); return /* why */; }\r\n",
            "  void line() { g(); return // why\r\n",
            "  ; }\r\n",
            "}",
        );
        let expected = concat!(
            "class A {\r\n",
            "  void block() { g();  /* why */ }\r\n",
            "  void line() { g();  // why\r\n",
            "   }\r\n",
            "}",
        );

        let report = tester().fixes(source, expected);

        insta::assert_snapshot!(report, @r#"
        2:23: warning[redundant-return]: the trailing return is redundant ("return /* why */;")
        3:22: warning[redundant-return]: the trailing return is redundant ("return // why\r\n  ;")
        "#);
    }

    #[test]
    fn removes_a_return_whose_delimiters_are_unicode_escaped() {
        let expected = "class A { void f() { g();  } }";
        tester().fixes(r"class A { void f() { g(); \u0072eturn; } }", expected);
        tester().fixes(r"class A { void f() { g(); return\u003B } }", expected);
        tester().fixes(
            r"class A { void f() { g(); re\uu0074urn /* why */ \uuu003b } }",
            "class A { void f() { g();  /* why */  } }",
        );
    }

    #[test]
    fn removes_adjacent_trailing_returns_in_separate_bodies() {
        tester().fixes(
            "class A { void f() { return; } void g() { return; } }",
            "class A { void f() {  } void g() {  } }",
        );
    }

    #[test]
    fn removes_a_trailing_return_without_a_final_newline() {
        tester().fixes("class A { void f() { return; } }", "class A { void f() {  } }");
    }
}
