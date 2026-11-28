// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/formatter/format.rs`.

use crate::{
    ast_nodes::AstNode,
    formatter::{JavaFormatContext, JavaFormatter, JavaFormatterExt as _},
    print::FormatWrite,
};
use robusta_ast::*;
use robusta_formatter_core::Format;
use robusta_span::GetSpan;
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CompilationUnit<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CompilationUnitContent<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            CompilationUnitContent::Ordinary(field_0) => {
                AstNode::<OrdinaryCompilationUnit<'a>> {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            CompilationUnitContent::Compact(field_0) => {
                AstNode::<CompactCompilationUnit<'a>> {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            CompilationUnitContent::Modular(field_0) => {
                AstNode::<ModularCompilationUnit<'a>> {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, OrdinaryCompilationUnit<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ModularCompilationUnit<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, PackageDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ImportDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CompactCompilationUnit<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ModuleDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ModuleDirective<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            ModuleDirective::Requires(field_0) => {
                AstNode::<RequiresDirective<'a>> {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ModuleDirective::Package(field_0) => {
                AstNode::<PackageDirective<'a>> {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ModuleDirective::Uses(field_0) => {
                AstNode::<UsesDirective<'a>> {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ModuleDirective::Provides(field_0) => {
                AstNode::<ProvidesDirective<'a>> {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, RequiresDirective<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, PackageDirective<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, UsesDirective<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ProvidesDirective<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, TypeDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            TypeDeclaration::Empty(field_0) => {
                AstNode::<EmptyStatement> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeDeclaration::AnnotationInterface(field_0) => {
                AstNode::<AnnotationInterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeDeclaration::Class(field_0) => {
                AstNode::<ClassDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeDeclaration::Interface(field_0) => {
                AstNode::<InterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeDeclaration::Enum(field_0) => {
                AstNode::<EnumDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeDeclaration::Record(field_0) => {
                AstNode::<RecordDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, LocalTypeDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            LocalTypeDeclaration::Class(field_0) => {
                AstNode::<ClassDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LocalTypeDeclaration::Interface(field_0) => {
                AstNode::<InterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LocalTypeDeclaration::Enum(field_0) => {
                AstNode::<EnumDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LocalTypeDeclaration::Record(field_0) => {
                AstNode::<RecordDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ClassDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ClassBody<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, InterfaceDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, InterfaceBody<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, EnumDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, EnumBody<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, EnumConstant<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, RecordDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, RecordHeader<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, RecordComponent<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, AnnotationInterfaceDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, AnnotationInterfaceBody<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, AnnotationInterfaceMember<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            AnnotationInterfaceMember::Element(field_0) => {
                AstNode::<AnnotationInterfaceElementDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationInterfaceMember::Field(field_0) => {
                AstNode::<FieldDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationInterfaceMember::Empty(field_0) => {
                AstNode::<EmptyStatement> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationInterfaceMember::AnnotationInterface(field_0) => {
                AstNode::<AnnotationInterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationInterfaceMember::Class(field_0) => {
                AstNode::<ClassDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationInterfaceMember::Interface(field_0) => {
                AstNode::<InterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationInterfaceMember::Enum(field_0) => {
                AstNode::<EnumDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationInterfaceMember::Record(field_0) => {
                AstNode::<RecordDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>>
    for AstNode<'_, 'a, AnnotationInterfaceElementDeclaration<'a>>
{
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ClassMember<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            ClassMember::Field(field_0) => {
                AstNode::<FieldDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::Method(field_0) => {
                AstNode::<MethodDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::Constructor(field_0) => {
                AstNode::<ConstructorDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::CompactConstructor(field_0) => {
                AstNode::<CompactConstructorDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::Initializer(field_0) => {
                AstNode::<InitializerBlock<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::Empty(field_0) => {
                AstNode::<EmptyStatement> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::AnnotationInterface(field_0) => {
                AstNode::<AnnotationInterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::Class(field_0) => {
                AstNode::<ClassDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::Interface(field_0) => {
                AstNode::<InterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::Enum(field_0) => {
                AstNode::<EnumDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassMember::Record(field_0) => {
                AstNode::<RecordDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, InterfaceMember<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            InterfaceMember::Constant(field_0) => {
                AstNode::<FieldDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InterfaceMember::Method(field_0) => {
                AstNode::<MethodDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InterfaceMember::Empty(field_0) => {
                AstNode::<EmptyStatement> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InterfaceMember::AnnotationInterface(field_0) => {
                AstNode::<AnnotationInterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InterfaceMember::Class(field_0) => {
                AstNode::<ClassDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InterfaceMember::Interface(field_0) => {
                AstNode::<InterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InterfaceMember::Enum(field_0) => {
                AstNode::<EnumDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InterfaceMember::Record(field_0) => {
                AstNode::<RecordDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, InitializerBlock<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, FieldDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, MethodDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ConstructorDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CompactConstructorDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ReceiverParameter<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, FormalParameters<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, FormalParameter<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Block<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Statement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            Statement::Block(field_0) => {
                AstNode::<Block<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Empty(field_0) => {
                AstNode::<EmptyStatement> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::ConstructorInvocation(field_0) => {
                AstNode::<ConstructorInvocation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::LocalVariable(field_0) => {
                AstNode::<LocalVariableDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Expression(field_0) => {
                AstNode::<ExpressionStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::If(field_0) => {
                AstNode::<IfStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Switch(field_0) => {
                AstNode::<SwitchStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::While(field_0) => {
                AstNode::<WhileStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Do(field_0) => {
                AstNode::<DoStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::For(field_0) => {
                AstNode::<ForStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::EnhancedFor(field_0) => {
                AstNode::<EnhancedForStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Break(field_0) => {
                AstNode::<BreakStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Continue(field_0) => {
                AstNode::<ContinueStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Throw(field_0) => {
                AstNode::<ThrowStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Assert(field_0) => {
                AstNode::<AssertStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Try(field_0) => {
                AstNode::<TryStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Synchronized(field_0) => {
                AstNode::<SynchronizedStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Return(field_0) => {
                AstNode::<ReturnStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Yield(field_0) => {
                AstNode::<YieldStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Labeled(field_0) => {
                AstNode::<LabeledStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Class(field_0) => {
                AstNode::<ClassDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Interface(field_0) => {
                AstNode::<InterfaceDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Enum(field_0) => {
                AstNode::<EnumDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Statement::Record(field_0) => {
                AstNode::<RecordDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, EmptyStatement> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, IfStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, SwitchStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, SwitchItem<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            SwitchItem::Rule(field_0) => {
                AstNode::<SwitchRule<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchItem::Case(field_0) => {
                AstNode::<SwitchCase<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, SwitchRule<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, SwitchRuleBody<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            SwitchRuleBody::Block(field_0) => {
                AstNode::<Block<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Throw(field_0) => {
                AstNode::<ThrowStatement<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::IntLiteral(field_0) => {
                AstNode::<IntLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::FloatLiteral(field_0) => {
                AstNode::<FloatLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::CharLiteral(field_0) => {
                AstNode::<CharLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::StringLiteral(field_0) => {
                AstNode::<StringLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::BooleanLiteral(field_0) => {
                AstNode::<BooleanLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::NullLiteral(field_0) => {
                AstNode::<NullLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::This(field_0) => {
                AstNode::<ThisExpression> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Super(field_0) => {
                AstNode::<Super> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::QualifiedThis(field_0) => {
                AstNode::<QualifiedThis<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::QualifiedSuper(field_0) => {
                AstNode::<QualifiedSuper<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::FieldAccess(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::MethodCall(field_0) => {
                AstNode::<MethodCall<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::ClassLiteral(field_0) => {
                AstNode::<ClassLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::ClassInstanceCreation(field_0) => {
                AstNode::<ClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::QualifiedClassInstanceCreation(field_0) => {
                AstNode::<QualifiedClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::ArrayCreation(field_0) => {
                AstNode::<ArrayCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::ArrayAccess(field_0) => {
                AstNode::<ArrayAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Update(field_0) => {
                AstNode::<UpdateExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Unary(field_0) => {
                AstNode::<UnaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Cast(field_0) => {
                AstNode::<CastExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Instanceof(field_0) => {
                AstNode::<InstanceofExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Binary(field_0) => {
                AstNode::<BinaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Assignment(field_0) => {
                AstNode::<AssignmentExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Conditional(field_0) => {
                AstNode::<ConditionalExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Switch(field_0) => {
                AstNode::<SwitchExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Parenthesized(field_0) => {
                AstNode::<ParenthesizedExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::Lambda(field_0) => {
                AstNode::<LambdaExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchRuleBody::MethodReference(field_0) => {
                AstNode::<MethodReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, SwitchCase<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, SwitchLabel<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            SwitchLabel::Case(field_0) => {
                AstNode::<CaseLabel<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            SwitchLabel::Pattern(field_0) => {
                AstNode::<CasePatternLabel<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CaseLabel<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CasePatternLabel<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, WhileStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, DoStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ForStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ForInit<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            ForInit::Declaration(field_0) => {
                AstNode::<LocalVariableDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ForInit::Expressions(field_0) => {
                AstNode::<StatementExpressionList<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, StatementExpressionList<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, EnhancedForStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, BreakStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ContinueStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ThrowStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, AssertStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, TryStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Resource<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            Resource::Declaration(field_0) => {
                AstNode::<LocalVariableDeclaration<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Resource::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Resource::Field(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ResourceAccess<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            ResourceAccess::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ResourceAccess::Field(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CatchClause<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CatchParameter<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.write(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CatchType<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            CatchType::Union(field_0) => {
                AstNode::<UnionType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            CatchType::Simple(field_0) => {
                AstNode::<SimpleClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            CatchType::Qualified(field_0) => {
                AstNode::<QualifiedClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, UnionType<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, SynchronizedStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, LabeledStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ConstructorInvocation<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ExpressionStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, LocalVariableDeclaration<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ReturnStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, YieldStatement<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, VariableDeclarator<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, VariableInitializer<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            VariableInitializer::Array(field_0) => {
                AstNode::<ArrayInitializer<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::IntLiteral(field_0) => {
                AstNode::<IntLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::FloatLiteral(field_0) => {
                AstNode::<FloatLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::CharLiteral(field_0) => {
                AstNode::<CharLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::StringLiteral(field_0) => {
                AstNode::<StringLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::BooleanLiteral(field_0) => {
                AstNode::<BooleanLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::NullLiteral(field_0) => {
                AstNode::<NullLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::This(field_0) => {
                AstNode::<ThisExpression> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Super(field_0) => {
                AstNode::<Super> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::QualifiedThis(field_0) => {
                AstNode::<QualifiedThis<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::QualifiedSuper(field_0) => {
                AstNode::<QualifiedSuper<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::FieldAccess(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::MethodCall(field_0) => {
                AstNode::<MethodCall<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::ClassLiteral(field_0) => {
                AstNode::<ClassLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::ClassInstanceCreation(field_0) => {
                AstNode::<ClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::QualifiedClassInstanceCreation(field_0) => {
                AstNode::<QualifiedClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::ArrayCreation(field_0) => {
                AstNode::<ArrayCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::ArrayAccess(field_0) => {
                AstNode::<ArrayAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Update(field_0) => {
                AstNode::<UpdateExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Unary(field_0) => {
                AstNode::<UnaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Cast(field_0) => {
                AstNode::<CastExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Instanceof(field_0) => {
                AstNode::<InstanceofExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Binary(field_0) => {
                AstNode::<BinaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Assignment(field_0) => {
                AstNode::<AssignmentExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Conditional(field_0) => {
                AstNode::<ConditionalExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Switch(field_0) => {
                AstNode::<SwitchExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Parenthesized(field_0) => {
                AstNode::<ParenthesizedExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::Lambda(field_0) => {
                AstNode::<LambdaExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableInitializer::MethodReference(field_0) => {
                AstNode::<MethodReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ArrayInitializer<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Expression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            Expression::IntLiteral(field_0) => {
                AstNode::<IntLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::FloatLiteral(field_0) => {
                AstNode::<FloatLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::CharLiteral(field_0) => {
                AstNode::<CharLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::StringLiteral(field_0) => {
                AstNode::<StringLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::BooleanLiteral(field_0) => {
                AstNode::<BooleanLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::NullLiteral(field_0) => {
                AstNode::<NullLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::This(field_0) => {
                AstNode::<ThisExpression> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Super(field_0) => {
                AstNode::<Super> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::QualifiedThis(field_0) => {
                AstNode::<QualifiedThis<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::QualifiedSuper(field_0) => {
                AstNode::<QualifiedSuper<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::FieldAccess(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::MethodCall(field_0) => {
                AstNode::<MethodCall<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::ClassLiteral(field_0) => {
                AstNode::<ClassLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::ClassInstanceCreation(field_0) => {
                AstNode::<ClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::QualifiedClassInstanceCreation(field_0) => {
                AstNode::<QualifiedClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::ArrayCreation(field_0) => {
                AstNode::<ArrayCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::ArrayAccess(field_0) => {
                AstNode::<ArrayAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Update(field_0) => {
                AstNode::<UpdateExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Unary(field_0) => {
                AstNode::<UnaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Cast(field_0) => {
                AstNode::<CastExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Instanceof(field_0) => {
                AstNode::<InstanceofExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Binary(field_0) => {
                AstNode::<BinaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Assignment(field_0) => {
                AstNode::<AssignmentExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Conditional(field_0) => {
                AstNode::<ConditionalExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Switch(field_0) => {
                AstNode::<SwitchExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Parenthesized(field_0) => {
                AstNode::<ParenthesizedExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::Lambda(field_0) => {
                AstNode::<LambdaExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Expression::MethodReference(field_0) => {
                AstNode::<MethodReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, FieldAccess<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, MethodCall<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Callee<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            Callee::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Callee::Member(field_0) => {
                AstNode::<MethodMember<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, MethodMember<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ClassLiteral<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ThisExpression> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Super> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, QualifiedThis<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, QualifiedSuper<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ClassInstanceCreation<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, QualifiedClassInstanceCreation<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ArrayCreation<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ArrayDimension<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ArrayAccess<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, UpdateExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, UnaryExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CastExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, InstanceofExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, InstanceofRhs<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            InstanceofRhs::Primitive(field_0) => {
                AstNode::<PrimitiveType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InstanceofRhs::Array(field_0) => {
                AstNode::<ArrayType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InstanceofRhs::Simple(field_0) => {
                AstNode::<SimpleClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InstanceofRhs::Qualified(field_0) => {
                AstNode::<QualifiedClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InstanceofRhs::TypePattern(field_0) => {
                AstNode::<TypePattern<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            InstanceofRhs::RecordPattern(field_0) => {
                AstNode::<RecordPattern<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Pattern<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            Pattern::TypePattern(field_0) => {
                AstNode::<TypePattern<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Pattern::RecordPattern(field_0) => {
                AstNode::<RecordPattern<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, TypePattern<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, RecordPattern<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ComponentPattern<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            ComponentPattern::MatchAll(field_0) => {
                AstNode::<MatchAllPattern> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ComponentPattern::TypePattern(field_0) => {
                AstNode::<TypePattern<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ComponentPattern::RecordPattern(field_0) => {
                AstNode::<RecordPattern<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, MatchAllPattern> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, BinaryExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, AssignmentExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, AssignmentTarget<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            AssignmentTarget::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AssignmentTarget::FieldAccess(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AssignmentTarget::ArrayAccess(field_0) => {
                AstNode::<ArrayAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AssignmentTarget::Parenthesized(field_0) => {
                AstNode::<ParenthesizedExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ConditionalExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, SwitchExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ParenthesizedExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, LambdaExpression<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, LambdaBody<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            LambdaBody::Block(field_0) => {
                AstNode::<Block<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::IntLiteral(field_0) => {
                AstNode::<IntLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::FloatLiteral(field_0) => {
                AstNode::<FloatLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::CharLiteral(field_0) => {
                AstNode::<CharLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::StringLiteral(field_0) => {
                AstNode::<StringLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::BooleanLiteral(field_0) => {
                AstNode::<BooleanLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::NullLiteral(field_0) => {
                AstNode::<NullLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::This(field_0) => {
                AstNode::<ThisExpression> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Super(field_0) => {
                AstNode::<Super> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::QualifiedThis(field_0) => {
                AstNode::<QualifiedThis<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::QualifiedSuper(field_0) => {
                AstNode::<QualifiedSuper<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::FieldAccess(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::MethodCall(field_0) => {
                AstNode::<MethodCall<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::ClassLiteral(field_0) => {
                AstNode::<ClassLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::ClassInstanceCreation(field_0) => {
                AstNode::<ClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::QualifiedClassInstanceCreation(field_0) => {
                AstNode::<QualifiedClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::ArrayCreation(field_0) => {
                AstNode::<ArrayCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::ArrayAccess(field_0) => {
                AstNode::<ArrayAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Update(field_0) => {
                AstNode::<UpdateExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Unary(field_0) => {
                AstNode::<UnaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Cast(field_0) => {
                AstNode::<CastExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Instanceof(field_0) => {
                AstNode::<InstanceofExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Binary(field_0) => {
                AstNode::<BinaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Assignment(field_0) => {
                AstNode::<AssignmentExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Conditional(field_0) => {
                AstNode::<ConditionalExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Switch(field_0) => {
                AstNode::<SwitchExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Parenthesized(field_0) => {
                AstNode::<ParenthesizedExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::Lambda(field_0) => {
                AstNode::<LambdaExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            LambdaBody::MethodReference(field_0) => {
                AstNode::<MethodReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, MethodReference<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, MethodReferenceTarget<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            MethodReferenceTarget::IntLiteral(field_0) => {
                AstNode::<IntLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::FloatLiteral(field_0) => {
                AstNode::<FloatLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::CharLiteral(field_0) => {
                AstNode::<CharLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::StringLiteral(field_0) => {
                AstNode::<StringLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::BooleanLiteral(field_0) => {
                AstNode::<BooleanLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::NullLiteral(field_0) => {
                AstNode::<NullLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::This(field_0) => {
                AstNode::<ThisExpression> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Super(field_0) => {
                AstNode::<Super> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::QualifiedThis(field_0) => {
                AstNode::<QualifiedThis<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::QualifiedSuper(field_0) => {
                AstNode::<QualifiedSuper<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::FieldAccess(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::MethodCall(field_0) => {
                AstNode::<MethodCall<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::ClassLiteral(field_0) => {
                AstNode::<ClassLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::ClassInstanceCreation(field_0) => {
                AstNode::<ClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::QualifiedClassInstanceCreation(field_0) => {
                AstNode::<QualifiedClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::ArrayCreation(field_0) => {
                AstNode::<ArrayCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::ArrayAccess(field_0) => {
                AstNode::<ArrayAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Update(field_0) => {
                AstNode::<UpdateExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Unary(field_0) => {
                AstNode::<UnaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Cast(field_0) => {
                AstNode::<CastExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Instanceof(field_0) => {
                AstNode::<InstanceofExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Binary(field_0) => {
                AstNode::<BinaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Assignment(field_0) => {
                AstNode::<AssignmentExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Conditional(field_0) => {
                AstNode::<ConditionalExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Switch(field_0) => {
                AstNode::<SwitchExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Parenthesized(field_0) => {
                AstNode::<ParenthesizedExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Lambda(field_0) => {
                AstNode::<LambdaExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::MethodReference(field_0) => {
                AstNode::<MethodReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Primitive(field_0) => {
                AstNode::<PrimitiveType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Array(field_0) => {
                AstNode::<ArrayType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Simple(field_0) => {
                AstNode::<SimpleClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodReferenceTarget::Qualified(field_0) => {
                AstNode::<QualifiedClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, TypeParameters<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, TypeParameter<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, VariableType<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            VariableType::Var(field_0) => {
                AstNode::<VarType> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableType::Primitive(field_0) => {
                AstNode::<PrimitiveType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableType::Array(field_0) => {
                AstNode::<ArrayType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableType::Simple(field_0) => {
                AstNode::<SimpleClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            VariableType::Qualified(field_0) => {
                AstNode::<QualifiedClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, VarType> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Type<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            Type::Primitive(field_0) => {
                AstNode::<PrimitiveType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Type::Array(field_0) => {
                AstNode::<ArrayType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Type::Simple(field_0) => {
                AstNode::<SimpleClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Type::Qualified(field_0) => {
                AstNode::<QualifiedClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, MethodResult<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            MethodResult::Void(field_0) => {
                AstNode::<VoidType> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodResult::Primitive(field_0) => {
                AstNode::<PrimitiveType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodResult::Array(field_0) => {
                AstNode::<ArrayType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodResult::Simple(field_0) => {
                AstNode::<SimpleClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            MethodResult::Qualified(field_0) => {
                AstNode::<QualifiedClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, VoidType> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, PrimitiveType<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ClassType<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            ClassType::Simple(field_0) => {
                AstNode::<SimpleClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ClassType::Qualified(field_0) => {
                AstNode::<QualifiedClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, SimpleClassType<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, QualifiedClassType<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ArrayType<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ArrayTypeDimension<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, TypeArguments<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, TypeArgument<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            TypeArgument::Wildcard(field_0) => {
                AstNode::<WildcardTypeArgument<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeArgument::Primitive(field_0) => {
                AstNode::<PrimitiveType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeArgument::Array(field_0) => {
                AstNode::<ArrayType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeArgument::Simple(field_0) => {
                AstNode::<SimpleClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeArgument::Qualified(field_0) => {
                AstNode::<QualifiedClassType<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, WildcardTypeArgument<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, WildcardBound<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, IntLiteral<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, FloatLiteral<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, CharLiteral<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, StringLiteral<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, BooleanLiteral> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, NullLiteral> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, IdentifierName<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, IdentifierReference<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, BindingIdentifier<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, LabelIdentifier<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Name<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            Name::Identifier(field_0) => {
                AstNode::<IdentifierName<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Name::Qualified(field_0) => {
                AstNode::<QualifiedName<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, QualifiedName<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, TypeName<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            TypeName::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            TypeName::Qualified(field_0) => {
                AstNode::<QualifiedTypeName<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, QualifiedTypeName<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Modifier<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            Modifier::Keyword(field_0) => {
                AstNode::<KeywordModifier> {
                    inner: field_0,
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            Modifier::Annotation(field_0) => {
                AstNode::<Annotation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, KeywordModifier> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Annotation<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, AnnotationArgument<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            AnnotationArgument::Pair(field_0) => {
                AstNode::<ElementValuePair<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Annotation(field_0) => {
                AstNode::<Annotation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Array(field_0) => {
                AstNode::<ElementValueArrayInitializer<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::IntLiteral(field_0) => {
                AstNode::<IntLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::FloatLiteral(field_0) => {
                AstNode::<FloatLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::CharLiteral(field_0) => {
                AstNode::<CharLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::StringLiteral(field_0) => {
                AstNode::<StringLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::BooleanLiteral(field_0) => {
                AstNode::<BooleanLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::NullLiteral(field_0) => {
                AstNode::<NullLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::This(field_0) => {
                AstNode::<ThisExpression> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Super(field_0) => {
                AstNode::<Super> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::QualifiedThis(field_0) => {
                AstNode::<QualifiedThis<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::QualifiedSuper(field_0) => {
                AstNode::<QualifiedSuper<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::FieldAccess(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::MethodCall(field_0) => {
                AstNode::<MethodCall<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::ClassLiteral(field_0) => {
                AstNode::<ClassLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::ClassInstanceCreation(field_0) => {
                AstNode::<ClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::QualifiedClassInstanceCreation(field_0) => {
                AstNode::<QualifiedClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::ArrayCreation(field_0) => {
                AstNode::<ArrayCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::ArrayAccess(field_0) => {
                AstNode::<ArrayAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Update(field_0) => {
                AstNode::<UpdateExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Unary(field_0) => {
                AstNode::<UnaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Cast(field_0) => {
                AstNode::<CastExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Instanceof(field_0) => {
                AstNode::<InstanceofExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Binary(field_0) => {
                AstNode::<BinaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Assignment(field_0) => {
                AstNode::<AssignmentExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Conditional(field_0) => {
                AstNode::<ConditionalExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Switch(field_0) => {
                AstNode::<SwitchExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Parenthesized(field_0) => {
                AstNode::<ParenthesizedExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::Lambda(field_0) => {
                AstNode::<LambdaExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            AnnotationArgument::MethodReference(field_0) => {
                AstNode::<MethodReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ElementValuePair<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ElementValue<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self.inner {
            ElementValue::Annotation(field_0) => {
                AstNode::<Annotation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Array(field_0) => {
                AstNode::<ElementValueArrayInitializer<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::IntLiteral(field_0) => {
                AstNode::<IntLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::FloatLiteral(field_0) => {
                AstNode::<FloatLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::CharLiteral(field_0) => {
                AstNode::<CharLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::StringLiteral(field_0) => {
                AstNode::<StringLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::BooleanLiteral(field_0) => {
                AstNode::<BooleanLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::NullLiteral(field_0) => {
                AstNode::<NullLiteral> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::This(field_0) => {
                AstNode::<ThisExpression> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Super(field_0) => {
                AstNode::<Super> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::QualifiedThis(field_0) => {
                AstNode::<QualifiedThis<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::QualifiedSuper(field_0) => {
                AstNode::<QualifiedSuper<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Identifier(field_0) => {
                AstNode::<IdentifierReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::FieldAccess(field_0) => {
                AstNode::<FieldAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::MethodCall(field_0) => {
                AstNode::<MethodCall<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::ClassLiteral(field_0) => {
                AstNode::<ClassLiteral<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::ClassInstanceCreation(field_0) => {
                AstNode::<ClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::QualifiedClassInstanceCreation(field_0) => {
                AstNode::<QualifiedClassInstanceCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::ArrayCreation(field_0) => {
                AstNode::<ArrayCreation<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::ArrayAccess(field_0) => {
                AstNode::<ArrayAccess<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Update(field_0) => {
                AstNode::<UpdateExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Unary(field_0) => {
                AstNode::<UnaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Cast(field_0) => {
                AstNode::<CastExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Instanceof(field_0) => {
                AstNode::<InstanceofExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Binary(field_0) => {
                AstNode::<BinaryExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Assignment(field_0) => {
                AstNode::<AssignmentExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Conditional(field_0) => {
                AstNode::<ConditionalExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Switch(field_0) => {
                AstNode::<SwitchExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Parenthesized(field_0) => {
                AstNode::<ParenthesizedExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::Lambda(field_0) => {
                AstNode::<LambdaExpression<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
            ElementValue::MethodReference(field_0) => {
                AstNode::<MethodReference<'a>> {
                    inner: field_0.as_ref(),
                    parent: self.parent,
                    following_span_start: self.following_span_start,
                }
                .fmt(f);
            }
        }
    }
}
impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, ElementValueArrayInitializer<'a>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) {
            self.write(f);
            return;
        }
        self.format_leading_comments(f);
        self.write(f);
        self.format_trailing_comments(f);
    }
}
