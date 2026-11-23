// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_ast/src/ast/comment.rs
use bitflags::bitflags;
use robusta_ast_macros::ast;
use robusta_span::Span;

/// Indicates a line or block comment.
#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentKind {
    /// Line comment
    Line = 0,
    /// Single-line block comment
    SingleLineBlock = 1,
    /// Multi-line block comment (contains line breaks)
    MultiLineBlock = 2,
}

/// Whether a comment attaches to the token after it or the token before it.
#[ast]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CommentPosition {
    /// Comments prior to a token until another token or trailing comment.
    ///
    /// e.g.
    ///
    /// ```ignore
    /// /* leading */ token;
    /// /* leading */
    /// // leading
    /// token;
    /// ```
    #[default]
    Leading = 0,

    /// Comments tailing a token until a newline.
    /// e.g. `token /* trailing */ // trailing`
    Trailing = 1,
}

/// What a comment's opening delimiter makes it.
#[ast]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CommentContent {
    /// An ordinary comment.
    #[default]
    None = 0,
    /// `/** doc */` or `/// doc`, whether or not it documents a declaration.
    Documentation = 1,
}

bitflags! {
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    /// State of newlines around a comment.
    pub struct CommentNewlines: u8 {
        /// Preceded by a newline
        const Leading = 1 << 0;
        /// Followed by a newline
        const Trailing = 1 << 1;
        /// No newlines before or after
        const None = 0;
    }
}

/// Dummy type to communicate the content of `CommentNewlines` to `robusta_ast_tools`.
#[ast(foreign = CommentNewlines)]
#[expect(dead_code)]
struct CommentNewlinesAlias(u8);

/// A source comment retained in the lexer's trivia side-table.
#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Comment {
    /// The span of the comment text, with leading and trailing delimiters.
    pub span: Span,

    /// Source boundary this comment is attached to.
    ///
    /// Leading comments use the start of the following token:
    /// ```text
    /// /* Leading */ token
    ///               ^ attached_to
    /// ```
    ///
    /// Trailing comments use the end of the preceding token:
    /// ```text
    /// token| /* Trailing */
    ///      ^ attached_to (the boundary immediately after `token`)
    /// ```
    pub attached_to: u32,

    /// Line or block comment
    pub kind: CommentKind,

    /// Leading or trailing comment
    pub position: CommentPosition,

    /// Whether this comment has newlines around it.
    pub newlines: CommentNewlines,

    /// Content of the comment
    pub content: CommentContent,
}

impl Comment {
    /// Create a line or block comment at a given location.
    #[inline]
    pub fn new(start: u32, end: u32, kind: CommentKind) -> Self {
        Self {
            span: Span::new(start, end),
            attached_to: 0,
            kind,
            position: CommentPosition::Trailing,
            newlines: CommentNewlines::None,
            content: CommentContent::None,
        }
    }

    /// The span between the comment's delimiters.
    #[expect(clippy::cast_possible_truncation)]
    pub fn content_span(&self, source: &str) -> Span {
        let text = &self.span.source_text(source)[2..];
        let text = if self.is_block() { text.strip_suffix("*/").unwrap_or(text) } else { text };
        let start = self.span.start + 2;
        Span::new(start, start + text.len() as u32)
    }

    /// Returns `true` if this is a line comment.
    #[inline]
    pub fn is_line(&self) -> bool {
        self.kind == CommentKind::Line
    }

    /// Returns `true` if this is a block comment (either single-line or multi-line).
    #[inline]
    pub fn is_block(&self) -> bool {
        matches!(self.kind, CommentKind::SingleLineBlock | CommentKind::MultiLineBlock)
    }

    /// Returns `true` if this is a multi-line block comment.
    #[inline]
    pub fn is_multiline_block(&self) -> bool {
        self.kind == CommentKind::MultiLineBlock
    }

    /// Whether this comment is written in a documentation-comment form.
    #[inline]
    pub fn is_documentation(&self) -> bool {
        self.content == CommentContent::Documentation
    }

    /// Returns `true` if this comment is preceded by a newline.
    #[inline]
    pub fn preceded_by_newline(&self) -> bool {
        self.newlines.contains(CommentNewlines::Leading)
    }

    /// Returns `true` if this comment is followed by a newline.
    #[inline]
    pub fn followed_by_newline(&self) -> bool {
        self.newlines.contains(CommentNewlines::Trailing)
    }

    /// Sets the state of `newlines` to include/exclude a newline before the comment.
    #[inline]
    pub fn set_preceded_by_newline(&mut self, preceded_by_newline: bool) {
        self.newlines.set(CommentNewlines::Leading, preceded_by_newline);
    }

    /// Sets the state of `newlines` to include/exclude a newline after the comment.
    #[inline]
    pub fn set_followed_by_newline(&mut self, followed_by_newline: bool) {
        self.newlines.set(CommentNewlines::Trailing, followed_by_newline);
    }
}

/// Recognizes a lint disable directive at the start of a trimmed comment body.
/// Returns whether it targets the next line and the text after the directive.
pub fn parse_lint_disable_directive(content: &str) -> Option<(bool, &str)> {
    [("robusta-disable-next-line", true), ("robusta-disable-line", false)].into_iter().find_map(
        |(prefix, next_line)| {
            let rest = content.strip_prefix(prefix)?;
            (rest.is_empty() || rest.starts_with(char::is_whitespace)).then_some((next_line, rest))
        },
    )
}
