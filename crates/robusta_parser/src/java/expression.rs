use super::{
    grammar::CoverGrammar,
    operator::{assign_op, binary_op, update_op},
};
use crate::*;

impl<'a> Parser<'a> {
    pub(crate) fn identifier_name(&mut self, what: &'static str) -> Result<IdentifierName<'a>> {
        let token = self.expect_identifier(what)?;
        let name = self.identifier_text(token);
        Ok(IdentifierName::new(token.span, name, self))
    }

    pub(crate) fn identifier_reference(
        &mut self,
        what: &'static str,
    ) -> Result<IdentifierReference<'a>> {
        let token = self.expect_identifier(what)?;
        let name = self.identifier_text(token);
        Ok(IdentifierReference::new(token.span, name, self))
    }

    pub(crate) fn binding_identifier(
        &mut self,
        what: &'static str,
    ) -> Result<BindingIdentifier<'a>> {
        let token = self.expect_identifier(what)?;
        let name = self.identifier_text(token);
        Ok(self.binding(token.span, name))
    }

    pub(crate) fn label_identifier(&mut self, what: &'static str) -> Result<LabelIdentifier<'a>> {
        let token = self.expect_identifier(what)?;
        let name = self.identifier_text(token);
        Ok(LabelIdentifier::new(token.span, name, self))
    }

    fn identifier_text(&self, token: Token) -> &'a str {
        self.identifier_names
            .get(&token.span.start)
            .copied()
            .unwrap_or_else(|| token.span.source_text(self.source))
    }

    pub(crate) fn binding(&self, span: Span, name: &'a str) -> BindingIdentifier<'a> {
        BindingIdentifier::new(span, name, self)
    }

    /// Parse a variable name, allowing `_` for unnamed binders.
    pub(super) fn variable_declarator_id(
        &mut self,
        what: &'static str,
    ) -> Result<BindingIdentifier<'a>> {
        if let Some(underscore) = self.eat(TokenKind::Underscore) {
            return Ok(self.binding(underscore.span, "_"));
        }
        if !self.kind().is_identifier() {
            return Err(diagnostics::expected(what, self.cur().span));
        }
        self.binding_identifier(what)
    }

    pub(crate) fn expression(&mut self) -> Result<Expression<'a>> {
        // `when test(x -> x > 0) ->` permits nested lambdas.
        self.with_case_label(false, Self::assignment_expression)
    }

    pub(crate) fn assignment_expression(&mut self) -> Result<Expression<'a>> {
        // Lambdas share the assignment-expression level (JLS §15.27).
        if self.at_lambda() {
            return self.lambda_expression();
        }
        let lhs = self.conditional_expression()?;
        let Some((op, width)) = self.peek_assign_op() else {
            return Ok(lhs);
        };
        let target = AssignmentTarget::cover(lhs)?;
        for _ in 0..width {
            self.bump();
        }
        // `a = b = c` associates right.
        let value = self.assignment_expression()?;
        Ok(Expression::new_assignment_expression(
            target.span().cover(value.span()),
            target,
            op,
            value,
            self,
        ))
    }

    pub(crate) fn conditional_expression(&mut self) -> Result<Expression<'a>> {
        let condition = self.binary_expression(Precedence::Conditional)?;
        if self.eat(TokenKind::Question).is_none() {
            return Ok(condition);
        }
        let when_true = self.expression()?;
        self.expect(TokenKind::Colon, "`:` in the conditional expression")?;
        // The false branch allows lambdas, excluding assignments (JLS §15.25).
        let when_false = if self.at_lambda() {
            self.lambda_expression()?
        } else {
            self.conditional_expression()?
        };
        Ok(Expression::new_conditional_expression(
            condition.span().cover(when_false.span()),
            condition,
            when_true,
            when_false,
            self,
        ))
    }

    /// Climb operators that bind tighter than `min_precedence`, associating equal
    /// operators left. `instanceof` takes a type or pattern instead of an
    /// expression (JLS §15.20).
    fn binary_expression(&mut self, min_precedence: Precedence) -> Result<Expression<'a>> {
        let mut lhs = self.unary_expression()?;
        loop {
            if self.kind() == TokenKind::Instanceof {
                if Precedence::Compare <= min_precedence {
                    break;
                }
                self.bump();
                let rhs = self.instanceof_rhs()?;
                lhs = Expression::new_instanceof_expression(
                    lhs.span().cover(rhs.span()),
                    lhs,
                    rhs,
                    self,
                );
                continue;
            }
            let Some((op, width)) = self.peek_binary_op() else {
                break;
            };
            let precedence = op.precedence();
            if precedence <= min_precedence {
                break;
            }
            for _ in 0..width {
                self.bump();
            }
            let rhs = self.binary_expression(precedence)?;
            lhs =
                Expression::new_binary_expression(lhs.span().cover(rhs.span()), op, lhs, rhs, self);
        }
        Ok(lhs)
    }

    fn instanceof_rhs(&mut self) -> Result<InstanceofRhs<'a>> {
        if self.at_pattern() {
            return Ok(InstanceofRhs::from(self.pattern()?));
        }
        Ok(InstanceofRhs::from(self.ty()?))
    }

    pub(crate) fn at_pattern(&self) -> bool {
        self.at_record_pattern() || self.at_type_pattern()
    }

    pub(crate) fn pattern(&mut self) -> Result<Pattern<'a>> {
        if self.at_record_pattern() {
            return Ok(Pattern::RecordPattern(self.record_pattern()?));
        }
        if self.at_type_pattern() {
            return Ok(Pattern::TypePattern(self.type_pattern()?));
        }
        Err(diagnostics::message("expected a pattern", self.cur().span))
    }

    fn at_record_pattern(&self) -> bool {
        self.type_end_from(0)
            .is_some_and(|after_type| self.nth_kind(after_type) == TokenKind::LParen)
    }

    fn record_pattern(&mut self) -> Result<Box<'a, RecordPattern<'a>>> {
        let ty = self.ty()?;
        self.expect(TokenKind::LParen, "`(` to open the record pattern")?;
        let mut components = Vec::new_in(self);
        if self.kind() != TokenKind::RParen {
            loop {
                components.push(self.component_pattern()?);
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
            }
        }
        let end = self.expect(TokenKind::RParen, "`)` to close the record pattern")?.span;
        Ok(RecordPattern::boxed(ty.span().cover(end), ty, components, self))
    }

    fn component_pattern(&mut self) -> Result<ComponentPattern<'a>> {
        if let Some(underscore) = self.eat(TokenKind::Underscore) {
            return Ok(ComponentPattern::new_match_all_pattern(underscore.span, self));
        }
        Ok(ComponentPattern::from(self.pattern()?))
    }

    /// Require a binder after the type and any pattern modifiers.
    pub(crate) fn at_type_pattern(&self) -> bool {
        let after_modifiers = self.modifier_list_end_from(0);
        self.variable_type_end_from(after_modifiers).is_some_and(|after_type| {
            self.nth_kind(after_type).is_identifier()
                || self.nth_kind(after_type) == TokenKind::Underscore
        })
    }

    pub(crate) fn type_pattern(&mut self) -> Result<Box<'a, TypePattern<'a>>> {
        let modifiers = self.modifiers()?;
        let ty = self.variable_type()?;
        let name = self.variable_declarator_id("a pattern variable name")?;
        let declarator = VariableDeclarator::new(name.span, name, Vec::new_in(self), None, self);
        let declaration = self.single_variable_declaration(modifiers, ty, declarator);
        Ok(TypePattern::boxed(declaration.span, declaration, self))
    }

    /// Read a binary operator and its token count. Reconstruct right shifts
    /// from adjacent `>` tokens, leaving shift assignments to [`Self::peek_assign_op`].
    fn peek_binary_op(&self) -> Option<(BinaryOp, usize)> {
        if self.kind() == TokenKind::Gt && self.adjacent(0) {
            match self.nth_kind(1) {
                TokenKind::Gt if self.adjacent(1) && self.nth_kind(2) == TokenKind::Gt => {
                    return Some((BinaryOp::Ushr, 3));
                }
                TokenKind::Gt if self.adjacent(1) && self.nth_kind(2) == TokenKind::GtEq => {
                    return None; // `>>>=`
                }
                TokenKind::Gt => return Some((BinaryOp::Shr, 2)),
                TokenKind::GtEq => return None, // `>>=`
                _ => {}
            }
        }
        binary_op(self.kind()).map(|op| (op, 1))
    }

    /// Read an assignment operator, reconstructing adjacent `>>=` and `>>>=` tokens.
    fn peek_assign_op(&self) -> Option<(AssignOp, usize)> {
        if self.kind() == TokenKind::Gt && self.adjacent(0) {
            match self.nth_kind(1) {
                TokenKind::GtEq => return Some((AssignOp::Shr, 2)),
                TokenKind::Gt if self.adjacent(1) && self.nth_kind(2) == TokenKind::GtEq => {
                    return Some((AssignOp::Ushr, 3));
                }
                _ => return None,
            }
        }
        assign_op(self.kind()).map(|op| (op, 1))
    }

    fn unary_expression(&mut self) -> Result<Expression<'a>> {
        let op = match self.kind() {
            TokenKind::Plus => UnaryOp::Plus,
            TokenKind::Minus => UnaryOp::Minus,
            TokenKind::Bang => UnaryOp::Not,
            TokenKind::Tilde => UnaryOp::BitNot,
            TokenKind::LParen if self.at_cast() => return self.cast_expression(),
            TokenKind::Switch => return self.switch_expression(),
            _ => return self.update_expression(),
        };
        let start = self.bump().span;
        let operand = self.unary_expression()?;
        Ok(Expression::new_unary_expression(start.cover(operand.span()), op, operand, self))
    }

    /// Distinguish casts from parenthesized expressions (JLS §15.16).
    /// Names alone require a following unary-not-plus-minus operand:
    /// `(a) !x` casts, `(a) - x` subtracts, and `(a)++` increments.
    /// Annotations, type arguments, array suffixes, and primitive heads disambiguate types.
    fn at_cast(&self) -> bool {
        let mut start = 1;
        let mut names_only = true;
        let close = loop {
            let Some(end) = self.type_end_from(start) else {
                return false;
            };
            // Type syntax stops the plain-name walk before `end`.
            let mut n = start + 1;
            while self.nth_kind(n) == TokenKind::Dot {
                n += 2;
            }
            names_only &= n == end && !self.nth_kind(start).is_primitive_type();
            match self.nth_kind(end) {
                TokenKind::RParen => break end,
                TokenKind::Amp => start = end + 1,
                _ => return false,
            }
        };
        if !names_only {
            return true;
        }
        let next = self.nth_kind(close + 1);
        next.is_identifier()
            || next.is_literal()
            // `(a) int.class` and `(a) int[]::new`.
            || next.is_primitive_type()
            || matches!(
                next,
                TokenKind::This
                    | TokenKind::Super
                    | TokenKind::New
                    | TokenKind::Void
                    | TokenKind::Switch
                    | TokenKind::LParen
                    | TokenKind::Bang
                    | TokenKind::Tilde
            )
    }

    /// Parse a cast with a unary operand (JLS §15.16).
    fn cast_expression(&mut self) -> Result<Expression<'a>> {
        let start = self.bump().span;
        let ty = self.ty()?;
        // `int[]` allows reference-type bounds, unlike `int`.
        if matches!(ty, Type::Primitive(_)) && self.kind() == TokenKind::Amp {
            return Err(diagnostics::message(
                "a cast to a primitive type cannot have additional bounds",
                self.cur().span,
            ));
        }
        let mut additional_bounds = Vec::new_in(self);
        while self.eat(TokenKind::Amp).is_some() {
            additional_bounds.push(self.class_type()?.into());
        }
        self.expect(TokenKind::RParen, "`)` to close the cast")?;
        // Reference casts accept lambdas: `(Runnable) () -> {}` (JLS §15.16).
        let operand =
            if self.at_lambda() { self.lambda_expression()? } else { self.unary_expression()? };
        Ok(Expression::new_cast_expression(
            start.cover(operand.span()),
            ty,
            additional_bounds,
            operand,
            self,
        ))
    }

    /// Parse a switch expression using the shared block grammar (JLS §15.28).
    fn switch_expression(&mut self) -> Result<Expression<'a>> {
        let start = self.expect(TokenKind::Switch, "`switch`")?.span;
        self.expect(TokenKind::LParen, "`(` after `switch`")?;
        let selector = self.expression()?;
        self.expect(TokenKind::RParen, "`)` after the `switch` selector")?;
        self.expect(TokenKind::LBrace, "`{` to open the `switch` block")?;
        let cases = self.switch_cases()?;
        let end = self.expect(TokenKind::RBrace, "`}` to close the `switch` block")?.span;
        Ok(Expression::new_switch_expression(start.cover(end), selector, cases, self))
    }

    /// Parse `++`/`--` before a unary expression (JLS §15.15.1–2), or a primary
    /// chain and the `++`/`--` after it (JLS §15.14). `.<T>name` requires a call.
    /// No chain link may follow `++`/`--`.
    fn update_expression(&mut self) -> Result<Expression<'a>> {
        if let Some(op) = update_op(self.kind()) {
            let start = self.bump().span;
            let target = AssignmentTarget::cover(self.unary_expression()?)?;
            return Ok(Expression::new_update_expression(
                start.cover(target.span()),
                op,
                true,
                target,
                self,
            ));
        }
        let mut expr = self.primary_chain()?;
        // `x++++` is `(x++)++`, which the cover grammar rejects.
        while let Some(op) = update_op(self.kind()) {
            let target = AssignmentTarget::cover(expr)?;
            let end = self.bump().span;
            expr = Expression::new_update_expression(
                target.span().cover(end),
                op,
                false,
                target,
                self,
            );
        }
        Ok(expr)
    }

    fn primary_chain(&mut self) -> Result<Expression<'a>> {
        self.primary_chain_with_constructor_super_stop(false)
    }

    pub(crate) fn primary_chain_stopping_before_constructor_super(
        &mut self,
    ) -> Result<Expression<'a>> {
        self.primary_chain_with_constructor_super_stop(true)
    }

    fn primary_chain_with_constructor_super_stop(
        &mut self,
        stop_before_constructor_super: bool,
    ) -> Result<Expression<'a>> {
        let mut expr =
            self.primary_expression_with_constructor_super_stop(stop_before_constructor_super)?;
        loop {
            if stop_before_constructor_super && self.at_qualified_constructor_super_tail() {
                return Ok(expr);
            }
            if self.eat(TokenKind::Dot).is_some() {
                // `outer.new Inner()`: the primary supplies the enclosing instance (JLS §15.9).
                if self.kind() == TokenKind::New {
                    expr = self.qualified_creation_expression(expr)?;
                    continue;
                }
                let type_args = self.type_arguments(false)?;
                let name = self.identifier_name("a member name")?;
                match (self.kind() == TokenKind::LParen, type_args) {
                    (true, type_args) => {
                        let callee = Callee::new_method_member(
                            expr.span().cover(name.span),
                            expr,
                            type_args,
                            name,
                            self,
                        );
                        expr = self.method_call(callee)?;
                    }
                    (false, None) => {
                        expr = Expression::new_field_access(
                            expr.span().cover(name.span),
                            expr,
                            name,
                            self,
                        );
                    }
                    (false, Some(_)) => {
                        return Err(diagnostics::message(
                            "expected `(` after explicit method type arguments",
                            self.cur().span,
                        ));
                    }
                }
            } else if self.eat(TokenKind::LBracket).is_some() {
                let index = self.expression()?;
                let end = self.expect(TokenKind::RBracket, "`]` to close the array access")?.span;
                expr = Expression::new_array_access(expr.span().cover(end), expr, index, self);
            } else if self.kind() == TokenKind::ColonColon {
                expr = self.method_reference_rest(MethodReferenceTarget::from(expr))?;
            } else {
                return Ok(expr);
            }
        }
    }

    fn primary_expression_with_constructor_super_stop(
        &mut self,
        stop_before_constructor_super: bool,
    ) -> Result<Expression<'a>> {
        match self.kind() {
            TokenKind::IntLiteral => {
                let (span, raw) = self.literal_token();
                Ok(Expression::new_int_literal(span, raw, self))
            }
            TokenKind::FloatLiteral => {
                let (span, raw) = self.literal_token();
                Ok(Expression::new_float_literal(span, raw, self))
            }
            TokenKind::CharLiteral => {
                let (span, raw) = self.literal_token();
                Ok(Expression::new_char_literal(span, raw, self))
            }
            TokenKind::StringLiteral => {
                let (span, raw) = self.literal_token();
                Ok(Expression::new_string_literal(span, raw, self))
            }
            kind @ (TokenKind::True | TokenKind::False) => {
                let span = self.bump().span;
                Ok(Expression::new_boolean_literal(span, kind == TokenKind::True, self))
            }
            TokenKind::Null => {
                let span = self.bump().span;
                Ok(Expression::new_null_literal(span, self))
            }
            TokenKind::This => {
                let span = self.bump().span;
                Ok(Expression::new_this_expression(span, self))
            }
            TokenKind::Super => {
                let span = self.bump().span;
                Ok(Expression::new_super(span, self))
            }
            TokenKind::New => self.creation_expression(),
            TokenKind::LParen => {
                let start = self.bump().span;
                let expression = self.expression()?;
                let end = self
                    .expect(TokenKind::RParen, "`)` to close the parenthesised expression")?
                    .span;
                Ok(Expression::new_parenthesized_expression(start.cover(end), expression, self))
            }
            // Annotated targets require method references: `@A String::length`.
            TokenKind::At if self.at_type_method_reference() => {
                let ty = self.ty()?;
                self.method_reference_rest(MethodReferenceTarget::from(ty))
            }
            kind if kind.is_identifier() => {
                if self.at_class_literal() {
                    return self.class_literal();
                }
                if self.at_type_method_reference() {
                    let ty = self.ty()?;
                    return self.method_reference_rest(MethodReferenceTarget::from(ty));
                }
                if self.at_qualified_this_or_super(stop_before_constructor_super) {
                    return self.qualified_this_or_super();
                }
                let name = self.identifier_reference("a name")?;
                let name = self.alloc(name);
                if self.kind() == TokenKind::LParen {
                    return self.method_call(Callee::Identifier(name));
                }
                Ok(Expression::Identifier(name))
            }
            kind if kind.is_primitive_type() || kind == TokenKind::Void => {
                if self.at_class_literal() {
                    return self.class_literal();
                }
                if kind.is_primitive_type() && self.at_type_method_reference() {
                    let ty = self.ty()?;
                    return self.method_reference_rest(MethodReferenceTarget::from(ty));
                }
                Err(diagnostics::message("expected an expression", self.cur().span))
            }
            _ => Err(diagnostics::message("expected an expression", self.cur().span)),
        }
    }

    /// Whether a dotted name ends in `.this` or `.super`, which makes it a type
    /// name (JLS 15.8.4, 15.12.1). `stop_before_constructor_super` leaves the
    /// `super` of `a.b.super(...)` to the constructor invocation.
    fn at_qualified_this_or_super(&self, stop_before_constructor_super: bool) -> bool {
        let Some(end) = self.qualified_name_end_from(0) else {
            return false;
        };
        self.nth_kind(end) == TokenKind::Dot
            && match self.nth_kind(end + 1) {
                TokenKind::This => true,
                TokenKind::Super => {
                    !stop_before_constructor_super || self.nth_kind(end + 2) != TokenKind::LParen
                }
                _ => false,
            }
    }

    fn qualified_this_or_super(&mut self) -> Result<Expression<'a>> {
        let qualifier = self.type_name()?;
        self.expect(TokenKind::Dot, "`.` after the type name")?;
        let start = qualifier.span();
        if let Some(this_token) = self.eat(TokenKind::This) {
            return Ok(Expression::new_qualified_this(
                start.cover(this_token.span),
                qualifier,
                self,
            ));
        }
        let super_token = self.expect(TokenKind::Super, "`super` after the type name")?;
        if !matches!(self.kind(), TokenKind::Dot | TokenKind::ColonColon) {
            return Err(diagnostics::message(
                "expected a member access or method reference after qualified `super`",
                self.cur().span,
            ));
        }
        Ok(Expression::new_qualified_super(start.cover(super_token.span), qualifier, self))
    }

    fn at_class_literal(&self) -> bool {
        self.class_literal_type_end_from(0).is_some_and(|end| {
            self.nth_kind(end) == TokenKind::Dot && self.nth_kind(end + 1) == TokenKind::Class
        })
    }

    fn class_literal(&mut self) -> Result<Expression<'a>> {
        let ty = self.method_result()?;
        self.expect(TokenKind::Dot, "`.` before `class` in a class literal")?;
        let class_token = self.expect(TokenKind::Class, "`class` in a class literal")?;
        Ok(Expression::new_class_literal(ty.span().cover(class_token.span), ty, self))
    }

    fn method_call(&mut self, callee: Callee<'a>) -> Result<Expression<'a>> {
        let (args, end) = self.argument_list()?;
        Ok(Expression::new_method_call(callee.span().cover(end), callee, args, self))
    }

    /// Recognize type-only method-reference targets (JLS §15.13.1).
    /// Annotations, type arguments, or array suffixes disambiguate a type.
    /// Plain dotted names remain expression targets. Reject non-type syntax
    /// early to avoid scanning whole comparisons or array accesses.
    fn at_type_method_reference(&self) -> bool {
        let head = self.annotations_end_from(0);
        let mut n = head + 1;
        while self.nth_kind(n) == TokenKind::Dot && self.nth_kind(n + 1).is_identifier() {
            n += 2;
        }
        let suffix = self.annotations_end_from(n);
        let type_only = match self.nth_kind(suffix) {
            TokenKind::Lt => true,
            TokenKind::LBracket => self.nth_kind(suffix + 1) == TokenKind::RBracket,
            // `int::m` is invalid, including with annotations.
            _ => head > 0 && !self.nth_kind(head).is_primitive_type(),
        };
        type_only
            && self.type_end_from(0).is_some_and(|end| self.nth_kind(end) == TokenKind::ColonColon)
    }

    /// Parse `:: TypeArguments? (Identifier | new)` (JLS §15.13).
    /// Accept `new` after any target. Validity requires semantic analysis.
    fn method_reference_rest(
        &mut self,
        target: MethodReferenceTarget<'a>,
    ) -> Result<Expression<'a>> {
        self.expect(TokenKind::ColonColon, "`::`")?;
        let type_args = self.type_arguments(false)?;
        let (method, end) = if let Some(new) = self.eat(TokenKind::New) {
            (None, new.span)
        } else {
            let name = self.identifier_name("a method name after `::`")?;
            let end = name.span;
            (Some(name), end)
        };
        Ok(Expression::new_method_reference(
            target.span().cover(end),
            target,
            type_args,
            method,
            self,
        ))
    }

    /// Parse class or array creation (JLS §15.9, §15.10.1).
    /// Allow diamond syntax and anonymous class bodies (JLS §15.9.5).
    fn creation_expression(&mut self) -> Result<Expression<'a>> {
        let new_span = self.expect(TokenKind::New, "`new`")?.span;
        let constructor_type_args = self.type_arguments(false)?;
        let after_annotations = self.annotations_end_from(0);
        if self.nth_kind(after_annotations).is_primitive_type() {
            if constructor_type_args.is_some() {
                return Err(diagnostics::message(
                    "an array creation cannot use explicit constructor type arguments",
                    self.cur().span,
                ));
            }
            let ty = self.primitive_type()?;
            if self.nth_kind(self.annotations_end_from(0)) != TokenKind::LBracket {
                return Err(diagnostics::message(
                    "a primitive array creation needs an array dimension",
                    self.cur().span,
                ));
            }
            return self.array_creation_rest(new_span, ty);
        }
        let ty = self.class_type_allowing_diamond(true)?;
        if self.nth_kind(self.annotations_end_from(0)) == TokenKind::LBracket {
            if ty.type_arguments().is_some_and(TypeArguments::is_diamond) {
                return Err(diagnostics::message(
                    "an array creation cannot use the diamond `<>`",
                    self.cur().span,
                ));
            }
            if constructor_type_args.is_some() {
                return Err(diagnostics::message(
                    "an array creation cannot use explicit constructor type arguments",
                    self.cur().span,
                ));
            }
            return self.array_creation_rest(new_span, Type::from(ty));
        }
        let (args, body, end) = self.creation_rest()?;
        Ok(Expression::new_class_instance_creation(
            new_span.cover(end),
            constructor_type_args,
            ty,
            args,
            body,
            self,
        ))
    }

    /// Parse `new` after `qualifier .`, which creates an inner class, whose name
    /// is a member of the qualifier's type (JLS §15.9.1).
    fn qualified_creation_expression(
        &mut self,
        qualifier: Expression<'a>,
    ) -> Result<Expression<'a>> {
        self.expect(TokenKind::New, "`new`")?;
        let constructor_type_args = self.type_arguments(false)?;
        if self.nth_kind(self.annotations_end_from(0)).is_primitive_type() {
            return Err(Self::qualified_array_creation(self.cur().span));
        }
        let annotations = self.annotations()?;
        let name = self.identifier_name("a name")?;
        let type_args = self.type_arguments(true)?;
        if self.nth_kind(self.annotations_end_from(0)) == TokenKind::LBracket {
            return Err(Self::qualified_array_creation(self.cur().span));
        }
        let (args, body, end) = self.creation_rest()?;
        Ok(Expression::new_qualified_class_instance_creation(
            qualifier.span().cover(end),
            qualifier,
            constructor_type_args,
            annotations,
            name,
            type_args,
            args,
            body,
            self,
        ))
    }

    fn qualified_array_creation(span: Span) -> ParserDiagnostic {
        diagnostics::message("a qualified `new` creates an inner class, not an array", span)
    }

    /// Parse the arguments and optional class body, returning where they end.
    fn creation_rest(&mut self) -> Result<(Vec<'a, Expression<'a>>, Option<ClassBody<'a>>, Span)> {
        let (args, mut end) = self.argument_list()?;
        let body = if self.kind() == TokenKind::LBrace {
            let body = self.type_body()?;
            end = body.span;
            Some(body)
        } else {
            None
        };
        Ok((args, body, end))
    }

    /// Parse array dimensions and an optional initializer (JLS §15.10.1).
    /// Sized dimensions precede empty ones and forbid initializers.
    /// All-empty dimensions require an initializer.
    fn array_creation_rest(&mut self, start: Span, element: Type<'a>) -> Result<Expression<'a>> {
        let mut dims = Vec::new_in(self);
        let mut seen_empty = false;
        let mut end = start;
        while self.nth_kind(self.annotations_end_from(0)) == TokenKind::LBracket {
            let annotations = self.annotations()?;
            let open = self.bump().span;
            let length = if self.kind() == TokenKind::RBracket {
                None
            } else {
                if seen_empty {
                    return Err(diagnostics::message(
                        "a sized array dimension cannot follow an empty one",
                        self.cur().span,
                    ));
                }
                Some(self.expression()?)
            };
            seen_empty |= length.is_none();
            let close = self.expect(TokenKind::RBracket, "`]` to close the array dimension")?.span;
            end = close;
            let span = annotations.first().map_or(open, |annotation| annotation.span).cover(close);
            dims.push(ArrayDimension::new(span, annotations, length, self));
        }
        let init = if dims[0].length.is_none() {
            let init = self.array_initializer()?;
            end = init.span;
            Some(init)
        } else {
            None
        };
        Ok(Expression::new_array_creation(start.cover(end), element, dims, init, self))
    }

    /// Parse call arguments and return the closing parenthesis span.
    pub(crate) fn argument_list(&mut self) -> Result<(Vec<'a, Expression<'a>>, Span)> {
        self.expect(TokenKind::LParen, "`(` to open the argument list")?;
        let mut args = Vec::new_in(self);
        if self.kind() != TokenKind::RParen {
            loop {
                args.push(self.expression()?);
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
            }
        }
        let end = self.expect(TokenKind::RParen, "`)` to close the argument list")?.span;
        Ok((args, end))
    }

    /// Consume a literal token, with its span and spelling.
    fn literal_token(&mut self) -> (Span, &'a str) {
        let span = self.bump().span;
        (span, span.source_text(self.source))
    }
}
