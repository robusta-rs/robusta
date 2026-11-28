// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/get_span.rs`.

use robusta_span::{GetSpan, Span};

use crate::ast::java::annotations::*;
use crate::ast::java::declarations::*;
use crate::ast::java::expressions::*;
use crate::ast::java::names::*;
use crate::ast::java::statements::*;
use crate::ast::java::types::*;
use crate::ast::literal::*;

impl GetSpan for CompilationUnit<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for CompilationUnitContent<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Ordinary(it) => GetSpan::span(it),
            Self::Compact(it) => GetSpan::span(it),
            Self::Modular(it) => GetSpan::span(it),
        }
    }
}

impl GetSpan for OrdinaryCompilationUnit<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ModularCompilationUnit<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for PackageDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ImportDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for CompactCompilationUnit<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ModuleDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ModuleDirective<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Requires(it) => GetSpan::span(it),
            Self::Package(it) => GetSpan::span(it),
            Self::Uses(it) => GetSpan::span(it),
            Self::Provides(it) => GetSpan::span(it),
        }
    }
}

impl GetSpan for RequiresDirective<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for PackageDirective<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for UsesDirective<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ProvidesDirective<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TypeDeclaration<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Empty(it) => GetSpan::span(&**it),
            Self::AnnotationInterface(it) => GetSpan::span(&**it),
            Self::Class(it) => GetSpan::span(&**it),
            Self::Interface(it) => GetSpan::span(&**it),
            Self::Enum(it) => GetSpan::span(&**it),
            Self::Record(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for LocalClassOrInterfaceDeclaration<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Class(it) => GetSpan::span(&**it),
            Self::Interface(it) => GetSpan::span(&**it),
            Self::Enum(it) => GetSpan::span(&**it),
            Self::Record(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for ClassDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ClassBody<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for InterfaceDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for InterfaceBody<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for EnumDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for EnumBody<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for EnumConstant<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for RecordDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for RecordHeader<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for RecordComponent<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for AnnotationInterfaceDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for AnnotationInterfaceBody<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for AnnotationInterfaceMemberDeclaration<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Element(it) => GetSpan::span(&**it),
            Self::Field(it) => GetSpan::span(&**it),
            Self::Empty(it) => GetSpan::span(&**it),
            Self::AnnotationInterface(it) => GetSpan::span(&**it),
            Self::Class(it) => GetSpan::span(&**it),
            Self::Interface(it) => GetSpan::span(&**it),
            Self::Enum(it) => GetSpan::span(&**it),
            Self::Record(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for AnnotationInterfaceElementDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ClassMember<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Field(it) => GetSpan::span(&**it),
            Self::Method(it) => GetSpan::span(&**it),
            Self::Constructor(it) => GetSpan::span(&**it),
            Self::CompactConstructor(it) => GetSpan::span(&**it),
            Self::Initializer(it) => GetSpan::span(&**it),
            Self::Empty(it) => GetSpan::span(&**it),
            Self::AnnotationInterface(it) => GetSpan::span(&**it),
            Self::Class(it) => GetSpan::span(&**it),
            Self::Interface(it) => GetSpan::span(&**it),
            Self::Enum(it) => GetSpan::span(&**it),
            Self::Record(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for InterfaceMember<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Constant(it) => GetSpan::span(&**it),
            Self::Method(it) => GetSpan::span(&**it),
            Self::Empty(it) => GetSpan::span(&**it),
            Self::AnnotationInterface(it) => GetSpan::span(&**it),
            Self::Class(it) => GetSpan::span(&**it),
            Self::Interface(it) => GetSpan::span(&**it),
            Self::Enum(it) => GetSpan::span(&**it),
            Self::Record(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for InitializerBlock<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for FieldDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for MethodDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ConstructorDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for CompactConstructorDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ReceiverParameter<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for FormalParameters<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for FormalParameter<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Block<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Statement<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Block(it) => GetSpan::span(&**it),
            Self::Empty(it) => GetSpan::span(&**it),
            Self::ConstructorInvocation(it) => GetSpan::span(&**it),
            Self::LocalVariable(it) => GetSpan::span(&**it),
            Self::Expression(it) => GetSpan::span(&**it),
            Self::If(it) => GetSpan::span(&**it),
            Self::Switch(it) => GetSpan::span(&**it),
            Self::While(it) => GetSpan::span(&**it),
            Self::Do(it) => GetSpan::span(&**it),
            Self::For(it) => GetSpan::span(&**it),
            Self::EnhancedFor(it) => GetSpan::span(&**it),
            Self::Break(it) => GetSpan::span(&**it),
            Self::Continue(it) => GetSpan::span(&**it),
            Self::Throw(it) => GetSpan::span(&**it),
            Self::Assert(it) => GetSpan::span(&**it),
            Self::Try(it) => GetSpan::span(&**it),
            Self::Synchronized(it) => GetSpan::span(&**it),
            Self::Return(it) => GetSpan::span(&**it),
            Self::Yield(it) => GetSpan::span(&**it),
            Self::Labeled(it) => GetSpan::span(&**it),
            Self::Class(it) => GetSpan::span(&**it),
            Self::Interface(it) => GetSpan::span(&**it),
            Self::Enum(it) => GetSpan::span(&**it),
            Self::Record(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for EmptyStatement {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for IfStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for SwitchStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for SwitchItem<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Rule(it) => GetSpan::span(&**it),
            Self::Case(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for SwitchRule<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for SwitchRuleBody<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Block(it) => GetSpan::span(&**it),
            Self::Throw(it) => GetSpan::span(&**it),
            Self::IntLiteral(it) => GetSpan::span(&**it),
            Self::FloatLiteral(it) => GetSpan::span(&**it),
            Self::CharLiteral(it) => GetSpan::span(&**it),
            Self::StringLiteral(it) => GetSpan::span(&**it),
            Self::BooleanLiteral(it) => GetSpan::span(&**it),
            Self::NullLiteral(it) => GetSpan::span(&**it),
            Self::This(it) => GetSpan::span(&**it),
            Self::Super(it) => GetSpan::span(&**it),
            Self::QualifiedThis(it) => GetSpan::span(&**it),
            Self::QualifiedSuper(it) => GetSpan::span(&**it),
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::FieldAccess(it) => GetSpan::span(&**it),
            Self::MethodCall(it) => GetSpan::span(&**it),
            Self::ClassLiteral(it) => GetSpan::span(&**it),
            Self::ClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::QualifiedClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::ArrayCreation(it) => GetSpan::span(&**it),
            Self::ArrayAccess(it) => GetSpan::span(&**it),
            Self::Update(it) => GetSpan::span(&**it),
            Self::Unary(it) => GetSpan::span(&**it),
            Self::Cast(it) => GetSpan::span(&**it),
            Self::Instanceof(it) => GetSpan::span(&**it),
            Self::Binary(it) => GetSpan::span(&**it),
            Self::Assignment(it) => GetSpan::span(&**it),
            Self::Conditional(it) => GetSpan::span(&**it),
            Self::Switch(it) => GetSpan::span(&**it),
            Self::Parenthesized(it) => GetSpan::span(&**it),
            Self::Lambda(it) => GetSpan::span(&**it),
            Self::MethodReference(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for SwitchCase<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for SwitchLabel<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Case(it) => GetSpan::span(&**it),
            Self::Pattern(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for CaseLabel<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for CasePatternLabel<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for WhileStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for DoStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ForStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ForInit<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Declaration(it) => GetSpan::span(&**it),
            Self::Expressions(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for StatementExpressionList<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for EnhancedForStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for BreakStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ContinueStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ThrowStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for AssertStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TryStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Resource<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Declaration(it) => GetSpan::span(&**it),
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::Field(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for ResourceAccess<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::Field(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for CatchClause<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for CatchParameter<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for CatchType<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Union(it) => GetSpan::span(&**it),
            Self::Simple(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for UnionType<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for SynchronizedStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for LabeledStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ConstructorInvocation<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ExpressionStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for LocalVariableDeclaration<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ReturnStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for YieldStatement<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for VariableDeclarator<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for VariableInitializer<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Array(it) => GetSpan::span(&**it),
            Self::IntLiteral(it) => GetSpan::span(&**it),
            Self::FloatLiteral(it) => GetSpan::span(&**it),
            Self::CharLiteral(it) => GetSpan::span(&**it),
            Self::StringLiteral(it) => GetSpan::span(&**it),
            Self::BooleanLiteral(it) => GetSpan::span(&**it),
            Self::NullLiteral(it) => GetSpan::span(&**it),
            Self::This(it) => GetSpan::span(&**it),
            Self::Super(it) => GetSpan::span(&**it),
            Self::QualifiedThis(it) => GetSpan::span(&**it),
            Self::QualifiedSuper(it) => GetSpan::span(&**it),
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::FieldAccess(it) => GetSpan::span(&**it),
            Self::MethodCall(it) => GetSpan::span(&**it),
            Self::ClassLiteral(it) => GetSpan::span(&**it),
            Self::ClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::QualifiedClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::ArrayCreation(it) => GetSpan::span(&**it),
            Self::ArrayAccess(it) => GetSpan::span(&**it),
            Self::Update(it) => GetSpan::span(&**it),
            Self::Unary(it) => GetSpan::span(&**it),
            Self::Cast(it) => GetSpan::span(&**it),
            Self::Instanceof(it) => GetSpan::span(&**it),
            Self::Binary(it) => GetSpan::span(&**it),
            Self::Assignment(it) => GetSpan::span(&**it),
            Self::Conditional(it) => GetSpan::span(&**it),
            Self::Switch(it) => GetSpan::span(&**it),
            Self::Parenthesized(it) => GetSpan::span(&**it),
            Self::Lambda(it) => GetSpan::span(&**it),
            Self::MethodReference(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for ArrayInitializer<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Expression<'_> {
    fn span(&self) -> Span {
        match self {
            Self::IntLiteral(it) => GetSpan::span(&**it),
            Self::FloatLiteral(it) => GetSpan::span(&**it),
            Self::CharLiteral(it) => GetSpan::span(&**it),
            Self::StringLiteral(it) => GetSpan::span(&**it),
            Self::BooleanLiteral(it) => GetSpan::span(&**it),
            Self::NullLiteral(it) => GetSpan::span(&**it),
            Self::This(it) => GetSpan::span(&**it),
            Self::Super(it) => GetSpan::span(&**it),
            Self::QualifiedThis(it) => GetSpan::span(&**it),
            Self::QualifiedSuper(it) => GetSpan::span(&**it),
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::FieldAccess(it) => GetSpan::span(&**it),
            Self::MethodCall(it) => GetSpan::span(&**it),
            Self::ClassLiteral(it) => GetSpan::span(&**it),
            Self::ClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::QualifiedClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::ArrayCreation(it) => GetSpan::span(&**it),
            Self::ArrayAccess(it) => GetSpan::span(&**it),
            Self::Update(it) => GetSpan::span(&**it),
            Self::Unary(it) => GetSpan::span(&**it),
            Self::Cast(it) => GetSpan::span(&**it),
            Self::Instanceof(it) => GetSpan::span(&**it),
            Self::Binary(it) => GetSpan::span(&**it),
            Self::Assignment(it) => GetSpan::span(&**it),
            Self::Conditional(it) => GetSpan::span(&**it),
            Self::Switch(it) => GetSpan::span(&**it),
            Self::Parenthesized(it) => GetSpan::span(&**it),
            Self::Lambda(it) => GetSpan::span(&**it),
            Self::MethodReference(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for FieldAccess<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for MethodCall<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Callee<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::Member(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for MethodMember<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ClassLiteral<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ThisExpression {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Super {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for QualifiedThis<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for QualifiedSuper<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ClassInstanceCreation<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for QualifiedClassInstanceCreation<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ArrayCreation<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ArrayDimension<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ArrayAccess<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for UpdateExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for UnaryExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for CastExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for InstanceofExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for InstanceofRhs<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Primitive(it) => GetSpan::span(&**it),
            Self::Array(it) => GetSpan::span(&**it),
            Self::Simple(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
            Self::TypePattern(it) => GetSpan::span(&**it),
            Self::RecordPattern(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for Pattern<'_> {
    fn span(&self) -> Span {
        match self {
            Self::TypePattern(it) => GetSpan::span(&**it),
            Self::RecordPattern(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for TypePattern<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for RecordPattern<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ComponentPattern<'_> {
    fn span(&self) -> Span {
        match self {
            Self::MatchAll(it) => GetSpan::span(&**it),
            Self::TypePattern(it) => GetSpan::span(&**it),
            Self::RecordPattern(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for MatchAllPattern {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for BinaryExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for AssignmentExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for AssignmentTarget<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::FieldAccess(it) => GetSpan::span(&**it),
            Self::ArrayAccess(it) => GetSpan::span(&**it),
            Self::Parenthesized(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for ConditionalExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for SwitchExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ParenthesizedExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for LambdaExpression<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for MethodReference<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for MethodReferenceTarget<'_> {
    fn span(&self) -> Span {
        match self {
            Self::IntLiteral(it) => GetSpan::span(&**it),
            Self::FloatLiteral(it) => GetSpan::span(&**it),
            Self::CharLiteral(it) => GetSpan::span(&**it),
            Self::StringLiteral(it) => GetSpan::span(&**it),
            Self::BooleanLiteral(it) => GetSpan::span(&**it),
            Self::NullLiteral(it) => GetSpan::span(&**it),
            Self::This(it) => GetSpan::span(&**it),
            Self::Super(it) => GetSpan::span(&**it),
            Self::QualifiedThis(it) => GetSpan::span(&**it),
            Self::QualifiedSuper(it) => GetSpan::span(&**it),
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::FieldAccess(it) => GetSpan::span(&**it),
            Self::MethodCall(it) => GetSpan::span(&**it),
            Self::ClassLiteral(it) => GetSpan::span(&**it),
            Self::ClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::QualifiedClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::ArrayCreation(it) => GetSpan::span(&**it),
            Self::ArrayAccess(it) => GetSpan::span(&**it),
            Self::Update(it) => GetSpan::span(&**it),
            Self::Unary(it) => GetSpan::span(&**it),
            Self::Cast(it) => GetSpan::span(&**it),
            Self::Instanceof(it) => GetSpan::span(&**it),
            Self::Binary(it) => GetSpan::span(&**it),
            Self::Assignment(it) => GetSpan::span(&**it),
            Self::Conditional(it) => GetSpan::span(&**it),
            Self::Switch(it) => GetSpan::span(&**it),
            Self::Parenthesized(it) => GetSpan::span(&**it),
            Self::Lambda(it) => GetSpan::span(&**it),
            Self::MethodReference(it) => GetSpan::span(&**it),
            Self::Primitive(it) => GetSpan::span(&**it),
            Self::Array(it) => GetSpan::span(&**it),
            Self::Simple(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for TypeParameters<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TypeParameter<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for VariableType<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Var(it) => GetSpan::span(&**it),
            Self::Primitive(it) => GetSpan::span(&**it),
            Self::Array(it) => GetSpan::span(&**it),
            Self::Simple(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for VarType {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Type<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Primitive(it) => GetSpan::span(&**it),
            Self::Array(it) => GetSpan::span(&**it),
            Self::Simple(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for MethodResult<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Void(it) => GetSpan::span(&**it),
            Self::Primitive(it) => GetSpan::span(&**it),
            Self::Array(it) => GetSpan::span(&**it),
            Self::Simple(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for VoidType {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for PrimitiveType<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ClassType<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Simple(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for SimpleClassType<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for QualifiedClassType<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ArrayType<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ArrayTypeDimension<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TypeArguments<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TypeArgument<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Wildcard(it) => GetSpan::span(&**it),
            Self::Primitive(it) => GetSpan::span(&**it),
            Self::Array(it) => GetSpan::span(&**it),
            Self::Simple(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for WildcardTypeArgument<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for WildcardBound<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for IntLiteral<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for FloatLiteral<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for CharLiteral<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for StringLiteral<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for BooleanLiteral {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for NullLiteral {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for IdentifierName<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for IdentifierReference<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for BindingIdentifier<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for LabelIdentifier<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Name<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for QualifiedName<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TypeName<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::Qualified(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for QualifiedTypeName<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Modifier<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Keyword(it) => GetSpan::span(it),
            Self::Annotation(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for KeywordModifier {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Annotation<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for AnnotationArgument<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Pair(it) => GetSpan::span(&**it),
            Self::Annotation(it) => GetSpan::span(&**it),
            Self::Array(it) => GetSpan::span(&**it),
            Self::IntLiteral(it) => GetSpan::span(&**it),
            Self::FloatLiteral(it) => GetSpan::span(&**it),
            Self::CharLiteral(it) => GetSpan::span(&**it),
            Self::StringLiteral(it) => GetSpan::span(&**it),
            Self::BooleanLiteral(it) => GetSpan::span(&**it),
            Self::NullLiteral(it) => GetSpan::span(&**it),
            Self::This(it) => GetSpan::span(&**it),
            Self::Super(it) => GetSpan::span(&**it),
            Self::QualifiedThis(it) => GetSpan::span(&**it),
            Self::QualifiedSuper(it) => GetSpan::span(&**it),
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::FieldAccess(it) => GetSpan::span(&**it),
            Self::MethodCall(it) => GetSpan::span(&**it),
            Self::ClassLiteral(it) => GetSpan::span(&**it),
            Self::ClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::QualifiedClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::ArrayCreation(it) => GetSpan::span(&**it),
            Self::ArrayAccess(it) => GetSpan::span(&**it),
            Self::Update(it) => GetSpan::span(&**it),
            Self::Unary(it) => GetSpan::span(&**it),
            Self::Cast(it) => GetSpan::span(&**it),
            Self::Instanceof(it) => GetSpan::span(&**it),
            Self::Binary(it) => GetSpan::span(&**it),
            Self::Assignment(it) => GetSpan::span(&**it),
            Self::Conditional(it) => GetSpan::span(&**it),
            Self::Switch(it) => GetSpan::span(&**it),
            Self::Parenthesized(it) => GetSpan::span(&**it),
            Self::Lambda(it) => GetSpan::span(&**it),
            Self::MethodReference(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for ElementValuePair<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ElementValue<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Annotation(it) => GetSpan::span(&**it),
            Self::Array(it) => GetSpan::span(&**it),
            Self::IntLiteral(it) => GetSpan::span(&**it),
            Self::FloatLiteral(it) => GetSpan::span(&**it),
            Self::CharLiteral(it) => GetSpan::span(&**it),
            Self::StringLiteral(it) => GetSpan::span(&**it),
            Self::BooleanLiteral(it) => GetSpan::span(&**it),
            Self::NullLiteral(it) => GetSpan::span(&**it),
            Self::This(it) => GetSpan::span(&**it),
            Self::Super(it) => GetSpan::span(&**it),
            Self::QualifiedThis(it) => GetSpan::span(&**it),
            Self::QualifiedSuper(it) => GetSpan::span(&**it),
            Self::Identifier(it) => GetSpan::span(&**it),
            Self::FieldAccess(it) => GetSpan::span(&**it),
            Self::MethodCall(it) => GetSpan::span(&**it),
            Self::ClassLiteral(it) => GetSpan::span(&**it),
            Self::ClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::QualifiedClassInstanceCreation(it) => GetSpan::span(&**it),
            Self::ArrayCreation(it) => GetSpan::span(&**it),
            Self::ArrayAccess(it) => GetSpan::span(&**it),
            Self::Update(it) => GetSpan::span(&**it),
            Self::Unary(it) => GetSpan::span(&**it),
            Self::Cast(it) => GetSpan::span(&**it),
            Self::Instanceof(it) => GetSpan::span(&**it),
            Self::Binary(it) => GetSpan::span(&**it),
            Self::Assignment(it) => GetSpan::span(&**it),
            Self::Conditional(it) => GetSpan::span(&**it),
            Self::Switch(it) => GetSpan::span(&**it),
            Self::Parenthesized(it) => GetSpan::span(&**it),
            Self::Lambda(it) => GetSpan::span(&**it),
            Self::MethodReference(it) => GetSpan::span(&**it),
        }
    }
}

impl GetSpan for ElementValueArrayInitializer<'_> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
