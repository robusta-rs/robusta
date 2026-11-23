use crate::*;

impl<'a> Parser<'a> {
    /// Check for `name ->`, `_ ->`, or `(params) ->`.
    pub(crate) fn at_lambda(&self) -> bool {
        if self.in_case_label {
            return false;
        }
        match self.kind() {
            kind if kind.is_identifier() => self.nth_kind(1) == TokenKind::Arrow,
            TokenKind::Underscore => self.nth_kind(1) == TokenKind::Arrow,
            TokenKind::LParen => self.arrow_after_parens(),
            _ => false,
        }
    }

    /// Scan for `(...) ->`, stopping at non-parameter tokens.
    fn arrow_after_parens(&self) -> bool {
        let mut depth = 1u32;
        let mut n = 1;
        loop {
            match self.nth_kind(n) {
                // Annotation arguments may nest parentheses: `(@A(1) T x)`.
                TokenKind::LParen => depth += 1,
                TokenKind::RParen => {
                    depth -= 1;
                    if depth == 0 {
                        return self.nth_kind(n + 1) == TokenKind::Arrow;
                    }
                }
                TokenKind::Eof => return false,
                // Annotation arguments allow arbitrary element values.
                _ if depth > 1 => {}
                // Contextual keywords can name parameters or types.
                kind if kind.is_identifier() => {}
                TokenKind::Underscore
                | TokenKind::Boolean
                | TokenKind::Byte
                | TokenKind::Char
                | TokenKind::Double
                | TokenKind::Float
                | TokenKind::Int
                | TokenKind::Long
                | TokenKind::Short
                | TokenKind::Dot
                | TokenKind::Comma
                | TokenKind::Lt
                | TokenKind::Gt
                | TokenKind::Ellipsis
                | TokenKind::LBracket
                | TokenKind::RBracket
                | TokenKind::Question
                | TokenKind::Extends
                | TokenKind::Super
                | TokenKind::At
                | TokenKind::Final => {}
                _ => return false,
            }
            n += 1;
        }
    }

    /// Parse `params -> body` after [`Self::at_lambda`] (JLS §15.27).
    pub(crate) fn lambda_expression(&mut self) -> Result<Expression<'a>> {
        let start = self.cur().span;
        let mut params = Vec::new_in(self);
        let params_span = if self.kind().is_identifier() || self.kind() == TokenKind::Underscore {
            let name = self.variable_declarator_id("a lambda parameter")?;
            let span = name.span;
            params.push(FormalParameter::new(span, [], None, false, [], name, [], self));
            span
        } else {
            let open = self.expect(TokenKind::LParen, "`(` to open the lambda parameters")?;
            if self.kind() != TokenKind::RParen {
                loop {
                    let param = self.lambda_parameter()?;
                    let varargs = param.varargs;
                    params.push(param);
                    if self.eat(TokenKind::Comma).is_none() {
                        break;
                    }
                    if varargs {
                        return Err(diagnostics::message_with_help(
                            "a varargs parameter must be the final lambda parameter",
                            self.cur().span,
                            "Move the varargs parameter to the end of the parameter list.",
                        ));
                    }
                }
            }
            let close = self.expect(TokenKind::RParen, "`)` to close the lambda parameters")?;
            open.span.cover(close.span)
        };
        // JLS §15.27.1 forbids mixing typed and concise parameters.
        if let Some(first) = params.first() {
            let first_is_typed = first.ty.is_some();
            if let Some(mixed) = params.iter().find(|p| p.ty.is_some() != first_is_typed) {
                return Err(diagnostics::message_with_help(
                    "lambda parameters must either all declare types or all omit them",
                    mixed.span,
                    "Declare a type for every parameter or omit every parameter type.",
                ));
            }
        }
        self.expect(TokenKind::Arrow, "`->` after the lambda parameters")?;
        let (body, end) = if self.kind() == TokenKind::LBrace {
            let block = self.block()?;
            let end = block.span;
            (LambdaBody::Block(self.alloc(block)), end)
        } else {
            let expression = self.expression()?;
            let end = expression.span();
            (LambdaBody::from(expression), end)
        };
        let params = FormalParameters::new(params_span, None, params, self);
        Ok(Expression::new_lambda_expression(start.cover(end), params, body, self))
    }

    /// Parse a concise name or a typed parameter.
    fn lambda_parameter(&mut self) -> Result<FormalParameter<'a>> {
        let modifiers = self.modifiers()?;
        if modifiers.is_empty()
            && (self.kind().is_identifier() || self.kind() == TokenKind::Underscore)
            && matches!(self.nth_kind(1), TokenKind::Comma | TokenKind::RParen)
        {
            let name = self.variable_declarator_id("the lambda parameter name")?;
            return Ok(FormalParameter::new(name.span, modifiers, None, false, [], name, [], self));
        }
        let ty = self.variable_type()?;
        if matches!(ty, VariableType::Var(_))
            && self.nth_kind(self.annotations_end_from(0)) == TokenKind::Ellipsis
        {
            return Err(diagnostics::message_with_help(
                "`var` lambda parameters cannot be varargs",
                ty.span(),
                "Use an explicit element type for the varargs parameter.",
            ));
        }
        let (varargs_annotations, varargs) = self.varargs_marker()?;
        let name = self.variable_declarator_id("the lambda parameter name")?;
        let dims = self.parameter_dims(varargs, &name)?;
        Self::reject_var_dims(&ty, &dims)?;
        let start = modifiers.first().map_or_else(|| ty.span(), robusta_span::GetSpan::span);
        let end = dims.last().map_or(name.span, |last| last.span);
        Ok(FormalParameter::new(
            start.cover(end),
            modifiers,
            Some(ty),
            varargs,
            varargs_annotations,
            name,
            dims,
            self,
        ))
    }
}
