use super::*;

/// `<T, U extends Comparable<U>>` of a generic class, interface, record, method,
/// or constructor (JLS §8.1.2, §8.4.4).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct TypeParameters<'a> {
    pub node_id: Cell<NodeId>,
    /// From `<` to `>`.
    pub span: Span,
    pub params: Vec<'a, TypeParameter<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
// Pluralize as `TypeParameterList` to avoid naming clash with `TypeParameters`.
#[plural(TypeParameterList)]
pub struct TypeParameter<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// `@A` in `<@A T>`.
    pub annotations: Vec<'a, Annotation<'a>>,
    pub name: BindingIdentifier<'a>,
    pub bounds: Vec<'a, Type<'a>>,
}

/// A type or contextual `var` in inference-capable variable positions.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum VariableType<'a> {
    Var(Box<'a, VarType>) = 0,
    INHERIT(Type<'a>),
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct VarType {
    pub node_id: Cell<NodeId>,
    pub span: Span,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum Type<'a> {
    Primitive(Box<'a, PrimitiveType<'a>>) = 66,
    Array(Box<'a, ArrayType<'a>>) = 67,
    INHERIT(ClassType<'a>),
}

/// A method's result, or the type of a class literal (JLS 8.4.5, 15.8.2).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum MethodResult<'a> {
    Void(Box<'a, VoidType>) = 0,
    INHERIT(Type<'a>),
}

/// `void`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct VoidType {
    pub node_id: Cell<NodeId>,
    pub span: Span,
}

/// `int`, or `@A int` in a nested type position (JLS 4.2).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct PrimitiveType<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub annotations: Vec<'a, Annotation<'a>>,
    pub kind: PrimitiveTypeKind,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum PrimitiveTypeKind {
    Boolean = 0,
    Byte = 1,
    Char = 2,
    Short = 3,
    Int = 4,
    Long = 5,
    Float = 6,
    Double = 7,
}

/// A class or interface type (JLS 4.3).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum ClassType<'a> {
    Simple(Box<'a, SimpleClassType<'a>>) = 64,
    Qualified(Box<'a, QualifiedClassType<'a>>) = 65,
}

/// `List<String>`, or the `java` of `java.util.List`.
///
/// Leading declaration annotations are modifiers. The annotations here are in
/// nested type positions.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct SimpleClassType<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub annotations: Vec<'a, Annotation<'a>>,
    pub name: IdentifierReference<'a>,
    pub args: Option<Box<'a, TypeArguments<'a>>>,
}

/// `Map.@Nullable Entry<E>` holds `Map`, `@Nullable`, `Entry`, and `E`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct QualifiedClassType<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub qualifier: ClassType<'a>,
    pub annotations: Vec<'a, Annotation<'a>>,
    pub name: IdentifierName<'a>,
    pub args: Option<Box<'a, TypeArguments<'a>>>,
}

/// `int[][]` holds `int` and two dimensions (JLS §10.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ArrayType<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// Never an array type.
    pub element: Type<'a>,
    /// Dimensions in source order. In `int @A [] @B []`, `@A` annotates
    /// `int[][]` and `@B` its component type `int[]` (JLS §9.7.4).
    pub dims: Vec<'a, ArrayTypeDimension<'a>>,
}

/// One `[]` and the annotations before it, e.g. `@A []` in `char @A []` or
/// `String value() @A []`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ArrayTypeDimension<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub annotations: Vec<'a, Annotation<'a>>,
}

/// `<String, T>`, or the `<>` of a class instance creation (JLS §4.5.1, §15.9).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct TypeArguments<'a> {
    pub node_id: Cell<NodeId>,
    /// From `<` to `>`.
    pub span: Span,
    /// Empty for `<>`.
    pub args: Vec<'a, TypeArgument<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
// Pluralize as `TypeArgumentList` to avoid naming clash with `TypeArguments`.
#[plural(TypeArgumentList)]
pub enum TypeArgument<'a> {
    Wildcard(Box<'a, WildcardTypeArgument<'a>>) = 0,
    INHERIT(Type<'a>),
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct WildcardTypeArgument<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// `@A` in `List<@A ?>`.
    pub annotations: Vec<'a, Annotation<'a>>,
    pub bound: Option<WildcardBound<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct WildcardBound<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub kind: WildcardBoundKind,
    pub ty: Type<'a>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum WildcardBoundKind {
    Extends = 0,
    Super = 1,
}
