use super::*;

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum Expression<'a> {
    IntLiteral(Box<'a, IntLiteral<'a>>) = 0,
    FloatLiteral(Box<'a, FloatLiteral<'a>>) = 1,
    CharLiteral(Box<'a, CharLiteral<'a>>) = 2,
    StringLiteral(Box<'a, StringLiteral<'a>>) = 3,
    BooleanLiteral(Box<'a, BooleanLiteral>) = 4,
    NullLiteral(Box<'a, NullLiteral>) = 5,
    This(Box<'a, ThisExpression>) = 6,
    Super(Box<'a, Super>) = 7,
    /// The enclosing instance `ClassName.this` (JLS §15.8.4).
    QualifiedThis(Box<'a, QualifiedThis<'a>>) = 8,
    /// Qualified superclass target for [`FieldAccess`], [`MethodCall`], or
    /// [`MethodReference`].
    QualifiedSuper(Box<'a, QualifiedSuper<'a>>) = 9,
    /// A simple name, the first of a dotted one included (JLS 6.5).
    Identifier(Box<'a, IdentifierReference<'a>>) = 10,
    FieldAccess(Box<'a, FieldAccess<'a>>) = 11,
    MethodCall(Box<'a, MethodCall<'a>>) = 12,
    ClassLiteral(Box<'a, ClassLiteral<'a>>) = 13,
    ClassInstanceCreation(Box<'a, ClassInstanceCreation<'a>>) = 14,
    QualifiedClassInstanceCreation(Box<'a, QualifiedClassInstanceCreation<'a>>) = 15,
    ArrayCreation(Box<'a, ArrayCreation<'a>>) = 16,
    ArrayAccess(Box<'a, ArrayAccess<'a>>) = 17,
    Update(Box<'a, UpdateExpression<'a>>) = 18,
    Unary(Box<'a, UnaryExpression<'a>>) = 19,
    Cast(Box<'a, CastExpression<'a>>) = 20,
    Instanceof(Box<'a, InstanceofExpression<'a>>) = 21,
    Binary(Box<'a, BinaryExpression<'a>>) = 22,
    Assignment(Box<'a, AssignmentExpression<'a>>) = 23,
    Conditional(Box<'a, ConditionalExpression<'a>>) = 24,
    Switch(Box<'a, SwitchExpression<'a>>) = 25,
    /// Explicit parentheses, preserved by the formatter.
    Parenthesized(Box<'a, ParenthesizedExpression<'a>>) = 26,
    Lambda(Box<'a, LambdaExpression<'a>>) = 27,
    MethodReference(Box<'a, MethodReference<'a>>) = 28,
}

/// `this.x`, `f().x`, or `a.x`. A dotted name is built from the left, so `a.b.c`
/// holds `a.b` and `c`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct FieldAccess<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub target: Expression<'a>,
    pub name: IdentifierName<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct MethodCall<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub callee: Callee<'a>,
    pub args: Vec<'a, Expression<'a>>,
}

/// The method a [`MethodCall`] invokes: `f` in `f()`, or `a.f` in `a.f()`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum Callee<'a> {
    Identifier(Box<'a, IdentifierReference<'a>>) = 0,
    Member(Box<'a, MethodMember<'a>>) = 1,
}

/// `target.<T>method`, the callee of `target.<T>method()`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct MethodMember<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub target: Expression<'a>,
    pub type_args: Option<Box<'a, TypeArguments<'a>>>,
    pub name: IdentifierName<'a>,
}

/// `Type.class` or `void.class` (JLS §15.8.2).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ClassLiteral<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub ty: MethodResult<'a>,
}

/// `this` (JLS §15.8.3).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ThisExpression {
    pub node_id: Cell<NodeId>,
    pub span: Span,
}

/// `super`, the target of a superclass member access or method reference.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct Super {
    pub node_id: Cell<NodeId>,
    pub span: Span,
}

/// `ClassName.this` with a plain, non-generic qualifier (JLS §15.8.4).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct QualifiedThis<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub qualifier: TypeName<'a>,
}

/// `ClassName.super` with a plain, non-generic qualifier.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct QualifiedSuper<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub qualifier: TypeName<'a>,
}

/// `new <T> Foo<…>(…)` with an optional anonymous body (JLS §15.9).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ClassInstanceCreation<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// Constructor type arguments in `new <T> Foo()`.
    pub constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
    /// The instantiated type, whose last name holds empty type arguments for
    /// an inferred `<>` (JLS §15.9.1).
    pub ty: ClassType<'a>,
    pub args: Vec<'a, Expression<'a>>,
    /// `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    pub body: Option<ClassBody<'a>>,
}

/// `outer.new <T> Inner<…>(…)` with an optional anonymous body (JLS §15.9).
///
/// `Inner` is a member of `outer`'s type (JLS §15.9.1), so it is never looked up
/// and this node holds the class's annotations, name, and type arguments.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct QualifiedClassInstanceCreation<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub qualifier: Expression<'a>,
    /// See [`ClassInstanceCreation::constructor_type_args`].
    pub constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
    pub annotations: Vec<'a, Annotation<'a>>,
    pub name: IdentifierName<'a>,
    /// Empty for an inferred `<>` (JLS §15.9.1).
    pub type_args: Option<Box<'a, TypeArguments<'a>>>,
    pub args: Vec<'a, Expression<'a>>,
    /// See [`ClassInstanceCreation::body`].
    pub body: Option<ClassBody<'a>>,
}

/// `new T[n]…[]…` or `new T[]… {…}` (JLS §15.10.1).
/// The element type excludes dimensions.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ArrayCreation<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub element: Type<'a>,
    /// Dimensions in source order, sized before empty. All empty with an initializer.
    pub dims: Vec<'a, ArrayDimension<'a>>,
    pub init: Option<ArrayInitializer<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ArrayDimension<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// `@A` in `new int @A [4]`.
    pub annotations: Vec<'a, Annotation<'a>>,
    /// `None` for an empty `[]` dimension.
    pub length: Option<Expression<'a>>,
}

/// `array[index]` (JLS §15.10.3).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ArrayAccess<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub array: Expression<'a>,
    pub index: Expression<'a>,
}

/// `++x`, `x++`, `--x`, or `x--` (JLS §15.14.2–3, §15.15.1–2).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct UpdateExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub op: UpdateOp,
    /// Whether `op` comes before the target, as in `++x`.
    pub prefix: bool,
    pub target: AssignmentTarget<'a>,
}

/// `+x`, `-x`, `~x`, or `!x` (JLS §15.15.3–6).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct UnaryExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub op: UnaryOp,
    pub operand: Expression<'a>,
}

/// `(Type) operand` or `(Type & Bound …) operand` (JLS §15.16).
/// `(long) x * y` casts only `x`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct CastExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub ty: Type<'a>,
    /// Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    pub additional_bounds: Vec<'a, Type<'a>>,
    pub operand: Expression<'a>,
}

/// `operand instanceof Type` or `operand instanceof Pattern` (JLS §15.20.2).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct InstanceofExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub operand: Expression<'a>,
    pub rhs: InstanceofRhs<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum InstanceofRhs<'a> {
    INHERIT(Type<'a>),
    INHERIT(Pattern<'a>),
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum Pattern<'a> {
    TypePattern(Box<'a, TypePattern<'a>>) = 72,
    RecordPattern(Box<'a, RecordPattern<'a>>) = 73,
}

/// `String s`, `final var value`, or `Foo _` in pattern positions.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct TypePattern<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub ty: VariableType<'a>,
    pub name: BindingIdentifier<'a>,
}

/// `Point(int x, int y)` with nested patterns or `_` components.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct RecordPattern<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub ty: Type<'a>,
    pub components: Vec<'a, ComponentPattern<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum ComponentPattern<'a> {
    MatchAll(Box<'a, MatchAllPattern>) = 0,
    INHERIT(Pattern<'a>),
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct MatchAllPattern {
    pub node_id: Cell<NodeId>,
    pub span: Span,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct BinaryExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub op: BinaryOp,
    pub lhs: Expression<'a>,
    pub rhs: Expression<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct AssignmentExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub target: AssignmentTarget<'a>,
    pub op: AssignOp,
    pub value: Expression<'a>,
}

/// The variable an assignment or update changes: a name, field access, or array access,
/// which parentheses may enclose (JLS §15.26.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum AssignmentTarget<'a> {
    Identifier(Box<'a, IdentifierReference<'a>>) = 0,
    FieldAccess(Box<'a, FieldAccess<'a>>) = 1,
    ArrayAccess(Box<'a, ArrayAccess<'a>>) = 2,
    Parenthesized(Box<'a, ParenthesizedExpression<'a>>) = 3,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ConditionalExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub condition: Expression<'a>,
    pub when_true: Expression<'a>,
    pub when_false: Expression<'a>,
}

/// `switch (selector) { … }` as an expression (JLS §15.28).
/// Colon cases yield values through [`YieldStatement`].
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct SwitchExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub selector: Expression<'a>,
    #[scope(enter_before)]
    pub cases: Vec<'a, SwitchItem<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ParenthesizedExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub expression: Expression<'a>,
}

/// `params -> body` at assignment precedence (JLS §15.27).
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct LambdaExpression<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub params: FormalParameters<'a>,
    pub body: LambdaBody<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress)]
pub enum LambdaBody<'a> {
    Block(Box<'a, Block<'a>>) = 64,
    INHERIT(Expression<'a>),
}

/// `target::method` or `Type::new` (JLS §15.13).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct MethodReference<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub target: MethodReferenceTarget<'a>,
    /// Type arguments after `::`.
    pub type_args: Option<Box<'a, TypeArguments<'a>>>,
    /// The referenced method, or `None` for `::new`.
    pub method: Option<IdentifierName<'a>>,
}

/// A method reference target (JLS §15.13.1).
///
/// A plain dotted name stays an expression, ambiguous without name resolution
/// (JLS 6.5). Type arguments or array suffixes require `Type`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum MethodReferenceTarget<'a> {
    INHERIT(Expression<'a>),
    INHERIT(Type<'a>),
}
