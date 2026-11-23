// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_parser/src/lexer/trivia_builder.rs
use robusta_ast::{Comment, CommentContent, CommentKind, CommentPosition};
use robusta_span::Span;

use super::{Token, TokenKind};

#[derive(Debug)]
pub struct TriviaBuilder {
    pub(crate) comments: Vec<Comment>,

    /// index of processed comments
    processed: usize,

    /// Saw a newline before this position (since last token).
    /// Used to determine if comments are trailing comments of the previous token.
    saw_newline: bool,

    /// Saw a newline before this position (since last comment or token).
    /// Used to set `preceded_by_newline` on comments.
    saw_newline_for_comment: bool,

    /// Previous token, used to attach trailing comments to its end. `Eof` before the first.
    previous_token: Token,
}

impl TriviaBuilder {
    pub fn new() -> Self {
        Self {
            comments: Vec::new(),
            processed: 0,
            saw_newline: true,
            saw_newline_for_comment: true,
            previous_token: Token { kind: TokenKind::Eof, span: Span::empty(0) },
        }
    }

    pub fn add_line_comment(&mut self, start: u32, end: u32, source_text: &str) {
        self.add_comment(Comment::new(start, end, CommentKind::Line), source_text);
    }

    pub fn add_block_comment(
        &mut self,
        start: u32,
        end: u32,
        kind: CommentKind,
        source_text: &str,
    ) {
        self.add_comment(Comment::new(start, end, kind), source_text);
    }

    pub fn handle_newline(&mut self) {
        // The last unprocessed comment is on a newline.
        let len = self.comments.len();
        if self.processed < len {
            let comment = &mut self.comments[len - 1];
            comment.set_followed_by_newline(true);
            if !self.saw_newline && !Self::should_stay_leading(comment) {
                self.attach_pending_comments_to_previous_token(len);
            }
        }
        self.saw_newline = true;
        self.saw_newline_for_comment = true;
    }

    #[inline]
    pub fn handle_token(&mut self, token: Token) {
        self.saw_newline = false;
        self.saw_newline_for_comment = false;
        // Cold path: attach any unprocessed comments between the previous token and this one.
        // For files with no comments (or once all comments are consumed), this branch is skipped.
        let len = self.comments.len();
        if self.processed < len {
            self.attach_pending_comments_to_token(token, len);
        }
        self.previous_token = token;
    }

    #[cold]
    fn attach_pending_comments_to_token(&mut self, token: Token, len: usize) {
        let previous = self.previous_token;
        let mut can_attach_to_previous = previous.kind != TokenKind::Eof
            && (token.kind == TokenKind::Eof
                || (Self::can_end_expression(previous.kind)
                    && Self::continues_previous_expression(token.kind)));
        for comment in &mut self.comments[self.processed..len] {
            // What follows a leading documentation comment cannot reach back past it.
            can_attach_to_previous &= !Self::should_stay_leading(comment);
            if can_attach_to_previous && !comment.preceded_by_newline() {
                comment.position = CommentPosition::Trailing;
                comment.attached_to = previous.span.end;
            } else {
                comment.position = CommentPosition::Leading;
                comment.attached_to = token.span.start;
            }
        }
        self.processed = len;
    }

    /// Attach the pending comments before the first documentation comment to
    /// the previous token. That comment and the ones after it wait for the next token.
    #[cold]
    fn attach_pending_comments_to_previous_token(&mut self, len: usize) {
        for comment in &mut self.comments[self.processed..len] {
            if Self::should_stay_leading(comment) {
                return;
            }
            comment.position = CommentPosition::Trailing;
            comment.attached_to = self.previous_token.span.end;
            self.processed += 1;
        }
    }

    /// Whether a token can form the completed left-hand side of an infix or suffix boundary.
    #[inline]
    fn can_end_expression(kind: TokenKind) -> bool {
        kind.is_identifier()
            || kind.is_literal()
            || matches!(
                kind,
                TokenKind::This
                    | TokenKind::Class
                    | TokenKind::RParen
                    | TokenKind::RBracket
                    | TokenKind::RBrace
                    | TokenKind::Gt
                    | TokenKind::PlusPlus
                    | TokenKind::MinusMinus
            )
    }

    /// Whether trivia before `kind` belongs after a completed expression rather than before the
    /// next operand. This only runs when comments are present.
    #[inline]
    fn continues_previous_expression(kind: TokenKind) -> bool {
        kind.is_binary_operator() || kind.is_assignment_operator()
    }

    /// Whether a line comment is on the previous token's line and does not
    /// immediately follow an `=`, `(` or `:` token.
    fn should_be_treated_as_trailing_comment(&self) -> bool {
        !self.saw_newline
            && !matches!(
                self.previous_token.kind,
                TokenKind::Assign | TokenKind::LParen | TokenKind::Colon
            )
    }

    /// javac gives a documentation comment to the next token, even when it
    /// shares a line with the token before it.
    fn should_stay_leading(comment: &Comment) -> bool {
        comment.is_documentation()
    }

    fn add_comment(&mut self, mut comment: Comment, source_text: &str) {
        Self::parse_content(&mut comment, source_text);

        // This newly added comment may be preceded by a newline.
        comment.set_preceded_by_newline(self.saw_newline_for_comment);
        let becomes_trailing = if comment.is_line() {
            // A line comment is always followed by a newline, or ends the file.
            comment.set_followed_by_newline(true);
            let becomes_trailing = self.should_be_treated_as_trailing_comment()
                && !Self::should_stay_leading(&comment);
            self.saw_newline = true;
            self.saw_newline_for_comment = true;
            becomes_trailing
        } else {
            // A block comment does not end with a newline; `handle_newline` sets it back to true.
            self.saw_newline_for_comment = false;
            false
        };
        self.comments.push(comment);
        if becomes_trailing {
            self.attach_pending_comments_to_previous_token(self.comments.len());
        }
    }

    /// A comment is documentation when it opens with `///`, or with `/**` but
    /// not `/**/`, judged on the translated text, so escaped delimiters count.
    /// No source level is checked: before Java 23, javac reads `///` as an
    /// ordinary comment.
    fn parse_content(comment: &mut Comment, source_text: &str) {
        let text = comment.span.source_text(source_text);
        let documentation = if comment.is_line() {
            text.starts_with("///")
        } else {
            text.starts_with("/**") && !text.starts_with("/**/")
        };
        if documentation {
            comment.content = CommentContent::Documentation;
        }
    }
}
