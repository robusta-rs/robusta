use std::cell::Cell;

use robusta_ast_macros::ast;
use robusta_span::{ContentEq, GetSpan, Span};
use robusta_syntax::node::NodeId;

/// An integer literal (JLS §3.10.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct IntLiteral<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub raw: &'a str,
}

/// A floating-point literal (JLS §3.10.2).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct FloatLiteral<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub raw: &'a str,
}

/// A character literal (JLS §3.10.4).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct CharLiteral<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub raw: &'a str,
}

/// A string literal or text block (JLS §3.10.5, §3.10.6).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct StringLiteral<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub raw: &'a str,
}

/// `true` or `false` (JLS §3.10.3).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct BooleanLiteral {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub value: bool,
}

/// `null` (JLS §3.10.8).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct NullLiteral {
    pub node_id: Cell<NodeId>,
    pub span: Span,
}
