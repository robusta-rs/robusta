// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/utils/member_chain/simple_argument.rs

use robusta_ast::{
    ArrayCreation, ArrayInitializer, Expression, VariableInitializer, match_expression,
};
use robusta_syntax::operator::UnaryOp;

/// Tracks whether a call argument is simple, so that a short argument keeps
/// its call's layout.
///
/// A literal, `this`, `super`, or a name is simple, and so is an access, a
/// unary expression, or an array initializer made of simple parts. A call or
/// instance creation is simple while its arguments are simple and few. Each
/// nested call or array counts towards a depth of two, past which nothing is
/// simple.
#[derive(Debug)]
pub struct SimpleArgument<'a, 'b>(&'b Expression<'a>);

impl<'a, 'b> SimpleArgument<'a, 'b> {
    pub fn is_simple(&self) -> bool {
        self.is_simple_impl(0)
    }

    pub fn is_simple_with_depth(&self, depth: u8) -> bool {
        self.is_simple_impl(depth)
    }

    fn is_simple_impl(&self, depth: u8) -> bool {
        if depth >= 2 {
            return false;
        }

        match self.0 {
            // A text block reads as an array initializer of string literals,
            // whose elements are one level deeper.
            Expression::StringLiteral(literal) => !literal.is_text_block() || depth == 0,
            Expression::IntLiteral(_)
            | Expression::FloatLiteral(_)
            | Expression::CharLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::This(_)
            | Expression::Super(_)
            | Expression::QualifiedThis(_)
            | Expression::QualifiedSuper(_)
            | Expression::Identifier(_)
            | Expression::ClassLiteral(_) => true,
            Expression::Unary(unary) => match unary.op {
                UnaryOp::Not | UnaryOp::Minus | UnaryOp::Plus | UnaryOp::BitNot => {
                    Self(&unary.operand).is_simple_impl(depth)
                }
                UnaryOp::PreIncrement | UnaryOp::PreDecrement => is_simple_name(&unary.operand),
            },
            Expression::Postfix(postfix) => is_simple_name(&postfix.operand),
            Expression::FieldAccess(access) => Self(&access.target).is_simple_impl(depth),
            Expression::ArrayAccess(access) => {
                Self(&access.index).is_simple_impl(depth)
                    && Self(&access.array).is_simple_impl(depth)
            }
            // A method reference reads as an access to the method.
            Expression::MethodReference(reference) => reference
                .target
                .as_expression()
                .is_none_or(|target| Self(target).is_simple_impl(depth)),
            Expression::Parenthesized(parenthesized) => {
                Self(&parenthesized.expression).is_simple_impl(depth)
            }
            Expression::MethodCall(call) => {
                call.target().is_none_or(|target| Self(target).is_simple_impl(depth))
                    && Self::is_simple_arguments(&call.args, depth)
            }
            // An anonymous class is a body, not a simple value.
            Expression::ClassInstanceCreation(creation) => {
                creation.body.is_none() && Self::is_simple_arguments(&creation.args, depth)
            }
            Expression::QualifiedClassInstanceCreation(creation) => {
                creation.body.is_none()
                    && Self(&creation.qualifier).is_simple_impl(depth)
                    && Self::is_simple_arguments(&creation.args, depth)
            }
            Expression::ArrayCreation(creation) => Self::is_simple_array(creation, depth),
            _ => false,
        }
    }

    /// Whether a call's arguments are simple and few: two at the top level,
    /// one inside another call.
    fn is_simple_arguments(arguments: &'b [Expression<'a>], depth: u8) -> bool {
        arguments.len() + usize::from(depth) <= 2
            && arguments.iter().all(|argument| Self(argument).is_simple_impl(depth + 1))
    }

    /// An array initializer reads as an array literal. Without one, the
    /// dimension lengths read as a call's arguments.
    fn is_simple_array(creation: &'b ArrayCreation<'a>, depth: u8) -> bool {
        if let Some(init) = &creation.init {
            return Self::is_simple_initializer(init, depth);
        }
        let lengths = creation.dims.iter().filter_map(|dim| dim.length.as_ref());
        lengths.clone().count() + usize::from(depth) <= 2
            && lengths.into_iter().all(|length| Self(length).is_simple_impl(depth + 1))
    }

    fn is_simple_initializer(init: &'b ArrayInitializer<'a>, depth: u8) -> bool {
        depth < 2
            && init.elements.iter().all(|element| match element {
                VariableInitializer::Array(inner) => Self::is_simple_initializer(inner, depth + 1),
                it @ match_expression!(VariableInitializer) => {
                    Self(it.to_expression()).is_simple_impl(depth + 1)
                }
            })
    }
}

impl<'a, 'b> From<&'b Expression<'a>> for SimpleArgument<'a, 'b> {
    fn from(expression: &'b Expression<'a>) -> Self {
        Self(expression)
    }
}

/// An update's operand is simple when it is a plain variable name.
fn is_simple_name(operand: &Expression<'_>) -> bool {
    matches!(operand, Expression::Identifier(_))
}
