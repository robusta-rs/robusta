use crate::*;

impl<'a> Parser<'a> {
    pub(crate) fn ty(&mut self) -> Result<Type<'a>> {
        let after_annotations = self.annotations_end_from(0);
        let mut ty = if self.nth_kind(after_annotations).is_primitive_type() {
            self.primitive_type()?
        } else {
            self.class_type()?
        };
        // Leave annotations before `...` for the varargs parser.
        for dimension in self.array_type_dimensions()? {
            ty = Type::new_array_type(
                ty.span().cover(dimension.span),
                ty,
                dimension.annotations,
                self,
            );
        }
        Ok(ty)
    }

    /// Parse a type or `void`.
    pub(crate) fn method_result(&mut self) -> Result<MethodResult<'a>> {
        if let Some(token) = self.eat(TokenKind::Void) {
            return Ok(MethodResult::new_void_type(token.span, self));
        }
        Ok(MethodResult::from(self.ty()?))
    }

    pub(crate) fn variable_type(&mut self) -> Result<VariableType<'a>> {
        if self.kind() == TokenKind::Var {
            let span = self.bump().span;
            return Ok(VariableType::new_var_type(span, self));
        }
        Ok(VariableType::from(self.ty()?))
    }

    pub(crate) fn array_type_dimensions(&mut self) -> Result<Vec<'a, ArrayTypeDimension<'a>>> {
        let mut dimensions = Vec::new_in(self);
        loop {
            let after = self.annotations_end_from(0);
            if self.nth_kind(after) != TokenKind::LBracket {
                break;
            }
            let annotations = self.annotations()?;
            let start = annotations.first().map_or(self.cur().span, |ann| ann.span);
            self.expect(TokenKind::LBracket, "`[` in the array type suffix")?;
            let end = self.expect(TokenKind::RBracket, "`]` after the array type suffix")?.span;
            dimensions.push(ArrayTypeDimension::new(start.cover(end), annotations, self));
        }
        Ok(dimensions)
    }

    pub(crate) fn class_type(&mut self) -> Result<Type<'a>> {
        let ty = self.class_type_allowing_diamond(false)?;
        if ty.name() == "var" {
            return Err(diagnostics::message(
                "`var` is only valid as an inferred variable type",
                ty.name_span(),
            ));
        }
        Ok(Type::from(ty))
    }

    /// Parse a class type, each of whose names allows annotations and type
    /// arguments (JLS §4.3). When allowed, `<>` ends it (JLS §15.9).
    pub(crate) fn class_type_allowing_diamond(
        &mut self,
        allow_diamond: bool,
    ) -> Result<ClassType<'a>> {
        let annotations = self.annotations()?;
        let name = self.identifier_reference("a name")?;
        let start = annotations.first().map_or(name.span, |annotation| annotation.span);
        let args = self.type_arguments(allow_diamond)?;
        let mut diamond = args.as_deref().is_some_and(TypeArguments::is_diamond);
        let mut ty = ClassType::new_simple_class_type(
            start.cover(args.as_ref().map_or(name.span, |args| args.span)),
            annotations,
            name,
            args,
            self,
        );
        while !diamond && self.at_class_type_continuation() {
            self.bump();
            let annotations = self.annotations()?;
            let name = self.identifier_name("a name")?;
            let args = self.type_arguments(allow_diamond)?;
            diamond = args.as_deref().is_some_and(TypeArguments::is_diamond);
            ty = ClassType::new_qualified_class_type(
                ty.span().cover(args.as_ref().map_or(name.span, |args| args.span)),
                ty,
                annotations,
                name,
                args,
                self,
            );
        }
        Ok(ty)
    }

    pub(crate) fn primitive_type(&mut self) -> Result<Type<'a>> {
        let annotations = self.annotations()?;
        let kind = match self.kind() {
            TokenKind::Boolean => PrimitiveTypeKind::Boolean,
            TokenKind::Byte => PrimitiveTypeKind::Byte,
            TokenKind::Char => PrimitiveTypeKind::Char,
            TokenKind::Short => PrimitiveTypeKind::Short,
            TokenKind::Int => PrimitiveTypeKind::Int,
            TokenKind::Long => PrimitiveTypeKind::Long,
            TokenKind::Float => PrimitiveTypeKind::Float,
            TokenKind::Double => PrimitiveTypeKind::Double,
            _ => return Err(diagnostics::message("expected a primitive type", self.cur().span)),
        };
        let keyword = self.bump().span;
        let span = annotations.first().map_or(keyword, |annotation| annotation.span).cover(keyword);
        Ok(Type::new_primitive_type(span, annotations, kind, self))
    }

    /// Check for `.Identifier` or `.@Annotation` type continuation.
    pub(crate) fn at_class_type_continuation(&self) -> bool {
        self.is_class_type_continuation_at(0)
    }

    /// Check type continuation at token `n`.
    fn is_class_type_continuation_at(&self, n: usize) -> bool {
        self.nth_kind(n) == TokenKind::Dot
            && (self.nth_kind(n + 1).is_identifier()
                || (self.nth_kind(n + 1) == TokenKind::At && self.nth_kind(n + 2).is_identifier()))
    }

    /// Parse optional type arguments (JLS §4.5.1), or `<>` as an empty list
    /// when `allow_diamond` allows it (JLS §15.9).
    pub(crate) fn type_arguments(
        &mut self,
        allow_diamond: bool,
    ) -> Result<Option<Box<'a, TypeArguments<'a>>>> {
        let Some(open) = self.eat(TokenKind::Lt) else { return Ok(None) };
        let mut args = Vec::new_in(self);
        if self.kind() != TokenKind::Gt {
            loop {
                args.push(self.type_argument()?);
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
            }
        } else if !allow_diamond {
            return Err(diagnostics::message("expected a type argument", self.cur().span));
        }
        let close = self.expect(TokenKind::Gt, "`>` to close the type argument list")?.span;
        Ok(Some(TypeArguments::boxed(open.span.cover(close), args, self)))
    }

    pub(crate) fn type_parameters(&mut self) -> Result<Option<Box<'a, TypeParameters<'a>>>> {
        let Some(open) = self.eat(TokenKind::Lt) else { return Ok(None) };
        if self.kind() == TokenKind::Gt {
            return Err(diagnostics::message("expected a type parameter", self.cur().span));
        }
        let mut params = Vec::new_in(self);
        loop {
            params.push(self.type_parameter()?);
            if self.eat(TokenKind::Comma).is_none() {
                break;
            }
        }
        let close = self.expect(TokenKind::Gt, "`>` to close the type parameter list")?.span;
        Ok(Some(TypeParameters::boxed(open.span.cover(close), params, self)))
    }

    pub(crate) fn type_parameter(&mut self) -> Result<TypeParameter<'a>> {
        let annotations = self.annotations()?;
        let name = self.binding_identifier("a type parameter name")?;
        let mut span =
            annotations.first().map_or(name.span, |annotation| annotation.span).cover(name.span);
        let mut bounds = Vec::new_in(self);
        if self.eat(TokenKind::Extends).is_some() {
            let bound = self.class_type()?;
            span = span.cover(bound.span());
            bounds.push(bound);
            while self.eat(TokenKind::Amp).is_some() {
                let bound = self.class_type()?;
                span = span.cover(bound.span());
                bounds.push(bound);
            }
        }
        Ok(TypeParameter::new(span, annotations, name, bounds, self))
    }

    pub(crate) fn type_argument(&mut self) -> Result<TypeArgument<'a>> {
        let after_annotations = self.annotations_end_from(0);
        if self.nth_kind(after_annotations) == TokenKind::Question {
            let annotations = self.annotations()?;
            let start = self.expect(TokenKind::Question, "`?` in a wildcard type argument")?.span;
            let bound = match self.kind() {
                TokenKind::Extends => Some(self.wildcard_bound(WildcardBoundKind::Extends)?),
                TokenKind::Super => Some(self.wildcard_bound(WildcardBoundKind::Super)?),
                _ => None,
            };
            let start_span = annotations.first().map_or(start, |annotation| annotation.span);
            let span = bound
                .as_ref()
                .map_or(start_span.cover(start), |bound| start_span.cover(bound.span));
            return Ok(TypeArgument::new_wildcard_type_argument(span, annotations, bound, self));
        }
        Ok(TypeArgument::from(self.ty()?))
    }

    pub(crate) fn wildcard_bound(&mut self, kind: WildcardBoundKind) -> Result<WildcardBound<'a>> {
        let start = self.bump().span;
        let ty = self.ty()?;
        Ok(WildcardBound::new(start.cover(ty.span()), kind, ty, self))
    }

    pub(crate) fn type_end_from(&self, mut n: usize) -> Option<usize> {
        // Each name may carry annotations and arguments (JLS §4.3).
        n = self.annotations_end_from(n);
        let last_is_var = if self.nth_kind(n).is_primitive_type() {
            n += 1;
            false
        } else {
            loop {
                if !self.nth_kind(n).is_identifier() {
                    return None;
                }
                let name_is_var = self.nth_kind(n) == TokenKind::Var;
                n += 1;
                if self.nth_kind(n) == TokenKind::Lt {
                    n = self.type_arguments_end_from(n + 1)?;
                }
                if self.is_class_type_continuation_at(n) {
                    n = self.annotations_end_from(n + 1);
                    continue;
                }
                break name_is_var;
            }
        };
        if last_is_var {
            return None;
        }
        Some(self.dimensions_end_from(n))
    }

    /// Skip `[]` dimensions with optional leading annotations.
    pub(crate) fn dimensions_end_from(&self, mut n: usize) -> usize {
        loop {
            let after = self.annotations_end_from(n);
            if self.nth_kind(after) == TokenKind::LBracket
                && self.nth_kind(after + 1) == TokenKind::RBracket
            {
                n = after + 2;
            } else {
                return n;
            }
        }
    }

    pub(crate) fn class_literal_type_end_from(&self, n: usize) -> Option<usize> {
        if self.nth_kind(n) == TokenKind::Void { Some(n + 1) } else { self.type_end_from(n) }
    }

    pub(crate) fn variable_type_end_from(&self, n: usize) -> Option<usize> {
        if self.nth_kind(n) == TokenKind::Var { Some(n + 1) } else { self.type_end_from(n) }
    }

    pub(crate) fn type_arguments_end_from(&self, mut n: usize) -> Option<usize> {
        if self.nth_kind(n) == TokenKind::Gt {
            return None;
        }
        loop {
            n = self.type_argument_end_from(n)?;
            match self.nth_kind(n) {
                TokenKind::Comma => n += 1,
                TokenKind::Gt => return Some(n + 1),
                _ => return None,
            }
        }
    }

    pub(crate) fn type_argument_end_from(&self, n: usize) -> Option<usize> {
        let n = self.annotations_end_from(n);
        if self.nth_kind(n) == TokenKind::Question {
            return match self.nth_kind(n + 1) {
                TokenKind::Extends | TokenKind::Super => self.type_end_from(n + 2),
                _ => Some(n + 1),
            };
        }
        self.type_end_from(n)
    }
}
