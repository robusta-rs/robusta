use std::{fmt, ops::Deref};

use robusta_allocator::Allocator;
use robusta_span::{GetSpan, Span};

use super::AstNodes;

/// An AST node and its parent on the formatter's stack.
pub struct AstNode<'parent, 'a, T> {
    pub(super) inner: &'a T,
    pub(super) parent: AstNodes<'parent, 'a>,
    /// The next sibling's start, the enclosing boundary, or zero when unbounded.
    pub(super) following_span_start: u32,
}

impl<T> Clone for AstNode<'_, '_, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for AstNode<'_, '_, T> {}

impl<T: fmt::Debug> fmt::Debug for AstNode<'_, '_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AstNode")
            .field("inner", &self.inner)
            .field("parent", &self.parent.debug_name())
            .field("following_span_start", &self.following_span_start)
            .finish()
    }
}

impl<'a, T> Deref for AstNode<'_, 'a, T> {
    type Target = T;
    fn deref(&self) -> &'a T {
        self.inner
    }
}
impl<'a, T> AsRef<T> for AstNode<'_, 'a, T> {
    fn as_ref(&self) -> &'a T {
        self.inner
    }
}
impl<T: GetSpan> GetSpan for AstNode<'_, '_, T> {
    fn span(&self) -> Span {
        self.inner.span()
    }
}
impl<T: GetSpan> GetSpan for &AstNode<'_, '_, T> {
    fn span(&self) -> Span {
        self.inner.span()
    }
}
impl<'parent, 'a, T> AstNode<'parent, 'a, T> {
    /// Returns the AST node, for the lifetime of the AST rather than the borrow of `self` (unlike [`Deref`] and [`AsRef`]).
    pub fn inner(&self) -> &'a T {
        self.inner
    }
    pub fn new(inner: &'a T, parent: AstNodes<'parent, 'a>) -> Self {
        Self { inner, parent, following_span_start: 0 }
    }
    pub fn parent(&self) -> &AstNodes<'parent, 'a> {
        &self.parent
    }
    pub fn following_span_start(&self) -> u32 {
        self.following_span_start
    }

    /// Moves this node into the arena, so a node found in a loop can be the
    /// parent of the next one.
    pub fn in_arena(self, allocator: &'parent Allocator) -> &'parent Self {
        allocator.alloc(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use robusta_allocator::Allocator;

    #[test]
    fn child_accessors_preserve_parents_and_following_siblings() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(
            &allocator,
            "class A { void m() { if (a) f(); else g(); h(); } }",
        );
        let unit = AstNode::new(&parsed.unit, AstNodes::Dummy);
        let ordinary = unit.content().as_ordinary().unwrap();
        let class = ordinary.types().iter().next().unwrap().as_class().unwrap();
        let class_body = class.body();
        let method = class_body.members().iter().next().unwrap().as_method().unwrap();
        let block = method.body().unwrap();
        let mut statements = block.statements().iter();
        let first = statements.next().unwrap();
        let last = statements.next().unwrap();
        assert_eq!(first.following_span_start(), last.span().start);
        assert_eq!(last.following_span_start(), 0);
        assert!(matches!(first.parent(), AstNodes::Block(parent) if parent.span() == block.span()));
        let conditional = first.as_if().unwrap();
        let then_branch = conditional.then_branch();
        let else_branch = conditional.else_branch().unwrap();
        assert_eq!(conditional.condition().following_span_start(), then_branch.span().start);
        assert_eq!(then_branch.following_span_start(), else_branch.span().start);
        assert_eq!(else_branch.following_span_start(), 0);
        assert!(
            matches!(then_branch.parent(), AstNodes::IfStatement(parent) if parent.span() == conditional.span())
        );
        assert_eq!(statements.len(), 0);
    }

    #[test]
    fn constructor_children_follow_the_declared_source_order() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, "class A { A() { outer.<T>super(); } }");
        let unit = AstNode::new(&parsed.unit, AstNodes::Dummy);
        let ordinary = unit.content().as_ordinary().unwrap();
        let class = ordinary.types().iter().next().unwrap().as_class().unwrap();
        let class_body = class.body();
        let constructor = class_body.members().iter().next().unwrap().as_constructor().unwrap();
        let body = constructor.body();
        let invocation =
            body.statements().iter().next().unwrap().as_constructor_invocation().unwrap();
        let qualifier = invocation.qualifier().unwrap();
        let type_args = invocation.type_args().unwrap();
        assert_eq!(qualifier.following_span_start(), type_args.span().start);
        assert!(matches!(qualifier.parent(), AstNodes::ConstructorInvocation(_)));
    }

    #[test]
    fn the_last_parameter_has_no_following_sibling() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, "class A { void m(A this) throws E {} }");
        let unit = AstNode::new(&parsed.unit, AstNodes::Dummy);
        let ordinary = unit.content().as_ordinary().unwrap();
        let class = ordinary.types().iter().next().unwrap().as_class().unwrap();
        let class_body = class.body();
        let method = class_body.members().iter().next().unwrap().as_method().unwrap();
        let params = method.params();
        let throws = method.throws();
        assert_eq!(params.following_span_start(), throws.iter().next().unwrap().span().start);
        let receiver = params.receiver().unwrap();
        assert!(matches!(receiver.parent(), AstNodes::FormalParameters(_)));
        assert_eq!(receiver.following_span_start(), 0);
    }

    #[test]
    fn the_last_record_component_has_no_following_sibling() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, "record R(int x) implements I {}");
        let unit = AstNode::new(&parsed.unit, AstNodes::Dummy);
        let ordinary = unit.content().as_ordinary().unwrap();
        let record = ordinary.types().iter().next().unwrap().as_record().unwrap();
        let header = record.header();
        let interfaces = record.interfaces();
        assert_eq!(header.following_span_start(), interfaces.iter().next().unwrap().span().start);
        let components = header.components();
        assert_eq!(components.following_span_start(), 0);
        let component = components.iter().next().unwrap();
        assert!(matches!(component.parent(), AstNodes::RecordHeader(_)));
        assert_eq!(component.following_span_start(), 0);
    }

    #[test]
    fn resolved_enum_kinds_preserve_the_parent_and_sibling_bound() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, "class A { void m() { return; run(); } }");
        let unit = AstNode::new(&parsed.unit, AstNodes::Dummy);
        let ordinary = unit.content().as_ordinary().unwrap();
        let class = ordinary.types().iter().next().unwrap().as_class().unwrap();
        let class_body = class.body();
        let method = class_body.members().iter().next().unwrap().as_method().unwrap();
        let block = method.body().unwrap();
        let statement = block.statements().iter().next().unwrap();
        let kind = statement.as_kind();
        let super::super::generated::ast_nodes::AstNodeKind::ReturnStatement(return_node) = kind
        else {
            panic!("return statement kind");
        };
        assert_eq!(return_node.following_span_start(), statement.following_span_start());
        assert!(
            matches!(return_node.parent(), AstNodes::Block(parent) if parent.span() == block.span())
        );
        assert!(matches!(kind.as_ast_nodes(), AstNodes::ReturnStatement(_)));
        assert_eq!(kind.span(), statement.span());
    }

    #[test]
    fn default_labels_are_nodes() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(
            &allocator,
            "class A { void m() { switch (x) { default -> f(); } switch (x) { case 1: default: g(); } } }",
        );
        let unit = AstNode::new(&parsed.unit, AstNodes::Dummy);
        let ordinary = unit.content().as_ordinary().unwrap();
        let class = ordinary.types().iter().next().unwrap().as_class().unwrap();
        let class_body = class.body();
        let method = class_body.members().iter().next().unwrap().as_method().unwrap();
        let block = method.body().unwrap();
        let mut statements = block.statements().iter();

        let switch = statements.next().unwrap();
        let switch = switch.as_switch().unwrap();
        let cases = switch.cases();
        let case = cases.iter().next().unwrap();
        let rule = case.as_rule().unwrap();
        assert!(matches!(rule.parent(), AstNodes::SwitchStatement(_)));
        let label = rule.label().as_default().unwrap();
        assert!(matches!(label.parent(), AstNodes::SwitchRule(_)));

        let switch = statements.next().unwrap();
        let switch = switch.as_switch().unwrap();
        let cases = switch.cases();
        let case = cases.iter().next().unwrap();
        let group = case.as_group().unwrap();
        assert!(matches!(group.parent(), AstNodes::SwitchStatement(_)));
        let labels = group.labels();
        let mut labels = labels.iter();
        let case = labels.next().unwrap();
        let label = labels.next().unwrap().as_default().unwrap();
        assert!(matches!(label.parent(), AstNodes::SwitchBlockStatementGroup(_)));
        assert_eq!(case.following_span_start(), label.span().start);
    }

    #[test]
    fn annotation_arguments_are_one_list() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, "@A(x = 1, y = 2) @B(3) @C() @D class E {}");
        let unit = AstNode::new(&parsed.unit, AstNodes::Dummy);
        let ordinary = unit.content().as_ordinary().unwrap();
        let class = ordinary.types().iter().next().unwrap().as_class().unwrap();
        let modifiers = class.modifiers();
        let mut modifiers = modifiers.iter();

        let annotation = modifiers.next().unwrap();
        let annotation = annotation.as_annotation().unwrap();
        let args = annotation.args().unwrap();
        let mut pairs = args.iter();
        let first = pairs.next().unwrap();
        let second = pairs.next().unwrap();
        assert!(matches!(first.as_pair().unwrap().parent(), AstNodes::Annotation(_)));
        assert_eq!(first.following_span_start(), second.span().start);
        assert_eq!(second.following_span_start(), 0);

        let annotation = modifiers.next().unwrap();
        let annotation = annotation.as_annotation().unwrap();
        let args = annotation.args().unwrap();
        let value = args.iter().next().unwrap();
        assert!(matches!(value.as_element_value().unwrap().parent(), AstNodes::Annotation(_)));
        assert_eq!(annotation.name().following_span_start(), value.span().start);

        let annotation = modifiers.next().unwrap();
        assert!(annotation.as_annotation().unwrap().args().unwrap().is_empty());
        let annotation = modifiers.next().unwrap();
        assert!(annotation.as_annotation().unwrap().args().is_none());
    }

    #[test]
    fn for_expression_lists_are_nodes() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(
            &allocator,
            "class A { void m() { for (i = 0, j = 1; i < j; i++, j--) g(); for (;; i++) g(); } }",
        );
        let unit = AstNode::new(&parsed.unit, AstNodes::Dummy);
        let ordinary = unit.content().as_ordinary().unwrap();
        let class = ordinary.types().iter().next().unwrap().as_class().unwrap();
        let class_body = class.body();
        let method = class_body.members().iter().next().unwrap().as_method().unwrap();
        let block = method.body().unwrap();
        let mut statements = block.statements().iter();

        let statement = statements.next().unwrap();
        let statement = statement.as_for().unwrap();
        let init = statement.init().unwrap();
        let init = init.as_expressions().unwrap();
        assert!(matches!(init.parent(), AstNodes::ForStatement(_)));
        assert_eq!(init.following_span_start(), statement.condition().unwrap().span().start);
        let expressions = init.expressions();
        let first = expressions.iter().next().unwrap();
        assert!(matches!(first.parent(), AstNodes::StatementExpressionList(_)));

        let update = statement.update().unwrap();
        assert!(matches!(update.parent(), AstNodes::ForStatement(_)));
        assert_eq!(update.following_span_start(), statement.body().span().start);
        assert_eq!(statement.condition().unwrap().following_span_start(), update.span().start);
        let expressions = update.expressions();
        assert!(matches!(
            expressions.iter().next().unwrap().parent(),
            AstNodes::StatementExpressionList(_)
        ));

        let statement = statements.next().unwrap();
        let statement = statement.as_for().unwrap();
        assert!(statement.init().is_none());
        assert_eq!(statement.update().unwrap().expressions.len(), 1);
    }

    #[test]
    fn qualified_super_method_reference_targets_are_nodes() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, "class A { Object x = Outer.super::m; }");
        let unit = AstNode::new(&parsed.unit, AstNodes::Dummy);
        let ordinary = unit.content().as_ordinary().unwrap();
        let class = ordinary.types().iter().next().unwrap().as_class().unwrap();
        let class_body = class.body();
        let field = class_body.members().iter().next().unwrap().as_field().unwrap();
        let declarator = field.declarators().iter().next().unwrap();
        let expression = declarator.init().unwrap().as_expression().unwrap();
        let reference = expression.as_method_reference().unwrap();
        let target = reference.target().as_expression().unwrap().as_qualified_super().unwrap();
        assert!(
            matches!(target.parent(), AstNodes::MethodReference(parent) if parent.span() == reference.span())
        );
        assert_eq!(target.following_span_start(), reference.method.as_ref().unwrap().span.start);
        let qualifier = target.qualifier();
        assert_eq!(qualifier.last(), "Outer");
        assert!(matches!(qualifier.parent(), AstNodes::QualifiedSuper(_)));
    }
}
