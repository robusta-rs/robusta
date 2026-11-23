//! Java operator vocabulary.

use robusta_ast_macros::ast;
use robusta_span::ContentEq;

use crate::precedence::{GetPrecedence, Precedence};

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum UnaryOp {
    Plus = 0,         // +
    Minus = 1,        // -
    Not = 2,          // !
    BitNot = 3,       // ~
    PreIncrement = 4, // ++
    PreDecrement = 5, // --
}

impl UnaryOp {
    pub fn as_str(self) -> &'static str {
        match self {
            UnaryOp::Plus => "+",
            UnaryOp::Minus => "-",
            UnaryOp::Not => "!",
            UnaryOp::BitNot => "~",
            UnaryOp::PreIncrement => "++",
            UnaryOp::PreDecrement => "--",
        }
    }
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum PostfixOp {
    Increment = 0, // ++
    Decrement = 1, // --
}

impl PostfixOp {
    pub fn as_str(self) -> &'static str {
        match self {
            PostfixOp::Increment => "++",
            PostfixOp::Decrement => "--",
        }
    }
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum BinaryOp {
    Or = 0,     // ||
    And = 1,    // &&
    BitOr = 2,  // |
    BitXor = 3, // ^
    BitAnd = 4, // &
    Eq = 5,     // ==
    NotEq = 6,  // !=
    Lt = 7,     // <
    Gt = 8,     // >
    LtEq = 9,   // <=
    GtEq = 10,  // >=
    Shl = 11,   // <<
    Shr = 12,   // >>
    Ushr = 13,  // >>>
    Add = 14,   // +
    Sub = 15,   // -
    Mul = 16,   // *
    Div = 17,   // /
    Rem = 18,   // %
}

impl BinaryOp {
    pub fn as_str(self) -> &'static str {
        match self {
            BinaryOp::Or => "||",
            BinaryOp::And => "&&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "^",
            BinaryOp::BitAnd => "&",
            BinaryOp::Eq => "==",
            BinaryOp::NotEq => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Gt => ">",
            BinaryOp::LtEq => "<=",
            BinaryOp::GtEq => ">=",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",
            BinaryOp::Ushr => ">>>",
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Rem => "%",
        }
    }
}

impl GetPrecedence for BinaryOp {
    fn precedence(&self) -> Precedence {
        match self {
            BinaryOp::Or => Precedence::LogicalOr,
            BinaryOp::And => Precedence::LogicalAnd,
            BinaryOp::BitOr => Precedence::BitwiseOr,
            BinaryOp::BitXor => Precedence::BitwiseXor,
            BinaryOp::BitAnd => Precedence::BitwiseAnd,
            BinaryOp::Eq | BinaryOp::NotEq => Precedence::Equals,
            BinaryOp::Lt | BinaryOp::Gt | BinaryOp::LtEq | BinaryOp::GtEq => Precedence::Compare,
            BinaryOp::Shl | BinaryOp::Shr | BinaryOp::Ushr => Precedence::Shift,
            BinaryOp::Add | BinaryOp::Sub => Precedence::Add,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => Precedence::Multiply,
        }
    }
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum AssignOp {
    Assign = 0, // =
    Add = 1,    // +=
    Sub = 2,    // -=
    Mul = 3,    // *=
    Div = 4,    // /=
    Rem = 5,    // %=
    BitAnd = 6, // &=
    BitOr = 7,  // |=
    BitXor = 8, // ^=
    Shl = 9,    // <<=
    Shr = 10,   // >>=
    Ushr = 11,  // >>>=
}

impl AssignOp {
    pub fn as_str(self) -> &'static str {
        match self {
            AssignOp::Assign => "=",
            AssignOp::Add => "+=",
            AssignOp::Sub => "-=",
            AssignOp::Mul => "*=",
            AssignOp::Div => "/=",
            AssignOp::Rem => "%=",
            AssignOp::BitAnd => "&=",
            AssignOp::BitOr => "|=",
            AssignOp::BitXor => "^=",
            AssignOp::Shl => "<<=",
            AssignOp::Shr => ">>=",
            AssignOp::Ushr => ">>>=",
        }
    }
}
