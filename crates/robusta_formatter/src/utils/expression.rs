use robusta_ast::{AssignmentTarget, Expression};

/// The expression that starts `expression`, such as `a` in `a.b()`, `a + b`,
/// `a ? b : c`, or `(a)`, whose parentheses stay on its first token, or
/// `expression` itself when it starts with its own token or the name it changes.
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
            Expression::Update(update) if !update.prefix => match target_start(&update.target) {
                Some(start) => start,
                None => return expression,
            },
            Expression::Binary(binary) => &binary.lhs,
            Expression::Instanceof(instanceof) => &instanceof.operand,
            Expression::Assignment(assignment) => match target_start(&assignment.target) {
                Some(start) => start,
                None => return expression,
            },
            Expression::Conditional(conditional) => &conditional.condition,
            _ => return expression,
        };
    }
}

/// The expression that starts `target`, or `None` when `target` is a name.
fn target_start<'e, 'a>(target: &'e AssignmentTarget<'a>) -> Option<&'e Expression<'a>> {
    match target {
        AssignmentTarget::FieldAccess(access) => Some(&access.target),
        AssignmentTarget::ArrayAccess(access) => Some(&access.array),
        AssignmentTarget::Parenthesized(parenthesized) => Some(&parenthesized.expression),
        AssignmentTarget::Identifier(_) => None,
    }
}
