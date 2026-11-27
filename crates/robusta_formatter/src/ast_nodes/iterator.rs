use robusta_allocator::Vec;
use robusta_ast::*;
use robusta_span::GetSpan;

use super::{AstNode, AstNodes};

pub struct AstNodeIterator<'parent, 'a, T> {
    inner: std::slice::Iter<'a, T>,
    parent: AstNodes<'parent, 'a>,
    following_span_start: u32,
}

impl<'parent, 'a, T: AstNodeListElement> AstNode<'parent, 'a, Vec<'a, T>> {
    pub fn iter(&self) -> AstNodeIterator<'parent, 'a, T> {
        AstNodeIterator {
            inner: self.inner.iter(),
            parent: self.parent,
            following_span_start: if T::HAS_FOLLOWING_SPAN { self.following_span_start } else { 0 },
        }
    }

    pub fn get(&self, index: usize) -> Option<AstNode<'parent, 'a, T>> {
        Some(AstNode {
            inner: self.inner.get(index)?,
            parent: self.parent,
            following_span_start: self.inner.get(index + 1).map_or_else(
                || if T::HAS_FOLLOWING_SPAN { self.following_span_start } else { 0 },
                |next| next.span().start,
            ),
        })
    }
}

pub trait AstNodeListElement: GetSpan {
    const HAS_FOLLOWING_SPAN: bool;
}

macro_rules! list_elements {
    ($following:literal; $($ty:ty),+ $(,)?) => {
        $(impl AstNodeListElement for $ty {
            const HAS_FOLLOWING_SPAN: bool = $following;
        })+
    };
}

list_elements!(false;
    AnnotationArgument<'_>, AnnotationInterfaceMemberDeclaration<'_>, Annotation<'_>,
    ArrayDimension<'_>, ArrayTypeDimension<'_>, CatchClause<'_>, ClassMember<'_>,
    ComponentPattern<'_>, ElementValue<'_>, Expression<'_>, FormalParameter<'_>,
    InterfaceMember<'_>, Modifier<'_>, ModuleDirective<'_>, Name<'_>,
    RecordComponent<'_>, RequiresModifier, Resource<'_>, Statement<'_>,
    SwitchItem<'_>, Type<'_>, TypeArgument<'_>, TypeDeclaration<'_>, TypeName<'_>,
    TypeParameter<'_>, VariableDeclarator<'_>, VariableInitializer<'_>,
);
// More of the parent follows these lists: the enum's members (JLS 8.9.1), the unit's type
// declarations (JLS 7.3), and a case's guard (JLS 14.11.1).
// Comments before them lead them instead of trailing the last item.
list_elements!(true; EnumConstant<'_>, ImportDeclaration<'_>, Pattern<'_>);

impl<'parent, 'a, T: GetSpan> Iterator for AstNodeIterator<'parent, 'a, T> {
    type Item = AstNode<'parent, 'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.next()?;
        let following_span_start = self
            .inner
            .as_slice()
            .first()
            .map_or(self.following_span_start, |next| next.span().start);
        Some(AstNode { inner, parent: self.parent, following_span_start })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T: GetSpan> ExactSizeIterator for AstNodeIterator<'_, '_, T> {}

impl<'parent, 'a, T: AstNodeListElement> IntoIterator for &AstNode<'parent, 'a, Vec<'a, T>> {
    type Item = AstNode<'parent, 'a, T>;
    type IntoIter = AstNodeIterator<'parent, 'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
