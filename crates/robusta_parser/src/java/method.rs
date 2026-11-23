use crate::*;

impl<'a> Parser<'a> {
    pub(crate) fn method_rest(
        &mut self,
        start: u32,
        modifiers: Vec<'a, Modifier>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        return_type: MethodResult<'a>,
        name: BindingIdentifier<'a>,
    ) -> Result<Box<'a, MethodDeclaration<'a>>> {
        let params = self.formal_parameters()?;
        // Leave `[` after `void` for the body diagnostic.
        let post_paren_dimensions = if matches!(return_type, MethodResult::Void(_)) {
            Vec::new_in(self)
        } else {
            self.array_type_dimensions()?
        };
        let throws = self.throws_clause()?;
        let (body, end) = if self.kind() == TokenKind::LBrace {
            let block = self.block()?;
            let end = block.span.end;
            (Some(block), end)
        } else {
            let semi = self.expect(TokenKind::Semicolon, "a method body or `;`")?;
            (None, semi.span.end)
        };
        Ok(MethodDeclaration::boxed(
            Span::new(start, end),
            modifiers,
            type_params,
            return_type,
            name,
            params,
            post_paren_dimensions,
            throws,
            body,
            self,
        ))
    }

    pub(crate) fn constructor_rest(
        &mut self,
        start: u32,
        modifiers: Vec<'a, Modifier>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
    ) -> Result<Box<'a, ConstructorDeclaration<'a>>> {
        let name = self.identifier_name("a constructor name")?;
        let params = self.formal_parameters()?;
        let throws = self.throws_clause()?;
        let body = self.constructor_body()?;
        Ok(ConstructorDeclaration::boxed(
            Span::new(start, body.span.end),
            modifiers,
            type_params,
            name,
            params,
            throws,
            body,
            self,
        ))
    }

    pub(crate) fn compact_constructor_rest(
        &mut self,
        modifiers: Vec<'a, Modifier>,
    ) -> Result<Box<'a, CompactConstructorDeclaration<'a>>> {
        let name = self.identifier_name("a compact constructor name")?;
        let start = modifiers.first().map_or(name.span, robusta_span::GetSpan::span);
        let body = self.constructor_body()?;
        Ok(CompactConstructorDeclaration::boxed(
            start.cover(body.span),
            modifiers,
            name,
            body,
            self,
        ))
    }

    fn formal_parameters(&mut self) -> Result<FormalParameters<'a>> {
        let open = self.expect(TokenKind::LParen, "`(` to open the parameter list")?;
        let mut receiver = None;
        let mut params = Vec::new_in(self);
        if self.kind() != TokenKind::RParen {
            loop {
                let modifiers = self.modifiers()?;
                let ty = self.ty()?;
                let varargs = if self.at_receiver_name() {
                    if receiver.is_some() || !params.is_empty() {
                        return Err(diagnostics::message_with_help(
                            "a receiver parameter must be the first parameter",
                            self.cur().span,
                            "Move the receiver parameter to the start of the parameter list.",
                        ));
                    }
                    receiver = Some(self.receiver_parameter(modifiers, ty)?);
                    false
                } else {
                    let param = self.formal_parameter(modifiers, ty)?;
                    let varargs = param.varargs;
                    params.push(param);
                    varargs
                };
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
                if varargs {
                    return Err(diagnostics::message_with_help(
                        "a varargs parameter must be the final parameter",
                        self.cur().span,
                        "Move the varargs parameter to the end of the parameter list.",
                    ));
                }
            }
        }
        let close = self.expect(TokenKind::RParen, "`)` to close the parameter list")?;
        Ok(FormalParameters::new(open.span.cover(close.span), receiver, params, self))
    }

    /// `this` or `Identifier . this` after a type.
    fn at_receiver_name(&self) -> bool {
        self.kind() == TokenKind::This
            || (self.kind().is_identifier()
                && self.nth_kind(1) == TokenKind::Dot
                && self.nth_kind(2) == TokenKind::This)
    }

    fn receiver_parameter(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
        ty: Type<'a>,
    ) -> Result<Box<'a, ReceiverParameter<'a>>> {
        let qualifier = if self.kind() == TokenKind::This {
            None
        } else {
            let qualifier = self.identifier_name("a receiver qualifier")?;
            self.bump();
            Some(qualifier)
        };
        let this = self.expect(TokenKind::This, "`this`")?.span;
        let start = modifiers.first().map_or_else(|| ty.span(), robusta_span::GetSpan::span);
        Ok(ReceiverParameter::boxed(start.cover(this), modifiers, ty, qualifier, self))
    }

    fn formal_parameter(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
        ty: Type<'a>,
    ) -> Result<FormalParameter<'a>> {
        let (varargs_annotations, varargs) = self.varargs_marker()?;
        let name = self.variable_declarator_id("a parameter name")?;
        let dims = self.parameter_dims(varargs, &name)?;
        let start = modifiers.first().map_or_else(|| ty.span(), robusta_span::GetSpan::span);
        let end = dims.last().map_or(name.span, |last| last.span);
        Ok(FormalParameter::new(
            start.cover(end),
            modifiers,
            Some(ty.into()),
            varargs,
            varargs_annotations,
            name,
            dims,
            self,
        ))
    }

    /// Consume annotations only when followed by `...`.
    pub(crate) fn varargs_marker(&mut self) -> Result<(Vec<'a, Annotation<'a>>, bool)> {
        if self.nth_kind(self.annotations_end_from(0)) == TokenKind::Ellipsis {
            let annotations = self.annotations()?;
            self.bump();
            Ok((annotations, true))
        } else {
            Ok((Vec::new_in(self), false))
        }
    }

    fn throws_clause(&mut self) -> Result<Vec<'a, Type<'a>>> {
        if self.eat(TokenKind::Throws).is_some() {
            // Thrown types allow annotations, excluding array types (JLS §8.4.6).
            return self.separated_list(
                TokenKind::Comma,
                |parser| matches!(parser.kind(), TokenKind::LBrace | TokenKind::Semicolon),
                Self::class_type,
            );
        }
        Ok(Vec::new_in(self))
    }
}
