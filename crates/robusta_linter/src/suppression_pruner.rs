use std::ptr;

use robusta_ast::{Annotation, CommentKind, ElementValue};
use robusta_semantic::Semantic;
use robusta_span::{GetSpan, Span};
use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::{contains_line_terminator, line_content_end, line_start, terminator_len},
    unicode_escape::skip_whitespace,
};

use crate::Fix;
use crate::disable_directives::{CommentRemoval, RuleComment, Suppression, SuppressionOrigin};

/// One removal edit per unused suppression, in input order.
///
/// `None` marks a suppression too ambiguous to rewrite, or one whose removal
/// would change which declaration a documentation comment documents.
/// Suppressions in one annotation or comment, and deletions that empty one
/// line, share an edit.
pub fn removal_edits(semantic: &Semantic<'_>, unused: &[&Suppression<'_>]) -> Vec<Option<Fix>> {
    let source = semantic.source_text();
    let mut edits = unused
        .iter()
        .map(|suppression| match suppression.origin {
            SuppressionOrigin::Annotation(annotation) => {
                let directives = unused
                    .iter()
                    .filter(|other| {
                        matches!(other.origin, SuppressionOrigin::Annotation(other)
                            if ptr::eq(other, annotation))
                    })
                    .map(|other| other.directive)
                    .collect::<Vec<_>>();
                prune_annotation(semantic, annotation, &directives)
            }
            SuppressionOrigin::Comment(comment) => {
                let directives = unused
                    .iter()
                    .filter(|other| {
                        matches!(other.origin, SuppressionOrigin::Comment(other)
                            if other.span == comment.span)
                    })
                    .map(|other| other.directive)
                    .collect::<Vec<_>>();
                prune_comment(source, comment, &directives)
            }
        })
        .collect::<Vec<_>>();

    let mut normalized = edits.iter().flatten().cloned().collect::<Vec<_>>();
    normalize_deletions(source, &mut normalized);
    for edit in edits.iter_mut().flatten().filter(|edit| edit.content.is_empty()) {
        if let Some(merged) = normalized.iter().find(|merged| {
            merged.content.is_empty()
                && merged.span.start <= edit.span.start
                && edit.span.end <= merged.span.end
        }) {
            edit.clone_from(merged);
        }
    }
    edits
}

fn prune_comment(source: &str, comment: RuleComment<'_>, directives: &[Span]) -> Option<Fix> {
    let retained = comment
        .rule_names(source)
        .into_iter()
        .filter_map(|(name, span)| (!directives.contains(&span)).then_some(name))
        .collect::<Vec<_>>();
    if !retained.is_empty() {
        return Some(Fix::new(retained.join(", "), comment.rules));
    }
    comment.removal.map(comment_removal)
}

/// Deletes a suppression comment, or rewrites it to the explanation it keeps.
fn comment_removal(removal: CommentRemoval<'_>) -> Fix {
    let explanation = removal.explanation;
    if explanation.is_empty() {
        return Fix::delete(removal.span);
    }
    let comment = match removal.kind {
        CommentKind::Line => format!("// {explanation}"),
        CommentKind::SingleLineBlock | CommentKind::MultiLineBlock => {
            format!("/* {explanation} */")
        }
    };
    Fix::new(comment, removal.span)
}

fn normalize_deletions(source: &str, edits: &mut Vec<Fix>) {
    coalesce_deletions(edits);
    for edit in &mut *edits {
        if edit.content.is_empty() {
            edit.span = expand_empty_line_deletion(source, edit.span);
        }
    }
    coalesce_deletions(edits);
}

fn coalesce_deletions(edits: &mut Vec<Fix>) {
    edits.sort_by_key(|edit| (edit.span.start, edit.span.end));
    let mut normalized = Vec::<Fix>::with_capacity(edits.len());
    for edit in edits.drain(..) {
        if let Some(previous) = normalized.last_mut()
            && previous.content.is_empty()
            && edit.content.is_empty()
            && edit.span.start <= previous.span.end
        {
            previous.span.end = previous.span.end.max(edit.span.end);
        } else {
            normalized.push(edit);
        }
    }
    *edits = normalized;
}

#[expect(clippy::cast_possible_truncation)]
fn expand_empty_line_deletion(source: &str, deletion: Span) -> Span {
    if contains_line_terminator(deletion.source_text(source)) {
        return deletion;
    }
    let line_start = line_start(source, deletion.start) as usize;
    let line_end = line_content_end(source, deletion.end) as usize;
    let before = &source[line_start..deletion.start as usize];
    let after = &source[deletion.end as usize..line_end];
    if !before.bytes().all(is_horizontal_whitespace) || !after.bytes().all(is_horizontal_whitespace)
    {
        return deletion;
    }

    let end = line_end + terminator_len(source, line_end);
    Span::new(line_start as u32, end as u32)
}

fn prune_annotation(
    semantic: &Semantic<'_>,
    annotation: &Annotation<'_>,
    directives: &[Span],
) -> Option<Fix> {
    let source = semantic.source_text();
    let value = annotation.value()?;
    let deletable =
        annotation.args.as_ref()?.len() == 1 && !precedes_documentation(semantic, annotation.span);

    match value {
        ElementValue::Array(array) => {
            let mut retained = Vec::new();
            let mut matched = 0usize;
            for value in &array.values {
                if directives.iter().any(|directive| same_span(value.span(), *directive)) {
                    matched += 1;
                } else {
                    retained.push(value.span());
                }
            }
            if matched != directives.len() {
                return None;
            }
            if retained.is_empty() {
                return deletable.then(|| Fix::delete(removal_span(source, annotation.span)));
            }
            if !plain_array_syntax(source, array.span, &array.values) {
                return None;
            }
            let replacement = format!(
                "{{{}}}",
                retained.iter().map(|span| span.source_text(source)).collect::<Vec<_>>().join(", ")
            );
            Some(Fix::new(replacement, array.span))
        }
        _ => {
            if directives.len() == 1 && same_span(value.span(), directives[0]) && deletable {
                Some(Fix::delete(removal_span(source, annotation.span)))
            } else {
                None
            }
        }
    }
}

/// Whether a documentation comment directly follows `annotation`, past
/// whitespace, escaped or not, and other comments.
///
/// Deleting the annotation can make the comment's token start the declaration,
/// and javac then gives the comment to the declaration.
fn precedes_documentation(semantic: &Semantic<'_>, annotation: Span) -> bool {
    let source = semantic.source_text();
    let comments = semantic.comments();
    let mut end = annotation.end;
    for comment in &comments[comments.partition_point(|comment| comment.span.start < end)..] {
        if skip_whitespace(source, end, |byte| byte.is_ascii_whitespace()) != comment.span.start {
            return false;
        }
        if comment.is_documentation() {
            return true;
        }
        end = comment.span.end;
    }
    false
}

fn same_span(left: Span, right: Span) -> bool {
    left.start == right.start && left.end == right.end
}

fn plain_array_syntax(source: &str, array: Span, values: &[ElementValue<'_>]) -> bool {
    let mut cursor = array.start as usize + 1;
    for value in values {
        let span = value.span();
        if !separator_is_plain(&source[cursor..span.start as usize]) {
            return false;
        }
        cursor = span.end as usize;
    }
    separator_is_plain(&source[cursor..array.end as usize - 1])
}

fn separator_is_plain(separator: &str) -> bool {
    separator.chars().all(|character| character == ',' || character.is_whitespace())
}

/// The span deleting `item`, with its lines when nothing else is on them,
/// otherwise with the whitespace on one side.
// Parser spans are bounded to `u32`, so the source positions converted back
// to `Span` offsets below cannot truncate.
#[expect(clippy::cast_possible_truncation)]
pub fn removal_span(source: &str, item: Span) -> Span {
    let bytes = source.as_bytes();
    let line_start = line_start(source, item.start) as usize;
    let line_end = line_content_end(source, item.end) as usize;
    let before = &source[line_start..item.start as usize];
    let after = &source[item.end as usize..line_end];

    let leading = before.bytes().all(is_horizontal_whitespace);
    if leading && after.bytes().all(is_horizontal_whitespace) {
        let end = line_end + terminator_len(source, line_end);
        return Span::new(line_start as u32, end as u32);
    }

    let mut start = item.start as usize;
    while !leading && start > line_start && is_horizontal_whitespace(bytes[start - 1]) {
        start -= 1;
    }
    if start < item.start as usize {
        return Span::new(start as u32, item.end);
    }

    // `@SuppressWarnings("robusta:x") // reason` -> `// reason`
    let mut end = item.end as usize;
    while end < line_end && is_horizontal_whitespace(bytes[end]) {
        end += 1;
    }
    if !leading && source[end..line_end].starts_with("//") {
        end = item.end as usize;
    }
    Span::new(item.start, end as u32)
}
