// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_parser/src/lexer/comment.rs
use memchr::{memchr2, memchr3, memmem::Finder};
use robusta_ast::CommentKind;

use super::{Lexer, diagnostics, offset32, span32};

impl Lexer<'_> {
    /// JLS §3.7 end-of-line comment. Its line terminator stays for [`Self::skip_trivia`].
    pub(super) fn skip_single_line_comment(&mut self) {
        let start = self.pos;
        self.pos += 2;
        let remaining = &self.src[self.pos..];
        self.pos += memchr2(b'\n', b'\r', remaining).unwrap_or(remaining.len());
        self.trivia_builder.add_line_comment(offset32(start), offset32(self.pos), self.text);
    }

    /// JLS §3.7 traditional comment.
    pub(super) fn skip_multi_line_comment(&mut self) {
        // We need to identify if comment contains line breaks or not.
        // If the loop finds a line break before end of the comment, we then switch to
        // the faster `skip_multi_line_comment_after_line_break` which searches
        // for the end of the comment only.
        let start = self.pos;
        self.pos += 2;
        while let Some(index) = memchr3(b'*', b'\n', b'\r', &self.src[self.pos..]) {
            self.pos += index + 1;
            if self.src[self.pos - 1] != b'*' {
                self.skip_multi_line_comment_after_line_break(start);
                return;
            }
            if self.peek() == Some(b'/') {
                self.pos += 1;
                self.add_block_comment(start, CommentKind::SingleLineBlock);
                return;
            }
        }
        self.pos = self.src.len();
        self.errors.push(diagnostics::unterminated_block_comment(span32(start, self.pos)));
        self.add_block_comment(start, CommentKind::SingleLineBlock);
    }

    fn skip_multi_line_comment_after_line_break(&mut self, start: usize) {
        let finder = self.multi_line_comment_end_finder.get_or_insert_with(|| Finder::new(b"*/"));
        if let Some(index) = finder.find(&self.src[self.pos..]) {
            self.pos += index + 2;
        } else {
            self.pos = self.src.len();
            self.errors.push(diagnostics::unterminated_block_comment(span32(start, self.pos)));
        }
        self.add_block_comment(start, CommentKind::MultiLineBlock);
    }

    fn add_block_comment(&mut self, start: usize, kind: CommentKind) {
        self.trivia_builder.add_block_comment(offset32(start), offset32(self.pos), kind, self.text);
    }
}
