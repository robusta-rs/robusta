//! Declare symbols for Java declarations.

use robusta_ast::{
    AnnotationInterfaceElementDeclaration, AstKind, CatchClause, CompactConstructorDeclaration,
    EnhancedForStatement, EnumConstant, FormalParameter, ImportDeclaration, MethodDeclaration,
    Modifier, ModifierKind, RecordComponent, ResourceDeclaration, TypeDeclarationKind,
    TypeParameter, TypePattern, VariableDeclarator,
};
use robusta_span::{GetSpan, Span};

use crate::{
    SemanticBuilder, SymbolKind,
    pattern_scope::{PatternScope, pattern_scope},
};

pub trait Binder<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>);
}

impl<'a> Binder<'a> for ImportDeclaration<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        if let Some(local) = &self.local {
            builder.declare_symbol(local, SymbolKind::Import(self.kind));
        }
    }
}

impl<'a> Binder<'a> for TypeDeclarationKind<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        if let parent @ (AstKind::Block(_) | AstKind::SwitchCase(_)) = builder.parent_kind() {
            // A local type is in scope from its own declaration to the end
            // of its block or case (JLS 6.3).
            let start = builder.nodes.kind(builder.current_node_id).span().start;
            let window = Span::new(start, parent.span().end);
            builder.declare_symbol_within(self.name(), SymbolKind::Type, window);
        } else if let Some(symbol_id) = builder.declare_symbol(self.name(), SymbolKind::Type)
            && has_modifier(self.modifiers(), ModifierKind::Private)
        {
            builder.scoping.symbols[symbol_id].is_private = true;
        }
    }
}

impl<'a> Binder<'a> for MethodDeclaration<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        builder.declare_method(&self.name, has_modifier(&self.modifiers, ModifierKind::Private));
    }
}

impl<'a> Binder<'a> for AnnotationInterfaceElementDeclaration<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        builder.declare_method(&self.name, false);
    }
}

impl<'a> Binder<'a> for CompactConstructorDeclaration<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        let body_id = builder.nodes.parent_id(builder.current_node_id);
        if let AstKind::RecordDeclaration(record) = builder.nodes.parent_kind(body_id) {
            // The component names declare parameters in this constructor (JLS 8.10.4.2).
            for component in &record.header.components {
                builder.declare_compact_constructor_parameter(&component.name, self.body.span);
            }
        }
    }
}

impl<'a> Binder<'a> for TypeParameter<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        builder.declare_symbol(&self.name, SymbolKind::TypeParameter);
    }
}

impl<'a> Binder<'a> for RecordComponent<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        builder.declare_symbol(&self.name, SymbolKind::RecordComponent);
    }
}

impl<'a> Binder<'a> for EnumConstant<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        builder.declare_symbol(&self.name, SymbolKind::EnumConstant);
    }
}

impl<'a> Binder<'a> for VariableDeclarator<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        match builder.parent_kind() {
            AstKind::FieldDeclaration(field) => {
                let symbol_id = builder.declare_symbol(&self.name, SymbolKind::Field);
                if let Some(symbol_id) = symbol_id
                    && has_modifier(&field.modifiers, ModifierKind::Private)
                {
                    builder.scoping.symbols[symbol_id].is_private = true;
                }
            }
            AstKind::LocalVariableDeclaration(_) => {
                let scope_end = builder.scoping.scope(builder.current_scope_id).span.end;
                let window = Span::new(self.name.span.start, scope_end);
                builder.declare_symbol_within(&self.name, SymbolKind::LocalVariable, window);
            }
            _ => {}
        }
    }
}

impl<'a> Binder<'a> for FormalParameter<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        builder.declare_symbol(&self.name, SymbolKind::Parameter);
    }
}

impl<'a> Binder<'a> for EnhancedForStatement<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        builder.declare_symbol_within(
            &self.name,
            SymbolKind::EnhancedForVariable,
            self.body.span(),
        );
    }
}

impl<'a> Binder<'a> for ResourceDeclaration<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        let end = builder
            .nodes
            .ancestor_kinds(builder.current_node_id)
            .find_map(|ancestor| match ancestor {
                AstKind::TryStatement(statement) => Some(statement.block.span.end),
                _ => None,
            })
            .unwrap_or_else(|| self.span().end);
        let window = Span::new(self.name.span.start, end);
        builder.declare_symbol_within(&self.name, SymbolKind::Resource, window);
    }
}

impl<'a> Binder<'a> for CatchClause<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        builder.declare_symbol_within(&self.name, SymbolKind::CatchParameter, self.block.span);
    }
}

impl<'a> Binder<'a> for TypePattern<'a> {
    fn bind(&self, builder: &mut SemanticBuilder<'a>) {
        let PatternScope { scope_id, parts, unmatched } =
            pattern_scope(&builder.nodes, builder.current_node_id, builder.current_scope_id);
        let symbol_id = builder.declare_symbol_in_parts(
            &self.name,
            SymbolKind::PatternVariable,
            scope_id,
            &parts,
        );
        if let (Some(symbol_id), Some(unmatched)) = (symbol_id, unmatched) {
            builder.pending_patterns.push((symbol_id, unmatched));
        }
    }
}

pub fn has_modifier(modifiers: &[Modifier<'_>], kind: ModifierKind) -> bool {
    modifiers
        .iter()
        .any(|modifier| matches!(modifier, Modifier::Keyword(keyword) if keyword.kind == kind))
}
