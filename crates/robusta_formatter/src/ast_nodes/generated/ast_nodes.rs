// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/formatter/ast_nodes.rs`.

use crate::ast_nodes::AstNode;
use robusta_allocator::Vec;
use robusta_ast::*;
use robusta_span::{GetSpan, Span};

#[derive(Clone, Copy)]
pub enum AstNodes<'parent, 'a> {
    Dummy,
    CompilationUnit(&'parent AstNode<'parent, 'a, CompilationUnit<'a>>),
    OrdinaryCompilationUnit(&'parent AstNode<'parent, 'a, OrdinaryCompilationUnit<'a>>),
    ModularCompilationUnit(&'parent AstNode<'parent, 'a, ModularCompilationUnit<'a>>),
    PackageDeclaration(&'parent AstNode<'parent, 'a, PackageDeclaration<'a>>),
    ImportDeclaration(&'parent AstNode<'parent, 'a, ImportDeclaration<'a>>),
    CompactCompilationUnit(&'parent AstNode<'parent, 'a, CompactCompilationUnit<'a>>),
    ModuleDeclaration(&'parent AstNode<'parent, 'a, ModuleDeclaration<'a>>),
    RequiresDirective(&'parent AstNode<'parent, 'a, RequiresDirective<'a>>),
    PackageDirective(&'parent AstNode<'parent, 'a, PackageDirective<'a>>),
    UsesDirective(&'parent AstNode<'parent, 'a, UsesDirective<'a>>),
    ProvidesDirective(&'parent AstNode<'parent, 'a, ProvidesDirective<'a>>),
    EmptyDeclaration(&'parent AstNode<'parent, 'a, EmptyDeclaration>),
    ClassDeclaration(&'parent AstNode<'parent, 'a, ClassDeclaration<'a>>),
    ClassBody(&'parent AstNode<'parent, 'a, ClassBody<'a>>),
    InterfaceDeclaration(&'parent AstNode<'parent, 'a, InterfaceDeclaration<'a>>),
    InterfaceBody(&'parent AstNode<'parent, 'a, InterfaceBody<'a>>),
    EnumDeclaration(&'parent AstNode<'parent, 'a, EnumDeclaration<'a>>),
    EnumBody(&'parent AstNode<'parent, 'a, EnumBody<'a>>),
    EnumConstant(&'parent AstNode<'parent, 'a, EnumConstant<'a>>),
    RecordDeclaration(&'parent AstNode<'parent, 'a, RecordDeclaration<'a>>),
    RecordHeader(&'parent AstNode<'parent, 'a, RecordHeader<'a>>),
    RecordComponent(&'parent AstNode<'parent, 'a, RecordComponent<'a>>),
    AnnotationInterfaceDeclaration(
        &'parent AstNode<'parent, 'a, AnnotationInterfaceDeclaration<'a>>,
    ),
    AnnotationInterfaceBody(&'parent AstNode<'parent, 'a, AnnotationInterfaceBody<'a>>),
    AnnotationInterfaceElementDeclaration(
        &'parent AstNode<'parent, 'a, AnnotationInterfaceElementDeclaration<'a>>,
    ),
    InitializerBlock(&'parent AstNode<'parent, 'a, InitializerBlock<'a>>),
    FieldDeclaration(&'parent AstNode<'parent, 'a, FieldDeclaration<'a>>),
    MethodDeclaration(&'parent AstNode<'parent, 'a, MethodDeclaration<'a>>),
    ConstructorDeclaration(&'parent AstNode<'parent, 'a, ConstructorDeclaration<'a>>),
    CompactConstructorDeclaration(&'parent AstNode<'parent, 'a, CompactConstructorDeclaration<'a>>),
    ReceiverParameter(&'parent AstNode<'parent, 'a, ReceiverParameter<'a>>),
    FormalParameters(&'parent AstNode<'parent, 'a, FormalParameters<'a>>),
    FormalParameter(&'parent AstNode<'parent, 'a, FormalParameter<'a>>),
    Block(&'parent AstNode<'parent, 'a, Block<'a>>),
    EmptyStatement(&'parent AstNode<'parent, 'a, EmptyStatement>),
    IfStatement(&'parent AstNode<'parent, 'a, IfStatement<'a>>),
    SwitchStatement(&'parent AstNode<'parent, 'a, SwitchStatement<'a>>),
    SwitchRule(&'parent AstNode<'parent, 'a, SwitchRule<'a>>),
    SwitchBlockStatementGroup(&'parent AstNode<'parent, 'a, SwitchBlockStatementGroup<'a>>),
    CaseLabel(&'parent AstNode<'parent, 'a, CaseLabel<'a>>),
    CasePatternLabel(&'parent AstNode<'parent, 'a, CasePatternLabel<'a>>),
    DefaultLabel(&'parent AstNode<'parent, 'a, DefaultLabel>),
    WhileStatement(&'parent AstNode<'parent, 'a, WhileStatement<'a>>),
    DoStatement(&'parent AstNode<'parent, 'a, DoStatement<'a>>),
    ForStatement(&'parent AstNode<'parent, 'a, ForStatement<'a>>),
    StatementExpressionList(&'parent AstNode<'parent, 'a, StatementExpressionList<'a>>),
    EnhancedForStatement(&'parent AstNode<'parent, 'a, EnhancedForStatement<'a>>),
    BreakStatement(&'parent AstNode<'parent, 'a, BreakStatement<'a>>),
    ContinueStatement(&'parent AstNode<'parent, 'a, ContinueStatement<'a>>),
    ThrowStatement(&'parent AstNode<'parent, 'a, ThrowStatement<'a>>),
    AssertStatement(&'parent AstNode<'parent, 'a, AssertStatement<'a>>),
    TryStatement(&'parent AstNode<'parent, 'a, TryStatement<'a>>),
    ResourceDeclaration(&'parent AstNode<'parent, 'a, ResourceDeclaration<'a>>),
    CatchClause(&'parent AstNode<'parent, 'a, CatchClause<'a>>),
    SynchronizedStatement(&'parent AstNode<'parent, 'a, SynchronizedStatement<'a>>),
    LabeledStatement(&'parent AstNode<'parent, 'a, LabeledStatement<'a>>),
    ConstructorInvocation(&'parent AstNode<'parent, 'a, ConstructorInvocation<'a>>),
    ExpressionStatement(&'parent AstNode<'parent, 'a, ExpressionStatement<'a>>),
    LocalVariableDeclaration(&'parent AstNode<'parent, 'a, LocalVariableDeclaration<'a>>),
    ReturnStatement(&'parent AstNode<'parent, 'a, ReturnStatement<'a>>),
    YieldStatement(&'parent AstNode<'parent, 'a, YieldStatement<'a>>),
    VariableDeclarator(&'parent AstNode<'parent, 'a, VariableDeclarator<'a>>),
    ArrayInitializer(&'parent AstNode<'parent, 'a, ArrayInitializer<'a>>),
    FieldAccess(&'parent AstNode<'parent, 'a, FieldAccess<'a>>),
    MethodCall(&'parent AstNode<'parent, 'a, MethodCall<'a>>),
    MethodMember(&'parent AstNode<'parent, 'a, MethodMember<'a>>),
    ClassLiteral(&'parent AstNode<'parent, 'a, ClassLiteral<'a>>),
    ThisExpression(&'parent AstNode<'parent, 'a, ThisExpression>),
    Super(&'parent AstNode<'parent, 'a, Super>),
    QualifiedThis(&'parent AstNode<'parent, 'a, QualifiedThis<'a>>),
    QualifiedSuper(&'parent AstNode<'parent, 'a, QualifiedSuper<'a>>),
    ClassInstanceCreation(&'parent AstNode<'parent, 'a, ClassInstanceCreation<'a>>),
    QualifiedClassInstanceCreation(
        &'parent AstNode<'parent, 'a, QualifiedClassInstanceCreation<'a>>,
    ),
    ArrayCreation(&'parent AstNode<'parent, 'a, ArrayCreation<'a>>),
    ArrayDimension(&'parent AstNode<'parent, 'a, ArrayDimension<'a>>),
    ArrayAccess(&'parent AstNode<'parent, 'a, ArrayAccess<'a>>),
    UpdateExpression(&'parent AstNode<'parent, 'a, UpdateExpression<'a>>),
    UnaryExpression(&'parent AstNode<'parent, 'a, UnaryExpression<'a>>),
    CastExpression(&'parent AstNode<'parent, 'a, CastExpression<'a>>),
    InstanceofExpression(&'parent AstNode<'parent, 'a, InstanceofExpression<'a>>),
    TypePattern(&'parent AstNode<'parent, 'a, TypePattern<'a>>),
    RecordPattern(&'parent AstNode<'parent, 'a, RecordPattern<'a>>),
    MatchAllPattern(&'parent AstNode<'parent, 'a, MatchAllPattern>),
    BinaryExpression(&'parent AstNode<'parent, 'a, BinaryExpression<'a>>),
    AssignmentExpression(&'parent AstNode<'parent, 'a, AssignmentExpression<'a>>),
    ConditionalExpression(&'parent AstNode<'parent, 'a, ConditionalExpression<'a>>),
    SwitchExpression(&'parent AstNode<'parent, 'a, SwitchExpression<'a>>),
    ParenthesizedExpression(&'parent AstNode<'parent, 'a, ParenthesizedExpression<'a>>),
    LambdaExpression(&'parent AstNode<'parent, 'a, LambdaExpression<'a>>),
    MethodReference(&'parent AstNode<'parent, 'a, MethodReference<'a>>),
    TypeParameters(&'parent AstNode<'parent, 'a, TypeParameters<'a>>),
    TypeParameter(&'parent AstNode<'parent, 'a, TypeParameter<'a>>),
    VarType(&'parent AstNode<'parent, 'a, VarType>),
    VoidType(&'parent AstNode<'parent, 'a, VoidType>),
    PrimitiveType(&'parent AstNode<'parent, 'a, PrimitiveType<'a>>),
    SimpleClassType(&'parent AstNode<'parent, 'a, SimpleClassType<'a>>),
    QualifiedClassType(&'parent AstNode<'parent, 'a, QualifiedClassType<'a>>),
    ArrayType(&'parent AstNode<'parent, 'a, ArrayType<'a>>),
    ArrayTypeDimension(&'parent AstNode<'parent, 'a, ArrayTypeDimension<'a>>),
    TypeArguments(&'parent AstNode<'parent, 'a, TypeArguments<'a>>),
    WildcardTypeArgument(&'parent AstNode<'parent, 'a, WildcardTypeArgument<'a>>),
    WildcardBound(&'parent AstNode<'parent, 'a, WildcardBound<'a>>),
    IntLiteral(&'parent AstNode<'parent, 'a, IntLiteral<'a>>),
    FloatLiteral(&'parent AstNode<'parent, 'a, FloatLiteral<'a>>),
    CharLiteral(&'parent AstNode<'parent, 'a, CharLiteral<'a>>),
    StringLiteral(&'parent AstNode<'parent, 'a, StringLiteral<'a>>),
    BooleanLiteral(&'parent AstNode<'parent, 'a, BooleanLiteral>),
    NullLiteral(&'parent AstNode<'parent, 'a, NullLiteral>),
    IdentifierName(&'parent AstNode<'parent, 'a, IdentifierName<'a>>),
    IdentifierReference(&'parent AstNode<'parent, 'a, IdentifierReference<'a>>),
    BindingIdentifier(&'parent AstNode<'parent, 'a, BindingIdentifier<'a>>),
    LabelIdentifier(&'parent AstNode<'parent, 'a, LabelIdentifier<'a>>),
    QualifiedName(&'parent AstNode<'parent, 'a, QualifiedName<'a>>),
    QualifiedTypeName(&'parent AstNode<'parent, 'a, QualifiedTypeName<'a>>),
    Annotation(&'parent AstNode<'parent, 'a, Annotation<'a>>),
    ElementValuePair(&'parent AstNode<'parent, 'a, ElementValuePair<'a>>),
    ElementValueArrayInitializer(&'parent AstNode<'parent, 'a, ElementValueArrayInitializer<'a>>),
}
impl<'parent, 'a> AstNodes<'parent, 'a> {
    pub fn span(&self) -> Span {
        match self {
            Self::Dummy => panic!("root has no parent"),
            Self::CompilationUnit(node) => node.span(),
            Self::OrdinaryCompilationUnit(node) => node.span(),
            Self::ModularCompilationUnit(node) => node.span(),
            Self::PackageDeclaration(node) => node.span(),
            Self::ImportDeclaration(node) => node.span(),
            Self::CompactCompilationUnit(node) => node.span(),
            Self::ModuleDeclaration(node) => node.span(),
            Self::RequiresDirective(node) => node.span(),
            Self::PackageDirective(node) => node.span(),
            Self::UsesDirective(node) => node.span(),
            Self::ProvidesDirective(node) => node.span(),
            Self::EmptyDeclaration(node) => node.span(),
            Self::ClassDeclaration(node) => node.span(),
            Self::ClassBody(node) => node.span(),
            Self::InterfaceDeclaration(node) => node.span(),
            Self::InterfaceBody(node) => node.span(),
            Self::EnumDeclaration(node) => node.span(),
            Self::EnumBody(node) => node.span(),
            Self::EnumConstant(node) => node.span(),
            Self::RecordDeclaration(node) => node.span(),
            Self::RecordHeader(node) => node.span(),
            Self::RecordComponent(node) => node.span(),
            Self::AnnotationInterfaceDeclaration(node) => node.span(),
            Self::AnnotationInterfaceBody(node) => node.span(),
            Self::AnnotationInterfaceElementDeclaration(node) => node.span(),
            Self::InitializerBlock(node) => node.span(),
            Self::FieldDeclaration(node) => node.span(),
            Self::MethodDeclaration(node) => node.span(),
            Self::ConstructorDeclaration(node) => node.span(),
            Self::CompactConstructorDeclaration(node) => node.span(),
            Self::ReceiverParameter(node) => node.span(),
            Self::FormalParameters(node) => node.span(),
            Self::FormalParameter(node) => node.span(),
            Self::Block(node) => node.span(),
            Self::EmptyStatement(node) => node.span(),
            Self::IfStatement(node) => node.span(),
            Self::SwitchStatement(node) => node.span(),
            Self::SwitchRule(node) => node.span(),
            Self::SwitchBlockStatementGroup(node) => node.span(),
            Self::CaseLabel(node) => node.span(),
            Self::CasePatternLabel(node) => node.span(),
            Self::DefaultLabel(node) => node.span(),
            Self::WhileStatement(node) => node.span(),
            Self::DoStatement(node) => node.span(),
            Self::ForStatement(node) => node.span(),
            Self::StatementExpressionList(node) => node.span(),
            Self::EnhancedForStatement(node) => node.span(),
            Self::BreakStatement(node) => node.span(),
            Self::ContinueStatement(node) => node.span(),
            Self::ThrowStatement(node) => node.span(),
            Self::AssertStatement(node) => node.span(),
            Self::TryStatement(node) => node.span(),
            Self::ResourceDeclaration(node) => node.span(),
            Self::CatchClause(node) => node.span(),
            Self::SynchronizedStatement(node) => node.span(),
            Self::LabeledStatement(node) => node.span(),
            Self::ConstructorInvocation(node) => node.span(),
            Self::ExpressionStatement(node) => node.span(),
            Self::LocalVariableDeclaration(node) => node.span(),
            Self::ReturnStatement(node) => node.span(),
            Self::YieldStatement(node) => node.span(),
            Self::VariableDeclarator(node) => node.span(),
            Self::ArrayInitializer(node) => node.span(),
            Self::FieldAccess(node) => node.span(),
            Self::MethodCall(node) => node.span(),
            Self::MethodMember(node) => node.span(),
            Self::ClassLiteral(node) => node.span(),
            Self::ThisExpression(node) => node.span(),
            Self::Super(node) => node.span(),
            Self::QualifiedThis(node) => node.span(),
            Self::QualifiedSuper(node) => node.span(),
            Self::ClassInstanceCreation(node) => node.span(),
            Self::QualifiedClassInstanceCreation(node) => node.span(),
            Self::ArrayCreation(node) => node.span(),
            Self::ArrayDimension(node) => node.span(),
            Self::ArrayAccess(node) => node.span(),
            Self::UpdateExpression(node) => node.span(),
            Self::UnaryExpression(node) => node.span(),
            Self::CastExpression(node) => node.span(),
            Self::InstanceofExpression(node) => node.span(),
            Self::TypePattern(node) => node.span(),
            Self::RecordPattern(node) => node.span(),
            Self::MatchAllPattern(node) => node.span(),
            Self::BinaryExpression(node) => node.span(),
            Self::AssignmentExpression(node) => node.span(),
            Self::ConditionalExpression(node) => node.span(),
            Self::SwitchExpression(node) => node.span(),
            Self::ParenthesizedExpression(node) => node.span(),
            Self::LambdaExpression(node) => node.span(),
            Self::MethodReference(node) => node.span(),
            Self::TypeParameters(node) => node.span(),
            Self::TypeParameter(node) => node.span(),
            Self::VarType(node) => node.span(),
            Self::VoidType(node) => node.span(),
            Self::PrimitiveType(node) => node.span(),
            Self::SimpleClassType(node) => node.span(),
            Self::QualifiedClassType(node) => node.span(),
            Self::ArrayType(node) => node.span(),
            Self::ArrayTypeDimension(node) => node.span(),
            Self::TypeArguments(node) => node.span(),
            Self::WildcardTypeArgument(node) => node.span(),
            Self::WildcardBound(node) => node.span(),
            Self::IntLiteral(node) => node.span(),
            Self::FloatLiteral(node) => node.span(),
            Self::CharLiteral(node) => node.span(),
            Self::StringLiteral(node) => node.span(),
            Self::BooleanLiteral(node) => node.span(),
            Self::NullLiteral(node) => node.span(),
            Self::IdentifierName(node) => node.span(),
            Self::IdentifierReference(node) => node.span(),
            Self::BindingIdentifier(node) => node.span(),
            Self::LabelIdentifier(node) => node.span(),
            Self::QualifiedName(node) => node.span(),
            Self::QualifiedTypeName(node) => node.span(),
            Self::Annotation(node) => node.span(),
            Self::ElementValuePair(node) => node.span(),
            Self::ElementValueArrayInitializer(node) => node.span(),
        }
    }
    pub fn parent(&self) -> &AstNodes<'parent, 'a> {
        match self {
            Self::Dummy => panic!("root has no parent"),
            Self::CompilationUnit(node) => node.parent(),
            Self::OrdinaryCompilationUnit(node) => node.parent(),
            Self::ModularCompilationUnit(node) => node.parent(),
            Self::PackageDeclaration(node) => node.parent(),
            Self::ImportDeclaration(node) => node.parent(),
            Self::CompactCompilationUnit(node) => node.parent(),
            Self::ModuleDeclaration(node) => node.parent(),
            Self::RequiresDirective(node) => node.parent(),
            Self::PackageDirective(node) => node.parent(),
            Self::UsesDirective(node) => node.parent(),
            Self::ProvidesDirective(node) => node.parent(),
            Self::EmptyDeclaration(node) => node.parent(),
            Self::ClassDeclaration(node) => node.parent(),
            Self::ClassBody(node) => node.parent(),
            Self::InterfaceDeclaration(node) => node.parent(),
            Self::InterfaceBody(node) => node.parent(),
            Self::EnumDeclaration(node) => node.parent(),
            Self::EnumBody(node) => node.parent(),
            Self::EnumConstant(node) => node.parent(),
            Self::RecordDeclaration(node) => node.parent(),
            Self::RecordHeader(node) => node.parent(),
            Self::RecordComponent(node) => node.parent(),
            Self::AnnotationInterfaceDeclaration(node) => node.parent(),
            Self::AnnotationInterfaceBody(node) => node.parent(),
            Self::AnnotationInterfaceElementDeclaration(node) => node.parent(),
            Self::InitializerBlock(node) => node.parent(),
            Self::FieldDeclaration(node) => node.parent(),
            Self::MethodDeclaration(node) => node.parent(),
            Self::ConstructorDeclaration(node) => node.parent(),
            Self::CompactConstructorDeclaration(node) => node.parent(),
            Self::ReceiverParameter(node) => node.parent(),
            Self::FormalParameters(node) => node.parent(),
            Self::FormalParameter(node) => node.parent(),
            Self::Block(node) => node.parent(),
            Self::EmptyStatement(node) => node.parent(),
            Self::IfStatement(node) => node.parent(),
            Self::SwitchStatement(node) => node.parent(),
            Self::SwitchRule(node) => node.parent(),
            Self::SwitchBlockStatementGroup(node) => node.parent(),
            Self::CaseLabel(node) => node.parent(),
            Self::CasePatternLabel(node) => node.parent(),
            Self::DefaultLabel(node) => node.parent(),
            Self::WhileStatement(node) => node.parent(),
            Self::DoStatement(node) => node.parent(),
            Self::ForStatement(node) => node.parent(),
            Self::StatementExpressionList(node) => node.parent(),
            Self::EnhancedForStatement(node) => node.parent(),
            Self::BreakStatement(node) => node.parent(),
            Self::ContinueStatement(node) => node.parent(),
            Self::ThrowStatement(node) => node.parent(),
            Self::AssertStatement(node) => node.parent(),
            Self::TryStatement(node) => node.parent(),
            Self::ResourceDeclaration(node) => node.parent(),
            Self::CatchClause(node) => node.parent(),
            Self::SynchronizedStatement(node) => node.parent(),
            Self::LabeledStatement(node) => node.parent(),
            Self::ConstructorInvocation(node) => node.parent(),
            Self::ExpressionStatement(node) => node.parent(),
            Self::LocalVariableDeclaration(node) => node.parent(),
            Self::ReturnStatement(node) => node.parent(),
            Self::YieldStatement(node) => node.parent(),
            Self::VariableDeclarator(node) => node.parent(),
            Self::ArrayInitializer(node) => node.parent(),
            Self::FieldAccess(node) => node.parent(),
            Self::MethodCall(node) => node.parent(),
            Self::MethodMember(node) => node.parent(),
            Self::ClassLiteral(node) => node.parent(),
            Self::ThisExpression(node) => node.parent(),
            Self::Super(node) => node.parent(),
            Self::QualifiedThis(node) => node.parent(),
            Self::QualifiedSuper(node) => node.parent(),
            Self::ClassInstanceCreation(node) => node.parent(),
            Self::QualifiedClassInstanceCreation(node) => node.parent(),
            Self::ArrayCreation(node) => node.parent(),
            Self::ArrayDimension(node) => node.parent(),
            Self::ArrayAccess(node) => node.parent(),
            Self::UpdateExpression(node) => node.parent(),
            Self::UnaryExpression(node) => node.parent(),
            Self::CastExpression(node) => node.parent(),
            Self::InstanceofExpression(node) => node.parent(),
            Self::TypePattern(node) => node.parent(),
            Self::RecordPattern(node) => node.parent(),
            Self::MatchAllPattern(node) => node.parent(),
            Self::BinaryExpression(node) => node.parent(),
            Self::AssignmentExpression(node) => node.parent(),
            Self::ConditionalExpression(node) => node.parent(),
            Self::SwitchExpression(node) => node.parent(),
            Self::ParenthesizedExpression(node) => node.parent(),
            Self::LambdaExpression(node) => node.parent(),
            Self::MethodReference(node) => node.parent(),
            Self::TypeParameters(node) => node.parent(),
            Self::TypeParameter(node) => node.parent(),
            Self::VarType(node) => node.parent(),
            Self::VoidType(node) => node.parent(),
            Self::PrimitiveType(node) => node.parent(),
            Self::SimpleClassType(node) => node.parent(),
            Self::QualifiedClassType(node) => node.parent(),
            Self::ArrayType(node) => node.parent(),
            Self::ArrayTypeDimension(node) => node.parent(),
            Self::TypeArguments(node) => node.parent(),
            Self::WildcardTypeArgument(node) => node.parent(),
            Self::WildcardBound(node) => node.parent(),
            Self::IntLiteral(node) => node.parent(),
            Self::FloatLiteral(node) => node.parent(),
            Self::CharLiteral(node) => node.parent(),
            Self::StringLiteral(node) => node.parent(),
            Self::BooleanLiteral(node) => node.parent(),
            Self::NullLiteral(node) => node.parent(),
            Self::IdentifierName(node) => node.parent(),
            Self::IdentifierReference(node) => node.parent(),
            Self::BindingIdentifier(node) => node.parent(),
            Self::LabelIdentifier(node) => node.parent(),
            Self::QualifiedName(node) => node.parent(),
            Self::QualifiedTypeName(node) => node.parent(),
            Self::Annotation(node) => node.parent(),
            Self::ElementValuePair(node) => node.parent(),
            Self::ElementValueArrayInitializer(node) => node.parent(),
        }
    }
    pub fn debug_name(&self) -> &'static str {
        match self {
            Self::Dummy => "Dummy",
            Self::CompilationUnit(_) => "CompilationUnit",
            Self::OrdinaryCompilationUnit(_) => "OrdinaryCompilationUnit",
            Self::ModularCompilationUnit(_) => "ModularCompilationUnit",
            Self::PackageDeclaration(_) => "PackageDeclaration",
            Self::ImportDeclaration(_) => "ImportDeclaration",
            Self::CompactCompilationUnit(_) => "CompactCompilationUnit",
            Self::ModuleDeclaration(_) => "ModuleDeclaration",
            Self::RequiresDirective(_) => "RequiresDirective",
            Self::PackageDirective(_) => "PackageDirective",
            Self::UsesDirective(_) => "UsesDirective",
            Self::ProvidesDirective(_) => "ProvidesDirective",
            Self::EmptyDeclaration(_) => "EmptyDeclaration",
            Self::ClassDeclaration(_) => "ClassDeclaration",
            Self::ClassBody(_) => "ClassBody",
            Self::InterfaceDeclaration(_) => "InterfaceDeclaration",
            Self::InterfaceBody(_) => "InterfaceBody",
            Self::EnumDeclaration(_) => "EnumDeclaration",
            Self::EnumBody(_) => "EnumBody",
            Self::EnumConstant(_) => "EnumConstant",
            Self::RecordDeclaration(_) => "RecordDeclaration",
            Self::RecordHeader(_) => "RecordHeader",
            Self::RecordComponent(_) => "RecordComponent",
            Self::AnnotationInterfaceDeclaration(_) => "AnnotationInterfaceDeclaration",
            Self::AnnotationInterfaceBody(_) => "AnnotationInterfaceBody",
            Self::AnnotationInterfaceElementDeclaration(_) => {
                "AnnotationInterfaceElementDeclaration"
            }
            Self::InitializerBlock(_) => "InitializerBlock",
            Self::FieldDeclaration(_) => "FieldDeclaration",
            Self::MethodDeclaration(_) => "MethodDeclaration",
            Self::ConstructorDeclaration(_) => "ConstructorDeclaration",
            Self::CompactConstructorDeclaration(_) => "CompactConstructorDeclaration",
            Self::ReceiverParameter(_) => "ReceiverParameter",
            Self::FormalParameters(_) => "FormalParameters",
            Self::FormalParameter(_) => "FormalParameter",
            Self::Block(_) => "Block",
            Self::EmptyStatement(_) => "EmptyStatement",
            Self::IfStatement(_) => "IfStatement",
            Self::SwitchStatement(_) => "SwitchStatement",
            Self::SwitchRule(_) => "SwitchRule",
            Self::SwitchBlockStatementGroup(_) => "SwitchBlockStatementGroup",
            Self::CaseLabel(_) => "CaseLabel",
            Self::CasePatternLabel(_) => "CasePatternLabel",
            Self::DefaultLabel(_) => "DefaultLabel",
            Self::WhileStatement(_) => "WhileStatement",
            Self::DoStatement(_) => "DoStatement",
            Self::ForStatement(_) => "ForStatement",
            Self::StatementExpressionList(_) => "StatementExpressionList",
            Self::EnhancedForStatement(_) => "EnhancedForStatement",
            Self::BreakStatement(_) => "BreakStatement",
            Self::ContinueStatement(_) => "ContinueStatement",
            Self::ThrowStatement(_) => "ThrowStatement",
            Self::AssertStatement(_) => "AssertStatement",
            Self::TryStatement(_) => "TryStatement",
            Self::ResourceDeclaration(_) => "ResourceDeclaration",
            Self::CatchClause(_) => "CatchClause",
            Self::SynchronizedStatement(_) => "SynchronizedStatement",
            Self::LabeledStatement(_) => "LabeledStatement",
            Self::ConstructorInvocation(_) => "ConstructorInvocation",
            Self::ExpressionStatement(_) => "ExpressionStatement",
            Self::LocalVariableDeclaration(_) => "LocalVariableDeclaration",
            Self::ReturnStatement(_) => "ReturnStatement",
            Self::YieldStatement(_) => "YieldStatement",
            Self::VariableDeclarator(_) => "VariableDeclarator",
            Self::ArrayInitializer(_) => "ArrayInitializer",
            Self::FieldAccess(_) => "FieldAccess",
            Self::MethodCall(_) => "MethodCall",
            Self::MethodMember(_) => "MethodMember",
            Self::ClassLiteral(_) => "ClassLiteral",
            Self::ThisExpression(_) => "ThisExpression",
            Self::Super(_) => "Super",
            Self::QualifiedThis(_) => "QualifiedThis",
            Self::QualifiedSuper(_) => "QualifiedSuper",
            Self::ClassInstanceCreation(_) => "ClassInstanceCreation",
            Self::QualifiedClassInstanceCreation(_) => "QualifiedClassInstanceCreation",
            Self::ArrayCreation(_) => "ArrayCreation",
            Self::ArrayDimension(_) => "ArrayDimension",
            Self::ArrayAccess(_) => "ArrayAccess",
            Self::UpdateExpression(_) => "UpdateExpression",
            Self::UnaryExpression(_) => "UnaryExpression",
            Self::CastExpression(_) => "CastExpression",
            Self::InstanceofExpression(_) => "InstanceofExpression",
            Self::TypePattern(_) => "TypePattern",
            Self::RecordPattern(_) => "RecordPattern",
            Self::MatchAllPattern(_) => "MatchAllPattern",
            Self::BinaryExpression(_) => "BinaryExpression",
            Self::AssignmentExpression(_) => "AssignmentExpression",
            Self::ConditionalExpression(_) => "ConditionalExpression",
            Self::SwitchExpression(_) => "SwitchExpression",
            Self::ParenthesizedExpression(_) => "ParenthesizedExpression",
            Self::LambdaExpression(_) => "LambdaExpression",
            Self::MethodReference(_) => "MethodReference",
            Self::TypeParameters(_) => "TypeParameters",
            Self::TypeParameter(_) => "TypeParameter",
            Self::VarType(_) => "VarType",
            Self::VoidType(_) => "VoidType",
            Self::PrimitiveType(_) => "PrimitiveType",
            Self::SimpleClassType(_) => "SimpleClassType",
            Self::QualifiedClassType(_) => "QualifiedClassType",
            Self::ArrayType(_) => "ArrayType",
            Self::ArrayTypeDimension(_) => "ArrayTypeDimension",
            Self::TypeArguments(_) => "TypeArguments",
            Self::WildcardTypeArgument(_) => "WildcardTypeArgument",
            Self::WildcardBound(_) => "WildcardBound",
            Self::IntLiteral(_) => "IntLiteral",
            Self::FloatLiteral(_) => "FloatLiteral",
            Self::CharLiteral(_) => "CharLiteral",
            Self::StringLiteral(_) => "StringLiteral",
            Self::BooleanLiteral(_) => "BooleanLiteral",
            Self::NullLiteral(_) => "NullLiteral",
            Self::IdentifierName(_) => "IdentifierName",
            Self::IdentifierReference(_) => "IdentifierReference",
            Self::BindingIdentifier(_) => "BindingIdentifier",
            Self::LabelIdentifier(_) => "LabelIdentifier",
            Self::QualifiedName(_) => "QualifiedName",
            Self::QualifiedTypeName(_) => "QualifiedTypeName",
            Self::Annotation(_) => "Annotation",
            Self::ElementValuePair(_) => "ElementValuePair",
            Self::ElementValueArrayInitializer(_) => "ElementValueArrayInitializer",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum AstNodeKind<'parent, 'a> {
    CompilationUnit(AstNode<'parent, 'a, CompilationUnit<'a>>),
    OrdinaryCompilationUnit(AstNode<'parent, 'a, OrdinaryCompilationUnit<'a>>),
    ModularCompilationUnit(AstNode<'parent, 'a, ModularCompilationUnit<'a>>),
    PackageDeclaration(AstNode<'parent, 'a, PackageDeclaration<'a>>),
    ImportDeclaration(AstNode<'parent, 'a, ImportDeclaration<'a>>),
    CompactCompilationUnit(AstNode<'parent, 'a, CompactCompilationUnit<'a>>),
    ModuleDeclaration(AstNode<'parent, 'a, ModuleDeclaration<'a>>),
    RequiresDirective(AstNode<'parent, 'a, RequiresDirective<'a>>),
    PackageDirective(AstNode<'parent, 'a, PackageDirective<'a>>),
    UsesDirective(AstNode<'parent, 'a, UsesDirective<'a>>),
    ProvidesDirective(AstNode<'parent, 'a, ProvidesDirective<'a>>),
    EmptyDeclaration(AstNode<'parent, 'a, EmptyDeclaration>),
    ClassDeclaration(AstNode<'parent, 'a, ClassDeclaration<'a>>),
    ClassBody(AstNode<'parent, 'a, ClassBody<'a>>),
    InterfaceDeclaration(AstNode<'parent, 'a, InterfaceDeclaration<'a>>),
    InterfaceBody(AstNode<'parent, 'a, InterfaceBody<'a>>),
    EnumDeclaration(AstNode<'parent, 'a, EnumDeclaration<'a>>),
    EnumBody(AstNode<'parent, 'a, EnumBody<'a>>),
    EnumConstant(AstNode<'parent, 'a, EnumConstant<'a>>),
    RecordDeclaration(AstNode<'parent, 'a, RecordDeclaration<'a>>),
    RecordHeader(AstNode<'parent, 'a, RecordHeader<'a>>),
    RecordComponent(AstNode<'parent, 'a, RecordComponent<'a>>),
    AnnotationInterfaceDeclaration(AstNode<'parent, 'a, AnnotationInterfaceDeclaration<'a>>),
    AnnotationInterfaceBody(AstNode<'parent, 'a, AnnotationInterfaceBody<'a>>),
    AnnotationInterfaceElementDeclaration(
        AstNode<'parent, 'a, AnnotationInterfaceElementDeclaration<'a>>,
    ),
    InitializerBlock(AstNode<'parent, 'a, InitializerBlock<'a>>),
    FieldDeclaration(AstNode<'parent, 'a, FieldDeclaration<'a>>),
    MethodDeclaration(AstNode<'parent, 'a, MethodDeclaration<'a>>),
    ConstructorDeclaration(AstNode<'parent, 'a, ConstructorDeclaration<'a>>),
    CompactConstructorDeclaration(AstNode<'parent, 'a, CompactConstructorDeclaration<'a>>),
    ReceiverParameter(AstNode<'parent, 'a, ReceiverParameter<'a>>),
    FormalParameters(AstNode<'parent, 'a, FormalParameters<'a>>),
    FormalParameter(AstNode<'parent, 'a, FormalParameter<'a>>),
    Block(AstNode<'parent, 'a, Block<'a>>),
    EmptyStatement(AstNode<'parent, 'a, EmptyStatement>),
    IfStatement(AstNode<'parent, 'a, IfStatement<'a>>),
    SwitchStatement(AstNode<'parent, 'a, SwitchStatement<'a>>),
    SwitchRule(AstNode<'parent, 'a, SwitchRule<'a>>),
    SwitchBlockStatementGroup(AstNode<'parent, 'a, SwitchBlockStatementGroup<'a>>),
    CaseLabel(AstNode<'parent, 'a, CaseLabel<'a>>),
    CasePatternLabel(AstNode<'parent, 'a, CasePatternLabel<'a>>),
    DefaultLabel(AstNode<'parent, 'a, DefaultLabel>),
    WhileStatement(AstNode<'parent, 'a, WhileStatement<'a>>),
    DoStatement(AstNode<'parent, 'a, DoStatement<'a>>),
    ForStatement(AstNode<'parent, 'a, ForStatement<'a>>),
    StatementExpressionList(AstNode<'parent, 'a, StatementExpressionList<'a>>),
    EnhancedForStatement(AstNode<'parent, 'a, EnhancedForStatement<'a>>),
    BreakStatement(AstNode<'parent, 'a, BreakStatement<'a>>),
    ContinueStatement(AstNode<'parent, 'a, ContinueStatement<'a>>),
    ThrowStatement(AstNode<'parent, 'a, ThrowStatement<'a>>),
    AssertStatement(AstNode<'parent, 'a, AssertStatement<'a>>),
    TryStatement(AstNode<'parent, 'a, TryStatement<'a>>),
    ResourceDeclaration(AstNode<'parent, 'a, ResourceDeclaration<'a>>),
    CatchClause(AstNode<'parent, 'a, CatchClause<'a>>),
    SynchronizedStatement(AstNode<'parent, 'a, SynchronizedStatement<'a>>),
    LabeledStatement(AstNode<'parent, 'a, LabeledStatement<'a>>),
    ConstructorInvocation(AstNode<'parent, 'a, ConstructorInvocation<'a>>),
    ExpressionStatement(AstNode<'parent, 'a, ExpressionStatement<'a>>),
    LocalVariableDeclaration(AstNode<'parent, 'a, LocalVariableDeclaration<'a>>),
    ReturnStatement(AstNode<'parent, 'a, ReturnStatement<'a>>),
    YieldStatement(AstNode<'parent, 'a, YieldStatement<'a>>),
    VariableDeclarator(AstNode<'parent, 'a, VariableDeclarator<'a>>),
    ArrayInitializer(AstNode<'parent, 'a, ArrayInitializer<'a>>),
    FieldAccess(AstNode<'parent, 'a, FieldAccess<'a>>),
    MethodCall(AstNode<'parent, 'a, MethodCall<'a>>),
    MethodMember(AstNode<'parent, 'a, MethodMember<'a>>),
    ClassLiteral(AstNode<'parent, 'a, ClassLiteral<'a>>),
    ThisExpression(AstNode<'parent, 'a, ThisExpression>),
    Super(AstNode<'parent, 'a, Super>),
    QualifiedThis(AstNode<'parent, 'a, QualifiedThis<'a>>),
    QualifiedSuper(AstNode<'parent, 'a, QualifiedSuper<'a>>),
    ClassInstanceCreation(AstNode<'parent, 'a, ClassInstanceCreation<'a>>),
    QualifiedClassInstanceCreation(AstNode<'parent, 'a, QualifiedClassInstanceCreation<'a>>),
    ArrayCreation(AstNode<'parent, 'a, ArrayCreation<'a>>),
    ArrayDimension(AstNode<'parent, 'a, ArrayDimension<'a>>),
    ArrayAccess(AstNode<'parent, 'a, ArrayAccess<'a>>),
    UpdateExpression(AstNode<'parent, 'a, UpdateExpression<'a>>),
    UnaryExpression(AstNode<'parent, 'a, UnaryExpression<'a>>),
    CastExpression(AstNode<'parent, 'a, CastExpression<'a>>),
    InstanceofExpression(AstNode<'parent, 'a, InstanceofExpression<'a>>),
    TypePattern(AstNode<'parent, 'a, TypePattern<'a>>),
    RecordPattern(AstNode<'parent, 'a, RecordPattern<'a>>),
    MatchAllPattern(AstNode<'parent, 'a, MatchAllPattern>),
    BinaryExpression(AstNode<'parent, 'a, BinaryExpression<'a>>),
    AssignmentExpression(AstNode<'parent, 'a, AssignmentExpression<'a>>),
    ConditionalExpression(AstNode<'parent, 'a, ConditionalExpression<'a>>),
    SwitchExpression(AstNode<'parent, 'a, SwitchExpression<'a>>),
    ParenthesizedExpression(AstNode<'parent, 'a, ParenthesizedExpression<'a>>),
    LambdaExpression(AstNode<'parent, 'a, LambdaExpression<'a>>),
    MethodReference(AstNode<'parent, 'a, MethodReference<'a>>),
    TypeParameters(AstNode<'parent, 'a, TypeParameters<'a>>),
    TypeParameter(AstNode<'parent, 'a, TypeParameter<'a>>),
    VarType(AstNode<'parent, 'a, VarType>),
    VoidType(AstNode<'parent, 'a, VoidType>),
    PrimitiveType(AstNode<'parent, 'a, PrimitiveType<'a>>),
    SimpleClassType(AstNode<'parent, 'a, SimpleClassType<'a>>),
    QualifiedClassType(AstNode<'parent, 'a, QualifiedClassType<'a>>),
    ArrayType(AstNode<'parent, 'a, ArrayType<'a>>),
    ArrayTypeDimension(AstNode<'parent, 'a, ArrayTypeDimension<'a>>),
    TypeArguments(AstNode<'parent, 'a, TypeArguments<'a>>),
    WildcardTypeArgument(AstNode<'parent, 'a, WildcardTypeArgument<'a>>),
    WildcardBound(AstNode<'parent, 'a, WildcardBound<'a>>),
    IntLiteral(AstNode<'parent, 'a, IntLiteral<'a>>),
    FloatLiteral(AstNode<'parent, 'a, FloatLiteral<'a>>),
    CharLiteral(AstNode<'parent, 'a, CharLiteral<'a>>),
    StringLiteral(AstNode<'parent, 'a, StringLiteral<'a>>),
    BooleanLiteral(AstNode<'parent, 'a, BooleanLiteral>),
    NullLiteral(AstNode<'parent, 'a, NullLiteral>),
    IdentifierName(AstNode<'parent, 'a, IdentifierName<'a>>),
    IdentifierReference(AstNode<'parent, 'a, IdentifierReference<'a>>),
    BindingIdentifier(AstNode<'parent, 'a, BindingIdentifier<'a>>),
    LabelIdentifier(AstNode<'parent, 'a, LabelIdentifier<'a>>),
    QualifiedName(AstNode<'parent, 'a, QualifiedName<'a>>),
    QualifiedTypeName(AstNode<'parent, 'a, QualifiedTypeName<'a>>),
    Annotation(AstNode<'parent, 'a, Annotation<'a>>),
    ElementValuePair(AstNode<'parent, 'a, ElementValuePair<'a>>),
    ElementValueArrayInitializer(AstNode<'parent, 'a, ElementValueArrayInitializer<'a>>),
}
impl<'a> AstNodeKind<'_, 'a> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        match self {
            Self::CompilationUnit(node) => AstNodes::CompilationUnit(node),
            Self::OrdinaryCompilationUnit(node) => AstNodes::OrdinaryCompilationUnit(node),
            Self::ModularCompilationUnit(node) => AstNodes::ModularCompilationUnit(node),
            Self::PackageDeclaration(node) => AstNodes::PackageDeclaration(node),
            Self::ImportDeclaration(node) => AstNodes::ImportDeclaration(node),
            Self::CompactCompilationUnit(node) => AstNodes::CompactCompilationUnit(node),
            Self::ModuleDeclaration(node) => AstNodes::ModuleDeclaration(node),
            Self::RequiresDirective(node) => AstNodes::RequiresDirective(node),
            Self::PackageDirective(node) => AstNodes::PackageDirective(node),
            Self::UsesDirective(node) => AstNodes::UsesDirective(node),
            Self::ProvidesDirective(node) => AstNodes::ProvidesDirective(node),
            Self::EmptyDeclaration(node) => AstNodes::EmptyDeclaration(node),
            Self::ClassDeclaration(node) => AstNodes::ClassDeclaration(node),
            Self::ClassBody(node) => AstNodes::ClassBody(node),
            Self::InterfaceDeclaration(node) => AstNodes::InterfaceDeclaration(node),
            Self::InterfaceBody(node) => AstNodes::InterfaceBody(node),
            Self::EnumDeclaration(node) => AstNodes::EnumDeclaration(node),
            Self::EnumBody(node) => AstNodes::EnumBody(node),
            Self::EnumConstant(node) => AstNodes::EnumConstant(node),
            Self::RecordDeclaration(node) => AstNodes::RecordDeclaration(node),
            Self::RecordHeader(node) => AstNodes::RecordHeader(node),
            Self::RecordComponent(node) => AstNodes::RecordComponent(node),
            Self::AnnotationInterfaceDeclaration(node) => {
                AstNodes::AnnotationInterfaceDeclaration(node)
            }
            Self::AnnotationInterfaceBody(node) => AstNodes::AnnotationInterfaceBody(node),
            Self::AnnotationInterfaceElementDeclaration(node) => {
                AstNodes::AnnotationInterfaceElementDeclaration(node)
            }
            Self::InitializerBlock(node) => AstNodes::InitializerBlock(node),
            Self::FieldDeclaration(node) => AstNodes::FieldDeclaration(node),
            Self::MethodDeclaration(node) => AstNodes::MethodDeclaration(node),
            Self::ConstructorDeclaration(node) => AstNodes::ConstructorDeclaration(node),
            Self::CompactConstructorDeclaration(node) => {
                AstNodes::CompactConstructorDeclaration(node)
            }
            Self::ReceiverParameter(node) => AstNodes::ReceiverParameter(node),
            Self::FormalParameters(node) => AstNodes::FormalParameters(node),
            Self::FormalParameter(node) => AstNodes::FormalParameter(node),
            Self::Block(node) => AstNodes::Block(node),
            Self::EmptyStatement(node) => AstNodes::EmptyStatement(node),
            Self::IfStatement(node) => AstNodes::IfStatement(node),
            Self::SwitchStatement(node) => AstNodes::SwitchStatement(node),
            Self::SwitchRule(node) => AstNodes::SwitchRule(node),
            Self::SwitchBlockStatementGroup(node) => AstNodes::SwitchBlockStatementGroup(node),
            Self::CaseLabel(node) => AstNodes::CaseLabel(node),
            Self::CasePatternLabel(node) => AstNodes::CasePatternLabel(node),
            Self::DefaultLabel(node) => AstNodes::DefaultLabel(node),
            Self::WhileStatement(node) => AstNodes::WhileStatement(node),
            Self::DoStatement(node) => AstNodes::DoStatement(node),
            Self::ForStatement(node) => AstNodes::ForStatement(node),
            Self::StatementExpressionList(node) => AstNodes::StatementExpressionList(node),
            Self::EnhancedForStatement(node) => AstNodes::EnhancedForStatement(node),
            Self::BreakStatement(node) => AstNodes::BreakStatement(node),
            Self::ContinueStatement(node) => AstNodes::ContinueStatement(node),
            Self::ThrowStatement(node) => AstNodes::ThrowStatement(node),
            Self::AssertStatement(node) => AstNodes::AssertStatement(node),
            Self::TryStatement(node) => AstNodes::TryStatement(node),
            Self::ResourceDeclaration(node) => AstNodes::ResourceDeclaration(node),
            Self::CatchClause(node) => AstNodes::CatchClause(node),
            Self::SynchronizedStatement(node) => AstNodes::SynchronizedStatement(node),
            Self::LabeledStatement(node) => AstNodes::LabeledStatement(node),
            Self::ConstructorInvocation(node) => AstNodes::ConstructorInvocation(node),
            Self::ExpressionStatement(node) => AstNodes::ExpressionStatement(node),
            Self::LocalVariableDeclaration(node) => AstNodes::LocalVariableDeclaration(node),
            Self::ReturnStatement(node) => AstNodes::ReturnStatement(node),
            Self::YieldStatement(node) => AstNodes::YieldStatement(node),
            Self::VariableDeclarator(node) => AstNodes::VariableDeclarator(node),
            Self::ArrayInitializer(node) => AstNodes::ArrayInitializer(node),
            Self::FieldAccess(node) => AstNodes::FieldAccess(node),
            Self::MethodCall(node) => AstNodes::MethodCall(node),
            Self::MethodMember(node) => AstNodes::MethodMember(node),
            Self::ClassLiteral(node) => AstNodes::ClassLiteral(node),
            Self::ThisExpression(node) => AstNodes::ThisExpression(node),
            Self::Super(node) => AstNodes::Super(node),
            Self::QualifiedThis(node) => AstNodes::QualifiedThis(node),
            Self::QualifiedSuper(node) => AstNodes::QualifiedSuper(node),
            Self::ClassInstanceCreation(node) => AstNodes::ClassInstanceCreation(node),
            Self::QualifiedClassInstanceCreation(node) => {
                AstNodes::QualifiedClassInstanceCreation(node)
            }
            Self::ArrayCreation(node) => AstNodes::ArrayCreation(node),
            Self::ArrayDimension(node) => AstNodes::ArrayDimension(node),
            Self::ArrayAccess(node) => AstNodes::ArrayAccess(node),
            Self::UpdateExpression(node) => AstNodes::UpdateExpression(node),
            Self::UnaryExpression(node) => AstNodes::UnaryExpression(node),
            Self::CastExpression(node) => AstNodes::CastExpression(node),
            Self::InstanceofExpression(node) => AstNodes::InstanceofExpression(node),
            Self::TypePattern(node) => AstNodes::TypePattern(node),
            Self::RecordPattern(node) => AstNodes::RecordPattern(node),
            Self::MatchAllPattern(node) => AstNodes::MatchAllPattern(node),
            Self::BinaryExpression(node) => AstNodes::BinaryExpression(node),
            Self::AssignmentExpression(node) => AstNodes::AssignmentExpression(node),
            Self::ConditionalExpression(node) => AstNodes::ConditionalExpression(node),
            Self::SwitchExpression(node) => AstNodes::SwitchExpression(node),
            Self::ParenthesizedExpression(node) => AstNodes::ParenthesizedExpression(node),
            Self::LambdaExpression(node) => AstNodes::LambdaExpression(node),
            Self::MethodReference(node) => AstNodes::MethodReference(node),
            Self::TypeParameters(node) => AstNodes::TypeParameters(node),
            Self::TypeParameter(node) => AstNodes::TypeParameter(node),
            Self::VarType(node) => AstNodes::VarType(node),
            Self::VoidType(node) => AstNodes::VoidType(node),
            Self::PrimitiveType(node) => AstNodes::PrimitiveType(node),
            Self::SimpleClassType(node) => AstNodes::SimpleClassType(node),
            Self::QualifiedClassType(node) => AstNodes::QualifiedClassType(node),
            Self::ArrayType(node) => AstNodes::ArrayType(node),
            Self::ArrayTypeDimension(node) => AstNodes::ArrayTypeDimension(node),
            Self::TypeArguments(node) => AstNodes::TypeArguments(node),
            Self::WildcardTypeArgument(node) => AstNodes::WildcardTypeArgument(node),
            Self::WildcardBound(node) => AstNodes::WildcardBound(node),
            Self::IntLiteral(node) => AstNodes::IntLiteral(node),
            Self::FloatLiteral(node) => AstNodes::FloatLiteral(node),
            Self::CharLiteral(node) => AstNodes::CharLiteral(node),
            Self::StringLiteral(node) => AstNodes::StringLiteral(node),
            Self::BooleanLiteral(node) => AstNodes::BooleanLiteral(node),
            Self::NullLiteral(node) => AstNodes::NullLiteral(node),
            Self::IdentifierName(node) => AstNodes::IdentifierName(node),
            Self::IdentifierReference(node) => AstNodes::IdentifierReference(node),
            Self::BindingIdentifier(node) => AstNodes::BindingIdentifier(node),
            Self::LabelIdentifier(node) => AstNodes::LabelIdentifier(node),
            Self::QualifiedName(node) => AstNodes::QualifiedName(node),
            Self::QualifiedTypeName(node) => AstNodes::QualifiedTypeName(node),
            Self::Annotation(node) => AstNodes::Annotation(node),
            Self::ElementValuePair(node) => AstNodes::ElementValuePair(node),
            Self::ElementValueArrayInitializer(node) => {
                AstNodes::ElementValueArrayInitializer(node)
            }
        }
    }
}
impl GetSpan for AstNodeKind<'_, '_> {
    fn span(&self) -> Span {
        match self {
            Self::CompilationUnit(node) => node.span(),
            Self::OrdinaryCompilationUnit(node) => node.span(),
            Self::ModularCompilationUnit(node) => node.span(),
            Self::PackageDeclaration(node) => node.span(),
            Self::ImportDeclaration(node) => node.span(),
            Self::CompactCompilationUnit(node) => node.span(),
            Self::ModuleDeclaration(node) => node.span(),
            Self::RequiresDirective(node) => node.span(),
            Self::PackageDirective(node) => node.span(),
            Self::UsesDirective(node) => node.span(),
            Self::ProvidesDirective(node) => node.span(),
            Self::EmptyDeclaration(node) => node.span(),
            Self::ClassDeclaration(node) => node.span(),
            Self::ClassBody(node) => node.span(),
            Self::InterfaceDeclaration(node) => node.span(),
            Self::InterfaceBody(node) => node.span(),
            Self::EnumDeclaration(node) => node.span(),
            Self::EnumBody(node) => node.span(),
            Self::EnumConstant(node) => node.span(),
            Self::RecordDeclaration(node) => node.span(),
            Self::RecordHeader(node) => node.span(),
            Self::RecordComponent(node) => node.span(),
            Self::AnnotationInterfaceDeclaration(node) => node.span(),
            Self::AnnotationInterfaceBody(node) => node.span(),
            Self::AnnotationInterfaceElementDeclaration(node) => node.span(),
            Self::InitializerBlock(node) => node.span(),
            Self::FieldDeclaration(node) => node.span(),
            Self::MethodDeclaration(node) => node.span(),
            Self::ConstructorDeclaration(node) => node.span(),
            Self::CompactConstructorDeclaration(node) => node.span(),
            Self::ReceiverParameter(node) => node.span(),
            Self::FormalParameters(node) => node.span(),
            Self::FormalParameter(node) => node.span(),
            Self::Block(node) => node.span(),
            Self::EmptyStatement(node) => node.span(),
            Self::IfStatement(node) => node.span(),
            Self::SwitchStatement(node) => node.span(),
            Self::SwitchRule(node) => node.span(),
            Self::SwitchBlockStatementGroup(node) => node.span(),
            Self::CaseLabel(node) => node.span(),
            Self::CasePatternLabel(node) => node.span(),
            Self::DefaultLabel(node) => node.span(),
            Self::WhileStatement(node) => node.span(),
            Self::DoStatement(node) => node.span(),
            Self::ForStatement(node) => node.span(),
            Self::StatementExpressionList(node) => node.span(),
            Self::EnhancedForStatement(node) => node.span(),
            Self::BreakStatement(node) => node.span(),
            Self::ContinueStatement(node) => node.span(),
            Self::ThrowStatement(node) => node.span(),
            Self::AssertStatement(node) => node.span(),
            Self::TryStatement(node) => node.span(),
            Self::ResourceDeclaration(node) => node.span(),
            Self::CatchClause(node) => node.span(),
            Self::SynchronizedStatement(node) => node.span(),
            Self::LabeledStatement(node) => node.span(),
            Self::ConstructorInvocation(node) => node.span(),
            Self::ExpressionStatement(node) => node.span(),
            Self::LocalVariableDeclaration(node) => node.span(),
            Self::ReturnStatement(node) => node.span(),
            Self::YieldStatement(node) => node.span(),
            Self::VariableDeclarator(node) => node.span(),
            Self::ArrayInitializer(node) => node.span(),
            Self::FieldAccess(node) => node.span(),
            Self::MethodCall(node) => node.span(),
            Self::MethodMember(node) => node.span(),
            Self::ClassLiteral(node) => node.span(),
            Self::ThisExpression(node) => node.span(),
            Self::Super(node) => node.span(),
            Self::QualifiedThis(node) => node.span(),
            Self::QualifiedSuper(node) => node.span(),
            Self::ClassInstanceCreation(node) => node.span(),
            Self::QualifiedClassInstanceCreation(node) => node.span(),
            Self::ArrayCreation(node) => node.span(),
            Self::ArrayDimension(node) => node.span(),
            Self::ArrayAccess(node) => node.span(),
            Self::UpdateExpression(node) => node.span(),
            Self::UnaryExpression(node) => node.span(),
            Self::CastExpression(node) => node.span(),
            Self::InstanceofExpression(node) => node.span(),
            Self::TypePattern(node) => node.span(),
            Self::RecordPattern(node) => node.span(),
            Self::MatchAllPattern(node) => node.span(),
            Self::BinaryExpression(node) => node.span(),
            Self::AssignmentExpression(node) => node.span(),
            Self::ConditionalExpression(node) => node.span(),
            Self::SwitchExpression(node) => node.span(),
            Self::ParenthesizedExpression(node) => node.span(),
            Self::LambdaExpression(node) => node.span(),
            Self::MethodReference(node) => node.span(),
            Self::TypeParameters(node) => node.span(),
            Self::TypeParameter(node) => node.span(),
            Self::VarType(node) => node.span(),
            Self::VoidType(node) => node.span(),
            Self::PrimitiveType(node) => node.span(),
            Self::SimpleClassType(node) => node.span(),
            Self::QualifiedClassType(node) => node.span(),
            Self::ArrayType(node) => node.span(),
            Self::ArrayTypeDimension(node) => node.span(),
            Self::TypeArguments(node) => node.span(),
            Self::WildcardTypeArgument(node) => node.span(),
            Self::WildcardBound(node) => node.span(),
            Self::IntLiteral(node) => node.span(),
            Self::FloatLiteral(node) => node.span(),
            Self::CharLiteral(node) => node.span(),
            Self::StringLiteral(node) => node.span(),
            Self::BooleanLiteral(node) => node.span(),
            Self::NullLiteral(node) => node.span(),
            Self::IdentifierName(node) => node.span(),
            Self::IdentifierReference(node) => node.span(),
            Self::BindingIdentifier(node) => node.span(),
            Self::LabelIdentifier(node) => node.span(),
            Self::QualifiedName(node) => node.span(),
            Self::QualifiedTypeName(node) => node.span(),
            Self::Annotation(node) => node.span(),
            Self::ElementValuePair(node) => node.span(),
            Self::ElementValueArrayInitializer(node) => node.span(),
        }
    }
}
impl<'a> AstNode<'_, 'a, CompilationUnit<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::CompilationUnit(self)
    }
    pub fn content(&self) -> AstNode<'_, 'a, CompilationUnitContent<'a>> {
        AstNode {
            inner: &self.inner.content,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, CompilationUnitContent<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            CompilationUnitContent::Ordinary(field_0) => {
                AstNodeKind::OrdinaryCompilationUnit(AstNode {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            CompilationUnitContent::Compact(field_0) => {
                AstNodeKind::CompactCompilationUnit(AstNode {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            CompilationUnitContent::Modular(field_0) => {
                AstNodeKind::ModularCompilationUnit(AstNode {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
        }
    }
    pub fn as_ordinary(&self) -> Option<AstNode<'parent, 'a, OrdinaryCompilationUnit<'a>>> {
        match self.inner {
            CompilationUnitContent::Ordinary(field_0) => Some(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_compact(&self) -> Option<AstNode<'parent, 'a, CompactCompilationUnit<'a>>> {
        match self.inner {
            CompilationUnitContent::Compact(field_0) => Some(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_modular(&self) -> Option<AstNode<'parent, 'a, ModularCompilationUnit<'a>>> {
        match self.inner {
            CompilationUnitContent::Modular(field_0) => Some(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, OrdinaryCompilationUnit<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::OrdinaryCompilationUnit(self)
    }
    pub fn package(&self) -> Option<AstNode<'_, 'a, PackageDeclaration<'a>>> {
        self.inner.package.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .imports
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.types.first().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        })
    }
    pub fn imports(&self) -> AstNode<'_, 'a, Vec<'a, ImportDeclaration<'a>>> {
        AstNode {
            inner: &self.inner.imports,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .types
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn types(&self) -> AstNode<'_, 'a, Vec<'a, TypeDeclaration<'a>>> {
        AstNode {
            inner: &self.inner.types,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ModularCompilationUnit<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ModularCompilationUnit(self)
    }
    pub fn imports(&self) -> AstNode<'_, 'a, Vec<'a, ImportDeclaration<'a>>> {
        AstNode {
            inner: &self.inner.imports,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.module.span().start,
        }
    }
    pub fn module(&self) -> AstNode<'_, 'a, ModuleDeclaration<'a>> {
        AstNode {
            inner: &self.inner.module,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, PackageDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::PackageDeclaration(self)
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, Name<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ImportDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ImportDeclaration(self)
    }
    pub fn name(&self) -> AstNode<'_, 'a, Name<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .local
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn local(&self) -> Option<AstNode<'_, 'a, BindingIdentifier<'a>>> {
        self.inner.local.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, CompactCompilationUnit<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::CompactCompilationUnit(self)
    }
    pub fn imports(&self) -> AstNode<'_, 'a, Vec<'a, ImportDeclaration<'a>>> {
        AstNode {
            inner: &self.inner.imports,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .members
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn members(&self) -> AstNode<'_, 'a, Vec<'a, ClassMember<'a>>> {
        AstNode {
            inner: &self.inner.members,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ModuleDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ModuleDeclaration(self)
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, Name<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .directives
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn directives(&self) -> AstNode<'_, 'a, Vec<'a, ModuleDirective<'a>>> {
        AstNode {
            inner: &self.inner.directives,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, ModuleDirective<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            ModuleDirective::Requires(field_0) => AstNodeKind::RequiresDirective(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ModuleDirective::Package(field_0) => AstNodeKind::PackageDirective(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ModuleDirective::Uses(field_0) => AstNodeKind::UsesDirective(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ModuleDirective::Provides(field_0) => AstNodeKind::ProvidesDirective(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_requires(&self) -> Option<AstNode<'parent, 'a, RequiresDirective<'a>>> {
        match self.inner {
            ModuleDirective::Requires(field_0) => Some(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_package(&self) -> Option<AstNode<'parent, 'a, PackageDirective<'a>>> {
        match self.inner {
            ModuleDirective::Package(field_0) => Some(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_uses(&self) -> Option<AstNode<'parent, 'a, UsesDirective<'a>>> {
        match self.inner {
            ModuleDirective::Uses(field_0) => Some(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_provides(&self) -> Option<AstNode<'parent, 'a, ProvidesDirective<'a>>> {
        match self.inner {
            ModuleDirective::Provides(field_0) => Some(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, RequiresDirective<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::RequiresDirective(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, RequiresModifier>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, Name<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl AstNode<'_, '_, RequiresModifier> {}
impl<'a> AstNode<'_, 'a, PackageDirective<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::PackageDirective(self)
    }
    pub fn name(&self) -> AstNode<'_, 'a, Name<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .targets
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn targets(&self) -> AstNode<'_, 'a, Vec<'a, Name<'a>>> {
        AstNode {
            inner: &self.inner.targets,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, UsesDirective<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::UsesDirective(self)
    }
    pub fn name(&self) -> AstNode<'_, 'a, TypeName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ProvidesDirective<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ProvidesDirective(self)
    }
    pub fn name(&self) -> AstNode<'_, 'a, TypeName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .with
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn with(&self) -> AstNode<'_, 'a, Vec<'a, TypeName<'a>>> {
        AstNode {
            inner: &self.inner.with,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, TypeDeclaration<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            TypeDeclaration::Empty(field_0) => AstNodeKind::EmptyDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeDeclaration::AnnotationInterface(field_0) => {
                AstNodeKind::AnnotationInterfaceDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            TypeDeclaration::Class(field_0) => AstNodeKind::ClassDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeDeclaration::Interface(field_0) => AstNodeKind::InterfaceDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeDeclaration::Enum(field_0) => AstNodeKind::EnumDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeDeclaration::Record(field_0) => AstNodeKind::RecordDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_empty(&self) -> Option<AstNode<'parent, 'a, EmptyDeclaration>> {
        match self.inner {
            TypeDeclaration::Empty(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_annotation_interface(
        &self,
    ) -> Option<AstNode<'parent, 'a, AnnotationInterfaceDeclaration<'a>>> {
        match self.inner {
            TypeDeclaration::AnnotationInterface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class(&self) -> Option<AstNode<'parent, 'a, ClassDeclaration<'a>>> {
        match self.inner {
            TypeDeclaration::Class(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_interface(&self) -> Option<AstNode<'parent, 'a, InterfaceDeclaration<'a>>> {
        match self.inner {
            TypeDeclaration::Interface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_enum(&self) -> Option<AstNode<'parent, 'a, EnumDeclaration<'a>>> {
        match self.inner {
            TypeDeclaration::Enum(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_record(&self) -> Option<AstNode<'parent, 'a, RecordDeclaration<'a>>> {
        match self.inner {
            TypeDeclaration::Record(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<AstNode<'parent, 'a, LocalClassOrInterfaceDeclaration<'a>>> {
        self.inner.as_local_class_or_interface_declaration().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, LocalClassOrInterfaceDeclaration<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            LocalClassOrInterfaceDeclaration::Class(field_0) => {
                AstNodeKind::ClassDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            LocalClassOrInterfaceDeclaration::Interface(field_0) => {
                AstNodeKind::InterfaceDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            LocalClassOrInterfaceDeclaration::Enum(field_0) => {
                AstNodeKind::EnumDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            LocalClassOrInterfaceDeclaration::Record(field_0) => {
                AstNodeKind::RecordDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
        }
    }
    pub fn as_class(&self) -> Option<AstNode<'parent, 'a, ClassDeclaration<'a>>> {
        match self.inner {
            LocalClassOrInterfaceDeclaration::Class(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_interface(&self) -> Option<AstNode<'parent, 'a, InterfaceDeclaration<'a>>> {
        match self.inner {
            LocalClassOrInterfaceDeclaration::Interface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_enum(&self) -> Option<AstNode<'parent, 'a, EnumDeclaration<'a>>> {
        match self.inner {
            LocalClassOrInterfaceDeclaration::Enum(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_record(&self) -> Option<AstNode<'parent, 'a, RecordDeclaration<'a>>> {
        match self.inner {
            LocalClassOrInterfaceDeclaration::Record(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, EmptyDeclaration> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::EmptyDeclaration(self)
    }
}
impl<'a> AstNode<'_, 'a, ClassDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ClassDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .type_params
                .as_ref()
                .map(|it| it.span().start)
                .or_else(|| self.inner.superclass.as_ref().map(|it| it.span().start))
                .or_else(|| self.inner.interfaces.first().map(|it| it.span().start))
                .or_else(|| self.inner.permits.first().map(|it| it.span().start))
                .unwrap_or(self.inner.body.span().start),
        }
    }
    pub fn type_params(&self) -> Option<AstNode<'_, 'a, TypeParameters<'a>>> {
        self.inner.type_params.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .superclass
                .as_ref()
                .map(|it| it.span().start)
                .or_else(|| self.inner.interfaces.first().map(|it| it.span().start))
                .or_else(|| self.inner.permits.first().map(|it| it.span().start))
                .unwrap_or(self.inner.body.span().start),
        })
    }
    pub fn superclass(&self) -> Option<AstNode<'_, 'a, Type<'a>>> {
        self.inner.superclass.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .interfaces
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.permits.first().map(|it| it.span().start))
                .unwrap_or(self.inner.body.span().start),
        })
    }
    pub fn interfaces(&self) -> AstNode<'_, 'a, Vec<'a, Type<'a>>> {
        AstNode {
            inner: &self.inner.interfaces,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .permits
                .first()
                .map_or(self.inner.body.span().start, |it| it.span().start),
        }
    }
    pub fn permits(&self) -> AstNode<'_, 'a, Vec<'a, TypeName<'a>>> {
        AstNode {
            inner: &self.inner.permits,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, ClassBody<'a>> {
        AstNode {
            inner: &self.inner.body,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ClassBody<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ClassBody(self)
    }
    pub fn members(&self) -> AstNode<'_, 'a, Vec<'a, ClassMember<'a>>> {
        AstNode {
            inner: &self.inner.members,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, InterfaceDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::InterfaceDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .type_params
                .as_ref()
                .map(|it| it.span().start)
                .or_else(|| self.inner.extends.first().map(|it| it.span().start))
                .or_else(|| self.inner.permits.first().map(|it| it.span().start))
                .unwrap_or(self.inner.body.span().start),
        }
    }
    pub fn type_params(&self) -> Option<AstNode<'_, 'a, TypeParameters<'a>>> {
        self.inner.type_params.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .extends
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.permits.first().map(|it| it.span().start))
                .unwrap_or(self.inner.body.span().start),
        })
    }
    pub fn extends(&self) -> AstNode<'_, 'a, Vec<'a, Type<'a>>> {
        AstNode {
            inner: &self.inner.extends,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .permits
                .first()
                .map_or(self.inner.body.span().start, |it| it.span().start),
        }
    }
    pub fn permits(&self) -> AstNode<'_, 'a, Vec<'a, TypeName<'a>>> {
        AstNode {
            inner: &self.inner.permits,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, InterfaceBody<'a>> {
        AstNode {
            inner: &self.inner.body,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, InterfaceBody<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::InterfaceBody(self)
    }
    pub fn members(&self) -> AstNode<'_, 'a, Vec<'a, InterfaceMember<'a>>> {
        AstNode {
            inner: &self.inner.members,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, EnumDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::EnumDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .interfaces
                .first()
                .map_or(self.inner.body.span().start, |it| it.span().start),
        }
    }
    pub fn interfaces(&self) -> AstNode<'_, 'a, Vec<'a, Type<'a>>> {
        AstNode {
            inner: &self.inner.interfaces,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, EnumBody<'a>> {
        AstNode {
            inner: &self.inner.body,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, EnumBody<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::EnumBody(self)
    }
    pub fn constants(&self) -> AstNode<'_, 'a, Vec<'a, EnumConstant<'a>>> {
        AstNode {
            inner: &self.inner.constants,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .members
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn members(&self) -> AstNode<'_, 'a, Vec<'a, ClassMember<'a>>> {
        AstNode {
            inner: &self.inner.members,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, EnumConstant<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::EnumConstant(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .args
                .as_ref()
                .and_then(|it| it.first().map(|it| it.span().start))
                .or_else(|| self.inner.body.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn args(&self) -> Option<AstNode<'_, 'a, Vec<'a, Expression<'a>>>> {
        self.inner.args.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .body
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        })
    }
    pub fn body(&self) -> Option<AstNode<'_, 'a, ClassBody<'a>>> {
        self.inner.body.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, RecordDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::RecordDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .type_params
                .as_ref()
                .map_or(self.inner.header.span().start, |it| it.span().start),
        }
    }
    pub fn type_params(&self) -> Option<AstNode<'_, 'a, TypeParameters<'a>>> {
        self.inner.type_params.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.header.span().start,
        })
    }
    pub fn header(&self) -> AstNode<'_, 'a, RecordHeader<'a>> {
        AstNode {
            inner: &self.inner.header,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .interfaces
                .first()
                .map_or(self.inner.body.span().start, |it| it.span().start),
        }
    }
    pub fn interfaces(&self) -> AstNode<'_, 'a, Vec<'a, Type<'a>>> {
        AstNode {
            inner: &self.inner.interfaces,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, ClassBody<'a>> {
        AstNode {
            inner: &self.inner.body,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, RecordHeader<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::RecordHeader(self)
    }
    pub fn components(&self) -> AstNode<'_, 'a, Vec<'a, RecordComponent<'a>>> {
        AstNode {
            inner: &self.inner.components,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        }
    }
}
impl<'a> AstNode<'_, 'a, RecordComponent<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::RecordComponent(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.ty.span().start,
        }
    }
    pub fn ty(&self) -> AstNode<'_, 'a, Type<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .varargs_annotations
                .first()
                .map_or(self.inner.name.span().start, |it| it.span().start),
        }
    }
    pub fn varargs_annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.varargs_annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, AnnotationInterfaceDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::AnnotationInterfaceDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, AnnotationInterfaceBody<'a>> {
        AstNode {
            inner: &self.inner.body,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, AnnotationInterfaceBody<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::AnnotationInterfaceBody(self)
    }
    pub fn members(&self) -> AstNode<'_, 'a, Vec<'a, AnnotationInterfaceMemberDeclaration<'a>>> {
        AstNode {
            inner: &self.inner.members,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, AnnotationInterfaceMemberDeclaration<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            AnnotationInterfaceMemberDeclaration::Element(field_0) => {
                AstNodeKind::AnnotationInterfaceElementDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationInterfaceMemberDeclaration::Field(field_0) => {
                AstNodeKind::FieldDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationInterfaceMemberDeclaration::Empty(field_0) => {
                AstNodeKind::EmptyDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationInterfaceMemberDeclaration::AnnotationInterface(field_0) => {
                AstNodeKind::AnnotationInterfaceDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationInterfaceMemberDeclaration::Class(field_0) => {
                AstNodeKind::ClassDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationInterfaceMemberDeclaration::Interface(field_0) => {
                AstNodeKind::InterfaceDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationInterfaceMemberDeclaration::Enum(field_0) => {
                AstNodeKind::EnumDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationInterfaceMemberDeclaration::Record(field_0) => {
                AstNodeKind::RecordDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
        }
    }
    pub fn as_element(
        &self,
    ) -> Option<AstNode<'parent, 'a, AnnotationInterfaceElementDeclaration<'a>>> {
        match self.inner {
            AnnotationInterfaceMemberDeclaration::Element(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field(&self) -> Option<AstNode<'parent, 'a, FieldDeclaration<'a>>> {
        match self.inner {
            AnnotationInterfaceMemberDeclaration::Field(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<AstNode<'parent, 'a, EmptyDeclaration>> {
        match self.inner {
            AnnotationInterfaceMemberDeclaration::Empty(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_annotation_interface(
        &self,
    ) -> Option<AstNode<'parent, 'a, AnnotationInterfaceDeclaration<'a>>> {
        match self.inner {
            AnnotationInterfaceMemberDeclaration::AnnotationInterface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class(&self) -> Option<AstNode<'parent, 'a, ClassDeclaration<'a>>> {
        match self.inner {
            AnnotationInterfaceMemberDeclaration::Class(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_interface(&self) -> Option<AstNode<'parent, 'a, InterfaceDeclaration<'a>>> {
        match self.inner {
            AnnotationInterfaceMemberDeclaration::Interface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_enum(&self) -> Option<AstNode<'parent, 'a, EnumDeclaration<'a>>> {
        match self.inner {
            AnnotationInterfaceMemberDeclaration::Enum(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_record(&self) -> Option<AstNode<'parent, 'a, RecordDeclaration<'a>>> {
        match self.inner {
            AnnotationInterfaceMemberDeclaration::Record(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_type_declaration(&self) -> Option<AstNode<'parent, 'a, TypeDeclaration<'a>>> {
        self.inner.as_type_declaration().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<AstNode<'parent, 'a, LocalClassOrInterfaceDeclaration<'a>>> {
        self.inner.as_local_class_or_interface_declaration().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, AnnotationInterfaceElementDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::AnnotationInterfaceElementDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.ty.span().start,
        }
    }
    pub fn ty(&self) -> AstNode<'_, 'a, Type<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .post_paren_dimensions
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.default_value.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn post_paren_dimensions(&self) -> AstNode<'_, 'a, Vec<'a, ArrayTypeDimension<'a>>> {
        AstNode {
            inner: &self.inner.post_paren_dimensions,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .default_value
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn default_value(&self) -> Option<AstNode<'_, 'a, ElementValue<'a>>> {
        self.inner.default_value.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, ClassMember<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            ClassMember::Field(field_0) => AstNodeKind::FieldDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassMember::Method(field_0) => AstNodeKind::MethodDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassMember::Constructor(field_0) => AstNodeKind::ConstructorDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassMember::CompactConstructor(field_0) => {
                AstNodeKind::CompactConstructorDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            ClassMember::Initializer(field_0) => AstNodeKind::InitializerBlock(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassMember::Empty(field_0) => AstNodeKind::EmptyDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassMember::AnnotationInterface(field_0) => {
                AstNodeKind::AnnotationInterfaceDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            ClassMember::Class(field_0) => AstNodeKind::ClassDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassMember::Interface(field_0) => AstNodeKind::InterfaceDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassMember::Enum(field_0) => AstNodeKind::EnumDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassMember::Record(field_0) => AstNodeKind::RecordDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_field(&self) -> Option<AstNode<'parent, 'a, FieldDeclaration<'a>>> {
        match self.inner {
            ClassMember::Field(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method(&self) -> Option<AstNode<'parent, 'a, MethodDeclaration<'a>>> {
        match self.inner {
            ClassMember::Method(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_constructor(&self) -> Option<AstNode<'parent, 'a, ConstructorDeclaration<'a>>> {
        match self.inner {
            ClassMember::Constructor(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_compact_constructor(
        &self,
    ) -> Option<AstNode<'parent, 'a, CompactConstructorDeclaration<'a>>> {
        match self.inner {
            ClassMember::CompactConstructor(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_initializer(&self) -> Option<AstNode<'parent, 'a, InitializerBlock<'a>>> {
        match self.inner {
            ClassMember::Initializer(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<AstNode<'parent, 'a, EmptyDeclaration>> {
        match self.inner {
            ClassMember::Empty(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_annotation_interface(
        &self,
    ) -> Option<AstNode<'parent, 'a, AnnotationInterfaceDeclaration<'a>>> {
        match self.inner {
            ClassMember::AnnotationInterface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class(&self) -> Option<AstNode<'parent, 'a, ClassDeclaration<'a>>> {
        match self.inner {
            ClassMember::Class(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_interface(&self) -> Option<AstNode<'parent, 'a, InterfaceDeclaration<'a>>> {
        match self.inner {
            ClassMember::Interface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_enum(&self) -> Option<AstNode<'parent, 'a, EnumDeclaration<'a>>> {
        match self.inner {
            ClassMember::Enum(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_record(&self) -> Option<AstNode<'parent, 'a, RecordDeclaration<'a>>> {
        match self.inner {
            ClassMember::Record(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_type_declaration(&self) -> Option<AstNode<'parent, 'a, TypeDeclaration<'a>>> {
        self.inner.as_type_declaration().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<AstNode<'parent, 'a, LocalClassOrInterfaceDeclaration<'a>>> {
        self.inner.as_local_class_or_interface_declaration().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, InterfaceMember<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            InterfaceMember::Constant(field_0) => AstNodeKind::FieldDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InterfaceMember::Method(field_0) => AstNodeKind::MethodDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InterfaceMember::Empty(field_0) => AstNodeKind::EmptyDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InterfaceMember::AnnotationInterface(field_0) => {
                AstNodeKind::AnnotationInterfaceDeclaration(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            InterfaceMember::Class(field_0) => AstNodeKind::ClassDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InterfaceMember::Interface(field_0) => AstNodeKind::InterfaceDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InterfaceMember::Enum(field_0) => AstNodeKind::EnumDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InterfaceMember::Record(field_0) => AstNodeKind::RecordDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_constant(&self) -> Option<AstNode<'parent, 'a, FieldDeclaration<'a>>> {
        match self.inner {
            InterfaceMember::Constant(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method(&self) -> Option<AstNode<'parent, 'a, MethodDeclaration<'a>>> {
        match self.inner {
            InterfaceMember::Method(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<AstNode<'parent, 'a, EmptyDeclaration>> {
        match self.inner {
            InterfaceMember::Empty(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_annotation_interface(
        &self,
    ) -> Option<AstNode<'parent, 'a, AnnotationInterfaceDeclaration<'a>>> {
        match self.inner {
            InterfaceMember::AnnotationInterface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class(&self) -> Option<AstNode<'parent, 'a, ClassDeclaration<'a>>> {
        match self.inner {
            InterfaceMember::Class(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_interface(&self) -> Option<AstNode<'parent, 'a, InterfaceDeclaration<'a>>> {
        match self.inner {
            InterfaceMember::Interface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_enum(&self) -> Option<AstNode<'parent, 'a, EnumDeclaration<'a>>> {
        match self.inner {
            InterfaceMember::Enum(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_record(&self) -> Option<AstNode<'parent, 'a, RecordDeclaration<'a>>> {
        match self.inner {
            InterfaceMember::Record(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_type_declaration(&self) -> Option<AstNode<'parent, 'a, TypeDeclaration<'a>>> {
        self.inner.as_type_declaration().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<AstNode<'parent, 'a, LocalClassOrInterfaceDeclaration<'a>>> {
        self.inner.as_local_class_or_interface_declaration().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, InitializerBlock<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::InitializerBlock(self)
    }
    pub fn block(&self) -> AstNode<'_, 'a, Block<'a>> {
        AstNode {
            inner: &self.inner.block,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, FieldDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::FieldDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.ty.span().start,
        }
    }
    pub fn ty(&self) -> AstNode<'_, 'a, Type<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .declarators
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn declarators(&self) -> AstNode<'_, 'a, Vec<'a, VariableDeclarator<'a>>> {
        AstNode {
            inner: &self.inner.declarators,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, MethodDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::MethodDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .type_params
                .as_ref()
                .map_or(self.inner.return_type.span().start, |it| it.span().start),
        }
    }
    pub fn type_params(&self) -> Option<AstNode<'_, 'a, TypeParameters<'a>>> {
        self.inner.type_params.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.return_type.span().start,
        })
    }
    pub fn return_type(&self) -> AstNode<'_, 'a, MethodResult<'a>> {
        AstNode {
            inner: &self.inner.return_type,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.params.span().start,
        }
    }
    pub fn params(&self) -> AstNode<'_, 'a, FormalParameters<'a>> {
        AstNode {
            inner: &self.inner.params,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .post_paren_dimensions
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.throws.first().map(|it| it.span().start))
                .or_else(|| self.inner.body.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn post_paren_dimensions(&self) -> AstNode<'_, 'a, Vec<'a, ArrayTypeDimension<'a>>> {
        AstNode {
            inner: &self.inner.post_paren_dimensions,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .throws
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.body.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn throws(&self) -> AstNode<'_, 'a, Vec<'a, Type<'a>>> {
        AstNode {
            inner: &self.inner.throws,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .body
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn body(&self) -> Option<AstNode<'_, 'a, Block<'a>>> {
        self.inner.body.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, ConstructorDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ConstructorDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .type_params
                .as_ref()
                .map_or(self.inner.name.span().start, |it| it.span().start),
        }
    }
    pub fn type_params(&self) -> Option<AstNode<'_, 'a, TypeParameters<'a>>> {
        self.inner.type_params.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        })
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.params.span().start,
        }
    }
    pub fn params(&self) -> AstNode<'_, 'a, FormalParameters<'a>> {
        AstNode {
            inner: &self.inner.params,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .throws
                .first()
                .map_or(self.inner.body.span().start, |it| it.span().start),
        }
    }
    pub fn throws(&self) -> AstNode<'_, 'a, Vec<'a, Type<'a>>> {
        AstNode {
            inner: &self.inner.throws,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, Block<'a>> {
        AstNode {
            inner: &self.inner.body,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, CompactConstructorDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::CompactConstructorDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, Block<'a>> {
        AstNode {
            inner: &self.inner.body,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ReceiverParameter<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ReceiverParameter(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.ty.span().start,
        }
    }
    pub fn ty(&self) -> AstNode<'_, 'a, Type<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .qualifier
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn qualifier(&self) -> Option<AstNode<'_, 'a, IdentifierName<'a>>> {
        self.inner.qualifier.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, FormalParameters<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::FormalParameters(self)
    }
    pub fn receiver(&self) -> Option<AstNode<'_, 'a, ReceiverParameter<'a>>> {
        self.inner.receiver.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.items.first().map_or(0, |it| it.span().start),
        })
    }
    pub fn items(&self) -> AstNode<'_, 'a, Vec<'a, FormalParameter<'a>>> {
        AstNode { inner: &self.inner.items, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'a> AstNode<'_, 'a, FormalParameter<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::FormalParameter(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .ty
                .as_ref()
                .map(|it| it.span().start)
                .or_else(|| self.inner.varargs_annotations.first().map(|it| it.span().start))
                .unwrap_or(self.inner.name.span().start),
        }
    }
    pub fn ty(&self) -> Option<AstNode<'_, 'a, VariableType<'a>>> {
        self.inner.ty.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .varargs_annotations
                .first()
                .map_or(self.inner.name.span().start, |it| it.span().start),
        })
    }
    pub fn varargs_annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.varargs_annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .dims
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn dims(&self) -> AstNode<'_, 'a, Vec<'a, ArrayTypeDimension<'a>>> {
        AstNode {
            inner: &self.inner.dims,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, Block<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::Block(self)
    }
    pub fn statements(&self) -> AstNode<'_, 'a, Vec<'a, Statement<'a>>> {
        AstNode {
            inner: &self.inner.statements,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, Statement<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            Statement::Block(field_0) => AstNodeKind::Block(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Empty(field_0) => AstNodeKind::EmptyStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::ConstructorInvocation(field_0) => {
                AstNodeKind::ConstructorInvocation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            Statement::LocalVariable(field_0) => AstNodeKind::LocalVariableDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Expression(field_0) => AstNodeKind::ExpressionStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::If(field_0) => AstNodeKind::IfStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Switch(field_0) => AstNodeKind::SwitchStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::While(field_0) => AstNodeKind::WhileStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Do(field_0) => AstNodeKind::DoStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::For(field_0) => AstNodeKind::ForStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::EnhancedFor(field_0) => AstNodeKind::EnhancedForStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Break(field_0) => AstNodeKind::BreakStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Continue(field_0) => AstNodeKind::ContinueStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Throw(field_0) => AstNodeKind::ThrowStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Assert(field_0) => AstNodeKind::AssertStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Try(field_0) => AstNodeKind::TryStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Synchronized(field_0) => AstNodeKind::SynchronizedStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Return(field_0) => AstNodeKind::ReturnStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Yield(field_0) => AstNodeKind::YieldStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Labeled(field_0) => AstNodeKind::LabeledStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Class(field_0) => AstNodeKind::ClassDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Interface(field_0) => AstNodeKind::InterfaceDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Enum(field_0) => AstNodeKind::EnumDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Statement::Record(field_0) => AstNodeKind::RecordDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_block(&self) -> Option<AstNode<'parent, 'a, Block<'a>>> {
        match self.inner {
            Statement::Block(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<AstNode<'parent, 'a, EmptyStatement>> {
        match self.inner {
            Statement::Empty(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_constructor_invocation(
        &self,
    ) -> Option<AstNode<'parent, 'a, ConstructorInvocation<'a>>> {
        match self.inner {
            Statement::ConstructorInvocation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_local_variable(&self) -> Option<AstNode<'parent, 'a, LocalVariableDeclaration<'a>>> {
        match self.inner {
            Statement::LocalVariable(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_expression(&self) -> Option<AstNode<'parent, 'a, ExpressionStatement<'a>>> {
        match self.inner {
            Statement::Expression(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_if(&self) -> Option<AstNode<'parent, 'a, IfStatement<'a>>> {
        match self.inner {
            Statement::If(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_switch(&self) -> Option<AstNode<'parent, 'a, SwitchStatement<'a>>> {
        match self.inner {
            Statement::Switch(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_while(&self) -> Option<AstNode<'parent, 'a, WhileStatement<'a>>> {
        match self.inner {
            Statement::While(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_do(&self) -> Option<AstNode<'parent, 'a, DoStatement<'a>>> {
        match self.inner {
            Statement::Do(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_for(&self) -> Option<AstNode<'parent, 'a, ForStatement<'a>>> {
        match self.inner {
            Statement::For(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_enhanced_for(&self) -> Option<AstNode<'parent, 'a, EnhancedForStatement<'a>>> {
        match self.inner {
            Statement::EnhancedFor(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_break(&self) -> Option<AstNode<'parent, 'a, BreakStatement<'a>>> {
        match self.inner {
            Statement::Break(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_continue(&self) -> Option<AstNode<'parent, 'a, ContinueStatement<'a>>> {
        match self.inner {
            Statement::Continue(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_throw(&self) -> Option<AstNode<'parent, 'a, ThrowStatement<'a>>> {
        match self.inner {
            Statement::Throw(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_assert(&self) -> Option<AstNode<'parent, 'a, AssertStatement<'a>>> {
        match self.inner {
            Statement::Assert(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_try(&self) -> Option<AstNode<'parent, 'a, TryStatement<'a>>> {
        match self.inner {
            Statement::Try(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_synchronized(&self) -> Option<AstNode<'parent, 'a, SynchronizedStatement<'a>>> {
        match self.inner {
            Statement::Synchronized(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_return(&self) -> Option<AstNode<'parent, 'a, ReturnStatement<'a>>> {
        match self.inner {
            Statement::Return(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_yield(&self) -> Option<AstNode<'parent, 'a, YieldStatement<'a>>> {
        match self.inner {
            Statement::Yield(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_labeled(&self) -> Option<AstNode<'parent, 'a, LabeledStatement<'a>>> {
        match self.inner {
            Statement::Labeled(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class(&self) -> Option<AstNode<'parent, 'a, ClassDeclaration<'a>>> {
        match self.inner {
            Statement::Class(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_interface(&self) -> Option<AstNode<'parent, 'a, InterfaceDeclaration<'a>>> {
        match self.inner {
            Statement::Interface(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_enum(&self) -> Option<AstNode<'parent, 'a, EnumDeclaration<'a>>> {
        match self.inner {
            Statement::Enum(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_record(&self) -> Option<AstNode<'parent, 'a, RecordDeclaration<'a>>> {
        match self.inner {
            Statement::Record(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_local_class_or_interface_declaration(
        &self,
    ) -> Option<AstNode<'parent, 'a, LocalClassOrInterfaceDeclaration<'a>>> {
        self.inner.as_local_class_or_interface_declaration().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, EmptyStatement> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::EmptyStatement(self)
    }
}
impl<'a> AstNode<'_, 'a, IfStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::IfStatement(self)
    }
    pub fn condition(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.condition,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.then_branch.span().start,
        }
    }
    pub fn then_branch(&self) -> AstNode<'_, 'a, Statement<'a>> {
        AstNode {
            inner: &self.inner.then_branch,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.else_branch.as_ref().map_or(0, |it| it.span().start),
        }
    }
    pub fn else_branch(&self) -> Option<AstNode<'_, 'a, Statement<'a>>> {
        self.inner.else_branch.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        })
    }
}
impl<'a> AstNode<'_, 'a, SwitchStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::SwitchStatement(self)
    }
    pub fn selector(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.selector,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.cases.first().map_or(0, |it| it.span().start),
        }
    }
    pub fn cases(&self) -> AstNode<'_, 'a, Vec<'a, SwitchCase<'a>>> {
        AstNode { inner: &self.inner.cases, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, SwitchCase<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            SwitchCase::Rule(field_0) => AstNodeKind::SwitchRule(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchCase::Group(field_0) => AstNodeKind::SwitchBlockStatementGroup(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_rule(&self) -> Option<AstNode<'parent, 'a, SwitchRule<'a>>> {
        match self.inner {
            SwitchCase::Rule(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchCase::Group(..) => None,
        }
    }
    pub fn as_group(&self) -> Option<AstNode<'parent, 'a, SwitchBlockStatementGroup<'a>>> {
        match self.inner {
            SwitchCase::Group(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchCase::Rule(..) => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, SwitchRule<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::SwitchRule(self)
    }
    pub fn label(&self) -> AstNode<'_, 'a, SwitchLabel<'a>> {
        AstNode {
            inner: &self.inner.label,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, SwitchRuleBody<'a>> {
        AstNode { inner: &self.inner.body, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, SwitchRuleBody<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            SwitchRuleBody::Block(field_0) => AstNodeKind::Block(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Throw(field_0) => AstNodeKind::ThrowStatement(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::IntLiteral(field_0) => AstNodeKind::IntLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::FloatLiteral(field_0) => AstNodeKind::FloatLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::CharLiteral(field_0) => AstNodeKind::CharLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::StringLiteral(field_0) => AstNodeKind::StringLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::BooleanLiteral(field_0) => AstNodeKind::BooleanLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::NullLiteral(field_0) => AstNodeKind::NullLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::This(field_0) => AstNodeKind::ThisExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Super(field_0) => AstNodeKind::Super(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::QualifiedThis(field_0) => AstNodeKind::QualifiedThis(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::QualifiedSuper(field_0) => AstNodeKind::QualifiedSuper(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::FieldAccess(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::MethodCall(field_0) => AstNodeKind::MethodCall(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::ClassLiteral(field_0) => AstNodeKind::ClassLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::ClassInstanceCreation(field_0) => {
                AstNodeKind::ClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            SwitchRuleBody::QualifiedClassInstanceCreation(field_0) => {
                AstNodeKind::QualifiedClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            SwitchRuleBody::ArrayCreation(field_0) => AstNodeKind::ArrayCreation(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::ArrayAccess(field_0) => AstNodeKind::ArrayAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Update(field_0) => AstNodeKind::UpdateExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Unary(field_0) => AstNodeKind::UnaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Cast(field_0) => AstNodeKind::CastExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Instanceof(field_0) => AstNodeKind::InstanceofExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Binary(field_0) => AstNodeKind::BinaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Assignment(field_0) => AstNodeKind::AssignmentExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Conditional(field_0) => AstNodeKind::ConditionalExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Switch(field_0) => AstNodeKind::SwitchExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::Parenthesized(field_0) => {
                AstNodeKind::ParenthesizedExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            SwitchRuleBody::Lambda(field_0) => AstNodeKind::LambdaExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchRuleBody::MethodReference(field_0) => AstNodeKind::MethodReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_block(&self) -> Option<AstNode<'parent, 'a, Block<'a>>> {
        match self.inner {
            SwitchRuleBody::Block(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_throw(&self) -> Option<AstNode<'parent, 'a, ThrowStatement<'a>>> {
        match self.inner {
            SwitchRuleBody::Throw(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_int_literal(&self) -> Option<AstNode<'parent, 'a, IntLiteral<'a>>> {
        match self.inner {
            SwitchRuleBody::IntLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_float_literal(&self) -> Option<AstNode<'parent, 'a, FloatLiteral<'a>>> {
        match self.inner {
            SwitchRuleBody::FloatLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_char_literal(&self) -> Option<AstNode<'parent, 'a, CharLiteral<'a>>> {
        match self.inner {
            SwitchRuleBody::CharLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_string_literal(&self) -> Option<AstNode<'parent, 'a, StringLiteral<'a>>> {
        match self.inner {
            SwitchRuleBody::StringLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_boolean_literal(&self) -> Option<AstNode<'parent, 'a, BooleanLiteral>> {
        match self.inner {
            SwitchRuleBody::BooleanLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_null_literal(&self) -> Option<AstNode<'parent, 'a, NullLiteral>> {
        match self.inner {
            SwitchRuleBody::NullLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_this(&self) -> Option<AstNode<'parent, 'a, ThisExpression>> {
        match self.inner {
            SwitchRuleBody::This(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_super(&self) -> Option<AstNode<'parent, 'a, Super>> {
        match self.inner {
            SwitchRuleBody::Super(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_this(&self) -> Option<AstNode<'parent, 'a, QualifiedThis<'a>>> {
        match self.inner {
            SwitchRuleBody::QualifiedThis(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_super(&self) -> Option<AstNode<'parent, 'a, QualifiedSuper<'a>>> {
        match self.inner {
            SwitchRuleBody::QualifiedSuper(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            SwitchRuleBody::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            SwitchRuleBody::FieldAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_call(&self) -> Option<AstNode<'parent, 'a, MethodCall<'a>>> {
        match self.inner {
            SwitchRuleBody::MethodCall(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_literal(&self) -> Option<AstNode<'parent, 'a, ClassLiteral<'a>>> {
        match self.inner {
            SwitchRuleBody::ClassLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, ClassInstanceCreation<'a>>> {
        match self.inner {
            SwitchRuleBody::ClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, QualifiedClassInstanceCreation<'a>>> {
        match self.inner {
            SwitchRuleBody::QualifiedClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_creation(&self) -> Option<AstNode<'parent, 'a, ArrayCreation<'a>>> {
        match self.inner {
            SwitchRuleBody::ArrayCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_access(&self) -> Option<AstNode<'parent, 'a, ArrayAccess<'a>>> {
        match self.inner {
            SwitchRuleBody::ArrayAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_update(&self) -> Option<AstNode<'parent, 'a, UpdateExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Update(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_unary(&self) -> Option<AstNode<'parent, 'a, UnaryExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Unary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_cast(&self) -> Option<AstNode<'parent, 'a, CastExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Cast(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_instanceof(&self) -> Option<AstNode<'parent, 'a, InstanceofExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Instanceof(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_binary(&self) -> Option<AstNode<'parent, 'a, BinaryExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Binary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_assignment(&self) -> Option<AstNode<'parent, 'a, AssignmentExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Assignment(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_conditional(&self) -> Option<AstNode<'parent, 'a, ConditionalExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Conditional(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_switch(&self) -> Option<AstNode<'parent, 'a, SwitchExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Switch(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_parenthesized(&self) -> Option<AstNode<'parent, 'a, ParenthesizedExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Parenthesized(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_lambda(&self) -> Option<AstNode<'parent, 'a, LambdaExpression<'a>>> {
        match self.inner {
            SwitchRuleBody::Lambda(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_reference(&self) -> Option<AstNode<'parent, 'a, MethodReference<'a>>> {
        match self.inner {
            SwitchRuleBody::MethodReference(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_expression(&self) -> Option<AstNode<'parent, 'a, Expression<'a>>> {
        self.inner.as_expression().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, SwitchBlockStatementGroup<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::SwitchBlockStatementGroup(self)
    }
    pub fn labels(&self) -> AstNode<'_, 'a, Vec<'a, SwitchLabel<'a>>> {
        AstNode {
            inner: &self.inner.labels,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .statements
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn statements(&self) -> AstNode<'_, 'a, Vec<'a, Statement<'a>>> {
        AstNode {
            inner: &self.inner.statements,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, SwitchLabel<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            SwitchLabel::Case(field_0) => AstNodeKind::CaseLabel(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchLabel::Pattern(field_0) => AstNodeKind::CasePatternLabel(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            SwitchLabel::Default(field_0) => AstNodeKind::DefaultLabel(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_case(&self) -> Option<AstNode<'parent, 'a, CaseLabel<'a>>> {
        match self.inner {
            SwitchLabel::Case(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_pattern(&self) -> Option<AstNode<'parent, 'a, CasePatternLabel<'a>>> {
        match self.inner {
            SwitchLabel::Pattern(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_default(&self) -> Option<AstNode<'parent, 'a, DefaultLabel>> {
        match self.inner {
            SwitchLabel::Default(field_0) => Some(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, CaseLabel<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::CaseLabel(self)
    }
    pub fn constants(&self) -> AstNode<'_, 'a, Vec<'a, Expression<'a>>> {
        AstNode {
            inner: &self.inner.constants,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, CasePatternLabel<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::CasePatternLabel(self)
    }
    pub fn patterns(&self) -> AstNode<'_, 'a, Vec<'a, Pattern<'a>>> {
        AstNode {
            inner: &self.inner.patterns,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .guard
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn guard(&self) -> Option<AstNode<'_, 'a, Expression<'a>>> {
        self.inner.guard.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, DefaultLabel> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::DefaultLabel(self)
    }
}
impl<'a> AstNode<'_, 'a, WhileStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::WhileStatement(self)
    }
    pub fn condition(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.condition,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, Statement<'a>> {
        AstNode { inner: &self.inner.body, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'a> AstNode<'_, 'a, DoStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::DoStatement(self)
    }
    pub fn body(&self) -> AstNode<'_, 'a, Statement<'a>> {
        AstNode {
            inner: &self.inner.body,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.condition.span().start,
        }
    }
    pub fn condition(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.condition,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        }
    }
}
impl<'a> AstNode<'_, 'a, ForStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ForStatement(self)
    }
    pub fn init(&self) -> Option<AstNode<'_, 'a, ForInit<'a>>> {
        self.inner.init.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .condition
                .as_ref()
                .map(|it| it.span().start)
                .or_else(|| self.inner.update.as_ref().map(|it| it.span().start))
                .unwrap_or(self.inner.body.span().start),
        })
    }
    pub fn condition(&self) -> Option<AstNode<'_, 'a, Expression<'a>>> {
        self.inner.condition.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .update
                .as_ref()
                .map_or(self.inner.body.span().start, |it| it.span().start),
        })
    }
    pub fn update(&self) -> Option<AstNode<'_, 'a, StatementExpressionList<'a>>> {
        self.inner.update.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        })
    }
    pub fn body(&self) -> AstNode<'_, 'a, Statement<'a>> {
        AstNode { inner: &self.inner.body, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, ForInit<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            ForInit::Declaration(field_0) => AstNodeKind::LocalVariableDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ForInit::Expressions(field_0) => AstNodeKind::StatementExpressionList(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_declaration(&self) -> Option<AstNode<'parent, 'a, LocalVariableDeclaration<'a>>> {
        match self.inner {
            ForInit::Declaration(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ForInit::Expressions(..) => None,
        }
    }
    pub fn as_expressions(&self) -> Option<AstNode<'parent, 'a, StatementExpressionList<'a>>> {
        match self.inner {
            ForInit::Expressions(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ForInit::Declaration(..) => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, StatementExpressionList<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::StatementExpressionList(self)
    }
    pub fn expressions(&self) -> AstNode<'_, 'a, Vec<'a, Expression<'a>>> {
        AstNode {
            inner: &self.inner.expressions,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, EnhancedForStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::EnhancedForStatement(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.ty.span().start,
        }
    }
    pub fn ty(&self) -> AstNode<'_, 'a, VariableType<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .dims
                .first()
                .map_or(self.inner.iterable.span().start, |it| it.span().start),
        }
    }
    pub fn dims(&self) -> AstNode<'_, 'a, Vec<'a, ArrayTypeDimension<'a>>> {
        AstNode {
            inner: &self.inner.dims,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.iterable.span().start,
        }
    }
    pub fn iterable(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.iterable,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, Statement<'a>> {
        AstNode { inner: &self.inner.body, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'a> AstNode<'_, 'a, BreakStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::BreakStatement(self)
    }
    pub fn label(&self) -> Option<AstNode<'_, 'a, LabelIdentifier<'a>>> {
        self.inner.label.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        })
    }
}
impl<'a> AstNode<'_, 'a, ContinueStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ContinueStatement(self)
    }
    pub fn label(&self) -> Option<AstNode<'_, 'a, LabelIdentifier<'a>>> {
        self.inner.label.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        })
    }
}
impl<'a> AstNode<'_, 'a, ThrowStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ThrowStatement(self)
    }
    pub fn value(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode { inner: &self.inner.value, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'a> AstNode<'_, 'a, AssertStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::AssertStatement(self)
    }
    pub fn condition(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.condition,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.message.as_ref().map_or(0, |it| it.span().start),
        }
    }
    pub fn message(&self) -> Option<AstNode<'_, 'a, Expression<'a>>> {
        self.inner.message.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        })
    }
}
impl<'a> AstNode<'_, 'a, TryStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::TryStatement(self)
    }
    pub fn resources(&self) -> AstNode<'_, 'a, Vec<'a, Resource<'a>>> {
        AstNode {
            inner: &self.inner.resources,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.block.span().start,
        }
    }
    pub fn block(&self) -> AstNode<'_, 'a, Block<'a>> {
        AstNode {
            inner: &self.inner.block,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .catches
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.finally_block.as_ref().map(|it| it.span().start))
                .unwrap_or(0),
        }
    }
    pub fn catches(&self) -> AstNode<'_, 'a, Vec<'a, CatchClause<'a>>> {
        AstNode {
            inner: &self.inner.catches,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.finally_block.as_ref().map_or(0, |it| it.span().start),
        }
    }
    pub fn finally_block(&self) -> Option<AstNode<'_, 'a, Block<'a>>> {
        self.inner.finally_block.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, Resource<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            Resource::Declaration(field_0) => AstNodeKind::ResourceDeclaration(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Resource::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Resource::Field(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_declaration(&self) -> Option<AstNode<'parent, 'a, ResourceDeclaration<'a>>> {
        match self.inner {
            Resource::Declaration(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            Resource::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            Resource::Field(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_resource_access(&self) -> Option<AstNode<'parent, 'a, ResourceAccess<'a>>> {
        self.inner.as_resource_access().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, ResourceAccess<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            ResourceAccess::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ResourceAccess::Field(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            ResourceAccess::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ResourceAccess::Field(..) => None,
        }
    }
    pub fn as_field(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            ResourceAccess::Field(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ResourceAccess::Identifier(..) => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, ResourceDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ResourceDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.ty.span().start,
        }
    }
    pub fn ty(&self) -> AstNode<'_, 'a, VariableType<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.init.span().start,
        }
    }
    pub fn init(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.init,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, CatchClause<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::CatchClause(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .types
                .first()
                .map_or(self.inner.name.span().start, |it| it.span().start),
        }
    }
    pub fn types(&self) -> AstNode<'_, 'a, Vec<'a, Type<'a>>> {
        AstNode {
            inner: &self.inner.types,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.block.span().start,
        }
    }
    pub fn block(&self) -> AstNode<'_, 'a, Block<'a>> {
        AstNode {
            inner: &self.inner.block,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, SynchronizedStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::SynchronizedStatement(self)
    }
    pub fn lock(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.lock,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.body.span().start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, Block<'a>> {
        AstNode { inner: &self.inner.body, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'a> AstNode<'_, 'a, LabeledStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::LabeledStatement(self)
    }
    pub fn label(&self) -> AstNode<'_, 'a, LabelIdentifier<'a>> {
        AstNode {
            inner: &self.inner.label,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.statement.span().start,
        }
    }
    pub fn statement(&self) -> AstNode<'_, 'a, Statement<'a>> {
        AstNode {
            inner: &self.inner.statement,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        }
    }
}
impl<'a> AstNode<'_, 'a, ConstructorInvocation<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ConstructorInvocation(self)
    }
    pub fn qualifier(&self) -> Option<AstNode<'_, 'a, Expression<'a>>> {
        self.inner.qualifier.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .type_args
                .as_ref()
                .map(|it| it.span().start)
                .or_else(|| self.inner.args.first().map(|it| it.span().start))
                .unwrap_or(0),
        })
    }
    pub fn type_args(&self) -> Option<AstNode<'_, 'a, TypeArguments<'a>>> {
        self.inner.type_args.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.args.first().map_or(0, |it| it.span().start),
        })
    }
    pub fn args(&self) -> AstNode<'_, 'a, Vec<'a, Expression<'a>>> {
        AstNode { inner: &self.inner.args, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'a> AstNode<'_, 'a, ExpressionStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ExpressionStatement(self)
    }
    pub fn expression(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.expression,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        }
    }
}
impl<'a> AstNode<'_, 'a, LocalVariableDeclaration<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::LocalVariableDeclaration(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.ty.span().start,
        }
    }
    pub fn ty(&self) -> AstNode<'_, 'a, VariableType<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.declarators.first().map_or(0, |it| it.span().start),
        }
    }
    pub fn declarators(&self) -> AstNode<'_, 'a, Vec<'a, VariableDeclarator<'a>>> {
        AstNode {
            inner: &self.inner.declarators,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        }
    }
}
impl<'a> AstNode<'_, 'a, ReturnStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ReturnStatement(self)
    }
    pub fn value(&self) -> Option<AstNode<'_, 'a, Expression<'a>>> {
        self.inner.value.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: 0,
        })
    }
}
impl<'a> AstNode<'_, 'a, YieldStatement<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::YieldStatement(self)
    }
    pub fn value(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode { inner: &self.inner.value, parent: self.as_ast_nodes(), following_span_start: 0 }
    }
}
impl<'a> AstNode<'_, 'a, VariableDeclarator<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::VariableDeclarator(self)
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .dims
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.init.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn dims(&self) -> AstNode<'_, 'a, Vec<'a, ArrayTypeDimension<'a>>> {
        AstNode {
            inner: &self.inner.dims,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .init
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn init(&self) -> Option<AstNode<'_, 'a, VariableInitializer<'a>>> {
        self.inner.init.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, VariableInitializer<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            VariableInitializer::Array(field_0) => AstNodeKind::ArrayInitializer(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::IntLiteral(field_0) => AstNodeKind::IntLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::FloatLiteral(field_0) => AstNodeKind::FloatLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::CharLiteral(field_0) => AstNodeKind::CharLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::StringLiteral(field_0) => AstNodeKind::StringLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::BooleanLiteral(field_0) => AstNodeKind::BooleanLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::NullLiteral(field_0) => AstNodeKind::NullLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::This(field_0) => AstNodeKind::ThisExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::Super(field_0) => AstNodeKind::Super(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::QualifiedThis(field_0) => AstNodeKind::QualifiedThis(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::QualifiedSuper(field_0) => AstNodeKind::QualifiedSuper(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::FieldAccess(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::MethodCall(field_0) => AstNodeKind::MethodCall(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::ClassLiteral(field_0) => AstNodeKind::ClassLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::ClassInstanceCreation(field_0) => {
                AstNodeKind::ClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            VariableInitializer::QualifiedClassInstanceCreation(field_0) => {
                AstNodeKind::QualifiedClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            VariableInitializer::ArrayCreation(field_0) => AstNodeKind::ArrayCreation(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::ArrayAccess(field_0) => AstNodeKind::ArrayAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::Update(field_0) => AstNodeKind::UpdateExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::Unary(field_0) => AstNodeKind::UnaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::Cast(field_0) => AstNodeKind::CastExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::Instanceof(field_0) => {
                AstNodeKind::InstanceofExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            VariableInitializer::Binary(field_0) => AstNodeKind::BinaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::Assignment(field_0) => {
                AstNodeKind::AssignmentExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            VariableInitializer::Conditional(field_0) => {
                AstNodeKind::ConditionalExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            VariableInitializer::Switch(field_0) => AstNodeKind::SwitchExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::Parenthesized(field_0) => {
                AstNodeKind::ParenthesizedExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            VariableInitializer::Lambda(field_0) => AstNodeKind::LambdaExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableInitializer::MethodReference(field_0) => {
                AstNodeKind::MethodReference(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
        }
    }
    pub fn as_array(&self) -> Option<AstNode<'parent, 'a, ArrayInitializer<'a>>> {
        match self.inner {
            VariableInitializer::Array(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_int_literal(&self) -> Option<AstNode<'parent, 'a, IntLiteral<'a>>> {
        match self.inner {
            VariableInitializer::IntLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_float_literal(&self) -> Option<AstNode<'parent, 'a, FloatLiteral<'a>>> {
        match self.inner {
            VariableInitializer::FloatLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_char_literal(&self) -> Option<AstNode<'parent, 'a, CharLiteral<'a>>> {
        match self.inner {
            VariableInitializer::CharLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_string_literal(&self) -> Option<AstNode<'parent, 'a, StringLiteral<'a>>> {
        match self.inner {
            VariableInitializer::StringLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_boolean_literal(&self) -> Option<AstNode<'parent, 'a, BooleanLiteral>> {
        match self.inner {
            VariableInitializer::BooleanLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_null_literal(&self) -> Option<AstNode<'parent, 'a, NullLiteral>> {
        match self.inner {
            VariableInitializer::NullLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_this(&self) -> Option<AstNode<'parent, 'a, ThisExpression>> {
        match self.inner {
            VariableInitializer::This(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_super(&self) -> Option<AstNode<'parent, 'a, Super>> {
        match self.inner {
            VariableInitializer::Super(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_this(&self) -> Option<AstNode<'parent, 'a, QualifiedThis<'a>>> {
        match self.inner {
            VariableInitializer::QualifiedThis(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_super(&self) -> Option<AstNode<'parent, 'a, QualifiedSuper<'a>>> {
        match self.inner {
            VariableInitializer::QualifiedSuper(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            VariableInitializer::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            VariableInitializer::FieldAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_call(&self) -> Option<AstNode<'parent, 'a, MethodCall<'a>>> {
        match self.inner {
            VariableInitializer::MethodCall(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_literal(&self) -> Option<AstNode<'parent, 'a, ClassLiteral<'a>>> {
        match self.inner {
            VariableInitializer::ClassLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, ClassInstanceCreation<'a>>> {
        match self.inner {
            VariableInitializer::ClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, QualifiedClassInstanceCreation<'a>>> {
        match self.inner {
            VariableInitializer::QualifiedClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_creation(&self) -> Option<AstNode<'parent, 'a, ArrayCreation<'a>>> {
        match self.inner {
            VariableInitializer::ArrayCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_access(&self) -> Option<AstNode<'parent, 'a, ArrayAccess<'a>>> {
        match self.inner {
            VariableInitializer::ArrayAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_update(&self) -> Option<AstNode<'parent, 'a, UpdateExpression<'a>>> {
        match self.inner {
            VariableInitializer::Update(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_unary(&self) -> Option<AstNode<'parent, 'a, UnaryExpression<'a>>> {
        match self.inner {
            VariableInitializer::Unary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_cast(&self) -> Option<AstNode<'parent, 'a, CastExpression<'a>>> {
        match self.inner {
            VariableInitializer::Cast(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_instanceof(&self) -> Option<AstNode<'parent, 'a, InstanceofExpression<'a>>> {
        match self.inner {
            VariableInitializer::Instanceof(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_binary(&self) -> Option<AstNode<'parent, 'a, BinaryExpression<'a>>> {
        match self.inner {
            VariableInitializer::Binary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_assignment(&self) -> Option<AstNode<'parent, 'a, AssignmentExpression<'a>>> {
        match self.inner {
            VariableInitializer::Assignment(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_conditional(&self) -> Option<AstNode<'parent, 'a, ConditionalExpression<'a>>> {
        match self.inner {
            VariableInitializer::Conditional(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_switch(&self) -> Option<AstNode<'parent, 'a, SwitchExpression<'a>>> {
        match self.inner {
            VariableInitializer::Switch(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_parenthesized(&self) -> Option<AstNode<'parent, 'a, ParenthesizedExpression<'a>>> {
        match self.inner {
            VariableInitializer::Parenthesized(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_lambda(&self) -> Option<AstNode<'parent, 'a, LambdaExpression<'a>>> {
        match self.inner {
            VariableInitializer::Lambda(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_reference(&self) -> Option<AstNode<'parent, 'a, MethodReference<'a>>> {
        match self.inner {
            VariableInitializer::MethodReference(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_expression(&self) -> Option<AstNode<'parent, 'a, Expression<'a>>> {
        self.inner.as_expression().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, ArrayInitializer<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ArrayInitializer(self)
    }
    pub fn elements(&self) -> AstNode<'_, 'a, Vec<'a, VariableInitializer<'a>>> {
        AstNode {
            inner: &self.inner.elements,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, Expression<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            Expression::IntLiteral(field_0) => AstNodeKind::IntLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::FloatLiteral(field_0) => AstNodeKind::FloatLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::CharLiteral(field_0) => AstNodeKind::CharLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::StringLiteral(field_0) => AstNodeKind::StringLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::BooleanLiteral(field_0) => AstNodeKind::BooleanLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::NullLiteral(field_0) => AstNodeKind::NullLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::This(field_0) => AstNodeKind::ThisExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Super(field_0) => AstNodeKind::Super(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::QualifiedThis(field_0) => AstNodeKind::QualifiedThis(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::QualifiedSuper(field_0) => AstNodeKind::QualifiedSuper(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::FieldAccess(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::MethodCall(field_0) => AstNodeKind::MethodCall(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::ClassLiteral(field_0) => AstNodeKind::ClassLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::ClassInstanceCreation(field_0) => {
                AstNodeKind::ClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            Expression::QualifiedClassInstanceCreation(field_0) => {
                AstNodeKind::QualifiedClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            Expression::ArrayCreation(field_0) => AstNodeKind::ArrayCreation(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::ArrayAccess(field_0) => AstNodeKind::ArrayAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Update(field_0) => AstNodeKind::UpdateExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Unary(field_0) => AstNodeKind::UnaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Cast(field_0) => AstNodeKind::CastExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Instanceof(field_0) => AstNodeKind::InstanceofExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Binary(field_0) => AstNodeKind::BinaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Assignment(field_0) => AstNodeKind::AssignmentExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Conditional(field_0) => AstNodeKind::ConditionalExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Switch(field_0) => AstNodeKind::SwitchExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Parenthesized(field_0) => AstNodeKind::ParenthesizedExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::Lambda(field_0) => AstNodeKind::LambdaExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Expression::MethodReference(field_0) => AstNodeKind::MethodReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_int_literal(&self) -> Option<AstNode<'parent, 'a, IntLiteral<'a>>> {
        match self.inner {
            Expression::IntLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_float_literal(&self) -> Option<AstNode<'parent, 'a, FloatLiteral<'a>>> {
        match self.inner {
            Expression::FloatLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_char_literal(&self) -> Option<AstNode<'parent, 'a, CharLiteral<'a>>> {
        match self.inner {
            Expression::CharLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_string_literal(&self) -> Option<AstNode<'parent, 'a, StringLiteral<'a>>> {
        match self.inner {
            Expression::StringLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_boolean_literal(&self) -> Option<AstNode<'parent, 'a, BooleanLiteral>> {
        match self.inner {
            Expression::BooleanLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_null_literal(&self) -> Option<AstNode<'parent, 'a, NullLiteral>> {
        match self.inner {
            Expression::NullLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_this(&self) -> Option<AstNode<'parent, 'a, ThisExpression>> {
        match self.inner {
            Expression::This(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_super(&self) -> Option<AstNode<'parent, 'a, Super>> {
        match self.inner {
            Expression::Super(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_this(&self) -> Option<AstNode<'parent, 'a, QualifiedThis<'a>>> {
        match self.inner {
            Expression::QualifiedThis(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_super(&self) -> Option<AstNode<'parent, 'a, QualifiedSuper<'a>>> {
        match self.inner {
            Expression::QualifiedSuper(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            Expression::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            Expression::FieldAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_call(&self) -> Option<AstNode<'parent, 'a, MethodCall<'a>>> {
        match self.inner {
            Expression::MethodCall(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_literal(&self) -> Option<AstNode<'parent, 'a, ClassLiteral<'a>>> {
        match self.inner {
            Expression::ClassLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, ClassInstanceCreation<'a>>> {
        match self.inner {
            Expression::ClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, QualifiedClassInstanceCreation<'a>>> {
        match self.inner {
            Expression::QualifiedClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_creation(&self) -> Option<AstNode<'parent, 'a, ArrayCreation<'a>>> {
        match self.inner {
            Expression::ArrayCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_access(&self) -> Option<AstNode<'parent, 'a, ArrayAccess<'a>>> {
        match self.inner {
            Expression::ArrayAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_update(&self) -> Option<AstNode<'parent, 'a, UpdateExpression<'a>>> {
        match self.inner {
            Expression::Update(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_unary(&self) -> Option<AstNode<'parent, 'a, UnaryExpression<'a>>> {
        match self.inner {
            Expression::Unary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_cast(&self) -> Option<AstNode<'parent, 'a, CastExpression<'a>>> {
        match self.inner {
            Expression::Cast(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_instanceof(&self) -> Option<AstNode<'parent, 'a, InstanceofExpression<'a>>> {
        match self.inner {
            Expression::Instanceof(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_binary(&self) -> Option<AstNode<'parent, 'a, BinaryExpression<'a>>> {
        match self.inner {
            Expression::Binary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_assignment(&self) -> Option<AstNode<'parent, 'a, AssignmentExpression<'a>>> {
        match self.inner {
            Expression::Assignment(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_conditional(&self) -> Option<AstNode<'parent, 'a, ConditionalExpression<'a>>> {
        match self.inner {
            Expression::Conditional(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_switch(&self) -> Option<AstNode<'parent, 'a, SwitchExpression<'a>>> {
        match self.inner {
            Expression::Switch(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_parenthesized(&self) -> Option<AstNode<'parent, 'a, ParenthesizedExpression<'a>>> {
        match self.inner {
            Expression::Parenthesized(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_lambda(&self) -> Option<AstNode<'parent, 'a, LambdaExpression<'a>>> {
        match self.inner {
            Expression::Lambda(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_reference(&self) -> Option<AstNode<'parent, 'a, MethodReference<'a>>> {
        match self.inner {
            Expression::MethodReference(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, FieldAccess<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::FieldAccess(self)
    }
    pub fn target(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.target,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, MethodCall<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::MethodCall(self)
    }
    pub fn callee(&self) -> AstNode<'_, 'a, Callee<'a>> {
        AstNode {
            inner: &self.inner.callee,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .args
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn args(&self) -> AstNode<'_, 'a, Vec<'a, Expression<'a>>> {
        AstNode {
            inner: &self.inner.args,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, Callee<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            Callee::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Callee::Member(field_0) => AstNodeKind::MethodMember(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            Callee::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Callee::Member(..) => None,
        }
    }
    pub fn as_member(&self) -> Option<AstNode<'parent, 'a, MethodMember<'a>>> {
        match self.inner {
            Callee::Member(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Callee::Identifier(..) => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, MethodMember<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::MethodMember(self)
    }
    pub fn target(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.target,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .type_args
                .as_ref()
                .map_or(self.inner.name.span().start, |it| it.span().start),
        }
    }
    pub fn type_args(&self) -> Option<AstNode<'_, 'a, TypeArguments<'a>>> {
        self.inner.type_args.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        })
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ClassLiteral<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ClassLiteral(self)
    }
    pub fn ty(&self) -> AstNode<'_, 'a, MethodResult<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ThisExpression> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ThisExpression(self)
    }
}
impl<'a> AstNode<'_, 'a, Super> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::Super(self)
    }
}
impl<'a> AstNode<'_, 'a, QualifiedThis<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::QualifiedThis(self)
    }
    pub fn qualifier(&self) -> AstNode<'_, 'a, TypeName<'a>> {
        AstNode {
            inner: &self.inner.qualifier,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, QualifiedSuper<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::QualifiedSuper(self)
    }
    pub fn qualifier(&self) -> AstNode<'_, 'a, TypeName<'a>> {
        AstNode {
            inner: &self.inner.qualifier,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ClassInstanceCreation<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ClassInstanceCreation(self)
    }
    pub fn constructor_type_args(&self) -> Option<AstNode<'_, 'a, TypeArguments<'a>>> {
        self.inner.constructor_type_args.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.ty.span().start,
        })
    }
    pub fn ty(&self) -> AstNode<'_, 'a, ClassType<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .args
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.body.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn args(&self) -> AstNode<'_, 'a, Vec<'a, Expression<'a>>> {
        AstNode {
            inner: &self.inner.args,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .body
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn body(&self) -> Option<AstNode<'_, 'a, ClassBody<'a>>> {
        self.inner.body.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, QualifiedClassInstanceCreation<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::QualifiedClassInstanceCreation(self)
    }
    pub fn qualifier(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.qualifier,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .constructor_type_args
                .as_ref()
                .map(|it| it.span().start)
                .or_else(|| self.inner.annotations.first().map(|it| it.span().start))
                .unwrap_or(self.inner.name.span().start),
        }
    }
    pub fn constructor_type_args(&self) -> Option<AstNode<'_, 'a, TypeArguments<'a>>> {
        self.inner.constructor_type_args.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .annotations
                .first()
                .map_or(self.inner.name.span().start, |it| it.span().start),
        })
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .type_args
                .as_ref()
                .map(|it| it.span().start)
                .or_else(|| self.inner.args.first().map(|it| it.span().start))
                .or_else(|| self.inner.body.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn type_args(&self) -> Option<AstNode<'_, 'a, TypeArguments<'a>>> {
        self.inner.type_args.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .args
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.body.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        })
    }
    pub fn args(&self) -> AstNode<'_, 'a, Vec<'a, Expression<'a>>> {
        AstNode {
            inner: &self.inner.args,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .body
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn body(&self) -> Option<AstNode<'_, 'a, ClassBody<'a>>> {
        self.inner.body.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, ArrayCreation<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ArrayCreation(self)
    }
    pub fn element(&self) -> AstNode<'_, 'a, Type<'a>> {
        AstNode {
            inner: &self.inner.element,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .dims
                .first()
                .map(|it| it.span().start)
                .or_else(|| self.inner.init.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn dims(&self) -> AstNode<'_, 'a, Vec<'a, ArrayDimension<'a>>> {
        AstNode {
            inner: &self.inner.dims,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .init
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn init(&self) -> Option<AstNode<'_, 'a, ArrayInitializer<'a>>> {
        self.inner.init.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, ArrayDimension<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ArrayDimension(self)
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .length
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn length(&self) -> Option<AstNode<'_, 'a, Expression<'a>>> {
        self.inner.length.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, ArrayAccess<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ArrayAccess(self)
    }
    pub fn array(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.array,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.index.span().start,
        }
    }
    pub fn index(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.index,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, UpdateExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::UpdateExpression(self)
    }
    pub fn target(&self) -> AstNode<'_, 'a, AssignmentTarget<'a>> {
        AstNode {
            inner: &self.inner.target,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, UnaryExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::UnaryExpression(self)
    }
    pub fn operand(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.operand,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, CastExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::CastExpression(self)
    }
    pub fn ty(&self) -> AstNode<'_, 'a, Type<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .additional_bounds
                .first()
                .map_or(self.inner.operand.span().start, |it| it.span().start),
        }
    }
    pub fn additional_bounds(&self) -> AstNode<'_, 'a, Vec<'a, Type<'a>>> {
        AstNode {
            inner: &self.inner.additional_bounds,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.operand.span().start,
        }
    }
    pub fn operand(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.operand,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, InstanceofExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::InstanceofExpression(self)
    }
    pub fn operand(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.operand,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.rhs.span().start,
        }
    }
    pub fn rhs(&self) -> AstNode<'_, 'a, InstanceofRhs<'a>> {
        AstNode {
            inner: &self.inner.rhs,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, InstanceofRhs<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            InstanceofRhs::Primitive(field_0) => AstNodeKind::PrimitiveType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InstanceofRhs::Array(field_0) => AstNodeKind::ArrayType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InstanceofRhs::Simple(field_0) => AstNodeKind::SimpleClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InstanceofRhs::Qualified(field_0) => AstNodeKind::QualifiedClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InstanceofRhs::TypePattern(field_0) => AstNodeKind::TypePattern(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            InstanceofRhs::RecordPattern(field_0) => AstNodeKind::RecordPattern(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_primitive(&self) -> Option<AstNode<'parent, 'a, PrimitiveType<'a>>> {
        match self.inner {
            InstanceofRhs::Primitive(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<AstNode<'parent, 'a, ArrayType<'a>>> {
        match self.inner {
            InstanceofRhs::Array(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_simple(&self) -> Option<AstNode<'parent, 'a, SimpleClassType<'a>>> {
        match self.inner {
            InstanceofRhs::Simple(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified(&self) -> Option<AstNode<'parent, 'a, QualifiedClassType<'a>>> {
        match self.inner {
            InstanceofRhs::Qualified(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_type_pattern(&self) -> Option<AstNode<'parent, 'a, TypePattern<'a>>> {
        match self.inner {
            InstanceofRhs::TypePattern(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_record_pattern(&self) -> Option<AstNode<'parent, 'a, RecordPattern<'a>>> {
        match self.inner {
            InstanceofRhs::RecordPattern(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_type(&self) -> Option<AstNode<'parent, 'a, Type<'a>>> {
        self.inner.as_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_class_type(&self) -> Option<AstNode<'parent, 'a, ClassType<'a>>> {
        self.inner.as_class_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_pattern(&self) -> Option<AstNode<'parent, 'a, Pattern<'a>>> {
        self.inner.as_pattern().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, Pattern<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            Pattern::TypePattern(field_0) => AstNodeKind::TypePattern(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Pattern::RecordPattern(field_0) => AstNodeKind::RecordPattern(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_type_pattern(&self) -> Option<AstNode<'parent, 'a, TypePattern<'a>>> {
        match self.inner {
            Pattern::TypePattern(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Pattern::RecordPattern(..) => None,
        }
    }
    pub fn as_record_pattern(&self) -> Option<AstNode<'parent, 'a, RecordPattern<'a>>> {
        match self.inner {
            Pattern::RecordPattern(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Pattern::TypePattern(..) => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, TypePattern<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::TypePattern(self)
    }
    pub fn modifiers(&self) -> AstNode<'_, 'a, Vec<'a, Modifier<'a>>> {
        AstNode {
            inner: &self.inner.modifiers,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.ty.span().start,
        }
    }
    pub fn ty(&self) -> AstNode<'_, 'a, VariableType<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, RecordPattern<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::RecordPattern(self)
    }
    pub fn ty(&self) -> AstNode<'_, 'a, Type<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .components
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn components(&self) -> AstNode<'_, 'a, Vec<'a, ComponentPattern<'a>>> {
        AstNode {
            inner: &self.inner.components,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, ComponentPattern<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            ComponentPattern::MatchAll(field_0) => AstNodeKind::MatchAllPattern(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ComponentPattern::TypePattern(field_0) => AstNodeKind::TypePattern(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ComponentPattern::RecordPattern(field_0) => AstNodeKind::RecordPattern(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_match_all(&self) -> Option<AstNode<'parent, 'a, MatchAllPattern>> {
        match self.inner {
            ComponentPattern::MatchAll(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_type_pattern(&self) -> Option<AstNode<'parent, 'a, TypePattern<'a>>> {
        match self.inner {
            ComponentPattern::TypePattern(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_record_pattern(&self) -> Option<AstNode<'parent, 'a, RecordPattern<'a>>> {
        match self.inner {
            ComponentPattern::RecordPattern(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_pattern(&self) -> Option<AstNode<'parent, 'a, Pattern<'a>>> {
        self.inner.as_pattern().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, MatchAllPattern> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::MatchAllPattern(self)
    }
}
impl<'a> AstNode<'_, 'a, BinaryExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::BinaryExpression(self)
    }
    pub fn lhs(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.lhs,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.rhs.span().start,
        }
    }
    pub fn rhs(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.rhs,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, AssignmentExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::AssignmentExpression(self)
    }
    pub fn target(&self) -> AstNode<'_, 'a, AssignmentTarget<'a>> {
        AstNode {
            inner: &self.inner.target,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.value.span().start,
        }
    }
    pub fn value(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.value,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, AssignmentTarget<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            AssignmentTarget::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AssignmentTarget::FieldAccess(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AssignmentTarget::ArrayAccess(field_0) => AstNodeKind::ArrayAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AssignmentTarget::Parenthesized(field_0) => {
                AstNodeKind::ParenthesizedExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            AssignmentTarget::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            AssignmentTarget::FieldAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_access(&self) -> Option<AstNode<'parent, 'a, ArrayAccess<'a>>> {
        match self.inner {
            AssignmentTarget::ArrayAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_parenthesized(&self) -> Option<AstNode<'parent, 'a, ParenthesizedExpression<'a>>> {
        match self.inner {
            AssignmentTarget::Parenthesized(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, ConditionalExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ConditionalExpression(self)
    }
    pub fn condition(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.condition,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.when_true.span().start,
        }
    }
    pub fn when_true(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.when_true,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.when_false.span().start,
        }
    }
    pub fn when_false(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.when_false,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, SwitchExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::SwitchExpression(self)
    }
    pub fn selector(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.selector,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .cases
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn cases(&self) -> AstNode<'_, 'a, Vec<'a, SwitchCase<'a>>> {
        AstNode {
            inner: &self.inner.cases,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ParenthesizedExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ParenthesizedExpression(self)
    }
    pub fn expression(&self) -> AstNode<'_, 'a, Expression<'a>> {
        AstNode {
            inner: &self.inner.expression,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, LambdaExpression<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::LambdaExpression(self)
    }
    pub fn params(&self) -> AstNode<'_, 'a, FormalParameters<'a>> {
        AstNode {
            inner: &self.inner.params,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
    pub fn body(&self) -> AstNode<'_, 'a, LambdaBody<'a>> {
        AstNode {
            inner: &self.inner.body,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, LambdaBody<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            LambdaBody::Block(field_0) => AstNodeKind::Block(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::IntLiteral(field_0) => AstNodeKind::IntLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::FloatLiteral(field_0) => AstNodeKind::FloatLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::CharLiteral(field_0) => AstNodeKind::CharLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::StringLiteral(field_0) => AstNodeKind::StringLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::BooleanLiteral(field_0) => AstNodeKind::BooleanLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::NullLiteral(field_0) => AstNodeKind::NullLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::This(field_0) => AstNodeKind::ThisExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Super(field_0) => AstNodeKind::Super(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::QualifiedThis(field_0) => AstNodeKind::QualifiedThis(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::QualifiedSuper(field_0) => AstNodeKind::QualifiedSuper(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::FieldAccess(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::MethodCall(field_0) => AstNodeKind::MethodCall(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::ClassLiteral(field_0) => AstNodeKind::ClassLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::ClassInstanceCreation(field_0) => {
                AstNodeKind::ClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            LambdaBody::QualifiedClassInstanceCreation(field_0) => {
                AstNodeKind::QualifiedClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            LambdaBody::ArrayCreation(field_0) => AstNodeKind::ArrayCreation(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::ArrayAccess(field_0) => AstNodeKind::ArrayAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Update(field_0) => AstNodeKind::UpdateExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Unary(field_0) => AstNodeKind::UnaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Cast(field_0) => AstNodeKind::CastExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Instanceof(field_0) => AstNodeKind::InstanceofExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Binary(field_0) => AstNodeKind::BinaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Assignment(field_0) => AstNodeKind::AssignmentExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Conditional(field_0) => AstNodeKind::ConditionalExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Switch(field_0) => AstNodeKind::SwitchExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Parenthesized(field_0) => AstNodeKind::ParenthesizedExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::Lambda(field_0) => AstNodeKind::LambdaExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            LambdaBody::MethodReference(field_0) => AstNodeKind::MethodReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_block(&self) -> Option<AstNode<'parent, 'a, Block<'a>>> {
        match self.inner {
            LambdaBody::Block(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_int_literal(&self) -> Option<AstNode<'parent, 'a, IntLiteral<'a>>> {
        match self.inner {
            LambdaBody::IntLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_float_literal(&self) -> Option<AstNode<'parent, 'a, FloatLiteral<'a>>> {
        match self.inner {
            LambdaBody::FloatLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_char_literal(&self) -> Option<AstNode<'parent, 'a, CharLiteral<'a>>> {
        match self.inner {
            LambdaBody::CharLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_string_literal(&self) -> Option<AstNode<'parent, 'a, StringLiteral<'a>>> {
        match self.inner {
            LambdaBody::StringLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_boolean_literal(&self) -> Option<AstNode<'parent, 'a, BooleanLiteral>> {
        match self.inner {
            LambdaBody::BooleanLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_null_literal(&self) -> Option<AstNode<'parent, 'a, NullLiteral>> {
        match self.inner {
            LambdaBody::NullLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_this(&self) -> Option<AstNode<'parent, 'a, ThisExpression>> {
        match self.inner {
            LambdaBody::This(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_super(&self) -> Option<AstNode<'parent, 'a, Super>> {
        match self.inner {
            LambdaBody::Super(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_this(&self) -> Option<AstNode<'parent, 'a, QualifiedThis<'a>>> {
        match self.inner {
            LambdaBody::QualifiedThis(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_super(&self) -> Option<AstNode<'parent, 'a, QualifiedSuper<'a>>> {
        match self.inner {
            LambdaBody::QualifiedSuper(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            LambdaBody::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            LambdaBody::FieldAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_call(&self) -> Option<AstNode<'parent, 'a, MethodCall<'a>>> {
        match self.inner {
            LambdaBody::MethodCall(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_literal(&self) -> Option<AstNode<'parent, 'a, ClassLiteral<'a>>> {
        match self.inner {
            LambdaBody::ClassLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, ClassInstanceCreation<'a>>> {
        match self.inner {
            LambdaBody::ClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, QualifiedClassInstanceCreation<'a>>> {
        match self.inner {
            LambdaBody::QualifiedClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_creation(&self) -> Option<AstNode<'parent, 'a, ArrayCreation<'a>>> {
        match self.inner {
            LambdaBody::ArrayCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_access(&self) -> Option<AstNode<'parent, 'a, ArrayAccess<'a>>> {
        match self.inner {
            LambdaBody::ArrayAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_update(&self) -> Option<AstNode<'parent, 'a, UpdateExpression<'a>>> {
        match self.inner {
            LambdaBody::Update(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_unary(&self) -> Option<AstNode<'parent, 'a, UnaryExpression<'a>>> {
        match self.inner {
            LambdaBody::Unary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_cast(&self) -> Option<AstNode<'parent, 'a, CastExpression<'a>>> {
        match self.inner {
            LambdaBody::Cast(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_instanceof(&self) -> Option<AstNode<'parent, 'a, InstanceofExpression<'a>>> {
        match self.inner {
            LambdaBody::Instanceof(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_binary(&self) -> Option<AstNode<'parent, 'a, BinaryExpression<'a>>> {
        match self.inner {
            LambdaBody::Binary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_assignment(&self) -> Option<AstNode<'parent, 'a, AssignmentExpression<'a>>> {
        match self.inner {
            LambdaBody::Assignment(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_conditional(&self) -> Option<AstNode<'parent, 'a, ConditionalExpression<'a>>> {
        match self.inner {
            LambdaBody::Conditional(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_switch(&self) -> Option<AstNode<'parent, 'a, SwitchExpression<'a>>> {
        match self.inner {
            LambdaBody::Switch(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_parenthesized(&self) -> Option<AstNode<'parent, 'a, ParenthesizedExpression<'a>>> {
        match self.inner {
            LambdaBody::Parenthesized(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_lambda(&self) -> Option<AstNode<'parent, 'a, LambdaExpression<'a>>> {
        match self.inner {
            LambdaBody::Lambda(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_reference(&self) -> Option<AstNode<'parent, 'a, MethodReference<'a>>> {
        match self.inner {
            LambdaBody::MethodReference(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_expression(&self) -> Option<AstNode<'parent, 'a, Expression<'a>>> {
        self.inner.as_expression().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, MethodReference<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::MethodReference(self)
    }
    pub fn target(&self) -> AstNode<'_, 'a, MethodReferenceTarget<'a>> {
        AstNode {
            inner: &self.inner.target,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .type_args
                .as_ref()
                .map(|it| it.span().start)
                .or_else(|| self.inner.method.as_ref().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn type_args(&self) -> Option<AstNode<'_, 'a, TypeArguments<'a>>> {
        self.inner.type_args.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .method
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        })
    }
    pub fn method(&self) -> Option<AstNode<'_, 'a, IdentifierName<'a>>> {
        self.inner.method.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, MethodReferenceTarget<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            MethodReferenceTarget::IntLiteral(field_0) => AstNodeKind::IntLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::FloatLiteral(field_0) => AstNodeKind::FloatLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::CharLiteral(field_0) => AstNodeKind::CharLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::StringLiteral(field_0) => AstNodeKind::StringLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::BooleanLiteral(field_0) => {
                AstNodeKind::BooleanLiteral(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::NullLiteral(field_0) => AstNodeKind::NullLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::This(field_0) => AstNodeKind::ThisExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Super(field_0) => AstNodeKind::Super(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::QualifiedThis(field_0) => AstNodeKind::QualifiedThis(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::QualifiedSuper(field_0) => {
                AstNodeKind::QualifiedSuper(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::Identifier(field_0) => {
                AstNodeKind::IdentifierReference(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::FieldAccess(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::MethodCall(field_0) => AstNodeKind::MethodCall(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::ClassLiteral(field_0) => AstNodeKind::ClassLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::ClassInstanceCreation(field_0) => {
                AstNodeKind::ClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::QualifiedClassInstanceCreation(field_0) => {
                AstNodeKind::QualifiedClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::ArrayCreation(field_0) => AstNodeKind::ArrayCreation(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::ArrayAccess(field_0) => AstNodeKind::ArrayAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Update(field_0) => AstNodeKind::UpdateExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Unary(field_0) => AstNodeKind::UnaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Cast(field_0) => AstNodeKind::CastExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Instanceof(field_0) => {
                AstNodeKind::InstanceofExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::Binary(field_0) => AstNodeKind::BinaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Assignment(field_0) => {
                AstNodeKind::AssignmentExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::Conditional(field_0) => {
                AstNodeKind::ConditionalExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::Switch(field_0) => AstNodeKind::SwitchExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Parenthesized(field_0) => {
                AstNodeKind::ParenthesizedExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::Lambda(field_0) => AstNodeKind::LambdaExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::MethodReference(field_0) => {
                AstNodeKind::MethodReference(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            MethodReferenceTarget::Primitive(field_0) => AstNodeKind::PrimitiveType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Array(field_0) => AstNodeKind::ArrayType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Simple(field_0) => AstNodeKind::SimpleClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodReferenceTarget::Qualified(field_0) => AstNodeKind::QualifiedClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_int_literal(&self) -> Option<AstNode<'parent, 'a, IntLiteral<'a>>> {
        match self.inner {
            MethodReferenceTarget::IntLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_float_literal(&self) -> Option<AstNode<'parent, 'a, FloatLiteral<'a>>> {
        match self.inner {
            MethodReferenceTarget::FloatLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_char_literal(&self) -> Option<AstNode<'parent, 'a, CharLiteral<'a>>> {
        match self.inner {
            MethodReferenceTarget::CharLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_string_literal(&self) -> Option<AstNode<'parent, 'a, StringLiteral<'a>>> {
        match self.inner {
            MethodReferenceTarget::StringLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_boolean_literal(&self) -> Option<AstNode<'parent, 'a, BooleanLiteral>> {
        match self.inner {
            MethodReferenceTarget::BooleanLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_null_literal(&self) -> Option<AstNode<'parent, 'a, NullLiteral>> {
        match self.inner {
            MethodReferenceTarget::NullLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_this(&self) -> Option<AstNode<'parent, 'a, ThisExpression>> {
        match self.inner {
            MethodReferenceTarget::This(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_super(&self) -> Option<AstNode<'parent, 'a, Super>> {
        match self.inner {
            MethodReferenceTarget::Super(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_this(&self) -> Option<AstNode<'parent, 'a, QualifiedThis<'a>>> {
        match self.inner {
            MethodReferenceTarget::QualifiedThis(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_super(&self) -> Option<AstNode<'parent, 'a, QualifiedSuper<'a>>> {
        match self.inner {
            MethodReferenceTarget::QualifiedSuper(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            MethodReferenceTarget::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            MethodReferenceTarget::FieldAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_call(&self) -> Option<AstNode<'parent, 'a, MethodCall<'a>>> {
        match self.inner {
            MethodReferenceTarget::MethodCall(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_literal(&self) -> Option<AstNode<'parent, 'a, ClassLiteral<'a>>> {
        match self.inner {
            MethodReferenceTarget::ClassLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, ClassInstanceCreation<'a>>> {
        match self.inner {
            MethodReferenceTarget::ClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, QualifiedClassInstanceCreation<'a>>> {
        match self.inner {
            MethodReferenceTarget::QualifiedClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_creation(&self) -> Option<AstNode<'parent, 'a, ArrayCreation<'a>>> {
        match self.inner {
            MethodReferenceTarget::ArrayCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_access(&self) -> Option<AstNode<'parent, 'a, ArrayAccess<'a>>> {
        match self.inner {
            MethodReferenceTarget::ArrayAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_update(&self) -> Option<AstNode<'parent, 'a, UpdateExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Update(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_unary(&self) -> Option<AstNode<'parent, 'a, UnaryExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Unary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_cast(&self) -> Option<AstNode<'parent, 'a, CastExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Cast(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_instanceof(&self) -> Option<AstNode<'parent, 'a, InstanceofExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Instanceof(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_binary(&self) -> Option<AstNode<'parent, 'a, BinaryExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Binary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_assignment(&self) -> Option<AstNode<'parent, 'a, AssignmentExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Assignment(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_conditional(&self) -> Option<AstNode<'parent, 'a, ConditionalExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Conditional(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_switch(&self) -> Option<AstNode<'parent, 'a, SwitchExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Switch(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_parenthesized(&self) -> Option<AstNode<'parent, 'a, ParenthesizedExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Parenthesized(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_lambda(&self) -> Option<AstNode<'parent, 'a, LambdaExpression<'a>>> {
        match self.inner {
            MethodReferenceTarget::Lambda(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_reference(&self) -> Option<AstNode<'parent, 'a, MethodReference<'a>>> {
        match self.inner {
            MethodReferenceTarget::MethodReference(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_primitive(&self) -> Option<AstNode<'parent, 'a, PrimitiveType<'a>>> {
        match self.inner {
            MethodReferenceTarget::Primitive(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<AstNode<'parent, 'a, ArrayType<'a>>> {
        match self.inner {
            MethodReferenceTarget::Array(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_simple(&self) -> Option<AstNode<'parent, 'a, SimpleClassType<'a>>> {
        match self.inner {
            MethodReferenceTarget::Simple(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified(&self) -> Option<AstNode<'parent, 'a, QualifiedClassType<'a>>> {
        match self.inner {
            MethodReferenceTarget::Qualified(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_expression(&self) -> Option<AstNode<'parent, 'a, Expression<'a>>> {
        self.inner.as_expression().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_type(&self) -> Option<AstNode<'parent, 'a, Type<'a>>> {
        self.inner.as_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_class_type(&self) -> Option<AstNode<'parent, 'a, ClassType<'a>>> {
        self.inner.as_class_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, TypeParameters<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::TypeParameters(self)
    }
    pub fn params(&self) -> AstNode<'_, 'a, Vec<'a, TypeParameter<'a>>> {
        AstNode {
            inner: &self.inner.params,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, TypeParameter<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::TypeParameter(self)
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, BindingIdentifier<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .bounds
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn bounds(&self) -> AstNode<'_, 'a, Vec<'a, Type<'a>>> {
        AstNode {
            inner: &self.inner.bounds,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, VariableType<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            VariableType::Var(field_0) => AstNodeKind::VarType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableType::Primitive(field_0) => AstNodeKind::PrimitiveType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableType::Array(field_0) => AstNodeKind::ArrayType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableType::Simple(field_0) => AstNodeKind::SimpleClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            VariableType::Qualified(field_0) => AstNodeKind::QualifiedClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_var(&self) -> Option<AstNode<'parent, 'a, VarType>> {
        match self.inner {
            VariableType::Var(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_primitive(&self) -> Option<AstNode<'parent, 'a, PrimitiveType<'a>>> {
        match self.inner {
            VariableType::Primitive(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<AstNode<'parent, 'a, ArrayType<'a>>> {
        match self.inner {
            VariableType::Array(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_simple(&self) -> Option<AstNode<'parent, 'a, SimpleClassType<'a>>> {
        match self.inner {
            VariableType::Simple(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified(&self) -> Option<AstNode<'parent, 'a, QualifiedClassType<'a>>> {
        match self.inner {
            VariableType::Qualified(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_type(&self) -> Option<AstNode<'parent, 'a, Type<'a>>> {
        self.inner.as_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_class_type(&self) -> Option<AstNode<'parent, 'a, ClassType<'a>>> {
        self.inner.as_class_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, VarType> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::VarType(self)
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, Type<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            Type::Primitive(field_0) => AstNodeKind::PrimitiveType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Type::Array(field_0) => AstNodeKind::ArrayType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Type::Simple(field_0) => AstNodeKind::SimpleClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Type::Qualified(field_0) => AstNodeKind::QualifiedClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_primitive(&self) -> Option<AstNode<'parent, 'a, PrimitiveType<'a>>> {
        match self.inner {
            Type::Primitive(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<AstNode<'parent, 'a, ArrayType<'a>>> {
        match self.inner {
            Type::Array(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_simple(&self) -> Option<AstNode<'parent, 'a, SimpleClassType<'a>>> {
        match self.inner {
            Type::Simple(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified(&self) -> Option<AstNode<'parent, 'a, QualifiedClassType<'a>>> {
        match self.inner {
            Type::Qualified(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_type(&self) -> Option<AstNode<'parent, 'a, ClassType<'a>>> {
        self.inner.as_class_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, MethodResult<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            MethodResult::Void(field_0) => AstNodeKind::VoidType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodResult::Primitive(field_0) => AstNodeKind::PrimitiveType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodResult::Array(field_0) => AstNodeKind::ArrayType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodResult::Simple(field_0) => AstNodeKind::SimpleClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            MethodResult::Qualified(field_0) => AstNodeKind::QualifiedClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_void(&self) -> Option<AstNode<'parent, 'a, VoidType>> {
        match self.inner {
            MethodResult::Void(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_primitive(&self) -> Option<AstNode<'parent, 'a, PrimitiveType<'a>>> {
        match self.inner {
            MethodResult::Primitive(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<AstNode<'parent, 'a, ArrayType<'a>>> {
        match self.inner {
            MethodResult::Array(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_simple(&self) -> Option<AstNode<'parent, 'a, SimpleClassType<'a>>> {
        match self.inner {
            MethodResult::Simple(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified(&self) -> Option<AstNode<'parent, 'a, QualifiedClassType<'a>>> {
        match self.inner {
            MethodResult::Qualified(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_type(&self) -> Option<AstNode<'parent, 'a, Type<'a>>> {
        self.inner.as_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_class_type(&self) -> Option<AstNode<'parent, 'a, ClassType<'a>>> {
        self.inner.as_class_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, VoidType> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::VoidType(self)
    }
}
impl<'a> AstNode<'_, 'a, PrimitiveType<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::PrimitiveType(self)
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, ClassType<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            ClassType::Simple(field_0) => AstNodeKind::SimpleClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassType::Qualified(field_0) => AstNodeKind::QualifiedClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_simple(&self) -> Option<AstNode<'parent, 'a, SimpleClassType<'a>>> {
        match self.inner {
            ClassType::Simple(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassType::Qualified(..) => None,
        }
    }
    pub fn as_qualified(&self) -> Option<AstNode<'parent, 'a, QualifiedClassType<'a>>> {
        match self.inner {
            ClassType::Qualified(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ClassType::Simple(..) => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, SimpleClassType<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::SimpleClassType(self)
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierReference<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .args
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn args(&self) -> Option<AstNode<'_, 'a, TypeArguments<'a>>> {
        self.inner.args.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, QualifiedClassType<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::QualifiedClassType(self)
    }
    pub fn qualifier(&self) -> AstNode<'_, 'a, ClassType<'a>> {
        AstNode {
            inner: &self.inner.qualifier,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .annotations
                .first()
                .map_or(self.inner.name.span().start, |it| it.span().start),
        }
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .args
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn args(&self) -> Option<AstNode<'_, 'a, TypeArguments<'a>>> {
        self.inner.args.as_ref().map(|inner| AstNode {
            inner: (inner).as_ref(),
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, ArrayType<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ArrayType(self)
    }
    pub fn element(&self) -> AstNode<'_, 'a, Type<'a>> {
        AstNode {
            inner: &self.inner.element,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .annotations
                .first()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, ArrayTypeDimension<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ArrayTypeDimension(self)
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, TypeArguments<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::TypeArguments(self)
    }
    pub fn args(&self) -> AstNode<'_, 'a, Vec<'a, TypeArgument<'a>>> {
        AstNode {
            inner: &self.inner.args,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, TypeArgument<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            TypeArgument::Wildcard(field_0) => AstNodeKind::WildcardTypeArgument(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeArgument::Primitive(field_0) => AstNodeKind::PrimitiveType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeArgument::Array(field_0) => AstNodeKind::ArrayType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeArgument::Simple(field_0) => AstNodeKind::SimpleClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeArgument::Qualified(field_0) => AstNodeKind::QualifiedClassType(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_wildcard(&self) -> Option<AstNode<'parent, 'a, WildcardTypeArgument<'a>>> {
        match self.inner {
            TypeArgument::Wildcard(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_primitive(&self) -> Option<AstNode<'parent, 'a, PrimitiveType<'a>>> {
        match self.inner {
            TypeArgument::Primitive(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<AstNode<'parent, 'a, ArrayType<'a>>> {
        match self.inner {
            TypeArgument::Array(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_simple(&self) -> Option<AstNode<'parent, 'a, SimpleClassType<'a>>> {
        match self.inner {
            TypeArgument::Simple(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified(&self) -> Option<AstNode<'parent, 'a, QualifiedClassType<'a>>> {
        match self.inner {
            TypeArgument::Qualified(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_type(&self) -> Option<AstNode<'parent, 'a, Type<'a>>> {
        self.inner.as_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_class_type(&self) -> Option<AstNode<'parent, 'a, ClassType<'a>>> {
        self.inner.as_class_type().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, WildcardTypeArgument<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::WildcardTypeArgument(self)
    }
    pub fn annotations(&self) -> AstNode<'_, 'a, Vec<'a, Annotation<'a>>> {
        AstNode {
            inner: &self.inner.annotations,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .bound
                .as_ref()
                .map_or(self.following_span_start, |it| it.span().start),
        }
    }
    pub fn bound(&self) -> Option<AstNode<'_, 'a, WildcardBound<'a>>> {
        self.inner.bound.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, WildcardBound<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::WildcardBound(self)
    }
    pub fn ty(&self) -> AstNode<'_, 'a, Type<'a>> {
        AstNode {
            inner: &self.inner.ty,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'a> AstNode<'_, 'a, IntLiteral<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::IntLiteral(self)
    }
}
impl<'a> AstNode<'_, 'a, FloatLiteral<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::FloatLiteral(self)
    }
}
impl<'a> AstNode<'_, 'a, CharLiteral<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::CharLiteral(self)
    }
}
impl<'a> AstNode<'_, 'a, StringLiteral<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::StringLiteral(self)
    }
}
impl<'a> AstNode<'_, 'a, BooleanLiteral> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::BooleanLiteral(self)
    }
}
impl<'a> AstNode<'_, 'a, NullLiteral> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::NullLiteral(self)
    }
}
impl<'a> AstNode<'_, 'a, IdentifierName<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::IdentifierName(self)
    }
}
impl<'a> AstNode<'_, 'a, IdentifierReference<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::IdentifierReference(self)
    }
}
impl<'a> AstNode<'_, 'a, BindingIdentifier<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::BindingIdentifier(self)
    }
}
impl<'a> AstNode<'_, 'a, LabelIdentifier<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::LabelIdentifier(self)
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, Name<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            Name::Identifier(field_0) => AstNodeKind::IdentifierName(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Name::Qualified(field_0) => AstNodeKind::QualifiedName(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierName<'a>>> {
        match self.inner {
            Name::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Name::Qualified(..) => None,
        }
    }
    pub fn as_qualified(&self) -> Option<AstNode<'parent, 'a, QualifiedName<'a>>> {
        match self.inner {
            Name::Qualified(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Name::Identifier(..) => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, QualifiedName<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::QualifiedName(self)
    }
    pub fn qualifier(&self) -> AstNode<'_, 'a, Name<'a>> {
        AstNode {
            inner: &self.inner.qualifier,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, TypeName<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            TypeName::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeName::Qualified(field_0) => AstNodeKind::QualifiedTypeName(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            TypeName::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeName::Qualified(..) => None,
        }
    }
    pub fn as_qualified(&self) -> Option<AstNode<'parent, 'a, QualifiedTypeName<'a>>> {
        match self.inner {
            TypeName::Qualified(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            TypeName::Identifier(..) => None,
        }
    }
}
impl<'a> AstNode<'_, 'a, QualifiedTypeName<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::QualifiedTypeName(self)
    }
    pub fn qualifier(&self) -> AstNode<'_, 'a, TypeName<'a>> {
        AstNode {
            inner: &self.inner.qualifier,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.name.span().start,
        }
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, Modifier<'a>> {
    pub fn as_keyword(&self) -> Option<AstNode<'parent, 'a, KeywordModifier>> {
        match self.inner {
            Modifier::Keyword(field_0) => Some(AstNode {
                inner: field_0,
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Modifier::Annotation(..) => None,
        }
    }
    pub fn as_annotation(&self) -> Option<AstNode<'parent, 'a, Annotation<'a>>> {
        match self.inner {
            Modifier::Annotation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            Modifier::Keyword(..) => None,
        }
    }
}
impl AstNode<'_, '_, KeywordModifier> {}
impl<'a> AstNode<'_, 'a, Annotation<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::Annotation(self)
    }
    pub fn name(&self) -> AstNode<'_, 'a, TypeName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self
                .inner
                .args
                .as_ref()
                .and_then(|it| it.first().map(|it| it.span().start))
                .unwrap_or(self.following_span_start),
        }
    }
    pub fn args(&self) -> Option<AstNode<'_, 'a, Vec<'a, AnnotationArgument<'a>>>> {
        self.inner.args.as_ref().map(|inner| AstNode {
            inner,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        })
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, AnnotationArgument<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            AnnotationArgument::Pair(field_0) => AstNodeKind::ElementValuePair(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Annotation(field_0) => AstNodeKind::Annotation(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Array(field_0) => {
                AstNodeKind::ElementValueArrayInitializer(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationArgument::IntLiteral(field_0) => AstNodeKind::IntLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::FloatLiteral(field_0) => AstNodeKind::FloatLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::CharLiteral(field_0) => AstNodeKind::CharLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::StringLiteral(field_0) => AstNodeKind::StringLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::BooleanLiteral(field_0) => AstNodeKind::BooleanLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::NullLiteral(field_0) => AstNodeKind::NullLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::This(field_0) => AstNodeKind::ThisExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Super(field_0) => AstNodeKind::Super(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::QualifiedThis(field_0) => AstNodeKind::QualifiedThis(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::QualifiedSuper(field_0) => AstNodeKind::QualifiedSuper(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::FieldAccess(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::MethodCall(field_0) => AstNodeKind::MethodCall(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::ClassLiteral(field_0) => AstNodeKind::ClassLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::ClassInstanceCreation(field_0) => {
                AstNodeKind::ClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationArgument::QualifiedClassInstanceCreation(field_0) => {
                AstNodeKind::QualifiedClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationArgument::ArrayCreation(field_0) => AstNodeKind::ArrayCreation(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::ArrayAccess(field_0) => AstNodeKind::ArrayAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Update(field_0) => AstNodeKind::UpdateExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Unary(field_0) => AstNodeKind::UnaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Cast(field_0) => AstNodeKind::CastExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Instanceof(field_0) => AstNodeKind::InstanceofExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Binary(field_0) => AstNodeKind::BinaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Assignment(field_0) => AstNodeKind::AssignmentExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Conditional(field_0) => {
                AstNodeKind::ConditionalExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationArgument::Switch(field_0) => AstNodeKind::SwitchExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::Parenthesized(field_0) => {
                AstNodeKind::ParenthesizedExpression(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            AnnotationArgument::Lambda(field_0) => AstNodeKind::LambdaExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            AnnotationArgument::MethodReference(field_0) => AstNodeKind::MethodReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_pair(&self) -> Option<AstNode<'parent, 'a, ElementValuePair<'a>>> {
        match self.inner {
            AnnotationArgument::Pair(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_annotation(&self) -> Option<AstNode<'parent, 'a, Annotation<'a>>> {
        match self.inner {
            AnnotationArgument::Annotation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<AstNode<'parent, 'a, ElementValueArrayInitializer<'a>>> {
        match self.inner {
            AnnotationArgument::Array(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_int_literal(&self) -> Option<AstNode<'parent, 'a, IntLiteral<'a>>> {
        match self.inner {
            AnnotationArgument::IntLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_float_literal(&self) -> Option<AstNode<'parent, 'a, FloatLiteral<'a>>> {
        match self.inner {
            AnnotationArgument::FloatLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_char_literal(&self) -> Option<AstNode<'parent, 'a, CharLiteral<'a>>> {
        match self.inner {
            AnnotationArgument::CharLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_string_literal(&self) -> Option<AstNode<'parent, 'a, StringLiteral<'a>>> {
        match self.inner {
            AnnotationArgument::StringLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_boolean_literal(&self) -> Option<AstNode<'parent, 'a, BooleanLiteral>> {
        match self.inner {
            AnnotationArgument::BooleanLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_null_literal(&self) -> Option<AstNode<'parent, 'a, NullLiteral>> {
        match self.inner {
            AnnotationArgument::NullLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_this(&self) -> Option<AstNode<'parent, 'a, ThisExpression>> {
        match self.inner {
            AnnotationArgument::This(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_super(&self) -> Option<AstNode<'parent, 'a, Super>> {
        match self.inner {
            AnnotationArgument::Super(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_this(&self) -> Option<AstNode<'parent, 'a, QualifiedThis<'a>>> {
        match self.inner {
            AnnotationArgument::QualifiedThis(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_super(&self) -> Option<AstNode<'parent, 'a, QualifiedSuper<'a>>> {
        match self.inner {
            AnnotationArgument::QualifiedSuper(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            AnnotationArgument::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            AnnotationArgument::FieldAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_call(&self) -> Option<AstNode<'parent, 'a, MethodCall<'a>>> {
        match self.inner {
            AnnotationArgument::MethodCall(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_literal(&self) -> Option<AstNode<'parent, 'a, ClassLiteral<'a>>> {
        match self.inner {
            AnnotationArgument::ClassLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, ClassInstanceCreation<'a>>> {
        match self.inner {
            AnnotationArgument::ClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, QualifiedClassInstanceCreation<'a>>> {
        match self.inner {
            AnnotationArgument::QualifiedClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_creation(&self) -> Option<AstNode<'parent, 'a, ArrayCreation<'a>>> {
        match self.inner {
            AnnotationArgument::ArrayCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_access(&self) -> Option<AstNode<'parent, 'a, ArrayAccess<'a>>> {
        match self.inner {
            AnnotationArgument::ArrayAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_update(&self) -> Option<AstNode<'parent, 'a, UpdateExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Update(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_unary(&self) -> Option<AstNode<'parent, 'a, UnaryExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Unary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_cast(&self) -> Option<AstNode<'parent, 'a, CastExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Cast(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_instanceof(&self) -> Option<AstNode<'parent, 'a, InstanceofExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Instanceof(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_binary(&self) -> Option<AstNode<'parent, 'a, BinaryExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Binary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_assignment(&self) -> Option<AstNode<'parent, 'a, AssignmentExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Assignment(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_conditional(&self) -> Option<AstNode<'parent, 'a, ConditionalExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Conditional(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_switch(&self) -> Option<AstNode<'parent, 'a, SwitchExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Switch(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_parenthesized(&self) -> Option<AstNode<'parent, 'a, ParenthesizedExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Parenthesized(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_lambda(&self) -> Option<AstNode<'parent, 'a, LambdaExpression<'a>>> {
        match self.inner {
            AnnotationArgument::Lambda(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_reference(&self) -> Option<AstNode<'parent, 'a, MethodReference<'a>>> {
        match self.inner {
            AnnotationArgument::MethodReference(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_element_value(&self) -> Option<AstNode<'parent, 'a, ElementValue<'a>>> {
        self.inner.as_element_value().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
    pub fn as_expression(&self) -> Option<AstNode<'parent, 'a, Expression<'a>>> {
        self.inner.as_expression().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, ElementValuePair<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ElementValuePair(self)
    }
    pub fn name(&self) -> AstNode<'_, 'a, IdentifierName<'a>> {
        AstNode {
            inner: &self.inner.name,
            parent: self.as_ast_nodes(),
            following_span_start: self.inner.value.span().start,
        }
    }
    pub fn value(&self) -> AstNode<'_, 'a, ElementValue<'a>> {
        AstNode {
            inner: &self.inner.value,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
impl<'parent, 'a> AstNode<'parent, 'a, ElementValue<'a>> {
    pub fn as_kind(&self) -> AstNodeKind<'parent, 'a> {
        match self.inner {
            ElementValue::Annotation(field_0) => AstNodeKind::Annotation(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Array(field_0) => AstNodeKind::ElementValueArrayInitializer(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::IntLiteral(field_0) => AstNodeKind::IntLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::FloatLiteral(field_0) => AstNodeKind::FloatLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::CharLiteral(field_0) => AstNodeKind::CharLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::StringLiteral(field_0) => AstNodeKind::StringLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::BooleanLiteral(field_0) => AstNodeKind::BooleanLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::NullLiteral(field_0) => AstNodeKind::NullLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::This(field_0) => AstNodeKind::ThisExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Super(field_0) => AstNodeKind::Super(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::QualifiedThis(field_0) => AstNodeKind::QualifiedThis(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::QualifiedSuper(field_0) => AstNodeKind::QualifiedSuper(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Identifier(field_0) => AstNodeKind::IdentifierReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::FieldAccess(field_0) => AstNodeKind::FieldAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::MethodCall(field_0) => AstNodeKind::MethodCall(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::ClassLiteral(field_0) => AstNodeKind::ClassLiteral(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::ClassInstanceCreation(field_0) => {
                AstNodeKind::ClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            ElementValue::QualifiedClassInstanceCreation(field_0) => {
                AstNodeKind::QualifiedClassInstanceCreation(AstNode {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                })
            }
            ElementValue::ArrayCreation(field_0) => AstNodeKind::ArrayCreation(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::ArrayAccess(field_0) => AstNodeKind::ArrayAccess(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Update(field_0) => AstNodeKind::UpdateExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Unary(field_0) => AstNodeKind::UnaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Cast(field_0) => AstNodeKind::CastExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Instanceof(field_0) => AstNodeKind::InstanceofExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Binary(field_0) => AstNodeKind::BinaryExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Assignment(field_0) => AstNodeKind::AssignmentExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Conditional(field_0) => AstNodeKind::ConditionalExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Switch(field_0) => AstNodeKind::SwitchExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Parenthesized(field_0) => AstNodeKind::ParenthesizedExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::Lambda(field_0) => AstNodeKind::LambdaExpression(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            ElementValue::MethodReference(field_0) => AstNodeKind::MethodReference(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
        }
    }
    pub fn as_annotation(&self) -> Option<AstNode<'parent, 'a, Annotation<'a>>> {
        match self.inner {
            ElementValue::Annotation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<AstNode<'parent, 'a, ElementValueArrayInitializer<'a>>> {
        match self.inner {
            ElementValue::Array(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_int_literal(&self) -> Option<AstNode<'parent, 'a, IntLiteral<'a>>> {
        match self.inner {
            ElementValue::IntLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_float_literal(&self) -> Option<AstNode<'parent, 'a, FloatLiteral<'a>>> {
        match self.inner {
            ElementValue::FloatLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_char_literal(&self) -> Option<AstNode<'parent, 'a, CharLiteral<'a>>> {
        match self.inner {
            ElementValue::CharLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_string_literal(&self) -> Option<AstNode<'parent, 'a, StringLiteral<'a>>> {
        match self.inner {
            ElementValue::StringLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_boolean_literal(&self) -> Option<AstNode<'parent, 'a, BooleanLiteral>> {
        match self.inner {
            ElementValue::BooleanLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_null_literal(&self) -> Option<AstNode<'parent, 'a, NullLiteral>> {
        match self.inner {
            ElementValue::NullLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_this(&self) -> Option<AstNode<'parent, 'a, ThisExpression>> {
        match self.inner {
            ElementValue::This(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_super(&self) -> Option<AstNode<'parent, 'a, Super>> {
        match self.inner {
            ElementValue::Super(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_this(&self) -> Option<AstNode<'parent, 'a, QualifiedThis<'a>>> {
        match self.inner {
            ElementValue::QualifiedThis(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_super(&self) -> Option<AstNode<'parent, 'a, QualifiedSuper<'a>>> {
        match self.inner {
            ElementValue::QualifiedSuper(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<AstNode<'parent, 'a, IdentifierReference<'a>>> {
        match self.inner {
            ElementValue::Identifier(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<AstNode<'parent, 'a, FieldAccess<'a>>> {
        match self.inner {
            ElementValue::FieldAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_call(&self) -> Option<AstNode<'parent, 'a, MethodCall<'a>>> {
        match self.inner {
            ElementValue::MethodCall(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_literal(&self) -> Option<AstNode<'parent, 'a, ClassLiteral<'a>>> {
        match self.inner {
            ElementValue::ClassLiteral(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, ClassInstanceCreation<'a>>> {
        match self.inner {
            ElementValue::ClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_qualified_class_instance_creation(
        &self,
    ) -> Option<AstNode<'parent, 'a, QualifiedClassInstanceCreation<'a>>> {
        match self.inner {
            ElementValue::QualifiedClassInstanceCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_creation(&self) -> Option<AstNode<'parent, 'a, ArrayCreation<'a>>> {
        match self.inner {
            ElementValue::ArrayCreation(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_array_access(&self) -> Option<AstNode<'parent, 'a, ArrayAccess<'a>>> {
        match self.inner {
            ElementValue::ArrayAccess(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_update(&self) -> Option<AstNode<'parent, 'a, UpdateExpression<'a>>> {
        match self.inner {
            ElementValue::Update(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_unary(&self) -> Option<AstNode<'parent, 'a, UnaryExpression<'a>>> {
        match self.inner {
            ElementValue::Unary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_cast(&self) -> Option<AstNode<'parent, 'a, CastExpression<'a>>> {
        match self.inner {
            ElementValue::Cast(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_instanceof(&self) -> Option<AstNode<'parent, 'a, InstanceofExpression<'a>>> {
        match self.inner {
            ElementValue::Instanceof(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_binary(&self) -> Option<AstNode<'parent, 'a, BinaryExpression<'a>>> {
        match self.inner {
            ElementValue::Binary(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_assignment(&self) -> Option<AstNode<'parent, 'a, AssignmentExpression<'a>>> {
        match self.inner {
            ElementValue::Assignment(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_conditional(&self) -> Option<AstNode<'parent, 'a, ConditionalExpression<'a>>> {
        match self.inner {
            ElementValue::Conditional(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_switch(&self) -> Option<AstNode<'parent, 'a, SwitchExpression<'a>>> {
        match self.inner {
            ElementValue::Switch(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_parenthesized(&self) -> Option<AstNode<'parent, 'a, ParenthesizedExpression<'a>>> {
        match self.inner {
            ElementValue::Parenthesized(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_lambda(&self) -> Option<AstNode<'parent, 'a, LambdaExpression<'a>>> {
        match self.inner {
            ElementValue::Lambda(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_method_reference(&self) -> Option<AstNode<'parent, 'a, MethodReference<'a>>> {
        match self.inner {
            ElementValue::MethodReference(field_0) => Some(AstNode {
                inner: field_0.as_ref(),
                parent: self.parent,
                following_span_start: self.following_span_start,
            }),
            _ => None,
        }
    }
    pub fn as_expression(&self) -> Option<AstNode<'parent, 'a, Expression<'a>>> {
        self.inner.as_expression().map(|inner| AstNode {
            inner,
            parent: self.parent,
            following_span_start: self.following_span_start,
        })
    }
}
impl<'a> AstNode<'_, 'a, ElementValueArrayInitializer<'a>> {
    pub fn as_ast_nodes(&self) -> AstNodes<'_, 'a> {
        AstNodes::ElementValueArrayInitializer(self)
    }
    pub fn values(&self) -> AstNode<'_, 'a, Vec<'a, ElementValue<'a>>> {
        AstNode {
            inner: &self.inner.values,
            parent: self.as_ast_nodes(),
            following_span_start: self.following_span_start,
        }
    }
}
