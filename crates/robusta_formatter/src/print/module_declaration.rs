use crate::ast_nodes::{AstNode, AstNodeListElement};
use crate::formatter::prelude::*;
use crate::print::FormatWrite;
use crate::print::annotations::{format_annotations_stacked, format_inline_nodes};
use crate::print::semicolon::FormatContentWithSemicolon;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::{format_attached_braced_body, write_comments_between_blocks};
use robusta_ast::{
    ModuleDeclaration, PackageDirective, ProvidesDirective, RequiresDirective, UsesDirective,
};
use robusta_formatter_core::write;
use robusta_span::GetSpan;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ModuleDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_annotations_stacked(f, &self.annotations());
        if self.is_open {
            write!(f, ["open", space()]);
        }
        write!(f, ["module", space()]);
        let name_end = self.name.span().end;
        format_content_without_comments_after(&self.name(), name_end, f);
        format_attached_braced_body(f, name_end, self.directives.is_empty(), self.span, |f| {
            f.join_nodes_with_hardline().entries(self.directives().iter());
        });
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, RequiresDirective<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let content = format_with(|f| {
            write!(f, ["requires", space()]);
            format_inline_nodes(f, self.modifiers().iter());
            self.name().fmt(f);
        });
        FormatContentWithSemicolon::new(&content, self.name.span().end, self.span.end)
            .moving_documentation_comments()
            .fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, PackageDirective<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let content = format_with(|f| {
            write!(f, [self.kind.as_str(), space()]);
            if self.targets.is_empty() {
                self.name().fmt(f);
            } else {
                format_content_without_comments_after(&self.name(), self.name.span().end, f);
                format_module_targets(f, &self.targets(), self.name.span().end, "to");
            }
        });
        let content_end = self.targets.last().map_or(self.name.span().end, |last| last.span().end);
        FormatContentWithSemicolon::new(&content, content_end, self.span.end)
            .moving_documentation_comments()
            .fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, UsesDirective<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let content = format_with(|f| write!(f, ["uses", space(), self.name()]));
        FormatContentWithSemicolon::new(&content, self.name.span().end, self.span.end)
            .moving_documentation_comments()
            .fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ProvidesDirective<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let content = format_with(|f| {
            write!(f, ["provides", space()]);
            format_content_without_comments_after(&self.name(), self.name.span().end, f);
            format_module_targets(f, &self.with(), self.name.span().end, "with");
        });
        let content_end = self.with.last().map_or(self.name.span().end, |last| last.span().end);
        FormatContentWithSemicolon::new(&content, content_end, self.span.end)
            .moving_documentation_comments()
            .fmt(f);
    }
}

fn format_module_targets<'parent, 'a, T: AstNodeListElement>(
    f: &mut JavaFormatter<'_, 'a>,
    targets: &AstNode<'parent, 'a, robusta_allocator::Vec<'a, T>>,
    name_end: u32,
    keyword: &'static str,
) where
    AstNode<'parent, 'a, T>: Format<'a, JavaFormatContext<'a>>,
{
    let comments = f.comments().comments_before_character(name_end, keyword.as_bytes()[0]);
    write_comments_between_blocks(comments, f);
    write!(f, [space(), keyword, space()]);
    for (index, target) in targets.iter().enumerate() {
        target.fmt(f);
        if index + 1 < targets.len() {
            write!(f, [",", line_suffix_boundary(), space()]);
        }
    }
}
