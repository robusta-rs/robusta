use super::*;

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum Modifier<'a> {
    Keyword(KeywordModifier) = 0,
    Annotation(Box<'a, Annotation<'a>>) = 1,
}

#[ast(visit)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct KeywordModifier {
    pub span: Span,
    pub kind: ModifierKind,
}

/// Marker, single-element, or normal annotation (JLS §9.7).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct Annotation<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub name: TypeName<'a>,
    /// `None` for `@Foo`, `Some([])` for `@Foo()`.
    pub args: Option<Vec<'a, AnnotationArgument<'a>>>,
}

/// `x = 1` or `1` in `@Foo(...)`. A single-element annotation holds one value.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum AnnotationArgument<'a> {
    Pair(Box<'a, ElementValuePair<'a>>) = 66,
    INHERIT(ElementValue<'a>),
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ElementValuePair<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub name: IdentifierName<'a>,
    pub value: ElementValue<'a>,
}

/// A conditional expression, nested annotation, or array initializer (JLS §9.7.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum ElementValue<'a> {
    Annotation(Box<'a, Annotation<'a>>) = 64,
    Array(Box<'a, ElementValueArrayInitializer<'a>>) = 65,
    INHERIT(Expression<'a>),
}

/// `{v, …}` with its optional trailing comma.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ElementValueArrayInitializer<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub values: Vec<'a, ElementValue<'a>>,
    pub trailing_comma: bool,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum ModifierKind {
    Public = 0,
    Protected = 1,
    Private = 2,
    Abstract = 3,
    /// Interface-method `default`, excluding switch labels (JLS §9.4).
    Default = 4,
    Static = 5,
    Final = 6,
    Native = 7,
    /// Contextual `sealed` in class/interface headers.
    Sealed = 8,
    /// Contextual `non-sealed` in class/interface headers.
    NonSealed = 9,
    Strictfp = 10,
    Synchronized = 11,
    Transient = 12,
    Volatile = 13,
}
