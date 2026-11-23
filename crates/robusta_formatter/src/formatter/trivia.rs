// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_formatter/src/formatter/trivia.rs

use crate::formatter::prelude::*;
use crate::{CommentLineStrategy, JavaLabels};
use robusta_ast::Comment;
use robusta_formatter_core::format_element::{LINE_TERMINATORS, normalize_newlines};
use robusta_formatter_core::{FormatElement, LabelId, Tag, arena_cow_str, write};

use robusta_span::Span;
use robusta_syntax::line_terminator::{contains_line_terminator, line_start, split_lines};

use crate::formatter::javadoc::{format_javadoc_comment, javadoc_options};

/// Formats the leading comments of the node at `span`.
pub const fn format_leading_comments<'c>(span: Span) -> FormatLeadingComments<'c> {
    FormatLeadingComments::Node(span)
}

/// Formats leading comments.
#[derive(Clone, Copy, Debug)]
pub enum FormatLeadingComments<'c> {
    Node(Span),
    /// A run a printer selected that is not a node's leading pass, so no
    /// separator printed elsewhere sits between it and what follows.
    Comments(&'c [Comment]),
}

/// Leading comments hoisted before a leading operator, when one of them ends its line.
pub const fn format_hoisted_leading_comments(span: Span) -> FormatHoistedLeadingComments {
    FormatHoistedLeadingComments(span)
}

pub struct FormatHoistedLeadingComments(Span);

impl<'a> Format<'a, crate::JavaFormatContext<'a>> for FormatHoistedLeadingComments {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if f.comments().has_leading_own_line_comment(self.0.start) {
            FormatLeadingComments::Node(self.0).fmt(f);
        }
    }
}

pub fn should_nestle_adjacent_comments(
    current: Comment,
    next: Comment,
    f: &JavaFormatter<'_, '_>,
) -> bool {
    if current.span.end != next.span.start
        || !is_alignable_block_comment(current, f)
        || !is_alignable_block_comment(next, f)
    {
        return false;
    }
    // A Javadoc that can become single-line would lose its adjacency on the next pass.
    let can_collapse = |comment| {
        javadoc_options(comment, false, f)
            .is_some_and(|options| options.comment_line_strategy == CommentLineStrategy::SingleLine)
    };
    !can_collapse(current) && !can_collapse(next)
}

pub fn is_alignable_block_comment(comment: Comment, f: &JavaFormatter<'_, '_>) -> bool {
    comment.is_multiline_block() && is_alignable_comment(f.source_text().text_for(&comment.span))
}

/// Whether a block comment has later physical lines and each starts with `*`.
/// An escaped line break starts no physical line.
fn is_alignable_comment(text: &str) -> bool {
    let mut lines = split_lines(text).skip(1).peekable();
    lines.peek().is_some()
        && lines.all(|line| line.trim_start_matches([' ', '\t']).starts_with('*'))
}

impl<'a> Format<'a, crate::JavaFormatContext<'a>> for FormatLeadingComments<'_> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        fn format_leading_comments_impl(
            comments: &[Comment],
            // The following node's start, which bounds each comment's line
            // count (see `lines_after_skipping_terminators`).
            following_start: Option<u32>,
            f: &mut JavaFormatter<'_, '_>,
        ) {
            // A run with no following node documents no declaration, so it has no
            // own-line Javadoc decision.
            let following_line_start =
                following_start.map_or(0, |start| following_line_start(start, f));
            // Whether code comes before the comment on its line, not only comments.
            let mut after_code = false;
            for (index, comment) in comments.iter().copied().enumerate() {
                f.comments().increment_printed_count(&comment);
                let previous = index.checked_sub(1).map(|previous| comments[previous]);
                after_code = f.lines_before(comment.span) == 0
                    && (after_code
                        || previous.is_none_or(|previous| {
                            f.comments().token_start_after(previous.span.end) < comment.span.start
                        }));
                let next = comments.get(index + 1).copied();
                let nestle =
                    next.is_some_and(|next| should_nestle_adjacent_comments(comment, next, f));
                let line_count = f
                    .source_text()
                    .lines_after_skipping_terminators(comment.span.end, following_start);
                // A comment ending its line may start one when the caller's group breaks.
                let own_line = comment.span.end <= following_line_start;
                format_comment_text(comment, own_line).fmt(f);
                match line_count {
                    0 if comment.is_block() => maybe_space(!nestle).fmt(f),
                    // A comment ending a line it shares with code may join what follows.
                    1 if comment.is_block() && after_code => soft_line_break_or_space().fmt(f),
                    0 | 1 => format_comment_line_end(comment, next, f),
                    _ => empty_line().fmt(f),
                }
            }
        }

        let (comments, following_start) = match *self {
            Self::Node(span) => (f.comments().comments_before(span.start), Some(span.start)),
            Self::Comments(comments) => (comments, None),
        };
        if comments.is_empty() {
            return;
        }
        format_leading_comments_impl(comments, following_start, f);
    }
}

/// Formats the trailing comments of the node at `preceding_span`.
pub const fn format_trailing_comments<'c>(
    enclosing_span: Span,
    preceding_span: Span,
    following_span_start: u32,
) -> FormatTrailingComments<'c> {
    FormatTrailingComments::Node((enclosing_span, preceding_span, following_span_start))
}

/// Formats trailing comments.
#[derive(Clone, Copy, Debug)]
pub enum FormatTrailingComments<'c> {
    /// `(enclosing_span, preceding_span, following_span_start)`
    Node((Span, Span, u32)),
    /// Comments a printer selected.
    Comments(&'c [Comment]),
    /// The same-line comments a `return`, `throw`, or `yield` moved behind its
    /// `;`, printed as the statement's dangling comments: a line comment among
    /// them ends the line without expanding the group around the statement.
    Dangling(&'c [Comment]),
}

impl<'a> Format<'a, crate::JavaFormatContext<'a>> for FormatTrailingComments<'_> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        match *self {
            Self::Node((enclosing_span, preceding_span, following_span_start)) => {
                let comments = f.comments().get_trailing_comments(
                    enclosing_span,
                    preceding_span,
                    following_span_start,
                );
                format_trailing_comments_impl(comments, TrailingRun::Trailing, f);
            }
            Self::Comments(comments) => {
                format_trailing_comments_impl(comments, TrailingRun::Trailing, f);
            }
            Self::Dangling(comments) => {
                format_trailing_comments_impl(comments, TrailingRun::Dangling, f);
            }
        }
    }
}

/// Formats the dangling comments of the node at `span`.
pub const fn format_dangling_comments(span: Span) -> FormatDanglingComments {
    FormatDanglingComments::Node { span, indent: DanglingIndentMode::None }
}

/// Formats dangling comments, one per line except adjacent aligned block
/// comments.
#[derive(Clone, Copy, Debug)]
pub enum FormatDanglingComments {
    Node { span: Span, indent: DanglingIndentMode },
}

/// How an empty container indents its dangling comments.
#[derive(Clone, Copy, Debug)]
pub enum DanglingIndentMode {
    /// Every comment on its own line, in a block indent, as in an empty body.
    Block,
    /// A soft block indent, so a lone block comment can stay inline, as in
    /// `f(/* c */)`; a line comment ends with a line break.
    Soft,
    /// No indent.
    None,
}

impl FormatDanglingComments {
    /// Indents the comments with a [block](DanglingIndentMode::Block) indent.
    #[must_use]
    pub const fn with_block_indent(self) -> Self {
        self.with_indent_mode(DanglingIndentMode::Block)
    }

    /// Indents the comments with a [soft block](DanglingIndentMode::Soft) indent.
    #[must_use]
    pub const fn with_soft_block_indent(self) -> Self {
        self.with_indent_mode(DanglingIndentMode::Soft)
    }

    const fn with_indent_mode(self, indent: DanglingIndentMode) -> Self {
        let Self::Node { span, .. } = self;
        Self::Node { span, indent }
    }
}

impl<'a> Format<'a, crate::JavaFormatContext<'a>> for FormatDanglingComments {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let Self::Node { span, indent } = *self;
        let comments = f.comments().comments_before(span.end);
        if comments.is_empty() {
            return;
        }
        let own_line = matches!(indent, DanglingIndentMode::Block);
        let format_dangling_comments = format_once(|f| {
            for (index, &comment) in comments.iter().enumerate() {
                f.comments().increment_printed_count(&comment);
                let rest = &comments[index + 1..];
                let nestle = rest
                    .first()
                    .is_some_and(|&next| should_nestle_adjacent_comments(comment, next, f));
                let ends_line = comment.is_line() || (!rest.is_empty() && !nestle);
                if index == 0
                    && f.lines_before(comment.span) > 0
                    && lines_before_output(comment.span.end, rest, span.end, f) > 0
                {
                    expand_parent().fmt(f);
                }
                format_run_comment(comment, rest.first().copied(), own_line, ends_line, f);
            }
        });
        match indent {
            DanglingIndentMode::Block => block_indent(&format_dangling_comments).fmt(f),
            DanglingIndentMode::Soft => group(&soft_block_indent(&format_dangling_comments)).fmt(f),
            DanglingIndentMode::None => format_dangling_comments.fmt(f),
        }
    }
}

/// Where a run of trailing comments sits.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TrailingRun {
    Trailing,
    /// A line comment already rides the line the run trails.
    AfterLineComment,
    /// See [`FormatTrailingComments::Dangling`].
    Dangling,
}

/// Formats trailing `comments`.
fn format_trailing_comments_impl(
    comments: &[Comment],
    run: TrailingRun,
    f: &mut JavaFormatter<'_, '_>,
) {
    let after_line_comment = run == TrailingRun::AfterLineComment;
    let mut total_lines_before = usize::from(after_line_comment);
    let mut previous_is_line = after_line_comment;
    let mut previous_comment: Option<Comment> = None;
    for &comment in comments {
        f.comments().increment_printed_count(&comment);
        let line_count = f.lines_before(comment.span);
        total_lines_before += line_count;
        let nestle = previous_comment
            .is_some_and(|previous| should_nestle_adjacent_comments(previous, comment, f));

        // A comment that already sits on a later line, or follows a line
        // comment, has no place left on the current output line and must ride
        // a line suffix to the next break.
        if total_lines_before > 0 || previous_is_line {
            write!(
                f,
                [line_suffix(&format_once(|f| {
                    match line_count {
                        _ if nestle => {}
                        // Sharing a line with the comment before it. A block
                        // comment can sit beside it; a line comment ended the
                        // line and forces a break.
                        0 if !previous_is_line => space().fmt(f),
                        0 | 1 => hard_line_break().fmt(f),
                        _ => empty_line().fmt(f),
                    }
                    format_comment_text(comment, true).fmt(f);
                }))]
            );
        } else {
            // Still on the line it trails. A line comment must be deferred or
            // it would swallow whatever the caller writes next; a block comment
            // has a real place right here, and deferring it would carry it past
            // that syntax — a `}` before an `else` being the case that shows it.
            let content = format_once(|f| {
                maybe_space(!nestle).fmt(f);
                format_comment_text(comment, false).fmt(f);
            });
            if comment.is_line() {
                line_suffix(&content).fmt(f);
                if run != TrailingRun::Dangling {
                    expand_parent().fmt(f);
                }
            } else {
                content.fmt(f);
            }
        }
        previous_comment = Some(comment);
        previous_is_line = comment.is_line();
    }
}

/// Format comments before a token, preserving their source gaps.
fn format_comment_slice_before_token(comments: &[Comment], f: &mut JavaFormatter<'_, '_>) {
    let mut line_already_written = false;
    for (index, &comment) in comments.iter().enumerate() {
        f.comments().increment_printed_count(&comment);
        let lines = f.lines_before(comment.span);
        if !line_already_written {
            if lines > 0 {
                hard_line_break().fmt(f);
            } else {
                let nestle =
                    index > 0 && should_nestle_adjacent_comments(comments[index - 1], comment, f);
                maybe_space(!nestle).fmt(f);
            }
        }
        let next = comments.get(index + 1).copied();
        if comment.is_line() && lines == 0 && !line_already_written {
            line_suffix(&format_once(|f| {
                format_comment_text(comment, false).fmt(f);
                format_comment_trailing_whitespace(comment, next, f);
            }))
            .fmt(f);
            hard_line_break().fmt(f);
        } else {
            format_run_comment(comment, next, false, comment.is_line(), f);
        }
        line_already_written = comment.is_line();
    }
}

/// Print the comments between `from` and punctuation ending before `upto`
/// in place, up to the last documentation comment. Behind a `;`, `,`, `:`,
/// `<`, or `(`, the comment would document a declaration that follows. A `///`
/// comment ends its line, so the punctuation starts the next whenever the last
/// comment printed is one, even one the code before `from` printed, with or
/// without a space before it. Returns where the printed comments end.
pub fn format_documentation_comments_before(
    from: u32,
    upto: u32,
    f: &mut JavaFormatter<'_, '_>,
) -> u32 {
    let comments = f.comments().documentation_comment_run(from, upto);
    format_comment_slice_before_token(comments, f);
    if f.comments().printed_comments().last().is_some_and(|comment| {
        comment.is_line() && comment.is_documentation() && comment.span.end <= upto
    }) {
        line_suffix_boundary().fmt(f);
    }
    comments.last().map_or(from, |comment| comment.span.end)
}

/// After a line comment moved behind the semicolon ending at `node_end`, a
/// documentation comment on the semicolon's line would become part of it, so
/// the run starts on a new line. Returns where the printed comments end.
pub fn format_documentation_line_after_terminator(
    node_end: u32,
    f: &mut JavaFormatter<'_, '_>,
) -> u32 {
    let count = f.comments().documentation_line_after(node_end);
    let comments = &f.comments().unprinted_comments()[..count];
    format_trailing_comments_impl(comments, TrailingRun::AfterLineComment, f);
    comments.last().map_or(node_end, |comment| comment.span.end)
}

/// The line breaks between `from` and the output that follows it: the
/// first of `comments`, or `to` when there are none, across a separator
/// before `to`.
pub fn lines_before_output(
    from: u32,
    comments: &[Comment],
    to: u32,
    f: &JavaFormatter<'_, '_>,
) -> usize {
    match comments.first() {
        Some(&comment) => f.source_text().get_lines_before(comment.span, None),
        None => f.source_text().lines_after_skipping_terminators(from, Some(to)),
    }
}

/// The start of `following_start`'s line, before which a comment ends a line
/// of its own, or zero when no Javadoc policy asks.
fn following_line_start(following_start: u32, f: &JavaFormatter<'_, '_>) -> u32 {
    if f.options().javadoc.is_some() {
        line_start(f.source_text().as_str(), following_start)
    } else {
        0
    }
}

/// Format `comment` from a run, ending its line when `ends_line` holds.
fn format_run_comment(
    comment: Comment,
    next: Option<Comment>,
    own_line: bool,
    ends_line: bool,
    f: &mut JavaFormatter<'_, '_>,
) {
    format_comment_text(comment, own_line).fmt(f);
    if ends_line {
        format_comment_line_end(comment, next, f);
    }
}

/// End `comment`'s line, with `next` printed on the line after it.
fn format_comment_line_end(comment: Comment, next: Option<Comment>, f: &mut JavaFormatter<'_, '_>) {
    format_comment_trailing_whitespace(comment, next, f);
    hard_line_break().fmt(f);
}

/// Trailing spaces make a Markdown hard break when another line of the `///`
/// comment follows, and are code text on its last line when that is code.
fn format_comment_trailing_whitespace(
    comment: Comment,
    next: Option<Comment>,
    f: &mut JavaFormatter<'_, '_>,
) {
    let whitespace = trailing_whitespace(f.source_text().text_for(&comment.span));
    let comments = f.comments();
    if !whitespace.is_empty()
        && (next.is_some_and(|next| comments.is_next_markdown_line(comment, next))
            || comments.ends_markdown_code(comment))
    {
        text(whitespace).fmt(f);
    }
}

pub fn trailing_whitespace(text: &str) -> &str {
    &text[text.trim_end_matches([' ', '\t']).len()..]
}

struct FormatCommentText {
    comment: Comment,
    own_line: bool,
}

/// Writes `comment` and leaves its line open; `own_line` says no code shares
/// its output line.
const fn format_comment_text(comment: Comment, own_line: bool) -> FormatCommentText {
    FormatCommentText { comment, own_line }
}

impl<'a> Format<'a, crate::JavaFormatContext<'a>> for FormatCommentText {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let comment = self.comment;
        if comment.is_block() {
            // A pending line comment ends its line before a block comment starts.
            line_suffix_boundary().fmt(f);
        }
        f.write_element(FormatElement::Tag(Tag::StartLabelled(LabelId::of(JavaLabels::Comment))));
        if let Some(options) = javadoc_options(comment, self.own_line, f) {
            format_javadoc_comment(comment, options, f);
        } else {
            let content = f.source_text().text_for(&comment.span);
            if comment.is_multiline_block() {
                // Line up the stars under the first one, wherever the comment
                // starts; any other comment's later lines stay as written.
                if is_alignable_comment(content) {
                    let lines = format_once(|f| {
                        for (index, line) in split_lines(content).enumerate() {
                            if index == 0 {
                                text(line.trim_end_matches([' ', '\t'])).fmt(f);
                            } else {
                                write!(
                                    f,
                                    [
                                        hard_line_break(),
                                        text(" "),
                                        text(line.trim_matches([' ', '\t']))
                                    ]
                                );
                            }
                        }
                    });
                    align_to_column(&lines).fmt(f);
                } else {
                    let normalized = normalize_newlines(content, LINE_TERMINATORS);
                    text(arena_cow_str(&normalized, f)).fmt(f);
                    // Line breaks written as `\u000a` escapes break the group
                    // as written ones do.
                    if !contains_line_terminator(content) {
                        expand_parent().fmt(f);
                    }
                }
            } else {
                text(content.trim_end_matches([' ', '\t'])).fmt(f);
            }
        }
        f.write_element(FormatElement::Tag(Tag::EndLabelled));
    }
}

/// A comment a printer selected, marked printed and written in place: a line
/// comment ends its line. `own_line` says no code shares its output line.
#[must_use = "formatted comments must be written to the formatter"]
#[derive(Clone, Copy, Debug)]
pub struct FormatCommentBeforeContent {
    comment: Comment,
    own_line: bool,
}

impl FormatCommentBeforeContent {
    pub const fn new(comment: Comment, own_line: bool) -> Self {
        Self { comment, own_line }
    }
}

impl<'a> Format<'a, crate::JavaFormatContext<'a>> for FormatCommentBeforeContent {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        f.comments().increment_printed_count(&self.comment);
        format_comment_text(self.comment, self.own_line).fmt(f);
        if self.comment.is_line() {
            hard_line_break().fmt(f);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{JavadocOptions, LineEnding, Options, format};

    #[test]
    fn javadoc_rendering_normalizes_margins() {
        let source = "class A {\n/**\n*First.\n *\n *     more\nno star\n */\nint x;\n}\n";
        let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
        assert_eq!(
            format(source, &options).unwrap(),
            "class A {\n  /**\n   * First.\n   *\n   * more no star\n   */\n  int x;\n}\n"
        );
    }

    #[test]
    fn block_comment_rendering_normalizes_crlf_lines() {
        let source = "class A {\r\n/* a\r\n * b\r\n */\r\nint x;\r\n}\r\n";
        let options = Options { line_ending: LineEnding::Lf, ..Options::default() };
        assert_eq!(
            format(source, &options).unwrap(),
            "class A {\n  /* a\n   * b\n   */\n  int x;\n}\n"
        );
    }
}
