//! Token cursor

use super::*;

impl<'a> Parser<'a> {
    pub(crate) fn cur(&self) -> Token {
        self.tokens[self.idx]
    }

    pub(crate) fn kind(&self) -> TokenKind {
        self.cur().kind
    }

    pub(crate) fn nth_kind(&self, n: usize) -> TokenKind {
        self.tokens.get(self.idx + n).map_or(TokenKind::Eof, |t| t.kind)
    }

    /// Whether tokens `n` and `n + 1` touch, for reconstructing right shifts.
    pub(crate) fn adjacent(&self, n: usize) -> bool {
        match (self.tokens.get(self.idx + n), self.tokens.get(self.idx + n + 1)) {
            (Some(a), Some(b)) => a.span.end == b.span.start,
            _ => false,
        }
    }

    pub(crate) fn bump(&mut self) -> Token {
        let token = self.cur();
        if token.kind != TokenKind::Eof {
            self.idx += 1;
        }
        token
    }

    pub(crate) fn eat(&mut self, kind: TokenKind) -> Option<Token> {
        (self.kind() == kind).then(|| self.bump())
    }

    #[inline]
    pub(crate) fn expect(&mut self, kind: TokenKind, what: &'static str) -> Result<Token> {
        if self.kind() == kind { Ok(self.bump()) } else { Err(self.expected_error(what)) }
    }

    /// Consume an identifier, including contextual keywords (JLS §3.9).
    #[inline]
    pub(crate) fn expect_identifier(&mut self, what: &'static str) -> Result<Token> {
        if self.kind().is_identifier() { Ok(self.bump()) } else { Err(self.expected_error(what)) }
    }

    /// Temporarily set [`Self::in_case_label`] while running `parse`.
    pub(crate) fn with_case_label<T>(
        &mut self,
        value: bool,
        parse: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let in_case_label = std::mem::replace(&mut self.in_case_label, value);
        let result = parse(self);
        self.in_case_label = in_case_label;
        result
    }

    /// Parse a required list, recovering malformed items at separators or caller-supplied boundaries.
    pub(crate) fn separated_list<T>(
        &mut self,
        separator: TokenKind,
        mut at_end: impl FnMut(&Self) -> bool,
        mut parse_item: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<'a, T>> {
        let first = self.list_item(separator, &mut at_end, &mut parse_item)?;
        if at_end(self) || self.kind() == TokenKind::Eof {
            let mut items = Vec::with_capacity_in(1, self);
            items.extend(first);
            return Ok(items);
        }
        let mut items = Vec::with_capacity_in(2, self);
        items.extend(first);
        loop {
            if self.eat(separator).is_none() {
                if !self.recovery_enabled {
                    break;
                }
                self.record_error(diagnostics::message(
                    "expected `,` between list items",
                    self.cur().span,
                ));
                self.recover_list_item(separator, &mut at_end);
                if self.eat(separator).is_none() && !at_end(self) {
                    break;
                }
            }
            items.extend(self.list_item(separator, &mut at_end, &mut parse_item)?);
            if at_end(self) || self.kind() == TokenKind::Eof {
                break;
            }
        }
        Ok(items)
    }

    /// Parse one list item, or record its error and skip to the next separator or boundary.
    fn list_item<T>(
        &mut self,
        separator: TokenKind,
        at_end: &mut impl FnMut(&Self) -> bool,
        parse_item: &mut impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Option<T>> {
        match parse_item(self) {
            Ok(item) => Ok(Some(item)),
            Err(error) if self.recovery_enabled => {
                self.record_error(error);
                self.recover_list_item(separator, at_end);
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }
}
