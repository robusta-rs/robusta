use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::formatter::trivia::FormatTrailingComments;
use crate::print::FormatWrite;
use crate::print::annotations::format_annotations_stacked;
use crate::print::class::join_type_members;
use crate::print::semicolon::FormatContentWithSemicolon;
use robusta_allocator::Vec;
use robusta_ast::{
    CompactCompilationUnit, CompilationUnit, CompilationUnitContent, ModularCompilationUnit,
    OrdinaryCompilationUnit, PackageDeclaration, TypeDeclaration,
};
use robusta_formatter_core::write;
use robusta_span::GetSpan;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, CompilationUnit<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.content().fmt(f);
        let mut wrote_content = match &self.content {
            CompilationUnitContent::Ordinary(unit) => {
                unit.package.is_some() || !unit.imports.is_empty() || !unit.types.is_empty()
            }
            CompilationUnitContent::Compact(unit) => {
                !unit.imports.is_empty() || !unit.members.is_empty()
            }
            CompilationUnitContent::Modular(_) => true,
        };
        let comments = f.comments().unprinted_comments();
        if !comments.is_empty() {
            FormatTrailingComments::Comments(comments).fmt(f);
            wrote_content = true;
        }
        if wrote_content && f.options().insert_final_newline {
            hard_line_break().fmt(f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, OrdinaryCompilationUnit<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        if self.imports.is_empty() {
            let mut join = f.join_nodes_with_hardline();
            let package = self.package();
            if let Some(package) = &package {
                join.entry_with_empty_line(false, package);
            }
            join_type_declarations(&mut join, package.is_some(), &self.types());
        } else {
            if let Some(package) = self.package() {
                write!(f, [package, empty_line()]);
            }
            self.imports().fmt(f);
            join_type_declarations(&mut f.join_nodes_with_hardline(), false, &self.types());
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, CompactCompilationUnit<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.imports().fmt(f);
        join_type_members(&mut f.join_nodes_with_hardline(), &self.members());
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ModularCompilationUnit<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        self.imports().fmt(f);
        self.module().fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, PackageDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_annotations_stacked(f, &self.annotations());
        write!(f, [token("package"), space()]);
        FormatContentWithSemicolon::new(&self.name(), self.name.span().end, self.span.end).fmt(f);
    }
}

/// Separate type declarations by an empty line, and an empty declaration by
/// a line break. A package declaration the join holds takes an empty line
/// after it.
fn join_type_declarations<'a>(
    join: &mut JoinNodesBuilder<'_, '_, 'a, Line>,
    after_package: bool,
    declarations: &AstNode<'_, 'a, Vec<'a, TypeDeclaration<'a>>>,
) {
    let mut previous_was_package = after_package;
    let mut previous_was_type = false;
    for declaration in declarations {
        let is_empty = matches!(declaration.inner(), TypeDeclaration::Empty(_));
        let empty_line = previous_was_package || (!is_empty && previous_was_type);
        join.entry_with_empty_line(empty_line, &declaration);
        previous_was_package = false;
        previous_was_type = !is_empty;
    }
}
