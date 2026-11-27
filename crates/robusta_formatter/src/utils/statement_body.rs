// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_formatter/src/utils/statement_body.rs

use crate::AfterClosingBrace;
use crate::ast_nodes::AstNode;
use crate::formatter::prelude::*;
use crate::formatter::trivia::{
    FormatCommentBeforeContent, FormatLeadingComments, FormatTrailingComments,
    format_dangling_comments, should_nestle_adjacent_comments,
};
use crate::print::FormatWrite;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use robusta_ast::{
    AnnotationInterfaceBody, AnnotationInterfaceMemberDeclaration, Block, ClassBody, ClassMember,
    Comment, EnumBody, InterfaceBody, InterfaceMember, Statement,
};
use robusta_formatter_core::write;
use robusta_span::{GetSpan, Span};

/// Print `node` without its trailing pass, then its trailing comments bounded
/// at the next `character`, such as a label's `:`.
///
/// Returns whether a line comment rides a line suffix: the caller's token
/// must be followed by a line break.
#[must_use = "a riding line comment needs a line break right after the caller's token"]
pub fn write_node_with_trailing_comments_before<'a, T>(
    node: &T,
    character: u8,
    f: &mut JavaFormatter<'_, 'a>,
) -> bool
where
    T: Format<'a, JavaFormatContext<'a>> + GetSpan,
{
    let end = node.span().end;
    format_content_without_comments_after(node, end, f);
    write_trailing_comments_before(end, character, f)
}

/// [`write_node_with_trailing_comments_before`] plus the terminator and the
/// line break a riding line comment needs.
pub fn write_node_with_terminator<'a, T>(
    node: &T,
    terminator: &'static str,
    f: &mut JavaFormatter<'_, 'a>,
) where
    T: Format<'a, JavaFormatContext<'a>> + GetSpan,
{
    let needs_flush = write_node_with_trailing_comments_before(node, terminator.as_bytes()[0], f);
    terminator.fmt(f);
    if needs_flush {
        hard_line_break().fmt(f);
    }
}

/// Print a call's callee, then the comments before its `(`. When one ends its
/// line, they move inside the parentheses, where the arguments print them, so
/// the method name stays attached to its `(`.
pub fn format_callee<'a, T>(callee: &T, f: &mut JavaFormatter<'_, 'a>)
where
    T: Format<'a, JavaFormatContext<'a>> + GetSpan,
{
    let end = callee.span().end;
    format_content_without_comments_after(callee, end, f);
    if !comment_ends_line_before_opener(end, f) {
        write_comments_before_opener(end, b'(', f);
    }
}

/// Print the pending comments before an opener such as `(`, `<`, or `.` on
/// the side of the code before it, as before a `}`-to-keyword gap's keyword.
pub fn write_comments_before_opener(start: u32, opener: u8, f: &mut JavaFormatter<'_, '_>) {
    let comments = f.comments().comments_before_character(start, opener);
    write_comments_between_blocks(comments, f);
}

/// Print the pending comments on `start`'s line before the next `opener` as
/// trailing comments of the code before it, and leave those on later lines to
/// lead the code after it, so a block comment that ends its line prints as it
/// does once joined to the opener.
pub fn write_same_line_comments_before_opener(
    start: u32,
    opener: u8,
    f: &mut JavaFormatter<'_, '_>,
) {
    let comments = f.comments().comments_before_character(start, opener);
    let same_line = comments.iter().take_while(|comment| !comment.preceded_by_newline()).count();
    FormatTrailingComments::Comments(&comments[..same_line]).fmt(f);
}

/// [`write_comments_before_opener`], then `code`, which starts at the opener.
/// When a comment ends its line there, the comments and `code` are indented as
/// a wrapped line.
pub fn format_after_comments_before_opener<'a>(
    start: u32,
    opener: u8,
    code: &impl Format<'a, JavaFormatContext<'a>>,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let content = format_with(|f| {
        write_comments_before_opener(start, opener, f);
        code.fmt(f);
    });
    format_on_wrapped_line(comment_ends_line_before_opener(start, f), &content, f);
}

/// `code`, indented as a wrapped line when `wrapped`: the decision
/// [`comment_ends_line_before_opener`] made for the opener whose line `code`
/// continues.
pub fn format_on_wrapped_line<'a>(
    wrapped: bool,
    code: &impl Format<'a, JavaFormatContext<'a>>,
    f: &mut JavaFormatter<'_, 'a>,
) {
    if wrapped {
        indent(code).fmt(f);
    } else {
        code.fmt(f);
    }
}

/// Whether a comment between `start` and the opener after it ends its line: a
/// line comment, or one on a line of its own. Printed comments count too, so
/// the printer of the code after the opener's line decides as the opener did.
pub fn comment_ends_line_before_opener(start: u32, f: &JavaFormatter<'_, '_>) -> bool {
    let comments = f.comments();
    comments
        .all_comments_in_range(start, comments.token_start_after(start))
        .any(|comment| comment.is_line() || comment.preceded_by_newline())
}

/// Print the pending comments before the next `character` as trailing
/// comments, such as an empty `for` clause's before its `;`.
///
/// Returns whether a line comment rides a line suffix: the caller's token
/// must be followed by a line break.
#[must_use = "a riding line comment needs a line break right after the caller's token"]
pub fn write_trailing_comments_before(
    start: u32,
    character: u8,
    f: &mut JavaFormatter<'_, '_>,
) -> bool {
    let comments = f.comments().comments_before_character(start, character);
    write_trailing_comments(comments, false, f)
}

/// [`write_trailing_comments_before`] for a selected run of comments.
///
/// Right after an opener such as `(`, nothing precedes them to trail, so every
/// comment prints in place: same-line block comments follow it directly, as in
/// `for (/* c */;;)`, and a same-line run ending in a line comment moves to its
/// own line as a whole.
#[must_use = "a riding line comment needs a line break right after the caller's token"]
pub fn write_trailing_comments(
    comments: &[Comment],
    after_opener: bool,
    f: &mut JavaFormatter<'_, '_>,
) -> bool {
    let same_line_count =
        comments.iter().take_while(|comment| !comment.preceded_by_newline()).count();
    let (same_line, in_place) = if after_opener {
        if comments[..same_line_count].last().is_some_and(Comment::is_line) {
            hard_line_break().fmt(f);
        }
        (&[][..], comments)
    } else {
        comments.split_at(same_line_count)
    };

    FormatTrailingComments::Comments(same_line).fmt(f);
    // Own-line comments print in place, never through a line suffix, which
    // would carry them past the caller's token. The caller's token follows a
    // block comment directly (`/* c */;`).
    for (index, &comment) in in_place.iter().enumerate() {
        if !comment.preceded_by_newline() {
            // Following the opener, or the previous comment's line (`/* a */ // b`)
            if let Some(&previous) = in_place[..index].last() {
                maybe_space(!should_nestle_adjacent_comments(previous, comment, f)).fmt(f);
            }
        } else if f.lines_before(comment.span) > 1 {
            empty_line().fmt(f);
        } else {
            hard_line_break().fmt(f);
        }
        FormatCommentBeforeContent::new(comment, comment.preceded_by_newline()).fmt(f);
    }
    // The in-place loop's breaks have already flushed a riding line comment.
    in_place.is_empty() && same_line.last().is_some_and(Comment::is_line)
}

/// The separator between a construct's head and its `{`-delimited body,
/// followed by the pending head-side comments, which stay outside the braces:
/// a same-line comment follows the space, a line comment then forces the `{`
/// onto the next line via its own break, and an own-line comment keeps its
/// own line:
///
/// ```java
/// while (x) /* c */ {
///
/// while (x) // c
/// {
///
/// if (x)
/// // c
/// {
/// ```
pub fn write_head_body_separator(body_start: u32, f: &mut JavaFormatter<'_, '_>) {
    let comments = f.comments().comments_before(body_start);
    if comments.first().is_some_and(Comment::preceded_by_newline) {
        hard_line_break().fmt(f);
    } else {
        space().fmt(f);
    }
    FormatLeadingComments::Comments(comments).fmt(f);
}

/// A `{`-delimited body that [`format_head_body`] prints after its head.
pub trait Body: GetSpan {
    /// Whether the body has no entries.
    fn is_empty(&self) -> bool;

    /// Whether the body's first entry is a declaration, which a documentation
    /// comment moved after the `{` would come to document.
    fn starts_with_declaration(&self) -> bool;
}

impl Body for Block<'_> {
    fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    fn starts_with_declaration(&self) -> bool {
        self.statements.first().is_some_and(Statement::is_local_class_or_interface_declaration)
    }
}

impl Body for ClassBody<'_> {
    fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    fn starts_with_declaration(&self) -> bool {
        self.members.first().is_some_and(|member| {
            !matches!(member, ClassMember::Initializer(_) | ClassMember::Empty(_))
        })
    }
}

impl Body for InterfaceBody<'_> {
    fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    fn starts_with_declaration(&self) -> bool {
        self.members.first().is_some_and(|member| !matches!(member, InterfaceMember::Empty(_)))
    }
}

impl Body for EnumBody<'_> {
    fn is_empty(&self) -> bool {
        self.constants.is_empty() && !self.has_body_semicolon && self.members.is_empty()
    }

    fn starts_with_declaration(&self) -> bool {
        !self.constants.is_empty()
    }
}

impl Body for AnnotationInterfaceBody<'_> {
    fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    fn starts_with_declaration(&self) -> bool {
        self.members
            .first()
            .is_some_and(|member| !matches!(member, AnnotationInterfaceMemberDeclaration::Empty(_)))
    }
}

/// How many of the head-side `comments` before a `{` stay in front of it: those
/// before the first that would push the `{` to a later line, by being a line
/// comment or by starting a line.
pub fn comments_staying_before_brace(comments: &[Comment]) -> usize {
    comments
        .iter()
        .position(|comment| comment.is_line() || comment.preceded_by_newline())
        .unwrap_or(comments.len())
}

/// The head's side of [`format_head_body`]: the comments that stay before the
/// `{` at `body_start`, trailing the head, then the separator.
fn write_brace_separator(
    body_start: u32,
    starts_with_declaration: bool,
    f: &mut JavaFormatter<'_, '_>,
) {
    let comments = f.comments().comments_before(body_start);
    let (staying, moving) = comments.split_at(comments_staying_before_brace(comments));
    if starts_with_declaration && moving.iter().any(Comment::is_documentation) {
        write_head_body_separator(body_start, f);
    } else {
        FormatTrailingComments::Comments(staying).fmt(f);
        space().fmt(f);
    }
}

/// The separator between a construct's head and its `{`-delimited `body`, then
/// the body, whose `{` stays on the head's line. A comment before the `{` that
/// would push it to a later line, by being a line comment or by starting a
/// line, moves after the `{` with every comment after it, where the body
/// prints them; the comments before that one stay in front of the `{`:
///
/// ```java
/// if (full) /* by count */ { // or by weight
///
/// while (more) {
///   // drained in order
/// ```
///
/// None moves when a documentation comment would come to document the body's
/// first declaration; [`write_head_body_separator`] then places them.
pub fn format_head_body<'a, T>(body: &AstNode<'_, 'a, T>, f: &mut JavaFormatter<'_, 'a>)
where
    T: Body,
    for<'p> AstNode<'p, 'a, T>: FormatWrite<'a>,
{
    write_brace_separator(body.span().start, body.starts_with_declaration(), f);
    body.write(f);
    body.format_trailing_comments(f);
}

/// Comments between a block's `}` and a following keyword (`else`, `catch`,
/// `finally`, `while`) keep their positions: the same-line prefix trails the
/// previous block, own-line comments keep their own lines. No blank line
/// precedes them, as none would precede the keyword.
///
/// A same-line line comment rides a `line_suffix` and flushes at the emitted
/// hard break.
///
/// Returns whether the caller still has to write its default separator
/// before the keyword (`false` when the comments ended with their own).
pub fn write_comments_between_blocks(comments: &[Comment], f: &mut JavaFormatter<'_, '_>) -> bool {
    let same_line_count =
        comments.iter().take_while(|comment| !comment.preceded_by_newline()).count();
    write_split_comments_between_blocks(comments, same_line_count, f)
}

/// [`write_comments_between_blocks`], with the first `same_line_count` of
/// `comments` on the previous block's line.
fn write_split_comments_between_blocks(
    comments: &[Comment],
    same_line_count: usize,
    f: &mut JavaFormatter<'_, '_>,
) -> bool {
    let (same_line, own_line) = comments.split_at(same_line_count);

    if !same_line.is_empty() {
        FormatTrailingComments::Comments(same_line).fmt(f);
    }
    if !own_line.is_empty() {
        // Flushes a pending same-line line comment, then starts the own-line run
        hard_line_break().fmt(f);
        FormatLeadingComments::Comments(own_line).fmt(f);
        false
    } else if same_line.last().is_some_and(Comment::is_line) {
        // Only the last same-line comment can be a line comment
        // (anything after one starts a new line, landing in `own_line`)
        hard_line_break().fmt(f);
        false
    } else {
        true
    }
}

/// Print the comments between the unbraced body that ends at `body_end` and
/// the `keyword` that continues its statement, then start the keyword's line.
/// No declaration can follow the body's `;` here.
pub fn write_comments_after_unbraced_body(
    body_end: u32,
    keyword_start: u32,
    f: &mut JavaFormatter<'_, '_>,
) {
    let comments = f.comments().comments_before(keyword_start);
    // Claim earlier pending comments too so the ones after the body stay in order.
    let needs_break = if comments.last().is_none_or(|comment| comment.span.start < body_end) {
        true
    } else if f.comments().line_after_moved_comment(body_end, false) > 0 {
        write_split_comments_between_blocks(comments, 0, f)
    } else {
        write_comments_between_blocks(comments, f)
    };
    if needs_break {
        hard_line_break().fmt(f);
    }
}

pub struct FormatStatementBody<'a, 'b> {
    body: &'b AstNode<'b, 'a, Statement<'a>>,
    force_space: bool,
}

#[derive(Clone, Copy)]
pub enum StatementContinuation {
    Else,
    DoWhile(u32),
}

impl<'a, 'b> FormatStatementBody<'a, 'b> {
    pub fn new(body: &'b AstNode<'b, 'a, Statement<'a>>) -> Self {
        Self { body, force_space: false }
    }

    /// Prevents that the body is formatted on its own line and indented by one
    /// level and instead gets separated by a space.
    pub fn with_forced_space(mut self, forced: bool) -> Self {
        self.force_space = forced;
        self
    }

    pub(crate) fn format_before_continuation(
        &self,
        continuation: StatementContinuation,
        f: &mut JavaFormatter<'_, 'a>,
    ) {
        let end = self.body.span().end;
        let previous = f.context_mut().continuation_body_end.replace(end);
        match continuation {
            StatementContinuation::Else => format_content_without_comments_after(self, end, f),
            StatementContinuation::DoWhile(keyword_start) => {
                let comments = f.comments().comments_in_range(end, keyword_start);
                if comments.is_empty() {
                    self.fmt(f);
                } else {
                    format_content_without_comments_after(self, end, f);
                }
                let comments = f.comments().comments_before(keyword_start);
                let trails_with_break =
                    comments.first().is_some_and(|comment| comment.span.start >= end)
                        && comments
                            .iter()
                            .take_while(|comment| !comment.preceded_by_newline())
                            .any(|comment| comment.is_line() || comment.is_multiline_block());
                // The same suffix breaks the `do` group whether it moved or was already there.
                if trails_with_break || f.comments().moved_line_comment_at(end).is_some() {
                    expand_parent().fmt(f);
                }
            }
        }
        f.context_mut().continuation_body_end = previous;
    }
}

impl<'a> Format<'a, JavaFormatContext<'a>> for FormatStatementBody<'a, '_> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let body_start = self.body.span().start;
        match self.body.inner() {
            Statement::Block(_) => {
                format_head_body(&self.body.as_block().expect("block variant"), f);
            }
            Statement::Empty(_) => {
                // The `;` is the body, so the head-body separator applies before it
                // too; without comments no separator is written (`while (x);`).
                if f.comments().has_comment_before(body_start) {
                    write_head_body_separator(body_start, f);
                }
                self.body.fmt(f);
            }
            _ if self.force_space => write!(f, [space(), self.body]),
            _ => write!(f, [soft_line_indent_or_space(self.body)]),
        }
    }
}

/// Write the keyword that continues a statement after the block ending at
/// `previous_end`, with the comments before it.
pub fn format_continuation_keyword(
    f: &mut JavaFormatter<'_, '_>,
    keyword: &'static str,
    previous_end: u32,
) {
    // Lexical scan for the keyword's first byte: the gap holds only trivia and the keyword.
    let comments = f.comments().comments_before_character(previous_end, keyword.as_bytes()[0]);
    if write_comments_between_blocks(comments, f) {
        match f.options().after_closing_brace {
            AfterClosingBrace::SameLine => space().fmt(f),
            AfterClosingBrace::NewLine => hard_line_break().fmt(f),
        }
    }
    keyword.fmt(f);
}

/// The `{`-delimited frame every body sits in: the collapsed `{}` when there
/// is nothing to write, otherwise braces around an indented body that starts
/// and ends on its own line.
///
/// `span` ends with the body, and its `{` is the first after `span`'s start.
/// `is_empty` says the body has no entries.
/// Comments before the end of `span` still open the frame, and take a line
/// each inside it, except a line comment that stays on the `{`'s line.
///
/// The frame owns both of those line breaks, so `body` writes what it has
/// without a leading break — the same division as `JoinNodesBuilder`, which
/// separates its entries only between them.
pub fn format_braced_body<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    is_empty: bool,
    span: Span,
    body: impl FnOnce(&mut JavaFormatter<'_, 'ast>),
) {
    "{".fmt(f);
    let kept = write_line_comment_after_brace(span.start, f);
    if !is_empty {
        write!(f, [block_indent(&format_once(body)), "}"]);
    } else if kept && f.comments().comments_before(span.end).is_empty() {
        write!(f, [hard_line_break(), "}"]);
    } else {
        write!(f, [format_dangling_comments(span).with_block_indent(), "}"]);
    }
}

/// Print the first comment after the first `{` from `start`, written there or
/// moved there by [`format_head_body`], on the `{`'s line when it is a line
/// comment that does not start a line. Returns whether it did.
///
/// A `///` comment that a blank line ends stays on a line of its own, where
/// the blank line keeps it apart from a `///` comment after it.
fn write_line_comment_after_brace(start: u32, f: &mut JavaFormatter<'_, '_>) -> bool {
    let comments = f.comments();
    let first_entry = comments.token_start_after(comments.position_after_character(start, b'{'));
    let first = comments.comments_before(first_entry);
    let stays = first.first().is_some_and(|comment| {
        comment.is_line()
            && !comment.preceded_by_newline()
            && !(comment.is_documentation()
                && f.source_text().lines_after_skipping_terminators(comment.span.end, None) > 1)
    });
    if !stays {
        return false;
    }
    // Indented with the body, where a documentation comment's later lines go.
    indent(&FormatTrailingComments::Comments(&first[..1])).fmt(f);
    true
}

/// A [`format_braced_body`] attached to the header that ends at `header_end`,
/// after the comments between them.
pub fn format_attached_braced_body<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    header_end: u32,
    is_empty: bool,
    span: Span,
    body: impl FnOnce(&mut JavaFormatter<'_, 'ast>),
) {
    let body_start = f.comments().position_after_character(header_end, b'{') - 1;
    write_brace_separator(body_start, false, f);
    format_braced_body(f, is_empty, Span::new(header_end, span.end), body);
}
