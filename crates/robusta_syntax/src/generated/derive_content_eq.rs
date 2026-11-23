// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/content_eq.rs`.

use robusta_span::ContentEq;

use crate::operator::*;

impl ContentEq for UnaryOp {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl ContentEq for PostfixOp {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl ContentEq for BinaryOp {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl ContentEq for AssignOp {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}
