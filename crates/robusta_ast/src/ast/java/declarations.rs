use super::*;

#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct CompilationUnit<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// Source-ordered comments, skipped by visitors.
    #[content_eq(skip)]
    pub comments: Vec<'a, Comment>,
    /// Exactly one compilation-unit form (JLS §19).
    pub content: CompilationUnitContent<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// The three mutually exclusive compilation-unit forms (JLS §19).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum CompilationUnitContent<'a> {
    Ordinary(OrdinaryCompilationUnit<'a>) = 0,
    Compact(CompactCompilationUnit<'a>) = 1,
    Modular(ModularCompilationUnit<'a>) = 2,
}

/// `[PackageDeclaration] {ImportDeclaration} {TopLevelClassOrInterfaceDeclaration}` (JLS §19).
///
/// Only this form admits a package declaration.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct OrdinaryCompilationUnit<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub package: Option<PackageDeclaration<'a>>,
    pub imports: Vec<'a, ImportDeclaration<'a>>,
    pub types: Vec<'a, TypeDeclaration<'a>>,
}

/// `{ImportDeclaration} ModuleDeclaration` (JLS §19).
///
/// Imports belong to the unit, directives to the module declaration.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct ModularCompilationUnit<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub imports: Vec<'a, ImportDeclaration<'a>>,
    pub module: ModuleDeclaration<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct PackageDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// Package annotations (JLS §7.4.1), e.g. `@NullMarked package com.example;`.
    pub annotations: Vec<'a, Annotation<'a>>,
    pub name: Name<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct ImportDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub kind: ImportKind,
    pub name: Name<'a>,
    /// The last name, which a single-type or single-static import also binds.
    pub local: Option<BindingIdentifier<'a>>,
}

/// Import forms (JLS §7.5).
///
/// `import module M;` imports a module. `import module.foo.Bar;` imports a type.
#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum ImportKind {
    SingleType = 0,
    TypeOnDemand = 1,
    SingleStatic = 2,
    StaticOnDemand = 3,
    SingleModule = 4,
}

/// Members of an implicitly declared class, without a host-chosen name (JLS §7.3).
///
/// Constructors and initializer blocks are rejected by the parser.
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct CompactCompilationUnit<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub imports: Vec<'a, ImportDeclaration<'a>>,
    #[scope(enter_before)]
    pub members: Vec<'a, ClassMember<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// A module declaration (JLS §7.7).
///
/// Module keywords remain usable as names where the grammar permits.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct ModuleDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// Annotations preceding `[open] module`.
    pub annotations: Vec<'a, Annotation<'a>>,
    /// Whether this is `open module`.
    pub is_open: bool,
    pub name: Name<'a>,
    pub directives: Vec<'a, ModuleDirective<'a>>,
}

/// A module directive (JLS §7.7).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum ModuleDirective<'a> {
    /// `requires {transitive|static} ModuleName;`
    Requires(RequiresDirective<'a>) = 0,
    /// `{exports|opens} PackageName [to ModuleName {, ModuleName}];`
    Package(PackageDirective<'a>) = 1,
    /// `uses TypeName;`
    Uses(UsesDirective<'a>) = 2,
    /// `provides TypeName with TypeName {, TypeName};`
    Provides(ProvidesDirective<'a>) = 3,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct RequiresDirective<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// `transitive`/`static` modifiers in source order, including duplicates.
    pub modifiers: Vec<'a, RequiresModifier>,
    pub name: Name<'a>,
}

#[ast(visit)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq, GetSpan)]
pub struct RequiresModifier {
    pub span: Span,
    pub kind: RequiresModifierKind,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum RequiresModifierKind {
    Transitive = 0,
    Static = 1,
}

/// An `exports` or `opens` directive. Empty `targets` means unqualified.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct PackageDirective<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub kind: PackageDirectiveKind,
    pub name: Name<'a>,
    pub targets: Vec<'a, Name<'a>>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum PackageDirectiveKind {
    Exports = 0,
    Opens = 1,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct UsesDirective<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub name: TypeName<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct ProvidesDirective<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub name: TypeName<'a>,
    /// The non-empty `with` implementation-type list.
    pub with: Vec<'a, TypeName<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum TypeDeclaration<'a> {
    /// A preserved `;` among declarations (JLS §7.6, §8.1.7, §9.1.4, §9.6).
    Empty(Box<'a, EmptyDeclaration>) = 36,
    AnnotationInterface(Box<'a, AnnotationInterfaceDeclaration<'a>>) = 37,
    INHERIT(LocalClassOrInterfaceDeclaration<'a>),
}

/// The declarations a block can hold (JLS §14.3).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum LocalClassOrInterfaceDeclaration<'a> {
    Class(Box<'a, ClassDeclaration<'a>>) = 32,
    Interface(Box<'a, InterfaceDeclaration<'a>>) = 33,
    Enum(Box<'a, EnumDeclaration<'a>>) = 34,
    Record(Box<'a, RecordDeclaration<'a>>) = 35,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct EmptyDeclaration {
    pub node_id: Cell<NodeId>,
    pub span: Span,
}

#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct ClassDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub name: BindingIdentifier<'a>,
    #[scope(enter_before)]
    pub type_params: Option<Box<'a, TypeParameters<'a>>>,
    pub superclass: Option<Type<'a>>,
    pub interfaces: Vec<'a, Type<'a>>,
    /// Direct subclasses in `permits`, empty if absent (JLS §8.1.6).
    pub permits: Vec<'a, TypeName<'a>>,
    pub body: ClassBody<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// The body of a class, record, enum constant, or anonymous class (JLS §8.1.7).
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct ClassBody<'a> {
    pub node_id: Cell<NodeId>,
    /// Covers the opening `{` through the closing `}`.
    pub span: Span,
    pub members: Vec<'a, ClassMember<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// A normal interface (JLS §9.1).
///
/// Annotation interfaces use [`AnnotationInterfaceDeclaration`]. [`InterfaceMember`] excludes class-only members.
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct InterfaceDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub name: BindingIdentifier<'a>,
    #[scope(enter_before)]
    pub type_params: Option<Box<'a, TypeParameters<'a>>>,
    /// Superinterfaces listed after `extends`.
    pub extends: Vec<'a, Type<'a>>,
    /// Direct subtypes in `permits`, empty if absent (JLS §9.1.4).
    pub permits: Vec<'a, TypeName<'a>>,
    pub body: InterfaceBody<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct InterfaceBody<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub members: Vec<'a, InterfaceMember<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct EnumDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub name: BindingIdentifier<'a>,
    pub interfaces: Vec<'a, Type<'a>>,
    pub body: EnumBody<'a>,
}

/// Enum constants followed by optional class members (JLS §8.9.1).
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct EnumBody<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub constants: Vec<'a, EnumConstant<'a>>,
    /// Whether the body has the `;` that ends its constants, required before members (JLS §8.9.1).
    pub has_body_semicolon: bool,
    pub members: Vec<'a, ClassMember<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct EnumConstant<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// Annotation-only modifiers, using the shared modifier container.
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub name: BindingIdentifier<'a>,
    /// `None` for `A`, `Some([])` for `A()`.
    pub args: Option<Vec<'a, Expression<'a>>>,
    /// `None` for `A`, empty members for `A {}` (JLS §8.9.1).
    pub body: Option<ClassBody<'a>>,
}

/// A record with class members, including compact constructors (JLS §8.10).
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct RecordDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub name: BindingIdentifier<'a>,
    #[scope(enter_before)]
    pub type_params: Option<Box<'a, TypeParameters<'a>>>,
    pub header: RecordHeader<'a>,
    pub interfaces: Vec<'a, Type<'a>>,
    pub body: ClassBody<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// The parenthesized component list of a record (JLS §8.10.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct RecordHeader<'a> {
    pub node_id: Cell<NodeId>,
    /// From `(` to `)`.
    pub span: Span,
    pub components: Vec<'a, RecordComponent<'a>>,
}

/// A record header component. Only annotations are permitted as modifiers.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct RecordComponent<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub ty: Type<'a>,
    pub varargs: bool,
    pub varargs_annotations: Vec<'a, Annotation<'a>>,
    pub name: BindingIdentifier<'a>,
}

/// `@interface` declaration (JLS §9.6).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct AnnotationInterfaceDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub name: BindingIdentifier<'a>,
    pub body: AnnotationInterfaceBody<'a>,
}

#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct AnnotationInterfaceBody<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub members: Vec<'a, AnnotationInterfaceMemberDeclaration<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum AnnotationInterfaceMemberDeclaration<'a> {
    Element(Box<'a, AnnotationInterfaceElementDeclaration<'a>>) = 0,
    Field(Box<'a, FieldDeclaration<'a>>) = 1,
    INHERIT(TypeDeclaration<'a>),
}

/// `T name() [] [default value];` in an annotation interface.
///
/// Post-parenthesis dimensions stay separate to preserve their source placement.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct AnnotationInterfaceElementDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub ty: Type<'a>,
    pub name: BindingIdentifier<'a>,
    pub post_paren_dimensions: Vec<'a, ArrayTypeDimension<'a>>,
    pub default_value: Option<ElementValue<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum ClassMember<'a> {
    Field(Box<'a, FieldDeclaration<'a>>) = 0,
    Method(Box<'a, MethodDeclaration<'a>>) = 1,
    Constructor(Box<'a, ConstructorDeclaration<'a>>) = 2,
    /// A record constructor without a parameter list or `throws` (JLS §8.10.4).
    CompactConstructor(Box<'a, CompactConstructorDeclaration<'a>>) = 3,
    /// An instance or static initializer (JLS §8.6, §8.7).
    Initializer(Box<'a, InitializerBlock<'a>>) = 4,
    /// A member type declaration (JLS §8.5, §9.5).
    INHERIT(TypeDeclaration<'a>),
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum InterfaceMember<'a> {
    /// An interface constant using the shared field payload (JLS §9.3).
    Constant(Box<'a, FieldDeclaration<'a>>) = 0,
    /// An interface method using the shared method payload (JLS §9.4).
    Method(Box<'a, MethodDeclaration<'a>>) = 1,
    /// A member type declaration (JLS §9.5).
    INHERIT(TypeDeclaration<'a>),
}

/// `{ … }` or `static { … }` (JLS §8.6, §8.7).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct InitializerBlock<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// Whether this is `static { … }`.
    pub is_static: bool,
    pub block: Block<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct FieldDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub ty: Type<'a>,
    pub declarators: Vec<'a, VariableDeclarator<'a>>,
}

#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct MethodDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub type_params: Option<Box<'a, TypeParameters<'a>>>,
    pub return_type: MethodResult<'a>,
    pub name: BindingIdentifier<'a>,
    pub params: FormalParameters<'a>,
    /// Post-parameter dimensions, e.g. `int f()[]` (JLS §8.4).
    pub post_paren_dimensions: Vec<'a, ArrayTypeDimension<'a>>,
    /// Exception types with type-use annotations, e.g. `throws @Critical IOException` (JLS §8.4.6).
    pub throws: Vec<'a, Type<'a>>,
    /// `None` for a bodyless declaration (`abstract void f();`).
    pub body: Option<Block<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct ConstructorDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub type_params: Option<Box<'a, TypeParameters<'a>>>,
    pub name: IdentifierName<'a>,
    pub params: FormalParameters<'a>,
    /// See [`MethodDeclaration::throws`].
    pub throws: Vec<'a, Type<'a>>,
    pub body: Block<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct CompactConstructorDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub name: IdentifierName<'a>,
    pub body: Block<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// `A this` or `Outer Outer.this` (JLS §8.4.1).
///
/// Carries type-use annotations without declaring a variable.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct ReceiverParameter<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub ty: Type<'a>,
    /// `Outer` in `Outer.this`.
    pub qualifier: Option<IdentifierName<'a>>,
}

/// The parameter list of a method, constructor, or lambda (JLS §8.4.1, §15.27.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub struct FormalParameters<'a> {
    pub node_id: Cell<NodeId>,
    /// From `(` to `)`, or the parameter of a concise lambda `x -> …`.
    pub span: Span,
    /// The receiver parameter, written before the others.
    pub receiver: Option<Box<'a, ReceiverParameter<'a>>>,
    pub items: Vec<'a, FormalParameter<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
// Pluralize as `FormalParameterList` to avoid naming clash with `FormalParameters`.
#[plural(FormalParameterList)]
pub struct FormalParameter<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    /// `None` for an untyped lambda parameter. Only a lambda parameter may be `var`.
    pub ty: Option<VariableType<'a>>,
    /// `T... name`, permitted only as the final parameter.
    pub varargs: bool,
    /// Annotations on `...`, e.g. `Object @Nullable ...`. Empty unless `varargs`.
    pub varargs_annotations: Vec<'a, Annotation<'a>>,
    pub name: BindingIdentifier<'a>,
    /// See [`VariableDeclarator::dims`]. Empty when `varargs`.
    pub dims: Vec<'a, ArrayTypeDimension<'a>>,
}
