use super::VariableDeclarationParent;
use crate::*;

impl<'a> Parser<'a> {
    /// Check for a type followed by a variable name.
    pub(crate) fn at_local_variable_declaration(&self) -> bool {
        let n = self.modifier_list_end_from(0);
        self.variable_type_end_from(n).is_some_and(|after_type| {
            self.nth_kind(after_type).is_identifier()
                || self.nth_kind(after_type) == TokenKind::Underscore
        })
    }

    /// Parse a local variable declaration. A `for` header's declaration ends
    /// at its last declarator, leaving the separator and the `var` check to
    /// the header (JLS §14.14.1).
    pub(crate) fn local_variable_declaration(
        &mut self,
        parent: VariableDeclarationParent,
    ) -> Result<Box<'a, LocalVariableDeclaration<'a>>> {
        let modifiers = self.modifiers()?;
        let ty = self.variable_type()?;
        let start = modifiers.first().map_or_else(|| ty.span(), robusta_span::GetSpan::span);
        let first = self.variable_declarator_id("a variable name")?;
        let declarators = self.declarator_list(first)?;
        let end = match parent {
            VariableDeclarationParent::For => declarators.last().expect("declarator").span,
            VariableDeclarationParent::Statement => {
                let semicolon =
                    self.expect(TokenKind::Semicolon, "`;` after the variable declaration")?;
                Self::validate_local_variable_type_inference(&ty, &declarators)?;
                semicolon.span
            }
        };
        Ok(LocalVariableDeclaration::boxed(start.cover(end), modifiers, ty, declarators, self))
    }

    pub(crate) fn validate_local_variable_type_inference(
        ty: &VariableType<'a>,
        declarators: &[VariableDeclarator<'a>],
    ) -> Result<()> {
        if !matches!(ty, VariableType::Var(_)) {
            return Ok(());
        }
        if declarators.len() != 1 {
            return Err(diagnostics::message_with_help(
                "`var` declarations must declare exactly one variable",
                ty.span(),
                "Split the declaration or use an explicit type.",
            ));
        }
        let declarator = &declarators[0];
        Self::reject_var_dims(ty, &declarator.dims)?;
        let Some(init) = &declarator.init else {
            return Err(diagnostics::message_with_help(
                "`var` declarations need an initializer",
                declarator.name.span,
                "Add an initializer or use an explicit type.",
            ));
        };
        if matches!(init, VariableInitializer::Array(_)) {
            return Err(diagnostics::message_with_help(
                "`var` declarations need an expression initializer",
                init.span(),
                "Use an expression initializer or an explicit array type.",
            ));
        }
        Ok(())
    }

    pub(crate) fn field_rest(
        &mut self,
        modifiers: Vec<'a, Modifier>,
        ty: Type<'a>,
        first: BindingIdentifier<'a>,
    ) -> Result<Box<'a, FieldDeclaration<'a>>> {
        let start = modifiers.first().map_or_else(|| ty.span(), robusta_span::GetSpan::span);
        let declarators = self.declarator_list(first)?;
        let end = self.expect(TokenKind::Semicolon, "`;` after the field declaration")?.span;
        Ok(FieldDeclaration::boxed(start.cover(end), modifiers, ty, declarators, self))
    }

    fn declarator_list(
        &mut self,
        first: BindingIdentifier<'a>,
    ) -> Result<Vec<'a, VariableDeclarator<'a>>> {
        let mut declarators = Vec::new_in(self);
        declarators.push(self.declarator_rest(first)?);
        while self.eat(TokenKind::Comma).is_some() {
            let name = self.variable_declarator_id("a variable name")?;
            declarators.push(self.declarator_rest(name)?);
        }
        Ok(declarators)
    }

    fn declarator_rest(&mut self, name: BindingIdentifier<'a>) -> Result<VariableDeclarator<'a>> {
        let dims = self.variable_declarator_dims(&name)?;
        let mut span = dims.last().map_or(name.span, |last| name.span.cover(last.span));
        let init = if self.eat(TokenKind::Assign).is_some() {
            let init = self.variable_initializer()?;
            span = span.cover(init.span());
            Some(init)
        } else {
            None
        };
        Ok(VariableDeclarator::new(span, name, dims, init, self))
    }

    fn variable_initializer(&mut self) -> Result<VariableInitializer<'a>> {
        if self.kind() == TokenKind::LBrace {
            let array = self.array_initializer()?;
            return Ok(VariableInitializer::Array(self.alloc(array)));
        }
        Ok(VariableInitializer::from(self.expression()?))
    }

    /// Parse an array initializer with optional trailing comma (JLS §10.6).
    pub(crate) fn array_initializer(&mut self) -> Result<ArrayInitializer<'a>> {
        let start = self.expect(TokenKind::LBrace, "`{` to open the array initializer")?.span;
        let mut elements = Vec::new_in(self);
        if self.eat(TokenKind::Comma).is_none() {
            while !matches!(self.kind(), TokenKind::RBrace | TokenKind::Eof) {
                elements.push(self.variable_initializer()?);
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
            }
        }
        let end = self.expect(TokenKind::RBrace, "`}` to close the array initializer")?.span;
        Ok(ArrayInitializer::new(start.cover(end), elements, self))
    }
}
