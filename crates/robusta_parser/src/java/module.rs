use crate::*;

impl<'a> Parser<'a> {
    pub(crate) fn package_declaration(&mut self) -> Result<PackageDeclaration<'a>> {
        let annotations = self.annotations()?;
        let keyword = self.expect(TokenKind::Package, "`package`")?.span;
        let start = annotations.first().map_or(keyword, |a| a.span);
        let name = self.name()?;
        let end = self.expect(TokenKind::Semicolon, "`;` after the package name")?.span;
        Ok(PackageDeclaration::new(start.cover(end), annotations, name, self))
    }

    pub(crate) fn import_declaration(&mut self) -> Result<ImportDeclaration<'a>> {
        let start = self.expect(TokenKind::Import, "`import`")?.span;
        if self.kind() == TokenKind::Module && self.nth_kind(1).is_identifier() {
            self.bump();
            let name = self.name()?;
            let end = self.expect(TokenKind::Semicolon, "`;` after the import")?.span;
            return Ok(ImportDeclaration::new(
                start.cover(end),
                ImportKind::SingleModule,
                name,
                None,
                self,
            ));
        }

        let is_static = self.eat(TokenKind::Static).is_some();
        let name = self.name()?;
        let is_on_demand = if self.eat(TokenKind::Dot).is_some() {
            self.expect(TokenKind::Star, "`*` in an on-demand import")?;
            true
        } else {
            false
        };
        let end = self.expect(TokenKind::Semicolon, "`;` after the import")?.span;
        let kind = match (is_static, is_on_demand) {
            (false, false) => ImportKind::SingleType,
            (false, true) => ImportKind::TypeOnDemand,
            (true, false) => ImportKind::SingleStatic,
            (true, true) => ImportKind::StaticOnDemand,
        };
        let local = matches!(kind, ImportKind::SingleType | ImportKind::SingleStatic).then(|| {
            let last = name.last();
            self.binding(last.span, last.name)
        });
        Ok(ImportDeclaration::new(start.cover(end), kind, name, local, self))
    }

    /// Look ahead for `{Annotation} [open] module ModuleName {`.
    pub(crate) fn at_module_declaration(&self) -> bool {
        let mut n = self.annotations_end_from(0);
        if self.nth_kind(n) == TokenKind::Open {
            n += 1;
        }
        if self.nth_kind(n) != TokenKind::Module {
            return false;
        }
        let Some(after_name) = self.qualified_name_end_from(n + 1) else {
            return false;
        };
        self.nth_kind(after_name) == TokenKind::LBrace
    }

    pub(crate) fn qualified_name_end_from(&self, mut n: usize) -> Option<usize> {
        if !self.nth_kind(n).is_identifier() {
            return None;
        }
        n += 1;
        while self.nth_kind(n) == TokenKind::Dot && self.nth_kind(n + 1).is_identifier() {
            n += 2;
        }
        Some(n)
    }

    pub(crate) fn module_declaration(&mut self) -> Result<ModuleDeclaration<'a>> {
        let annotations = self.annotations()?;
        let open = if self.kind() == TokenKind::Open { Some(self.bump().span) } else { None };
        if self.kind() != TokenKind::Module {
            return Err(diagnostics::message("expected `module`", self.cur().span));
        }
        let module_span = self.bump().span;
        let start = annotations.first().map(|a| a.span).or(open).unwrap_or(module_span);
        let name = self.name()?;
        self.expect(TokenKind::LBrace, "`{` to open the module body")?;
        let mut directives = Vec::new_in(self);
        while !matches!(self.kind(), TokenKind::RBrace | TokenKind::Eof) {
            match self.module_directive() {
                Ok(directive) => directives.push(directive),
                Err(error) if self.recovery_enabled => {
                    self.record_error(error);
                    self.recover_body_item(false);
                }
                Err(error) => return Err(error),
            }
        }
        let close = self.expect(TokenKind::RBrace, "`}` to close the module body")?.span;
        Ok(ModuleDeclaration::new(
            start.cover(close),
            annotations,
            open.is_some(),
            name,
            directives,
            self,
        ))
    }

    fn module_directive(&mut self) -> Result<ModuleDirective<'a>> {
        // Directive keywords remain ordinary names elsewhere.
        if self.kind() == TokenKind::Requires {
            let start = self.bump().span;
            let mut modifiers = Vec::new_in(self);
            loop {
                // `static` is always a modifier here.
                if let Some(tok) = self.eat(TokenKind::Static) {
                    modifiers.push(RequiresModifier::new(
                        tok.span,
                        RequiresModifierKind::Static,
                        self,
                    ));
                    continue;
                }
                // `requires transitive;` names a module (JLS §7.7.1).
                if self.kind() == TokenKind::Transitive
                    && (self.nth_kind(1).is_identifier() || self.nth_kind(1) == TokenKind::Static)
                {
                    let tok = self.bump();
                    modifiers.push(RequiresModifier::new(
                        tok.span,
                        RequiresModifierKind::Transitive,
                        self,
                    ));
                    continue;
                }
                break;
            }
            let name = self.name()?;
            let end = self.expect(TokenKind::Semicolon, "`;` after a `requires` directive")?.span;
            Ok(ModuleDirective::new_requires_directive(start.cover(end), modifiers, name, self))
        } else if self.kind() == TokenKind::Exports || self.kind() == TokenKind::Opens {
            let kind = if self.kind() == TokenKind::Exports {
                PackageDirectiveKind::Exports
            } else {
                PackageDirectiveKind::Opens
            };
            let start = self.bump().span;
            let name = self.name()?;
            let mut targets = Vec::new_in(self);
            if self.kind() == TokenKind::To {
                self.bump();
                targets = self.separated_list(
                    TokenKind::Comma,
                    |parser| parser.kind() == TokenKind::Semicolon,
                    Self::name,
                )?;
            }
            let end = self.expect(TokenKind::Semicolon, "`;` after the directive")?.span;
            Ok(ModuleDirective::new_package_directive(start.cover(end), kind, name, targets, self))
        } else if self.kind() == TokenKind::Uses {
            let start = self.bump().span;
            let name = self.type_name()?;
            let end = self.expect(TokenKind::Semicolon, "`;` after a `uses` directive")?.span;
            Ok(ModuleDirective::new_uses_directive(start.cover(end), name, self))
        } else if self.kind() == TokenKind::Provides {
            let start = self.bump().span;
            let name = self.type_name()?;
            if self.kind() != TokenKind::With {
                return Err(diagnostics::message(
                    "expected `with` in a `provides` directive",
                    self.cur().span,
                ));
            }
            self.bump();
            let with = self.separated_list(
                TokenKind::Comma,
                |parser| parser.kind() == TokenKind::Semicolon,
                Self::type_name,
            )?;
            let end = self.expect(TokenKind::Semicolon, "`;` after a `provides` directive")?.span;
            Ok(ModuleDirective::new_provides_directive(start.cover(end), name, with, self))
        } else {
            Err(diagnostics::message(
                "expected a module directive (`requires`, `exports`, `opens`, `uses`, or `provides`)",
                self.cur().span,
            ))
        }
    }
}
