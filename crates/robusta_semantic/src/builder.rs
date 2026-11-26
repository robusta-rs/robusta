//! Semantic Builder

use std::{
    cell::{Cell, OnceCell},
    cmp::Reverse,
};

use robusta_allocator::{Address, GetAddress, UnstableAddress};
use robusta_ast::{
    AnnotationInterfaceDeclaration, AnnotationInterfaceElementDeclaration, AstKind,
    BindingIdentifier, CatchClause, ClassDeclaration, ClassType, CompactConstructorDeclaration,
    CompilationUnit, EnhancedForStatement, EnumConstant, EnumDeclaration, FormalParameter,
    IdentifierReference, ImportDeclaration, ImportKind, InterfaceDeclaration, MethodDeclaration,
    ModifierKind, RecordComponent, RecordDeclaration, ResourceDeclaration, Statement, Type,
    TypeDeclarationKind, TypeParameter, TypePattern, VariableDeclarator, VariableInitializer,
};
use robusta_ast_visit::Visit;
use robusta_span::{GetSpan, Span};
use robusta_str::Ident;
use robusta_syntax::operator::{AssignOp, UnaryOp};

use crate::{
    AstNodes, NodeId, Reference, ReferenceKind, ScopeId, Scoping, Semantic, Symbol, SymbolId,
    SymbolKind,
    binder::{Binder, has_modifier},
    pattern_scope::{NameContext, NameValue, can_complete_normally},
};

/// Builds [`Semantic`] in one traversal of a compilation unit.
#[derive(Debug)]
pub struct SemanticBuilder<'a> {
    pub(crate) current_node_id: NodeId,
    pub(crate) current_scope_id: ScopeId,
    pub(crate) nodes: AstNodes<'a>,
    pub(crate) scoping: Scoping<'a>,
    /// Where each symbol is usable, gathered until symbols are indexed.
    symbol_parts: Vec<(SymbolId, Span)>,
    /// Pattern variables whose last part follows an `if` and holds only if
    /// this unmatched branch cannot complete normally.
    pub(crate) pending_patterns: Vec<(SymbolId, &'a Statement<'a>)>,
}

impl Default for SemanticBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SemanticBuilder<'a> {
    pub fn new() -> Self {
        Self {
            current_node_id: NodeId::ROOT,
            current_scope_id: ScopeId::ROOT,
            nodes: AstNodes::default(),
            scoping: Scoping::default(),
            symbol_parts: Vec::new(),
            pending_patterns: Vec::new(),
        }
    }

    /// Build nodes, scopes, symbols, and references in source order.
    pub fn build(mut self, source: &'a str, unit: &'a CompilationUnit<'a>) -> Semantic<'a> {
        self.visit_compilation_unit(unit);

        self.scoping.index_scopes();
        self.scoping.index_symbols(&self.symbol_parts);
        self.end_pattern_scopes();
        self.link_shadowing();
        self.resolve_all_references();
        self.scoping.index_references();
        Semantic {
            source_text: source,
            comments: &unit.comments,
            javadocs: OnceCell::new(),
            nodes: self.nodes,
            scoping: self.scoping,
        }
    }

    pub(crate) fn parent_kind(&self) -> AstKind<'a> {
        self.nodes.parent_kind(self.current_node_id)
    }

    /// Declare `name` in the current scope, usable throughout it.
    pub(crate) fn declare_symbol(
        &mut self,
        name: &BindingIdentifier<'a>,
        kind: SymbolKind,
    ) -> Option<SymbolId> {
        let window = Span::new(0, self.scoping.scope(self.current_scope_id).span.end);
        self.declare_symbol_within(name, kind, window)
    }

    /// Declare a method `name` in the current scope. Methods of one name share a
    /// symbol, which is private only while all of them are.
    pub(crate) fn declare_method(&mut self, name: &BindingIdentifier<'a>, is_private: bool) {
        let declared = self
            .scoping
            .symbols_for_name(self.current_scope_id, Ident::from(name.name))
            .find(|(_, symbol)| symbol.kind == SymbolKind::Method)
            .map(|(id, _)| id);
        if let Some(symbol_id) = declared {
            self.scoping.add_symbol_redeclaration(symbol_id, self.current_node_id, name.span);
            self.scoping.symbols[symbol_id].is_private &= is_private;
            name.symbol_id.set(Some(symbol_id));
        } else if let Some(symbol_id) = self.declare_symbol(name, SymbolKind::Method) {
            self.scoping.symbols[symbol_id].is_private = is_private;
        }
    }

    /// Declare `name` in the current scope, usable only within `window`.
    pub(crate) fn declare_symbol_within(
        &mut self,
        name: &BindingIdentifier<'a>,
        kind: SymbolKind,
        window: Span,
    ) -> Option<SymbolId> {
        self.declare_symbol_in_parts(name, kind, self.current_scope_id, &[window])
    }

    /// Declare `name` in `scope_id`, usable only within `parts`.
    pub(crate) fn declare_symbol_in_parts(
        &mut self,
        name: &BindingIdentifier<'a>,
        kind: SymbolKind,
        scope_id: ScopeId,
        parts: &[Span],
    ) -> Option<SymbolId> {
        let symbol_id =
            self.create_symbol(Ident::from(name.name), name.span, kind, scope_id, parts);
        name.symbol_id.set(symbol_id);
        symbol_id
    }

    /// Declare a compact constructor's parameter for `component`, which stays
    /// the binding of the record component (JLS 8.10.4.2).
    pub(crate) fn declare_compact_constructor_parameter(
        &mut self,
        component: &BindingIdentifier<'a>,
        window: Span,
    ) {
        self.create_symbol(
            Ident::from(component.name),
            component.span,
            SymbolKind::Parameter,
            self.current_scope_id,
            &[window],
        );
    }

    fn create_symbol(
        &mut self,
        name: Ident<'a>,
        name_span: Span,
        kind: SymbolKind,
        scope_id: ScopeId,
        parts: &[Span],
    ) -> Option<SymbolId> {
        // The unnamed `_` declares nothing.
        if name == "_" {
            return None;
        }
        let symbol_id = self.scoping.create_symbol_with_binding(Symbol {
            name,
            name_span,
            kind,
            scope_id,
            declaration_node_id: self.current_node_id,
            is_private: false,
            shadows: None,
        });
        self.symbol_parts.extend(parts.iter().map(|&part| (symbol_id, part)));
        Some(symbol_id)
    }

    /// Open a scope owned by the current node.
    fn open_scope(&mut self, scope_id: &Cell<Option<ScopeId>>, span: Span, is_type: bool) {
        self.current_scope_id = self.scoping.add_scope(
            Some(self.current_scope_id),
            self.current_node_id,
            span,
            is_type,
        );
        scope_id.set(Some(self.current_scope_id));
    }

    fn reference_identifier(&mut self, name: &IdentifierReference<'a>) {
        let reference_id = self.scoping.create_reference(Reference {
            name: Ident::from(name.name),
            span: name.span,
            // The builder borrows the AST, so the name stays in place.
            kind: self.reference_kind(name.unstable_address()),
            scope_id: self.current_scope_id,
            node_id: self.current_node_id,
            symbol_id: None,
        });
        name.reference_id.set(Some(reference_id));
    }

    /// What the context of the name at `address` expects it to be (JLS 6.5).
    fn reference_kind(&self, address: Address) -> ReferenceKind {
        match self.parent_kind() {
            AstKind::SimpleClassType(_)
            | AstKind::QualifiedTypeName(_)
            | AstKind::QualifiedThis(_)
            | AstKind::QualifiedSuper(_)
            | AstKind::UsesDirective(_)
            | AstKind::ProvidesDirective(_)
            | AstKind::ClassDeclaration(_)
            | AstKind::InterfaceDeclaration(_) => ReferenceKind::Type,
            // The other name directly in an annotation is an element value.
            AstKind::Annotation(annotation) if annotation.name.address() == address => {
                ReferenceKind::Type
            }
            AstKind::MethodCall(call) if call.callee.address() == address => ReferenceKind::Method,
            _ => self.expression_reference_kind(address),
        }
    }

    /// Whether the name at `address` is read, written, or both: only the
    /// target of an assignment or the operand of an update writes it. An
    /// update's operand may be a parenthesized variable (JLS 15.8.5), so
    /// parentheses are looked through.
    fn expression_reference_kind(&self, address: Address) -> ReferenceKind {
        let mut operand = address;
        for ancestor in self.nodes.ancestor_kinds(self.current_node_id) {
            match ancestor {
                AstKind::ParenthesizedExpression(_) => operand = ancestor.address(),
                AstKind::AssignmentExpression(assignment)
                    if assignment.target.address() == operand =>
                {
                    return if assignment.op == AssignOp::Assign {
                        ReferenceKind::Write
                    } else {
                        ReferenceKind::ReadWrite
                    };
                }
                AstKind::UnaryExpression(unary)
                    if matches!(unary.op, UnaryOp::PreIncrement | UnaryOp::PreDecrement)
                        && unary.operand.address() == operand =>
                {
                    return ReferenceKind::ReadWrite;
                }
                AstKind::PostfixExpression(postfix) if postfix.operand.address() == operand => {
                    return ReferenceKind::ReadWrite;
                }
                _ => break,
            }
        }
        ReferenceKind::Read
    }

    /// A name in one unmatched branch can fall in the part after another `if`
    /// only when that `if`'s branch ends first, so deciding in order of branch
    /// end decides that part first.
    fn end_pattern_scopes(&mut self) {
        let mut pending = std::mem::take(&mut self.pending_patterns);
        pending.sort_unstable_by_key(|(_, unmatched)| unmatched.span().end);
        for (symbol_id, unmatched) in pending {
            if can_complete_normally(unmatched, &*self) {
                self.scoping.pop_symbol_part(symbol_id);
            }
        }
    }

    fn link_shadowing(&mut self) {
        for symbol_id in self.scoping.symbols.indices() {
            let symbol = &self.scoping.symbols[symbol_id];
            if symbol.kind.is_variable() {
                let shadows = self.shadowed_variable(symbol);
                self.scoping.symbols[symbol_id].shadows = shadows;
            }
        }
    }

    /// The latest same-name variable in the nearest enclosing scope that has one.
    fn shadowed_variable(&self, symbol: &Symbol<'a>) -> Option<SymbolId> {
        self.scoping.scope_ancestors(symbol.scope_id).skip(1).find_map(|scope_id| {
            self.scoping
                .symbols_for_name(scope_id, symbol.name)
                .filter(|(_, candidate)| candidate.kind.is_variable())
                .last()
                .map(|(id, _)| id)
        })
    }

    fn resolve_all_references(&mut self) {
        for reference_id in self.scoping.references.indices() {
            let reference = &self.scoping.references[reference_id];
            let symbol_id = self.resolve(
                reference.name,
                reference.kind,
                reference.scope_id,
                reference.span.start,
            );
            self.scoping.references[reference_id].symbol_id = symbol_id;
        }
    }

    /// The binding of `name` at `position`, looked up from `scope_id` outward.
    fn resolve(
        &self,
        name: Ident<'a>,
        kind: ReferenceKind,
        scope_id: ScopeId,
        position: u32,
    ) -> Option<SymbolId> {
        self.scoping.scope_ancestors(scope_id).find_map(|scope_id| {
            // The binding whose part starts latest before `position` wins.
            // Ties keep the first.
            self.scoping
                .symbols_for_name(scope_id, name)
                .filter(|(_, symbol)| compatible(symbol.kind, kind))
                .filter_map(|(id, symbol)| {
                    let part = self
                        .scoping
                        .symbol_parts(id)
                        .iter()
                        .find(|part| part.start <= position && position <= part.end)?;
                    Some((id, symbol_priority(symbol.kind), part.start))
                })
                .min_by_key(|&(_, priority, start)| (priority, Reverse(start)))
                .map(|(id, ..)| id)
        })
    }

    /// Whether `scope_id` is the body of a type that may inherit fields
    /// (JLS 8.2, 9.2).
    fn may_inherit_fields(&self, scope_id: ScopeId) -> bool {
        let scope = self.scoping.scope(scope_id);
        scope.is_type
            && match self.nodes.parent_kind(scope.owner_node_id) {
                AstKind::ClassDeclaration(class) => {
                    class.superclass.is_some() || !class.interfaces.is_empty()
                }
                AstKind::InterfaceDeclaration(interface) => !interface.extends.is_empty(),
                AstKind::EnumDeclaration(declaration) => !declaration.interfaces.is_empty(),
                AstKind::RecordDeclaration(record) => !record.interfaces.is_empty(),
                AstKind::AnnotationInterfaceDeclaration(_) | AstKind::CompilationUnit(_) => false,
                // An anonymous class or an enum constant's body.
                _ => true,
            }
    }
}

impl<'a> NameContext<'a> for SemanticBuilder<'a> {
    fn name_value(&self, name: &IdentifierReference<'a>) -> NameValue<'a> {
        let position = name.span.start;
        let reference = self.scoping.reference(name.reference_id());
        let scope_id = reference.scope_id;
        let Some(symbol_id) = self.resolve(reference.name, ReferenceKind::Read, scope_id, position)
        else {
            return NameValue::Unknown;
        };
        let symbol = &self.scoping.symbols[symbol_id];
        // A class between the name and its declaration may inherit a field
        // that shadows the declaration (JLS 6.4.1).
        if self
            .scoping
            .scope_ancestors(scope_id)
            .take_while(|&id| id != symbol.scope_id)
            .any(|id| self.may_inherit_fields(id))
        {
            return NameValue::Unknown;
        }
        let AstKind::VariableDeclarator(declarator) = self.nodes.kind(symbol.declaration_node_id)
        else {
            return if symbol.kind.is_variable() {
                NameValue::Variable
            } else {
                NameValue::Unknown
            };
        };
        let (modifiers, ty) = match self.nodes.parent_kind(symbol.declaration_node_id) {
            AstKind::FieldDeclaration(field) => (&field.modifiers, Some(&field.ty)),
            // A constant initializer gives `var` a primitive or `String` type.
            AstKind::LocalVariableDeclaration(local) => (&local.modifiers, local.ty.as_type()),
            _ => return NameValue::Variable,
        };
        // Interface fields are implicitly final (JLS 9.3).
        let is_final = has_modifier(modifiers, ModifierKind::Final)
            || matches!(
                self.nodes.kind(self.scoping.scope(symbol.scope_id).owner_node_id),
                AstKind::InterfaceBody(_) | AstKind::AnnotationInterfaceBody(_)
            );
        match declarator.init.as_ref().and_then(VariableInitializer::as_expression) {
            Some(initializer)
                if is_final
                    && declarator.dims.is_empty()
                    && ty.is_none_or(is_primitive_or_string) =>
            {
                NameValue::Initialized { start: declarator.span.start, initializer }
            }
            _ => NameValue::Variable,
        }
    }
}

impl<'a> Visit<'a> for SemanticBuilder<'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        self.current_node_id =
            self.nodes.add_node(kind, self.current_scope_id, self.current_node_id);
        kind.set_node_id(self.current_node_id);
    }

    fn leave_node(&mut self, _kind: AstKind<'a>) {
        self.current_node_id = self.nodes.parent_id(self.current_node_id);
    }

    fn enter_scope(&mut self, scope_id: &Cell<Option<ScopeId>>) {
        let kind = self.nodes.kind(self.current_node_id);
        self.open_scope(scope_id, kind.span(), is_type_scope(kind));
    }

    fn leave_scope(&mut self) {
        if let Some(parent_id) = self.scoping.scope(self.current_scope_id).parent_id {
            self.current_scope_id = parent_id;
        }
    }

    fn visit_compilation_unit(&mut self, unit: &'a CompilationUnit<'a>) {
        // The root node is its own parent. The root scope has no parent.
        let kind = AstKind::CompilationUnit(unit);
        kind.set_node_id(NodeId::ROOT);
        self.nodes.add_compilation_unit_node(kind, self.current_scope_id);
        self.current_scope_id =
            self.scoping.add_scope(None, self.current_node_id, unit.span, false);
        unit.scope_id.set(Some(self.current_scope_id));
        self.visit_compilation_unit_content(&unit.content);
    }

    fn visit_import_declaration(&mut self, decl: &'a ImportDeclaration<'a>) {
        let kind = AstKind::ImportDeclaration(decl);
        self.enter_node(kind);
        decl.bind(self);
        self.visit_name(&decl.name);
        if let Some(local) = &decl.local {
            self.visit_binding_identifier(local);
        }
        self.leave_node(kind);
    }

    fn visit_class_declaration(&mut self, decl: &'a ClassDeclaration<'a>) {
        let kind = AstKind::ClassDeclaration(decl);
        self.enter_node(kind);
        TypeDeclarationKind::Class(decl).bind(self);
        self.visit_modifiers(&decl.modifiers);
        self.visit_binding_identifier(&decl.name);
        self.enter_scope(&decl.scope_id);
        if let Some(type_params) = &decl.type_params {
            self.visit_type_parameters(type_params);
        }
        if let Some(superclass) = &decl.superclass {
            self.visit_type(superclass);
        }
        self.visit_types(&decl.interfaces);
        self.visit_type_names(&decl.permits);
        self.visit_class_body(&decl.body);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_interface_declaration(&mut self, decl: &'a InterfaceDeclaration<'a>) {
        let kind = AstKind::InterfaceDeclaration(decl);
        self.enter_node(kind);
        TypeDeclarationKind::Interface(decl).bind(self);
        self.visit_modifiers(&decl.modifiers);
        self.visit_binding_identifier(&decl.name);
        self.enter_scope(&decl.scope_id);
        if let Some(type_params) = &decl.type_params {
            self.visit_type_parameters(type_params);
        }
        self.visit_types(&decl.extends);
        self.visit_type_names(&decl.permits);
        self.visit_interface_body(&decl.body);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_enum_declaration(&mut self, decl: &'a EnumDeclaration<'a>) {
        let kind = AstKind::EnumDeclaration(decl);
        self.enter_node(kind);
        TypeDeclarationKind::Enum(decl).bind(self);
        self.visit_modifiers(&decl.modifiers);
        self.visit_binding_identifier(&decl.name);
        self.visit_types(&decl.interfaces);
        self.visit_enum_body(&decl.body);
        self.leave_node(kind);
    }

    fn visit_enum_constant(&mut self, constant: &'a EnumConstant<'a>) {
        let kind = AstKind::EnumConstant(constant);
        self.enter_node(kind);
        constant.bind(self);
        self.visit_modifiers(&constant.modifiers);
        self.visit_binding_identifier(&constant.name);
        if let Some(args) = &constant.args {
            self.visit_expressions(args);
        }
        if let Some(body) = &constant.body {
            self.visit_class_body(body);
        }
        self.leave_node(kind);
    }

    /// A record's members are also in scope in its header (JLS 6.3), so the header and the body
    /// share the body's scope, with the `implements` clause between them outside it.
    fn visit_record_declaration(&mut self, record: &'a RecordDeclaration<'a>) {
        let kind = AstKind::RecordDeclaration(record);
        self.enter_node(kind);
        TypeDeclarationKind::Record(record).bind(self);
        self.visit_modifiers(&record.modifiers);
        self.visit_binding_identifier(&record.name);
        self.enter_scope(&record.scope_id);
        if let Some(type_params) = &record.type_params {
            self.visit_type_parameters(type_params);
        }

        let body = &record.body;
        self.open_scope(&body.scope_id, body.span, true);
        self.visit_record_header(&record.header);
        self.leave_scope();

        self.visit_types(&record.interfaces);

        let body_kind = AstKind::ClassBody(body);
        self.enter_node(body_kind);
        self.current_scope_id = body.scope_id();
        self.scoping.scopes[self.current_scope_id].owner_node_id = self.current_node_id;
        self.visit_class_members(&body.members);
        self.leave_scope();
        self.leave_node(body_kind);

        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_record_component(&mut self, component: &'a RecordComponent<'a>) {
        let kind = AstKind::RecordComponent(component);
        self.enter_node(kind);
        component.bind(self);
        self.visit_modifiers(&component.modifiers);
        self.visit_type(&component.ty);
        self.visit_annotations(&component.varargs_annotations);
        self.visit_binding_identifier(&component.name);
        self.leave_node(kind);
    }

    fn visit_annotation_interface_declaration(
        &mut self,
        decl: &'a AnnotationInterfaceDeclaration<'a>,
    ) {
        let kind = AstKind::AnnotationInterfaceDeclaration(decl);
        self.enter_node(kind);
        TypeDeclarationKind::AnnotationInterface(decl).bind(self);
        self.visit_modifiers(&decl.modifiers);
        self.visit_binding_identifier(&decl.name);
        self.visit_annotation_interface_body(&decl.body);
        self.leave_node(kind);
    }

    fn visit_annotation_interface_element_declaration(
        &mut self,
        decl: &'a AnnotationInterfaceElementDeclaration<'a>,
    ) {
        let kind = AstKind::AnnotationInterfaceElementDeclaration(decl);
        self.enter_node(kind);
        decl.bind(self);
        self.visit_modifiers(&decl.modifiers);
        self.visit_type(&decl.ty);
        self.visit_binding_identifier(&decl.name);
        self.visit_array_type_dimensions(&decl.post_paren_dimensions);
        if let Some(default_value) = &decl.default_value {
            self.visit_element_value(default_value);
        }
        self.leave_node(kind);
    }

    fn visit_method_declaration(&mut self, decl: &'a MethodDeclaration<'a>) {
        let kind = AstKind::MethodDeclaration(decl);
        self.enter_node(kind);
        decl.bind(self);
        self.enter_scope(&decl.scope_id);
        self.visit_modifiers(&decl.modifiers);
        if let Some(type_params) = &decl.type_params {
            self.visit_type_parameters(type_params);
        }
        self.visit_method_result(&decl.return_type);
        self.visit_binding_identifier(&decl.name);
        self.visit_formal_parameters(&decl.params);
        self.visit_array_type_dimensions(&decl.post_paren_dimensions);
        self.visit_types(&decl.throws);
        if let Some(body) = &decl.body {
            self.visit_block(body);
        }
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_compact_constructor_declaration(
        &mut self,
        decl: &'a CompactConstructorDeclaration<'a>,
    ) {
        let kind = AstKind::CompactConstructorDeclaration(decl);
        self.enter_node(kind);
        self.enter_scope(&decl.scope_id);
        decl.bind(self);
        self.visit_modifiers(&decl.modifiers);
        self.visit_identifier_name(&decl.name);
        self.visit_block(&decl.body);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_type_parameter(&mut self, param: &'a TypeParameter<'a>) {
        let kind = AstKind::TypeParameter(param);
        self.enter_node(kind);
        param.bind(self);
        self.visit_annotations(&param.annotations);
        self.visit_binding_identifier(&param.name);
        self.visit_types(&param.bounds);
        self.leave_node(kind);
    }

    fn visit_formal_parameter(&mut self, param: &'a FormalParameter<'a>) {
        let kind = AstKind::FormalParameter(param);
        self.enter_node(kind);
        param.bind(self);
        self.visit_modifiers(&param.modifiers);
        if let Some(ty) = &param.ty {
            self.visit_variable_type(ty);
        }
        self.visit_annotations(&param.varargs_annotations);
        self.visit_binding_identifier(&param.name);
        self.visit_array_type_dimensions(&param.dims);
        self.leave_node(kind);
    }

    fn visit_variable_declarator(&mut self, decl: &'a VariableDeclarator<'a>) {
        let kind = AstKind::VariableDeclarator(decl);
        self.enter_node(kind);
        decl.bind(self);
        self.visit_binding_identifier(&decl.name);
        self.visit_array_type_dimensions(&decl.dims);
        if let Some(init) = &decl.init {
            self.visit_variable_initializer(init);
        }
        self.leave_node(kind);
    }

    fn visit_enhanced_for_statement(&mut self, stmt: &'a EnhancedForStatement<'a>) {
        let kind = AstKind::EnhancedForStatement(stmt);
        self.enter_node(kind);
        self.enter_scope(&stmt.scope_id);
        stmt.bind(self);
        self.visit_modifiers(&stmt.modifiers);
        self.visit_variable_type(&stmt.ty);
        self.visit_binding_identifier(&stmt.name);
        self.visit_array_type_dimensions(&stmt.dims);
        self.visit_expression(&stmt.iterable);
        self.visit_statement(&stmt.body);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_resource_declaration(&mut self, decl: &'a ResourceDeclaration<'a>) {
        let kind = AstKind::ResourceDeclaration(decl);
        self.enter_node(kind);
        decl.bind(self);
        self.visit_modifiers(&decl.modifiers);
        self.visit_variable_type(&decl.ty);
        self.visit_binding_identifier(&decl.name);
        self.visit_expression(&decl.init);
        self.leave_node(kind);
    }

    fn visit_catch_clause(&mut self, clause: &'a CatchClause<'a>) {
        let kind = AstKind::CatchClause(clause);
        self.enter_node(kind);
        self.enter_scope(&clause.scope_id);
        clause.bind(self);
        self.visit_modifiers(&clause.modifiers);
        self.visit_types(&clause.types);
        self.visit_binding_identifier(&clause.name);
        self.visit_block(&clause.block);
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_type_pattern(&mut self, pattern: &'a TypePattern<'a>) {
        let kind = AstKind::TypePattern(pattern);
        self.enter_node(kind);
        pattern.bind(self);
        self.visit_modifiers(&pattern.modifiers);
        self.visit_variable_type(&pattern.ty);
        self.visit_binding_identifier(&pattern.name);
        self.leave_node(kind);
    }

    fn visit_identifier_reference(&mut self, ident: &'a IdentifierReference<'a>) {
        let kind = AstKind::IdentifierReference(ident);
        self.enter_node(kind);
        self.reference_identifier(ident);
        self.leave_node(kind);
    }
}

/// A scope that members bind in, because they are in scope only in the body
/// (JLS 6.3).
const fn is_type_scope(kind: AstKind<'_>) -> bool {
    matches!(
        kind,
        AstKind::CompactCompilationUnit(_)
            | AstKind::ClassBody(_)
            | AstKind::InterfaceBody(_)
            | AstKind::EnumBody(_)
            | AstKind::AnnotationInterfaceBody(_)
    )
}

/// The types a constant variable can have (JLS 4.12.4).
fn is_primitive_or_string(ty: &Type<'_>) -> bool {
    if matches!(ty, Type::Primitive(_)) {
        return true;
    }
    ty.as_class_type().is_some_and(|ty| {
        ty.name() == "String"
            && (ty.qualifier().is_none()
                || ty.self_and_qualifiers().map(ClassType::name).eq(["String", "lang", "java"]))
    })
}

const fn compatible(symbol: SymbolKind, reference: ReferenceKind) -> bool {
    match reference {
        ReferenceKind::Type => matches!(
            symbol,
            SymbolKind::Type
                | SymbolKind::TypeParameter
                | SymbolKind::Import(ImportKind::SingleType | ImportKind::SingleStatic)
        ),
        ReferenceKind::Method => {
            matches!(symbol, SymbolKind::Method | SymbolKind::Import(ImportKind::SingleStatic))
        }
        ReferenceKind::Read | ReferenceKind::Write | ReferenceKind::ReadWrite => {
            symbol.is_variable()
                || matches!(
                    symbol,
                    SymbolKind::Type
                        | SymbolKind::TypeParameter
                        | SymbolKind::Import(ImportKind::SingleType | ImportKind::SingleStatic)
                )
        }
    }
}

const fn symbol_priority(symbol: SymbolKind) -> u8 {
    match symbol {
        kind if kind.is_variable() => 0,
        SymbolKind::Method | SymbolKind::Type | SymbolKind::TypeParameter => 1,
        SymbolKind::Import(_) => 2,
        _ => 3,
    }
}
