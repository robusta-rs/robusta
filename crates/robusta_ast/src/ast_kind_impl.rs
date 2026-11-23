//! Node-family views over [`AstKind`].

use super::*;

/// Java's five named type declarations (JLS §§7.6, 8, 9).
///
/// [`TypeDeclaration`] is visitor-transparent. This view groups its payload
/// nodes without adding a traversal node.
#[derive(Debug, Clone, Copy)]
pub enum TypeDeclarationKind<'a> {
    Class(&'a ClassDeclaration<'a>),
    Interface(&'a InterfaceDeclaration<'a>),
    Enum(&'a EnumDeclaration<'a>),
    Record(&'a RecordDeclaration<'a>),
    AnnotationInterface(&'a AnnotationInterfaceDeclaration<'a>),
}

impl<'a> AstKind<'a> {
    /// Whether this is a named type declaration.
    #[inline]
    pub const fn is_type_declaration(self) -> bool {
        self.as_type_declaration_kind().is_some()
    }

    /// The type-declaration view, if applicable.
    #[inline]
    pub const fn as_type_declaration_kind(self) -> Option<TypeDeclarationKind<'a>> {
        match self {
            Self::ClassDeclaration(declaration) => Some(TypeDeclarationKind::Class(declaration)),
            Self::InterfaceDeclaration(declaration) => {
                Some(TypeDeclarationKind::Interface(declaration))
            }
            Self::EnumDeclaration(declaration) => Some(TypeDeclarationKind::Enum(declaration)),
            Self::RecordDeclaration(declaration) => Some(TypeDeclarationKind::Record(declaration)),
            Self::AnnotationInterfaceDeclaration(declaration) => {
                Some(TypeDeclarationKind::AnnotationInterface(declaration))
            }
            _ => None,
        }
    }
}

impl<'a> TypeDeclarationKind<'a> {
    /// The declaration's simple name.
    #[inline]
    pub const fn name(self) -> &'a BindingIdentifier<'a> {
        match self {
            Self::Class(declaration) => &declaration.name,
            Self::Interface(declaration) => &declaration.name,
            Self::Enum(declaration) => &declaration.name,
            Self::Record(declaration) => &declaration.name,
            Self::AnnotationInterface(declaration) => &declaration.name,
        }
    }

    /// The declaration modifiers in source order.
    #[inline]
    pub fn modifiers(self) -> &'a [Modifier<'a>] {
        match self {
            Self::Class(declaration) => &declaration.modifiers,
            Self::Interface(declaration) => &declaration.modifiers,
            Self::Enum(declaration) => &declaration.modifiers,
            Self::Record(declaration) => &declaration.modifiers,
            Self::AnnotationInterface(declaration) => &declaration.modifiers,
        }
    }
}

#[cfg(test)]
mod tests {
    use robusta_allocator::Allocator;
    use robusta_span::{GetSpan, Span};

    use super::*;
    use crate::builder::AstBuilder;

    #[test]
    fn ast_type_discriminants_are_dense_and_match_kinds() {
        assert_eq!(AstType::ALL.len(), AstType::COUNT);
        for (index, ty) in AstType::ALL.iter().enumerate() {
            assert_eq!(*ty as usize, index);
        }

        let allocator = &Allocator::new();
        let builder = AstBuilder::new(allocator);
        let empty = EmptyDeclaration::new(Span::new(0, 1), &builder);
        let literal = IntLiteral::new(Span::new(2, 3), "1", &builder);
        assert_eq!(AstKind::EmptyDeclaration(&empty).ty(), AstType::EmptyDeclaration);
        assert_eq!(AstKind::IntLiteral(&literal).ty(), AstType::IntLiteral);
    }

    #[test]
    fn ast_kind_preserves_typed_nodes_and_spans() {
        let allocator = &Allocator::new();
        let builder = AstBuilder::new(allocator);
        let empty = EmptyDeclaration::new(Span::new(4, 5), &builder);
        let literal = IntLiteral::new(Span::new(9, 12), "10L", &builder);

        let empty_kind = AstKind::EmptyDeclaration(&empty);
        let literal_kind = AstKind::IntLiteral(&literal);

        assert_eq!(empty_kind.span(), Span::new(4, 5));
        assert_eq!(literal_kind.span(), Span::new(9, 12));
        let AstKind::IntLiteral(node) = literal_kind else {
            panic!("expected a literal kind");
        };
        assert_eq!(node.raw, "10L");
    }

    #[test]
    fn all_structural_node_shapes_expose_spans_to_ast_kind() {
        let allocator = &Allocator::new();
        let builder = AstBuilder::new(allocator);
        let body = LambdaBody::new_block(Span::new(20, 22), [], &builder);
        let params = FormalParameters::new(Span::new(16, 18), None, [], &builder);
        let lambda = LambdaExpression::new(Span::new(16, 22), params, body, &builder);
        let class_body = ClassBody::new(Span::new(30, 32), [], &builder);

        assert_eq!(AstKind::LambdaExpression(&lambda).span(), Span::new(16, 22));
        assert_eq!(AstKind::ClassBody(&class_body).span(), Span::new(30, 32));
    }
}
