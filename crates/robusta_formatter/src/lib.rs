// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_formatter/src/lib.rs

mod ast_nodes;
#[cfg(feature = "check_idempotency")]
mod check_idempotency;
mod check_line_width;
#[cfg(feature = "detect_code_removal")]
mod detect_code_removal;
mod formatter;
mod ir_transform;
mod options;
mod print;
mod source_text;
mod utils;

use ast_nodes::{AstNode, AstNodes};
#[cfg(feature = "check_idempotency")]
pub use check_idempotency::{IdempotencyReport, check_idempotency};
pub use check_line_width::{LineWidthReport, WideLine, WideLineCause, check_line_width};
#[cfg(feature = "detect_code_removal")]
pub use detect_code_removal::detect_code_removal;
use formatter::prelude::*;
pub use ir_transform::sort_imports::options::{
    ImportGroup, ImportMatcher, ImportOrder, ImportSection,
};
pub use options::{
    AfterClosingBrace, CommentLineStrategy, DescriptionWithDot, JavadocOptions, LineWrappingStyle,
    Options, SeparateTagGroups,
};
use robusta_allocator::Allocator;
use robusta_ast::CompilationUnit;
use robusta_diagnostics::RobustaDiagnostic;
use robusta_formatter_core::{Document, FormatState, Formatted, Label};
pub use robusta_formatter_core::{IndentStyle, IndentWidth, LineEnding, LineWidth};

const UTF8_BOM: &str = "\u{feff}";

#[derive(Copy, Clone)]
pub(crate) enum JavaLabels {
    MemberChain,
    ImportDeclaration,
    Comment,
}

impl Label for JavaLabels {
    fn id(&self) -> u32 {
        *self as u32
    }

    fn debug_name(&self) -> &'static str {
        match self {
            Self::MemberChain => "MemberChain",
            Self::ImportDeclaration => "ImportDeclaration",
            Self::Comment => "Comment",
        }
    }
}

/// Parse `source` and print it in robusta's canonical layout.
///
/// # Errors
///
/// Returns the first parse diagnostic, even when the parser recovered.
pub fn format(source: &str, options: &Options) -> Result<String, RobustaDiagnostic> {
    let allocator = Allocator::new();
    let result = robusta_parser::parse(&allocator, source);
    if let Some(error) = result.errors.into_iter().next() {
        return Err(error);
    }
    Ok(format_compilation_unit(&allocator, source, &result.unit, options))
}

/// Print an already-parsed `unit` in robusta's canonical layout, re-attaching
/// the unit's own comments by source position.
///
/// Most callers want [`format()`]. `unit` must come from a clean parse.
///
/// The flat IR is allocated in `allocator`, usually the arena that holds `unit`,
/// and is freed with it, so the IR needs no per-element deallocation.
/// A leading UTF-8 byte-order mark is preserved as file metadata but remains
/// outside the syntax tree and formatting IR.
pub fn format_compilation_unit<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    unit: &'a CompilationUnit<'a>,
    options: &Options,
) -> String {
    let has_utf8_bom = source.starts_with(UTF8_BOM);
    let formatted = format_unit(allocator, source, unit, options);
    if !has_utf8_bom {
        return formatted;
    }

    let mut with_bom = String::with_capacity(UTF8_BOM.len() + formatted.len());
    with_bom.push_str(UTF8_BOM);
    with_bom.push_str(&formatted);
    with_bom
}

fn format_unit<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    unit: &'a CompilationUnit<'a>,
    options: &Options,
) -> String {
    let code =
        format_node(allocator, JavaFormatContext::new(allocator, source, unit, options), |f| {
            AstNode::new(unit, AstNodes::Dummy).fmt(f);
        });
    formatter::javadoc::format_embedded(allocator, code, options)
}

pub(crate) fn format_node<'a>(
    allocator: &'a Allocator,
    context: JavaFormatContext<'a>,
    build: impl FnOnce(&mut JavaFormatter<'_, 'a>),
) -> String {
    let insert_final_newline = context.options.insert_final_newline;
    let source_len = context.source_text.len();
    let mut state = FormatState::new(context, allocator);
    let mut f = JavaFormatter::with_capacity(source_len * 2 / 5, &mut state);
    build(&mut f);
    let elements = f.into_vec();
    let context = state.into_context();
    let document = Document::new(elements);
    let mut code = Formatted::new(document, context)
        .print()
        .expect("the Java formatter emits valid paired tags")
        .into_code();
    if !insert_final_newline {
        // Comment separators can emit a final line break.
        code.truncate(code.trim_end_matches(['\n', '\r']).len());
    }
    code
}
