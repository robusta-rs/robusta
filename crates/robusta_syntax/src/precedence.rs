// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_syntax/src/precedence.rs

pub trait GetPrecedence {
    fn precedence(&self) -> Precedence;
}

/// Operator Precedence
///
/// The values are meaningful only relative to each other. They follow the
/// JLS §15 expression grammar bottom up, from assignment to primary.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Precedence {
    /// Assignment and lambda expressions.
    Assign = 0,
    Conditional = 1,
    LogicalOr = 2,
    LogicalAnd = 3,
    BitwiseOr = 4,
    BitwiseXor = 5,
    BitwiseAnd = 6,
    Equals = 7,
    /// Relational operators and `instanceof`.
    Compare = 8,
    Shift = 9,
    Add = 10,
    Multiply = 11,
    /// Prefix operators, casts, and `switch` expressions.
    Prefix = 12,
    Postfix = 13,
    /// Array creation, a primary that cannot be indexed (JLS §15.10.3).
    New = 14,
    Member = 15,
}
