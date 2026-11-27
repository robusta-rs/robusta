// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/get_address.rs`.

use robusta_allocator::{Address, GetAddress};

use crate::ast::java::annotations::*;
use crate::ast::java::declarations::*;
use crate::ast::java::expressions::*;
use crate::ast::java::names::*;
use crate::ast::java::statements::*;
use crate::ast::java::types::*;

impl GetAddress for TypeDeclaration<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Empty(it) => GetAddress::address(it),
            Self::AnnotationInterface(it) => GetAddress::address(it),
            Self::Class(it) => GetAddress::address(it),
            Self::Interface(it) => GetAddress::address(it),
            Self::Enum(it) => GetAddress::address(it),
            Self::Record(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for LocalClassOrInterfaceDeclaration<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Class(it) => GetAddress::address(it),
            Self::Interface(it) => GetAddress::address(it),
            Self::Enum(it) => GetAddress::address(it),
            Self::Record(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for AnnotationInterfaceMemberDeclaration<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Element(it) => GetAddress::address(it),
            Self::Field(it) => GetAddress::address(it),
            Self::Empty(it) => GetAddress::address(it),
            Self::AnnotationInterface(it) => GetAddress::address(it),
            Self::Class(it) => GetAddress::address(it),
            Self::Interface(it) => GetAddress::address(it),
            Self::Enum(it) => GetAddress::address(it),
            Self::Record(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for ClassMember<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Field(it) => GetAddress::address(it),
            Self::Method(it) => GetAddress::address(it),
            Self::Constructor(it) => GetAddress::address(it),
            Self::CompactConstructor(it) => GetAddress::address(it),
            Self::Initializer(it) => GetAddress::address(it),
            Self::Empty(it) => GetAddress::address(it),
            Self::AnnotationInterface(it) => GetAddress::address(it),
            Self::Class(it) => GetAddress::address(it),
            Self::Interface(it) => GetAddress::address(it),
            Self::Enum(it) => GetAddress::address(it),
            Self::Record(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for InterfaceMember<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Constant(it) => GetAddress::address(it),
            Self::Method(it) => GetAddress::address(it),
            Self::Empty(it) => GetAddress::address(it),
            Self::AnnotationInterface(it) => GetAddress::address(it),
            Self::Class(it) => GetAddress::address(it),
            Self::Interface(it) => GetAddress::address(it),
            Self::Enum(it) => GetAddress::address(it),
            Self::Record(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for Statement<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Block(it) => GetAddress::address(it),
            Self::Empty(it) => GetAddress::address(it),
            Self::ConstructorInvocation(it) => GetAddress::address(it),
            Self::LocalVariable(it) => GetAddress::address(it),
            Self::Expression(it) => GetAddress::address(it),
            Self::If(it) => GetAddress::address(it),
            Self::Switch(it) => GetAddress::address(it),
            Self::While(it) => GetAddress::address(it),
            Self::Do(it) => GetAddress::address(it),
            Self::For(it) => GetAddress::address(it),
            Self::EnhancedFor(it) => GetAddress::address(it),
            Self::Break(it) => GetAddress::address(it),
            Self::Continue(it) => GetAddress::address(it),
            Self::Throw(it) => GetAddress::address(it),
            Self::Assert(it) => GetAddress::address(it),
            Self::Try(it) => GetAddress::address(it),
            Self::Synchronized(it) => GetAddress::address(it),
            Self::Return(it) => GetAddress::address(it),
            Self::Yield(it) => GetAddress::address(it),
            Self::Labeled(it) => GetAddress::address(it),
            Self::Class(it) => GetAddress::address(it),
            Self::Interface(it) => GetAddress::address(it),
            Self::Enum(it) => GetAddress::address(it),
            Self::Record(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for SwitchCase<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Rule(it) => GetAddress::address(it),
            Self::Group(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for SwitchRuleBody<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Block(it) => GetAddress::address(it),
            Self::Throw(it) => GetAddress::address(it),
            Self::IntLiteral(it) => GetAddress::address(it),
            Self::FloatLiteral(it) => GetAddress::address(it),
            Self::CharLiteral(it) => GetAddress::address(it),
            Self::StringLiteral(it) => GetAddress::address(it),
            Self::BooleanLiteral(it) => GetAddress::address(it),
            Self::NullLiteral(it) => GetAddress::address(it),
            Self::This(it) => GetAddress::address(it),
            Self::Super(it) => GetAddress::address(it),
            Self::QualifiedThis(it) => GetAddress::address(it),
            Self::QualifiedSuper(it) => GetAddress::address(it),
            Self::Identifier(it) => GetAddress::address(it),
            Self::FieldAccess(it) => GetAddress::address(it),
            Self::MethodCall(it) => GetAddress::address(it),
            Self::ClassLiteral(it) => GetAddress::address(it),
            Self::ClassInstanceCreation(it) => GetAddress::address(it),
            Self::QualifiedClassInstanceCreation(it) => GetAddress::address(it),
            Self::ArrayCreation(it) => GetAddress::address(it),
            Self::ArrayAccess(it) => GetAddress::address(it),
            Self::Update(it) => GetAddress::address(it),
            Self::Unary(it) => GetAddress::address(it),
            Self::Cast(it) => GetAddress::address(it),
            Self::Instanceof(it) => GetAddress::address(it),
            Self::Binary(it) => GetAddress::address(it),
            Self::Assignment(it) => GetAddress::address(it),
            Self::Conditional(it) => GetAddress::address(it),
            Self::Switch(it) => GetAddress::address(it),
            Self::Parenthesized(it) => GetAddress::address(it),
            Self::Lambda(it) => GetAddress::address(it),
            Self::MethodReference(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for ForInit<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Declaration(it) => GetAddress::address(it),
            Self::Expressions(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for Resource<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Declaration(it) => GetAddress::address(it),
            Self::Identifier(it) => GetAddress::address(it),
            Self::Field(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for ResourceAccess<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Identifier(it) => GetAddress::address(it),
            Self::Field(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for VariableInitializer<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Array(it) => GetAddress::address(it),
            Self::IntLiteral(it) => GetAddress::address(it),
            Self::FloatLiteral(it) => GetAddress::address(it),
            Self::CharLiteral(it) => GetAddress::address(it),
            Self::StringLiteral(it) => GetAddress::address(it),
            Self::BooleanLiteral(it) => GetAddress::address(it),
            Self::NullLiteral(it) => GetAddress::address(it),
            Self::This(it) => GetAddress::address(it),
            Self::Super(it) => GetAddress::address(it),
            Self::QualifiedThis(it) => GetAddress::address(it),
            Self::QualifiedSuper(it) => GetAddress::address(it),
            Self::Identifier(it) => GetAddress::address(it),
            Self::FieldAccess(it) => GetAddress::address(it),
            Self::MethodCall(it) => GetAddress::address(it),
            Self::ClassLiteral(it) => GetAddress::address(it),
            Self::ClassInstanceCreation(it) => GetAddress::address(it),
            Self::QualifiedClassInstanceCreation(it) => GetAddress::address(it),
            Self::ArrayCreation(it) => GetAddress::address(it),
            Self::ArrayAccess(it) => GetAddress::address(it),
            Self::Update(it) => GetAddress::address(it),
            Self::Unary(it) => GetAddress::address(it),
            Self::Cast(it) => GetAddress::address(it),
            Self::Instanceof(it) => GetAddress::address(it),
            Self::Binary(it) => GetAddress::address(it),
            Self::Assignment(it) => GetAddress::address(it),
            Self::Conditional(it) => GetAddress::address(it),
            Self::Switch(it) => GetAddress::address(it),
            Self::Parenthesized(it) => GetAddress::address(it),
            Self::Lambda(it) => GetAddress::address(it),
            Self::MethodReference(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for Expression<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::IntLiteral(it) => GetAddress::address(it),
            Self::FloatLiteral(it) => GetAddress::address(it),
            Self::CharLiteral(it) => GetAddress::address(it),
            Self::StringLiteral(it) => GetAddress::address(it),
            Self::BooleanLiteral(it) => GetAddress::address(it),
            Self::NullLiteral(it) => GetAddress::address(it),
            Self::This(it) => GetAddress::address(it),
            Self::Super(it) => GetAddress::address(it),
            Self::QualifiedThis(it) => GetAddress::address(it),
            Self::QualifiedSuper(it) => GetAddress::address(it),
            Self::Identifier(it) => GetAddress::address(it),
            Self::FieldAccess(it) => GetAddress::address(it),
            Self::MethodCall(it) => GetAddress::address(it),
            Self::ClassLiteral(it) => GetAddress::address(it),
            Self::ClassInstanceCreation(it) => GetAddress::address(it),
            Self::QualifiedClassInstanceCreation(it) => GetAddress::address(it),
            Self::ArrayCreation(it) => GetAddress::address(it),
            Self::ArrayAccess(it) => GetAddress::address(it),
            Self::Update(it) => GetAddress::address(it),
            Self::Unary(it) => GetAddress::address(it),
            Self::Cast(it) => GetAddress::address(it),
            Self::Instanceof(it) => GetAddress::address(it),
            Self::Binary(it) => GetAddress::address(it),
            Self::Assignment(it) => GetAddress::address(it),
            Self::Conditional(it) => GetAddress::address(it),
            Self::Switch(it) => GetAddress::address(it),
            Self::Parenthesized(it) => GetAddress::address(it),
            Self::Lambda(it) => GetAddress::address(it),
            Self::MethodReference(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for Callee<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Identifier(it) => GetAddress::address(it),
            Self::Member(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for InstanceofRhs<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Primitive(it) => GetAddress::address(it),
            Self::Array(it) => GetAddress::address(it),
            Self::Simple(it) => GetAddress::address(it),
            Self::Qualified(it) => GetAddress::address(it),
            Self::TypePattern(it) => GetAddress::address(it),
            Self::RecordPattern(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for Pattern<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::TypePattern(it) => GetAddress::address(it),
            Self::RecordPattern(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for ComponentPattern<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::MatchAll(it) => GetAddress::address(it),
            Self::TypePattern(it) => GetAddress::address(it),
            Self::RecordPattern(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for AssignmentTarget<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Identifier(it) => GetAddress::address(it),
            Self::FieldAccess(it) => GetAddress::address(it),
            Self::ArrayAccess(it) => GetAddress::address(it),
            Self::Parenthesized(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for LambdaBody<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Block(it) => GetAddress::address(it),
            Self::IntLiteral(it) => GetAddress::address(it),
            Self::FloatLiteral(it) => GetAddress::address(it),
            Self::CharLiteral(it) => GetAddress::address(it),
            Self::StringLiteral(it) => GetAddress::address(it),
            Self::BooleanLiteral(it) => GetAddress::address(it),
            Self::NullLiteral(it) => GetAddress::address(it),
            Self::This(it) => GetAddress::address(it),
            Self::Super(it) => GetAddress::address(it),
            Self::QualifiedThis(it) => GetAddress::address(it),
            Self::QualifiedSuper(it) => GetAddress::address(it),
            Self::Identifier(it) => GetAddress::address(it),
            Self::FieldAccess(it) => GetAddress::address(it),
            Self::MethodCall(it) => GetAddress::address(it),
            Self::ClassLiteral(it) => GetAddress::address(it),
            Self::ClassInstanceCreation(it) => GetAddress::address(it),
            Self::QualifiedClassInstanceCreation(it) => GetAddress::address(it),
            Self::ArrayCreation(it) => GetAddress::address(it),
            Self::ArrayAccess(it) => GetAddress::address(it),
            Self::Update(it) => GetAddress::address(it),
            Self::Unary(it) => GetAddress::address(it),
            Self::Cast(it) => GetAddress::address(it),
            Self::Instanceof(it) => GetAddress::address(it),
            Self::Binary(it) => GetAddress::address(it),
            Self::Assignment(it) => GetAddress::address(it),
            Self::Conditional(it) => GetAddress::address(it),
            Self::Switch(it) => GetAddress::address(it),
            Self::Parenthesized(it) => GetAddress::address(it),
            Self::Lambda(it) => GetAddress::address(it),
            Self::MethodReference(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for MethodReferenceTarget<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::IntLiteral(it) => GetAddress::address(it),
            Self::FloatLiteral(it) => GetAddress::address(it),
            Self::CharLiteral(it) => GetAddress::address(it),
            Self::StringLiteral(it) => GetAddress::address(it),
            Self::BooleanLiteral(it) => GetAddress::address(it),
            Self::NullLiteral(it) => GetAddress::address(it),
            Self::This(it) => GetAddress::address(it),
            Self::Super(it) => GetAddress::address(it),
            Self::QualifiedThis(it) => GetAddress::address(it),
            Self::QualifiedSuper(it) => GetAddress::address(it),
            Self::Identifier(it) => GetAddress::address(it),
            Self::FieldAccess(it) => GetAddress::address(it),
            Self::MethodCall(it) => GetAddress::address(it),
            Self::ClassLiteral(it) => GetAddress::address(it),
            Self::ClassInstanceCreation(it) => GetAddress::address(it),
            Self::QualifiedClassInstanceCreation(it) => GetAddress::address(it),
            Self::ArrayCreation(it) => GetAddress::address(it),
            Self::ArrayAccess(it) => GetAddress::address(it),
            Self::Update(it) => GetAddress::address(it),
            Self::Unary(it) => GetAddress::address(it),
            Self::Cast(it) => GetAddress::address(it),
            Self::Instanceof(it) => GetAddress::address(it),
            Self::Binary(it) => GetAddress::address(it),
            Self::Assignment(it) => GetAddress::address(it),
            Self::Conditional(it) => GetAddress::address(it),
            Self::Switch(it) => GetAddress::address(it),
            Self::Parenthesized(it) => GetAddress::address(it),
            Self::Lambda(it) => GetAddress::address(it),
            Self::MethodReference(it) => GetAddress::address(it),
            Self::Primitive(it) => GetAddress::address(it),
            Self::Array(it) => GetAddress::address(it),
            Self::Simple(it) => GetAddress::address(it),
            Self::Qualified(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for VariableType<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Var(it) => GetAddress::address(it),
            Self::Primitive(it) => GetAddress::address(it),
            Self::Array(it) => GetAddress::address(it),
            Self::Simple(it) => GetAddress::address(it),
            Self::Qualified(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for Type<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Primitive(it) => GetAddress::address(it),
            Self::Array(it) => GetAddress::address(it),
            Self::Simple(it) => GetAddress::address(it),
            Self::Qualified(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for MethodResult<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Void(it) => GetAddress::address(it),
            Self::Primitive(it) => GetAddress::address(it),
            Self::Array(it) => GetAddress::address(it),
            Self::Simple(it) => GetAddress::address(it),
            Self::Qualified(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for ClassType<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Simple(it) => GetAddress::address(it),
            Self::Qualified(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for TypeArgument<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Wildcard(it) => GetAddress::address(it),
            Self::Primitive(it) => GetAddress::address(it),
            Self::Array(it) => GetAddress::address(it),
            Self::Simple(it) => GetAddress::address(it),
            Self::Qualified(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for Name<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Identifier(it) => GetAddress::address(it),
            Self::Qualified(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for TypeName<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Identifier(it) => GetAddress::address(it),
            Self::Qualified(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for AnnotationArgument<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Pair(it) => GetAddress::address(it),
            Self::Annotation(it) => GetAddress::address(it),
            Self::Array(it) => GetAddress::address(it),
            Self::IntLiteral(it) => GetAddress::address(it),
            Self::FloatLiteral(it) => GetAddress::address(it),
            Self::CharLiteral(it) => GetAddress::address(it),
            Self::StringLiteral(it) => GetAddress::address(it),
            Self::BooleanLiteral(it) => GetAddress::address(it),
            Self::NullLiteral(it) => GetAddress::address(it),
            Self::This(it) => GetAddress::address(it),
            Self::Super(it) => GetAddress::address(it),
            Self::QualifiedThis(it) => GetAddress::address(it),
            Self::QualifiedSuper(it) => GetAddress::address(it),
            Self::Identifier(it) => GetAddress::address(it),
            Self::FieldAccess(it) => GetAddress::address(it),
            Self::MethodCall(it) => GetAddress::address(it),
            Self::ClassLiteral(it) => GetAddress::address(it),
            Self::ClassInstanceCreation(it) => GetAddress::address(it),
            Self::QualifiedClassInstanceCreation(it) => GetAddress::address(it),
            Self::ArrayCreation(it) => GetAddress::address(it),
            Self::ArrayAccess(it) => GetAddress::address(it),
            Self::Update(it) => GetAddress::address(it),
            Self::Unary(it) => GetAddress::address(it),
            Self::Cast(it) => GetAddress::address(it),
            Self::Instanceof(it) => GetAddress::address(it),
            Self::Binary(it) => GetAddress::address(it),
            Self::Assignment(it) => GetAddress::address(it),
            Self::Conditional(it) => GetAddress::address(it),
            Self::Switch(it) => GetAddress::address(it),
            Self::Parenthesized(it) => GetAddress::address(it),
            Self::Lambda(it) => GetAddress::address(it),
            Self::MethodReference(it) => GetAddress::address(it),
        }
    }
}

impl GetAddress for ElementValue<'_> {
    // `#[inline]` because compiler should boil this down to a single assembly instruction
    #[inline]
    fn address(&self) -> Address {
        match self {
            Self::Annotation(it) => GetAddress::address(it),
            Self::Array(it) => GetAddress::address(it),
            Self::IntLiteral(it) => GetAddress::address(it),
            Self::FloatLiteral(it) => GetAddress::address(it),
            Self::CharLiteral(it) => GetAddress::address(it),
            Self::StringLiteral(it) => GetAddress::address(it),
            Self::BooleanLiteral(it) => GetAddress::address(it),
            Self::NullLiteral(it) => GetAddress::address(it),
            Self::This(it) => GetAddress::address(it),
            Self::Super(it) => GetAddress::address(it),
            Self::QualifiedThis(it) => GetAddress::address(it),
            Self::QualifiedSuper(it) => GetAddress::address(it),
            Self::Identifier(it) => GetAddress::address(it),
            Self::FieldAccess(it) => GetAddress::address(it),
            Self::MethodCall(it) => GetAddress::address(it),
            Self::ClassLiteral(it) => GetAddress::address(it),
            Self::ClassInstanceCreation(it) => GetAddress::address(it),
            Self::QualifiedClassInstanceCreation(it) => GetAddress::address(it),
            Self::ArrayCreation(it) => GetAddress::address(it),
            Self::ArrayAccess(it) => GetAddress::address(it),
            Self::Update(it) => GetAddress::address(it),
            Self::Unary(it) => GetAddress::address(it),
            Self::Cast(it) => GetAddress::address(it),
            Self::Instanceof(it) => GetAddress::address(it),
            Self::Binary(it) => GetAddress::address(it),
            Self::Assignment(it) => GetAddress::address(it),
            Self::Conditional(it) => GetAddress::address(it),
            Self::Switch(it) => GetAddress::address(it),
            Self::Parenthesized(it) => GetAddress::address(it),
            Self::Lambda(it) => GetAddress::address(it),
            Self::MethodReference(it) => GetAddress::address(it),
        }
    }
}
