use robusta_span::Span;
use robusta_syntax::operator::UnaryOp;

use crate::{ast::*, match_element_value};

impl<'a> CompilationUnit<'a> {
    /// Import declarations from any compilation-unit form.
    pub fn imports(&self) -> &[ImportDeclaration<'a>] {
        match &self.content {
            CompilationUnitContent::Ordinary(unit) => &unit.imports,
            CompilationUnitContent::Compact(unit) => &unit.imports,
            CompilationUnitContent::Modular(unit) => &unit.imports,
        }
    }

    /// Top-level types, empty for compact and modular units.
    pub fn types(&self) -> &[TypeDeclaration<'a>] {
        match &self.content {
            CompilationUnitContent::Ordinary(unit) => &unit.types,
            CompilationUnitContent::Compact(_) | CompilationUnitContent::Modular(_) => &[],
        }
    }

    /// Package declaration, available only in ordinary units.
    pub fn package(&self) -> Option<&PackageDeclaration<'a>> {
        match &self.content {
            CompilationUnitContent::Ordinary(unit) => unit.package.as_ref(),
            CompilationUnitContent::Compact(_) | CompilationUnitContent::Modular(_) => None,
        }
    }

    /// The compact unit containing the implicitly declared class, if any.
    pub fn compact(&self) -> Option<&CompactCompilationUnit<'a>> {
        match &self.content {
            CompilationUnitContent::Compact(unit) => Some(unit),
            _ => None,
        }
    }

    /// The modular unit's module declaration, if any.
    pub fn module(&self) -> Option<&ModuleDeclaration<'a>> {
        match &self.content {
            CompilationUnitContent::Modular(unit) => Some(&unit.module),
            _ => None,
        }
    }
}

impl RequiresModifierKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RequiresModifierKind::Transitive => "transitive",
            RequiresModifierKind::Static => "static",
        }
    }
}

impl PackageDirectiveKind {
    pub fn as_str(self) -> &'static str {
        match self {
            PackageDirectiveKind::Exports => "exports",
            PackageDirectiveKind::Opens => "opens",
        }
    }
}

impl ConstructorInvocationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ConstructorInvocationKind::This => "this",
            ConstructorInvocationKind::Super => "super",
        }
    }
}

impl<'a> Expression<'a> {
    /// Whether the outermost expression matches JLS §14.8 `StatementExpression`.
    ///
    /// Parentheses disqualify it: `f();` is legal, `(f());` is not.
    pub fn is_statement_expression(&self) -> bool {
        match self {
            Self::Assignment(_)
            | Self::Postfix(_)
            | Self::MethodCall(_)
            | Self::ClassInstanceCreation(_)
            | Self::QualifiedClassInstanceCreation(_) => true,
            Self::Unary(unary) => {
                matches!(unary.op, UnaryOp::PreIncrement | UnaryOp::PreDecrement)
            }
            _ => false,
        }
    }

    /// The class body of `new Foo() {}` or `outer.new Foo() {}` (JLS §15.9.5).
    pub fn anonymous_class_body(&self) -> Option<&ClassBody<'a>> {
        match self {
            Self::ClassInstanceCreation(creation) => creation.body.as_ref(),
            Self::QualifiedClassInstanceCreation(creation) => creation.body.as_ref(),
            _ => None,
        }
    }

    /// Whether this is a dotted name such as `a.b.c`.
    pub fn is_dotted_name(&self) -> bool {
        matches!(self, Self::FieldAccess(access) if access.is_dotted_name())
    }
}

impl FieldAccess<'_> {
    /// Whether this is a dotted name such as `a.b.c`, which only resolving `a` can
    /// tell from a field access (JLS 6.5).
    pub fn is_dotted_name(&self) -> bool {
        let mut target = &self.target;
        loop {
            match target {
                Expression::Identifier(_) => return true,
                Expression::FieldAccess(access) => target = &access.target,
                _ => return false,
            }
        }
    }
}

impl<'a> MethodCall<'a> {
    /// The receiver, absent for `f()`.
    pub fn target(&self) -> Option<&Expression<'a>> {
        self.member().map(|member| &member.target)
    }

    /// The callee, unless it is a simple name.
    pub fn member(&self) -> Option<&MethodMember<'a>> {
        match &self.callee {
            Callee::Member(member) => Some(member),
            Callee::Identifier(_) => None,
        }
    }

    /// The type arguments before the method's name, empty if absent.
    pub fn type_args(&self) -> &[TypeArgument<'a>] {
        self.member().and_then(|member| member.type_args.as_deref()).map_or(&[], |list| &list.args)
    }

    /// The method's simple name.
    pub fn name(&self) -> &'a str {
        match &self.callee {
            Callee::Member(member) => member.name.name,
            Callee::Identifier(name) => name.name,
        }
    }
}

impl<'a> Name<'a> {
    /// The name to the left of the last dot.
    pub fn qualifier(&self) -> Option<&Self> {
        match self {
            Self::Identifier(_) => None,
            Self::Qualified(name) => Some(&name.qualifier),
        }
    }

    /// The name after the last dot.
    pub fn last(&self) -> &IdentifierName<'a> {
        match self {
            Self::Identifier(name) => name,
            Self::Qualified(name) => &name.name,
        }
    }

    /// The simple names, last first.
    pub fn names_from_last(&self) -> impl Iterator<Item = &'a str> + '_ {
        std::iter::successors(Some(self), |name| name.qualifier()).map(|name| name.last().name)
    }

    /// The name as Java writes it, with escapes decoded.
    pub fn dotted(&self) -> String {
        let mut names = self.names_from_last().collect::<std::vec::Vec<_>>();
        names.reverse();
        names.join(".")
    }
}

impl<'a> TypeName<'a> {
    /// The name after the last dot.
    pub fn last(&self) -> &'a str {
        match self {
            Self::Identifier(name) => name.name,
            Self::Qualified(name) => name.name.name,
        }
    }
}

impl<'a> LambdaExpression<'a> {
    // Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_ast/src/ast_impl/js.rs
    /// Get expression part of `LambdaExpression`: `() -> expression_part`.
    pub fn get_expression(&self) -> Option<&Expression<'a>> {
        self.body.as_expression()
    }
}

impl FormalParameters<'_> {
    /// Whether the list is written in parentheses, which only a concise lambda's
    /// `x -> …` is not.
    pub fn has_parentheses(&self) -> bool {
        self.items.first().is_none_or(|first| first.span != self.span)
    }
}

impl PrimitiveTypeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            PrimitiveTypeKind::Boolean => "boolean",
            PrimitiveTypeKind::Byte => "byte",
            PrimitiveTypeKind::Char => "char",
            PrimitiveTypeKind::Short => "short",
            PrimitiveTypeKind::Int => "int",
            PrimitiveTypeKind::Long => "long",
            PrimitiveTypeKind::Float => "float",
            PrimitiveTypeKind::Double => "double",
        }
    }
}

impl<'a> ClassType<'a> {
    /// The class type to the left of the last dot.
    pub fn qualifier(&self) -> Option<&Self> {
        match self {
            Self::Simple(_) => None,
            Self::Qualified(ty) => Some(&ty.qualifier),
        }
    }

    /// The name after the last dot.
    pub fn name(&self) -> &'a str {
        match self {
            Self::Simple(ty) => ty.name.name,
            Self::Qualified(ty) => ty.name.name,
        }
    }

    /// Where the name after the last dot is.
    pub fn name_span(&self) -> Span {
        match self {
            Self::Simple(ty) => ty.name.span,
            Self::Qualified(ty) => ty.name.span,
        }
    }

    /// The type arguments of the name after the last dot: `Map.Entry<K, V>` gives
    /// `K, V`.
    pub fn args(&self) -> &[TypeArgument<'a>] {
        self.type_arguments().map_or(&[], |list| &list.args)
    }

    /// The type argument list of the name after the last dot, if written.
    pub fn type_arguments(&self) -> Option<&TypeArguments<'a>> {
        match self {
            Self::Simple(ty) => ty.args.as_deref(),
            Self::Qualified(ty) => ty.args.as_deref(),
        }
    }

    /// This class type and those it is qualified by, last first.
    pub fn self_and_qualifiers(&self) -> impl Iterator<Item = &Self> + '_ {
        std::iter::successors(Some(self), |ty| ty.qualifier())
    }
}

impl TypeArguments<'_> {
    /// Whether this is a class instance creation's `<>` (JLS §15.9).
    pub fn is_diamond(&self) -> bool {
        self.args.is_empty()
    }
}

impl WildcardBoundKind {
    pub fn as_str(self) -> &'static str {
        match self {
            WildcardBoundKind::Extends => "extends",
            WildcardBoundKind::Super => "super",
        }
    }
}

impl<'a> Modifier<'a> {
    pub fn as_annotation(&self) -> Option<&Annotation<'a>> {
        match self {
            Modifier::Annotation(annotation) => Some(annotation),
            Modifier::Keyword(_) => None,
        }
    }
}

impl<'a> Annotation<'a> {
    /// Whether this is a marker annotation (`@Foo`, without parentheses).
    pub fn is_marker(&self) -> bool {
        self.args.is_none()
    }

    /// The value of the element named `value`, which a single-element
    /// annotation gives without the name (JLS §9.7.3).
    pub fn value(&self) -> Option<&ElementValue<'a>> {
        self.args.as_ref()?.iter().find_map(|argument| match argument {
            AnnotationArgument::Pair(pair) => (pair.name.name == "value").then_some(&pair.value),
            it @ match_element_value!(AnnotationArgument) => Some(it.to_element_value()),
        })
    }
}

impl ModifierKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ModifierKind::Public => "public",
            ModifierKind::Protected => "protected",
            ModifierKind::Private => "private",
            ModifierKind::Abstract => "abstract",
            ModifierKind::Default => "default",
            ModifierKind::Static => "static",
            ModifierKind::Final => "final",
            ModifierKind::Native => "native",
            ModifierKind::Sealed => "sealed",
            ModifierKind::NonSealed => "non-sealed",
            ModifierKind::Strictfp => "strictfp",
            ModifierKind::Synchronized => "synchronized",
            ModifierKind::Transient => "transient",
            ModifierKind::Volatile => "volatile",
        }
    }
}

#[cfg(test)]
mod tests {
    use robusta_allocator::{Allocator, Vec};
    use robusta_span::{ContentEq, GetSpan, Span};
    use robusta_syntax::{
        node::NodeId, operator::BinaryOp, reference::ReferenceId, scope::ScopeId, symbol::SymbolId,
    };

    use crate::{ast::*, builder::AstBuilder, match_expression, match_type};

    fn this(allocator: &Allocator) -> Expression<'_> {
        Expression::new_this_expression(Span::new(1, 5), &AstBuilder::new(allocator))
    }

    /// `import <qualifier>.<name>;` at `span`.
    fn import<'a>(
        span: Span,
        kind: ImportKind,
        qualifier: &'a str,
        name: &'a str,
        builder: &AstBuilder<'a>,
    ) -> ImportDeclaration<'a> {
        let qualified = Name::new_qualified_name(
            span,
            Name::new_identifier_name(span, qualifier, builder),
            IdentifierName::new(span, name, builder),
            builder,
        );
        let local = matches!(kind, ImportKind::SingleType | ImportKind::SingleStatic)
            .then(|| BindingIdentifier::new(span, name, builder));
        ImportDeclaration::new(span, kind, qualified, local, builder)
    }

    #[test]
    fn content_eq_ignores_spans_and_semantic_ids() {
        let allocator = &Allocator::new();
        let builder = &AstBuilder::new(allocator);

        let first = import(Span::new(0, 10), ImportKind::SingleType, "util", "List", builder);
        let second = import(Span::new(40, 50), ImportKind::SingleType, "util", "List", builder);
        second.set_node_id(NodeId::new(7));
        second.local.as_ref().unwrap().set_symbol_id(SymbolId::new(3));
        assert!(first.content_eq(&second));
        assert!(!first.content_ne(&second));

        let span = Span::new(0, 1);
        let first = TypeName::new_identifier_reference(span, "A", builder);
        let second = TypeName::new_identifier_reference(Span::new(5, 6), "A", builder);
        let TypeName::Identifier(reference) = &second else { unreachable!() };
        reference.set_reference_id(ReferenceId::new(1));
        assert!(first.content_eq(&second));
    }

    #[test]
    fn content_eq_compares_every_part_a_node_holds() {
        let allocator = &Allocator::new();
        let builder = &AstBuilder::new(allocator);
        let span = Span::new(0, 1);
        let kind = ImportKind::SingleType;

        let list = import(span, kind, "util", "List", builder);
        assert!(list.content_ne(&import(span, ImportKind::SingleStatic, "util", "List", builder)));
        assert!(list.content_ne(&import(span, kind, "util", "Map", builder)));
        assert!(list.content_ne(&import(span, kind, "lang", "List", builder)));

        // An option.
        let mut without_local = import(span, kind, "util", "List", builder);
        without_local.local = None;
        assert!(list.content_ne(&without_local));

        // A choice's variants, and a boxed node under one.
        let simple = Name::new_identifier_name(span, "List", builder);
        assert!(list.name.content_ne(&simple));
        assert!(simple.content_eq(&Name::new_identifier_name(span, "List", builder)));
    }

    #[test]
    fn content_eq_compares_vectors_by_length_and_elements_in_order() {
        let allocator = &Allocator::new();
        let builder = &AstBuilder::new(allocator);
        let span = Span::new(0, 1);
        let package = |annotations: &[&'static str]| {
            let annotations = annotations.iter().map(|&name| {
                let name = TypeName::new_identifier_reference(span, name, builder);
                Annotation::new(span, name, None, builder)
            });
            PackageDeclaration::new(
                span,
                Vec::from_iter_in(annotations, builder),
                Name::new_identifier_name(span, "p", builder),
                builder,
            )
        };

        assert!(package(&["A", "B"]).content_eq(&package(&["A", "B"])));
        assert!(package(&["A", "B"]).content_ne(&package(&["A"])));
        assert!(package(&["A", "B"]).content_ne(&package(&["B", "A"])));
        assert!(package(&[]).content_eq(&package(&[])));
    }

    #[test]
    fn content_eq_compares_stored_syntax_as_stored() {
        let allocator = &Allocator::new();
        let builder = &AstBuilder::new(allocator);
        let span = Span::new(0, 1);
        let int = |raw| Expression::new_int_literal(span, raw, builder);

        // Literals keep their spelling, so `1` and `0x1` differ.
        assert!(int("1").content_eq(&int("1")));
        assert!(int("1").content_ne(&int("0x1")));
        assert!(int("1").content_ne(&Expression::new_this_expression(span, builder)));

        // `@A` and `@A()` differ, because an annotation stores whether it has an argument list.
        let name = || TypeName::new_identifier_reference(span, "A", builder);
        let no_args = Annotation::new(span, name(), None, builder);
        let empty_args = Annotation::new(span, name(), Some(Vec::new_in(builder)), builder);
        assert!(no_args.content_ne(&empty_args));

        // A span is not content, so `x -> x` and `(x) -> x` are the same lambda.
        let lambda = |params_span| {
            let name = BindingIdentifier::new(span, "x", builder);
            let parameter = FormalParameter::new(span, [], None, false, [], name, [], builder);
            let params = FormalParameters::new(params_span, None, [parameter], builder);
            let body = LambdaBody::new_identifier_reference(span, "x", builder);
            LambdaExpression::new(span, params, body, builder)
        };
        assert!(lambda(span).content_eq(&lambda(Span::new(0, 3))));

        // An operator is content.
        let binary = |op| Expression::new_binary_expression(span, op, int("1"), int("2"), builder);
        assert!(binary(BinaryOp::Add).content_eq(&binary(BinaryOp::Add)));
        assert!(binary(BinaryOp::Add).content_ne(&binary(BinaryOp::Sub)));
    }

    #[test]
    fn content_eq_leaves_comments_out_of_a_compilation_unit() {
        let allocator = &Allocator::new();
        let builder = &AstBuilder::new(allocator);
        let span = Span::new(0, 1);
        let unit = |comments: &[Comment], name| {
            let import = import(span, ImportKind::SingleType, "util", name, builder);
            let content = CompilationUnitContent::new_ordinary_compilation_unit(
                span,
                None,
                [import],
                [],
                builder,
            );
            let comments = Vec::from_iter_in(comments.iter().copied(), builder);
            CompilationUnit::new(span, comments, content, builder)
        };

        let comment = Comment::new(0, 2, CommentKind::Line);
        let commented = unit(&[comment], "List");
        let bare = unit(&[], "List");
        bare.set_scope_id(ScopeId::new(1));
        assert!(commented.content_eq(&bare));
        assert!(commented.content_ne(&unit(&[comment], "Map")));
    }

    #[test]
    fn a_choice_converts_to_and_from_what_it_inherits() {
        let allocator = &Allocator::new();

        let mut init = VariableInitializer::from(this(allocator));
        assert!(matches!(init, VariableInitializer::This(_)));
        assert!(matches!(init, match_expression!(VariableInitializer)));
        assert!(init.is_expression());
        assert_eq!(init.as_expression().map(GetSpan::span), Some(Span::new(1, 5)));
        assert_eq!(init.to_expression().span(), Span::new(1, 5));
        assert!(matches!(init.as_expression_mut(), Some(Expression::This(_))));
        assert!(matches!(init.to_expression_mut(), Expression::This(_)));
        assert!(std::ptr::eq(init.to_expression().as_variable_initializer(), &raw const init));
        assert!(matches!(Expression::try_from(init), Ok(Expression::This(_))));

        let builder = AstBuilder::new(allocator);
        let init = VariableInitializer::new_array_initializer(Span::new(6, 8), [], &builder);
        assert!(!matches!(init, match_expression!(VariableInitializer)));
        assert!(!init.is_expression());
        assert!(init.as_expression().is_none());
        assert!(Expression::try_from(init).is_err());
    }

    #[test]
    fn a_choice_reads_variants_inherited_through_another_choice() {
        let allocator = &Allocator::new();

        let argument = AnnotationArgument::from(ElementValue::from(this(allocator)));
        assert!(matches!(argument, AnnotationArgument::This(_)));
        assert!(argument.is_element_value() && argument.is_expression());
        assert!(matches!(argument.to_element_value(), ElementValue::This(_)));
        assert!(matches!(argument.to_expression(), Expression::This(_)));

        let member =
            ClassMember::new_empty_declaration(Span::new(0, 1), &AstBuilder::new(allocator));
        assert!(member.is_type_declaration());
        assert!(!member.is_local_class_or_interface_declaration());
        assert!(matches!(member.to_type_declaration(), TypeDeclaration::Empty(_)));
    }

    #[test]
    fn choices_that_inherit_the_same_choice_share_its_discriminants() {
        let allocator = &Allocator::new();
        let builder = AstBuilder::new(allocator);
        let span = Span::new(0, 1);
        let kind = PrimitiveTypeKind::Int;

        let ty = Type::new_primitive_type(span, [], kind, &builder);
        let argument = TypeArgument::new_primitive_type(span, [], kind, &builder);
        let variable = VariableType::new_primitive_type(span, [], kind, &builder);
        assert!(matches!(argument, match_type!(TypeArgument)));
        assert!(matches!(variable, match_type!(VariableType)));
        assert!(matches!(argument.to_type(), Type::Primitive(_)));
        assert!(matches!(variable.to_type(), Type::Primitive(_)));
        assert!(matches!(ty.as_method_reference_target(), MethodReferenceTarget::Primitive(_)));
        assert!(matches!(ty.as_instanceof_rhs(), InstanceofRhs::Primitive(_)));
    }
}
