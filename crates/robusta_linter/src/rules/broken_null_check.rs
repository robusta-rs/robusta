//! `x == null && x.m()` — a null comparison whose short-circuited operand
//! dereferences the value that comparison just proved null.
//!
//! `&&` evaluates its right operand only when the left one is true, so
//! `x == null && x.m()` reaches `x.m()` in exactly the state that makes it
//! throw, and skips it whenever `x` is usable. The dual `x != null || x.m()`
//! is the same defect through `||`. Either way the operand is dead: it throws
//! or it never runs, so the defect holds at any frequency and needs no corpus
//! evidence.
//!
//! Report-only, deliberately. Two different one-token edits repair the
//! canonical shape — swapping the connective (`&&` to `||`) and swapping the
//! comparison (`==` to `!=`) — and they mean different things. Which one is
//! intended depends on whether the author wanted a guard or a disjunction,
//! which the rule cannot know, so it names both in its help text rather than
//! choosing one.
//!
//! Identity is the resolved symbol, not the spelling: a comparison and a
//! dereference that merely share a name are two different values under
//! shadowing, and the rule stays silent there.
//!
//! Unboxing calls a method on the wrapper, so it is a dereference too. The rule
//! has no type inference, so it sees unboxing only where declarations show it:
//! the variable is declared with a `java.lang` wrapper type, and it is a
//! primitive cast's operand, a conditional's condition, or an array index, or
//! the operator decides alone, or the other operand is a literal, a
//! primitive-valued operator, or a variable declared with a primitive or
//! wrapper type.

use robusta_allocator::GetAddress;
use robusta_ast::{AstKind, BinaryExpression, ClassType, Expression, IdentifierReference, Type};
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;
use robusta_semantic::{AstNode, NodeId, ReferenceKind, SymbolId, SymbolKind};
use robusta_span::{GetSpan, Span};
use robusta_syntax::operator::{BinaryOp, UnaryOp};

use crate::{LintContext, Rule, RuleFixMeta, utils::names_java_lang_type};

declare_robusta_lint! {
    /// Reports `x == null && x.m()` and `x != null || x.m()` for a local
    /// variable or parameter `x` that the chain does not assign, where
    /// short-circuit evaluation reaches the dereference only when `x` is null.
    ///
    /// When `x` is declared with a `java.lang` wrapper type, such as `Integer`
    /// or `Boolean`, unboxing it is a dereference too. The rule takes `x` as
    /// unboxed when it is cast to a primitive type, is the condition of `?:` or
    /// an array index, or is the operand of a unary operator, of a binary
    /// operator other than `==`, `!=`, and `+`, of `==` or `!=` with a primitive
    /// other operand, or of `+` with a primitive or wrapper other operand. The
    /// other operand's type is known only when it is a literal, an operator
    /// with a primitive result, such as `-1` or `i * 2`, or a local variable or
    /// parameter whose declaration shows it, so `x + y` with an unknown `y` may
    /// be string concatenation and is not reported.
    BrokenNullCheck {
        id: BROKEN_NULL_CHECK_ID = "broken-null-check",
        rule: BROKEN_NULL_CHECK,
        summary: "Reports a null comparison whose short-circuited operand dereferences the value just proved null.",
        category: Correctness,
        fix: RuleFixMeta::None,
        node_types: [BinaryExpression],
    }
}

/// One matched defect: the guard, the dereference it fails to guard, and the
/// name both resolve to.
struct BrokenGuard<'a> {
    name: &'a str,
    dereference: Span,
}

impl Rule for BrokenNullCheck {
    fn run(&self, node: &AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let AstKind::BinaryExpression(binary) = node.kind() else { return };

        // The connective decides which comparison is the broken one. `&&`
        // reaches its right operand only when the left is true, so `== null`
        // is the guard that inverts; `||` reaches it only when the left is
        // false, so `!= null` is.
        let comparison = match binary.op {
            BinaryOp::And => BinaryOp::Eq,
            BinaryOp::Or => BinaryOp::NotEq,
            _ => return,
        };

        let Some(broken) = broken_guard(
            &binary.lhs,
            Chain { op: binary.op, comparison, node: node.id() },
            &binary.rhs,
            context,
        ) else {
            return;
        };

        let name = broken.name;
        let help = if binary.op == BinaryOp::And {
            format!(
                "`&&` evaluates its right operand only when the left one is true, so `{name}` is \
                 null exactly when it is dereferenced. Either the comparison should be \
                 `!= null` or the connective should be `||`."
            )
        } else {
            format!(
                "`||` evaluates its right operand only when the left one is false, so `{name}` is \
                 null exactly when it is dereferenced. Either the comparison should be \
                 `== null` or the connective should be `&&`."
            )
        };

        context.diagnostic(
            RobustaDiagnostic::warning(
                format!(
                    "`{name}` is compared to null, then dereferenced in the operand that runs \
                     only when that comparison holds"
                ),
                broken.dereference,
            )
            .with_help(help),
        );
    }
}

/// The short-circuiting chain one operand hangs off: its connective, the
/// comparison that inverts under that connective, and the visited link of the
/// chain.
#[derive(Clone, Copy)]
struct Chain {
    op: BinaryOp,
    comparison: BinaryOp,
    node: NodeId,
}

impl Chain {
    /// The span of the whole chain: the outermost expression the visited link
    /// belongs to through parentheses and further links with the same
    /// connective, which may extend it on either side.
    fn span(self, context: &LintContext<'_, '_, '_>) -> Span {
        let nodes = context.nodes();
        let mut span = nodes.kind(self.node).span();
        for ancestor in nodes.ancestor_kinds(self.node) {
            match ancestor {
                AstKind::ParenthesizedExpression(_) => {}
                AstKind::BinaryExpression(binary) if binary.op == self.op => span = binary.span,
                _ => break,
            }
        }
        span
    }
}

/// The first null comparison in `guard` whose value `operand` dereferences.
///
/// `a && x == null && x.m()` parses left-associatively, so the guard for one
/// operand can be any earlier conjunct rather than only the adjacent one; the
/// chain is walked for that reason. Each candidate is tested against `operand`
/// as it is found, so no conjunct list is built.
fn broken_guard<'a>(
    guard: &'a Expression<'a>,
    chain: Chain,
    operand: &Expression<'a>,
    context: &LintContext<'_, '_, '_>,
) -> Option<BrokenGuard<'a>> {
    let Expression::Binary(binary) = without_parentheses(guard) else { return None };

    if binary.op == chain.op {
        return broken_guard(&binary.lhs, chain, operand, context)
            .or_else(|| broken_guard(&binary.rhs, chain, operand, context));
    }
    if binary.op != chain.comparison {
        return None;
    }

    let name = null_compared_name(binary)?;
    let symbol = local_or_parameter(name, context)?;
    // A chain that assigns the compared value anywhere, not just in the
    // guarded operand, is not reported. `s == null && (s = t).isEmpty() &&
    // s.isEmpty()` reassigns `s` in an intermediate conjunct, so the value the
    // last one dereferences need not be the one the guard compared.
    if is_written_within(chain.span(context), symbol, context) {
        return None;
    }
    let dereference = dereference_within(operand.span(), symbol, context)?;
    Some(BrokenGuard { name: name.name, dereference })
}

/// The simple name compared against `null` by `comparison`, in either operand
/// order. A dotted name is refused: only its first name is resolved
/// file-locally, so `a.b == null` and `a.b.c()` cannot be shown to be the same
/// value.
fn null_compared_name<'a>(
    comparison: &'a BinaryExpression<'a>,
) -> Option<&'a IdentifierReference<'a>> {
    let (lhs, rhs) = (without_parentheses(&comparison.lhs), without_parentheses(&comparison.rhs));
    let ((Expression::Identifier(name), Expression::NullLiteral(_))
    | (Expression::NullLiteral(_), Expression::Identifier(name))) = (lhs, rhs)
    else {
        return None;
    };
    Some(name)
}

/// The local variable or parameter a simple name resolves to.
///
/// A field is refused, since a call earlier in the chain can set it.
fn local_or_parameter(
    name: &IdentifierReference<'_>,
    context: &LintContext<'_, '_, '_>,
) -> Option<SymbolId> {
    let scoping = context.scoping();
    let symbol = scoping.reference(name.reference_id()).symbol_id()?;
    let kind = scoping.symbol(symbol).kind();
    (kind.is_local_variable() || matches!(kind, SymbolKind::Parameter | SymbolKind::CatchParameter))
        .then_some(symbol)
}

fn is_written_within(operand: Span, symbol: SymbolId, context: &LintContext<'_, '_, '_>) -> bool {
    references_within(operand, symbol, context)
        .any(|(_, kind)| matches!(kind, ReferenceKind::Write | ReferenceKind::ReadWrite))
}

/// The span of the first dereference of `symbol` inside `operand`.
fn dereference_within(
    operand: Span,
    symbol: SymbolId,
    context: &LintContext<'_, '_, '_>,
) -> Option<Span> {
    let is_wrapper = declared_type(symbol, context) == OperandType::Wrapper;
    references_within(operand, symbol, context)
        .find_map(|(node_id, _)| dereference_at(node_id, is_wrapper, context))
}

fn references_within<'a>(
    operand: Span,
    symbol: SymbolId,
    context: &'a LintContext<'_, '_, '_>,
) -> impl Iterator<Item = (NodeId, ReferenceKind)> + 'a {
    context
        .scoping()
        .references_for_symbol(symbol)
        .filter(move |reference| operand.contains_inclusive(reference.span()))
        .map(|reference| (reference.node_id(), reference.kind()))
}

/// The span of the dereference the name at `node_id` is the target of, or
/// `None` when the name merely stands for its own value: the whole access, or
/// the name itself when it is declared with a wrapper type (`is_wrapper`) and
/// the operator it is an operand of unboxes it (JLS 5.1.8).
fn dereference_at(
    node_id: NodeId,
    is_wrapper: bool,
    context: &LintContext<'_, '_, '_>,
) -> Option<Span> {
    let nodes = context.nodes();
    let kind = nodes.kind(node_id);
    let AstKind::IdentifierReference(name) = kind else { return None };
    let mut unboxable = is_wrapper;
    // The target may be the name inside parentheses and reference casts, which
    // pass its value on unchanged: casting null succeeds (JLS §5.5).
    let mut target = kind;
    for (parent_id, parent) in nodes.ancestors_enumerated(node_id) {
        let parent = parent.kind();
        match parent {
            AstKind::ParenthesizedExpression(_) => target = parent,
            AstKind::CastExpression(cast)
                if cast.operand.address() == target.address()
                    && !matches!(cast.ty, Type::Primitive(_)) =>
            {
                target = parent;
                // The cast's type, not the declared one, decides unboxing.
                unboxable = false;
            }
            AstKind::MethodMember(member) if member.target.address() == target.address() => {
                return Some(nodes.parent_kind(parent_id).span());
            }
            // The access is the whole dotted name.
            AstKind::FieldAccess(access) if access.target.address() == target.address() => {
                let mut outermost = parent;
                for ancestor in nodes.ancestor_kinds(parent_id) {
                    match ancestor {
                        AstKind::FieldAccess(access)
                            if access.target.address() == outermost.address() =>
                        {
                            outermost = ancestor;
                        }
                        _ => break,
                    }
                }
                return Some(outermost.span());
            }
            AstKind::ArrayAccess(access) if access.array.address() == target.address() => {
                return Some(access.span);
            }
            // Evaluating `x::m` throws when `x` is null (JLS §15.13.3).
            AstKind::MethodReference(reference)
                if reference.target.address() == target.address() =>
            {
                return Some(reference.span);
            }
            // A primitive cast unboxes its operand (JLS 5.5).
            AstKind::CastExpression(cast)
                if unboxable && cast.operand.address() == target.address() =>
            {
                return Some(name.span);
            }
            // A `Boolean` condition is unboxed (JLS 15.25).
            AstKind::ConditionalExpression(conditional)
                if unboxable && conditional.condition.address() == target.address() =>
            {
                return Some(name.span);
            }
            // An index is promoted to `int` (JLS 15.10.3).
            AstKind::ArrayAccess(access)
                if unboxable && access.index.address() == target.address() =>
            {
                return Some(name.span);
            }
            // Every unary operator takes a primitive operand (JLS 15.15).
            AstKind::UnaryExpression(_) if unboxable => return Some(name.span),
            AstKind::BinaryExpression(binary) if unboxable => {
                let other = if binary.lhs.address() == target.address() {
                    &binary.rhs
                } else {
                    &binary.lhs
                };
                let unboxes = match binary.op {
                    // Two wrappers compare by reference (JLS 15.21.3).
                    BinaryOp::Eq | BinaryOp::NotEq => {
                        operand_type(other, context) == OperandType::Primitive
                    }
                    // A `String` operand makes `+` concatenation (JLS 15.18).
                    BinaryOp::Add => operand_type(other, context) != OperandType::Other,
                    _ => true,
                };
                return unboxes.then_some(name.span);
            }
            _ => return None,
        }
    }
    None
}

/// What an operand's own text or declaration shows of its type.
#[derive(Clone, Copy, PartialEq, Eq)]
enum OperandType {
    Primitive,
    /// A `java.lang` wrapper class, which unboxes to a primitive (JLS 5.1.8).
    Wrapper,
    /// Any other type, or one the rule cannot see.
    Other,
}

fn operand_type(expression: &Expression<'_>, context: &LintContext<'_, '_, '_>) -> OperandType {
    match without_parentheses(expression) {
        Expression::IntLiteral(_)
        | Expression::FloatLiteral(_)
        | Expression::CharLiteral(_)
        | Expression::BooleanLiteral(_) => OperandType::Primitive,
        // `++x` and `--x` have the type of `x`, which may be a wrapper (JLS 15.15.1).
        Expression::Unary(unary)
            if !matches!(unary.op, UnaryOp::PreIncrement | UnaryOp::PreDecrement) =>
        {
            OperandType::Primitive
        }
        // Every binary operator yields a primitive, except `+` concatenating a
        // `String` (JLS 15.17-15.24).
        Expression::Binary(binary)
            if binary.op != BinaryOp::Add
                || (operand_type(&binary.lhs, context) != OperandType::Other
                    && operand_type(&binary.rhs, context) != OperandType::Other) =>
        {
            OperandType::Primitive
        }
        Expression::Identifier(name) => local_or_parameter(name, context)
            .map_or(OperandType::Other, |symbol| declared_type(symbol, context)),
        _ => OperandType::Other,
    }
}

/// The type a local variable or parameter is declared with. `var`, an array,
/// and a compact constructor's implicit parameter are not looked through.
fn declared_type(symbol: SymbolId, context: &LintContext<'_, '_, '_>) -> OperandType {
    let nodes = context.nodes();
    let declaration = context.scoping().symbol(symbol).declaration_node_id();
    let ty = match nodes.kind(declaration) {
        AstKind::VariableDeclarator(declarator) if declarator.dims.is_empty() => {
            match nodes.parent_kind(declaration) {
                AstKind::LocalVariableDeclaration(local) => local.ty.as_type(),
                _ => None,
            }
        }
        AstKind::FormalParameter(parameter) if !parameter.varargs && parameter.dims.is_empty() => {
            parameter.ty.as_ref().and_then(|ty| ty.as_type())
        }
        AstKind::EnhancedForStatement(statement) if statement.dims.is_empty() => {
            statement.ty.as_type()
        }
        AstKind::TypePattern(pattern) => pattern.ty.as_type(),
        _ => None,
    };
    match ty {
        Some(Type::Primitive(_)) => OperandType::Primitive,
        Some(ty) if ty.as_class_type().is_some_and(|ty| is_wrapper(ty, context)) => {
            OperandType::Wrapper
        }
        _ => OperandType::Other,
    }
}

/// Whether `ty` is a `java.lang` wrapper class, written in full or by a simple
/// name that names it.
fn is_wrapper(ty: &ClassType<'_>, context: &LintContext<'_, '_, '_>) -> bool {
    const WRAPPERS: [&str; 8] =
        ["Boolean", "Byte", "Character", "Double", "Float", "Integer", "Long", "Short"];
    if !WRAPPERS.contains(&ty.name()) {
        return false;
    }
    match ty {
        ClassType::Simple(simple) => names_java_lang_type(context.semantic(), &simple.name),
        ClassType::Qualified(_) => {
            ty.self_and_qualifiers().skip(1).map(ClassType::name).eq(["lang", "java"])
        }
    }
}

fn without_parentheses<'a>(mut expression: &'a Expression<'a>) -> &'a Expression<'a> {
    while let Expression::Parenthesized(parenthesized) = expression {
        expression = &parenthesized.expression;
    }
    expression
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tester::Tester;

    fn tester() -> Tester {
        Tester::new(&BROKEN_NULL_CHECK)
    }

    #[test]
    fn reports_a_method_call_guarded_by_an_inverted_and() {
        let source = "class A { boolean f(String s) { return s == null && s.isEmpty(); } }";
        insta::assert_snapshot!(tester().reports(source), @r#"1:53: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("s.isEmpty()")"#);
    }

    #[test]
    fn reports_dereferences_whatever_runs_them_unless_the_chain_assigns_the_value() {
        let source = r#"class Names {
    String cached;

    void check(String name, String label, Integer count, boolean ready) {
        boolean a = name == null && name.isEmpty(); // Reported.
        boolean b = name != null || name.isEmpty(); // Reported.
        boolean c = name == null && ready && name.isEmpty(); // Reported.
        boolean d = count == null && count > 0; // Reported, since unboxing dereferences count.
        boolean e = name == null && test(name::isEmpty); // Reported, since creating the method reference dereferences name.
        boolean f = name == null && (ready ? name.isEmpty() : true); // Reported, though the branch may not run.
        boolean g = name == null && false && name.isEmpty(); // Reported, though the constant stops the chain first.
        boolean h = name == null && test(() -> name.isEmpty()); // Reported, though the lambda may never run.
        boolean i = name == null && name.valueOf(1).isEmpty(); // Reported, though the static valueOf needs no instance.
        boolean j = label == null && (label = "") != null && label.isEmpty(); // Not reported, since the chain assigns label.
        boolean k = cached == null && load() && cached.isEmpty(); // Not reported, since cached is a field that load() can set.
    }
}
"#;
        insta::assert_snapshot!(tester().reports(source), @r#"
        5:37: warning[broken-null-check]: `name` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("name.isEmpty()")
        6:37: warning[broken-null-check]: `name` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("name.isEmpty()")
        7:46: warning[broken-null-check]: `name` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("name.isEmpty()")
        8:38: warning[broken-null-check]: `count` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("count")
        9:42: warning[broken-null-check]: `name` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("name::isEmpty")
        10:46: warning[broken-null-check]: `name` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("name.isEmpty()")
        11:46: warning[broken-null-check]: `name` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("name.isEmpty()")
        12:48: warning[broken-null-check]: `name` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("name.isEmpty()")
        13:37: warning[broken-null-check]: `name` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("name.valueOf(1)")
        "#);
    }

    #[test]
    fn reports_implicit_compact_constructor_parameters() {
        tester().reports_without_fix(
            "record R(String s) { R { boolean a = s == null && s.isEmpty(); } }",
        );
        tester().passes("record R(String s) { boolean f() { return s == null && s.isEmpty(); } }");
    }

    #[test]
    fn reports_the_or_dual() {
        let source = "class A { boolean f(String s) { return s != null || s.isEmpty(); } }";
        insta::assert_snapshot!(tester().reports(source), @r#"1:53: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("s.isEmpty()")"#);
    }

    #[test]
    fn reports_a_reversed_comparison() {
        let source = "class A { boolean f(String s) { return null == s && s.isEmpty(); } }";
        tester().reports_without_fix(source);
    }

    #[test]
    fn reports_a_field_access_dereference() {
        let source = "class A { int f(int[] a) { return a == null && a.length > 0 ? 1 : 0; } }";
        tester().reports_without_fix(source);
    }

    #[test]
    fn reports_an_array_access_dereference() {
        let source = "class A { boolean f(String[] a) { return a == null && a[0].isEmpty(); } }";
        tester().reports_without_fix(source);
    }

    #[test]
    fn reports_through_parentheses() {
        let source = "class A { boolean f(String s) { return (s == null) && (s.isEmpty()); } }";
        tester().reports_without_fix(source);
    }

    #[test]
    fn reports_a_dereference_through_parentheses_and_reference_casts() {
        let source = r#"class A {
    void f(String s, int[] a, boolean ready) {
        boolean t = s == null && (s).isEmpty(); // Reported.
        boolean u = a == null && (a)[0] == 0; // Reported.
        boolean v = s == null && ((CharSequence) s).length() > 0; // Reported.
        boolean w = s == null && ((Object) ((s))).hashCode() > 0; // Reported.
        boolean x = s != null || test(((String) (Object) s)::isEmpty); // Reported.
        boolean y = s != null && ((CharSequence) s).length() > 0; // Not reported, since the guard is correct.
        boolean z = s == null && (ready ? "" : s).isEmpty(); // Not reported, since the receiver need not be s.
    }

    boolean test(java.util.function.BooleanSupplier supplier) {
        return true;
    }
}
"#;
        insta::assert_snapshot!(tester().reports(source), @r#"
        3:34: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("(s).isEmpty()")
        4:34: warning[broken-null-check]: `a` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("(a)[0]")
        5:34: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("((CharSequence) s).length()")
        6:34: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("((Object) ((s))).hashCode()")
        7:39: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("((String) (Object) s)::isEmpty")
        "#);
    }

    #[test]
    fn reports_when_an_earlier_conjunct_is_the_guard() {
        let source =
            "class A { boolean f(boolean b, String s) { return b && s == null && s.isEmpty(); } }";
        tester().reports_without_fix(source);
    }

    #[test]
    fn the_correct_guard_is_not_reported() {
        tester().passes("class A { boolean f(String s) { return s != null && s.isEmpty(); } }");
        tester().passes("class A { boolean f(String s) { return s == null || s.isEmpty(); } }");
    }

    #[test]
    fn declines_when_the_operands_are_different_symbols() {
        tester().passes(
            "class A { boolean f(String s, String t) { return s == null && t.isEmpty(); } }",
        );
    }

    #[test]
    fn declines_when_the_dereferenced_name_is_a_different_symbol_of_the_same_spelling() {
        // The inner `s` is the anonymous class's own local, not the parameter
        // the guard compared. Matching on spelling would report this; matching
        // on the resolved symbol does not.
        tester().passes(
            "class A { boolean f(String s) { return s == null && new B() { \
             boolean g() { String s = \"\"; return s.isEmpty(); } }.g(); } }",
        );
    }

    #[test]
    fn declines_when_the_operand_reassigns_the_compared_value() {
        tester().passes(
            "class A { boolean f(String s, String t) { return s == null && (s = t).isEmpty() \
             && s.isEmpty(); } }",
        );
    }

    #[test]
    fn reports_unless_the_whole_chain_assigns_the_compared_value() {
        let source = r#"class A {
    void f(String s, String t, boolean ready) {
        boolean a = s == null && s.isEmpty() && (s = "") != null; // Not reported.
        boolean b = s != null || s.isEmpty() || (s = "") == null; // Not reported.
        boolean c = (s == null && s.isEmpty()) && (s = "") != null; // Not reported.
        boolean d = (s = t) != null && (s == null && s.isEmpty()); // Not reported.
        boolean e = s == null && s.isEmpty() && (t = "") != null; // Reported.
        boolean f = s == null && s.isEmpty() || (s = "") != null; // Reported.
        boolean g = ready && (s = t) == null || s == null && s.isEmpty(); // Reported.
        s = t;
        boolean h = s == null && s.isEmpty(); // Reported.
    }
}
"#;
        insta::assert_snapshot!(tester().reports(source), @r#"
        7:34: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("s.isEmpty()")
        8:34: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("s.isEmpty()")
        9:62: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("s.isEmpty()")
        11:34: warning[broken-null-check]: `s` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("s.isEmpty()")
        "#);
    }

    #[test]
    fn declines_a_bare_use_that_is_not_a_dereference() {
        tester().passes("class A { boolean f(String s) { return s == null && s == \"\"; } }");
    }

    #[test]
    fn declines_a_dereference_outside_the_short_circuited_operand() {
        // A real defect, but a statement-sequence one this rule does not model.
        tester().passes(
            "class A { int f(String s) { if (s == null) { return 0; } return s.length(); } }",
        );
    }

    #[test]
    fn declines_a_field_guard_shadowed_by_a_same_named_parameter() {
        // Reduced from trino's `EnforceSingleRowOperator.addInput`, the only
        // textual match for this rule's shape in either private corpus:
        // `this.page` is the field and `page` is the parameter, so the
        // dereference is of a different value than the one compared. Matching
        // on spelling would report this correct code.
        tester().passes(
            "class A { Object page; void f(Object page) { \
             if (this.page != null || page.hashCode() > 1) { return; } } }",
        );
    }

    #[test]
    fn declines_a_qualified_name_whose_identity_is_not_file_local() {
        tester().passes("class A { B b; boolean f() { return b.c == null && b.c.d(); } }");
    }

    #[test]
    fn declines_a_non_short_circuiting_operator() {
        // `&` evaluates both operands regardless, so the guard never had a
        // short-circuit to invert.
        tester().passes("class A { boolean f(String s) { return (s == null) & s.isEmpty(); } }");
    }

    #[test]
    fn declines_a_field_that_an_earlier_operand_can_set() {
        tester().passes(
            "class A { int[] a; boolean init() { a = new int[] {1}; return true; } \
             boolean f() { return a == null && init() && a.length > 0; } }",
        );
    }

    #[test]
    fn reports_a_local_that_an_earlier_operand_cannot_set() {
        tester().reports_without_fix(
            "class A { boolean init() { return true; } \
             boolean f() { int[] a = null; return a == null && init() && a.length > 0; } }",
        );
    }

    #[test]
    fn reports_unboxing_where_the_operator_or_a_declared_operand_shows_it() {
        let source = r"class A {
    void f(Integer n, java.lang.Long l, Boolean b, int i, Integer m) {
        boolean t = n == null && -n > 0;
        boolean u = n == null && (n) == 1;
        boolean v = n == null && n == i;
        boolean w = n == null && m + n > 0;
        boolean x = l == null && l * 2 > 0;
        boolean y = b != null || b;
        boolean z = b == null && !b;
        boolean s = n == null && n == -1;
        boolean r = n == null && n + i * 2 > 0;
    }
}
";
        insta::assert_snapshot!(tester().reports(source), @r#"
        3:35: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        4:35: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        5:34: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        6:38: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        7:34: warning[broken-null-check]: `l` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("l")
        8:34: warning[broken-null-check]: `b` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("b")
        9:35: warning[broken-null-check]: `b` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("b")
        10:34: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        11:34: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        "#);
    }

    #[test]
    fn reports_unboxing_by_a_primitive_cast_a_condition_or_an_index() {
        let source = r"class A {
    void f(java.lang.Integer n, Boolean b, int[] a) {
        boolean t = n == null && (int) n > 0;
        boolean u = n == null && (long) (n) > 0;
        boolean v = b == null && (b ? true : false);
        boolean w = n == null && new int[] {1}[n] > 0;
        boolean x = n != null || a[(n)] > 0;
    }
}
";
        insta::assert_snapshot!(tester().reports(source), @r#"
        3:40: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        4:42: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        5:35: warning[broken-null-check]: `b` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("b")
        6:48: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        7:37: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")
        "#);
    }

    #[test]
    fn declines_a_wrapper_where_its_position_does_not_unbox_it() {
        tester().passes(
            "class A { boolean f(Integer n, Integer m, boolean c) { return n == null && (c ? n : m) == null; } }",
        );
        tester().passes(
            "class A { boolean f(Integer n) { return n == null && new Integer[] {n}.length > 0; } }",
        );
        tester().passes(
            "class A { boolean f(Integer n, Object o) { return n == null && (Object) n == o; } }",
        );
    }

    #[test]
    fn reports_a_wrapper_imported_from_java_lang() {
        let source = "import java.lang.Integer; class A { boolean f(Integer n) { return n == null && n > 0; } }";
        insta::assert_snapshot!(tester().reports(source), @r#"1:80: warning[broken-null-check]: `n` is compared to null, then dereferenced in the operand that runs only when that comparison holds ("n")"#);
    }

    #[test]
    fn declines_a_wrapper_that_need_not_be_unboxed() {
        // Two wrappers compare by reference, and `+` with a `String` or an
        // operand of unknown type may be concatenation.
        tester()
            .passes("class A { boolean f(Integer n, Integer m) { return n == null && n == m; } }");
        tester().passes(
            "class A { boolean f(Integer n) { return n == null && (n + \"\").isEmpty(); } }",
        );
        tester().passes(
            "class A { boolean f(Integer n, String s) { return n == null && (n + s).isEmpty(); } }",
        );
        tester().passes(
            "class A { boolean f(Integer n) { return n == null && test(n); } boolean test(Integer n) { return true; } }",
        );
        tester().passes(
            "class A { boolean f(Integer n, String s) { return n == null && (n + (s + 1)).isEmpty(); } }",
        );
        tester().passes(
            "class A { boolean f(Integer n, Integer m) { return n == null && n == ++m; } }",
        );
    }

    #[test]
    fn declines_a_type_that_shadows_a_wrapper() {
        tester().passes(
            "import p.Integer; class A { boolean f(Integer n) { return n == null && n > 0; } }",
        );
        tester().passes(
            "class A { class Integer {} boolean f(Integer n) { return n == null && n > 0; } }",
        );
        tester()
            .passes("class A { <Integer> boolean f(Integer n) { return n == null && n > 0; } }");
        tester().passes(
            "class B { class Integer {} } class A extends B { boolean f(Integer n) { return n == null && n > 0; } }",
        );
        tester().passes("class A { boolean f(p.Integer n) { return n == null && n > 0; } }");
    }

    #[test]
    fn reports_a_local_variable_guard() {
        let source = "class A { boolean f() { String s = g(); return s == null && s.isEmpty(); } \
             String g() { return null; } }";
        tester().reports_without_fix(source);
    }
}
