use robusta_ast::{AssignmentTarget, Expression};

/// The expression that starts `expression`, such as `a` in `a.b()`, `a + b`,
/// `a ? b : c`, or `(a)`, whose parentheses stay on its first token, or
/// `expression` itself when it starts with its own token or the name it assigns.
pub fn leftmost<'e, 'a>(mut expression: &'e Expression<'a>) -> &'e Expression<'a> {
    loop {
        expression = match expression {
            Expression::Parenthesized(parenthesized) => &parenthesized.expression,
            Expression::FieldAccess(access) => &access.target,
            Expression::ArrayAccess(access) => &access.array,
            Expression::MethodCall(call) => match call.target() {
                Some(target) => target,
                None => return expression,
            },
            Expression::MethodReference(reference) => match reference.target.as_expression() {
                Some(target) => target,
                None => return expression,
            },
            Expression::QualifiedClassInstanceCreation(creation) => &creation.qualifier,
            Expression::Postfix(postfix) => &postfix.operand,
            Expression::Binary(binary) => &binary.lhs,
            Expression::Instanceof(instanceof) => &instanceof.operand,
            Expression::Assignment(assignment) => match &assignment.target {
                AssignmentTarget::FieldAccess(access) => &access.target,
                AssignmentTarget::ArrayAccess(access) => &access.array,
                AssignmentTarget::Parenthesized(parenthesized) => &parenthesized.expression,
                AssignmentTarget::Identifier(_) => return expression,
            },
            Expression::Conditional(conditional) => &conditional.condition,
            _ => return expression,
        };
    }
}
