use super::*;

#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct Block<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub statements: Vec<'a, Statement<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum Statement<'a> {
    Block(Box<'a, Block<'a>>) = 0,
    /// A preserved empty statement `;` (JLS §14.6).
    Empty(Box<'a, EmptyStatement>) = 1,
    /// `this(…)` or `super(…)`. Java 25 permits statements before and after it.
    ConstructorInvocation(Box<'a, ConstructorInvocation<'a>>) = 2,
    LocalVariable(Box<'a, LocalVariableDeclaration<'a>>) = 3,
    Expression(Box<'a, ExpressionStatement<'a>>) = 4,
    If(Box<'a, IfStatement<'a>>) = 5,
    Switch(Box<'a, SwitchStatement<'a>>) = 6,
    While(Box<'a, WhileStatement<'a>>) = 7,
    Do(Box<'a, DoStatement<'a>>) = 8,
    For(Box<'a, ForStatement<'a>>) = 9,
    EnhancedFor(Box<'a, EnhancedForStatement<'a>>) = 10,
    Break(Box<'a, BreakStatement<'a>>) = 11,
    Continue(Box<'a, ContinueStatement<'a>>) = 12,
    Throw(Box<'a, ThrowStatement<'a>>) = 13,
    Assert(Box<'a, AssertStatement<'a>>) = 14,
    Try(Box<'a, TryStatement<'a>>) = 15,
    Synchronized(Box<'a, SynchronizedStatement<'a>>) = 16,
    Return(Box<'a, ReturnStatement<'a>>) = 17,
    Yield(Box<'a, YieldStatement<'a>>) = 18,
    Labeled(Box<'a, LabeledStatement<'a>>) = 19,
    /// A local type declaration (JLS §14.3).
    INHERIT(LocalClassOrInterfaceDeclaration<'a>),
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct EmptyStatement {
    pub node_id: Cell<NodeId>,
    pub span: Span,
}

/// `if (…) … [else …]` (JLS §14.9).
/// An `else` binds to the nearest unmatched `if`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct IfStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub condition: Expression<'a>,
    pub then_branch: Statement<'a>,
    pub else_branch: Option<Statement<'a>>,
}

/// `switch (selector) { … }` as a statement (JLS §14.11).
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct SwitchStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub selector: Expression<'a>,
    #[scope(enter_before)]
    pub cases: Vec<'a, SwitchCase<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// An arrow rule or a colon-label group, never mixed in one switch.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum SwitchCase<'a> {
    Rule(Box<'a, SwitchRule<'a>>) = 0,
    Group(Box<'a, SwitchBlockStatementGroup<'a>>) = 1,
}

/// One `case … ->` or `default ->` rule.
/// Spans through its expression semicolon, block, or throw statement.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct SwitchRule<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub label: SwitchLabel<'a>,
    pub body: SwitchRuleBody<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum SwitchRuleBody<'a> {
    Block(Box<'a, Block<'a>>) = 64,
    Throw(Box<'a, ThrowStatement<'a>>) = 65,
    INHERIT(Expression<'a>),
}

/// Colon labels and their statements (JLS §14.11.1).
/// Trailing labels before `}` have no statements.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct SwitchBlockStatementGroup<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub labels: Vec<'a, SwitchLabel<'a>>,
    pub statements: Vec<'a, Statement<'a>>,
}

/// A label excluding the enclosing group's `:` or rule's `->`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan)]
pub enum SwitchLabel<'a> {
    Case(Box<'a, CaseLabel<'a>>) = 0,
    Pattern(Box<'a, CasePatternLabel<'a>>) = 1,
    Default(DefaultLabel) = 2,
}

/// `case c1, c2` with conditional-expression constants.
/// `has_default` marks `case null, default`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct CaseLabel<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub constants: Vec<'a, Expression<'a>>,
    pub has_default: bool,
}

/// `case Pattern, Pattern [when guard]` (JLS §14.11.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct CasePatternLabel<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub patterns: Vec<'a, Pattern<'a>>,
    pub guard: Option<Expression<'a>>,
}

/// `default` without `case` (JLS §14.11.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct DefaultLabel {
    pub node_id: Cell<NodeId>,
    pub span: Span,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct WhileStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub condition: Expression<'a>,
    pub body: Statement<'a>,
}

/// `do … while (…);` (JLS §14.13).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct DoStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub body: Statement<'a>,
    pub condition: Expression<'a>,
}

/// `for (init; cond; update) …` (JLS §14.14.1).
/// Each header clause is optional.
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ForStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub init: Option<ForInit<'a>>,
    pub condition: Option<Expression<'a>>,
    pub update: Option<StatementExpressionList<'a>>,
    pub body: Statement<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// A declaration or statement-expression list in `for` (JLS §14.14.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum ForInit<'a> {
    Declaration(Box<'a, LocalVariableDeclaration<'a>>) = 0,
    Expressions(Box<'a, StatementExpressionList<'a>>) = 1,
}

/// `i = 0, j = 1` in a basic `for` header, one expression or several (JLS §14.14.1).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct StatementExpressionList<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub expressions: Vec<'a, Expression<'a>>,
}

/// `for (T name : iterable) …` (JLS §14.14.2).
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct EnhancedForStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub ty: VariableType<'a>,
    pub name: BindingIdentifier<'a>,
    /// See [`VariableDeclarator::dims`].
    pub dims: Vec<'a, ArrayTypeDimension<'a>>,
    pub iterable: Expression<'a>,
    pub body: Statement<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct BreakStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub label: Option<LabelIdentifier<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ContinueStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub label: Option<LabelIdentifier<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ThrowStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub value: Expression<'a>,
}

/// `assert condition;` or `assert condition : message;` (JLS §14.10).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct AssertStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub condition: Expression<'a>,
    pub message: Option<Expression<'a>>,
}

/// `try (resources)? block catch* finally?` (JLS §14.20).
/// Without resources, at least one catch or finally block is required.
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct TryStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// Resource specifications, empty for a classic `try` (JLS §14.20.3).
    pub resources: Vec<'a, Resource<'a>>,
    /// Preserved trailing `;` in `try (r;)`, meaningful only with resources.
    pub resources_trailing_semicolon: bool,
    pub block: Block<'a>,
    #[scope(exit_before)]
    pub catches: Vec<'a, CatchClause<'a>>,
    pub finally_block: Option<Block<'a>>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// A resource declaration or access to an effectively-final variable (JLS §14.20.3).
/// Accesses are names or fields, excluding arbitrary expressions.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum Resource<'a> {
    Declaration(Box<'a, ResourceDeclaration<'a>>) = 2,
    INHERIT(ResourceAccess<'a>),
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum ResourceAccess<'a> {
    Identifier(Box<'a, IdentifierReference<'a>>) = 0,
    /// `owner.resource`, `primary.resource`, `super.resource`, or `TypeName.super.resource`.
    Field(Box<'a, FieldAccess<'a>>) = 1,
}

/// One initialized resource variable, `{modifier} Type name = init`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ResourceDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub ty: VariableType<'a>,
    pub name: BindingIdentifier<'a>,
    pub init: Expression<'a>,
}

/// `catch (T | U name) block` with multi-catch type alternatives.
#[ast(visit)]
#[scope]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct CatchClause<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub types: Vec<'a, Type<'a>>,
    pub name: BindingIdentifier<'a>,
    pub block: Block<'a>,
    pub scope_id: Cell<Option<ScopeId>>,
}

/// `synchronized (lock) block` (JLS §14.19).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct SynchronizedStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub lock: Expression<'a>,
    pub body: Block<'a>,
}

/// `label: statement`, targeted by labeled `break`/`continue` (JLS §14.7).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct LabeledStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub label: LabelIdentifier<'a>,
    pub statement: Statement<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ConstructorInvocation<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    /// The qualifier in `outer.super(…)`, absent for direct invocations (JLS §8.8.7.1).
    pub qualifier: Option<Expression<'a>>,
    /// Type arguments in `<T>this(…)` or `[outer.]<T>super(…)`.
    pub type_args: Option<Box<'a, TypeArguments<'a>>>,
    pub kind: ConstructorInvocationKind,
    pub args: Vec<'a, Expression<'a>>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(ContentEq)]
pub enum ConstructorInvocationKind {
    This = 0,
    Super = 1,
}

/// An assignment, increment/decrement, method call, or class creation (JLS §14.8).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ExpressionStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub expression: Expression<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct LocalVariableDeclaration<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub modifiers: Vec<'a, Modifier<'a>>,
    pub ty: VariableType<'a>,
    pub declarators: Vec<'a, VariableDeclarator<'a>>,
}

/// `return [value];` (JLS §14.17).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ReturnStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub value: Option<Expression<'a>>,
}

/// `yield value;` in a switch expression (JLS §14.21).
/// Contextual `yield` remains a name in statement expressions such as `yield++`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct YieldStatement<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub value: Expression<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct VariableDeclarator<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub name: BindingIdentifier<'a>,
    /// Post-name dimensions, e.g. `b[]` in `int a, b[]` (JLS §10.2).
    pub dims: Vec<'a, ArrayTypeDimension<'a>>,
    pub init: Option<VariableInitializer<'a>>,
}

/// An expression or array initializer after `=` (JLS §8.3).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetAddress, GetSpan)]
pub enum VariableInitializer<'a> {
    Array(Box<'a, ArrayInitializer<'a>>) = 64,
    INHERIT(Expression<'a>),
}

/// `{a, b, c}` with nestable initializers (JLS §10.6).
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(ContentEq, GetSpan, UnstableAddress)]
pub struct ArrayInitializer<'a> {
    pub node_id: Cell<NodeId>,
    pub span: Span,
    pub elements: Vec<'a, VariableInitializer<'a>>,
}
