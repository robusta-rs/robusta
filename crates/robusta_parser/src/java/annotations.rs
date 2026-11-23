use crate::*;

impl<'a> Parser<'a> {
    /// Find the end of an annotation at `n`, balancing argument parentheses.
    pub(crate) fn annotation_end_from(&self, mut n: usize) -> Option<usize> {
        if self.nth_kind(n) != TokenKind::At || !self.nth_kind(n + 1).is_identifier() {
            return None;
        }
        n += 2;
        while self.nth_kind(n) == TokenKind::Dot && self.nth_kind(n + 1).is_identifier() {
            n += 2;
        }
        if self.nth_kind(n) == TokenKind::LParen {
            let mut depth = 0usize;
            loop {
                match self.nth_kind(n) {
                    TokenKind::LParen => depth += 1,
                    TokenKind::RParen => {
                        depth -= 1;
                        n += 1;
                        if depth == 0 {
                            break;
                        }
                        continue;
                    }
                    TokenKind::Eof => return Some(n),
                    _ => {}
                }
                n += 1;
            }
        }
        Some(n)
    }

    /// Skip annotations starting at token `n`.
    pub(crate) fn annotations_end_from(&self, mut n: usize) -> usize {
        while let Some(next) = self.annotation_end_from(n) {
            n = next;
        }
        n
    }

    /// Parse type-use annotations (JLS §4.11).
    pub(crate) fn annotations(&mut self) -> Result<Vec<'a, Annotation<'a>>> {
        let mut annotations = Vec::new_in(self);
        while self.kind() == TokenKind::At && self.nth_kind(1).is_identifier() {
            annotations.push(self.annotation()?);
        }
        Ok(annotations)
    }

    /// Parse an annotation (JLS §9.7). `@Foo()` differs from marker `@Foo`.
    pub(crate) fn annotation(&mut self) -> Result<Annotation<'a>> {
        let at = self.expect(TokenKind::At, "`@`")?.span;
        let name = self.type_name()?;
        let mut span = at.cover(name.span());
        let args = if self.eat(TokenKind::LParen).is_some() {
            let mut args = Vec::new_in(self);
            if self.kind().is_identifier() && self.nth_kind(1) == TokenKind::Assign {
                loop {
                    args.push(AnnotationArgument::Pair(self.element_value_pair()?));
                    if self.eat(TokenKind::Comma).is_none() {
                        break;
                    }
                }
            } else if self.kind() != TokenKind::RParen {
                args.push(AnnotationArgument::from(self.element_value()?));
            }
            let close = self.expect(TokenKind::RParen, "`)` after the annotation arguments")?;
            span = span.cover(close.span);
            Some(args)
        } else {
            None
        };
        Ok(Annotation::new(span, name, args, self))
    }

    fn element_value_pair(&mut self) -> Result<Box<'a, ElementValuePair<'a>>> {
        let name = self.identifier_name("an annotation element name")?;
        self.expect(TokenKind::Assign, "`=` after the annotation element name")?;
        let value = self.element_value()?;
        Ok(ElementValuePair::boxed(name.span.cover(value.span()), name, value, self))
    }

    /// Parse an element value, excluding assignments (JLS §9.7.1).
    pub(crate) fn element_value(&mut self) -> Result<ElementValue<'a>> {
        match self.kind() {
            TokenKind::At => {
                let annotation = self.annotation()?;
                Ok(ElementValue::Annotation(self.alloc(annotation)))
            }
            TokenKind::LBrace => Ok(ElementValue::Array(self.element_value_array_initializer()?)),
            _ => Ok(ElementValue::from(self.conditional_expression()?)),
        }
    }

    /// `{v, …}`, preserving an optional trailing comma.
    fn element_value_array_initializer(
        &mut self,
    ) -> Result<Box<'a, ElementValueArrayInitializer<'a>>> {
        let start = self.expect(TokenKind::LBrace, "`{` to open the element value array")?.span;
        let mut values = Vec::new_in(self);
        let mut trailing_comma = self.eat(TokenKind::Comma).is_some();
        while !trailing_comma && self.kind() != TokenKind::RBrace {
            values.push(self.element_value()?);
            if self.eat(TokenKind::Comma).is_none() {
                break;
            }
            trailing_comma = self.kind() == TokenKind::RBrace;
        }
        let end = self.expect(TokenKind::RBrace, "`}` to close the element value array")?.span;
        Ok(ElementValueArrayInitializer::boxed(start.cover(end), values, trailing_comma, self))
    }
}
