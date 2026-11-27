// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/content_eq.rs`.

use robusta_span::ContentEq;

use crate::ast::java::annotations::*;
use crate::ast::java::declarations::*;
use crate::ast::java::expressions::*;
use crate::ast::java::names::*;
use crate::ast::java::statements::*;
use crate::ast::java::types::*;
use crate::ast::literal::*;

impl ContentEq for CompilationUnit<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.content, &other.content)
    }
}

impl ContentEq for CompilationUnitContent<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ordinary(a), Self::Ordinary(b)) => a.content_eq(b),
            (Self::Compact(a), Self::Compact(b)) => a.content_eq(b),
            (Self::Modular(a), Self::Modular(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for OrdinaryCompilationUnit<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.package, &other.package)
            && ContentEq::content_eq(&self.imports, &other.imports)
            && ContentEq::content_eq(&self.types, &other.types)
    }
}

impl ContentEq for ModularCompilationUnit<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.imports, &other.imports)
            && ContentEq::content_eq(&self.module, &other.module)
    }
}

impl ContentEq for PackageDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.annotations, &other.annotations)
            && ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for ImportDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.kind, &other.kind)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.local, &other.local)
    }
}

impl ContentEq for ImportKind {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl ContentEq for CompactCompilationUnit<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.imports, &other.imports)
            && ContentEq::content_eq(&self.members, &other.members)
    }
}

impl ContentEq for ModuleDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.annotations, &other.annotations)
            && ContentEq::content_eq(&self.is_open, &other.is_open)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.directives, &other.directives)
    }
}

impl ContentEq for ModuleDirective<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Requires(a), Self::Requires(b)) => a.content_eq(b),
            (Self::Package(a), Self::Package(b)) => a.content_eq(b),
            (Self::Uses(a), Self::Uses(b)) => a.content_eq(b),
            (Self::Provides(a), Self::Provides(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for RequiresDirective<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for PackageDirective<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.kind, &other.kind)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.targets, &other.targets)
    }
}

impl ContentEq for PackageDirectiveKind {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl ContentEq for UsesDirective<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for ProvidesDirective<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.with, &other.with)
    }
}

impl ContentEq for TypeDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty(a), Self::Empty(b)) => a.content_eq(b),
            (Self::AnnotationInterface(a), Self::AnnotationInterface(b)) => a.content_eq(b),
            (Self::Class(a), Self::Class(b)) => a.content_eq(b),
            (Self::Interface(a), Self::Interface(b)) => a.content_eq(b),
            (Self::Enum(a), Self::Enum(b)) => a.content_eq(b),
            (Self::Record(a), Self::Record(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for LocalClassOrInterfaceDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Class(a), Self::Class(b)) => a.content_eq(b),
            (Self::Interface(a), Self::Interface(b)) => a.content_eq(b),
            (Self::Enum(a), Self::Enum(b)) => a.content_eq(b),
            (Self::Record(a), Self::Record(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for EmptyDeclaration {
    fn content_eq(&self, _: &Self) -> bool {
        true
    }
}

impl ContentEq for ClassDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.type_params, &other.type_params)
            && ContentEq::content_eq(&self.superclass, &other.superclass)
            && ContentEq::content_eq(&self.interfaces, &other.interfaces)
            && ContentEq::content_eq(&self.permits, &other.permits)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for ClassBody<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.members, &other.members)
    }
}

impl ContentEq for InterfaceDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.type_params, &other.type_params)
            && ContentEq::content_eq(&self.extends, &other.extends)
            && ContentEq::content_eq(&self.permits, &other.permits)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for InterfaceBody<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.members, &other.members)
    }
}

impl ContentEq for EnumDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.interfaces, &other.interfaces)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for EnumBody<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.constants, &other.constants)
            && ContentEq::content_eq(&self.has_body_semicolon, &other.has_body_semicolon)
            && ContentEq::content_eq(&self.members, &other.members)
    }
}

impl ContentEq for EnumConstant<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.args, &other.args)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for RecordDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.type_params, &other.type_params)
            && ContentEq::content_eq(&self.header, &other.header)
            && ContentEq::content_eq(&self.interfaces, &other.interfaces)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for RecordHeader<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.components, &other.components)
    }
}

impl ContentEq for RecordComponent<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.varargs, &other.varargs)
            && ContentEq::content_eq(&self.varargs_annotations, &other.varargs_annotations)
            && ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for AnnotationInterfaceDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for AnnotationInterfaceBody<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.members, &other.members)
    }
}

impl ContentEq for AnnotationInterfaceMemberDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Element(a), Self::Element(b)) => a.content_eq(b),
            (Self::Field(a), Self::Field(b)) => a.content_eq(b),
            (Self::Empty(a), Self::Empty(b)) => a.content_eq(b),
            (Self::AnnotationInterface(a), Self::AnnotationInterface(b)) => a.content_eq(b),
            (Self::Class(a), Self::Class(b)) => a.content_eq(b),
            (Self::Interface(a), Self::Interface(b)) => a.content_eq(b),
            (Self::Enum(a), Self::Enum(b)) => a.content_eq(b),
            (Self::Record(a), Self::Record(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for AnnotationInterfaceElementDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.post_paren_dimensions, &other.post_paren_dimensions)
            && ContentEq::content_eq(&self.default_value, &other.default_value)
    }
}

impl ContentEq for ClassMember<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Field(a), Self::Field(b)) => a.content_eq(b),
            (Self::Method(a), Self::Method(b)) => a.content_eq(b),
            (Self::Constructor(a), Self::Constructor(b)) => a.content_eq(b),
            (Self::CompactConstructor(a), Self::CompactConstructor(b)) => a.content_eq(b),
            (Self::Initializer(a), Self::Initializer(b)) => a.content_eq(b),
            (Self::Empty(a), Self::Empty(b)) => a.content_eq(b),
            (Self::AnnotationInterface(a), Self::AnnotationInterface(b)) => a.content_eq(b),
            (Self::Class(a), Self::Class(b)) => a.content_eq(b),
            (Self::Interface(a), Self::Interface(b)) => a.content_eq(b),
            (Self::Enum(a), Self::Enum(b)) => a.content_eq(b),
            (Self::Record(a), Self::Record(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for InterfaceMember<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Constant(a), Self::Constant(b)) => a.content_eq(b),
            (Self::Method(a), Self::Method(b)) => a.content_eq(b),
            (Self::Empty(a), Self::Empty(b)) => a.content_eq(b),
            (Self::AnnotationInterface(a), Self::AnnotationInterface(b)) => a.content_eq(b),
            (Self::Class(a), Self::Class(b)) => a.content_eq(b),
            (Self::Interface(a), Self::Interface(b)) => a.content_eq(b),
            (Self::Enum(a), Self::Enum(b)) => a.content_eq(b),
            (Self::Record(a), Self::Record(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for InitializerBlock<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.is_static, &other.is_static)
            && ContentEq::content_eq(&self.block, &other.block)
    }
}

impl ContentEq for FieldDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.declarators, &other.declarators)
    }
}

impl ContentEq for MethodDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.type_params, &other.type_params)
            && ContentEq::content_eq(&self.return_type, &other.return_type)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.params, &other.params)
            && ContentEq::content_eq(&self.post_paren_dimensions, &other.post_paren_dimensions)
            && ContentEq::content_eq(&self.throws, &other.throws)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for ConstructorDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.type_params, &other.type_params)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.params, &other.params)
            && ContentEq::content_eq(&self.throws, &other.throws)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for CompactConstructorDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for ReceiverParameter<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.qualifier, &other.qualifier)
    }
}

impl ContentEq for FormalParameters<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.receiver, &other.receiver)
            && ContentEq::content_eq(&self.items, &other.items)
    }
}

impl ContentEq for FormalParameter<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.varargs, &other.varargs)
            && ContentEq::content_eq(&self.varargs_annotations, &other.varargs_annotations)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.dims, &other.dims)
    }
}

impl ContentEq for Block<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.statements, &other.statements)
    }
}

impl ContentEq for Statement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Block(a), Self::Block(b)) => a.content_eq(b),
            (Self::Empty(a), Self::Empty(b)) => a.content_eq(b),
            (Self::ConstructorInvocation(a), Self::ConstructorInvocation(b)) => a.content_eq(b),
            (Self::LocalVariable(a), Self::LocalVariable(b)) => a.content_eq(b),
            (Self::Expression(a), Self::Expression(b)) => a.content_eq(b),
            (Self::If(a), Self::If(b)) => a.content_eq(b),
            (Self::Switch(a), Self::Switch(b)) => a.content_eq(b),
            (Self::While(a), Self::While(b)) => a.content_eq(b),
            (Self::Do(a), Self::Do(b)) => a.content_eq(b),
            (Self::For(a), Self::For(b)) => a.content_eq(b),
            (Self::EnhancedFor(a), Self::EnhancedFor(b)) => a.content_eq(b),
            (Self::Break(a), Self::Break(b)) => a.content_eq(b),
            (Self::Continue(a), Self::Continue(b)) => a.content_eq(b),
            (Self::Throw(a), Self::Throw(b)) => a.content_eq(b),
            (Self::Assert(a), Self::Assert(b)) => a.content_eq(b),
            (Self::Try(a), Self::Try(b)) => a.content_eq(b),
            (Self::Synchronized(a), Self::Synchronized(b)) => a.content_eq(b),
            (Self::Return(a), Self::Return(b)) => a.content_eq(b),
            (Self::Yield(a), Self::Yield(b)) => a.content_eq(b),
            (Self::Labeled(a), Self::Labeled(b)) => a.content_eq(b),
            (Self::Class(a), Self::Class(b)) => a.content_eq(b),
            (Self::Interface(a), Self::Interface(b)) => a.content_eq(b),
            (Self::Enum(a), Self::Enum(b)) => a.content_eq(b),
            (Self::Record(a), Self::Record(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for EmptyStatement {
    fn content_eq(&self, _: &Self) -> bool {
        true
    }
}

impl ContentEq for IfStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.condition, &other.condition)
            && ContentEq::content_eq(&self.then_branch, &other.then_branch)
            && ContentEq::content_eq(&self.else_branch, &other.else_branch)
    }
}

impl ContentEq for SwitchStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.selector, &other.selector)
            && ContentEq::content_eq(&self.cases, &other.cases)
    }
}

impl ContentEq for SwitchItem<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rule(a), Self::Rule(b)) => a.content_eq(b),
            (Self::Case(a), Self::Case(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for SwitchRule<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.label, &other.label)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for SwitchRuleBody<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Block(a), Self::Block(b)) => a.content_eq(b),
            (Self::Throw(a), Self::Throw(b)) => a.content_eq(b),
            (Self::IntLiteral(a), Self::IntLiteral(b)) => a.content_eq(b),
            (Self::FloatLiteral(a), Self::FloatLiteral(b)) => a.content_eq(b),
            (Self::CharLiteral(a), Self::CharLiteral(b)) => a.content_eq(b),
            (Self::StringLiteral(a), Self::StringLiteral(b)) => a.content_eq(b),
            (Self::BooleanLiteral(a), Self::BooleanLiteral(b)) => a.content_eq(b),
            (Self::NullLiteral(a), Self::NullLiteral(b)) => a.content_eq(b),
            (Self::This(a), Self::This(b)) => a.content_eq(b),
            (Self::Super(a), Self::Super(b)) => a.content_eq(b),
            (Self::QualifiedThis(a), Self::QualifiedThis(b)) => a.content_eq(b),
            (Self::QualifiedSuper(a), Self::QualifiedSuper(b)) => a.content_eq(b),
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::FieldAccess(a), Self::FieldAccess(b)) => a.content_eq(b),
            (Self::MethodCall(a), Self::MethodCall(b)) => a.content_eq(b),
            (Self::ClassLiteral(a), Self::ClassLiteral(b)) => a.content_eq(b),
            (Self::ClassInstanceCreation(a), Self::ClassInstanceCreation(b)) => a.content_eq(b),
            (Self::QualifiedClassInstanceCreation(a), Self::QualifiedClassInstanceCreation(b)) => {
                a.content_eq(b)
            }
            (Self::ArrayCreation(a), Self::ArrayCreation(b)) => a.content_eq(b),
            (Self::ArrayAccess(a), Self::ArrayAccess(b)) => a.content_eq(b),
            (Self::Update(a), Self::Update(b)) => a.content_eq(b),
            (Self::Unary(a), Self::Unary(b)) => a.content_eq(b),
            (Self::Cast(a), Self::Cast(b)) => a.content_eq(b),
            (Self::Instanceof(a), Self::Instanceof(b)) => a.content_eq(b),
            (Self::Binary(a), Self::Binary(b)) => a.content_eq(b),
            (Self::Assignment(a), Self::Assignment(b)) => a.content_eq(b),
            (Self::Conditional(a), Self::Conditional(b)) => a.content_eq(b),
            (Self::Switch(a), Self::Switch(b)) => a.content_eq(b),
            (Self::Parenthesized(a), Self::Parenthesized(b)) => a.content_eq(b),
            (Self::Lambda(a), Self::Lambda(b)) => a.content_eq(b),
            (Self::MethodReference(a), Self::MethodReference(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for SwitchCase<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.label, &other.label)
            && ContentEq::content_eq(&self.statements, &other.statements)
    }
}

impl ContentEq for SwitchLabel<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Case(a), Self::Case(b)) => a.content_eq(b),
            (Self::Pattern(a), Self::Pattern(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for CaseLabel<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.constants, &other.constants)
            && ContentEq::content_eq(&self.has_default, &other.has_default)
    }
}

impl ContentEq for CasePatternLabel<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.patterns, &other.patterns)
            && ContentEq::content_eq(&self.guard, &other.guard)
    }
}

impl ContentEq for WhileStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.condition, &other.condition)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for DoStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.body, &other.body)
            && ContentEq::content_eq(&self.condition, &other.condition)
    }
}

impl ContentEq for ForStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.init, &other.init)
            && ContentEq::content_eq(&self.condition, &other.condition)
            && ContentEq::content_eq(&self.update, &other.update)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for ForInit<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Declaration(a), Self::Declaration(b)) => a.content_eq(b),
            (Self::Expressions(a), Self::Expressions(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for StatementExpressionList<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.expressions, &other.expressions)
    }
}

impl ContentEq for EnhancedForStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.dims, &other.dims)
            && ContentEq::content_eq(&self.iterable, &other.iterable)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for BreakStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.label, &other.label)
    }
}

impl ContentEq for ContinueStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.label, &other.label)
    }
}

impl ContentEq for ThrowStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.value, &other.value)
    }
}

impl ContentEq for AssertStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.condition, &other.condition)
            && ContentEq::content_eq(&self.message, &other.message)
    }
}

impl ContentEq for TryStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.resources, &other.resources)
            && ContentEq::content_eq(
                &self.resources_trailing_semicolon,
                &other.resources_trailing_semicolon,
            )
            && ContentEq::content_eq(&self.block, &other.block)
            && ContentEq::content_eq(&self.catches, &other.catches)
            && ContentEq::content_eq(&self.finally_block, &other.finally_block)
    }
}

impl ContentEq for Resource<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Declaration(a), Self::Declaration(b)) => a.content_eq(b),
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::Field(a), Self::Field(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for ResourceAccess<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::Field(a), Self::Field(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for ResourceDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.init, &other.init)
    }
}

impl ContentEq for CatchClause<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.types, &other.types)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.block, &other.block)
    }
}

impl ContentEq for SynchronizedStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.lock, &other.lock)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for LabeledStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.label, &other.label)
            && ContentEq::content_eq(&self.statement, &other.statement)
    }
}

impl ContentEq for ConstructorInvocation<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.qualifier, &other.qualifier)
            && ContentEq::content_eq(&self.type_args, &other.type_args)
            && ContentEq::content_eq(&self.kind, &other.kind)
            && ContentEq::content_eq(&self.args, &other.args)
    }
}

impl ContentEq for ConstructorInvocationKind {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl ContentEq for ExpressionStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.expression, &other.expression)
    }
}

impl ContentEq for LocalVariableDeclaration<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.declarators, &other.declarators)
    }
}

impl ContentEq for ReturnStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.value, &other.value)
    }
}

impl ContentEq for YieldStatement<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.value, &other.value)
    }
}

impl ContentEq for VariableDeclarator<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.dims, &other.dims)
            && ContentEq::content_eq(&self.init, &other.init)
    }
}

impl ContentEq for VariableInitializer<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Array(a), Self::Array(b)) => a.content_eq(b),
            (Self::IntLiteral(a), Self::IntLiteral(b)) => a.content_eq(b),
            (Self::FloatLiteral(a), Self::FloatLiteral(b)) => a.content_eq(b),
            (Self::CharLiteral(a), Self::CharLiteral(b)) => a.content_eq(b),
            (Self::StringLiteral(a), Self::StringLiteral(b)) => a.content_eq(b),
            (Self::BooleanLiteral(a), Self::BooleanLiteral(b)) => a.content_eq(b),
            (Self::NullLiteral(a), Self::NullLiteral(b)) => a.content_eq(b),
            (Self::This(a), Self::This(b)) => a.content_eq(b),
            (Self::Super(a), Self::Super(b)) => a.content_eq(b),
            (Self::QualifiedThis(a), Self::QualifiedThis(b)) => a.content_eq(b),
            (Self::QualifiedSuper(a), Self::QualifiedSuper(b)) => a.content_eq(b),
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::FieldAccess(a), Self::FieldAccess(b)) => a.content_eq(b),
            (Self::MethodCall(a), Self::MethodCall(b)) => a.content_eq(b),
            (Self::ClassLiteral(a), Self::ClassLiteral(b)) => a.content_eq(b),
            (Self::ClassInstanceCreation(a), Self::ClassInstanceCreation(b)) => a.content_eq(b),
            (Self::QualifiedClassInstanceCreation(a), Self::QualifiedClassInstanceCreation(b)) => {
                a.content_eq(b)
            }
            (Self::ArrayCreation(a), Self::ArrayCreation(b)) => a.content_eq(b),
            (Self::ArrayAccess(a), Self::ArrayAccess(b)) => a.content_eq(b),
            (Self::Update(a), Self::Update(b)) => a.content_eq(b),
            (Self::Unary(a), Self::Unary(b)) => a.content_eq(b),
            (Self::Cast(a), Self::Cast(b)) => a.content_eq(b),
            (Self::Instanceof(a), Self::Instanceof(b)) => a.content_eq(b),
            (Self::Binary(a), Self::Binary(b)) => a.content_eq(b),
            (Self::Assignment(a), Self::Assignment(b)) => a.content_eq(b),
            (Self::Conditional(a), Self::Conditional(b)) => a.content_eq(b),
            (Self::Switch(a), Self::Switch(b)) => a.content_eq(b),
            (Self::Parenthesized(a), Self::Parenthesized(b)) => a.content_eq(b),
            (Self::Lambda(a), Self::Lambda(b)) => a.content_eq(b),
            (Self::MethodReference(a), Self::MethodReference(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for ArrayInitializer<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.elements, &other.elements)
    }
}

impl ContentEq for Expression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IntLiteral(a), Self::IntLiteral(b)) => a.content_eq(b),
            (Self::FloatLiteral(a), Self::FloatLiteral(b)) => a.content_eq(b),
            (Self::CharLiteral(a), Self::CharLiteral(b)) => a.content_eq(b),
            (Self::StringLiteral(a), Self::StringLiteral(b)) => a.content_eq(b),
            (Self::BooleanLiteral(a), Self::BooleanLiteral(b)) => a.content_eq(b),
            (Self::NullLiteral(a), Self::NullLiteral(b)) => a.content_eq(b),
            (Self::This(a), Self::This(b)) => a.content_eq(b),
            (Self::Super(a), Self::Super(b)) => a.content_eq(b),
            (Self::QualifiedThis(a), Self::QualifiedThis(b)) => a.content_eq(b),
            (Self::QualifiedSuper(a), Self::QualifiedSuper(b)) => a.content_eq(b),
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::FieldAccess(a), Self::FieldAccess(b)) => a.content_eq(b),
            (Self::MethodCall(a), Self::MethodCall(b)) => a.content_eq(b),
            (Self::ClassLiteral(a), Self::ClassLiteral(b)) => a.content_eq(b),
            (Self::ClassInstanceCreation(a), Self::ClassInstanceCreation(b)) => a.content_eq(b),
            (Self::QualifiedClassInstanceCreation(a), Self::QualifiedClassInstanceCreation(b)) => {
                a.content_eq(b)
            }
            (Self::ArrayCreation(a), Self::ArrayCreation(b)) => a.content_eq(b),
            (Self::ArrayAccess(a), Self::ArrayAccess(b)) => a.content_eq(b),
            (Self::Update(a), Self::Update(b)) => a.content_eq(b),
            (Self::Unary(a), Self::Unary(b)) => a.content_eq(b),
            (Self::Cast(a), Self::Cast(b)) => a.content_eq(b),
            (Self::Instanceof(a), Self::Instanceof(b)) => a.content_eq(b),
            (Self::Binary(a), Self::Binary(b)) => a.content_eq(b),
            (Self::Assignment(a), Self::Assignment(b)) => a.content_eq(b),
            (Self::Conditional(a), Self::Conditional(b)) => a.content_eq(b),
            (Self::Switch(a), Self::Switch(b)) => a.content_eq(b),
            (Self::Parenthesized(a), Self::Parenthesized(b)) => a.content_eq(b),
            (Self::Lambda(a), Self::Lambda(b)) => a.content_eq(b),
            (Self::MethodReference(a), Self::MethodReference(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for FieldAccess<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.target, &other.target)
            && ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for MethodCall<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.callee, &other.callee)
            && ContentEq::content_eq(&self.args, &other.args)
    }
}

impl ContentEq for Callee<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::Member(a), Self::Member(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for MethodMember<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.target, &other.target)
            && ContentEq::content_eq(&self.type_args, &other.type_args)
            && ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for ClassLiteral<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.ty, &other.ty)
    }
}

impl ContentEq for ThisExpression {
    fn content_eq(&self, _: &Self) -> bool {
        true
    }
}

impl ContentEq for Super {
    fn content_eq(&self, _: &Self) -> bool {
        true
    }
}

impl ContentEq for QualifiedThis<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.qualifier, &other.qualifier)
    }
}

impl ContentEq for QualifiedSuper<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.qualifier, &other.qualifier)
    }
}

impl ContentEq for ClassInstanceCreation<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.constructor_type_args, &other.constructor_type_args)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.args, &other.args)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for QualifiedClassInstanceCreation<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.qualifier, &other.qualifier)
            && ContentEq::content_eq(&self.constructor_type_args, &other.constructor_type_args)
            && ContentEq::content_eq(&self.annotations, &other.annotations)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.type_args, &other.type_args)
            && ContentEq::content_eq(&self.args, &other.args)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for ArrayCreation<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.element, &other.element)
            && ContentEq::content_eq(&self.dims, &other.dims)
            && ContentEq::content_eq(&self.init, &other.init)
    }
}

impl ContentEq for ArrayDimension<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.annotations, &other.annotations)
            && ContentEq::content_eq(&self.length, &other.length)
    }
}

impl ContentEq for ArrayAccess<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.array, &other.array)
            && ContentEq::content_eq(&self.index, &other.index)
    }
}

impl ContentEq for UpdateExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.op, &other.op)
            && ContentEq::content_eq(&self.prefix, &other.prefix)
            && ContentEq::content_eq(&self.target, &other.target)
    }
}

impl ContentEq for UnaryExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.op, &other.op)
            && ContentEq::content_eq(&self.operand, &other.operand)
    }
}

impl ContentEq for CastExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.additional_bounds, &other.additional_bounds)
            && ContentEq::content_eq(&self.operand, &other.operand)
    }
}

impl ContentEq for InstanceofExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.operand, &other.operand)
            && ContentEq::content_eq(&self.rhs, &other.rhs)
    }
}

impl ContentEq for InstanceofRhs<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primitive(a), Self::Primitive(b)) => a.content_eq(b),
            (Self::Array(a), Self::Array(b)) => a.content_eq(b),
            (Self::Simple(a), Self::Simple(b)) => a.content_eq(b),
            (Self::Qualified(a), Self::Qualified(b)) => a.content_eq(b),
            (Self::TypePattern(a), Self::TypePattern(b)) => a.content_eq(b),
            (Self::RecordPattern(a), Self::RecordPattern(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for Pattern<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TypePattern(a), Self::TypePattern(b)) => a.content_eq(b),
            (Self::RecordPattern(a), Self::RecordPattern(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for TypePattern<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.modifiers, &other.modifiers)
            && ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for RecordPattern<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.ty, &other.ty)
            && ContentEq::content_eq(&self.components, &other.components)
    }
}

impl ContentEq for ComponentPattern<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::MatchAll(a), Self::MatchAll(b)) => a.content_eq(b),
            (Self::TypePattern(a), Self::TypePattern(b)) => a.content_eq(b),
            (Self::RecordPattern(a), Self::RecordPattern(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for MatchAllPattern {
    fn content_eq(&self, _: &Self) -> bool {
        true
    }
}

impl ContentEq for BinaryExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.op, &other.op)
            && ContentEq::content_eq(&self.lhs, &other.lhs)
            && ContentEq::content_eq(&self.rhs, &other.rhs)
    }
}

impl ContentEq for AssignmentExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.target, &other.target)
            && ContentEq::content_eq(&self.op, &other.op)
            && ContentEq::content_eq(&self.value, &other.value)
    }
}

impl ContentEq for AssignmentTarget<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::FieldAccess(a), Self::FieldAccess(b)) => a.content_eq(b),
            (Self::ArrayAccess(a), Self::ArrayAccess(b)) => a.content_eq(b),
            (Self::Parenthesized(a), Self::Parenthesized(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for ConditionalExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.condition, &other.condition)
            && ContentEq::content_eq(&self.when_true, &other.when_true)
            && ContentEq::content_eq(&self.when_false, &other.when_false)
    }
}

impl ContentEq for SwitchExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.selector, &other.selector)
            && ContentEq::content_eq(&self.cases, &other.cases)
    }
}

impl ContentEq for ParenthesizedExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.expression, &other.expression)
    }
}

impl ContentEq for LambdaExpression<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.params, &other.params)
            && ContentEq::content_eq(&self.body, &other.body)
    }
}

impl ContentEq for LambdaBody<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Block(a), Self::Block(b)) => a.content_eq(b),
            (Self::IntLiteral(a), Self::IntLiteral(b)) => a.content_eq(b),
            (Self::FloatLiteral(a), Self::FloatLiteral(b)) => a.content_eq(b),
            (Self::CharLiteral(a), Self::CharLiteral(b)) => a.content_eq(b),
            (Self::StringLiteral(a), Self::StringLiteral(b)) => a.content_eq(b),
            (Self::BooleanLiteral(a), Self::BooleanLiteral(b)) => a.content_eq(b),
            (Self::NullLiteral(a), Self::NullLiteral(b)) => a.content_eq(b),
            (Self::This(a), Self::This(b)) => a.content_eq(b),
            (Self::Super(a), Self::Super(b)) => a.content_eq(b),
            (Self::QualifiedThis(a), Self::QualifiedThis(b)) => a.content_eq(b),
            (Self::QualifiedSuper(a), Self::QualifiedSuper(b)) => a.content_eq(b),
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::FieldAccess(a), Self::FieldAccess(b)) => a.content_eq(b),
            (Self::MethodCall(a), Self::MethodCall(b)) => a.content_eq(b),
            (Self::ClassLiteral(a), Self::ClassLiteral(b)) => a.content_eq(b),
            (Self::ClassInstanceCreation(a), Self::ClassInstanceCreation(b)) => a.content_eq(b),
            (Self::QualifiedClassInstanceCreation(a), Self::QualifiedClassInstanceCreation(b)) => {
                a.content_eq(b)
            }
            (Self::ArrayCreation(a), Self::ArrayCreation(b)) => a.content_eq(b),
            (Self::ArrayAccess(a), Self::ArrayAccess(b)) => a.content_eq(b),
            (Self::Update(a), Self::Update(b)) => a.content_eq(b),
            (Self::Unary(a), Self::Unary(b)) => a.content_eq(b),
            (Self::Cast(a), Self::Cast(b)) => a.content_eq(b),
            (Self::Instanceof(a), Self::Instanceof(b)) => a.content_eq(b),
            (Self::Binary(a), Self::Binary(b)) => a.content_eq(b),
            (Self::Assignment(a), Self::Assignment(b)) => a.content_eq(b),
            (Self::Conditional(a), Self::Conditional(b)) => a.content_eq(b),
            (Self::Switch(a), Self::Switch(b)) => a.content_eq(b),
            (Self::Parenthesized(a), Self::Parenthesized(b)) => a.content_eq(b),
            (Self::Lambda(a), Self::Lambda(b)) => a.content_eq(b),
            (Self::MethodReference(a), Self::MethodReference(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for MethodReference<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.target, &other.target)
            && ContentEq::content_eq(&self.type_args, &other.type_args)
            && ContentEq::content_eq(&self.method, &other.method)
    }
}

impl ContentEq for MethodReferenceTarget<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IntLiteral(a), Self::IntLiteral(b)) => a.content_eq(b),
            (Self::FloatLiteral(a), Self::FloatLiteral(b)) => a.content_eq(b),
            (Self::CharLiteral(a), Self::CharLiteral(b)) => a.content_eq(b),
            (Self::StringLiteral(a), Self::StringLiteral(b)) => a.content_eq(b),
            (Self::BooleanLiteral(a), Self::BooleanLiteral(b)) => a.content_eq(b),
            (Self::NullLiteral(a), Self::NullLiteral(b)) => a.content_eq(b),
            (Self::This(a), Self::This(b)) => a.content_eq(b),
            (Self::Super(a), Self::Super(b)) => a.content_eq(b),
            (Self::QualifiedThis(a), Self::QualifiedThis(b)) => a.content_eq(b),
            (Self::QualifiedSuper(a), Self::QualifiedSuper(b)) => a.content_eq(b),
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::FieldAccess(a), Self::FieldAccess(b)) => a.content_eq(b),
            (Self::MethodCall(a), Self::MethodCall(b)) => a.content_eq(b),
            (Self::ClassLiteral(a), Self::ClassLiteral(b)) => a.content_eq(b),
            (Self::ClassInstanceCreation(a), Self::ClassInstanceCreation(b)) => a.content_eq(b),
            (Self::QualifiedClassInstanceCreation(a), Self::QualifiedClassInstanceCreation(b)) => {
                a.content_eq(b)
            }
            (Self::ArrayCreation(a), Self::ArrayCreation(b)) => a.content_eq(b),
            (Self::ArrayAccess(a), Self::ArrayAccess(b)) => a.content_eq(b),
            (Self::Update(a), Self::Update(b)) => a.content_eq(b),
            (Self::Unary(a), Self::Unary(b)) => a.content_eq(b),
            (Self::Cast(a), Self::Cast(b)) => a.content_eq(b),
            (Self::Instanceof(a), Self::Instanceof(b)) => a.content_eq(b),
            (Self::Binary(a), Self::Binary(b)) => a.content_eq(b),
            (Self::Assignment(a), Self::Assignment(b)) => a.content_eq(b),
            (Self::Conditional(a), Self::Conditional(b)) => a.content_eq(b),
            (Self::Switch(a), Self::Switch(b)) => a.content_eq(b),
            (Self::Parenthesized(a), Self::Parenthesized(b)) => a.content_eq(b),
            (Self::Lambda(a), Self::Lambda(b)) => a.content_eq(b),
            (Self::MethodReference(a), Self::MethodReference(b)) => a.content_eq(b),
            (Self::Primitive(a), Self::Primitive(b)) => a.content_eq(b),
            (Self::Array(a), Self::Array(b)) => a.content_eq(b),
            (Self::Simple(a), Self::Simple(b)) => a.content_eq(b),
            (Self::Qualified(a), Self::Qualified(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for TypeParameters<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.params, &other.params)
    }
}

impl ContentEq for TypeParameter<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.annotations, &other.annotations)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.bounds, &other.bounds)
    }
}

impl ContentEq for VariableType<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Var(a), Self::Var(b)) => a.content_eq(b),
            (Self::Primitive(a), Self::Primitive(b)) => a.content_eq(b),
            (Self::Array(a), Self::Array(b)) => a.content_eq(b),
            (Self::Simple(a), Self::Simple(b)) => a.content_eq(b),
            (Self::Qualified(a), Self::Qualified(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for VarType {
    fn content_eq(&self, _: &Self) -> bool {
        true
    }
}

impl ContentEq for Type<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primitive(a), Self::Primitive(b)) => a.content_eq(b),
            (Self::Array(a), Self::Array(b)) => a.content_eq(b),
            (Self::Simple(a), Self::Simple(b)) => a.content_eq(b),
            (Self::Qualified(a), Self::Qualified(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for MethodResult<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Void(a), Self::Void(b)) => a.content_eq(b),
            (Self::Primitive(a), Self::Primitive(b)) => a.content_eq(b),
            (Self::Array(a), Self::Array(b)) => a.content_eq(b),
            (Self::Simple(a), Self::Simple(b)) => a.content_eq(b),
            (Self::Qualified(a), Self::Qualified(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for VoidType {
    fn content_eq(&self, _: &Self) -> bool {
        true
    }
}

impl ContentEq for PrimitiveType<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.annotations, &other.annotations)
            && ContentEq::content_eq(&self.kind, &other.kind)
    }
}

impl ContentEq for PrimitiveTypeKind {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl ContentEq for ClassType<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Simple(a), Self::Simple(b)) => a.content_eq(b),
            (Self::Qualified(a), Self::Qualified(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for SimpleClassType<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.annotations, &other.annotations)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.args, &other.args)
    }
}

impl ContentEq for QualifiedClassType<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.qualifier, &other.qualifier)
            && ContentEq::content_eq(&self.annotations, &other.annotations)
            && ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.args, &other.args)
    }
}

impl ContentEq for ArrayType<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.element, &other.element)
            && ContentEq::content_eq(&self.annotations, &other.annotations)
    }
}

impl ContentEq for ArrayTypeDimension<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.annotations, &other.annotations)
    }
}

impl ContentEq for TypeArguments<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.args, &other.args)
    }
}

impl ContentEq for TypeArgument<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Wildcard(a), Self::Wildcard(b)) => a.content_eq(b),
            (Self::Primitive(a), Self::Primitive(b)) => a.content_eq(b),
            (Self::Array(a), Self::Array(b)) => a.content_eq(b),
            (Self::Simple(a), Self::Simple(b)) => a.content_eq(b),
            (Self::Qualified(a), Self::Qualified(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for WildcardTypeArgument<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.annotations, &other.annotations)
            && ContentEq::content_eq(&self.bound, &other.bound)
    }
}

impl ContentEq for WildcardBound<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.kind, &other.kind) && ContentEq::content_eq(&self.ty, &other.ty)
    }
}

impl ContentEq for WildcardBoundKind {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl ContentEq for IntLiteral<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.raw, &other.raw)
    }
}

impl ContentEq for FloatLiteral<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.raw, &other.raw)
    }
}

impl ContentEq for CharLiteral<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.raw, &other.raw)
    }
}

impl ContentEq for StringLiteral<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.raw, &other.raw)
    }
}

impl ContentEq for BooleanLiteral {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.value, &other.value)
    }
}

impl ContentEq for NullLiteral {
    fn content_eq(&self, _: &Self) -> bool {
        true
    }
}

impl ContentEq for IdentifierName<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for IdentifierReference<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for BindingIdentifier<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for LabelIdentifier<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for Name<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::Qualified(a), Self::Qualified(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for QualifiedName<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.qualifier, &other.qualifier)
            && ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for TypeName<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::Qualified(a), Self::Qualified(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for QualifiedTypeName<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.qualifier, &other.qualifier)
            && ContentEq::content_eq(&self.name, &other.name)
    }
}

impl ContentEq for Modifier<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Keyword(a), Self::Keyword(b)) => a.content_eq(b),
            (Self::Annotation(a), Self::Annotation(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for KeywordModifier {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.kind, &other.kind)
    }
}

impl ContentEq for Annotation<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.args, &other.args)
    }
}

impl ContentEq for AnnotationArgument<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Pair(a), Self::Pair(b)) => a.content_eq(b),
            (Self::Annotation(a), Self::Annotation(b)) => a.content_eq(b),
            (Self::Array(a), Self::Array(b)) => a.content_eq(b),
            (Self::IntLiteral(a), Self::IntLiteral(b)) => a.content_eq(b),
            (Self::FloatLiteral(a), Self::FloatLiteral(b)) => a.content_eq(b),
            (Self::CharLiteral(a), Self::CharLiteral(b)) => a.content_eq(b),
            (Self::StringLiteral(a), Self::StringLiteral(b)) => a.content_eq(b),
            (Self::BooleanLiteral(a), Self::BooleanLiteral(b)) => a.content_eq(b),
            (Self::NullLiteral(a), Self::NullLiteral(b)) => a.content_eq(b),
            (Self::This(a), Self::This(b)) => a.content_eq(b),
            (Self::Super(a), Self::Super(b)) => a.content_eq(b),
            (Self::QualifiedThis(a), Self::QualifiedThis(b)) => a.content_eq(b),
            (Self::QualifiedSuper(a), Self::QualifiedSuper(b)) => a.content_eq(b),
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::FieldAccess(a), Self::FieldAccess(b)) => a.content_eq(b),
            (Self::MethodCall(a), Self::MethodCall(b)) => a.content_eq(b),
            (Self::ClassLiteral(a), Self::ClassLiteral(b)) => a.content_eq(b),
            (Self::ClassInstanceCreation(a), Self::ClassInstanceCreation(b)) => a.content_eq(b),
            (Self::QualifiedClassInstanceCreation(a), Self::QualifiedClassInstanceCreation(b)) => {
                a.content_eq(b)
            }
            (Self::ArrayCreation(a), Self::ArrayCreation(b)) => a.content_eq(b),
            (Self::ArrayAccess(a), Self::ArrayAccess(b)) => a.content_eq(b),
            (Self::Update(a), Self::Update(b)) => a.content_eq(b),
            (Self::Unary(a), Self::Unary(b)) => a.content_eq(b),
            (Self::Cast(a), Self::Cast(b)) => a.content_eq(b),
            (Self::Instanceof(a), Self::Instanceof(b)) => a.content_eq(b),
            (Self::Binary(a), Self::Binary(b)) => a.content_eq(b),
            (Self::Assignment(a), Self::Assignment(b)) => a.content_eq(b),
            (Self::Conditional(a), Self::Conditional(b)) => a.content_eq(b),
            (Self::Switch(a), Self::Switch(b)) => a.content_eq(b),
            (Self::Parenthesized(a), Self::Parenthesized(b)) => a.content_eq(b),
            (Self::Lambda(a), Self::Lambda(b)) => a.content_eq(b),
            (Self::MethodReference(a), Self::MethodReference(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for ElementValuePair<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.name, &other.name)
            && ContentEq::content_eq(&self.value, &other.value)
    }
}

impl ContentEq for ElementValue<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Annotation(a), Self::Annotation(b)) => a.content_eq(b),
            (Self::Array(a), Self::Array(b)) => a.content_eq(b),
            (Self::IntLiteral(a), Self::IntLiteral(b)) => a.content_eq(b),
            (Self::FloatLiteral(a), Self::FloatLiteral(b)) => a.content_eq(b),
            (Self::CharLiteral(a), Self::CharLiteral(b)) => a.content_eq(b),
            (Self::StringLiteral(a), Self::StringLiteral(b)) => a.content_eq(b),
            (Self::BooleanLiteral(a), Self::BooleanLiteral(b)) => a.content_eq(b),
            (Self::NullLiteral(a), Self::NullLiteral(b)) => a.content_eq(b),
            (Self::This(a), Self::This(b)) => a.content_eq(b),
            (Self::Super(a), Self::Super(b)) => a.content_eq(b),
            (Self::QualifiedThis(a), Self::QualifiedThis(b)) => a.content_eq(b),
            (Self::QualifiedSuper(a), Self::QualifiedSuper(b)) => a.content_eq(b),
            (Self::Identifier(a), Self::Identifier(b)) => a.content_eq(b),
            (Self::FieldAccess(a), Self::FieldAccess(b)) => a.content_eq(b),
            (Self::MethodCall(a), Self::MethodCall(b)) => a.content_eq(b),
            (Self::ClassLiteral(a), Self::ClassLiteral(b)) => a.content_eq(b),
            (Self::ClassInstanceCreation(a), Self::ClassInstanceCreation(b)) => a.content_eq(b),
            (Self::QualifiedClassInstanceCreation(a), Self::QualifiedClassInstanceCreation(b)) => {
                a.content_eq(b)
            }
            (Self::ArrayCreation(a), Self::ArrayCreation(b)) => a.content_eq(b),
            (Self::ArrayAccess(a), Self::ArrayAccess(b)) => a.content_eq(b),
            (Self::Update(a), Self::Update(b)) => a.content_eq(b),
            (Self::Unary(a), Self::Unary(b)) => a.content_eq(b),
            (Self::Cast(a), Self::Cast(b)) => a.content_eq(b),
            (Self::Instanceof(a), Self::Instanceof(b)) => a.content_eq(b),
            (Self::Binary(a), Self::Binary(b)) => a.content_eq(b),
            (Self::Assignment(a), Self::Assignment(b)) => a.content_eq(b),
            (Self::Conditional(a), Self::Conditional(b)) => a.content_eq(b),
            (Self::Switch(a), Self::Switch(b)) => a.content_eq(b),
            (Self::Parenthesized(a), Self::Parenthesized(b)) => a.content_eq(b),
            (Self::Lambda(a), Self::Lambda(b)) => a.content_eq(b),
            (Self::MethodReference(a), Self::MethodReference(b)) => a.content_eq(b),
            _ => false,
        }
    }
}

impl ContentEq for ElementValueArrayInitializer<'_> {
    fn content_eq(&self, other: &Self) -> bool {
        ContentEq::content_eq(&self.values, &other.values)
            && ContentEq::content_eq(&self.trailing_comma, &other.trailing_comma)
    }
}

impl ContentEq for ModifierKind {
    fn content_eq(&self, other: &Self) -> bool {
        self == other
    }
}
