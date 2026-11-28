// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/tasks/ast_tools/src/generators/formatter/format.rs

use std::{fmt::Write as _, io};

use crate::{
    generators::{Generator, define_generator},
    output::Output,
    schema::{Def, Schema, StructOrEnum},
};

use super::{
    FORMATTER_GENERATED_PATH, ast_nodes::has_span, check_choice_payloads, enum_pattern, payload,
    source,
};

/// Generator for the formatter's `Format` implementations.
pub struct FormatterFormatGenerator;

define_generator!(FormatterFormatGenerator);

impl Generator for FormatterFormatGenerator {
    fn generate(&self, schema: &Schema) -> io::Result<Vec<Output>> {
        check_choice_payloads(schema)?;
        let path = format!("{FORMATTER_GENERATED_PATH}/format.rs");
        Ok(vec![Output { path, code: generate(schema) }])
    }
}

/// Nodes whose `fmt` prints no leading or trailing comments, because their own
/// printer or their parent's places them.
const AST_NODE_WITHOUT_PRINTING_COMMENTS_LIST: &[&str] = &[
    // The root has no parent to bound its comments, and prints those left at the end of the file.
    "CompilationUnit",
    // Comments before a unit go to its first declaration, and the root prints those after it.
    "OrdinaryCompilationUnit",
    "CompactCompilationUnit",
    "ModularCompilationUnit",
    // Their printers print the comments before `(`, and the declaration or lambda that holds them
    // places those after `)`, such as the comments before `->`.
    "FormalParameters",
    "RecordHeader",
    // The declaration, enum constant, or class instance creation that holds a body prints the
    // comments before its `{`, some of which move after it.
    "ClassBody",
    "InterfaceBody",
    "EnumBody",
    "AnnotationInterfaceBody",
    // The `try` statement prints `catch` with the comments before it,
    // and the next clause or the statement prints those after it.
    "CatchClause",
    // Its printer prints the parentheses, keeping the comments before the `)` inside them.
    "CatchParameter",
];

pub(super) fn generate(schema: &Schema) -> String {
    let mut output = String::from(
        "use robusta_ast::*; use robusta_formatter_core::Format; use robusta_span::GetSpan; use crate::{ast_nodes::AstNode, formatter::{JavaFormatContext, JavaFormatter, JavaFormatterExt as _}, print::FormatWrite};\n",
    );
    for item in schema.visited() {
        if let StructOrEnum::Enum(enum_def) = item {
            let _ = writeln!(
                output,
                "impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, {}> {{ fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {{ match self.inner {{",
                source(&item, schema)
            );
            for variant in enum_def.all_variants(schema) {
                let pattern = enum_pattern(enum_def, variant);
                let (ty, inner) = payload(variant, schema);
                let ty = source(ty, schema);
                let _ = writeln!(
                    output,
                    "{pattern} => {{ AstNode::<{ty}> {{ inner: {inner}, parent: self.parent, following_span_start: self.following_span_start }}.fmt(f); }},"
                );
            }
            output += "}}}\n";
            continue;
        }
        let _ = writeln!(
            output,
            "impl<'a> Format<'a, JavaFormatContext<'a>> for AstNode<'_, 'a, {}> {{ fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {{",
            source(&item, schema)
        );
        let comments =
            has_span(item) && !AST_NODE_WITHOUT_PRINTING_COMMENTS_LIST.contains(&item.name());
        if comments {
            // Most nodes have no comment near them: skip all of the comment handling below.
            output += "if f.comments().is_quiet(self.span(), || self.enclosing_span(), self.following_span_start) { self.write(f); return; }";
            output += "self.format_leading_comments(f);";
        }
        output += "self.write(f);";
        if comments {
            output += "self.format_trailing_comments(f);";
        }
        output += "}}\n";
    }
    output
}
