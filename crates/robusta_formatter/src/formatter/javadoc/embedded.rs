use std::borrow::Cow;
use std::ops::Range;

use markdown::mdast::Node;
use robusta_allocator::Allocator;
use robusta_ast::{ClassMember, Comment, CompilationUnitContent, TypeDeclaration};
use robusta_formatter_core::{Format, SourceText, text_width};
use robusta_javadoc::markdown::{block_end, continuation_prefix, parse, remove_margin, span};
use robusta_javadoc::{Javadoc, JavadocKind, Structure, balanced_inline_end};

use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::trivia::FormatTrailingComments;
use crate::formatter::{Comments, JavaFormatterExt as _, javadoc::JavadocDeclarations};
use crate::{LineEnding, LineWidth, Options};

/// What formatting the snippets of one comment shares.
#[derive(Clone, Copy)]
struct SnippetContext<'a> {
    allocator: &'a Allocator,
    options: &'a Options,
}

/// Rewrite snippets after host layout fixes the comment's indentation.
pub fn format_embedded(allocator: &Allocator, code: String, options: &Options) -> String {
    if options.javadoc.is_none()
        || !(code.contains("{@snippet")
            || code.contains("{@code")
                && code.as_bytes().windows(4).any(|tag| tag.eq_ignore_ascii_case(b"<pre"))
            || code.contains("///") && code.contains("```"))
    {
        return code;
    }
    let parsed = robusta_parser::parse(allocator, &code);
    if !parsed.errors.is_empty() {
        return code;
    }
    let comments = Comments::new(SourceText::new(&code), &parsed.unit.comments)
        .with_declarations(JavadocDeclarations::collect(&parsed.unit));
    let mut edits = Vec::new();
    let mut pending = &parsed.unit.comments[..];
    while let Some(&comment) = pending.first() {
        // A `///` run is judged whole, as the formatter claims it.
        let Some((document, count)) =
            Javadoc::from_comments(&code, pending).filter(|(document, _)| {
                comments.should_format_javadoc(Comment { span: document.span(), ..comment }, false)
            })
        else {
            pending = &pending[1..];
            continue;
        };
        pending = &pending[count..];
        let indent = comment_indent(document, &code, options);
        let columns = text_width(&indent, options.indent_width) as usize;
        let snippet_options = snippet_options(options, columns);
        if let Some(content) = format_content(
            SnippetContext { allocator, options: &snippet_options },
            &document.literal_lines().expect("literal Javadoc delimiters").join("\n"),
            document.kind(),
        ) {
            edits
                .push((document.span(), with_margins(&content, document.kind(), &indent, options)));
        }
    }
    if edits.is_empty() {
        return code;
    }
    let mut output = String::with_capacity(code.len());
    let mut cursor = 0;
    for (span, replacement) in edits {
        output.push_str(&code[cursor..span.start as usize]);
        output.push_str(&replacement);
        cursor = span.end as usize;
    }
    output.push_str(&code[cursor..]);
    output
}

fn comment_indent<'a>(document: Javadoc<'a>, code: &'a str, options: &Options) -> Cow<'a, str> {
    if let Some(newline) = document.text().find(['\n', '\r']) {
        let line = document.text()[newline..].trim_start_matches(['\n', '\r']);
        let indent = &line[..line.len() - line.trim_start_matches([' ', '\t']).len()];
        return Cow::Borrowed(if document.kind() == JavadocKind::Traditional {
            indent.strip_suffix(' ').unwrap_or(indent)
        } else {
            indent
        });
    }
    let before = &code[..document.span().start as usize];
    let line = &before[before.rfind(['\n', '\r']).map_or(0, |newline| newline + 1)..];
    let rest = line.trim_start_matches([' ', '\t']);
    let indent = &line[..line.len() - rest.len()];
    if document.kind() == JavadocKind::Markdown || rest.is_empty() {
        return Cow::Borrowed(indent);
    }
    // The stars line up under the first one, as the formatter prints them.
    let width = text_width(rest, options.indent_width) as usize;
    Cow::Owned(format!("{indent}{:width$}", ""))
}

fn with_margins(content: &str, kind: JavadocKind, indent: &str, options: &Options) -> String {
    let newline = std::str::from_utf8(options.line_ending.as_bytes()).expect("ASCII line ending");
    let mut output = String::new();
    match kind {
        JavadocKind::Traditional => {
            output.push_str("/**");
            for line in content.trim_matches('\n').split('\n') {
                output.push_str(newline);
                output.push_str(indent);
                output.push_str(" *");
                if !line.is_empty() {
                    output.push(' ');
                    output.push_str(line);
                }
            }
            output.push_str(newline);
            output.push_str(indent);
            output.push_str(" */");
        }
        JavadocKind::Markdown => {
            for (index, line) in content.split('\n').enumerate() {
                if index > 0 {
                    output.push_str(newline);
                    output.push_str(indent);
                }
                output.push_str("///");
                if !line.is_empty() {
                    output.push(' ');
                    output.push_str(line);
                }
            }
        }
    }
    output
}

/// Format parsed Java regions, keeping everything outside them intact.
fn format_content(context: SnippetContext<'_>, content: &str, kind: JavadocKind) -> Option<String> {
    if !(content.contains("{@") || kind == JavadocKind::Markdown && content.contains('`')) {
        return None;
    }
    let structure = match kind {
        JavadocKind::Traditional => Structure::from_traditional(content),
        JavadocKind::Markdown => Structure::from_markdown(content),
    };
    let mut edits = Vec::new();
    for range in structure.literal_ranges() {
        let text = &content[range.clone()];
        if let Some(body) = java_body(text) {
            let code = &text[body.clone()];
            if text.starts_with("{@snippet") && has_markup(code) {
                continue;
            }
            let margin = if kind == JavadocKind::Traditional { 3 } else { 4 };
            if let Some(formatted) = format_snippet(context, code, margin) {
                edits.push((
                    range.start + body.start..range.start + body.end,
                    format!("{}\n", code_lines(&formatted, "")),
                ));
            }
        }
    }
    if kind == JavadocKind::Markdown
        && let Some(root) = parse(content)
    {
        code_blocks(context, &root, content, structure.literal_ranges(), &mut edits);
    }
    edits.retain(|(range, text)| content[range.clone()] != *text);
    if edits.is_empty() {
        return None;
    }
    edits.sort_by_key(|(range, _)| range.start);
    let mut result = String::with_capacity(content.len());
    let mut cursor = 0;
    for (range, replacement) in edits {
        result.push_str(content.get(cursor..range.start)?);
        result.push_str(&replacement);
        cursor = range.end;
    }
    result.push_str(&content[cursor..]);
    // A code line rewritten under its fence's prefix can indent the least-indented line.
    Some(if kind == JavadocKind::Markdown { remove_margin(&result) } else { result })
}

/// The body of a standalone snippet or a `<pre>{@code ...}</pre>`.
fn java_body(text: &str) -> Option<Range<usize>> {
    if let Some(attributes) = text.strip_prefix("{@snippet") {
        let colon = unquoted_delimiter(attributes, ':')?;
        let attributes = &attributes[..colon];
        let mut rest = attributes.trim();
        while !rest.is_empty() {
            let name_end = rest.find(|ch: char| ch.is_whitespace() || ch == '=')?;
            let name = &rest[..name_end];
            rest = rest[name_end..].trim_start().strip_prefix('=')?.trim_start();
            let (value, after) = if let Some(quote @ ('\'' | '"')) = rest.chars().next() {
                let end = rest[1..].find(quote)? + 1;
                (&rest[1..end], &rest[end + 1..])
            } else {
                let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
                (&rest[..end], &rest[end..])
            };
            if matches!(name, "file" | "class") || (name == "lang" && value != "java") {
                return None;
            }
            rest = after.trim_start();
        }
        let end = balanced_inline_end(text, 0)?;
        let start = "{@snippet".len() + colon + 1;
        // Javadoc reads the body from the line after the colon, skipping only
        // spaces and tabs before it, and rejects a snippet without one.
        let opens_line = text[start..].trim_start_matches([' ', '\t']).starts_with('\n');
        return (end == text.len() && opens_line).then_some(start..end - 1);
    }
    let open_end = unquoted_delimiter(text, '>')? + 1;
    if !text.get(..4).is_some_and(|prefix| prefix.eq_ignore_ascii_case("<pre")) {
        return None;
    }
    let inner = text[open_end..].trim_start();
    let start = text.len() - inner.len();
    if !inner
        .strip_prefix("{@code")
        .is_some_and(|rest| rest.as_bytes().first().is_some_and(u8::is_ascii_whitespace))
    {
        return None;
    }
    let end = balanced_inline_end(text, start)?;
    let close = text[end..].trim().strip_circumfix("</", '>')?.trim_end();
    close.eq_ignore_ascii_case("pre").then_some(start + "{@code".len()..end - 1)
}

fn unquoted_delimiter(text: &str, delimiter: char) -> Option<usize> {
    let mut quoted = None;
    text.char_indices().find_map(|(index, ch)| {
        if Some(ch) == quoted {
            quoted = None;
        } else if quoted.is_none() {
            match ch {
                '\'' | '"' => quoted = Some(ch),
                _ if ch == delimiter => return Some(index),
                _ => {}
            }
        }
        None
    })
}

fn has_markup(code: &str) -> bool {
    robusta_lexer::tokenize(code).comments.iter().any(|comment| {
        let text = comment.span.source_text(code);
        ["@highlight", "@replace", "@link", "@start", "@end"].iter().any(|tag| {
            text.match_indices(tag).any(|(start, tag)| {
                text[start + tag.len()..].chars().next().is_none_or(|ch| !ch.is_ascii_alphabetic())
            })
        })
    })
}

fn code_blocks(
    context: SnippetContext<'_>,
    node: &Node,
    content: &str,
    literals: &[Range<usize>],
    edits: &mut Vec<(Range<usize>, String)>,
) {
    if let Node::Code(code) = node
        && code.lang.as_ref().is_none_or(|lang| lang.eq_ignore_ascii_case("java"))
        && let Some(range) = span(node)
        && !literals.iter().any(|literal| {
            literal.start <= range.start
                && range.end <= literal.end
                && content[literal.start..].starts_with("{@")
        })
    {
        let prefix = continuation_prefix(content, range.start);
        let margin = 4 + text_width(&prefix, context.options.indent_width) as usize;
        if let Some(formatted) = format_snippet(context, &code.value, margin) {
            let source = &content[range.clone()];
            let first_end = source.find('\n').unwrap_or(source.len());
            let opening = &source[..first_end];
            let marker = source.as_bytes()[0];
            let count = source.bytes().take_while(|byte| *byte == marker).count();
            if matches!(marker, b'`' | b'~') && count >= 3 {
                let fence = char::from(marker).to_string().repeat(count);
                let body = code_lines(&formatted, &prefix);
                let end = block_end(content, &range);
                edits.push((range.start..end, format!("{opening}{body}\n{prefix}{fence}")));
            }
        }
        return;
    }
    for child in node.children().into_iter().flatten() {
        code_blocks(context, child, content, literals, edits);
    }
}

/// Each line of `code` after a newline and `prefix`, which a blank line keeps without its
/// trailing whitespace; empty code adds no line.
pub(super) fn code_lines(code: &str, prefix: &str) -> String {
    let mut lines = String::new();
    for line in code.lines() {
        lines.push('\n');
        if line.is_empty() {
            lines.push_str(prefix.trim_end());
        } else {
            lines.push_str(prefix);
            lines.push_str(line);
        }
    }
    lines
}

fn snippet_options(options: &Options, margin: usize) -> Options {
    let width =
        options.line_width.value().saturating_sub(u16::try_from(margin).unwrap_or(u16::MAX)).max(1);
    Options {
        line_width: LineWidth::try_from(width).expect("reduced valid width"),
        line_ending: LineEnding::Lf,
        javadoc: None,
        insert_final_newline: false,
        ..options.clone()
    }
}

/// Try compilation units, members, then statements. Wrappers are parsed but never printed.
fn format_snippet(context: SnippetContext<'_>, code: &str, margin: usize) -> Option<String> {
    let SnippetContext { allocator, options } = context;
    let options = snippet_options(options, margin);
    for (prefix, suffix) in [
        ("", ""),
        ("class __Snippet {\n", "\n}"),
        ("class __Snippet { void __snippet() {\n", "\n} }"),
    ] {
        let source = format!("{prefix}{code}\n{suffix}");
        let parsed = robusta_parser::parse(allocator, &source);
        if !parsed.errors.is_empty() {
            continue;
        }
        let root = AstNode::new(&parsed.unit, AstNodes::Dummy);
        if !prefix.is_empty() && !valid_wrapper(&parsed.unit, source.len(), suffix) {
            continue;
        }
        let mut context =
            crate::formatter::JavaFormatContext::new(allocator, &source, &parsed.unit, &options);
        context.is_snippet = true;
        let formatted = crate::format_node(allocator, context, |f| {
            if prefix.is_empty() {
                root.fmt(f);
                return;
            }
            let CompilationUnitContent::Ordinary(unit) = &parsed.unit.content else {
                unreachable!()
            };
            let TypeDeclaration::Class(class) = &unit.types[0] else { unreachable!() };
            let class = AstNode::new(class.as_ref(), AstNodes::Dummy);
            let mut join = f.join_nodes_with_hardline();
            if suffix == "\n}" {
                crate::print::class::join_type_members(&mut join, &class.body().members());
            } else {
                let ClassMember::Method(method) = &class.inner().body.members[0] else {
                    unreachable!()
                };
                let block = method.body.as_ref().expect("wrapper body");
                let block = AstNode::new(block, AstNodes::Dummy);
                join.entries(block.statements().iter());
            }
            FormatTrailingComments::Comments(f.comments().unprinted_comments()).fmt(f);
        });
        return Some(formatted.trim_end_matches('\n').to_string());
    }
    None
}

fn valid_wrapper(unit: &robusta_ast::CompilationUnit<'_>, length: usize, suffix: &str) -> bool {
    let CompilationUnitContent::Ordinary(unit) = &unit.content else { return false };
    let [TypeDeclaration::Class(class)] = &unit.types[..] else { return false };
    if class.span.end as usize != length {
        return false;
    }
    if suffix == "\n}" {
        return true;
    }
    let [ClassMember::Method(method)] = &class.body.members[..] else { return false };
    method.body.as_ref().is_some_and(|body| body.span.end as usize == length - 2)
}
