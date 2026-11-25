use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::ir_transform::sort_imports::{
    FormattedImports, SortableImport, import_sort_key, sort_imports_chunk,
};
use crate::print::semicolon::FormatContentWithSemicolon;
use crate::print::{FormatWrite, format_dot_before_nodeless_operand};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use robusta_allocator::Vec;
use robusta_ast::{ImportDeclaration, ImportKind};
use robusta_formatter_core::{LabelId, write};
use robusta_span::GetSpan;

use crate::JavaLabels;

impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, Vec<'a, ImportDeclaration<'a>>> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if self.is_empty() {
            return;
        }
        let mut imports = FormattedImports {
            start: f.elements().len(),
            items: std::vec::Vec::with_capacity(self.len()),
        };
        let preserve_order = f.context().is_snippet;
        for (index, import) in self.iter().enumerate() {
            if preserve_order && index > 0 {
                hard_line_break().fmt(f);
            }
            let start = f.elements().len();
            import.fmt(f);
            imports.items.push(SortableImport {
                key: import_sort_key(f.options().import_order(), &import),
                elements: start..f.elements().len(),
            });
        }
        if !preserve_order {
            sort_imports_chunk(f, imports);
        }
        let declarations_follow = match self.parent() {
            AstNodes::OrdinaryCompilationUnit(unit) => !unit.types.is_empty(),
            AstNodes::CompactCompilationUnit(_) | AstNodes::ModularCompilationUnit(_) => true,
            _ => false,
        };
        if declarations_follow {
            empty_line().fmt(f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ImportDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let declaration = format_once(|f| {
            let name = self.name.span();
            let on_demand =
                matches!(self.kind, ImportKind::TypeOnDemand | ImportKind::StaticOnDemand);
            let content = format_with(|f| {
                "import".fmt(f);
                let modifier = match self.kind {
                    ImportKind::SingleStatic | ImportKind::StaticOnDemand => Some("static"),
                    ImportKind::SingleModule => Some("module"),
                    ImportKind::SingleType | ImportKind::TypeOnDemand => None,
                };
                if let Some(modifier) = modifier {
                    write!(f, [space(), modifier]);
                }
                space().fmt(f);
                if on_demand {
                    format_content_without_comments_after(&self.name(), name.end, f);
                    format_dot_before_nodeless_operand(name, &"*", f);
                } else {
                    self.name().fmt(f);
                }
            });
            let content_end = if on_demand {
                f.comments().position_after_character(name.end, b'*')
            } else {
                name.end
            };
            FormatContentWithSemicolon::new(&content, content_end, self.span.end).fmt(f);
        });
        labelled(LabelId::of(JavaLabels::ImportDeclaration), &declaration).fmt(f);
    }
}
