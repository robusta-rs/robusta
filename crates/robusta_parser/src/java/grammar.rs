// Adapted from https://github.com/oxc-project/oxc/blob/ee7d14d6c40d03b0f287bc66834bdf4ec95167ca/crates/oxc_parser/src/js/grammar.rs

//! Cover grammar for assignment: an expression that covers the variable an assignment or
//! `++`/`--` changes.

use crate::*;

pub trait CoverGrammar<T>: Sized {
    fn cover(value: T) -> Result<Self>;
}

impl<'a> CoverGrammar<Expression<'a>> for AssignmentTarget<'a> {
    fn cover(expression: Expression<'a>) -> Result<Self> {
        match expression {
            Expression::Identifier(name) => Ok(Self::Identifier(name)),
            Expression::FieldAccess(access) => Ok(Self::FieldAccess(access)),
            Expression::ArrayAccess(access) => Ok(Self::ArrayAccess(access)),
            Expression::Parenthesized(parenthesized) if is_variable(&parenthesized.expression) => {
                Ok(Self::Parenthesized(parenthesized))
            }
            expression => Err(diagnostics::message(
                "this expression cannot be assigned to",
                expression.span(),
            )),
        }
    }
}

/// Whether an assignment or `++`/`--` can change `expression`: a name, a field, or an array
/// element, which parentheses may enclose (JLS §15.26.1, §15.8.5).
fn is_variable(expression: &Expression<'_>) -> bool {
    matches!(
        expression.without_parentheses(),
        Expression::Identifier(_) | Expression::FieldAccess(_) | Expression::ArrayAccess(_)
    )
}
