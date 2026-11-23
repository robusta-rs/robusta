//! Error recording and recovery

use super::*;

/// Recovery action for [`Parser::skip_balanced_until`].
enum SkipAction {
    /// Keep skipping.
    Continue,
    /// Leave the boundary token for the caller.
    StopBefore,
    /// Consume the boundary token.
    StopAfter,
}

impl Parser<'_> {
    pub(crate) fn expected_error(&self, what: &'static str) -> ParserDiagnostic {
        diagnostics::expected(what, self.cur().span)
    }

    /// Suppress duplicate diagnostics and cascades within lexer error spans.
    pub(crate) fn record_error(&mut self, diagnostic: ParserDiagnostic) {
        let span = diagnostic.primary_span();
        let diagnostic_start = span.start;
        let is_lexical_cascade = self.lexical_error_spans.iter().any(|lexical| {
            lexical.start == diagnostic_start
                || (lexical.start <= diagnostic_start && diagnostic_start < lexical.end)
        });
        let is_duplicate = self.errors.iter().any(|existing| existing.same_identity(&diagnostic));
        if !is_lexical_cascade && !is_duplicate {
            self.errors.push(diagnostic);
        }
    }

    /// Skip with balanced `()`/`[]`/`{}` counters, saturating unmatched closers.
    /// The predicate receives whether nesting is zero and controls boundary consumption.
    /// Always stop at EOF.
    fn skip_balanced_until(&mut self, mut stop: impl FnMut(&Self, bool) -> SkipAction) {
        let mut parens = 0usize;
        let mut brackets = 0usize;
        let mut braces = 0usize;
        loop {
            if self.kind() == TokenKind::Eof {
                return;
            }
            let at_outer_level = parens == 0 && brackets == 0 && braces == 0;
            match stop(self, at_outer_level) {
                SkipAction::StopBefore => return,
                SkipAction::StopAfter => {
                    self.bump();
                    return;
                }
                SkipAction::Continue => {}
            }
            match self.kind() {
                TokenKind::LParen => parens += 1,
                TokenKind::RParen => parens = parens.saturating_sub(1),
                TokenKind::LBracket => brackets += 1,
                TokenKind::RBracket => brackets = brackets.saturating_sub(1),
                TokenKind::LBrace => braces += 1,
                TokenKind::RBrace => braces = braces.saturating_sub(1),
                _ => {}
            }
            self.bump();
        }
    }

    /// Recover through `;`, preserving `}` and optionally switch labels.
    pub(crate) fn recover_body_item(&mut self, stop_at_switch_label: bool) {
        self.skip_balanced_until(|parser, at_outer_level| {
            if !at_outer_level {
                return SkipAction::Continue;
            }
            match parser.kind() {
                TokenKind::RBrace => SkipAction::StopBefore,
                TokenKind::Case | TokenKind::Default if stop_at_switch_label => {
                    SkipAction::StopBefore
                }
                TokenKind::Semicolon => SkipAction::StopAfter,
                _ => SkipAction::Continue,
            }
        });
    }

    /// Recover a top-level declaration, balancing braces to skip nested declarations.
    pub(crate) fn recover_top_level_declaration(&mut self) -> bool {
        let start = self.idx;
        let mut braces = 0usize;
        loop {
            if self.kind() == TokenKind::Eof {
                return false;
            }
            if braces == 0 && self.idx > start && self.at_type_declaration_start_with_modifiers() {
                return true;
            }
            match self.kind() {
                TokenKind::Semicolon if braces == 0 => {
                    self.bump();
                    return true;
                }
                TokenKind::LBrace => braces += 1,
                TokenKind::RBrace if braces == 1 => {
                    self.bump();
                    return true;
                }
                TokenKind::RBrace => braces = braces.saturating_sub(1),
                _ => {}
            }
            self.bump();
        }
    }

    pub(crate) fn recover_list_item(
        &mut self,
        separator: TokenKind,
        at_end: &mut impl FnMut(&Self) -> bool,
    ) {
        self.skip_balanced_until(|parser, at_outer_level| {
            if at_outer_level && (parser.kind() == separator || at_end(parser)) {
                SkipAction::StopBefore
            } else {
                SkipAction::Continue
            }
        });
    }
}
