use super::*;

#[derive(Clone, Copy)]
enum ModifierContext {
    Type,
    Declaration,
    RecordComponent,
    EnumConstant,
}

impl<'a> Parser<'a> {
    /// Skip keyword modifiers and annotations starting at token `n`.
    pub(crate) fn modifier_list_end_from(&self, mut n: usize) -> usize {
        loop {
            if self.nth_kind(n).is_modifier_kind() {
                n += 1;
                continue;
            }
            match self.annotation_end_from(n) {
                Some(next) => n = next,
                None => return n,
            }
        }
    }

    /// Check for a type declaration after modifiers.
    /// Treat `sealed`/`non-sealed` as modifiers only before classes or interfaces.
    pub(crate) fn at_type_declaration_start_with_modifiers(&self) -> bool {
        let (n, saw_contextual) = self.type_modifier_list_end_from(0);
        self.is_type_declaration_start_at(n)
            && (!saw_contextual
                || matches!(self.nth_kind(n), TokenKind::Class | TokenKind::Interface))
    }

    fn type_modifier_list_end_from(&self, mut n: usize) -> (usize, bool) {
        let mut saw_contextual = false;
        loop {
            if self.nth_kind(n).is_modifier_kind() {
                n += 1;
                continue;
            }
            if self.nth_kind(n) == TokenKind::Sealed {
                saw_contextual = true;
                n += 1;
                continue;
            }
            if self.is_non_sealed_modifier_at(n) {
                saw_contextual = true;
                n += 3;
                continue;
            }
            match self.annotation_end_from(n) {
                Some(next) => n = next,
                None => return (n, saw_contextual),
            }
        }
    }

    pub(crate) fn type_modifiers(&mut self) -> Result<Vec<'a, Modifier<'a>>> {
        self.parse_modifiers(ModifierContext::Type)
    }

    pub(crate) fn modifiers(&mut self) -> Result<Vec<'a, Modifier<'a>>> {
        self.parse_modifiers(ModifierContext::Declaration)
    }

    pub(crate) fn record_component_modifiers(&mut self) -> Result<Vec<'a, Modifier<'a>>> {
        self.parse_modifiers(ModifierContext::RecordComponent)
    }

    pub(crate) fn enum_constant_modifiers(&mut self) -> Result<Vec<'a, Modifier<'a>>> {
        self.parse_modifiers(ModifierContext::EnumConstant)
    }

    fn parse_modifiers(&mut self, context: ModifierContext) -> Result<Vec<'a, Modifier<'a>>> {
        let mut modifiers = Vec::new_in(self);
        loop {
            if self.kind() == TokenKind::At
                && (matches!(context, ModifierContext::EnumConstant)
                    || self.nth_kind(1).is_identifier())
            {
                let annotation = self.annotation()?;
                modifiers.push(Modifier::Annotation(self.alloc(annotation)));
                continue;
            }
            if matches!(context, ModifierContext::RecordComponent | ModifierContext::EnumConstant) {
                break;
            }
            let (span, kind) = if matches!(context, ModifierContext::Type)
                && self.kind() == TokenKind::Sealed
            {
                (self.bump().span, ModifierKind::Sealed)
            } else if matches!(context, ModifierContext::Type) && self.is_non_sealed_modifier_at(0)
            {
                let start = self.bump().span;
                self.expect(TokenKind::Minus, "`-` in `non-sealed`")?;
                let end = self.expect(TokenKind::Sealed, "`sealed` in `non-sealed`")?;
                (start.cover(end.span), ModifierKind::NonSealed)
            } else {
                let Some(kind) = modifier_kind(self.kind()) else {
                    break;
                };
                (self.bump().span, kind)
            };
            modifiers.push(Modifier::new_keyword_modifier(span, kind, self));
        }
        Ok(modifiers)
    }

    fn is_non_sealed_modifier_at(&self, n: usize) -> bool {
        self.nth_kind(n) == TokenKind::Non
            && self.nth_kind(n + 1) == TokenKind::Minus
            && self.nth_kind(n + 2) == TokenKind::Sealed
            && self.adjacent(n)
            && self.adjacent(n + 1)
    }
}

/// Map keyword modifiers. `default` reaches here only for interface members.
fn modifier_kind(kind: TokenKind) -> Option<ModifierKind> {
    Some(match kind {
        TokenKind::Public => ModifierKind::Public,
        TokenKind::Protected => ModifierKind::Protected,
        TokenKind::Private => ModifierKind::Private,
        TokenKind::Abstract => ModifierKind::Abstract,
        TokenKind::Default => ModifierKind::Default,
        TokenKind::Static => ModifierKind::Static,
        TokenKind::Final => ModifierKind::Final,
        TokenKind::Native => ModifierKind::Native,
        TokenKind::Strictfp => ModifierKind::Strictfp,
        TokenKind::Synchronized => ModifierKind::Synchronized,
        TokenKind::Transient => ModifierKind::Transient,
        TokenKind::Volatile => ModifierKind::Volatile,
        _ => return None,
    })
}
