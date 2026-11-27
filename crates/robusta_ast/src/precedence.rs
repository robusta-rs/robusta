// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_ast/src/precedence.rs

//! [Expression precedence trait](GetPrecedence) implementations.
use robusta_syntax::precedence::{GetPrecedence, Precedence};

use crate::ast::Expression;

impl GetPrecedence for Expression<'_> {
    fn precedence(&self) -> Precedence {
        match self {
            Self::Assignment(_) | Self::Lambda(_) => Precedence::Assign,
            Self::Conditional(_) => Precedence::Conditional,
            Self::Binary(binary) => binary.op.precedence(),
            Self::Instanceof(_) => Precedence::Compare,
            Self::Update(update) => {
                if update.prefix {
                    Precedence::Prefix
                } else {
                    Precedence::Postfix
                }
            }
            Self::Unary(_) | Self::Cast(_) | Self::Switch(_) => Precedence::Prefix,
            Self::ArrayCreation(_) => Precedence::New,
            Self::IntLiteral(_)
            | Self::FloatLiteral(_)
            | Self::CharLiteral(_)
            | Self::StringLiteral(_)
            | Self::BooleanLiteral(_)
            | Self::NullLiteral(_)
            | Self::This(_)
            | Self::Super(_)
            | Self::QualifiedThis(_)
            | Self::QualifiedSuper(_)
            | Self::Identifier(_)
            | Self::FieldAccess(_)
            | Self::MethodCall(_)
            | Self::ClassLiteral(_)
            | Self::ClassInstanceCreation(_)
            | Self::QualifiedClassInstanceCreation(_)
            | Self::ArrayAccess(_)
            | Self::Parenthesized(_)
            | Self::MethodReference(_) => Precedence::Member,
        }
    }
}
