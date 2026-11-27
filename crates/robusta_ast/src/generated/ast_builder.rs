// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/ast_builder.rs`.

//! AST node builder methods.
//!
//! Each method is generic over [`GetAstBuilder`], so it can be called with a builder
//! directly, or with a type which holds one (e.g. the parser).
//!
//! Before forwarding the builder to another build method, methods first call `builder.builder()`
//! to obtain the concrete [`AstBuild`]er. This way, everything below the outermost call is
//! monomorphized over the [`AstBuild`] type (of which there are few), rather than over every
//! [`GetAstBuilder`] type the method is called with.

#![expect(clippy::default_trait_access)]

use std::cell::Cell;

use robusta_allocator::{Box, GetAllocator, IntoIn, Vec};
use robusta_span::Span;
use robusta_syntax::operator::{AssignOp, BinaryOp, PostfixOp, UnaryOp};

use crate::{
    ast::*,
    builder::{AstBuild, GetAstBuilder},
};

impl<'a> CompilationUnit<'a> {
    /// Build a [`CompilationUnit`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `comments`: Source-ordered comments, skipped by visitors.
    /// * `content`: Exactly one compilation-unit form (JLS §19).
    #[inline]
    pub fn new(
        span: Span,
        comments: impl IntoIn<'a, Vec<'a, Comment>>,
        content: CompilationUnitContent<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        CompilationUnit {
            node_id: Cell::new(builder.node_id()),
            span,
            comments: comments.into_in(builder.allocator()),
            content,
            scope_id: Default::default(),
        }
    }
}

impl<'a> CompilationUnitContent<'a> {
    /// Build a [`CompilationUnitContent::Ordinary`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `package`
    /// * `imports`
    /// * `types`
    #[inline]
    pub fn new_ordinary_compilation_unit(
        span: Span,
        package: Option<PackageDeclaration<'a>>,
        imports: impl IntoIn<'a, Vec<'a, ImportDeclaration<'a>>>,
        types: impl IntoIn<'a, Vec<'a, TypeDeclaration<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Ordinary(OrdinaryCompilationUnit::new(
            span,
            package,
            imports,
            types,
            builder.builder(),
        ))
    }

    /// Build a [`CompilationUnitContent::Compact`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `imports`
    /// * `members`
    #[inline]
    pub fn new_compact_compilation_unit(
        span: Span,
        imports: impl IntoIn<'a, Vec<'a, ImportDeclaration<'a>>>,
        members: impl IntoIn<'a, Vec<'a, ClassMember<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Compact(CompactCompilationUnit::new(span, imports, members, builder.builder()))
    }

    /// Build a [`CompilationUnitContent::Modular`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `imports`
    /// * `module`
    #[inline]
    pub fn new_modular_compilation_unit(
        span: Span,
        imports: impl IntoIn<'a, Vec<'a, ImportDeclaration<'a>>>,
        module: ModuleDeclaration<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Modular(ModularCompilationUnit::new(span, imports, module, builder.builder()))
    }
}

impl<'a> OrdinaryCompilationUnit<'a> {
    /// Build an [`OrdinaryCompilationUnit`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `package`
    /// * `imports`
    /// * `types`
    #[inline]
    pub fn new(
        span: Span,
        package: Option<PackageDeclaration<'a>>,
        imports: impl IntoIn<'a, Vec<'a, ImportDeclaration<'a>>>,
        types: impl IntoIn<'a, Vec<'a, TypeDeclaration<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        OrdinaryCompilationUnit {
            node_id: Cell::new(builder.node_id()),
            span,
            package,
            imports: imports.into_in(builder.allocator()),
            types: types.into_in(builder.allocator()),
        }
    }
}

impl<'a> ModularCompilationUnit<'a> {
    /// Build a [`ModularCompilationUnit`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `imports`
    /// * `module`
    #[inline]
    pub fn new(
        span: Span,
        imports: impl IntoIn<'a, Vec<'a, ImportDeclaration<'a>>>,
        module: ModuleDeclaration<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ModularCompilationUnit {
            node_id: Cell::new(builder.node_id()),
            span,
            imports: imports.into_in(builder.allocator()),
            module,
        }
    }
}

impl<'a> PackageDeclaration<'a> {
    /// Build a [`PackageDeclaration`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`: Package annotations (JLS §7.4.1), e.g. `@NullMarked package com.example;`.
    /// * `name`
    #[inline]
    pub fn new(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: Name<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        PackageDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            annotations: annotations.into_in(builder.allocator()),
            name,
        }
    }
}

impl<'a> ImportDeclaration<'a> {
    /// Build an [`ImportDeclaration`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `kind`
    /// * `name`
    /// * `local`: The last name, which a single-type or single-static import also binds.
    #[inline]
    pub fn new(
        span: Span,
        kind: ImportKind,
        name: Name<'a>,
        local: Option<BindingIdentifier<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ImportDeclaration { node_id: Cell::new(builder.node_id()), span, kind, name, local }
    }
}

impl<'a> CompactCompilationUnit<'a> {
    /// Build a [`CompactCompilationUnit`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `imports`
    /// * `members`
    #[inline]
    pub fn new(
        span: Span,
        imports: impl IntoIn<'a, Vec<'a, ImportDeclaration<'a>>>,
        members: impl IntoIn<'a, Vec<'a, ClassMember<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        CompactCompilationUnit {
            node_id: Cell::new(builder.node_id()),
            span,
            imports: imports.into_in(builder.allocator()),
            members: members.into_in(builder.allocator()),
            scope_id: Default::default(),
        }
    }
}

impl<'a> ModuleDeclaration<'a> {
    /// Build a [`ModuleDeclaration`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`: Annotations preceding `[open] module`.
    /// * `is_open`: Whether this is `open module`.
    /// * `name`
    /// * `directives`
    #[inline]
    pub fn new(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        is_open: bool,
        name: Name<'a>,
        directives: impl IntoIn<'a, Vec<'a, ModuleDirective<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ModuleDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            annotations: annotations.into_in(builder.allocator()),
            is_open,
            name,
            directives: directives.into_in(builder.allocator()),
        }
    }
}

impl<'a> ModuleDirective<'a> {
    /// Build a [`ModuleDirective::Requires`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`: `transitive`/`static` modifiers in source order, including duplicates.
    /// * `name`
    #[inline]
    pub fn new_requires_directive(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, RequiresModifier>>,
        name: Name<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Requires(RequiresDirective::new(span, modifiers, name, builder.builder()))
    }

    /// Build a [`ModuleDirective::Package`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `kind`
    /// * `name`
    /// * `targets`
    #[inline]
    pub fn new_package_directive(
        span: Span,
        kind: PackageDirectiveKind,
        name: Name<'a>,
        targets: impl IntoIn<'a, Vec<'a, Name<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Package(PackageDirective::new(span, kind, name, targets, builder.builder()))
    }

    /// Build a [`ModuleDirective::Uses`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    #[inline]
    pub fn new_uses_directive(
        span: Span,
        name: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Uses(UsesDirective::new(span, name, builder.builder()))
    }

    /// Build a [`ModuleDirective::Provides`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `with`: The non-empty `with` implementation-type list.
    #[inline]
    pub fn new_provides_directive(
        span: Span,
        name: TypeName<'a>,
        with: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Provides(ProvidesDirective::new(span, name, with, builder.builder()))
    }
}

impl<'a> RequiresDirective<'a> {
    /// Build a [`RequiresDirective`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`: `transitive`/`static` modifiers in source order, including duplicates.
    /// * `name`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, RequiresModifier>>,
        name: Name<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        RequiresDirective {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            name,
        }
    }
}

impl RequiresModifier {
    /// Build a [`RequiresModifier`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `kind`
    #[expect(unused_variables)]
    #[inline]
    pub fn new<'a>(
        span: Span,
        kind: RequiresModifierKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        RequiresModifier { span, kind }
    }
}

impl<'a> PackageDirective<'a> {
    /// Build a [`PackageDirective`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `kind`
    /// * `name`
    /// * `targets`
    #[inline]
    pub fn new(
        span: Span,
        kind: PackageDirectiveKind,
        name: Name<'a>,
        targets: impl IntoIn<'a, Vec<'a, Name<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        PackageDirective {
            node_id: Cell::new(builder.node_id()),
            span,
            kind,
            name,
            targets: targets.into_in(builder.allocator()),
        }
    }
}

impl<'a> UsesDirective<'a> {
    /// Build a [`UsesDirective`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    #[inline]
    pub fn new(span: Span, name: TypeName<'a>, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        UsesDirective { node_id: Cell::new(builder.node_id()), span, name }
    }
}

impl<'a> ProvidesDirective<'a> {
    /// Build a [`ProvidesDirective`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `with`: The non-empty `with` implementation-type list.
    #[inline]
    pub fn new(
        span: Span,
        name: TypeName<'a>,
        with: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ProvidesDirective {
            node_id: Cell::new(builder.node_id()),
            span,
            name,
            with: with.into_in(builder.allocator()),
        }
    }
}

impl<'a> TypeDeclaration<'a> {
    /// Build a [`TypeDeclaration::Empty`].
    ///
    /// This node contains an [`EmptyDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_empty_declaration(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Empty(EmptyDeclaration::boxed(span, builder.builder()))
    }

    /// Build a [`TypeDeclaration::AnnotationInterface`].
    ///
    /// This node contains an [`AnnotationInterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `body`
    #[inline]
    pub fn new_annotation_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        body: AnnotationInterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::AnnotationInterface(AnnotationInterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`TypeDeclaration::Class`].
    ///
    /// This node contains a [`ClassDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `superclass`
    /// * `interfaces`
    /// * `permits`: Direct subclasses in `permits`, empty if absent (JLS §8.1.6).
    /// * `body`
    #[inline]
    pub fn new_class_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        superclass: Option<Type<'a>>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Class(ClassDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            superclass,
            interfaces,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`TypeDeclaration::Interface`].
    ///
    /// This node contains an [`InterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `extends`: Superinterfaces listed after `extends`.
    /// * `permits`: Direct subtypes in `permits`, empty if absent (JLS §9.1.4).
    /// * `body`
    #[inline]
    pub fn new_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        extends: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: InterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Interface(InterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            extends,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`TypeDeclaration::Enum`].
    ///
    /// This node contains an [`EnumDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_enum_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: EnumBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Enum(EnumDeclaration::boxed(
            span,
            modifiers,
            name,
            interfaces,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`TypeDeclaration::Record`].
    ///
    /// This node contains a [`RecordDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `header`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_record_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        header: RecordHeader<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Record(RecordDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            header,
            interfaces,
            body,
            builder.builder(),
        ))
    }
}

impl<'a> LocalClassOrInterfaceDeclaration<'a> {
    /// Build a [`LocalClassOrInterfaceDeclaration::Class`].
    ///
    /// This node contains a [`ClassDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `superclass`
    /// * `interfaces`
    /// * `permits`: Direct subclasses in `permits`, empty if absent (JLS §8.1.6).
    /// * `body`
    #[inline]
    pub fn new_class_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        superclass: Option<Type<'a>>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Class(ClassDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            superclass,
            interfaces,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`LocalClassOrInterfaceDeclaration::Interface`].
    ///
    /// This node contains an [`InterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `extends`: Superinterfaces listed after `extends`.
    /// * `permits`: Direct subtypes in `permits`, empty if absent (JLS §9.1.4).
    /// * `body`
    #[inline]
    pub fn new_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        extends: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: InterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Interface(InterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            extends,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`LocalClassOrInterfaceDeclaration::Enum`].
    ///
    /// This node contains an [`EnumDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_enum_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: EnumBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Enum(EnumDeclaration::boxed(
            span,
            modifiers,
            name,
            interfaces,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`LocalClassOrInterfaceDeclaration::Record`].
    ///
    /// This node contains a [`RecordDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `header`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_record_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        header: RecordHeader<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Record(RecordDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            header,
            interfaces,
            body,
            builder.builder(),
        ))
    }
}

impl EmptyDeclaration {
    /// Build an [`EmptyDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`EmptyDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        EmptyDeclaration { node_id: Cell::new(builder.node_id()), span }
    }

    /// Build an [`EmptyDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`EmptyDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn boxed<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, builder), &builder.allocator())
    }
}

impl<'a> ClassDeclaration<'a> {
    /// Build a [`ClassDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ClassDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `superclass`
    /// * `interfaces`
    /// * `permits`: Direct subclasses in `permits`, empty if absent (JLS §8.1.6).
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        superclass: Option<Type<'a>>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ClassDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            name,
            type_params,
            superclass,
            interfaces: interfaces.into_in(builder.allocator()),
            permits: permits.into_in(builder.allocator()),
            body,
            scope_id: Default::default(),
        }
    }

    /// Build a [`ClassDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ClassDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `superclass`
    /// * `interfaces`
    /// * `permits`: Direct subclasses in `permits`, empty if absent (JLS §8.1.6).
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        superclass: Option<Type<'a>>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(
                span,
                modifiers,
                name,
                type_params,
                superclass,
                interfaces,
                permits,
                body,
                builder,
            ),
            &builder.allocator(),
        )
    }
}

impl<'a> ClassBody<'a> {
    /// Build a [`ClassBody`].
    ///
    /// ## Parameters
    /// * `span`: Covers the opening `{` through the closing `}`.
    /// * `members`
    #[inline]
    pub fn new(
        span: Span,
        members: impl IntoIn<'a, Vec<'a, ClassMember<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ClassBody {
            node_id: Cell::new(builder.node_id()),
            span,
            members: members.into_in(builder.allocator()),
            scope_id: Default::default(),
        }
    }
}

impl<'a> InterfaceDeclaration<'a> {
    /// Build an [`InterfaceDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`InterfaceDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `extends`: Superinterfaces listed after `extends`.
    /// * `permits`: Direct subtypes in `permits`, empty if absent (JLS §9.1.4).
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        extends: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: InterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        InterfaceDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            name,
            type_params,
            extends: extends.into_in(builder.allocator()),
            permits: permits.into_in(builder.allocator()),
            body,
            scope_id: Default::default(),
        }
    }

    /// Build an [`InterfaceDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`InterfaceDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `extends`: Superinterfaces listed after `extends`.
    /// * `permits`: Direct subtypes in `permits`, empty if absent (JLS §9.1.4).
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        extends: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: InterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, modifiers, name, type_params, extends, permits, body, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> InterfaceBody<'a> {
    /// Build an [`InterfaceBody`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `members`
    #[inline]
    pub fn new(
        span: Span,
        members: impl IntoIn<'a, Vec<'a, InterfaceMember<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        InterfaceBody {
            node_id: Cell::new(builder.node_id()),
            span,
            members: members.into_in(builder.allocator()),
            scope_id: Default::default(),
        }
    }
}

impl<'a> EnumDeclaration<'a> {
    /// Build an [`EnumDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`EnumDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: EnumBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        EnumDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            name,
            interfaces: interfaces.into_in(builder.allocator()),
            body,
        }
    }

    /// Build an [`EnumDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`EnumDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: EnumBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, modifiers, name, interfaces, body, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> EnumBody<'a> {
    /// Build an [`EnumBody`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constants`
    /// * `has_body_semicolon`: Whether the body has the `;` that ends its constants, required before members (JLS §8.9.1).
    /// * `members`
    #[inline]
    pub fn new(
        span: Span,
        constants: impl IntoIn<'a, Vec<'a, EnumConstant<'a>>>,
        has_body_semicolon: bool,
        members: impl IntoIn<'a, Vec<'a, ClassMember<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        EnumBody {
            node_id: Cell::new(builder.node_id()),
            span,
            constants: constants.into_in(builder.allocator()),
            has_body_semicolon,
            members: members.into_in(builder.allocator()),
            scope_id: Default::default(),
        }
    }
}

impl<'a> EnumConstant<'a> {
    /// Build an [`EnumConstant`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`: Annotation-only modifiers, using the shared modifier container.
    /// * `name`
    /// * `args`: `None` for `A`, `Some([])` for `A()`.
    /// * `body`: `None` for `A`, empty members for `A {}` (JLS §8.9.1).
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        args: Option<Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        EnumConstant {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            name,
            args,
            body,
        }
    }
}

impl<'a> RecordDeclaration<'a> {
    /// Build a [`RecordDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`RecordDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `header`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        header: RecordHeader<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        RecordDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            name,
            type_params,
            header,
            interfaces: interfaces.into_in(builder.allocator()),
            body,
            scope_id: Default::default(),
        }
    }

    /// Build a [`RecordDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`RecordDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `header`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        header: RecordHeader<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, modifiers, name, type_params, header, interfaces, body, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> RecordHeader<'a> {
    /// Build a [`RecordHeader`].
    ///
    /// ## Parameters
    /// * `span`: From `(` to `)`.
    /// * `components`
    #[inline]
    pub fn new(
        span: Span,
        components: impl IntoIn<'a, Vec<'a, RecordComponent<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        RecordHeader {
            node_id: Cell::new(builder.node_id()),
            span,
            components: components.into_in(builder.allocator()),
        }
    }
}

impl<'a> RecordComponent<'a> {
    /// Build a [`RecordComponent`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `varargs`
    /// * `varargs_annotations`
    /// * `name`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        varargs: bool,
        varargs_annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: BindingIdentifier<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        RecordComponent {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            ty,
            varargs,
            varargs_annotations: varargs_annotations.into_in(builder.allocator()),
            name,
        }
    }
}

impl<'a> AnnotationInterfaceDeclaration<'a> {
    /// Build an [`AnnotationInterfaceDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`AnnotationInterfaceDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        body: AnnotationInterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        AnnotationInterfaceDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            name,
            body,
        }
    }

    /// Build an [`AnnotationInterfaceDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`AnnotationInterfaceDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        body: AnnotationInterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, modifiers, name, body, builder), &builder.allocator())
    }
}

impl<'a> AnnotationInterfaceBody<'a> {
    /// Build an [`AnnotationInterfaceBody`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `members`
    #[inline]
    pub fn new(
        span: Span,
        members: impl IntoIn<'a, Vec<'a, AnnotationInterfaceMemberDeclaration<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        AnnotationInterfaceBody {
            node_id: Cell::new(builder.node_id()),
            span,
            members: members.into_in(builder.allocator()),
            scope_id: Default::default(),
        }
    }
}

impl<'a> AnnotationInterfaceMemberDeclaration<'a> {
    /// Build an [`AnnotationInterfaceMemberDeclaration::Element`].
    ///
    /// This node contains an [`AnnotationInterfaceElementDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    /// * `post_paren_dimensions`
    /// * `default_value`
    #[inline]
    pub fn new_annotation_interface_element_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        name: BindingIdentifier<'a>,
        post_paren_dimensions: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        default_value: Option<ElementValue<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Element(AnnotationInterfaceElementDeclaration::boxed(
            span,
            modifiers,
            ty,
            name,
            post_paren_dimensions,
            default_value,
            builder.builder(),
        ))
    }

    /// Build an [`AnnotationInterfaceMemberDeclaration::Field`].
    ///
    /// This node contains a [`FieldDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `declarators`
    #[inline]
    pub fn new_field_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        declarators: impl IntoIn<'a, Vec<'a, VariableDeclarator<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Field(FieldDeclaration::boxed(span, modifiers, ty, declarators, builder.builder()))
    }

    /// Build an [`AnnotationInterfaceMemberDeclaration::Empty`].
    ///
    /// This node contains an [`EmptyDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_empty_declaration(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Empty(EmptyDeclaration::boxed(span, builder.builder()))
    }

    /// Build an [`AnnotationInterfaceMemberDeclaration::AnnotationInterface`].
    ///
    /// This node contains an [`AnnotationInterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `body`
    #[inline]
    pub fn new_annotation_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        body: AnnotationInterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::AnnotationInterface(AnnotationInterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`AnnotationInterfaceMemberDeclaration::Class`].
    ///
    /// This node contains a [`ClassDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `superclass`
    /// * `interfaces`
    /// * `permits`: Direct subclasses in `permits`, empty if absent (JLS §8.1.6).
    /// * `body`
    #[inline]
    pub fn new_class_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        superclass: Option<Type<'a>>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Class(ClassDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            superclass,
            interfaces,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`AnnotationInterfaceMemberDeclaration::Interface`].
    ///
    /// This node contains an [`InterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `extends`: Superinterfaces listed after `extends`.
    /// * `permits`: Direct subtypes in `permits`, empty if absent (JLS §9.1.4).
    /// * `body`
    #[inline]
    pub fn new_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        extends: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: InterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Interface(InterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            extends,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`AnnotationInterfaceMemberDeclaration::Enum`].
    ///
    /// This node contains an [`EnumDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_enum_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: EnumBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Enum(EnumDeclaration::boxed(
            span,
            modifiers,
            name,
            interfaces,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`AnnotationInterfaceMemberDeclaration::Record`].
    ///
    /// This node contains a [`RecordDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `header`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_record_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        header: RecordHeader<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Record(RecordDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            header,
            interfaces,
            body,
            builder.builder(),
        ))
    }
}

impl<'a> AnnotationInterfaceElementDeclaration<'a> {
    /// Build an [`AnnotationInterfaceElementDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`AnnotationInterfaceElementDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    /// * `post_paren_dimensions`
    /// * `default_value`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        name: BindingIdentifier<'a>,
        post_paren_dimensions: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        default_value: Option<ElementValue<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        AnnotationInterfaceElementDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            ty,
            name,
            post_paren_dimensions: post_paren_dimensions.into_in(builder.allocator()),
            default_value,
        }
    }

    /// Build an [`AnnotationInterfaceElementDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`AnnotationInterfaceElementDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    /// * `post_paren_dimensions`
    /// * `default_value`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        name: BindingIdentifier<'a>,
        post_paren_dimensions: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        default_value: Option<ElementValue<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, modifiers, ty, name, post_paren_dimensions, default_value, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> ClassMember<'a> {
    /// Build a [`ClassMember::Field`].
    ///
    /// This node contains a [`FieldDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `declarators`
    #[inline]
    pub fn new_field_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        declarators: impl IntoIn<'a, Vec<'a, VariableDeclarator<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Field(FieldDeclaration::boxed(span, modifiers, ty, declarators, builder.builder()))
    }

    /// Build a [`ClassMember::Method`].
    ///
    /// This node contains a [`MethodDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `type_params`
    /// * `return_type`
    /// * `name`
    /// * `params`
    /// * `post_paren_dimensions`: Post-parameter dimensions, e.g. `int f()[]` (JLS §8.4).
    /// * `throws`: Exception types with type-use annotations, e.g. `throws @Critical IOException` (JLS §8.4.6).
    /// * `body`: `None` for a bodyless declaration (`abstract void f();`).
    #[inline]
    pub fn new_method_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        return_type: MethodResult<'a>,
        name: BindingIdentifier<'a>,
        params: FormalParameters<'a>,
        post_paren_dimensions: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        throws: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: Option<Block<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Method(MethodDeclaration::boxed(
            span,
            modifiers,
            type_params,
            return_type,
            name,
            params,
            post_paren_dimensions,
            throws,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`ClassMember::Constructor`].
    ///
    /// This node contains a [`ConstructorDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `type_params`
    /// * `name`
    /// * `params`
    /// * `throws`: See [`MethodDeclaration::throws`].
    /// * `body`
    #[inline]
    pub fn new_constructor_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        name: IdentifierName<'a>,
        params: FormalParameters<'a>,
        throws: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Constructor(ConstructorDeclaration::boxed(
            span,
            modifiers,
            type_params,
            name,
            params,
            throws,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`ClassMember::CompactConstructor`].
    ///
    /// This node contains a [`CompactConstructorDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `body`
    #[inline]
    pub fn new_compact_constructor_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: IdentifierName<'a>,
        body: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::CompactConstructor(CompactConstructorDeclaration::boxed(
            span,
            modifiers,
            name,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`ClassMember::Initializer`].
    ///
    /// This node contains an [`InitializerBlock`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `is_static`: Whether this is `static { … }`.
    /// * `block`
    #[inline]
    pub fn new_initializer_block(
        span: Span,
        is_static: bool,
        block: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Initializer(InitializerBlock::boxed(span, is_static, block, builder.builder()))
    }

    /// Build a [`ClassMember::Empty`].
    ///
    /// This node contains an [`EmptyDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_empty_declaration(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Empty(EmptyDeclaration::boxed(span, builder.builder()))
    }

    /// Build a [`ClassMember::AnnotationInterface`].
    ///
    /// This node contains an [`AnnotationInterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `body`
    #[inline]
    pub fn new_annotation_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        body: AnnotationInterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::AnnotationInterface(AnnotationInterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`ClassMember::Class`].
    ///
    /// This node contains a [`ClassDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `superclass`
    /// * `interfaces`
    /// * `permits`: Direct subclasses in `permits`, empty if absent (JLS §8.1.6).
    /// * `body`
    #[inline]
    pub fn new_class_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        superclass: Option<Type<'a>>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Class(ClassDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            superclass,
            interfaces,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`ClassMember::Interface`].
    ///
    /// This node contains an [`InterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `extends`: Superinterfaces listed after `extends`.
    /// * `permits`: Direct subtypes in `permits`, empty if absent (JLS §9.1.4).
    /// * `body`
    #[inline]
    pub fn new_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        extends: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: InterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Interface(InterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            extends,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`ClassMember::Enum`].
    ///
    /// This node contains an [`EnumDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_enum_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: EnumBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Enum(EnumDeclaration::boxed(
            span,
            modifiers,
            name,
            interfaces,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`ClassMember::Record`].
    ///
    /// This node contains a [`RecordDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `header`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_record_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        header: RecordHeader<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Record(RecordDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            header,
            interfaces,
            body,
            builder.builder(),
        ))
    }
}

impl<'a> InterfaceMember<'a> {
    /// Build an [`InterfaceMember::Constant`].
    ///
    /// This node contains a [`FieldDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `declarators`
    #[inline]
    pub fn new_field_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        declarators: impl IntoIn<'a, Vec<'a, VariableDeclarator<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Constant(FieldDeclaration::boxed(span, modifiers, ty, declarators, builder.builder()))
    }

    /// Build an [`InterfaceMember::Method`].
    ///
    /// This node contains a [`MethodDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `type_params`
    /// * `return_type`
    /// * `name`
    /// * `params`
    /// * `post_paren_dimensions`: Post-parameter dimensions, e.g. `int f()[]` (JLS §8.4).
    /// * `throws`: Exception types with type-use annotations, e.g. `throws @Critical IOException` (JLS §8.4.6).
    /// * `body`: `None` for a bodyless declaration (`abstract void f();`).
    #[inline]
    pub fn new_method_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        return_type: MethodResult<'a>,
        name: BindingIdentifier<'a>,
        params: FormalParameters<'a>,
        post_paren_dimensions: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        throws: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: Option<Block<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Method(MethodDeclaration::boxed(
            span,
            modifiers,
            type_params,
            return_type,
            name,
            params,
            post_paren_dimensions,
            throws,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`InterfaceMember::Empty`].
    ///
    /// This node contains an [`EmptyDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_empty_declaration(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Empty(EmptyDeclaration::boxed(span, builder.builder()))
    }

    /// Build an [`InterfaceMember::AnnotationInterface`].
    ///
    /// This node contains an [`AnnotationInterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `body`
    #[inline]
    pub fn new_annotation_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        body: AnnotationInterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::AnnotationInterface(AnnotationInterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`InterfaceMember::Class`].
    ///
    /// This node contains a [`ClassDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `superclass`
    /// * `interfaces`
    /// * `permits`: Direct subclasses in `permits`, empty if absent (JLS §8.1.6).
    /// * `body`
    #[inline]
    pub fn new_class_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        superclass: Option<Type<'a>>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Class(ClassDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            superclass,
            interfaces,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`InterfaceMember::Interface`].
    ///
    /// This node contains an [`InterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `extends`: Superinterfaces listed after `extends`.
    /// * `permits`: Direct subtypes in `permits`, empty if absent (JLS §9.1.4).
    /// * `body`
    #[inline]
    pub fn new_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        extends: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: InterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Interface(InterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            extends,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`InterfaceMember::Enum`].
    ///
    /// This node contains an [`EnumDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_enum_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: EnumBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Enum(EnumDeclaration::boxed(
            span,
            modifiers,
            name,
            interfaces,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`InterfaceMember::Record`].
    ///
    /// This node contains a [`RecordDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `header`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_record_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        header: RecordHeader<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Record(RecordDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            header,
            interfaces,
            body,
            builder.builder(),
        ))
    }
}

impl<'a> InitializerBlock<'a> {
    /// Build an [`InitializerBlock`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`InitializerBlock::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `is_static`: Whether this is `static { … }`.
    /// * `block`
    #[inline]
    pub fn new(
        span: Span,
        is_static: bool,
        block: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        InitializerBlock { node_id: Cell::new(builder.node_id()), span, is_static, block }
    }

    /// Build an [`InitializerBlock`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`InitializerBlock::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `is_static`: Whether this is `static { … }`.
    /// * `block`
    #[inline]
    pub fn boxed(
        span: Span,
        is_static: bool,
        block: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, is_static, block, builder), &builder.allocator())
    }
}

impl<'a> FieldDeclaration<'a> {
    /// Build a [`FieldDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`FieldDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `declarators`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        declarators: impl IntoIn<'a, Vec<'a, VariableDeclarator<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        FieldDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            ty,
            declarators: declarators.into_in(builder.allocator()),
        }
    }

    /// Build a [`FieldDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`FieldDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `declarators`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        declarators: impl IntoIn<'a, Vec<'a, VariableDeclarator<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, modifiers, ty, declarators, builder), &builder.allocator())
    }
}

impl<'a> MethodDeclaration<'a> {
    /// Build a [`MethodDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`MethodDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `type_params`
    /// * `return_type`
    /// * `name`
    /// * `params`
    /// * `post_paren_dimensions`: Post-parameter dimensions, e.g. `int f()[]` (JLS §8.4).
    /// * `throws`: Exception types with type-use annotations, e.g. `throws @Critical IOException` (JLS §8.4.6).
    /// * `body`: `None` for a bodyless declaration (`abstract void f();`).
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        return_type: MethodResult<'a>,
        name: BindingIdentifier<'a>,
        params: FormalParameters<'a>,
        post_paren_dimensions: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        throws: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: Option<Block<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        MethodDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            type_params,
            return_type,
            name,
            params,
            post_paren_dimensions: post_paren_dimensions.into_in(builder.allocator()),
            throws: throws.into_in(builder.allocator()),
            body,
            scope_id: Default::default(),
        }
    }

    /// Build a [`MethodDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`MethodDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `type_params`
    /// * `return_type`
    /// * `name`
    /// * `params`
    /// * `post_paren_dimensions`: Post-parameter dimensions, e.g. `int f()[]` (JLS §8.4).
    /// * `throws`: Exception types with type-use annotations, e.g. `throws @Critical IOException` (JLS §8.4.6).
    /// * `body`: `None` for a bodyless declaration (`abstract void f();`).
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        return_type: MethodResult<'a>,
        name: BindingIdentifier<'a>,
        params: FormalParameters<'a>,
        post_paren_dimensions: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        throws: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: Option<Block<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(
                span,
                modifiers,
                type_params,
                return_type,
                name,
                params,
                post_paren_dimensions,
                throws,
                body,
                builder,
            ),
            &builder.allocator(),
        )
    }
}

impl<'a> ConstructorDeclaration<'a> {
    /// Build a [`ConstructorDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ConstructorDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `type_params`
    /// * `name`
    /// * `params`
    /// * `throws`: See [`MethodDeclaration::throws`].
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        name: IdentifierName<'a>,
        params: FormalParameters<'a>,
        throws: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ConstructorDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            type_params,
            name,
            params,
            throws: throws.into_in(builder.allocator()),
            body,
            scope_id: Default::default(),
        }
    }

    /// Build a [`ConstructorDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ConstructorDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `type_params`
    /// * `name`
    /// * `params`
    /// * `throws`: See [`MethodDeclaration::throws`].
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        name: IdentifierName<'a>,
        params: FormalParameters<'a>,
        throws: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, modifiers, type_params, name, params, throws, body, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> CompactConstructorDeclaration<'a> {
    /// Build a [`CompactConstructorDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`CompactConstructorDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: IdentifierName<'a>,
        body: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        CompactConstructorDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            name,
            body,
            scope_id: Default::default(),
        }
    }

    /// Build a [`CompactConstructorDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`CompactConstructorDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: IdentifierName<'a>,
        body: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, modifiers, name, body, builder), &builder.allocator())
    }
}

impl<'a> ReceiverParameter<'a> {
    /// Build a [`ReceiverParameter`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ReceiverParameter::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `qualifier`: `Outer` in `Outer.this`.
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        qualifier: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ReceiverParameter {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            ty,
            qualifier,
        }
    }

    /// Build a [`ReceiverParameter`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ReceiverParameter::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `qualifier`: `Outer` in `Outer.this`.
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Type<'a>,
        qualifier: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, modifiers, ty, qualifier, builder), &builder.allocator())
    }
}

impl<'a> FormalParameters<'a> {
    /// Build a [`FormalParameters`].
    ///
    /// ## Parameters
    /// * `span`: From `(` to `)`, or the parameter of a concise lambda `x -> …`.
    /// * `receiver`: The receiver parameter, written before the others.
    /// * `items`
    #[inline]
    pub fn new(
        span: Span,
        receiver: Option<Box<'a, ReceiverParameter<'a>>>,
        items: impl IntoIn<'a, Vec<'a, FormalParameter<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        FormalParameters {
            node_id: Cell::new(builder.node_id()),
            span,
            receiver,
            items: items.into_in(builder.allocator()),
        }
    }
}

impl<'a> FormalParameter<'a> {
    /// Build a [`FormalParameter`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`: `None` for an untyped lambda parameter. Only a lambda parameter may be `var`.
    /// * `varargs`: `T... name`, permitted only as the final parameter.
    /// * `varargs_annotations`: Annotations on `...`, e.g. `Object @Nullable ...`. Empty unless `varargs`.
    /// * `name`
    /// * `dims`: See [`VariableDeclarator::dims`]. Empty when `varargs`.
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: Option<VariableType<'a>>,
        varargs: bool,
        varargs_annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: BindingIdentifier<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        FormalParameter {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            ty,
            varargs,
            varargs_annotations: varargs_annotations.into_in(builder.allocator()),
            name,
            dims: dims.into_in(builder.allocator()),
        }
    }
}

impl<'a> Block<'a> {
    /// Build a [`Block`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`Block::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `statements`
    #[inline]
    pub fn new(
        span: Span,
        statements: impl IntoIn<'a, Vec<'a, Statement<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        Block {
            node_id: Cell::new(builder.node_id()),
            span,
            statements: statements.into_in(builder.allocator()),
            scope_id: Default::default(),
        }
    }

    /// Build a [`Block`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`Block::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `statements`
    #[inline]
    pub fn boxed(
        span: Span,
        statements: impl IntoIn<'a, Vec<'a, Statement<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, statements, builder), &builder.allocator())
    }
}

impl<'a> Statement<'a> {
    /// Build a [`Statement::Block`].
    ///
    /// This node contains a [`Block`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `statements`
    #[inline]
    pub fn new_block(
        span: Span,
        statements: impl IntoIn<'a, Vec<'a, Statement<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Block(Block::boxed(span, statements, builder.builder()))
    }

    /// Build a [`Statement::Empty`].
    ///
    /// This node contains an [`EmptyStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_empty_statement(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Empty(EmptyStatement::boxed(span, builder.builder()))
    }

    /// Build a [`Statement::ConstructorInvocation`].
    ///
    /// This node contains a [`ConstructorInvocation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`: The qualifier in `outer.super(…)`, absent for direct invocations (JLS §8.8.7.1).
    /// * `type_args`: Type arguments in `<T>this(…)` or `[outer.]<T>super(…)`.
    /// * `kind`
    /// * `args`
    #[inline]
    pub fn new_constructor_invocation(
        span: Span,
        qualifier: Option<Expression<'a>>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        kind: ConstructorInvocationKind,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ConstructorInvocation(ConstructorInvocation::boxed(
            span,
            qualifier,
            type_args,
            kind,
            args,
            builder.builder(),
        ))
    }

    /// Build a [`Statement::LocalVariable`].
    ///
    /// This node contains a [`LocalVariableDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `declarators`
    #[inline]
    pub fn new_local_variable_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        declarators: impl IntoIn<'a, Vec<'a, VariableDeclarator<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::LocalVariable(LocalVariableDeclaration::boxed(
            span,
            modifiers,
            ty,
            declarators,
            builder.builder(),
        ))
    }

    /// Build a [`Statement::Expression`].
    ///
    /// This node contains an [`ExpressionStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new_expression_statement(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Expression(ExpressionStatement::boxed(span, expression, builder.builder()))
    }

    /// Build a [`Statement::If`].
    ///
    /// This node contains an [`IfStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `then_branch`
    /// * `else_branch`
    #[inline]
    pub fn new_if_statement(
        span: Span,
        condition: Expression<'a>,
        then_branch: Statement<'a>,
        else_branch: Option<Statement<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::If(IfStatement::boxed(span, condition, then_branch, else_branch, builder.builder()))
    }

    /// Build a [`Statement::Switch`].
    ///
    /// This node contains a [`SwitchStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new_switch_statement(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Switch(SwitchStatement::boxed(span, selector, cases, builder.builder()))
    }

    /// Build a [`Statement::While`].
    ///
    /// This node contains a [`WhileStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `body`
    #[inline]
    pub fn new_while_statement(
        span: Span,
        condition: Expression<'a>,
        body: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::While(WhileStatement::boxed(span, condition, body, builder.builder()))
    }

    /// Build a [`Statement::Do`].
    ///
    /// This node contains a [`DoStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `body`
    /// * `condition`
    #[inline]
    pub fn new_do_statement(
        span: Span,
        body: Statement<'a>,
        condition: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Do(DoStatement::boxed(span, body, condition, builder.builder()))
    }

    /// Build a [`Statement::For`].
    ///
    /// This node contains a [`ForStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `init`
    /// * `condition`
    /// * `update`
    /// * `body`
    #[inline]
    pub fn new_for_statement(
        span: Span,
        init: Option<ForInit<'a>>,
        condition: Option<Expression<'a>>,
        update: Option<StatementExpressionList<'a>>,
        body: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::For(ForStatement::boxed(span, init, condition, update, body, builder.builder()))
    }

    /// Build a [`Statement::EnhancedFor`].
    ///
    /// This node contains an [`EnhancedForStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    /// * `dims`: See [`VariableDeclarator::dims`].
    /// * `iterable`
    /// * `body`
    #[inline]
    pub fn new_enhanced_for_statement(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        iterable: Expression<'a>,
        body: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::EnhancedFor(EnhancedForStatement::boxed(
            span,
            modifiers,
            ty,
            name,
            dims,
            iterable,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`Statement::Break`].
    ///
    /// This node contains a [`BreakStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    #[inline]
    pub fn new_break_statement(
        span: Span,
        label: Option<LabelIdentifier<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Break(BreakStatement::boxed(span, label, builder.builder()))
    }

    /// Build a [`Statement::Continue`].
    ///
    /// This node contains a [`ContinueStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    #[inline]
    pub fn new_continue_statement(
        span: Span,
        label: Option<LabelIdentifier<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Continue(ContinueStatement::boxed(span, label, builder.builder()))
    }

    /// Build a [`Statement::Throw`].
    ///
    /// This node contains a [`ThrowStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_throw_statement(
        span: Span,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Throw(ThrowStatement::boxed(span, value, builder.builder()))
    }

    /// Build a [`Statement::Assert`].
    ///
    /// This node contains an [`AssertStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `message`
    #[inline]
    pub fn new_assert_statement(
        span: Span,
        condition: Expression<'a>,
        message: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Assert(AssertStatement::boxed(span, condition, message, builder.builder()))
    }

    /// Build a [`Statement::Try`].
    ///
    /// This node contains a [`TryStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `resources`: Resource specifications, empty for a classic `try` (JLS §14.20.3).
    /// * `resources_trailing_semicolon`: Preserved trailing `;` in `try (r;)`, meaningful only with resources.
    /// * `block`
    /// * `catches`
    /// * `finally_block`
    #[inline]
    pub fn new_try_statement(
        span: Span,
        resources: impl IntoIn<'a, Vec<'a, Resource<'a>>>,
        resources_trailing_semicolon: bool,
        block: Block<'a>,
        catches: impl IntoIn<'a, Vec<'a, CatchClause<'a>>>,
        finally_block: Option<Block<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Try(TryStatement::boxed(
            span,
            resources,
            resources_trailing_semicolon,
            block,
            catches,
            finally_block,
            builder.builder(),
        ))
    }

    /// Build a [`Statement::Synchronized`].
    ///
    /// This node contains a [`SynchronizedStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `lock`
    /// * `body`
    #[inline]
    pub fn new_synchronized_statement(
        span: Span,
        lock: Expression<'a>,
        body: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Synchronized(SynchronizedStatement::boxed(span, lock, body, builder.builder()))
    }

    /// Build a [`Statement::Return`].
    ///
    /// This node contains a [`ReturnStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_return_statement(
        span: Span,
        value: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Return(ReturnStatement::boxed(span, value, builder.builder()))
    }

    /// Build a [`Statement::Yield`].
    ///
    /// This node contains a [`YieldStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_yield_statement(
        span: Span,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Yield(YieldStatement::boxed(span, value, builder.builder()))
    }

    /// Build a [`Statement::Labeled`].
    ///
    /// This node contains a [`LabeledStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    /// * `statement`
    #[inline]
    pub fn new_labeled_statement(
        span: Span,
        label: LabelIdentifier<'a>,
        statement: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Labeled(LabeledStatement::boxed(span, label, statement, builder.builder()))
    }

    /// Build a [`Statement::Class`].
    ///
    /// This node contains a [`ClassDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `superclass`
    /// * `interfaces`
    /// * `permits`: Direct subclasses in `permits`, empty if absent (JLS §8.1.6).
    /// * `body`
    #[inline]
    pub fn new_class_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        superclass: Option<Type<'a>>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Class(ClassDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            superclass,
            interfaces,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`Statement::Interface`].
    ///
    /// This node contains an [`InterfaceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `extends`: Superinterfaces listed after `extends`.
    /// * `permits`: Direct subtypes in `permits`, empty if absent (JLS §9.1.4).
    /// * `body`
    #[inline]
    pub fn new_interface_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        extends: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        permits: impl IntoIn<'a, Vec<'a, TypeName<'a>>>,
        body: InterfaceBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Interface(InterfaceDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            extends,
            permits,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`Statement::Enum`].
    ///
    /// This node contains an [`EnumDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_enum_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: EnumBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Enum(EnumDeclaration::boxed(
            span,
            modifiers,
            name,
            interfaces,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`Statement::Record`].
    ///
    /// This node contains a [`RecordDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `name`
    /// * `type_params`
    /// * `header`
    /// * `interfaces`
    /// * `body`
    #[inline]
    pub fn new_record_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        name: BindingIdentifier<'a>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        header: RecordHeader<'a>,
        interfaces: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        body: ClassBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Record(RecordDeclaration::boxed(
            span,
            modifiers,
            name,
            type_params,
            header,
            interfaces,
            body,
            builder.builder(),
        ))
    }
}

impl EmptyStatement {
    /// Build an [`EmptyStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`EmptyStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        EmptyStatement { node_id: Cell::new(builder.node_id()), span }
    }

    /// Build an [`EmptyStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`EmptyStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn boxed<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, builder), &builder.allocator())
    }
}

impl<'a> IfStatement<'a> {
    /// Build an [`IfStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`IfStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `then_branch`
    /// * `else_branch`
    #[inline]
    pub fn new(
        span: Span,
        condition: Expression<'a>,
        then_branch: Statement<'a>,
        else_branch: Option<Statement<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        IfStatement {
            node_id: Cell::new(builder.node_id()),
            span,
            condition,
            then_branch,
            else_branch,
        }
    }

    /// Build an [`IfStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`IfStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `then_branch`
    /// * `else_branch`
    #[inline]
    pub fn boxed(
        span: Span,
        condition: Expression<'a>,
        then_branch: Statement<'a>,
        else_branch: Option<Statement<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, condition, then_branch, else_branch, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> SwitchStatement<'a> {
    /// Build a [`SwitchStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`SwitchStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        SwitchStatement {
            node_id: Cell::new(builder.node_id()),
            span,
            selector,
            cases: cases.into_in(builder.allocator()),
            scope_id: Default::default(),
        }
    }

    /// Build a [`SwitchStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`SwitchStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn boxed(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, selector, cases, builder), &builder.allocator())
    }
}

impl<'a> SwitchCase<'a> {
    /// Build a [`SwitchCase::Rule`].
    ///
    /// This node contains a [`SwitchRule`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    /// * `body`
    #[inline]
    pub fn new_switch_rule(
        span: Span,
        label: SwitchLabel<'a>,
        body: SwitchRuleBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Rule(SwitchRule::boxed(span, label, body, builder.builder()))
    }

    /// Build a [`SwitchCase::Group`].
    ///
    /// This node contains a [`SwitchBlockStatementGroup`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `labels`
    /// * `statements`
    #[inline]
    pub fn new_switch_block_statement_group(
        span: Span,
        labels: impl IntoIn<'a, Vec<'a, SwitchLabel<'a>>>,
        statements: impl IntoIn<'a, Vec<'a, Statement<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Group(SwitchBlockStatementGroup::boxed(span, labels, statements, builder.builder()))
    }
}

impl<'a> SwitchRule<'a> {
    /// Build a [`SwitchRule`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`SwitchRule::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        label: SwitchLabel<'a>,
        body: SwitchRuleBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        SwitchRule { node_id: Cell::new(builder.node_id()), span, label, body }
    }

    /// Build a [`SwitchRule`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`SwitchRule::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        label: SwitchLabel<'a>,
        body: SwitchRuleBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, label, body, builder), &builder.allocator())
    }
}

impl<'a> SwitchRuleBody<'a> {
    /// Build a [`SwitchRuleBody::Block`].
    ///
    /// This node contains a [`Block`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `statements`
    #[inline]
    pub fn new_block(
        span: Span,
        statements: impl IntoIn<'a, Vec<'a, Statement<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Block(Block::boxed(span, statements, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Throw`].
    ///
    /// This node contains a [`ThrowStatement`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_throw_statement(
        span: Span,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Throw(ThrowStatement::boxed(span, value, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::IntLiteral`].
    ///
    /// This node contains an [`IntLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_int_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::IntLiteral(IntLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::FloatLiteral`].
    ///
    /// This node contains a [`FloatLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_float_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::FloatLiteral(FloatLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::CharLiteral`].
    ///
    /// This node contains a [`CharLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_char_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::CharLiteral(CharLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::StringLiteral`].
    ///
    /// This node contains a [`StringLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_string_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::StringLiteral(StringLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::BooleanLiteral`].
    ///
    /// This node contains a [`BooleanLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_boolean_literal(span: Span, value: bool, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::BooleanLiteral(BooleanLiteral::boxed(span, value, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::NullLiteral`].
    ///
    /// This node contains a [`NullLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_null_literal(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::NullLiteral(NullLiteral::boxed(span, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::This`].
    ///
    /// This node contains a [`ThisExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_this_expression(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::This(ThisExpression::boxed(span, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Super`].
    ///
    /// This node contains a [`Super`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_super(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Super(Super::boxed(span, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::QualifiedThis`].
    ///
    /// This node contains a [`QualifiedThis`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_this(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedThis(QualifiedThis::boxed(span, qualifier, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::QualifiedSuper`].
    ///
    /// This node contains a [`QualifiedSuper`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_super(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedSuper(QualifiedSuper::boxed(span, qualifier, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::FieldAccess`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::FieldAccess(FieldAccess::boxed(span, target, name, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::MethodCall`].
    ///
    /// This node contains a [`MethodCall`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `callee`
    /// * `args`
    #[inline]
    pub fn new_method_call(
        span: Span,
        callee: Callee<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodCall(MethodCall::boxed(span, callee, args, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::ClassLiteral`].
    ///
    /// This node contains a [`ClassLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    #[inline]
    pub fn new_class_literal(
        span: Span,
        ty: MethodResult<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassLiteral(ClassLiteral::boxed(span, ty, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::ClassInstanceCreation`].
    ///
    /// This node contains a [`ClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constructor_type_args`: Constructor type arguments in `new <T> Foo()`.
    /// * `ty`: The instantiated type, whose last name holds empty type arguments for
    /// * `args`
    /// * `body`: `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    #[inline]
    pub fn new_class_instance_creation(
        span: Span,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        ty: ClassType<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassInstanceCreation(ClassInstanceCreation::boxed(
            span,
            constructor_type_args,
            ty,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`SwitchRuleBody::QualifiedClassInstanceCreation`].
    ///
    /// This node contains a [`QualifiedClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `constructor_type_args`: See [`ClassInstanceCreation::constructor_type_args`].
    /// * `annotations`
    /// * `name`
    /// * `type_args`: Empty for an inferred `<>` (JLS §15.9.1).
    /// * `args`
    /// * `body`: See [`ClassInstanceCreation::body`].
    #[inline]
    pub fn new_qualified_class_instance_creation(
        span: Span,
        qualifier: Expression<'a>,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedClassInstanceCreation(QualifiedClassInstanceCreation::boxed(
            span,
            qualifier,
            constructor_type_args,
            annotations,
            name,
            type_args,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`SwitchRuleBody::ArrayCreation`].
    ///
    /// This node contains an [`ArrayCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `dims`: Dimensions in source order, sized before empty. All empty with an initializer.
    /// * `init`
    #[inline]
    pub fn new_array_creation(
        span: Span,
        element: Type<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayDimension<'a>>>,
        init: Option<ArrayInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayCreation(ArrayCreation::boxed(span, element, dims, init, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::ArrayAccess`].
    ///
    /// This node contains an [`ArrayAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn new_array_access(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayAccess(ArrayAccess::boxed(span, array, index, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Unary`].
    ///
    /// This node contains a [`UnaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `operand`
    #[inline]
    pub fn new_unary_expression(
        span: Span,
        op: UnaryOp,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Unary(UnaryExpression::boxed(span, op, operand, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Postfix`].
    ///
    /// This node contains a [`PostfixExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `op`
    #[inline]
    pub fn new_postfix_expression(
        span: Span,
        operand: Expression<'a>,
        op: PostfixOp,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Postfix(PostfixExpression::boxed(span, operand, op, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Cast`].
    ///
    /// This node contains a [`CastExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `additional_bounds`: Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    /// * `operand`
    #[inline]
    pub fn new_cast_expression(
        span: Span,
        ty: Type<'a>,
        additional_bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Cast(CastExpression::boxed(span, ty, additional_bounds, operand, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Instanceof`].
    ///
    /// This node contains an [`InstanceofExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `rhs`
    #[inline]
    pub fn new_instanceof_expression(
        span: Span,
        operand: Expression<'a>,
        rhs: InstanceofRhs<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Instanceof(InstanceofExpression::boxed(span, operand, rhs, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Binary`].
    ///
    /// This node contains a [`BinaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `lhs`
    /// * `rhs`
    #[inline]
    pub fn new_binary_expression(
        span: Span,
        op: BinaryOp,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Binary(BinaryExpression::boxed(span, op, lhs, rhs, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Assignment`].
    ///
    /// This node contains an [`AssignmentExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `op`
    /// * `value`
    #[inline]
    pub fn new_assignment_expression(
        span: Span,
        target: AssignmentTarget<'a>,
        op: AssignOp,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Assignment(AssignmentExpression::boxed(span, target, op, value, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Conditional`].
    ///
    /// This node contains a [`ConditionalExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `when_true`
    /// * `when_false`
    #[inline]
    pub fn new_conditional_expression(
        span: Span,
        condition: Expression<'a>,
        when_true: Expression<'a>,
        when_false: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Conditional(ConditionalExpression::boxed(
            span,
            condition,
            when_true,
            when_false,
            builder.builder(),
        ))
    }

    /// Build a [`SwitchRuleBody::Switch`].
    ///
    /// This node contains a [`SwitchExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new_switch_expression(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Switch(SwitchExpression::boxed(span, selector, cases, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Parenthesized`].
    ///
    /// This node contains a [`ParenthesizedExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new_parenthesized_expression(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Parenthesized(ParenthesizedExpression::boxed(span, expression, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::Lambda`].
    ///
    /// This node contains a [`LambdaExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `params`
    /// * `body`
    #[inline]
    pub fn new_lambda_expression(
        span: Span,
        params: FormalParameters<'a>,
        body: LambdaBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Lambda(LambdaExpression::boxed(span, params, body, builder.builder()))
    }

    /// Build a [`SwitchRuleBody::MethodReference`].
    ///
    /// This node contains a [`MethodReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`: Type arguments after `::`.
    /// * `method`: The referenced method, or `None` for `::new`.
    #[inline]
    pub fn new_method_reference(
        span: Span,
        target: MethodReferenceTarget<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        method: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodReference(MethodReference::boxed(
            span,
            target,
            type_args,
            method,
            builder.builder(),
        ))
    }
}

impl<'a> SwitchBlockStatementGroup<'a> {
    /// Build a [`SwitchBlockStatementGroup`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`SwitchBlockStatementGroup::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `labels`
    /// * `statements`
    #[inline]
    pub fn new(
        span: Span,
        labels: impl IntoIn<'a, Vec<'a, SwitchLabel<'a>>>,
        statements: impl IntoIn<'a, Vec<'a, Statement<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        SwitchBlockStatementGroup {
            node_id: Cell::new(builder.node_id()),
            span,
            labels: labels.into_in(builder.allocator()),
            statements: statements.into_in(builder.allocator()),
        }
    }

    /// Build a [`SwitchBlockStatementGroup`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`SwitchBlockStatementGroup::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `labels`
    /// * `statements`
    #[inline]
    pub fn boxed(
        span: Span,
        labels: impl IntoIn<'a, Vec<'a, SwitchLabel<'a>>>,
        statements: impl IntoIn<'a, Vec<'a, Statement<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, labels, statements, builder), &builder.allocator())
    }
}

impl<'a> SwitchLabel<'a> {
    /// Build a [`SwitchLabel::Case`].
    ///
    /// This node contains a [`CaseLabel`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constants`
    /// * `has_default`
    #[inline]
    pub fn new_case_label(
        span: Span,
        constants: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        has_default: bool,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Case(CaseLabel::boxed(span, constants, has_default, builder.builder()))
    }

    /// Build a [`SwitchLabel::Pattern`].
    ///
    /// This node contains a [`CasePatternLabel`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `patterns`
    /// * `guard`
    #[inline]
    pub fn new_case_pattern_label(
        span: Span,
        patterns: impl IntoIn<'a, Vec<'a, Pattern<'a>>>,
        guard: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Pattern(CasePatternLabel::boxed(span, patterns, guard, builder.builder()))
    }

    /// Build a [`SwitchLabel::Default`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_default_label(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Default(DefaultLabel::new(span, builder.builder()))
    }
}

impl<'a> CaseLabel<'a> {
    /// Build a [`CaseLabel`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`CaseLabel::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constants`
    /// * `has_default`
    #[inline]
    pub fn new(
        span: Span,
        constants: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        has_default: bool,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        CaseLabel {
            node_id: Cell::new(builder.node_id()),
            span,
            constants: constants.into_in(builder.allocator()),
            has_default,
        }
    }

    /// Build a [`CaseLabel`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`CaseLabel::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constants`
    /// * `has_default`
    #[inline]
    pub fn boxed(
        span: Span,
        constants: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        has_default: bool,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, constants, has_default, builder), &builder.allocator())
    }
}

impl<'a> CasePatternLabel<'a> {
    /// Build a [`CasePatternLabel`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`CasePatternLabel::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `patterns`
    /// * `guard`
    #[inline]
    pub fn new(
        span: Span,
        patterns: impl IntoIn<'a, Vec<'a, Pattern<'a>>>,
        guard: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        CasePatternLabel {
            node_id: Cell::new(builder.node_id()),
            span,
            patterns: patterns.into_in(builder.allocator()),
            guard,
        }
    }

    /// Build a [`CasePatternLabel`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`CasePatternLabel::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `patterns`
    /// * `guard`
    #[inline]
    pub fn boxed(
        span: Span,
        patterns: impl IntoIn<'a, Vec<'a, Pattern<'a>>>,
        guard: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, patterns, guard, builder), &builder.allocator())
    }
}

impl DefaultLabel {
    /// Build a [`DefaultLabel`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        DefaultLabel { node_id: Cell::new(builder.node_id()), span }
    }
}

impl<'a> WhileStatement<'a> {
    /// Build a [`WhileStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`WhileStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        condition: Expression<'a>,
        body: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        WhileStatement { node_id: Cell::new(builder.node_id()), span, condition, body }
    }

    /// Build a [`WhileStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`WhileStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        condition: Expression<'a>,
        body: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, condition, body, builder), &builder.allocator())
    }
}

impl<'a> DoStatement<'a> {
    /// Build a [`DoStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`DoStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `body`
    /// * `condition`
    #[inline]
    pub fn new(
        span: Span,
        body: Statement<'a>,
        condition: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        DoStatement { node_id: Cell::new(builder.node_id()), span, body, condition }
    }

    /// Build a [`DoStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`DoStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `body`
    /// * `condition`
    #[inline]
    pub fn boxed(
        span: Span,
        body: Statement<'a>,
        condition: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, body, condition, builder), &builder.allocator())
    }
}

impl<'a> ForStatement<'a> {
    /// Build a [`ForStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ForStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `init`
    /// * `condition`
    /// * `update`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        init: Option<ForInit<'a>>,
        condition: Option<Expression<'a>>,
        update: Option<StatementExpressionList<'a>>,
        body: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ForStatement {
            node_id: Cell::new(builder.node_id()),
            span,
            init,
            condition,
            update,
            body,
            scope_id: Default::default(),
        }
    }

    /// Build a [`ForStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ForStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `init`
    /// * `condition`
    /// * `update`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        init: Option<ForInit<'a>>,
        condition: Option<Expression<'a>>,
        update: Option<StatementExpressionList<'a>>,
        body: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, init, condition, update, body, builder), &builder.allocator())
    }
}

impl<'a> ForInit<'a> {
    /// Build a [`ForInit::Declaration`].
    ///
    /// This node contains a [`LocalVariableDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `declarators`
    #[inline]
    pub fn new_local_variable_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        declarators: impl IntoIn<'a, Vec<'a, VariableDeclarator<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Declaration(LocalVariableDeclaration::boxed(
            span,
            modifiers,
            ty,
            declarators,
            builder.builder(),
        ))
    }

    /// Build a [`ForInit::Expressions`].
    ///
    /// This node contains a [`StatementExpressionList`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expressions`
    #[inline]
    pub fn new_statement_expression_list(
        span: Span,
        expressions: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Expressions(StatementExpressionList::boxed(span, expressions, builder.builder()))
    }
}

impl<'a> StatementExpressionList<'a> {
    /// Build a [`StatementExpressionList`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`StatementExpressionList::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expressions`
    #[inline]
    pub fn new(
        span: Span,
        expressions: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        StatementExpressionList {
            node_id: Cell::new(builder.node_id()),
            span,
            expressions: expressions.into_in(builder.allocator()),
        }
    }

    /// Build a [`StatementExpressionList`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`StatementExpressionList::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expressions`
    #[inline]
    pub fn boxed(
        span: Span,
        expressions: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, expressions, builder), &builder.allocator())
    }
}

impl<'a> EnhancedForStatement<'a> {
    /// Build an [`EnhancedForStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`EnhancedForStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    /// * `dims`: See [`VariableDeclarator::dims`].
    /// * `iterable`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        iterable: Expression<'a>,
        body: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        EnhancedForStatement {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            ty,
            name,
            dims: dims.into_in(builder.allocator()),
            iterable,
            body,
            scope_id: Default::default(),
        }
    }

    /// Build an [`EnhancedForStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`EnhancedForStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    /// * `dims`: See [`VariableDeclarator::dims`].
    /// * `iterable`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        iterable: Expression<'a>,
        body: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, modifiers, ty, name, dims, iterable, body, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> BreakStatement<'a> {
    /// Build a [`BreakStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`BreakStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    #[inline]
    pub fn new(
        span: Span,
        label: Option<LabelIdentifier<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        BreakStatement { node_id: Cell::new(builder.node_id()), span, label }
    }

    /// Build a [`BreakStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`BreakStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    #[inline]
    pub fn boxed(
        span: Span,
        label: Option<LabelIdentifier<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, label, builder), &builder.allocator())
    }
}

impl<'a> ContinueStatement<'a> {
    /// Build a [`ContinueStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ContinueStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    #[inline]
    pub fn new(
        span: Span,
        label: Option<LabelIdentifier<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ContinueStatement { node_id: Cell::new(builder.node_id()), span, label }
    }

    /// Build a [`ContinueStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ContinueStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    #[inline]
    pub fn boxed(
        span: Span,
        label: Option<LabelIdentifier<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, label, builder), &builder.allocator())
    }
}

impl<'a> ThrowStatement<'a> {
    /// Build a [`ThrowStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ThrowStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new(span: Span, value: Expression<'a>, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        ThrowStatement { node_id: Cell::new(builder.node_id()), span, value }
    }

    /// Build a [`ThrowStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ThrowStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn boxed(
        span: Span,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, value, builder), &builder.allocator())
    }
}

impl<'a> AssertStatement<'a> {
    /// Build an [`AssertStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`AssertStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `message`
    #[inline]
    pub fn new(
        span: Span,
        condition: Expression<'a>,
        message: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        AssertStatement { node_id: Cell::new(builder.node_id()), span, condition, message }
    }

    /// Build an [`AssertStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`AssertStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `message`
    #[inline]
    pub fn boxed(
        span: Span,
        condition: Expression<'a>,
        message: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, condition, message, builder), &builder.allocator())
    }
}

impl<'a> TryStatement<'a> {
    /// Build a [`TryStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`TryStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `resources`: Resource specifications, empty for a classic `try` (JLS §14.20.3).
    /// * `resources_trailing_semicolon`: Preserved trailing `;` in `try (r;)`, meaningful only with resources.
    /// * `block`
    /// * `catches`
    /// * `finally_block`
    #[inline]
    pub fn new(
        span: Span,
        resources: impl IntoIn<'a, Vec<'a, Resource<'a>>>,
        resources_trailing_semicolon: bool,
        block: Block<'a>,
        catches: impl IntoIn<'a, Vec<'a, CatchClause<'a>>>,
        finally_block: Option<Block<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        TryStatement {
            node_id: Cell::new(builder.node_id()),
            span,
            resources: resources.into_in(builder.allocator()),
            resources_trailing_semicolon,
            block,
            catches: catches.into_in(builder.allocator()),
            finally_block,
            scope_id: Default::default(),
        }
    }

    /// Build a [`TryStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`TryStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `resources`: Resource specifications, empty for a classic `try` (JLS §14.20.3).
    /// * `resources_trailing_semicolon`: Preserved trailing `;` in `try (r;)`, meaningful only with resources.
    /// * `block`
    /// * `catches`
    /// * `finally_block`
    #[inline]
    pub fn boxed(
        span: Span,
        resources: impl IntoIn<'a, Vec<'a, Resource<'a>>>,
        resources_trailing_semicolon: bool,
        block: Block<'a>,
        catches: impl IntoIn<'a, Vec<'a, CatchClause<'a>>>,
        finally_block: Option<Block<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(
                span,
                resources,
                resources_trailing_semicolon,
                block,
                catches,
                finally_block,
                builder,
            ),
            &builder.allocator(),
        )
    }
}

impl<'a> Resource<'a> {
    /// Build a [`Resource::Declaration`].
    ///
    /// This node contains a [`ResourceDeclaration`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    /// * `init`
    #[inline]
    pub fn new_resource_declaration(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        init: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Declaration(ResourceDeclaration::boxed(
            span,
            modifiers,
            ty,
            name,
            init,
            builder.builder(),
        ))
    }

    /// Build a [`Resource::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build a [`Resource::Field`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Field(FieldAccess::boxed(span, target, name, builder.builder()))
    }
}

impl<'a> ResourceAccess<'a> {
    /// Build a [`ResourceAccess::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build a [`ResourceAccess::Field`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Field(FieldAccess::boxed(span, target, name, builder.builder()))
    }
}

impl<'a> ResourceDeclaration<'a> {
    /// Build a [`ResourceDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ResourceDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    /// * `init`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        init: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ResourceDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            ty,
            name,
            init,
        }
    }

    /// Build a [`ResourceDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ResourceDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    /// * `init`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        init: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, modifiers, ty, name, init, builder), &builder.allocator())
    }
}

impl<'a> CatchClause<'a> {
    /// Build a [`CatchClause`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `types`
    /// * `name`
    /// * `block`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        types: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        name: BindingIdentifier<'a>,
        block: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        CatchClause {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            types: types.into_in(builder.allocator()),
            name,
            block,
            scope_id: Default::default(),
        }
    }
}

impl<'a> SynchronizedStatement<'a> {
    /// Build a [`SynchronizedStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`SynchronizedStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `lock`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        lock: Expression<'a>,
        body: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        SynchronizedStatement { node_id: Cell::new(builder.node_id()), span, lock, body }
    }

    /// Build a [`SynchronizedStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`SynchronizedStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `lock`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        lock: Expression<'a>,
        body: Block<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, lock, body, builder), &builder.allocator())
    }
}

impl<'a> LabeledStatement<'a> {
    /// Build a [`LabeledStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`LabeledStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    /// * `statement`
    #[inline]
    pub fn new(
        span: Span,
        label: LabelIdentifier<'a>,
        statement: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        LabeledStatement { node_id: Cell::new(builder.node_id()), span, label, statement }
    }

    /// Build a [`LabeledStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`LabeledStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `label`
    /// * `statement`
    #[inline]
    pub fn boxed(
        span: Span,
        label: LabelIdentifier<'a>,
        statement: Statement<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, label, statement, builder), &builder.allocator())
    }
}

impl<'a> ConstructorInvocation<'a> {
    /// Build a [`ConstructorInvocation`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ConstructorInvocation::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`: The qualifier in `outer.super(…)`, absent for direct invocations (JLS §8.8.7.1).
    /// * `type_args`: Type arguments in `<T>this(…)` or `[outer.]<T>super(…)`.
    /// * `kind`
    /// * `args`
    #[inline]
    pub fn new(
        span: Span,
        qualifier: Option<Expression<'a>>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        kind: ConstructorInvocationKind,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ConstructorInvocation {
            node_id: Cell::new(builder.node_id()),
            span,
            qualifier,
            type_args,
            kind,
            args: args.into_in(builder.allocator()),
        }
    }

    /// Build a [`ConstructorInvocation`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ConstructorInvocation::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`: The qualifier in `outer.super(…)`, absent for direct invocations (JLS §8.8.7.1).
    /// * `type_args`: Type arguments in `<T>this(…)` or `[outer.]<T>super(…)`.
    /// * `kind`
    /// * `args`
    #[inline]
    pub fn boxed(
        span: Span,
        qualifier: Option<Expression<'a>>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        kind: ConstructorInvocationKind,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, qualifier, type_args, kind, args, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> ExpressionStatement<'a> {
    /// Build an [`ExpressionStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ExpressionStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new(span: Span, expression: Expression<'a>, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        ExpressionStatement { node_id: Cell::new(builder.node_id()), span, expression }
    }

    /// Build an [`ExpressionStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ExpressionStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn boxed(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, expression, builder), &builder.allocator())
    }
}

impl<'a> LocalVariableDeclaration<'a> {
    /// Build a [`LocalVariableDeclaration`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`LocalVariableDeclaration::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `declarators`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        declarators: impl IntoIn<'a, Vec<'a, VariableDeclarator<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        LocalVariableDeclaration {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            ty,
            declarators: declarators.into_in(builder.allocator()),
        }
    }

    /// Build a [`LocalVariableDeclaration`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`LocalVariableDeclaration::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `declarators`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        declarators: impl IntoIn<'a, Vec<'a, VariableDeclarator<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, modifiers, ty, declarators, builder), &builder.allocator())
    }
}

impl<'a> ReturnStatement<'a> {
    /// Build a [`ReturnStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ReturnStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new(
        span: Span,
        value: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ReturnStatement { node_id: Cell::new(builder.node_id()), span, value }
    }

    /// Build a [`ReturnStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ReturnStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn boxed(
        span: Span,
        value: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, value, builder), &builder.allocator())
    }
}

impl<'a> YieldStatement<'a> {
    /// Build a [`YieldStatement`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`YieldStatement::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new(span: Span, value: Expression<'a>, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        YieldStatement { node_id: Cell::new(builder.node_id()), span, value }
    }

    /// Build a [`YieldStatement`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`YieldStatement::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn boxed(
        span: Span,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, value, builder), &builder.allocator())
    }
}

impl<'a> VariableDeclarator<'a> {
    /// Build a [`VariableDeclarator`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `dims`: Post-name dimensions, e.g. `b[]` in `int a, b[]` (JLS §10.2).
    /// * `init`
    #[inline]
    pub fn new(
        span: Span,
        name: BindingIdentifier<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayTypeDimension<'a>>>,
        init: Option<VariableInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        VariableDeclarator {
            node_id: Cell::new(builder.node_id()),
            span,
            name,
            dims: dims.into_in(builder.allocator()),
            init,
        }
    }
}

impl<'a> VariableInitializer<'a> {
    /// Build a [`VariableInitializer::Array`].
    ///
    /// This node contains an [`ArrayInitializer`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `elements`
    #[inline]
    pub fn new_array_initializer(
        span: Span,
        elements: impl IntoIn<'a, Vec<'a, VariableInitializer<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Array(ArrayInitializer::boxed(span, elements, builder.builder()))
    }

    /// Build a [`VariableInitializer::IntLiteral`].
    ///
    /// This node contains an [`IntLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_int_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::IntLiteral(IntLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`VariableInitializer::FloatLiteral`].
    ///
    /// This node contains a [`FloatLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_float_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::FloatLiteral(FloatLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`VariableInitializer::CharLiteral`].
    ///
    /// This node contains a [`CharLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_char_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::CharLiteral(CharLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`VariableInitializer::StringLiteral`].
    ///
    /// This node contains a [`StringLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_string_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::StringLiteral(StringLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`VariableInitializer::BooleanLiteral`].
    ///
    /// This node contains a [`BooleanLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_boolean_literal(span: Span, value: bool, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::BooleanLiteral(BooleanLiteral::boxed(span, value, builder.builder()))
    }

    /// Build a [`VariableInitializer::NullLiteral`].
    ///
    /// This node contains a [`NullLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_null_literal(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::NullLiteral(NullLiteral::boxed(span, builder.builder()))
    }

    /// Build a [`VariableInitializer::This`].
    ///
    /// This node contains a [`ThisExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_this_expression(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::This(ThisExpression::boxed(span, builder.builder()))
    }

    /// Build a [`VariableInitializer::Super`].
    ///
    /// This node contains a [`Super`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_super(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Super(Super::boxed(span, builder.builder()))
    }

    /// Build a [`VariableInitializer::QualifiedThis`].
    ///
    /// This node contains a [`QualifiedThis`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_this(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedThis(QualifiedThis::boxed(span, qualifier, builder.builder()))
    }

    /// Build a [`VariableInitializer::QualifiedSuper`].
    ///
    /// This node contains a [`QualifiedSuper`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_super(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedSuper(QualifiedSuper::boxed(span, qualifier, builder.builder()))
    }

    /// Build a [`VariableInitializer::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build a [`VariableInitializer::FieldAccess`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::FieldAccess(FieldAccess::boxed(span, target, name, builder.builder()))
    }

    /// Build a [`VariableInitializer::MethodCall`].
    ///
    /// This node contains a [`MethodCall`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `callee`
    /// * `args`
    #[inline]
    pub fn new_method_call(
        span: Span,
        callee: Callee<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodCall(MethodCall::boxed(span, callee, args, builder.builder()))
    }

    /// Build a [`VariableInitializer::ClassLiteral`].
    ///
    /// This node contains a [`ClassLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    #[inline]
    pub fn new_class_literal(
        span: Span,
        ty: MethodResult<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassLiteral(ClassLiteral::boxed(span, ty, builder.builder()))
    }

    /// Build a [`VariableInitializer::ClassInstanceCreation`].
    ///
    /// This node contains a [`ClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constructor_type_args`: Constructor type arguments in `new <T> Foo()`.
    /// * `ty`: The instantiated type, whose last name holds empty type arguments for
    /// * `args`
    /// * `body`: `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    #[inline]
    pub fn new_class_instance_creation(
        span: Span,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        ty: ClassType<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassInstanceCreation(ClassInstanceCreation::boxed(
            span,
            constructor_type_args,
            ty,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`VariableInitializer::QualifiedClassInstanceCreation`].
    ///
    /// This node contains a [`QualifiedClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `constructor_type_args`: See [`ClassInstanceCreation::constructor_type_args`].
    /// * `annotations`
    /// * `name`
    /// * `type_args`: Empty for an inferred `<>` (JLS §15.9.1).
    /// * `args`
    /// * `body`: See [`ClassInstanceCreation::body`].
    #[inline]
    pub fn new_qualified_class_instance_creation(
        span: Span,
        qualifier: Expression<'a>,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedClassInstanceCreation(QualifiedClassInstanceCreation::boxed(
            span,
            qualifier,
            constructor_type_args,
            annotations,
            name,
            type_args,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`VariableInitializer::ArrayCreation`].
    ///
    /// This node contains an [`ArrayCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `dims`: Dimensions in source order, sized before empty. All empty with an initializer.
    /// * `init`
    #[inline]
    pub fn new_array_creation(
        span: Span,
        element: Type<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayDimension<'a>>>,
        init: Option<ArrayInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayCreation(ArrayCreation::boxed(span, element, dims, init, builder.builder()))
    }

    /// Build a [`VariableInitializer::ArrayAccess`].
    ///
    /// This node contains an [`ArrayAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn new_array_access(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayAccess(ArrayAccess::boxed(span, array, index, builder.builder()))
    }

    /// Build a [`VariableInitializer::Unary`].
    ///
    /// This node contains a [`UnaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `operand`
    #[inline]
    pub fn new_unary_expression(
        span: Span,
        op: UnaryOp,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Unary(UnaryExpression::boxed(span, op, operand, builder.builder()))
    }

    /// Build a [`VariableInitializer::Postfix`].
    ///
    /// This node contains a [`PostfixExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `op`
    #[inline]
    pub fn new_postfix_expression(
        span: Span,
        operand: Expression<'a>,
        op: PostfixOp,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Postfix(PostfixExpression::boxed(span, operand, op, builder.builder()))
    }

    /// Build a [`VariableInitializer::Cast`].
    ///
    /// This node contains a [`CastExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `additional_bounds`: Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    /// * `operand`
    #[inline]
    pub fn new_cast_expression(
        span: Span,
        ty: Type<'a>,
        additional_bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Cast(CastExpression::boxed(span, ty, additional_bounds, operand, builder.builder()))
    }

    /// Build a [`VariableInitializer::Instanceof`].
    ///
    /// This node contains an [`InstanceofExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `rhs`
    #[inline]
    pub fn new_instanceof_expression(
        span: Span,
        operand: Expression<'a>,
        rhs: InstanceofRhs<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Instanceof(InstanceofExpression::boxed(span, operand, rhs, builder.builder()))
    }

    /// Build a [`VariableInitializer::Binary`].
    ///
    /// This node contains a [`BinaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `lhs`
    /// * `rhs`
    #[inline]
    pub fn new_binary_expression(
        span: Span,
        op: BinaryOp,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Binary(BinaryExpression::boxed(span, op, lhs, rhs, builder.builder()))
    }

    /// Build a [`VariableInitializer::Assignment`].
    ///
    /// This node contains an [`AssignmentExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `op`
    /// * `value`
    #[inline]
    pub fn new_assignment_expression(
        span: Span,
        target: AssignmentTarget<'a>,
        op: AssignOp,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Assignment(AssignmentExpression::boxed(span, target, op, value, builder.builder()))
    }

    /// Build a [`VariableInitializer::Conditional`].
    ///
    /// This node contains a [`ConditionalExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `when_true`
    /// * `when_false`
    #[inline]
    pub fn new_conditional_expression(
        span: Span,
        condition: Expression<'a>,
        when_true: Expression<'a>,
        when_false: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Conditional(ConditionalExpression::boxed(
            span,
            condition,
            when_true,
            when_false,
            builder.builder(),
        ))
    }

    /// Build a [`VariableInitializer::Switch`].
    ///
    /// This node contains a [`SwitchExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new_switch_expression(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Switch(SwitchExpression::boxed(span, selector, cases, builder.builder()))
    }

    /// Build a [`VariableInitializer::Parenthesized`].
    ///
    /// This node contains a [`ParenthesizedExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new_parenthesized_expression(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Parenthesized(ParenthesizedExpression::boxed(span, expression, builder.builder()))
    }

    /// Build a [`VariableInitializer::Lambda`].
    ///
    /// This node contains a [`LambdaExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `params`
    /// * `body`
    #[inline]
    pub fn new_lambda_expression(
        span: Span,
        params: FormalParameters<'a>,
        body: LambdaBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Lambda(LambdaExpression::boxed(span, params, body, builder.builder()))
    }

    /// Build a [`VariableInitializer::MethodReference`].
    ///
    /// This node contains a [`MethodReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`: Type arguments after `::`.
    /// * `method`: The referenced method, or `None` for `::new`.
    #[inline]
    pub fn new_method_reference(
        span: Span,
        target: MethodReferenceTarget<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        method: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodReference(MethodReference::boxed(
            span,
            target,
            type_args,
            method,
            builder.builder(),
        ))
    }
}

impl<'a> ArrayInitializer<'a> {
    /// Build an [`ArrayInitializer`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ArrayInitializer::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `elements`
    #[inline]
    pub fn new(
        span: Span,
        elements: impl IntoIn<'a, Vec<'a, VariableInitializer<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ArrayInitializer {
            node_id: Cell::new(builder.node_id()),
            span,
            elements: elements.into_in(builder.allocator()),
        }
    }

    /// Build an [`ArrayInitializer`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ArrayInitializer::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `elements`
    #[inline]
    pub fn boxed(
        span: Span,
        elements: impl IntoIn<'a, Vec<'a, VariableInitializer<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, elements, builder), &builder.allocator())
    }
}

impl<'a> Expression<'a> {
    /// Build an [`Expression::IntLiteral`].
    ///
    /// This node contains an [`IntLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_int_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::IntLiteral(IntLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`Expression::FloatLiteral`].
    ///
    /// This node contains a [`FloatLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_float_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::FloatLiteral(FloatLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`Expression::CharLiteral`].
    ///
    /// This node contains a [`CharLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_char_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::CharLiteral(CharLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`Expression::StringLiteral`].
    ///
    /// This node contains a [`StringLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_string_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::StringLiteral(StringLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`Expression::BooleanLiteral`].
    ///
    /// This node contains a [`BooleanLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_boolean_literal(span: Span, value: bool, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::BooleanLiteral(BooleanLiteral::boxed(span, value, builder.builder()))
    }

    /// Build an [`Expression::NullLiteral`].
    ///
    /// This node contains a [`NullLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_null_literal(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::NullLiteral(NullLiteral::boxed(span, builder.builder()))
    }

    /// Build an [`Expression::This`].
    ///
    /// This node contains a [`ThisExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_this_expression(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::This(ThisExpression::boxed(span, builder.builder()))
    }

    /// Build an [`Expression::Super`].
    ///
    /// This node contains a [`Super`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_super(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Super(Super::boxed(span, builder.builder()))
    }

    /// Build an [`Expression::QualifiedThis`].
    ///
    /// This node contains a [`QualifiedThis`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_this(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedThis(QualifiedThis::boxed(span, qualifier, builder.builder()))
    }

    /// Build an [`Expression::QualifiedSuper`].
    ///
    /// This node contains a [`QualifiedSuper`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_super(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedSuper(QualifiedSuper::boxed(span, qualifier, builder.builder()))
    }

    /// Build an [`Expression::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build an [`Expression::FieldAccess`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::FieldAccess(FieldAccess::boxed(span, target, name, builder.builder()))
    }

    /// Build an [`Expression::MethodCall`].
    ///
    /// This node contains a [`MethodCall`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `callee`
    /// * `args`
    #[inline]
    pub fn new_method_call(
        span: Span,
        callee: Callee<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodCall(MethodCall::boxed(span, callee, args, builder.builder()))
    }

    /// Build an [`Expression::ClassLiteral`].
    ///
    /// This node contains a [`ClassLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    #[inline]
    pub fn new_class_literal(
        span: Span,
        ty: MethodResult<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassLiteral(ClassLiteral::boxed(span, ty, builder.builder()))
    }

    /// Build an [`Expression::ClassInstanceCreation`].
    ///
    /// This node contains a [`ClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constructor_type_args`: Constructor type arguments in `new <T> Foo()`.
    /// * `ty`: The instantiated type, whose last name holds empty type arguments for
    /// * `args`
    /// * `body`: `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    #[inline]
    pub fn new_class_instance_creation(
        span: Span,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        ty: ClassType<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassInstanceCreation(ClassInstanceCreation::boxed(
            span,
            constructor_type_args,
            ty,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`Expression::QualifiedClassInstanceCreation`].
    ///
    /// This node contains a [`QualifiedClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `constructor_type_args`: See [`ClassInstanceCreation::constructor_type_args`].
    /// * `annotations`
    /// * `name`
    /// * `type_args`: Empty for an inferred `<>` (JLS §15.9.1).
    /// * `args`
    /// * `body`: See [`ClassInstanceCreation::body`].
    #[inline]
    pub fn new_qualified_class_instance_creation(
        span: Span,
        qualifier: Expression<'a>,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedClassInstanceCreation(QualifiedClassInstanceCreation::boxed(
            span,
            qualifier,
            constructor_type_args,
            annotations,
            name,
            type_args,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`Expression::ArrayCreation`].
    ///
    /// This node contains an [`ArrayCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `dims`: Dimensions in source order, sized before empty. All empty with an initializer.
    /// * `init`
    #[inline]
    pub fn new_array_creation(
        span: Span,
        element: Type<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayDimension<'a>>>,
        init: Option<ArrayInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayCreation(ArrayCreation::boxed(span, element, dims, init, builder.builder()))
    }

    /// Build an [`Expression::ArrayAccess`].
    ///
    /// This node contains an [`ArrayAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn new_array_access(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayAccess(ArrayAccess::boxed(span, array, index, builder.builder()))
    }

    /// Build an [`Expression::Unary`].
    ///
    /// This node contains a [`UnaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `operand`
    #[inline]
    pub fn new_unary_expression(
        span: Span,
        op: UnaryOp,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Unary(UnaryExpression::boxed(span, op, operand, builder.builder()))
    }

    /// Build an [`Expression::Postfix`].
    ///
    /// This node contains a [`PostfixExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `op`
    #[inline]
    pub fn new_postfix_expression(
        span: Span,
        operand: Expression<'a>,
        op: PostfixOp,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Postfix(PostfixExpression::boxed(span, operand, op, builder.builder()))
    }

    /// Build an [`Expression::Cast`].
    ///
    /// This node contains a [`CastExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `additional_bounds`: Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    /// * `operand`
    #[inline]
    pub fn new_cast_expression(
        span: Span,
        ty: Type<'a>,
        additional_bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Cast(CastExpression::boxed(span, ty, additional_bounds, operand, builder.builder()))
    }

    /// Build an [`Expression::Instanceof`].
    ///
    /// This node contains an [`InstanceofExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `rhs`
    #[inline]
    pub fn new_instanceof_expression(
        span: Span,
        operand: Expression<'a>,
        rhs: InstanceofRhs<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Instanceof(InstanceofExpression::boxed(span, operand, rhs, builder.builder()))
    }

    /// Build an [`Expression::Binary`].
    ///
    /// This node contains a [`BinaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `lhs`
    /// * `rhs`
    #[inline]
    pub fn new_binary_expression(
        span: Span,
        op: BinaryOp,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Binary(BinaryExpression::boxed(span, op, lhs, rhs, builder.builder()))
    }

    /// Build an [`Expression::Assignment`].
    ///
    /// This node contains an [`AssignmentExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `op`
    /// * `value`
    #[inline]
    pub fn new_assignment_expression(
        span: Span,
        target: AssignmentTarget<'a>,
        op: AssignOp,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Assignment(AssignmentExpression::boxed(span, target, op, value, builder.builder()))
    }

    /// Build an [`Expression::Conditional`].
    ///
    /// This node contains a [`ConditionalExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `when_true`
    /// * `when_false`
    #[inline]
    pub fn new_conditional_expression(
        span: Span,
        condition: Expression<'a>,
        when_true: Expression<'a>,
        when_false: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Conditional(ConditionalExpression::boxed(
            span,
            condition,
            when_true,
            when_false,
            builder.builder(),
        ))
    }

    /// Build an [`Expression::Switch`].
    ///
    /// This node contains a [`SwitchExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new_switch_expression(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Switch(SwitchExpression::boxed(span, selector, cases, builder.builder()))
    }

    /// Build an [`Expression::Parenthesized`].
    ///
    /// This node contains a [`ParenthesizedExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new_parenthesized_expression(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Parenthesized(ParenthesizedExpression::boxed(span, expression, builder.builder()))
    }

    /// Build an [`Expression::Lambda`].
    ///
    /// This node contains a [`LambdaExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `params`
    /// * `body`
    #[inline]
    pub fn new_lambda_expression(
        span: Span,
        params: FormalParameters<'a>,
        body: LambdaBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Lambda(LambdaExpression::boxed(span, params, body, builder.builder()))
    }

    /// Build an [`Expression::MethodReference`].
    ///
    /// This node contains a [`MethodReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`: Type arguments after `::`.
    /// * `method`: The referenced method, or `None` for `::new`.
    #[inline]
    pub fn new_method_reference(
        span: Span,
        target: MethodReferenceTarget<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        method: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodReference(MethodReference::boxed(
            span,
            target,
            type_args,
            method,
            builder.builder(),
        ))
    }
}

impl<'a> FieldAccess<'a> {
    /// Build a [`FieldAccess`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`FieldAccess::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        FieldAccess { node_id: Cell::new(builder.node_id()), span, target, name }
    }

    /// Build a [`FieldAccess`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`FieldAccess::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn boxed(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, target, name, builder), &builder.allocator())
    }
}

impl<'a> MethodCall<'a> {
    /// Build a [`MethodCall`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`MethodCall::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `callee`
    /// * `args`
    #[inline]
    pub fn new(
        span: Span,
        callee: Callee<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        MethodCall {
            node_id: Cell::new(builder.node_id()),
            span,
            callee,
            args: args.into_in(builder.allocator()),
        }
    }

    /// Build a [`MethodCall`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`MethodCall::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `callee`
    /// * `args`
    #[inline]
    pub fn boxed(
        span: Span,
        callee: Callee<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, callee, args, builder), &builder.allocator())
    }
}

impl<'a> Callee<'a> {
    /// Build a [`Callee::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build a [`Callee::Member`].
    ///
    /// This node contains a [`MethodMember`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`
    /// * `name`
    #[inline]
    pub fn new_method_member(
        span: Span,
        target: Expression<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Member(MethodMember::boxed(span, target, type_args, name, builder.builder()))
    }
}

impl<'a> MethodMember<'a> {
    /// Build a [`MethodMember`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`MethodMember::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`
    /// * `name`
    #[inline]
    pub fn new(
        span: Span,
        target: Expression<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        MethodMember { node_id: Cell::new(builder.node_id()), span, target, type_args, name }
    }

    /// Build a [`MethodMember`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`MethodMember::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`
    /// * `name`
    #[inline]
    pub fn boxed(
        span: Span,
        target: Expression<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, target, type_args, name, builder), &builder.allocator())
    }
}

impl<'a> ClassLiteral<'a> {
    /// Build a [`ClassLiteral`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ClassLiteral::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    #[inline]
    pub fn new(span: Span, ty: MethodResult<'a>, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        ClassLiteral { node_id: Cell::new(builder.node_id()), span, ty }
    }

    /// Build a [`ClassLiteral`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ClassLiteral::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    #[inline]
    pub fn boxed(
        span: Span,
        ty: MethodResult<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, ty, builder), &builder.allocator())
    }
}

impl ThisExpression {
    /// Build a [`ThisExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ThisExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        ThisExpression { node_id: Cell::new(builder.node_id()), span }
    }

    /// Build a [`ThisExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ThisExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn boxed<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, builder), &builder.allocator())
    }
}

impl Super {
    /// Build a [`Super`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`Super::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        Super { node_id: Cell::new(builder.node_id()), span }
    }

    /// Build a [`Super`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`Super::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn boxed<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, builder), &builder.allocator())
    }
}

impl<'a> QualifiedThis<'a> {
    /// Build a [`QualifiedThis`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`QualifiedThis::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new(span: Span, qualifier: TypeName<'a>, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        QualifiedThis { node_id: Cell::new(builder.node_id()), span, qualifier }
    }

    /// Build a [`QualifiedThis`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`QualifiedThis::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn boxed(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, qualifier, builder), &builder.allocator())
    }
}

impl<'a> QualifiedSuper<'a> {
    /// Build a [`QualifiedSuper`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`QualifiedSuper::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new(span: Span, qualifier: TypeName<'a>, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        QualifiedSuper { node_id: Cell::new(builder.node_id()), span, qualifier }
    }

    /// Build a [`QualifiedSuper`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`QualifiedSuper::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn boxed(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, qualifier, builder), &builder.allocator())
    }
}

impl<'a> ClassInstanceCreation<'a> {
    /// Build a [`ClassInstanceCreation`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ClassInstanceCreation::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constructor_type_args`: Constructor type arguments in `new <T> Foo()`.
    /// * `ty`: The instantiated type, whose last name holds empty type arguments for
    /// * `args`
    /// * `body`: `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    #[inline]
    pub fn new(
        span: Span,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        ty: ClassType<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ClassInstanceCreation {
            node_id: Cell::new(builder.node_id()),
            span,
            constructor_type_args,
            ty,
            args: args.into_in(builder.allocator()),
            body,
        }
    }

    /// Build a [`ClassInstanceCreation`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ClassInstanceCreation::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constructor_type_args`: Constructor type arguments in `new <T> Foo()`.
    /// * `ty`: The instantiated type, whose last name holds empty type arguments for
    /// * `args`
    /// * `body`: `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    #[inline]
    pub fn boxed(
        span: Span,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        ty: ClassType<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, constructor_type_args, ty, args, body, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> QualifiedClassInstanceCreation<'a> {
    /// Build a [`QualifiedClassInstanceCreation`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`QualifiedClassInstanceCreation::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `constructor_type_args`: See [`ClassInstanceCreation::constructor_type_args`].
    /// * `annotations`
    /// * `name`
    /// * `type_args`: Empty for an inferred `<>` (JLS §15.9.1).
    /// * `args`
    /// * `body`: See [`ClassInstanceCreation::body`].
    #[inline]
    pub fn new(
        span: Span,
        qualifier: Expression<'a>,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        QualifiedClassInstanceCreation {
            node_id: Cell::new(builder.node_id()),
            span,
            qualifier,
            constructor_type_args,
            annotations: annotations.into_in(builder.allocator()),
            name,
            type_args,
            args: args.into_in(builder.allocator()),
            body,
        }
    }

    /// Build a [`QualifiedClassInstanceCreation`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`QualifiedClassInstanceCreation::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `constructor_type_args`: See [`ClassInstanceCreation::constructor_type_args`].
    /// * `annotations`
    /// * `name`
    /// * `type_args`: Empty for an inferred `<>` (JLS §15.9.1).
    /// * `args`
    /// * `body`: See [`ClassInstanceCreation::body`].
    #[inline]
    pub fn boxed(
        span: Span,
        qualifier: Expression<'a>,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(
                span,
                qualifier,
                constructor_type_args,
                annotations,
                name,
                type_args,
                args,
                body,
                builder,
            ),
            &builder.allocator(),
        )
    }
}

impl<'a> ArrayCreation<'a> {
    /// Build an [`ArrayCreation`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ArrayCreation::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `dims`: Dimensions in source order, sized before empty. All empty with an initializer.
    /// * `init`
    #[inline]
    pub fn new(
        span: Span,
        element: Type<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayDimension<'a>>>,
        init: Option<ArrayInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ArrayCreation {
            node_id: Cell::new(builder.node_id()),
            span,
            element,
            dims: dims.into_in(builder.allocator()),
            init,
        }
    }

    /// Build an [`ArrayCreation`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ArrayCreation::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `dims`: Dimensions in source order, sized before empty. All empty with an initializer.
    /// * `init`
    #[inline]
    pub fn boxed(
        span: Span,
        element: Type<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayDimension<'a>>>,
        init: Option<ArrayInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, element, dims, init, builder), &builder.allocator())
    }
}

impl<'a> ArrayDimension<'a> {
    /// Build an [`ArrayDimension`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`: `@A` in `new int @A [4]`.
    /// * `length`: `None` for an empty `[]` dimension.
    #[inline]
    pub fn new(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        length: Option<Expression<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ArrayDimension {
            node_id: Cell::new(builder.node_id()),
            span,
            annotations: annotations.into_in(builder.allocator()),
            length,
        }
    }
}

impl<'a> ArrayAccess<'a> {
    /// Build an [`ArrayAccess`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ArrayAccess::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn new(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ArrayAccess { node_id: Cell::new(builder.node_id()), span, array, index }
    }

    /// Build an [`ArrayAccess`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ArrayAccess::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn boxed(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, array, index, builder), &builder.allocator())
    }
}

impl<'a> UnaryExpression<'a> {
    /// Build a [`UnaryExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`UnaryExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `operand`
    #[inline]
    pub fn new(
        span: Span,
        op: UnaryOp,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        UnaryExpression { node_id: Cell::new(builder.node_id()), span, op, operand }
    }

    /// Build a [`UnaryExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`UnaryExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `operand`
    #[inline]
    pub fn boxed(
        span: Span,
        op: UnaryOp,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, op, operand, builder), &builder.allocator())
    }
}

impl<'a> PostfixExpression<'a> {
    /// Build a [`PostfixExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`PostfixExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `op`
    #[inline]
    pub fn new(
        span: Span,
        operand: Expression<'a>,
        op: PostfixOp,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        PostfixExpression { node_id: Cell::new(builder.node_id()), span, operand, op }
    }

    /// Build a [`PostfixExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`PostfixExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `op`
    #[inline]
    pub fn boxed(
        span: Span,
        operand: Expression<'a>,
        op: PostfixOp,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, operand, op, builder), &builder.allocator())
    }
}

impl<'a> CastExpression<'a> {
    /// Build a [`CastExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`CastExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `additional_bounds`: Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    /// * `operand`
    #[inline]
    pub fn new(
        span: Span,
        ty: Type<'a>,
        additional_bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        CastExpression {
            node_id: Cell::new(builder.node_id()),
            span,
            ty,
            additional_bounds: additional_bounds.into_in(builder.allocator()),
            operand,
        }
    }

    /// Build a [`CastExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`CastExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `additional_bounds`: Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    /// * `operand`
    #[inline]
    pub fn boxed(
        span: Span,
        ty: Type<'a>,
        additional_bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, ty, additional_bounds, operand, builder), &builder.allocator())
    }
}

impl<'a> InstanceofExpression<'a> {
    /// Build an [`InstanceofExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`InstanceofExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `rhs`
    #[inline]
    pub fn new(
        span: Span,
        operand: Expression<'a>,
        rhs: InstanceofRhs<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        InstanceofExpression { node_id: Cell::new(builder.node_id()), span, operand, rhs }
    }

    /// Build an [`InstanceofExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`InstanceofExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `rhs`
    #[inline]
    pub fn boxed(
        span: Span,
        operand: Expression<'a>,
        rhs: InstanceofRhs<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, operand, rhs, builder), &builder.allocator())
    }
}

impl<'a> InstanceofRhs<'a> {
    /// Build an [`InstanceofRhs::Primitive`].
    ///
    /// This node contains a [`PrimitiveType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `kind`
    #[inline]
    pub fn new_primitive_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        kind: PrimitiveTypeKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Primitive(PrimitiveType::boxed(span, annotations, kind, builder.builder()))
    }

    /// Build an [`InstanceofRhs::Array`].
    ///
    /// This node contains an [`ArrayType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `annotations`: Annotations before this dimension's `[`, e.g. `char @Nullable []` (JLS §10.1).
    #[inline]
    pub fn new_array_type(
        span: Span,
        element: Type<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Array(ArrayType::boxed(span, element, annotations, builder.builder()))
    }

    /// Build an [`InstanceofRhs::Simple`].
    ///
    /// This node contains a [`SimpleClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_simple_class_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierReference<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Simple(SimpleClassType::boxed(span, annotations, name, args, builder.builder()))
    }

    /// Build an [`InstanceofRhs::Qualified`].
    ///
    /// This node contains a [`QualifiedClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_qualified_class_type(
        span: Span,
        qualifier: ClassType<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Qualified(QualifiedClassType::boxed(
            span,
            qualifier,
            annotations,
            name,
            args,
            builder.builder(),
        ))
    }

    /// Build an [`InstanceofRhs::TypePattern`].
    ///
    /// This node contains a [`TypePattern`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    #[inline]
    pub fn new_type_pattern(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::TypePattern(TypePattern::boxed(span, modifiers, ty, name, builder.builder()))
    }

    /// Build an [`InstanceofRhs::RecordPattern`].
    ///
    /// This node contains a [`RecordPattern`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `components`
    #[inline]
    pub fn new_record_pattern(
        span: Span,
        ty: Type<'a>,
        components: impl IntoIn<'a, Vec<'a, ComponentPattern<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::RecordPattern(RecordPattern::boxed(span, ty, components, builder.builder()))
    }
}

impl<'a> Pattern<'a> {
    /// Build a [`Pattern::TypePattern`].
    ///
    /// This node contains a [`TypePattern`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    #[inline]
    pub fn new_type_pattern(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::TypePattern(TypePattern::boxed(span, modifiers, ty, name, builder.builder()))
    }

    /// Build a [`Pattern::RecordPattern`].
    ///
    /// This node contains a [`RecordPattern`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `components`
    #[inline]
    pub fn new_record_pattern(
        span: Span,
        ty: Type<'a>,
        components: impl IntoIn<'a, Vec<'a, ComponentPattern<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::RecordPattern(RecordPattern::boxed(span, ty, components, builder.builder()))
    }
}

impl<'a> TypePattern<'a> {
    /// Build a [`TypePattern`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`TypePattern::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    #[inline]
    pub fn new(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        TypePattern {
            node_id: Cell::new(builder.node_id()),
            span,
            modifiers: modifiers.into_in(builder.allocator()),
            ty,
            name,
        }
    }

    /// Build a [`TypePattern`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`TypePattern::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    #[inline]
    pub fn boxed(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, modifiers, ty, name, builder), &builder.allocator())
    }
}

impl<'a> RecordPattern<'a> {
    /// Build a [`RecordPattern`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`RecordPattern::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `components`
    #[inline]
    pub fn new(
        span: Span,
        ty: Type<'a>,
        components: impl IntoIn<'a, Vec<'a, ComponentPattern<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        RecordPattern {
            node_id: Cell::new(builder.node_id()),
            span,
            ty,
            components: components.into_in(builder.allocator()),
        }
    }

    /// Build a [`RecordPattern`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`RecordPattern::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `components`
    #[inline]
    pub fn boxed(
        span: Span,
        ty: Type<'a>,
        components: impl IntoIn<'a, Vec<'a, ComponentPattern<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, ty, components, builder), &builder.allocator())
    }
}

impl<'a> ComponentPattern<'a> {
    /// Build a [`ComponentPattern::MatchAll`].
    ///
    /// This node contains a [`MatchAllPattern`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_match_all_pattern(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::MatchAll(MatchAllPattern::boxed(span, builder.builder()))
    }

    /// Build a [`ComponentPattern::TypePattern`].
    ///
    /// This node contains a [`TypePattern`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `modifiers`
    /// * `ty`
    /// * `name`
    #[inline]
    pub fn new_type_pattern(
        span: Span,
        modifiers: impl IntoIn<'a, Vec<'a, Modifier<'a>>>,
        ty: VariableType<'a>,
        name: BindingIdentifier<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::TypePattern(TypePattern::boxed(span, modifiers, ty, name, builder.builder()))
    }

    /// Build a [`ComponentPattern::RecordPattern`].
    ///
    /// This node contains a [`RecordPattern`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `components`
    #[inline]
    pub fn new_record_pattern(
        span: Span,
        ty: Type<'a>,
        components: impl IntoIn<'a, Vec<'a, ComponentPattern<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::RecordPattern(RecordPattern::boxed(span, ty, components, builder.builder()))
    }
}

impl MatchAllPattern {
    /// Build a [`MatchAllPattern`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`MatchAllPattern::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        MatchAllPattern { node_id: Cell::new(builder.node_id()), span }
    }

    /// Build a [`MatchAllPattern`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`MatchAllPattern::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn boxed<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, builder), &builder.allocator())
    }
}

impl<'a> BinaryExpression<'a> {
    /// Build a [`BinaryExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`BinaryExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `lhs`
    /// * `rhs`
    #[inline]
    pub fn new(
        span: Span,
        op: BinaryOp,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        BinaryExpression { node_id: Cell::new(builder.node_id()), span, op, lhs, rhs }
    }

    /// Build a [`BinaryExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`BinaryExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `lhs`
    /// * `rhs`
    #[inline]
    pub fn boxed(
        span: Span,
        op: BinaryOp,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, op, lhs, rhs, builder), &builder.allocator())
    }
}

impl<'a> AssignmentExpression<'a> {
    /// Build an [`AssignmentExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`AssignmentExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `op`
    /// * `value`
    #[inline]
    pub fn new(
        span: Span,
        target: AssignmentTarget<'a>,
        op: AssignOp,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        AssignmentExpression { node_id: Cell::new(builder.node_id()), span, target, op, value }
    }

    /// Build an [`AssignmentExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`AssignmentExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `op`
    /// * `value`
    #[inline]
    pub fn boxed(
        span: Span,
        target: AssignmentTarget<'a>,
        op: AssignOp,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, target, op, value, builder), &builder.allocator())
    }
}

impl<'a> AssignmentTarget<'a> {
    /// Build an [`AssignmentTarget::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build an [`AssignmentTarget::FieldAccess`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::FieldAccess(FieldAccess::boxed(span, target, name, builder.builder()))
    }

    /// Build an [`AssignmentTarget::ArrayAccess`].
    ///
    /// This node contains an [`ArrayAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn new_array_access(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayAccess(ArrayAccess::boxed(span, array, index, builder.builder()))
    }

    /// Build an [`AssignmentTarget::Parenthesized`].
    ///
    /// This node contains a [`ParenthesizedExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new_parenthesized_expression(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Parenthesized(ParenthesizedExpression::boxed(span, expression, builder.builder()))
    }
}

impl<'a> ConditionalExpression<'a> {
    /// Build a [`ConditionalExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ConditionalExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `when_true`
    /// * `when_false`
    #[inline]
    pub fn new(
        span: Span,
        condition: Expression<'a>,
        when_true: Expression<'a>,
        when_false: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ConditionalExpression {
            node_id: Cell::new(builder.node_id()),
            span,
            condition,
            when_true,
            when_false,
        }
    }

    /// Build a [`ConditionalExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ConditionalExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `when_true`
    /// * `when_false`
    #[inline]
    pub fn boxed(
        span: Span,
        condition: Expression<'a>,
        when_true: Expression<'a>,
        when_false: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, condition, when_true, when_false, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> SwitchExpression<'a> {
    /// Build a [`SwitchExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`SwitchExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        SwitchExpression {
            node_id: Cell::new(builder.node_id()),
            span,
            selector,
            cases: cases.into_in(builder.allocator()),
            scope_id: Default::default(),
        }
    }

    /// Build a [`SwitchExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`SwitchExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn boxed(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, selector, cases, builder), &builder.allocator())
    }
}

impl<'a> ParenthesizedExpression<'a> {
    /// Build a [`ParenthesizedExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ParenthesizedExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new(span: Span, expression: Expression<'a>, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        ParenthesizedExpression { node_id: Cell::new(builder.node_id()), span, expression }
    }

    /// Build a [`ParenthesizedExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ParenthesizedExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn boxed(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, expression, builder), &builder.allocator())
    }
}

impl<'a> LambdaExpression<'a> {
    /// Build a [`LambdaExpression`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`LambdaExpression::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `params`
    /// * `body`
    #[inline]
    pub fn new(
        span: Span,
        params: FormalParameters<'a>,
        body: LambdaBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        LambdaExpression {
            node_id: Cell::new(builder.node_id()),
            span,
            params,
            body,
            scope_id: Default::default(),
        }
    }

    /// Build a [`LambdaExpression`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`LambdaExpression::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `params`
    /// * `body`
    #[inline]
    pub fn boxed(
        span: Span,
        params: FormalParameters<'a>,
        body: LambdaBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, params, body, builder), &builder.allocator())
    }
}

impl<'a> LambdaBody<'a> {
    /// Build a [`LambdaBody::Block`].
    ///
    /// This node contains a [`Block`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `statements`
    #[inline]
    pub fn new_block(
        span: Span,
        statements: impl IntoIn<'a, Vec<'a, Statement<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Block(Block::boxed(span, statements, builder.builder()))
    }

    /// Build a [`LambdaBody::IntLiteral`].
    ///
    /// This node contains an [`IntLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_int_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::IntLiteral(IntLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`LambdaBody::FloatLiteral`].
    ///
    /// This node contains a [`FloatLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_float_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::FloatLiteral(FloatLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`LambdaBody::CharLiteral`].
    ///
    /// This node contains a [`CharLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_char_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::CharLiteral(CharLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`LambdaBody::StringLiteral`].
    ///
    /// This node contains a [`StringLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_string_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::StringLiteral(StringLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`LambdaBody::BooleanLiteral`].
    ///
    /// This node contains a [`BooleanLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_boolean_literal(span: Span, value: bool, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::BooleanLiteral(BooleanLiteral::boxed(span, value, builder.builder()))
    }

    /// Build a [`LambdaBody::NullLiteral`].
    ///
    /// This node contains a [`NullLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_null_literal(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::NullLiteral(NullLiteral::boxed(span, builder.builder()))
    }

    /// Build a [`LambdaBody::This`].
    ///
    /// This node contains a [`ThisExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_this_expression(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::This(ThisExpression::boxed(span, builder.builder()))
    }

    /// Build a [`LambdaBody::Super`].
    ///
    /// This node contains a [`Super`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_super(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Super(Super::boxed(span, builder.builder()))
    }

    /// Build a [`LambdaBody::QualifiedThis`].
    ///
    /// This node contains a [`QualifiedThis`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_this(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedThis(QualifiedThis::boxed(span, qualifier, builder.builder()))
    }

    /// Build a [`LambdaBody::QualifiedSuper`].
    ///
    /// This node contains a [`QualifiedSuper`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_super(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedSuper(QualifiedSuper::boxed(span, qualifier, builder.builder()))
    }

    /// Build a [`LambdaBody::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build a [`LambdaBody::FieldAccess`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::FieldAccess(FieldAccess::boxed(span, target, name, builder.builder()))
    }

    /// Build a [`LambdaBody::MethodCall`].
    ///
    /// This node contains a [`MethodCall`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `callee`
    /// * `args`
    #[inline]
    pub fn new_method_call(
        span: Span,
        callee: Callee<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodCall(MethodCall::boxed(span, callee, args, builder.builder()))
    }

    /// Build a [`LambdaBody::ClassLiteral`].
    ///
    /// This node contains a [`ClassLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    #[inline]
    pub fn new_class_literal(
        span: Span,
        ty: MethodResult<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassLiteral(ClassLiteral::boxed(span, ty, builder.builder()))
    }

    /// Build a [`LambdaBody::ClassInstanceCreation`].
    ///
    /// This node contains a [`ClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constructor_type_args`: Constructor type arguments in `new <T> Foo()`.
    /// * `ty`: The instantiated type, whose last name holds empty type arguments for
    /// * `args`
    /// * `body`: `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    #[inline]
    pub fn new_class_instance_creation(
        span: Span,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        ty: ClassType<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassInstanceCreation(ClassInstanceCreation::boxed(
            span,
            constructor_type_args,
            ty,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`LambdaBody::QualifiedClassInstanceCreation`].
    ///
    /// This node contains a [`QualifiedClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `constructor_type_args`: See [`ClassInstanceCreation::constructor_type_args`].
    /// * `annotations`
    /// * `name`
    /// * `type_args`: Empty for an inferred `<>` (JLS §15.9.1).
    /// * `args`
    /// * `body`: See [`ClassInstanceCreation::body`].
    #[inline]
    pub fn new_qualified_class_instance_creation(
        span: Span,
        qualifier: Expression<'a>,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedClassInstanceCreation(QualifiedClassInstanceCreation::boxed(
            span,
            qualifier,
            constructor_type_args,
            annotations,
            name,
            type_args,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`LambdaBody::ArrayCreation`].
    ///
    /// This node contains an [`ArrayCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `dims`: Dimensions in source order, sized before empty. All empty with an initializer.
    /// * `init`
    #[inline]
    pub fn new_array_creation(
        span: Span,
        element: Type<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayDimension<'a>>>,
        init: Option<ArrayInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayCreation(ArrayCreation::boxed(span, element, dims, init, builder.builder()))
    }

    /// Build a [`LambdaBody::ArrayAccess`].
    ///
    /// This node contains an [`ArrayAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn new_array_access(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayAccess(ArrayAccess::boxed(span, array, index, builder.builder()))
    }

    /// Build a [`LambdaBody::Unary`].
    ///
    /// This node contains a [`UnaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `operand`
    #[inline]
    pub fn new_unary_expression(
        span: Span,
        op: UnaryOp,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Unary(UnaryExpression::boxed(span, op, operand, builder.builder()))
    }

    /// Build a [`LambdaBody::Postfix`].
    ///
    /// This node contains a [`PostfixExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `op`
    #[inline]
    pub fn new_postfix_expression(
        span: Span,
        operand: Expression<'a>,
        op: PostfixOp,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Postfix(PostfixExpression::boxed(span, operand, op, builder.builder()))
    }

    /// Build a [`LambdaBody::Cast`].
    ///
    /// This node contains a [`CastExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `additional_bounds`: Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    /// * `operand`
    #[inline]
    pub fn new_cast_expression(
        span: Span,
        ty: Type<'a>,
        additional_bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Cast(CastExpression::boxed(span, ty, additional_bounds, operand, builder.builder()))
    }

    /// Build a [`LambdaBody::Instanceof`].
    ///
    /// This node contains an [`InstanceofExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `rhs`
    #[inline]
    pub fn new_instanceof_expression(
        span: Span,
        operand: Expression<'a>,
        rhs: InstanceofRhs<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Instanceof(InstanceofExpression::boxed(span, operand, rhs, builder.builder()))
    }

    /// Build a [`LambdaBody::Binary`].
    ///
    /// This node contains a [`BinaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `lhs`
    /// * `rhs`
    #[inline]
    pub fn new_binary_expression(
        span: Span,
        op: BinaryOp,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Binary(BinaryExpression::boxed(span, op, lhs, rhs, builder.builder()))
    }

    /// Build a [`LambdaBody::Assignment`].
    ///
    /// This node contains an [`AssignmentExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `op`
    /// * `value`
    #[inline]
    pub fn new_assignment_expression(
        span: Span,
        target: AssignmentTarget<'a>,
        op: AssignOp,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Assignment(AssignmentExpression::boxed(span, target, op, value, builder.builder()))
    }

    /// Build a [`LambdaBody::Conditional`].
    ///
    /// This node contains a [`ConditionalExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `when_true`
    /// * `when_false`
    #[inline]
    pub fn new_conditional_expression(
        span: Span,
        condition: Expression<'a>,
        when_true: Expression<'a>,
        when_false: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Conditional(ConditionalExpression::boxed(
            span,
            condition,
            when_true,
            when_false,
            builder.builder(),
        ))
    }

    /// Build a [`LambdaBody::Switch`].
    ///
    /// This node contains a [`SwitchExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new_switch_expression(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Switch(SwitchExpression::boxed(span, selector, cases, builder.builder()))
    }

    /// Build a [`LambdaBody::Parenthesized`].
    ///
    /// This node contains a [`ParenthesizedExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new_parenthesized_expression(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Parenthesized(ParenthesizedExpression::boxed(span, expression, builder.builder()))
    }

    /// Build a [`LambdaBody::Lambda`].
    ///
    /// This node contains a [`LambdaExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `params`
    /// * `body`
    #[inline]
    pub fn new_lambda_expression(
        span: Span,
        params: FormalParameters<'a>,
        body: LambdaBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Lambda(LambdaExpression::boxed(span, params, body, builder.builder()))
    }

    /// Build a [`LambdaBody::MethodReference`].
    ///
    /// This node contains a [`MethodReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`: Type arguments after `::`.
    /// * `method`: The referenced method, or `None` for `::new`.
    #[inline]
    pub fn new_method_reference(
        span: Span,
        target: MethodReferenceTarget<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        method: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodReference(MethodReference::boxed(
            span,
            target,
            type_args,
            method,
            builder.builder(),
        ))
    }
}

impl<'a> MethodReference<'a> {
    /// Build a [`MethodReference`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`MethodReference::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`: Type arguments after `::`.
    /// * `method`: The referenced method, or `None` for `::new`.
    #[inline]
    pub fn new(
        span: Span,
        target: MethodReferenceTarget<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        method: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        MethodReference { node_id: Cell::new(builder.node_id()), span, target, type_args, method }
    }

    /// Build a [`MethodReference`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`MethodReference::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`: Type arguments after `::`.
    /// * `method`: The referenced method, or `None` for `::new`.
    #[inline]
    pub fn boxed(
        span: Span,
        target: MethodReferenceTarget<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        method: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, target, type_args, method, builder), &builder.allocator())
    }
}

impl<'a> MethodReferenceTarget<'a> {
    /// Build a [`MethodReferenceTarget::IntLiteral`].
    ///
    /// This node contains an [`IntLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_int_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::IntLiteral(IntLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::FloatLiteral`].
    ///
    /// This node contains a [`FloatLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_float_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::FloatLiteral(FloatLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::CharLiteral`].
    ///
    /// This node contains a [`CharLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_char_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::CharLiteral(CharLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::StringLiteral`].
    ///
    /// This node contains a [`StringLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_string_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::StringLiteral(StringLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::BooleanLiteral`].
    ///
    /// This node contains a [`BooleanLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_boolean_literal(span: Span, value: bool, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::BooleanLiteral(BooleanLiteral::boxed(span, value, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::NullLiteral`].
    ///
    /// This node contains a [`NullLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_null_literal(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::NullLiteral(NullLiteral::boxed(span, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::This`].
    ///
    /// This node contains a [`ThisExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_this_expression(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::This(ThisExpression::boxed(span, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Super`].
    ///
    /// This node contains a [`Super`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_super(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Super(Super::boxed(span, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::QualifiedThis`].
    ///
    /// This node contains a [`QualifiedThis`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_this(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedThis(QualifiedThis::boxed(span, qualifier, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::QualifiedSuper`].
    ///
    /// This node contains a [`QualifiedSuper`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_super(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedSuper(QualifiedSuper::boxed(span, qualifier, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::FieldAccess`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::FieldAccess(FieldAccess::boxed(span, target, name, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::MethodCall`].
    ///
    /// This node contains a [`MethodCall`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `callee`
    /// * `args`
    #[inline]
    pub fn new_method_call(
        span: Span,
        callee: Callee<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodCall(MethodCall::boxed(span, callee, args, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::ClassLiteral`].
    ///
    /// This node contains a [`ClassLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    #[inline]
    pub fn new_class_literal(
        span: Span,
        ty: MethodResult<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassLiteral(ClassLiteral::boxed(span, ty, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::ClassInstanceCreation`].
    ///
    /// This node contains a [`ClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constructor_type_args`: Constructor type arguments in `new <T> Foo()`.
    /// * `ty`: The instantiated type, whose last name holds empty type arguments for
    /// * `args`
    /// * `body`: `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    #[inline]
    pub fn new_class_instance_creation(
        span: Span,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        ty: ClassType<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassInstanceCreation(ClassInstanceCreation::boxed(
            span,
            constructor_type_args,
            ty,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`MethodReferenceTarget::QualifiedClassInstanceCreation`].
    ///
    /// This node contains a [`QualifiedClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `constructor_type_args`: See [`ClassInstanceCreation::constructor_type_args`].
    /// * `annotations`
    /// * `name`
    /// * `type_args`: Empty for an inferred `<>` (JLS §15.9.1).
    /// * `args`
    /// * `body`: See [`ClassInstanceCreation::body`].
    #[inline]
    pub fn new_qualified_class_instance_creation(
        span: Span,
        qualifier: Expression<'a>,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedClassInstanceCreation(QualifiedClassInstanceCreation::boxed(
            span,
            qualifier,
            constructor_type_args,
            annotations,
            name,
            type_args,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build a [`MethodReferenceTarget::ArrayCreation`].
    ///
    /// This node contains an [`ArrayCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `dims`: Dimensions in source order, sized before empty. All empty with an initializer.
    /// * `init`
    #[inline]
    pub fn new_array_creation(
        span: Span,
        element: Type<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayDimension<'a>>>,
        init: Option<ArrayInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayCreation(ArrayCreation::boxed(span, element, dims, init, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::ArrayAccess`].
    ///
    /// This node contains an [`ArrayAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn new_array_access(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayAccess(ArrayAccess::boxed(span, array, index, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Unary`].
    ///
    /// This node contains a [`UnaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `operand`
    #[inline]
    pub fn new_unary_expression(
        span: Span,
        op: UnaryOp,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Unary(UnaryExpression::boxed(span, op, operand, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Postfix`].
    ///
    /// This node contains a [`PostfixExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `op`
    #[inline]
    pub fn new_postfix_expression(
        span: Span,
        operand: Expression<'a>,
        op: PostfixOp,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Postfix(PostfixExpression::boxed(span, operand, op, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Cast`].
    ///
    /// This node contains a [`CastExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `additional_bounds`: Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    /// * `operand`
    #[inline]
    pub fn new_cast_expression(
        span: Span,
        ty: Type<'a>,
        additional_bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Cast(CastExpression::boxed(span, ty, additional_bounds, operand, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Instanceof`].
    ///
    /// This node contains an [`InstanceofExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `rhs`
    #[inline]
    pub fn new_instanceof_expression(
        span: Span,
        operand: Expression<'a>,
        rhs: InstanceofRhs<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Instanceof(InstanceofExpression::boxed(span, operand, rhs, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Binary`].
    ///
    /// This node contains a [`BinaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `lhs`
    /// * `rhs`
    #[inline]
    pub fn new_binary_expression(
        span: Span,
        op: BinaryOp,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Binary(BinaryExpression::boxed(span, op, lhs, rhs, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Assignment`].
    ///
    /// This node contains an [`AssignmentExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `op`
    /// * `value`
    #[inline]
    pub fn new_assignment_expression(
        span: Span,
        target: AssignmentTarget<'a>,
        op: AssignOp,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Assignment(AssignmentExpression::boxed(span, target, op, value, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Conditional`].
    ///
    /// This node contains a [`ConditionalExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `when_true`
    /// * `when_false`
    #[inline]
    pub fn new_conditional_expression(
        span: Span,
        condition: Expression<'a>,
        when_true: Expression<'a>,
        when_false: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Conditional(ConditionalExpression::boxed(
            span,
            condition,
            when_true,
            when_false,
            builder.builder(),
        ))
    }

    /// Build a [`MethodReferenceTarget::Switch`].
    ///
    /// This node contains a [`SwitchExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new_switch_expression(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Switch(SwitchExpression::boxed(span, selector, cases, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Parenthesized`].
    ///
    /// This node contains a [`ParenthesizedExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new_parenthesized_expression(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Parenthesized(ParenthesizedExpression::boxed(span, expression, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Lambda`].
    ///
    /// This node contains a [`LambdaExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `params`
    /// * `body`
    #[inline]
    pub fn new_lambda_expression(
        span: Span,
        params: FormalParameters<'a>,
        body: LambdaBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Lambda(LambdaExpression::boxed(span, params, body, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::MethodReference`].
    ///
    /// This node contains a [`MethodReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`: Type arguments after `::`.
    /// * `method`: The referenced method, or `None` for `::new`.
    #[inline]
    pub fn new_method_reference(
        span: Span,
        target: MethodReferenceTarget<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        method: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodReference(MethodReference::boxed(
            span,
            target,
            type_args,
            method,
            builder.builder(),
        ))
    }

    /// Build a [`MethodReferenceTarget::Primitive`].
    ///
    /// This node contains a [`PrimitiveType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `kind`
    #[inline]
    pub fn new_primitive_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        kind: PrimitiveTypeKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Primitive(PrimitiveType::boxed(span, annotations, kind, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Array`].
    ///
    /// This node contains an [`ArrayType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `annotations`: Annotations before this dimension's `[`, e.g. `char @Nullable []` (JLS §10.1).
    #[inline]
    pub fn new_array_type(
        span: Span,
        element: Type<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Array(ArrayType::boxed(span, element, annotations, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Simple`].
    ///
    /// This node contains a [`SimpleClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_simple_class_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierReference<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Simple(SimpleClassType::boxed(span, annotations, name, args, builder.builder()))
    }

    /// Build a [`MethodReferenceTarget::Qualified`].
    ///
    /// This node contains a [`QualifiedClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_qualified_class_type(
        span: Span,
        qualifier: ClassType<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Qualified(QualifiedClassType::boxed(
            span,
            qualifier,
            annotations,
            name,
            args,
            builder.builder(),
        ))
    }
}

impl<'a> TypeParameters<'a> {
    /// Build a [`TypeParameters`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`TypeParameters::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: From `<` to `>`.
    /// * `params`
    #[inline]
    pub fn new(
        span: Span,
        params: impl IntoIn<'a, Vec<'a, TypeParameter<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        TypeParameters {
            node_id: Cell::new(builder.node_id()),
            span,
            params: params.into_in(builder.allocator()),
        }
    }

    /// Build a [`TypeParameters`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`TypeParameters::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: From `<` to `>`.
    /// * `params`
    #[inline]
    pub fn boxed(
        span: Span,
        params: impl IntoIn<'a, Vec<'a, TypeParameter<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, params, builder), &builder.allocator())
    }
}

impl<'a> TypeParameter<'a> {
    /// Build a [`TypeParameter`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`: `@A` in `<@A T>`.
    /// * `name`
    /// * `bounds`
    #[inline]
    pub fn new(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: BindingIdentifier<'a>,
        bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        TypeParameter {
            node_id: Cell::new(builder.node_id()),
            span,
            annotations: annotations.into_in(builder.allocator()),
            name,
            bounds: bounds.into_in(builder.allocator()),
        }
    }
}

impl<'a> VariableType<'a> {
    /// Build a [`VariableType::Var`].
    ///
    /// This node contains a [`VarType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_var_type(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Var(VarType::boxed(span, builder.builder()))
    }

    /// Build a [`VariableType::Primitive`].
    ///
    /// This node contains a [`PrimitiveType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `kind`
    #[inline]
    pub fn new_primitive_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        kind: PrimitiveTypeKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Primitive(PrimitiveType::boxed(span, annotations, kind, builder.builder()))
    }

    /// Build a [`VariableType::Array`].
    ///
    /// This node contains an [`ArrayType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `annotations`: Annotations before this dimension's `[`, e.g. `char @Nullable []` (JLS §10.1).
    #[inline]
    pub fn new_array_type(
        span: Span,
        element: Type<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Array(ArrayType::boxed(span, element, annotations, builder.builder()))
    }

    /// Build a [`VariableType::Simple`].
    ///
    /// This node contains a [`SimpleClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_simple_class_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierReference<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Simple(SimpleClassType::boxed(span, annotations, name, args, builder.builder()))
    }

    /// Build a [`VariableType::Qualified`].
    ///
    /// This node contains a [`QualifiedClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_qualified_class_type(
        span: Span,
        qualifier: ClassType<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Qualified(QualifiedClassType::boxed(
            span,
            qualifier,
            annotations,
            name,
            args,
            builder.builder(),
        ))
    }
}

impl VarType {
    /// Build a [`VarType`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`VarType::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        VarType { node_id: Cell::new(builder.node_id()), span }
    }

    /// Build a [`VarType`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`VarType::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn boxed<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, builder), &builder.allocator())
    }
}

impl<'a> Type<'a> {
    /// Build a [`Type::Primitive`].
    ///
    /// This node contains a [`PrimitiveType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `kind`
    #[inline]
    pub fn new_primitive_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        kind: PrimitiveTypeKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Primitive(PrimitiveType::boxed(span, annotations, kind, builder.builder()))
    }

    /// Build a [`Type::Array`].
    ///
    /// This node contains an [`ArrayType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `annotations`: Annotations before this dimension's `[`, e.g. `char @Nullable []` (JLS §10.1).
    #[inline]
    pub fn new_array_type(
        span: Span,
        element: Type<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Array(ArrayType::boxed(span, element, annotations, builder.builder()))
    }

    /// Build a [`Type::Simple`].
    ///
    /// This node contains a [`SimpleClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_simple_class_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierReference<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Simple(SimpleClassType::boxed(span, annotations, name, args, builder.builder()))
    }

    /// Build a [`Type::Qualified`].
    ///
    /// This node contains a [`QualifiedClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_qualified_class_type(
        span: Span,
        qualifier: ClassType<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Qualified(QualifiedClassType::boxed(
            span,
            qualifier,
            annotations,
            name,
            args,
            builder.builder(),
        ))
    }
}

impl<'a> MethodResult<'a> {
    /// Build a [`MethodResult::Void`].
    ///
    /// This node contains a [`VoidType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_void_type(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Void(VoidType::boxed(span, builder.builder()))
    }

    /// Build a [`MethodResult::Primitive`].
    ///
    /// This node contains a [`PrimitiveType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `kind`
    #[inline]
    pub fn new_primitive_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        kind: PrimitiveTypeKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Primitive(PrimitiveType::boxed(span, annotations, kind, builder.builder()))
    }

    /// Build a [`MethodResult::Array`].
    ///
    /// This node contains an [`ArrayType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `annotations`: Annotations before this dimension's `[`, e.g. `char @Nullable []` (JLS §10.1).
    #[inline]
    pub fn new_array_type(
        span: Span,
        element: Type<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Array(ArrayType::boxed(span, element, annotations, builder.builder()))
    }

    /// Build a [`MethodResult::Simple`].
    ///
    /// This node contains a [`SimpleClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_simple_class_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierReference<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Simple(SimpleClassType::boxed(span, annotations, name, args, builder.builder()))
    }

    /// Build a [`MethodResult::Qualified`].
    ///
    /// This node contains a [`QualifiedClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_qualified_class_type(
        span: Span,
        qualifier: ClassType<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Qualified(QualifiedClassType::boxed(
            span,
            qualifier,
            annotations,
            name,
            args,
            builder.builder(),
        ))
    }
}

impl VoidType {
    /// Build a [`VoidType`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`VoidType::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        VoidType { node_id: Cell::new(builder.node_id()), span }
    }

    /// Build a [`VoidType`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`VoidType::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn boxed<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, builder), &builder.allocator())
    }
}

impl<'a> PrimitiveType<'a> {
    /// Build a [`PrimitiveType`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`PrimitiveType::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `kind`
    #[inline]
    pub fn new(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        kind: PrimitiveTypeKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        PrimitiveType {
            node_id: Cell::new(builder.node_id()),
            span,
            annotations: annotations.into_in(builder.allocator()),
            kind,
        }
    }

    /// Build a [`PrimitiveType`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`PrimitiveType::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `kind`
    #[inline]
    pub fn boxed(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        kind: PrimitiveTypeKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, annotations, kind, builder), &builder.allocator())
    }
}

impl<'a> ClassType<'a> {
    /// Build a [`ClassType::Simple`].
    ///
    /// This node contains a [`SimpleClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_simple_class_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierReference<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Simple(SimpleClassType::boxed(span, annotations, name, args, builder.builder()))
    }

    /// Build a [`ClassType::Qualified`].
    ///
    /// This node contains a [`QualifiedClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_qualified_class_type(
        span: Span,
        qualifier: ClassType<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Qualified(QualifiedClassType::boxed(
            span,
            qualifier,
            annotations,
            name,
            args,
            builder.builder(),
        ))
    }
}

impl<'a> SimpleClassType<'a> {
    /// Build a [`SimpleClassType`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`SimpleClassType::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierReference<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        SimpleClassType {
            node_id: Cell::new(builder.node_id()),
            span,
            annotations: annotations.into_in(builder.allocator()),
            name,
            args,
        }
    }

    /// Build a [`SimpleClassType`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`SimpleClassType::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn boxed(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierReference<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, annotations, name, args, builder), &builder.allocator())
    }
}

impl<'a> QualifiedClassType<'a> {
    /// Build a [`QualifiedClassType`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`QualifiedClassType::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new(
        span: Span,
        qualifier: ClassType<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        QualifiedClassType {
            node_id: Cell::new(builder.node_id()),
            span,
            qualifier,
            annotations: annotations.into_in(builder.allocator()),
            name,
            args,
        }
    }

    /// Build a [`QualifiedClassType`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`QualifiedClassType::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn boxed(
        span: Span,
        qualifier: ClassType<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(
            Self::new(span, qualifier, annotations, name, args, builder),
            &builder.allocator(),
        )
    }
}

impl<'a> ArrayType<'a> {
    /// Build an [`ArrayType`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ArrayType::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `annotations`: Annotations before this dimension's `[`, e.g. `char @Nullable []` (JLS §10.1).
    #[inline]
    pub fn new(
        span: Span,
        element: Type<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ArrayType {
            node_id: Cell::new(builder.node_id()),
            span,
            element,
            annotations: annotations.into_in(builder.allocator()),
        }
    }

    /// Build an [`ArrayType`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ArrayType::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `annotations`: Annotations before this dimension's `[`, e.g. `char @Nullable []` (JLS §10.1).
    #[inline]
    pub fn boxed(
        span: Span,
        element: Type<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, element, annotations, builder), &builder.allocator())
    }
}

impl<'a> ArrayTypeDimension<'a> {
    /// Build an [`ArrayTypeDimension`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    #[inline]
    pub fn new(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ArrayTypeDimension {
            node_id: Cell::new(builder.node_id()),
            span,
            annotations: annotations.into_in(builder.allocator()),
        }
    }
}

impl<'a> TypeArguments<'a> {
    /// Build a [`TypeArguments`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`TypeArguments::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: From `<` to `>`.
    /// * `args`: Empty for `<>`.
    #[inline]
    pub fn new(
        span: Span,
        args: impl IntoIn<'a, Vec<'a, TypeArgument<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        TypeArguments {
            node_id: Cell::new(builder.node_id()),
            span,
            args: args.into_in(builder.allocator()),
        }
    }

    /// Build a [`TypeArguments`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`TypeArguments::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: From `<` to `>`.
    /// * `args`: Empty for `<>`.
    #[inline]
    pub fn boxed(
        span: Span,
        args: impl IntoIn<'a, Vec<'a, TypeArgument<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, args, builder), &builder.allocator())
    }
}

impl<'a> TypeArgument<'a> {
    /// Build a [`TypeArgument::Wildcard`].
    ///
    /// This node contains a [`WildcardTypeArgument`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`: `@A` in `List<@A ?>`.
    /// * `bound`
    #[inline]
    pub fn new_wildcard_type_argument(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        bound: Option<WildcardBound<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Wildcard(WildcardTypeArgument::boxed(span, annotations, bound, builder.builder()))
    }

    /// Build a [`TypeArgument::Primitive`].
    ///
    /// This node contains a [`PrimitiveType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `kind`
    #[inline]
    pub fn new_primitive_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        kind: PrimitiveTypeKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Primitive(PrimitiveType::boxed(span, annotations, kind, builder.builder()))
    }

    /// Build a [`TypeArgument::Array`].
    ///
    /// This node contains an [`ArrayType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `annotations`: Annotations before this dimension's `[`, e.g. `char @Nullable []` (JLS §10.1).
    #[inline]
    pub fn new_array_type(
        span: Span,
        element: Type<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Array(ArrayType::boxed(span, element, annotations, builder.builder()))
    }

    /// Build a [`TypeArgument::Simple`].
    ///
    /// This node contains a [`SimpleClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_simple_class_type(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierReference<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Simple(SimpleClassType::boxed(span, annotations, name, args, builder.builder()))
    }

    /// Build a [`TypeArgument::Qualified`].
    ///
    /// This node contains a [`QualifiedClassType`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `annotations`
    /// * `name`
    /// * `args`
    #[inline]
    pub fn new_qualified_class_type(
        span: Span,
        qualifier: ClassType<'a>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        args: Option<Box<'a, TypeArguments<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Qualified(QualifiedClassType::boxed(
            span,
            qualifier,
            annotations,
            name,
            args,
            builder.builder(),
        ))
    }
}

impl<'a> WildcardTypeArgument<'a> {
    /// Build a [`WildcardTypeArgument`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`WildcardTypeArgument::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`: `@A` in `List<@A ?>`.
    /// * `bound`
    #[inline]
    pub fn new(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        bound: Option<WildcardBound<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        WildcardTypeArgument {
            node_id: Cell::new(builder.node_id()),
            span,
            annotations: annotations.into_in(builder.allocator()),
            bound,
        }
    }

    /// Build a [`WildcardTypeArgument`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`WildcardTypeArgument::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `annotations`: `@A` in `List<@A ?>`.
    /// * `bound`
    #[inline]
    pub fn boxed(
        span: Span,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        bound: Option<WildcardBound<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, annotations, bound, builder), &builder.allocator())
    }
}

impl<'a> WildcardBound<'a> {
    /// Build a [`WildcardBound`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `kind`
    /// * `ty`
    #[inline]
    pub fn new(
        span: Span,
        kind: WildcardBoundKind,
        ty: Type<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        WildcardBound { node_id: Cell::new(builder.node_id()), span, kind, ty }
    }
}

impl<'a> IntLiteral<'a> {
    /// Build an [`IntLiteral`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`IntLiteral::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        IntLiteral { node_id: Cell::new(builder.node_id()), span, raw }
    }

    /// Build an [`IntLiteral`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`IntLiteral::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn boxed(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, raw, builder), &builder.allocator())
    }
}

impl<'a> FloatLiteral<'a> {
    /// Build a [`FloatLiteral`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`FloatLiteral::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        FloatLiteral { node_id: Cell::new(builder.node_id()), span, raw }
    }

    /// Build a [`FloatLiteral`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`FloatLiteral::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn boxed(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, raw, builder), &builder.allocator())
    }
}

impl<'a> CharLiteral<'a> {
    /// Build a [`CharLiteral`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`CharLiteral::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        CharLiteral { node_id: Cell::new(builder.node_id()), span, raw }
    }

    /// Build a [`CharLiteral`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`CharLiteral::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn boxed(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, raw, builder), &builder.allocator())
    }
}

impl<'a> StringLiteral<'a> {
    /// Build a [`StringLiteral`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`StringLiteral::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        StringLiteral { node_id: Cell::new(builder.node_id()), span, raw }
    }

    /// Build a [`StringLiteral`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`StringLiteral::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn boxed(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, raw, builder), &builder.allocator())
    }
}

impl BooleanLiteral {
    /// Build a [`BooleanLiteral`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`BooleanLiteral::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new<'a>(span: Span, value: bool, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        BooleanLiteral { node_id: Cell::new(builder.node_id()), span, value }
    }

    /// Build a [`BooleanLiteral`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`BooleanLiteral::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn boxed<'a>(span: Span, value: bool, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, value, builder), &builder.allocator())
    }
}

impl NullLiteral {
    /// Build a [`NullLiteral`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`NullLiteral::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        NullLiteral { node_id: Cell::new(builder.node_id()), span }
    }

    /// Build a [`NullLiteral`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`NullLiteral::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn boxed<'a>(span: Span, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, builder), &builder.allocator())
    }
}

impl<'a> IdentifierName<'a> {
    /// Build an [`IdentifierName`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`IdentifierName::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: Name after escape translation and BMP ignorable-character removal.
    #[inline]
    pub fn new(span: Span, name: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        IdentifierName { node_id: Cell::new(builder.node_id()), span, name }
    }

    /// Build an [`IdentifierName`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`IdentifierName::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: Name after escape translation and BMP ignorable-character removal.
    #[inline]
    pub fn boxed(span: Span, name: &'a str, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, name, builder), &builder.allocator())
    }
}

impl<'a> IdentifierReference<'a> {
    /// Build an [`IdentifierReference`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`IdentifierReference::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new(span: Span, name: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        IdentifierReference {
            node_id: Cell::new(builder.node_id()),
            span,
            name,
            reference_id: Default::default(),
        }
    }

    /// Build an [`IdentifierReference`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`IdentifierReference::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn boxed(span: Span, name: &'a str, builder: &impl GetAstBuilder<'a>) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, name, builder), &builder.allocator())
    }
}

impl<'a> BindingIdentifier<'a> {
    /// Build a [`BindingIdentifier`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`]. Includes unnamed `_`.
    #[inline]
    pub fn new(span: Span, name: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        BindingIdentifier {
            node_id: Cell::new(builder.node_id()),
            span,
            name,
            symbol_id: Default::default(),
        }
    }
}

impl<'a> LabelIdentifier<'a> {
    /// Build a [`LabelIdentifier`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new(span: Span, name: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        LabelIdentifier { node_id: Cell::new(builder.node_id()), span, name }
    }
}

impl<'a> Name<'a> {
    /// Build a [`Name::Identifier`].
    ///
    /// This node contains an [`IdentifierName`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: Name after escape translation and BMP ignorable-character removal.
    #[inline]
    pub fn new_identifier_name(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierName::boxed(span, name, builder.builder()))
    }

    /// Build a [`Name::Qualified`].
    ///
    /// This node contains a [`QualifiedName`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `name`
    #[inline]
    pub fn new_qualified_name(
        span: Span,
        qualifier: Name<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Qualified(QualifiedName::boxed(span, qualifier, name, builder.builder()))
    }
}

impl<'a> QualifiedName<'a> {
    /// Build a [`QualifiedName`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`QualifiedName::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `name`
    #[inline]
    pub fn new(
        span: Span,
        qualifier: Name<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        QualifiedName { node_id: Cell::new(builder.node_id()), span, qualifier, name }
    }

    /// Build a [`QualifiedName`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`QualifiedName::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `name`
    #[inline]
    pub fn boxed(
        span: Span,
        qualifier: Name<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, qualifier, name, builder), &builder.allocator())
    }
}

impl<'a> TypeName<'a> {
    /// Build a [`TypeName::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build a [`TypeName::Qualified`].
    ///
    /// This node contains a [`QualifiedTypeName`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `name`
    #[inline]
    pub fn new_qualified_type_name(
        span: Span,
        qualifier: TypeName<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Qualified(QualifiedTypeName::boxed(span, qualifier, name, builder.builder()))
    }
}

impl<'a> QualifiedTypeName<'a> {
    /// Build a [`QualifiedTypeName`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`QualifiedTypeName::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `name`
    #[inline]
    pub fn new(
        span: Span,
        qualifier: TypeName<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        QualifiedTypeName { node_id: Cell::new(builder.node_id()), span, qualifier, name }
    }

    /// Build a [`QualifiedTypeName`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`QualifiedTypeName::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `name`
    #[inline]
    pub fn boxed(
        span: Span,
        qualifier: TypeName<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, qualifier, name, builder), &builder.allocator())
    }
}

impl<'a> Modifier<'a> {
    /// Build a [`Modifier::Keyword`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `kind`
    #[inline]
    pub fn new_keyword_modifier(
        span: Span,
        kind: ModifierKind,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Keyword(KeywordModifier::new(span, kind, builder.builder()))
    }

    /// Build a [`Modifier::Annotation`].
    ///
    /// This node contains an [`Annotation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `args`: `None` for `@Foo`, `Some([])` for `@Foo()`.
    #[inline]
    pub fn new_annotation(
        span: Span,
        name: TypeName<'a>,
        args: Option<Vec<'a, AnnotationArgument<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Annotation(Annotation::boxed(span, name, args, builder.builder()))
    }
}

impl KeywordModifier {
    /// Build a [`KeywordModifier`].
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `kind`
    #[expect(unused_variables)]
    #[inline]
    pub fn new<'a>(span: Span, kind: ModifierKind, builder: &impl GetAstBuilder<'a>) -> Self {
        let builder = builder.builder();
        KeywordModifier { span, kind }
    }
}

impl<'a> Annotation<'a> {
    /// Build an [`Annotation`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`Annotation::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `args`: `None` for `@Foo`, `Some([])` for `@Foo()`.
    #[inline]
    pub fn new(
        span: Span,
        name: TypeName<'a>,
        args: Option<Vec<'a, AnnotationArgument<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        Annotation { node_id: Cell::new(builder.node_id()), span, name, args }
    }

    /// Build an [`Annotation`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`Annotation::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `args`: `None` for `@Foo`, `Some([])` for `@Foo()`.
    #[inline]
    pub fn boxed(
        span: Span,
        name: TypeName<'a>,
        args: Option<Vec<'a, AnnotationArgument<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, name, args, builder), &builder.allocator())
    }
}

impl<'a> AnnotationArgument<'a> {
    /// Build an [`AnnotationArgument::Pair`].
    ///
    /// This node contains an [`ElementValuePair`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `value`
    #[inline]
    pub fn new_element_value_pair(
        span: Span,
        name: IdentifierName<'a>,
        value: ElementValue<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Pair(ElementValuePair::boxed(span, name, value, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Annotation`].
    ///
    /// This node contains an [`Annotation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `args`: `None` for `@Foo`, `Some([])` for `@Foo()`.
    #[inline]
    pub fn new_annotation(
        span: Span,
        name: TypeName<'a>,
        args: Option<Vec<'a, AnnotationArgument<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Annotation(Annotation::boxed(span, name, args, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Array`].
    ///
    /// This node contains an [`ElementValueArrayInitializer`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `values`
    /// * `trailing_comma`
    #[inline]
    pub fn new_element_value_array_initializer(
        span: Span,
        values: impl IntoIn<'a, Vec<'a, ElementValue<'a>>>,
        trailing_comma: bool,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Array(ElementValueArrayInitializer::boxed(
            span,
            values,
            trailing_comma,
            builder.builder(),
        ))
    }

    /// Build an [`AnnotationArgument::IntLiteral`].
    ///
    /// This node contains an [`IntLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_int_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::IntLiteral(IntLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`AnnotationArgument::FloatLiteral`].
    ///
    /// This node contains a [`FloatLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_float_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::FloatLiteral(FloatLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`AnnotationArgument::CharLiteral`].
    ///
    /// This node contains a [`CharLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_char_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::CharLiteral(CharLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`AnnotationArgument::StringLiteral`].
    ///
    /// This node contains a [`StringLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_string_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::StringLiteral(StringLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`AnnotationArgument::BooleanLiteral`].
    ///
    /// This node contains a [`BooleanLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_boolean_literal(span: Span, value: bool, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::BooleanLiteral(BooleanLiteral::boxed(span, value, builder.builder()))
    }

    /// Build an [`AnnotationArgument::NullLiteral`].
    ///
    /// This node contains a [`NullLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_null_literal(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::NullLiteral(NullLiteral::boxed(span, builder.builder()))
    }

    /// Build an [`AnnotationArgument::This`].
    ///
    /// This node contains a [`ThisExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_this_expression(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::This(ThisExpression::boxed(span, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Super`].
    ///
    /// This node contains a [`Super`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_super(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Super(Super::boxed(span, builder.builder()))
    }

    /// Build an [`AnnotationArgument::QualifiedThis`].
    ///
    /// This node contains a [`QualifiedThis`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_this(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedThis(QualifiedThis::boxed(span, qualifier, builder.builder()))
    }

    /// Build an [`AnnotationArgument::QualifiedSuper`].
    ///
    /// This node contains a [`QualifiedSuper`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_super(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedSuper(QualifiedSuper::boxed(span, qualifier, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build an [`AnnotationArgument::FieldAccess`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::FieldAccess(FieldAccess::boxed(span, target, name, builder.builder()))
    }

    /// Build an [`AnnotationArgument::MethodCall`].
    ///
    /// This node contains a [`MethodCall`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `callee`
    /// * `args`
    #[inline]
    pub fn new_method_call(
        span: Span,
        callee: Callee<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodCall(MethodCall::boxed(span, callee, args, builder.builder()))
    }

    /// Build an [`AnnotationArgument::ClassLiteral`].
    ///
    /// This node contains a [`ClassLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    #[inline]
    pub fn new_class_literal(
        span: Span,
        ty: MethodResult<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassLiteral(ClassLiteral::boxed(span, ty, builder.builder()))
    }

    /// Build an [`AnnotationArgument::ClassInstanceCreation`].
    ///
    /// This node contains a [`ClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constructor_type_args`: Constructor type arguments in `new <T> Foo()`.
    /// * `ty`: The instantiated type, whose last name holds empty type arguments for
    /// * `args`
    /// * `body`: `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    #[inline]
    pub fn new_class_instance_creation(
        span: Span,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        ty: ClassType<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassInstanceCreation(ClassInstanceCreation::boxed(
            span,
            constructor_type_args,
            ty,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`AnnotationArgument::QualifiedClassInstanceCreation`].
    ///
    /// This node contains a [`QualifiedClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `constructor_type_args`: See [`ClassInstanceCreation::constructor_type_args`].
    /// * `annotations`
    /// * `name`
    /// * `type_args`: Empty for an inferred `<>` (JLS §15.9.1).
    /// * `args`
    /// * `body`: See [`ClassInstanceCreation::body`].
    #[inline]
    pub fn new_qualified_class_instance_creation(
        span: Span,
        qualifier: Expression<'a>,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedClassInstanceCreation(QualifiedClassInstanceCreation::boxed(
            span,
            qualifier,
            constructor_type_args,
            annotations,
            name,
            type_args,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`AnnotationArgument::ArrayCreation`].
    ///
    /// This node contains an [`ArrayCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `dims`: Dimensions in source order, sized before empty. All empty with an initializer.
    /// * `init`
    #[inline]
    pub fn new_array_creation(
        span: Span,
        element: Type<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayDimension<'a>>>,
        init: Option<ArrayInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayCreation(ArrayCreation::boxed(span, element, dims, init, builder.builder()))
    }

    /// Build an [`AnnotationArgument::ArrayAccess`].
    ///
    /// This node contains an [`ArrayAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn new_array_access(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayAccess(ArrayAccess::boxed(span, array, index, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Unary`].
    ///
    /// This node contains a [`UnaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `operand`
    #[inline]
    pub fn new_unary_expression(
        span: Span,
        op: UnaryOp,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Unary(UnaryExpression::boxed(span, op, operand, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Postfix`].
    ///
    /// This node contains a [`PostfixExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `op`
    #[inline]
    pub fn new_postfix_expression(
        span: Span,
        operand: Expression<'a>,
        op: PostfixOp,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Postfix(PostfixExpression::boxed(span, operand, op, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Cast`].
    ///
    /// This node contains a [`CastExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `additional_bounds`: Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    /// * `operand`
    #[inline]
    pub fn new_cast_expression(
        span: Span,
        ty: Type<'a>,
        additional_bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Cast(CastExpression::boxed(span, ty, additional_bounds, operand, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Instanceof`].
    ///
    /// This node contains an [`InstanceofExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `rhs`
    #[inline]
    pub fn new_instanceof_expression(
        span: Span,
        operand: Expression<'a>,
        rhs: InstanceofRhs<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Instanceof(InstanceofExpression::boxed(span, operand, rhs, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Binary`].
    ///
    /// This node contains a [`BinaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `lhs`
    /// * `rhs`
    #[inline]
    pub fn new_binary_expression(
        span: Span,
        op: BinaryOp,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Binary(BinaryExpression::boxed(span, op, lhs, rhs, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Assignment`].
    ///
    /// This node contains an [`AssignmentExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `op`
    /// * `value`
    #[inline]
    pub fn new_assignment_expression(
        span: Span,
        target: AssignmentTarget<'a>,
        op: AssignOp,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Assignment(AssignmentExpression::boxed(span, target, op, value, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Conditional`].
    ///
    /// This node contains a [`ConditionalExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `when_true`
    /// * `when_false`
    #[inline]
    pub fn new_conditional_expression(
        span: Span,
        condition: Expression<'a>,
        when_true: Expression<'a>,
        when_false: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Conditional(ConditionalExpression::boxed(
            span,
            condition,
            when_true,
            when_false,
            builder.builder(),
        ))
    }

    /// Build an [`AnnotationArgument::Switch`].
    ///
    /// This node contains a [`SwitchExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new_switch_expression(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Switch(SwitchExpression::boxed(span, selector, cases, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Parenthesized`].
    ///
    /// This node contains a [`ParenthesizedExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new_parenthesized_expression(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Parenthesized(ParenthesizedExpression::boxed(span, expression, builder.builder()))
    }

    /// Build an [`AnnotationArgument::Lambda`].
    ///
    /// This node contains a [`LambdaExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `params`
    /// * `body`
    #[inline]
    pub fn new_lambda_expression(
        span: Span,
        params: FormalParameters<'a>,
        body: LambdaBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Lambda(LambdaExpression::boxed(span, params, body, builder.builder()))
    }

    /// Build an [`AnnotationArgument::MethodReference`].
    ///
    /// This node contains a [`MethodReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`: Type arguments after `::`.
    /// * `method`: The referenced method, or `None` for `::new`.
    #[inline]
    pub fn new_method_reference(
        span: Span,
        target: MethodReferenceTarget<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        method: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodReference(MethodReference::boxed(
            span,
            target,
            type_args,
            method,
            builder.builder(),
        ))
    }
}

impl<'a> ElementValuePair<'a> {
    /// Build an [`ElementValuePair`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ElementValuePair::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `value`
    #[inline]
    pub fn new(
        span: Span,
        name: IdentifierName<'a>,
        value: ElementValue<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ElementValuePair { node_id: Cell::new(builder.node_id()), span, name, value }
    }

    /// Build an [`ElementValuePair`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ElementValuePair::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `value`
    #[inline]
    pub fn boxed(
        span: Span,
        name: IdentifierName<'a>,
        value: ElementValue<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, name, value, builder), &builder.allocator())
    }
}

impl<'a> ElementValue<'a> {
    /// Build an [`ElementValue::Annotation`].
    ///
    /// This node contains an [`Annotation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`
    /// * `args`: `None` for `@Foo`, `Some([])` for `@Foo()`.
    #[inline]
    pub fn new_annotation(
        span: Span,
        name: TypeName<'a>,
        args: Option<Vec<'a, AnnotationArgument<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Annotation(Annotation::boxed(span, name, args, builder.builder()))
    }

    /// Build an [`ElementValue::Array`].
    ///
    /// This node contains an [`ElementValueArrayInitializer`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `values`
    /// * `trailing_comma`
    #[inline]
    pub fn new_element_value_array_initializer(
        span: Span,
        values: impl IntoIn<'a, Vec<'a, ElementValue<'a>>>,
        trailing_comma: bool,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Array(ElementValueArrayInitializer::boxed(
            span,
            values,
            trailing_comma,
            builder.builder(),
        ))
    }

    /// Build an [`ElementValue::IntLiteral`].
    ///
    /// This node contains an [`IntLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_int_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::IntLiteral(IntLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`ElementValue::FloatLiteral`].
    ///
    /// This node contains a [`FloatLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_float_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::FloatLiteral(FloatLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`ElementValue::CharLiteral`].
    ///
    /// This node contains a [`CharLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_char_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::CharLiteral(CharLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`ElementValue::StringLiteral`].
    ///
    /// This node contains a [`StringLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `raw`
    #[inline]
    pub fn new_string_literal(span: Span, raw: &'a str, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::StringLiteral(StringLiteral::boxed(span, raw, builder.builder()))
    }

    /// Build an [`ElementValue::BooleanLiteral`].
    ///
    /// This node contains a [`BooleanLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `value`
    #[inline]
    pub fn new_boolean_literal(span: Span, value: bool, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::BooleanLiteral(BooleanLiteral::boxed(span, value, builder.builder()))
    }

    /// Build an [`ElementValue::NullLiteral`].
    ///
    /// This node contains a [`NullLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_null_literal(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::NullLiteral(NullLiteral::boxed(span, builder.builder()))
    }

    /// Build an [`ElementValue::This`].
    ///
    /// This node contains a [`ThisExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_this_expression(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::This(ThisExpression::boxed(span, builder.builder()))
    }

    /// Build an [`ElementValue::Super`].
    ///
    /// This node contains a [`Super`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    #[inline]
    pub fn new_super(span: Span, builder: &impl GetAstBuilder<'a>) -> Self {
        Self::Super(Super::boxed(span, builder.builder()))
    }

    /// Build an [`ElementValue::QualifiedThis`].
    ///
    /// This node contains a [`QualifiedThis`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_this(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedThis(QualifiedThis::boxed(span, qualifier, builder.builder()))
    }

    /// Build an [`ElementValue::QualifiedSuper`].
    ///
    /// This node contains a [`QualifiedSuper`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    #[inline]
    pub fn new_qualified_super(
        span: Span,
        qualifier: TypeName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedSuper(QualifiedSuper::boxed(span, qualifier, builder.builder()))
    }

    /// Build an [`ElementValue::Identifier`].
    ///
    /// This node contains an [`IdentifierReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `name`: See [`IdentifierName::name`].
    #[inline]
    pub fn new_identifier_reference(
        span: Span,
        name: &'a str,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Identifier(IdentifierReference::boxed(span, name, builder.builder()))
    }

    /// Build an [`ElementValue::FieldAccess`].
    ///
    /// This node contains a [`FieldAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `name`
    #[inline]
    pub fn new_field_access(
        span: Span,
        target: Expression<'a>,
        name: IdentifierName<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::FieldAccess(FieldAccess::boxed(span, target, name, builder.builder()))
    }

    /// Build an [`ElementValue::MethodCall`].
    ///
    /// This node contains a [`MethodCall`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `callee`
    /// * `args`
    #[inline]
    pub fn new_method_call(
        span: Span,
        callee: Callee<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodCall(MethodCall::boxed(span, callee, args, builder.builder()))
    }

    /// Build an [`ElementValue::ClassLiteral`].
    ///
    /// This node contains a [`ClassLiteral`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    #[inline]
    pub fn new_class_literal(
        span: Span,
        ty: MethodResult<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassLiteral(ClassLiteral::boxed(span, ty, builder.builder()))
    }

    /// Build an [`ElementValue::ClassInstanceCreation`].
    ///
    /// This node contains a [`ClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `constructor_type_args`: Constructor type arguments in `new <T> Foo()`.
    /// * `ty`: The instantiated type, whose last name holds empty type arguments for
    /// * `args`
    /// * `body`: `None` for `new Foo()`, empty members for `new Foo() {}` (JLS §15.9.5).
    #[inline]
    pub fn new_class_instance_creation(
        span: Span,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        ty: ClassType<'a>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ClassInstanceCreation(ClassInstanceCreation::boxed(
            span,
            constructor_type_args,
            ty,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`ElementValue::QualifiedClassInstanceCreation`].
    ///
    /// This node contains a [`QualifiedClassInstanceCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `qualifier`
    /// * `constructor_type_args`: See [`ClassInstanceCreation::constructor_type_args`].
    /// * `annotations`
    /// * `name`
    /// * `type_args`: Empty for an inferred `<>` (JLS §15.9.1).
    /// * `args`
    /// * `body`: See [`ClassInstanceCreation::body`].
    #[inline]
    pub fn new_qualified_class_instance_creation(
        span: Span,
        qualifier: Expression<'a>,
        constructor_type_args: Option<Box<'a, TypeArguments<'a>>>,
        annotations: impl IntoIn<'a, Vec<'a, Annotation<'a>>>,
        name: IdentifierName<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        args: impl IntoIn<'a, Vec<'a, Expression<'a>>>,
        body: Option<ClassBody<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::QualifiedClassInstanceCreation(QualifiedClassInstanceCreation::boxed(
            span,
            qualifier,
            constructor_type_args,
            annotations,
            name,
            type_args,
            args,
            body,
            builder.builder(),
        ))
    }

    /// Build an [`ElementValue::ArrayCreation`].
    ///
    /// This node contains an [`ArrayCreation`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `element`
    /// * `dims`: Dimensions in source order, sized before empty. All empty with an initializer.
    /// * `init`
    #[inline]
    pub fn new_array_creation(
        span: Span,
        element: Type<'a>,
        dims: impl IntoIn<'a, Vec<'a, ArrayDimension<'a>>>,
        init: Option<ArrayInitializer<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayCreation(ArrayCreation::boxed(span, element, dims, init, builder.builder()))
    }

    /// Build an [`ElementValue::ArrayAccess`].
    ///
    /// This node contains an [`ArrayAccess`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `array`
    /// * `index`
    #[inline]
    pub fn new_array_access(
        span: Span,
        array: Expression<'a>,
        index: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::ArrayAccess(ArrayAccess::boxed(span, array, index, builder.builder()))
    }

    /// Build an [`ElementValue::Unary`].
    ///
    /// This node contains a [`UnaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `operand`
    #[inline]
    pub fn new_unary_expression(
        span: Span,
        op: UnaryOp,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Unary(UnaryExpression::boxed(span, op, operand, builder.builder()))
    }

    /// Build an [`ElementValue::Postfix`].
    ///
    /// This node contains a [`PostfixExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `op`
    #[inline]
    pub fn new_postfix_expression(
        span: Span,
        operand: Expression<'a>,
        op: PostfixOp,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Postfix(PostfixExpression::boxed(span, operand, op, builder.builder()))
    }

    /// Build an [`ElementValue::Cast`].
    ///
    /// This node contains a [`CastExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `ty`
    /// * `additional_bounds`: Additional intersection-cast bounds, e.g. `Serializable` in `(Runnable & Serializable) value`.
    /// * `operand`
    #[inline]
    pub fn new_cast_expression(
        span: Span,
        ty: Type<'a>,
        additional_bounds: impl IntoIn<'a, Vec<'a, Type<'a>>>,
        operand: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Cast(CastExpression::boxed(span, ty, additional_bounds, operand, builder.builder()))
    }

    /// Build an [`ElementValue::Instanceof`].
    ///
    /// This node contains an [`InstanceofExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `operand`
    /// * `rhs`
    #[inline]
    pub fn new_instanceof_expression(
        span: Span,
        operand: Expression<'a>,
        rhs: InstanceofRhs<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Instanceof(InstanceofExpression::boxed(span, operand, rhs, builder.builder()))
    }

    /// Build an [`ElementValue::Binary`].
    ///
    /// This node contains a [`BinaryExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `op`
    /// * `lhs`
    /// * `rhs`
    #[inline]
    pub fn new_binary_expression(
        span: Span,
        op: BinaryOp,
        lhs: Expression<'a>,
        rhs: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Binary(BinaryExpression::boxed(span, op, lhs, rhs, builder.builder()))
    }

    /// Build an [`ElementValue::Assignment`].
    ///
    /// This node contains an [`AssignmentExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `op`
    /// * `value`
    #[inline]
    pub fn new_assignment_expression(
        span: Span,
        target: AssignmentTarget<'a>,
        op: AssignOp,
        value: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Assignment(AssignmentExpression::boxed(span, target, op, value, builder.builder()))
    }

    /// Build an [`ElementValue::Conditional`].
    ///
    /// This node contains a [`ConditionalExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `condition`
    /// * `when_true`
    /// * `when_false`
    #[inline]
    pub fn new_conditional_expression(
        span: Span,
        condition: Expression<'a>,
        when_true: Expression<'a>,
        when_false: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Conditional(ConditionalExpression::boxed(
            span,
            condition,
            when_true,
            when_false,
            builder.builder(),
        ))
    }

    /// Build an [`ElementValue::Switch`].
    ///
    /// This node contains a [`SwitchExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `selector`
    /// * `cases`
    #[inline]
    pub fn new_switch_expression(
        span: Span,
        selector: Expression<'a>,
        cases: impl IntoIn<'a, Vec<'a, SwitchCase<'a>>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Switch(SwitchExpression::boxed(span, selector, cases, builder.builder()))
    }

    /// Build an [`ElementValue::Parenthesized`].
    ///
    /// This node contains a [`ParenthesizedExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `expression`
    #[inline]
    pub fn new_parenthesized_expression(
        span: Span,
        expression: Expression<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Parenthesized(ParenthesizedExpression::boxed(span, expression, builder.builder()))
    }

    /// Build an [`ElementValue::Lambda`].
    ///
    /// This node contains a [`LambdaExpression`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `params`
    /// * `body`
    #[inline]
    pub fn new_lambda_expression(
        span: Span,
        params: FormalParameters<'a>,
        body: LambdaBody<'a>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::Lambda(LambdaExpression::boxed(span, params, body, builder.builder()))
    }

    /// Build an [`ElementValue::MethodReference`].
    ///
    /// This node contains a [`MethodReference`] that will be stored in the memory arena.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `target`
    /// * `type_args`: Type arguments after `::`.
    /// * `method`: The referenced method, or `None` for `::new`.
    #[inline]
    pub fn new_method_reference(
        span: Span,
        target: MethodReferenceTarget<'a>,
        type_args: Option<Box<'a, TypeArguments<'a>>>,
        method: Option<IdentifierName<'a>>,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        Self::MethodReference(MethodReference::boxed(
            span,
            target,
            type_args,
            method,
            builder.builder(),
        ))
    }
}

impl<'a> ElementValueArrayInitializer<'a> {
    /// Build an [`ElementValueArrayInitializer`].
    ///
    /// If you want the built node to be allocated in the memory arena,
    /// use [`ElementValueArrayInitializer::boxed`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `values`
    /// * `trailing_comma`
    #[inline]
    pub fn new(
        span: Span,
        values: impl IntoIn<'a, Vec<'a, ElementValue<'a>>>,
        trailing_comma: bool,
        builder: &impl GetAstBuilder<'a>,
    ) -> Self {
        let builder = builder.builder();
        ElementValueArrayInitializer {
            node_id: Cell::new(builder.node_id()),
            span,
            values: values.into_in(builder.allocator()),
            trailing_comma,
        }
    }

    /// Build an [`ElementValueArrayInitializer`], and store it in the memory arena.
    ///
    /// Returns a [`Box`] containing the newly-allocated node.
    /// If you want a stack-allocated node, use [`ElementValueArrayInitializer::new`] instead.
    ///
    /// ## Parameters
    /// * `span`: The [`Span`] covering this node
    /// * `values`
    /// * `trailing_comma`
    #[inline]
    pub fn boxed(
        span: Span,
        values: impl IntoIn<'a, Vec<'a, ElementValue<'a>>>,
        trailing_comma: bool,
        builder: &impl GetAstBuilder<'a>,
    ) -> Box<'a, Self> {
        let builder = builder.builder();
        Box::new_in(Self::new(span, values, trailing_comma, builder), &builder.allocator())
    }
}
