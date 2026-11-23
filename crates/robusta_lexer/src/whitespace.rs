// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_parser/src/lexer/whitespace.rs
use robusta_syntax::{identifier::is_horizontal_whitespace, line_terminator::is_line_terminator};

use super::Lexer;

impl Lexer<'_> {
    pub(super) fn line_break_handler(&mut self) {
        self.trivia_builder.handle_newline();

        // Indentation is common after a line break.
        // Consume it, along with any further line breaks.
        let remaining = &self.src[self.pos..];
        self.pos += remaining
            .iter()
            .position(|&byte| !is_horizontal_whitespace(byte) && !is_line_terminator(byte))
            .unwrap_or(remaining.len());
    }
}
