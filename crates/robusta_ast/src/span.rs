#[cfg(test)]
mod tests {
    use robusta_allocator::Allocator;
    use robusta_span::{GetSpan, Span};

    use super::super::*;
    use crate::builder::AstBuilder;

    #[test]
    fn derived_records_expose_source_spans() {
        let allocator = &Allocator::new();
        let builder = AstBuilder::new(allocator);
        let requires_modifier =
            RequiresModifier::new(Span::new(9, 19), RequiresModifierKind::Transitive, &builder);
        assert_eq!(requires_modifier.span(), Span::new(9, 19));

        let keyword_modifier =
            KeywordModifier::new(Span::new(20, 26), ModifierKind::Public, &builder);
        assert_eq!(keyword_modifier.span(), Span::new(20, 26));
    }

    #[test]
    fn derived_dispatch_enums_delegate_to_payload_spans() {
        let allocator = &Allocator::new();
        let builder = AstBuilder::new(allocator);
        let expression = Expression::new_this_expression(Span::new(30, 34), &builder);
        assert_eq!(expression.span(), Span::new(30, 34));

        let qualifier = TypeName::new_identifier_reference(Span::new(40, 45), "Outer", &builder);
        let target =
            MethodReferenceTarget::new_qualified_super(Span::new(40, 51), qualifier, &builder);
        assert_eq!(target.span(), Span::new(40, 51));
    }
}
