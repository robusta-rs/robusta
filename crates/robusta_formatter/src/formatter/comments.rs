// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/formatter/comments.rs

use std::cell::Cell;

use robusta_allocator::Allocator;
use robusta_ast::{Comment, parse_lint_disable_directive};
use robusta_formatter_core::SourceText;
use robusta_javadoc::{Javadoc, JavadocKind};
use robusta_span::Span;
use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::{line_content_end, line_start},
    unicode_escape,
};

use crate::formatter::javadoc::{DeclaredParameters, JavadocDeclarations};
use crate::source_text::{self, SourceTextExt as _};

#[derive(Clone, Copy)]
struct MovedLineComment {
    /// Where the code the comment moved behind ends.
    end: u32,
    comment: Comment,
}

/// The comment cursor: each comment prints once, in source order, found by position.
pub struct Comments<'a, 'c> {
    source: SourceText<'a>,
    /// The lexer's table before grouping rewritable `///` runs.
    lexed: &'c [Comment],
    inner: &'c [Comment],
    printed_count: Cell<usize>,
    view_limit: Cell<usize>,
    moved_line_comment: Cell<Option<MovedLineComment>>,
    declarations: JavadocDeclarations<'c>,
}

impl<'a, 'c> Comments<'a, 'c> {
    pub(crate) fn new(source: SourceText<'a>, comments: &'c [Comment]) -> Self {
        Self {
            source,
            lexed: comments,
            inner: comments,
            printed_count: Cell::new(0),
            view_limit: Cell::new(comments.len()),
            moved_line_comment: Cell::new(None),
            declarations: JavadocDeclarations::default(),
        }
    }

    /// Supply the declarations that decide whether a comment is Javadoc;
    /// without them every comment stays ordinary.
    pub(crate) fn with_declarations(mut self, declarations: JavadocDeclarations<'c>) -> Self {
        self.declarations = declarations;
        self
    }

    /// Make each `///` run that may be rewritten one comment, so it prints
    /// whole.
    pub(crate) fn with_markdown_javadoc_runs(mut self, allocator: &'c Allocator) -> Self {
        let mut grouped = Vec::with_capacity(self.lexed.len());
        let mut rest = self.lexed;
        while let Some(&first) = rest.first() {
            let (span, consumed) = self
                .rewritable_markdown_javadoc(rest)
                .map_or((first.span, 1), |(document, consumed)| (document.span(), consumed));
            grouped.push(Comment { span, ..first });
            rest = &rest[consumed..];
        }
        if grouped.len() < self.lexed.len() {
            self.inner = allocator.alloc_slice_fill_iter(grouped);
            self.view_limit.set(self.inner.len());
        }
        self
    }

    fn is_rewritable(&self, document: Javadoc<'_>) -> bool {
        let first =
            self.lexed.partition_point(|comment| comment.span.start < document.span().start);
        let continues = document.kind() == JavadocKind::Markdown
            && first > 0
            && self.is_next_markdown_line(self.lexed[first - 1], self.lexed[first]);
        // The compact `/**/` shares its star between delimiters, so it is kept as written.
        !continues
            && document.text() != "/**/"
            && document.has_literal_delimiters()
            && document.is_attached(self.lexed, self.declarations.starts())
    }

    /// The Markdown Javadoc run beginning at the front of `comments`, if it
    /// may be rewritten.
    fn rewritable_markdown_javadoc(&self, comments: &[Comment]) -> Option<(Javadoc<'a>, usize)> {
        Javadoc::from_comments(self.source.as_str(), comments).filter(|(document, _)| {
            document.kind() == JavadocKind::Markdown && self.is_rewritable(*document)
        })
    }

    /// Whether formatter policy may rewrite `comment` as Javadoc: it documents
    /// a declaration with literal delimiters, and a single-line traditional
    /// comment shares its line with no code, in the source or, with
    /// `own_line`, in the output. Other comments may share it, since printers
    /// can split them onto separate lines.
    pub(crate) fn should_format_javadoc(&self, comment: Comment, own_line: bool) -> bool {
        Javadoc::from_comment(self.source.as_str(), &comment).is_some_and(|document| {
            self.is_rewritable(document)
                && (document.kind() == JavadocKind::Markdown
                    || own_line
                    || comment.is_multiline_block()
                    || self.has_only_comments_on_line(comment.span))
        })
    }

    /// The names `document`'s `@param` tags may document.
    pub(crate) fn declared_parameters(
        &self,
        document: Javadoc<'_>,
    ) -> Option<DeclaredParameters<'_>> {
        self.declarations.parameters(document, self.lexed)
    }

    /// Whether the source lines `span` touches hold only whitespace and comments.
    fn has_only_comments_on_line(&self, span: Span) -> bool {
        let source = self.source.as_str();
        let line_end = line_content_end(source, span.end);
        let mut cursor = line_start(source, span.start);
        let first = self.lexed.partition_point(|comment| comment.span.end <= cursor);
        for comment in &self.lexed[first..] {
            if comment.span.start >= line_end {
                break;
            }
            if !self.source.all_characters_match(
                cursor,
                comment.span.start.max(cursor),
                is_horizontal_whitespace,
            ) {
                return false;
            }
            cursor = cursor.max(comment.span.end);
        }
        cursor >= line_end
            || self.source.all_characters_match(cursor, line_end, is_horizontal_whitespace)
    }

    /// The comments not yet printed, in source order.
    ///
    /// Every query below reads this slice without consuming it; printing a
    /// comment marks it with [`Self::increment_printed_count`].
    pub(crate) fn unprinted_comments(&self) -> &'c [Comment] {
        &self.inner[self.printed_count.get()..self.view_limit.get()]
    }

    /// The comments already printed, in source order.
    pub(crate) fn printed_comments(&self) -> &'c [Comment] {
        &self.inner[..self.printed_count.get()]
    }

    /// The span of the first comment not yet printed.
    pub(crate) fn first_unprinted_span(&self) -> Option<Span> {
        self.unprinted_comments().first().map(|comment| comment.span)
    }

    /// The last printed comment when it is a line comment starting at or after `pos`:
    /// a pending `line_suffix` the printer flushes at the next line break.
    pub(crate) fn printed_line_comment_after(&self, pos: u32) -> Option<Comment> {
        self.printed_comments()
            .last()
            .copied()
            .filter(|comment| comment.is_line() && comment.span.start >= pos)
    }

    /// The `///` comment printed last, when the next pending comment is another
    /// line of it.
    pub(crate) fn continued_markdown_line(&self) -> Option<Comment> {
        let next = self.printed_count.get();
        (next > 0
            && next < self.view_limit.get()
            && self.is_next_markdown_line(self.inner[next - 1], self.inner[next]))
        .then(|| self.inner[next - 1])
    }

    /// Whether `next` is the line after `comment` in one `///` comment.
    pub(crate) fn is_next_markdown_line(&self, comment: Comment, next: Comment) -> bool {
        Javadoc::from_comments(self.source.as_str(), &[comment, next])
            .is_some_and(|(_, consumed)| consumed == 2)
    }

    /// Whether `comment` is the last line of a `///` comment printed as written
    /// and that line is Markdown code.
    pub(crate) fn ends_markdown_code(&self, comment: Comment) -> bool {
        let end = self.inner.partition_point(|line| line.span.start <= comment.span.start);
        if end == 0 || self.inner[end - 1] != comment {
            return false;
        }
        let mut start = end - 1;
        while start > 0 && self.is_next_markdown_line(self.inner[start - 1], self.inner[start]) {
            start -= 1;
        }
        Javadoc::from_comments(self.source.as_str(), &self.inner[start..end])
            .is_some_and(|(document, _)| !self.is_rewritable(document) && document.ends_in_code())
    }

    /// Whether no pending comment leads, sits in, or trails the node at `span`,
    /// so its generated `fmt` can skip every comment query.
    #[inline]
    pub(crate) fn is_quiet(
        &self,
        span: Span,
        enclosing_span: impl Fn() -> Span,
        following_span_start: u32,
    ) -> bool {
        let quiet = self
            .unprinted_comments()
            .first()
            .is_none_or(|next| next.span.end > span.end && next.span.end > enclosing_span().end);
        debug_assert!(
            !quiet
                || (self.comments_before(span.start).is_empty()
                    && self
                        .get_trailing_comments(enclosing_span(), span, following_span_start)
                        .is_empty()),
            "a quiet node at {span:?} has comments to format"
        );
        quiet
    }

    pub(crate) fn limit_comments_up_to(&self, end: u32) -> usize {
        let previous = self.view_limit.get();
        let limit = self.inner.partition_point(|comment| comment.span.start < end);
        self.view_limit.set(limit.max(self.printed_count.get()).min(previous));
        previous
    }

    pub(crate) fn restore_view_limit(&self, previous: usize) {
        self.view_limit.set(previous);
    }

    /// Mark `comment`, the first unprinted one, as printed.
    pub(crate) fn increment_printed_count(&self, comment: &Comment) {
        debug_assert_eq!(
            self.first_unprinted_span(),
            Some(comment.span),
            "the printed comment must be the first unprinted one"
        );
        self.printed_count.set(self.printed_count.get() + 1);
    }

    /// The unprinted comments that end at or before `pos`.
    pub(crate) fn comments_before(&self, pos: u32) -> &'c [Comment] {
        let comments = self.unprinted_comments();
        &comments[..comments.iter().take_while(|comment| comment.span.end <= pos).count()]
    }

    /// Whether an unprinted comment ends at or before `start`.
    pub(crate) fn has_comment_before(&self, start: u32) -> bool {
        !self.comments_before(start).is_empty()
    }

    /// The leading run of pending comments before `pos` that start their line.
    pub(crate) fn own_line_comments_before(&self, pos: u32) -> &'c [Comment] {
        let comments = self.comments_before(pos);
        let count = comments.iter().take_while(|comment| comment.preceded_by_newline()).count();
        &comments[..count]
    }

    /// The first pending comment before `pos` when it is a line comment on the
    /// line of a `(` printed before it: one right after the `(`, or one that
    /// trailed the code before the `(` and moved after it. Otherwise empty.
    pub(crate) fn opening_paren_line_comment(&self, pos: u32) -> &'c [Comment] {
        let comments = self.comments_before(pos);
        match comments.first() {
            Some(comment) if comment.is_line() && !comment.preceded_by_newline() => &comments[..1],
            _ => &[],
        }
    }

    /// Pending comments that end at or after `pos`.
    pub(crate) fn comments_after(&self, pos: u32) -> &'c [Comment] {
        let comments = self.unprinted_comments();
        &comments[comments.partition_point(|comment| comment.span.end < pos)..]
    }

    /// Pending comments between `start` and `end`, including one ending
    /// exactly at `start`.
    pub(crate) fn comments_in_range(&self, start: u32, end: u32) -> &'c [Comment] {
        let comments = self.comments_after(start);
        let end_index = comments.iter().take_while(|comment| comment.span.end <= end).count();
        &comments[..end_index]
    }

    /// Every comment contained in `[start, end]`, printed or pending, for a
    /// layout decision that a later formatting pass must repeat.
    pub(crate) fn all_comments_in_range(
        &self,
        start: u32,
        end: u32,
    ) -> impl Iterator<Item = &'c Comment> {
        let first = self.inner.partition_point(|comment| comment.span.start < start);
        self.inner[first..].iter().take_while(move |comment| comment.span.end <= end)
    }

    /// Whether a pending comment lies within `span`.
    pub(crate) fn has_comment_in_span(&self, span: Span) -> bool {
        self.comments_before(span.end).iter().any(|comment| comment.span.end > span.start)
    }

    /// Whether a pending comment before `start` ends its line.
    pub(crate) fn has_leading_own_line_comment(&self, start: u32) -> bool {
        self.comments_before(start).iter().any(Comment::followed_by_newline)
    }

    /// The trailing comments owned by one syntax node.
    ///
    /// Before a following sibling, same-line comments stay with the preceding
    /// node and own-line comments remain available to the sibling's leading pass.
    ///
    /// `following_span_start` is the sibling's start, or `0` if none.
    /// `enclosing_span` bounds the comments returned.
    pub(crate) fn get_trailing_comments(
        &self,
        enclosing_span: Span,
        preceding_span: Span,
        following_span_start: u32,
    ) -> &'c [Comment] {
        let comments = self.unprinted_comments();
        let count = self.trailing_comment_count(
            comments,
            enclosing_span,
            preceding_span,
            following_span_start,
        );
        &comments[..count]
    }

    fn trailing_comment_count(
        &self,
        comments: &[Comment],
        enclosing_span: Span,
        preceding_span: Span,
        following_span_start: u32,
    ) -> usize {
        if comments.is_empty() {
            return 0;
        }

        debug_assert!(
            comments.first().is_none_or(|comment| { comment.span.end > preceding_span.start })
        );

        if following_span_start == 0 {
            let comments = &comments[..comments
                .iter()
                .take_while(|comment| comment.span.end <= enclosing_span.end)
                .count()];

            // When the enclosing node ends where the preceding one does, as a
            // switch case ends with its last statement's `;`, a comment run that
            // starts its line before that `;` stays pending for the parent to
            // print after it. The run may continue on that line: `/* a */ // b`.
            if enclosing_span.end == preceding_span.end {
                for (index, comment) in comments.iter().enumerate() {
                    if !comment.preceded_by_newline() {
                        continue;
                    }
                    let mut position = comment.span.end;
                    let deferrable = comments[index + 1..].iter().all(|next| {
                        let gap =
                            self.source.all_characters_match(position, next.span.start, |byte| {
                                byte.is_ascii_whitespace()
                            });
                        position = next.span.end;
                        gap
                    });
                    if !deferrable {
                        continue;
                    }
                    let gap = self.source.slice_range(position, preceding_span.end);
                    let start =
                        unicode_escape::skip_whitespace(gap, 0, |byte| byte.is_ascii_whitespace());
                    let terminator = &gap[start as usize..];
                    if unicode_escape::SourceBytes::new(terminator)
                        .next_with_escapes(|byte| byte == b';')
                        == Some((b';', terminator.len()))
                    {
                        return index;
                    }
                }
                return comments.len();
            }

            let mut start = preceding_span.end;
            for (index, comment) in comments.iter().enumerate() {
                // A comment inside the preceding node trails it too.
                if start > comment.span.start {
                    continue;
                }
                if !self.source.all_characters_match(start, comment.span.start, |byte| {
                    byte.is_ascii_whitespace() || matches!(byte, b')' | b',' | b';')
                }) {
                    return index;
                }
                start = comment.span.end;
            }
            return comments.len();
        }

        let mut comment_index = 0;
        // The following node starts at or after the enclosing end (spans are
        // half-open), so every comment inside the enclosing node is trailing.
        let following_is_outside = following_span_start >= enclosing_span.end;

        while let Some(comment) = comments.get(comment_index) {
            if comment.span.end > following_span_start || comment.span.end > enclosing_span.end {
                break;
            }

            if !following_is_outside {
                // Own-line comments lead the following node.
                if comment.preceded_by_newline() {
                    break;
                }
                // End-of-line comments always trail the preceding node.
                if comment.followed_by_newline() {
                    return comment_index + 1;
                }
            }

            comment_index += 1;
        }

        // The comments followed only by whitespace or `(` up to the following
        // node lead it.
        let mut gap_end = following_span_start;
        for (index, comment) in comments[..comment_index].iter().enumerate().rev() {
            if self.source.all_characters_match(comment.span.end, gap_end, |byte| {
                byte.is_ascii_whitespace() || byte == b'('
            }) {
                gap_end = comment.span.start;
            } else {
                return index + 1;
            }
        }
        0
    }

    /// The pending comments from `from` to `upto` up to the last
    /// documentation comment, and past the comments on its line when the run
    /// reaches a line after `from`.
    pub(crate) fn documentation_comment_run(&self, from: u32, upto: u32) -> &'c [Comment] {
        let comments = self.unprinted_comments();
        &comments[..self.documentation_run_len(comments, from, upto)]
    }

    fn documentation_run_len(&self, comments: &[Comment], from: u32, upto: u32) -> usize {
        let run = comments
            .iter()
            .take_while(|comment| comment.span.start >= from && comment.span.end <= upto)
            .count();
        let Some(last) = comments[..run].iter().rposition(Comment::is_documentation) else {
            return 0;
        };
        let mut count = last + 1;
        if self.source.contains_newline_between(from, comments[last].span.end) {
            while count < run
                && !self.source.contains_newline_between(
                    comments[count - 1].span.end,
                    comments[count].span.start,
                )
            {
                count += 1;
            }
        }
        count
    }

    /// The pending comments from `pos` on its line, up to the first that ends
    /// the line, or none when no comment does.
    pub(crate) fn end_of_line_comments_after(&self, pos: u32) -> &'c [Comment] {
        let run = self.same_line_comments_after(pos);
        if run.last().is_some_and(|comment| comment.is_line() || comment.followed_by_newline()) {
            run
        } else {
            &[]
        }
    }

    /// The pending comments from `pos` on its line, up to the first that ends
    /// the line, or else up to the code after them.
    pub(crate) fn same_line_comments_after(&self, mut pos: u32) -> &'c [Comment] {
        let comments = self.comments_after(pos);
        let mut count = 0;
        for &comment in comments {
            if !self.source.all_characters_match(pos, comment.span.start, is_end_of_line_gap_byte) {
                break;
            }
            count += 1;
            if comment.is_line() || comment.followed_by_newline() {
                break;
            }
            pos = comment.span.end;
        }
        &comments[..count]
    }

    /// The pending comments after `start` and before the first `character`
    /// outside a comment; none while a comment before `start` is pending,
    /// since that one prints first.
    pub(crate) fn comments_before_character(&self, start: u32, character: u8) -> &'c [Comment] {
        let comments = self.unprinted_comments();
        &comments[..self.count_before_character(comments, start, character)]
    }

    /// How many of `comments` come before the first `character` after `start`,
    /// skipping the text of every comment, printed ones included.
    fn count_before_character(&self, comments: &[Comment], mut start: u32, character: u8) -> usize {
        let printed = self.printed_comments();
        for comment in &printed[printed.partition_point(|comment| comment.span.start < start)..] {
            let gap = self.source.slice_range(start, comment.span.start);
            if source_text::character_end(gap, character).is_some() {
                return 0;
            }
            start = comment.span.end;
        }
        comments
            .iter()
            .take_while(|comment| {
                let before = comment.span.start >= start
                    && source_text::character_end(
                        self.source.slice_range(start, comment.span.start),
                        character,
                    )
                    .is_none();
                start = comment.span.end;
                before
            })
            .count()
    }

    /// How many pending comments follow `end` on its line when a line comment
    /// moved behind `end` cannot take them: one of them is a documentation
    /// comment, which would become part of it, or the first is a block comment,
    /// which starts the next line instead. Otherwise `0`.
    fn separated_line_after(&self, end: u32) -> usize {
        let comments = self.source.comments_on_line_after(self.unprinted_comments(), end);
        let separated = comments.first().is_some_and(Comment::is_block)
            || comments.iter().any(Comment::is_documentation);
        if separated { comments.len() } else { 0 }
    }

    /// Record that a line comment moved behind the code ending at `end`: the
    /// `;` that ends a statement or declaration, or what follows a method or
    /// constructor's name on its line.
    pub(crate) fn mark_line_comment_moved_behind(&self, end: u32, comment: &Comment) {
        self.moved_line_comment.set(Some(MovedLineComment { end, comment: *comment }));
    }

    pub(crate) fn moved_line_comment_at(&self, end: u32) -> Option<Comment> {
        self.moved_line_comment.get().filter(|it| it.end == end).map(|it| it.comment)
    }

    /// How many pending comments on the line of the code ending at `end` start
    /// a line of their own, as the line comment moved behind that code would
    /// make a documentation comment among them part of it, and a block comment
    /// after it starts the next line. Where no declaration can follow, a lone
    /// `///` line may join it.
    pub(crate) fn line_after_moved_comment(&self, end: u32, declaration_may_follow: bool) -> usize {
        let Some(moved) = self.moved_line_comment_at(end) else {
            return 0;
        };
        let count = self.separated_line_after(end);
        if count != 1 || declaration_may_follow {
            return count;
        }
        let line = self.unprinted_comments()[0];
        let next = self.inner.get(self.printed_count.get() + 1);
        let body = moved.content_span(self.source.as_str());
        // Keep a Markdown run whole and a lint directive's rule list intact.
        let joins = line.is_line()
            && (moved.is_documentation()
                || !next.is_some_and(|&next| self.is_next_markdown_line(line, next)))
            && parse_lint_disable_directive(self.source.text_for(&body).trim_start()).is_none();
        usize::from(!joins)
    }

    /// Position just past the first `character` at or after `start` outside a
    /// comment, written as itself or as a Unicode escape. The grammar
    /// guarantees `character` occurs there at every call site.
    pub(crate) fn position_after_character(&self, start: u32, character: u8) -> u32 {
        let source_end = u32::try_from(self.source.len()).unwrap();
        let first = self.inner.partition_point(|comment| comment.span.end <= start);
        let mut from = start;
        for bound in
            self.inner[first..].iter().map(|comment| comment.span).chain([Span::empty(source_end)])
        {
            // A comment ending exactly at `from` lies before the range.
            if bound.start < from {
                continue;
            }
            if let Some(end) =
                source_text::character_end(self.source.slice_range(from, bound.start), character)
            {
                return from + u32::try_from(end).unwrap();
            }
            from = bound.end;
        }
        unreachable!("the caller guarantees the character occurs outside a comment")
    }

    /// Where the first token at or after `from` starts, past whitespace and
    /// comments, including Unicode-escaped Java whitespace.
    ///
    /// The AST keeps no node for the operand after a member separator, such as
    /// `this` in `Outer.this`, and the trailing pass reads the gap up to it.
    /// Any token may be written as Unicode escapes (JLS §3.3), so its first
    /// byte does not mark where it starts.
    pub(crate) fn token_start_after(&self, from: u32) -> u32 {
        let mut position = self.skip_whitespace(from, |byte| byte.is_ascii_whitespace());
        let first = self.inner.partition_point(|comment| comment.span.start < from);
        for comment in &self.inner[first..] {
            if comment.span.start != position {
                break;
            }
            position = self.skip_whitespace(comment.span.end, |byte| byte.is_ascii_whitespace());
        }
        position
    }

    fn skip_whitespace(&self, position: u32, is_whitespace: impl Fn(u8) -> bool) -> u32 {
        unicode_escape::skip_whitespace(self.source.as_str(), position, is_whitespace)
    }
}

/// Whether `comment` stays on the line of an operator beside it: it neither
/// starts a line nor is a multi-line block ending one.
pub fn stays_on_operator_line(comment: &Comment) -> bool {
    !(comment.preceded_by_newline()
        || (comment.is_multiline_block() && comment.followed_by_newline()))
}

/// The bytes that may sit between a position and a comment still on its line
/// (`a = // c`, `case A: // c`, `x, // c`).
fn is_end_of_line_gap_byte(byte: u8) -> bool {
    is_horizontal_whitespace(byte) || matches!(byte, b'=' | b':' | b',')
}

#[cfg(test)]
mod tests {
    use robusta_ast::CommentKind;

    use super::*;

    fn starts(starts: Vec<u32>) -> JavadocDeclarations<'static> {
        JavadocDeclarations::with_starts(starts)
    }

    fn block_comment(source: &str, text: &str) -> Comment {
        let start = source.find(text).expect("test comment exists");
        let end = start + text.len();
        let kind = if text.contains('\n') {
            CommentKind::MultiLineBlock
        } else {
            CommentKind::SingleLineBlock
        };
        Comment::new(u32::try_from(start).unwrap(), u32::try_from(end).unwrap(), kind)
    }

    #[test]
    fn token_boundary_skips_escaped_whitespace_without_claiming_comments() {
        for whitespace in [
            " \t\r\n\u{c}",
            r"\u0020",
            r"\u0009",
            r"\u000a",
            r"\u000A",
            r"\u000c",
            r"\u000C",
            r"\u000d",
            r"\u000D",
            r"\uuu0020\uu0009",
        ] {
            let source = format!(r"a{whitespace}/* before */{whitespace}\u0028/* inside */)");
            let lexed = robusta_lexer::tokenize(&source);
            assert_eq!(lexed.errors.len(), 0);
            let comments = Comments::new(SourceText::new(&source), &lexed.comments);

            assert_eq!(
                comments.token_start_after(lexed.tokens[0].span.end),
                lexed.tokens[1].span.start
            );
            assert_eq!(comments.unprinted_comments().first().copied(), Some(lexed.comments[0]));
        }
    }

    #[test]
    fn position_after_character_skips_comments_and_reads_escapes() {
        for (source, end) in [
            ("a /* ) */ )", "a /* ) */ )"),
            ("a // )\n)", "a // )\n)"),
            (r"a /**/\u0029 b", r"a /**/\u0029"),
            ("a /* x */) )", "a /* x */)"),
        ] {
            let lexed = robusta_lexer::tokenize(source).comments;
            let comments = Comments::new(SourceText::new(source), &lexed);

            assert_eq!(
                comments.position_after_character(1, b')'),
                u32::try_from(end.len()).unwrap(),
                "{source:?}"
            );
        }
    }

    #[test]
    fn markdown_runs_keep_the_lexed_table_when_none_is_rewritten() {
        let allocator = Allocator::new();
        let source = "/* ordinary */";
        let lexed = [block_comment(source, source)];

        let comments =
            Comments::new(SourceText::new(source), &lexed).with_markdown_javadoc_runs(&allocator);
        assert!(std::ptr::eq(comments.unprinted_comments(), lexed.as_slice()));
    }

    #[test]
    fn markdown_runs_make_a_rewritable_run_one_comment() {
        let allocator = Allocator::new();
        let source = "/// first\n/// second\nclass A {}";
        let lexed = robusta_lexer::tokenize(source).comments;
        let comments = Comments::new(SourceText::new(source), &lexed)
            .with_declarations(starts(vec![21]))
            .with_markdown_javadoc_runs(&allocator);

        assert_eq!(comments.unprinted_comments(), [Comment { span: Span::new(0, 20), ..lexed[0] }]);
    }

    #[test]
    fn markdown_runs_join_a_run_starting_after_code_only_when_it_is_rewritten() {
        let allocator = Allocator::new();
        let source = "int x = 1; /// foo\n/// bar\nint y = 2;";
        let declarations = vec![u32::try_from(source.find("int y").unwrap()).unwrap()];
        let lexed = robusta_lexer::tokenize(source).comments;
        let cursor = |declarations| {
            Comments::new(SourceText::new(source), &lexed).with_declarations(starts(declarations))
        };

        let rewritten = cursor(declarations.clone()).with_markdown_javadoc_runs(&allocator);
        assert!(!rewritten.should_format_javadoc(lexed[1], true));
        let run = Span::new(lexed[0].span.start, lexed[1].span.end);
        assert_eq!(rewritten.unprinted_comments(), [Comment { span: run, ..lexed[0] }]);

        for separate in
            [cursor(declarations), cursor(Vec::new()).with_markdown_javadoc_runs(&allocator)]
        {
            assert_eq!(separate.unprinted_comments(), lexed.as_slice());
        }
    }

    #[test]
    fn end_of_line_comments_cross_separators_up_to_the_comment_ending_the_line() {
        fn run(source: &str) -> Vec<&str> {
            let lexed = robusta_lexer::tokenize(source).comments;
            let comments = Comments::new(SourceText::new(source), &lexed);
            comments
                .end_of_line_comments_after(1)
                .iter()
                .map(|comment| comment.span.source_text(source))
                .collect()
        }

        assert_eq!(run("a = /* b */ // c\nd"), ["/* b */", "// c"]);
        assert_eq!(run("a: // b\nc"), ["// b"]);
        assert_eq!(run("a, /* b */\nc"), ["/* b */"]);
        assert_eq!(run("a /* b */ c // d\n"), Vec::<&str>::new());
        assert_eq!(run("a; // b\n"), Vec::<&str>::new());
    }

    #[test]
    fn structural_trailing_query_keeps_same_line_end_comments_with_the_previous_node() {
        let source = "a /* owner */\nb";
        let comments = robusta_lexer::tokenize(source).comments;
        let comment = comments[0];
        let comments = Comments::new(SourceText::new(source), &comments);

        assert_eq!(
            comments.get_trailing_comments(
                Span::new(0, u32::try_from(source.len()).unwrap()),
                Span::new(0, 1),
                u32::try_from(source.find('b').unwrap()).unwrap(),
            ),
            [comment]
        );
    }

    #[test]
    fn structural_trailing_query_leaves_own_line_comments_for_the_following_node() {
        let source = "a\n/* next */\nb";
        let comments = robusta_lexer::tokenize(source).comments;
        let comments = Comments::new(SourceText::new(source), &comments);

        assert_eq!(
            comments.get_trailing_comments(
                Span::new(0, u32::try_from(source.len()).unwrap()),
                Span::new(0, 1),
                u32::try_from(source.rfind('b').unwrap()).unwrap(),
            ),
            []
        );
    }

    #[test]
    fn structural_trailing_query_claims_comments_dangling_at_the_end_of_a_container() {
        let source = "{ a\n/* dangling */\n}";
        let comment = block_comment(source, "/* dangling */");
        let comments = [comment];
        let comments = Comments::new(SourceText::new(source), &comments);

        assert_eq!(
            comments.get_trailing_comments(
                Span::new(0, u32::try_from(source.len()).unwrap()),
                Span::new(2, 3),
                0,
            ),
            [comment]
        );
    }

    #[test]
    fn comments_before_character_stop_at_the_character() {
        for pipe in ["|", r"\uu007C"] {
            let source = format!("A /* a */\n/* b */ {pipe} /* c */ B");
            let lexed = [
                block_comment(&source, "/* a */"),
                block_comment(&source, "/* b */"),
                block_comment(&source, "/* c */"),
            ];
            let comments = Comments::new(SourceText::new(&source), &lexed);

            assert_eq!(comments.comments_before_character(1, b'|'), &lexed[..2], "{source:?}");
        }
    }

    #[test]
    fn comments_before_character_wait_for_a_pending_comment_before_the_start() {
        let source = "A /* a */ B /* b */ | C";
        let lexed = [block_comment(source, "/* a */"), block_comment(source, "/* b */")];
        let comments = Comments::new(SourceText::new(source), &lexed);
        let after_b = u32::try_from(source.find(" /* b").unwrap()).unwrap();

        assert_eq!(comments.comments_before_character(after_b, b'|'), []);
        comments.increment_printed_count(&lexed[0]);
        assert_eq!(comments.comments_before_character(after_b, b'|'), &lexed[1..]);
    }

    #[test]
    fn comments_before_character_skip_the_text_of_printed_comments() {
        for (source, printed, taken) in
            [("A /* | */ /* b */ | B", "/* | */", 1), ("A | /* a */ /* b */ B", "/* a */", 0)]
        {
            let comments = [block_comment(source, printed), block_comment(source, "/* b */")];
            let comments = Comments::new(SourceText::new(source), &comments);
            comments.increment_printed_count(&comments.unprinted_comments()[0]);

            assert_eq!(comments.comments_before_character(1, b'|').len(), taken, "{source:?}");
        }
    }

    #[test]
    fn javadoc_formatting_policy_rewrites_attached_comments_without_code_on_their_line() {
        // `judged` is judged among the comments of `source`, where the last
        // `int` starts the only declaration.
        let policy = |source: &str, judged: &str, own_line| {
            let comments = robusta_lexer::tokenize(source).comments;
            let judged = *comments
                .iter()
                .find(|comment| comment.span.source_text(source) == judged)
                .expect("test comment exists");
            let declaration = source.rfind("int").map(|start| u32::try_from(start).unwrap());
            Comments::new(SourceText::new(source), &comments)
                .with_declarations(starts(declaration.into_iter().collect()))
                .should_format_javadoc(judged, own_line)
        };
        let one_line = |source, own_line| policy(source, "/** one line */", own_line);

        assert!(policy("int x; /**\n * x\n */\nint y;", "/**\n * x\n */", false));
        assert!(one_line("  /** one line */  \nint x;", false));
        assert!(!one_line("int x; /** one line */\nint y;", false));
        assert!(!one_line("  /** one line */ int x;", false));
        assert!(one_line("int x; /** one line */ int y;", true));
        assert!(one_line("/* a\n */ /** one line */ /* b */ // c\nint x;", false));
        assert!(!one_line("int x; /* a */ /** one line */\nint y;", false));
        assert!(!one_line("  /** one line */  \ndoThing();", true));
        assert!(!policy("/**\n * x\n */\ndoThing();", "/**\n * x\n */", true));
    }
}
