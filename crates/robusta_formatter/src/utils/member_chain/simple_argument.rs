// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/utils/member_chain/simple_argument.rs

use robusta_ast::{
    ArrayCreation, ArrayInitializer, AssignmentTarget, Expression, VariableInitializer,
    match_expression,
};

use crate::formatter::JavaFormatter;

/// Tracks whether a call argument is simple, so that a short argument keeps
/// its call's layout.
///
/// A literal, `this`, `super`, or a name is simple, and so is an access, a
/// unary expression, or an array initializer made of simple parts. A call or
/// instance creation is simple while its arguments are simple and few. Each
/// nested call or array counts towards a depth of two, past which nothing is
/// simple.
#[derive(Debug, Clone, Copy)]
pub struct SimpleArgument<'a, 'b> {
    expression: &'b Expression<'a>,
    /// Whether text blocks break every group around them, so that none is simple.
    text_blocks: bool,
}

impl<'a, 'b> SimpleArgument<'a, 'b> {
    pub fn new(expression: &'b Expression<'a>, f: &JavaFormatter<'_, '_>) -> Self {
        Self { expression, text_blocks: f.options().text_blocks }
    }

    fn with(self, expression: &'b Expression<'a>) -> Self {
        Self { expression, ..self }
    }

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

        match self.expression {
            // A text block reads as an array initializer of string literals,
            // whose elements are one level deeper, unless `text_blocks` breaks
            // every group around it.
            Expression::StringLiteral(literal) => {
                !literal.is_text_block() || (depth == 0 && !self.text_blocks)
            }
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
            Expression::Unary(unary) => self.with(&unary.operand).is_simple_impl(depth),
            // An update is simple when it changes a plain variable name.
            Expression::Update(update) => matches!(update.target, AssignmentTarget::Identifier(_)),
            Expression::FieldAccess(access) => self.with(&access.target).is_simple_impl(depth),
            Expression::ArrayAccess(access) => {
                self.with(&access.index).is_simple_impl(depth)
                    && self.with(&access.array).is_simple_impl(depth)
            }
            // A method reference reads as an access to the method.
            Expression::MethodReference(reference) => reference
                .target
                .as_expression()
                .is_none_or(|target| self.with(target).is_simple_impl(depth)),
            Expression::Parenthesized(parenthesized) => {
                self.with(&parenthesized.expression).is_simple_impl(depth)
            }
            Expression::MethodCall(call) => {
                call.target().is_none_or(|target| self.with(target).is_simple_impl(depth))
                    && self.is_simple_arguments(&call.args, depth)
            }
            // An anonymous class is a body, not a simple value.
            Expression::ClassInstanceCreation(creation) => {
                creation.body.is_none() && self.is_simple_arguments(&creation.args, depth)
            }
            Expression::QualifiedClassInstanceCreation(creation) => {
                creation.body.is_none()
                    && self.with(&creation.qualifier).is_simple_impl(depth)
                    && self.is_simple_arguments(&creation.args, depth)
            }
            Expression::ArrayCreation(creation) => self.is_simple_array(creation, depth),
            _ => false,
        }
    }

    /// Whether a call's arguments are simple and few: two at the top level,
    /// one inside another call.
    fn is_simple_arguments(&self, arguments: &'b [Expression<'a>], depth: u8) -> bool {
        arguments.len() + usize::from(depth) <= 2
            && arguments.iter().all(|argument| self.with(argument).is_simple_impl(depth + 1))
    }

    /// An array initializer reads as an array literal. Without one, the
    /// dimension lengths read as a call's arguments.
    fn is_simple_array(&self, creation: &'b ArrayCreation<'a>, depth: u8) -> bool {
        if let Some(init) = &creation.init {
            return self.is_simple_initializer(init, depth);
        }
        let lengths = creation.dims.iter().filter_map(|dim| dim.length.as_ref());
        lengths.clone().count() + usize::from(depth) <= 2
            && lengths.into_iter().all(|length| self.with(length).is_simple_impl(depth + 1))
    }

    fn is_simple_initializer(&self, init: &'b ArrayInitializer<'a>, depth: u8) -> bool {
        depth < 2
            && init.elements.iter().all(|element| match element {
                VariableInitializer::Array(inner) => self.is_simple_initializer(inner, depth + 1),
                it @ match_expression!(VariableInitializer) => {
                    self.with(it.to_expression()).is_simple_impl(depth + 1)
                }
            })
    }
}
