use super::*;

/// A name that is never looked up (JLS 6.2), such as `out` in `System.out`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct IdentifierName<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// Name after escape translation and BMP ignorable-character removal.
    /// Ordinary names borrow source text, cooked names the arena.
    /// Use `span` for source spelling.
    pub name: &'a str,
}

/// A name that semantic analysis looks up in the enclosing scopes (JLS 6.5),
/// such as `System` in `System.out`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct IdentifierReference<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// See [`IdentifierName::name`].
    pub name: &'a str,
    /// Set by semantic analysis.
    pub reference_id: Cell<Option<ReferenceId>>,
}

/// What a declaration declares, such as `x` in `int x`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct BindingIdentifier<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// See [`IdentifierName::name`]. Includes unnamed `_`.
    pub name: &'a str,
    /// Set by semantic analysis.
    pub symbol_id: Cell<Option<SymbolId>>,
}

/// A statement label, such as `outer` in `break outer`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct LabelIdentifier<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// See [`IdentifierName::name`].
    pub name: &'a str,
}

/// A package, import, or module name, none of which is looked up.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum Name<'a> {
    Identifier(Box<'a, IdentifierName<'a>>) = 0,
    Qualified(Box<'a, QualifiedName<'a>>) = 1,
}

/// `a.b.c` holds `a.b` and `c`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct QualifiedName<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub qualifier: Name<'a>,
    pub name: IdentifierName<'a>,
}

/// The name of a type, without annotations or type arguments (JLS 6.5).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum TypeName<'a> {
    Identifier(Box<'a, IdentifierReference<'a>>) = 0,
    Qualified(Box<'a, QualifiedTypeName<'a>>) = 1,
}

/// `a.b.C` holds `a.b` and `C`. Only `a` is looked up.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct QualifiedTypeName<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub qualifier: TypeName<'a>,
    pub name: IdentifierName<'a>,
}
