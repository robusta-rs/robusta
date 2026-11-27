use super::VariableDeclarationParent;
use crate::*;

impl<'a> Parser<'a> {
    pub(crate) fn constructor_body(&mut self) -> Result<Block<'a>> {
        let start = self.expect(TokenKind::LBrace, "`{` to open a constructor body")?.span;
        let mut statements = Vec::new_in(self);
        let mut seen_invocation = false;
        loop {
            if let Some(semi) = self.eat(TokenKind::Semicolon) {
                statements.push(self.empty_statement(semi.span));
                continue;
            }
            if matches!(self.kind(), TokenKind::RBrace | TokenKind::Eof) {
                break;
            }
            // JLS §8.8.7 allows constructor invocations after a prologue.
            let statement = if self.at_constructor_invocation() {
                if seen_invocation {
                    Err(diagnostics::message_with_help(
                        "only one constructor invocation is allowed",
                        self.cur().span,
                        "Remove the additional constructor invocation.",
                    ))
                } else {
                    self.constructor_invocation().map(|invocation| {
                        seen_invocation = true;
                        Statement::ConstructorInvocation(invocation)
                    })
                }
            } else {
                self.block_statement()
            };
            match statement {
                Ok(statement) => statements.push(statement),
                Err(error) if self.recovery_enabled => {
                    self.record_error(error);
                    self.recover_body_item(false);
                }
                Err(error) => return Err(error),
            }
        }
        let end = self.expect(TokenKind::RBrace, "`}` to close the constructor body")?.span;
        Ok(Block::new(start.cover(end), statements, self))
    }

    fn at_constructor_invocation(&self) -> bool {
        if matches!(self.kind(), TokenKind::This | TokenKind::Super) {
            return self.nth_kind(1) == TokenKind::LParen;
        }
        if self.kind() == TokenKind::Lt
            && self.type_arguments_end_from(1).is_some_and(|end| {
                matches!(self.nth_kind(end), TokenKind::This | TokenKind::Super)
                    && self.nth_kind(end + 1) == TokenKind::LParen
            })
        {
            return true;
        }
        self.at_qualified_constructor_invocation()
    }

    /// Look ahead for a primary chain ending in `.[TypeArguments]super(...)`.
    /// Stop at top-level operators that cannot belong to the qualifier.
    fn at_qualified_constructor_invocation(&self) -> bool {
        if !self.kind().is_identifier()
            && !self.kind().is_literal()
            && !matches!(
                self.kind(),
                TokenKind::This | TokenKind::Super | TokenKind::New | TokenKind::LParen
            )
        {
            return false;
        }

        let mut n = 0usize;
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut brace_depth = 0usize;
        loop {
            let at_top = paren_depth == 0 && bracket_depth == 0 && brace_depth == 0;
            match self.nth_kind(n) {
                TokenKind::Eof => return false,
                TokenKind::Dot if at_top => {
                    if self
                        .constructor_super_after_dot_from(n + 1)
                        .is_some_and(|super_at| self.nth_kind(super_at + 1) == TokenKind::LParen)
                    {
                        return n > 0;
                    }
                }
                TokenKind::Semicolon | TokenKind::RBrace if at_top => return false,
                kind if at_top && is_constructor_invocation_lookahead_boundary(kind) => {
                    return false;
                }
                TokenKind::LParen => paren_depth += 1,
                TokenKind::RParen => {
                    if paren_depth == 0 {
                        return false;
                    }
                    paren_depth -= 1;
                }
                TokenKind::LBracket => bracket_depth += 1,
                TokenKind::RBracket => {
                    if bracket_depth == 0 {
                        return false;
                    }
                    bracket_depth -= 1;
                }
                TokenKind::LBrace => brace_depth += 1,
                TokenKind::RBrace => {
                    if brace_depth == 0 {
                        return false;
                    }
                    brace_depth -= 1;
                }
                _ => {}
            }
            n += 1;
        }
    }

    /// Find `super` in `.super(...)` or `.<T>super(...)` after the dot.
    fn constructor_super_after_dot_from(&self, mut n: usize) -> Option<usize> {
        if self.nth_kind(n) == TokenKind::Lt {
            n = self.type_arguments_end_from(n + 1)?;
        }
        (self.nth_kind(n) == TokenKind::Super).then_some(n)
    }

    pub(crate) fn at_qualified_constructor_super_tail(&self) -> bool {
        self.kind() == TokenKind::Dot
            && self
                .constructor_super_after_dot_from(1)
                .is_some_and(|super_at| self.nth_kind(super_at + 1) == TokenKind::LParen)
    }

    fn constructor_invocation(&mut self) -> Result<Box<'a, ConstructorInvocation<'a>>> {
        let start = self.cur().span;
        let type_args;
        let (qualifier, kind) = if self.at_qualified_constructor_invocation() {
            let qualifier = self.constructor_invocation_qualifier()?;
            self.expect(TokenKind::Dot, "`.` before qualified `super`")?;
            type_args = self.type_arguments(false)?;
            self.expect(TokenKind::Super, "`super` after the qualifier")?;
            (Some(qualifier), ConstructorInvocationKind::Super)
        } else {
            type_args = self.type_arguments(false)?;
            let token = self.bump();
            let kind = match token.kind {
                TokenKind::This => ConstructorInvocationKind::This,
                TokenKind::Super => ConstructorInvocationKind::Super,
                _ => unreachable!("constructor invocation starts with this or super"),
            };
            (None, kind)
        };
        let (args, _) = self.argument_list()?;
        let end = self.expect(TokenKind::Semicolon, "`;` after the constructor invocation")?.span;
        Ok(ConstructorInvocation::boxed(start.cover(end), qualifier, type_args, kind, args, self))
    }

    fn constructor_invocation_qualifier(&mut self) -> Result<Expression<'a>> {
        let expr = self.primary_chain_stopping_before_constructor_super()?;
        if self.at_qualified_constructor_super_tail() {
            Ok(expr)
        } else {
            Err(diagnostics::message(
                "expected `.super` in the constructor invocation",
                self.cur().span,
            ))
        }
    }

    fn empty_statement(&self, span: Span) -> Statement<'a> {
        Statement::new_empty_statement(span, self)
    }

    pub(crate) fn block(&mut self) -> Result<Block<'a>> {
        let start = self.expect(TokenKind::LBrace, "`{` to open a block")?.span;
        let mut statements = Vec::new_in(self);
        loop {
            if let Some(semi) = self.eat(TokenKind::Semicolon) {
                statements.push(self.empty_statement(semi.span));
                continue;
            }
            if matches!(self.kind(), TokenKind::RBrace | TokenKind::Eof) {
                break;
            }
            match self.block_statement() {
                Ok(statement) => statements.push(statement),
                Err(error) if self.recovery_enabled => {
                    self.record_error(error);
                    self.recover_body_item(false);
                }
                Err(error) => return Err(error),
            }
        }
        let end = self.expect(TokenKind::RBrace, "`}` to close the block")?.span;
        Ok(Block::new(start.cover(end), statements, self))
    }

    fn block_statement(&mut self) -> Result<Statement<'a>> {
        if self.at_yield_statement() {
            return self.yield_statement();
        }
        if self.at_local_type_declaration_start() {
            let modifiers = self.type_modifiers()?;
            return Ok(Statement::from(self.local_type_declaration_rest(modifiers)?));
        }
        if self.at_local_variable_declaration() {
            return Ok(Statement::LocalVariable(
                self.local_variable_declaration(VariableDeclarationParent::Statement)?,
            ));
        }
        self.statement()
    }

    fn statement(&mut self) -> Result<Statement<'a>> {
        match self.kind() {
            TokenKind::Semicolon => {
                let semi = self.bump();
                Ok(self.empty_statement(semi.span))
            }
            TokenKind::LBrace => {
                let block = self.block()?;
                Ok(Statement::Block(self.alloc(block)))
            }
            TokenKind::If => self.if_statement(),
            TokenKind::Switch => self.switch_statement(),
            TokenKind::While => self.while_statement(),
            TokenKind::Do => self.do_statement(),
            TokenKind::For => self.for_statement(),
            TokenKind::Break => self.break_statement(),
            TokenKind::Continue => self.continue_statement(),
            TokenKind::Throw => self.throw_statement(),
            TokenKind::Assert => self.assert_statement(),
            TokenKind::Try => self.try_statement(),
            TokenKind::Synchronized => self.synchronized_statement(),
            TokenKind::Return => self.return_statement(),
            TokenKind::Yield if self.at_yield_statement() => self.yield_statement(),
            // `case` and `default` cannot be statement-label names.
            kind if kind.is_identifier() && self.nth_kind(1) == TokenKind::Colon => {
                self.labeled_statement()
            }
            _ => {
                let expression = self.expression()?;
                Self::require_statement_expression(&expression)?;
                let end =
                    self.expect(TokenKind::Semicolon, "`;` after the expression statement")?.span;
                Ok(Statement::new_expression_statement(
                    expression.span().cover(end),
                    expression,
                    self,
                ))
            }
        }
    }

    /// Require [`Expression::is_statement_expression`] for statements and `for` lists (JLS §14.8).
    fn require_statement_expression(expression: &Expression<'a>) -> Result<()> {
        if expression.is_statement_expression() {
            Ok(())
        } else {
            Err(diagnostics::message(
                "this expression cannot be used as a statement",
                expression.span(),
            ))
        }
    }

    /// Parse the statement governed by a control-flow header.
    fn embedded_statement(&mut self) -> Result<Statement<'a>> {
        if let Some(semi) = self.eat(TokenKind::Semicolon) {
            return Ok(self.empty_statement(semi.span));
        }
        self.statement()
    }

    fn if_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        self.expect(TokenKind::LParen, "`(` after `if`")?;
        let condition = self.expression()?;
        self.expect(TokenKind::RParen, "`)` after the `if` condition")?;
        let then_branch = self.embedded_statement()?;
        // `else` binds to the nearest unmatched `if`.
        let else_branch = if self.eat(TokenKind::Else).is_some() {
            Some(self.embedded_statement()?)
        } else {
            None
        };
        let end = else_branch.as_ref().map_or_else(|| then_branch.span(), GetSpan::span);
        Ok(Statement::new_if_statement(start.cover(end), condition, then_branch, else_branch, self))
    }

    /// Parse arrow rules or colon cases (JLS §14.11).
    fn switch_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        self.expect(TokenKind::LParen, "`(` after `switch`")?;
        let selector = self.expression()?;
        self.expect(TokenKind::RParen, "`)` after the `switch` selector")?;
        self.expect(TokenKind::LBrace, "`{` to open the `switch` block")?;
        let cases = self.switch_cases()?;
        let end = self.expect(TokenKind::RBrace, "`}` to close the `switch` block")?.span;
        Ok(Statement::new_switch_statement(start.cover(end), selector, cases, self))
    }

    pub(crate) fn switch_cases(&mut self) -> Result<Vec<'a, SwitchItem<'a>>> {
        let mut cases = Vec::new_in(self);
        if matches!(self.kind(), TokenKind::RBrace | TokenKind::Eof) {
            return Ok(cases);
        }
        let start = self.cur().span;
        let first_label = self.switch_label()?;
        let is_rule = match self.kind() {
            TokenKind::Arrow => true,
            TokenKind::Colon => false,
            _ => {
                return Err(diagnostics::message(
                    "expected `:` or `->` after a switch label",
                    self.cur().span,
                ));
            }
        };
        cases.push(if is_rule {
            self.switch_rule_after_label(start, first_label)?
        } else {
            self.switch_case_after_label(start, first_label)?
        });
        while !matches!(self.kind(), TokenKind::RBrace | TokenKind::Eof) {
            let case = if is_rule { self.switch_rule() } else { self.switch_case() };
            match case {
                Ok(case) => cases.push(case),
                Err(error) if self.recovery_enabled => {
                    self.record_error(error);
                    self.recover_body_item(true);
                }
                Err(error) => return Err(error),
            }
        }
        Ok(cases)
    }

    fn switch_rule(&mut self) -> Result<SwitchItem<'a>> {
        let start = self.cur().span;
        let label = self.switch_label()?;
        self.switch_rule_after_label(start, label)
    }

    fn switch_rule_after_label(
        &mut self,
        start: Span,
        label: Option<SwitchLabel<'a>>,
    ) -> Result<SwitchItem<'a>> {
        self.expect(TokenKind::Arrow, "`->` after the switch label")?;
        let (body, end) = match self.kind() {
            TokenKind::LBrace => {
                let block = self.block()?;
                let end = block.span;
                (SwitchRuleBody::Block(self.alloc(block)), end)
            }
            TokenKind::Throw => {
                let stmt = self.throw_statement()?;
                let Statement::Throw(throw_stmt) = stmt else {
                    unreachable!("throw_statement returns a throw statement");
                };
                let end = throw_stmt.span;
                (SwitchRuleBody::Throw(throw_stmt), end)
            }
            _ => {
                let expression = self.expression()?;
                let semicolon =
                    self.expect(TokenKind::Semicolon, "`;` after the switch rule expression")?.span;
                let end = expression.span().cover(semicolon);
                (SwitchRuleBody::from(expression), end)
            }
        };
        Ok(SwitchItem::new_switch_rule(start.cover(end), label, body, self))
    }

    fn switch_case(&mut self) -> Result<SwitchItem<'a>> {
        let start = self.cur().span;
        let label = self.switch_label()?;
        self.switch_case_after_label(start, label)
    }

    /// Parse the statements up to the next label (JLS §14.11.1).
    fn switch_case_after_label(
        &mut self,
        start: Span,
        label: Option<SwitchLabel<'a>>,
    ) -> Result<SwitchItem<'a>> {
        let colon = self.expect(TokenKind::Colon, "`:` after the switch label")?.span;
        let mut statements = Vec::new_in(self);
        loop {
            if let Some(semi) = self.eat(TokenKind::Semicolon) {
                statements.push(self.empty_statement(semi.span));
                continue;
            }
            if matches!(
                self.kind(),
                TokenKind::Case | TokenKind::Default | TokenKind::RBrace | TokenKind::Eof
            ) {
                break;
            }
            match self.block_statement() {
                Ok(statement) => statements.push(statement),
                Err(error) if self.recovery_enabled => {
                    self.record_error(error);
                    self.recover_body_item(true);
                }
                Err(error) => return Err(error),
            }
        }
        let end = statements.last().map_or_else(|| colon, GetSpan::span);
        Ok(SwitchItem::new_switch_case(start.cover(end), label, statements, self))
    }

    /// Parse a label, `None` for `default`.
    fn switch_label(&mut self) -> Result<Option<SwitchLabel<'a>>> {
        match self.kind() {
            // `case (int) X -> 1` casts `X`.
            TokenKind::Case => self.with_case_label(true, Self::case_label).map(Some),
            TokenKind::Default => {
                self.bump();
                Ok(None)
            }
            _ => Err(diagnostics::message(
                "expected `case` or `default` in a `switch` block",
                self.cur().span,
            )),
        }
    }

    /// Parse `case` alternatives using conditional expressions.
    /// Constant-expression validity requires semantic analysis.
    fn case_label(&mut self) -> Result<SwitchLabel<'a>> {
        let start = self.bump().span;
        if self.at_pattern() {
            return self.case_pattern_label_after_case(start);
        }
        let mut constants = Vec::new_in(self);
        let mut has_default = false;
        let constant = self.conditional_expression()?;
        let mut end = constant.span();
        constants.push(constant);
        while self.eat(TokenKind::Comma).is_some() {
            if self.kind() == TokenKind::Default {
                let default = self.bump();
                if constants.len() != 1
                    || !matches!(constants.first(), Some(Expression::NullLiteral(_)))
                {
                    return Err(diagnostics::message(
                        "`default` may only be combined with `null` in a switch label",
                        default.span,
                    ));
                }
                has_default = true;
                end = default.span;
                break;
            }
            let constant = self.conditional_expression()?;
            end = constant.span();
            constants.push(constant);
        }
        Ok(SwitchLabel::new_case_label(start.cover(end), constants, has_default, self))
    }

    fn case_pattern_label_after_case(&mut self, start: Span) -> Result<SwitchLabel<'a>> {
        let mut patterns = Vec::new_in(self);
        loop {
            patterns.push(self.pattern()?);
            if self.eat(TokenKind::Comma).is_none() {
                break;
            }
            if !self.at_pattern() {
                return Err(diagnostics::message(
                    "expected a pattern after `,` in the switch label",
                    self.cur().span,
                ));
            }
        }
        let guard = if self.kind() == TokenKind::When {
            self.bump();
            // Avoid reading `when b -> 1` as a lambda.
            Some(self.assignment_expression()?)
        } else {
            None
        };
        let end = guard.as_ref().map_or_else(
            || patterns.last().expect("case pattern label is non-empty").span(),
            GetSpan::span,
        );
        Ok(SwitchLabel::new_case_pattern_label(start.cover(end), patterns, guard, self))
    }

    fn while_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        self.expect(TokenKind::LParen, "`(` after `while`")?;
        let condition = self.expression()?;
        self.expect(TokenKind::RParen, "`)` after the `while` condition")?;
        let body = self.embedded_statement()?;
        Ok(Statement::new_while_statement(start.cover(body.span()), condition, body, self))
    }

    fn do_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        let body = self.embedded_statement()?;
        self.expect(TokenKind::While, "`while` after the `do` body")?;
        self.expect(TokenKind::LParen, "`(` after `while`")?;
        let condition = self.expression()?;
        self.expect(TokenKind::RParen, "`)` after the `do` condition")?;
        let end = self.expect(TokenKind::Semicolon, "`;` after the `do` statement")?.span;
        Ok(Statement::new_do_statement(start.cover(end), body, condition, self))
    }

    /// Parse basic or enhanced `for` (JLS §14.14).
    /// A colon after the variable and its dimensions identifies the enhanced form.
    fn for_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        self.expect(TokenKind::LParen, "`(` after `for`")?;
        if self.at_enhanced_for() {
            let modifiers = self.modifiers()?;
            let ty = self.variable_type()?;
            let name = self.variable_declarator_id("the loop variable name")?;
            let dims = self.variable_declarator_dims(&name)?;
            Self::reject_var_dims(&ty, &dims)?;
            let span = dims.last().map_or(name.span, |last| name.span.cover(last.span));
            let declarator = VariableDeclarator::new(span, name, dims, None, self);
            let declaration = self.single_variable_declaration(modifiers, ty, declarator);
            self.expect(TokenKind::Colon, "`:` in the enhanced `for` header")?;
            let iterable = self.expression()?;
            self.expect(TokenKind::RParen, "`)` to close the `for` header")?;
            let body = self.embedded_statement()?;
            return Ok(Statement::new_enhanced_for_statement(
                start.cover(body.span()),
                declaration,
                iterable,
                body,
                self,
            ));
        }
        let init = if self.kind() == TokenKind::Semicolon {
            None
        } else if self.at_local_variable_declaration() {
            Some(ForInit::Declaration(
                self.local_variable_declaration(VariableDeclarationParent::For)?,
            ))
        } else {
            let expressions = self.statement_expression_list()?;
            Some(ForInit::Expressions(self.alloc(expressions)))
        };
        self.expect(TokenKind::Semicolon, "`;` after the `for` initializer")?;
        if let Some(ForInit::Declaration(declaration)) = &init {
            Self::validate_local_variable_type_inference(
                &declaration.ty,
                &declaration.declarators,
            )?;
        }
        let condition =
            if self.kind() == TokenKind::Semicolon { None } else { Some(self.expression()?) };
        self.expect(TokenKind::Semicolon, "`;` after the `for` condition")?;
        let update = if self.kind() == TokenKind::RParen {
            None
        } else {
            Some(self.statement_expression_list()?)
        };
        self.expect(TokenKind::RParen, "`)` to close the `for` header")?;
        let body = self.embedded_statement()?;
        Ok(Statement::new_for_statement(
            start.cover(body.span()),
            init,
            condition,
            update,
            body,
            self,
        ))
    }

    /// Check for an enhanced `for` header.
    fn at_enhanced_for(&self) -> bool {
        let n = self.modifier_list_end_from(0);
        self.variable_type_end_from(n).is_some_and(|after_type| {
            (self.nth_kind(after_type).is_identifier()
                || self.nth_kind(after_type) == TokenKind::Underscore)
                && self.nth_kind(self.dimensions_end_from(after_type + 1)) == TokenKind::Colon
        })
    }

    /// Parse comma-separated `for` statement expressions (JLS §14.14.1).
    fn statement_expression_list(&mut self) -> Result<StatementExpressionList<'a>> {
        let start = self.cur().span;
        let mut expressions = Vec::new_in(self);
        let end = loop {
            let expression = self.expression()?;
            Self::require_statement_expression(&expression)?;
            let end = expression.span();
            expressions.push(expression);
            if self.eat(TokenKind::Comma).is_none() {
                break end;
            }
        };
        Ok(StatementExpressionList::new(start.cover(end), expressions, self))
    }

    fn break_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        let label = if self.kind().is_identifier() {
            Some(self.label_identifier("a label")?)
        } else {
            None
        };
        let end = self.expect(TokenKind::Semicolon, "`;` after `break`")?.span;
        Ok(Statement::new_break_statement(start.cover(end), label, self))
    }

    fn continue_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        let label = if self.kind().is_identifier() {
            Some(self.label_identifier("a label")?)
        } else {
            None
        };
        let end = self.expect(TokenKind::Semicolon, "`;` after `continue`")?.span;
        Ok(Statement::new_continue_statement(start.cover(end), label, self))
    }

    fn throw_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        let value = self.expression()?;
        let end = self.expect(TokenKind::Semicolon, "`;` after the throw statement")?.span;
        Ok(Statement::new_throw_statement(start.cover(end), value, self))
    }

    fn assert_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        let condition = self.expression()?;
        let message =
            if self.eat(TokenKind::Colon).is_some() { Some(self.expression()?) } else { None };
        let end = self.expect(TokenKind::Semicolon, "`;` after the assert statement")?.span;
        Ok(Statement::new_assert_statement(start.cover(end), condition, message, self))
    }

    fn try_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.expect(TokenKind::Try, "`try`")?.span;
        let (resources, resources_trailing_semicolon) = if self.kind() == TokenKind::LParen {
            self.resource_specification()?
        } else {
            (Vec::new_in(self), false)
        };
        let block = self.block()?;
        let mut catches = Vec::new_in(self);
        while self.kind() == TokenKind::Catch {
            catches.push(self.catch_clause()?);
        }
        let finally_block =
            if self.eat(TokenKind::Finally).is_some() { Some(self.block()?) } else { None };
        // Only try-with-resources can omit both catch and finally (JLS §14.20).
        if resources.is_empty() && catches.is_empty() && finally_block.is_none() {
            return Err(diagnostics::message(
                "expected `catch` or `finally` after the `try` block",
                block.span,
            ));
        }
        let end = finally_block
            .as_ref()
            .map_or_else(|| catches.last().map_or(block.span, GetSpan::span), GetSpan::span);
        Ok(Statement::new_try_statement(
            start.cover(end),
            resources,
            resources_trailing_semicolon,
            block,
            catches,
            finally_block,
            self,
        ))
    }

    /// Parse resources, preserving a trailing semicolon (JLS §14.20.3).
    /// Return the resources and trailing-semicolon flag.
    fn resource_specification(&mut self) -> Result<(Vec<'a, Resource<'a>>, bool)> {
        self.expect(TokenKind::LParen, "`(` to open the resource list")?;
        let mut resources = Vec::new_in(self);
        resources.push(self.resource()?);
        let mut trailing_semicolon = false;
        while self.eat(TokenKind::Semicolon).is_some() {
            if self.kind() == TokenKind::RParen {
                trailing_semicolon = true;
                break;
            }
            resources.push(self.resource()?);
        }
        self.expect(TokenKind::RParen, "`)` after the resource list")?;
        Ok((resources, trailing_semicolon))
    }

    /// Parse a resource declaration, name, or field access (JLS §14.20.3).
    fn resource(&mut self) -> Result<Resource<'a>> {
        if self.at_local_variable_declaration() {
            let modifiers = self.modifiers()?;
            let ty = self.variable_type()?;
            let name = self.variable_declarator_id("the resource name")?;
            self.expect(TokenKind::Assign, "`=` in the resource declaration")?;
            let init = self.expression()?;
            let span = name.span.cover(init.span());
            let declarator =
                VariableDeclarator::new(span, name, Vec::new_in(self), Some(init.into()), self);
            let declaration = self.single_variable_declaration(modifiers, ty, declarator);
            Ok(Resource::Declaration(self.alloc(declaration)))
        } else {
            let expression = self.expression()?;
            let span = expression.span();
            match expression {
                Expression::Identifier(name) => Ok(Resource::Identifier(name)),
                Expression::FieldAccess(access) => Ok(Resource::Field(access)),
                _ => Err(diagnostics::message(
                    "a resource variable access must be a name or field access",
                    span,
                )),
            }
        }
    }

    fn catch_clause(&mut self) -> Result<CatchClause<'a>> {
        let start = self.expect(TokenKind::Catch, "`catch`")?.span;
        self.expect(TokenKind::LParen, "`(` after `catch`")?;
        let modifiers = self.modifiers()?;
        let mut types = Vec::new_in(self);
        types.push(self.class_type()?);
        while self.eat(TokenKind::Pipe).is_some() {
            types.push(self.class_type()?);
        }
        let name = self.variable_declarator_id("the catch parameter name")?;
        self.expect(TokenKind::RParen, "`)` after the catch parameter")?;
        let block = self.block()?;
        Ok(CatchClause::new(start.cover(block.span), modifiers, types, name, block, self))
    }

    fn synchronized_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.expect(TokenKind::Synchronized, "`synchronized`")?.span;
        self.expect(TokenKind::LParen, "`(` after `synchronized`")?;
        let lock = self.expression()?;
        self.expect(TokenKind::RParen, "`)` after the synchronized lock")?;
        let body = self.block()?;
        Ok(Statement::new_synchronized_statement(start.cover(body.span), lock, body, self))
    }

    fn return_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        let value =
            if self.kind() == TokenKind::Semicolon { None } else { Some(self.expression()?) };
        let end = self.expect(TokenKind::Semicolon, "`;` after the return statement")?.span;
        Ok(Statement::new_return_statement(start.cover(end), value, self))
    }

    fn yield_statement(&mut self) -> Result<Statement<'a>> {
        let start = self.bump().span;
        let value = self.expression()?;
        let end = self.expect(TokenKind::Semicolon, "`;` after the yield statement")?.span;
        Ok(Statement::new_yield_statement(start.cover(end), value, self))
    }

    fn at_yield_statement(&self) -> bool {
        self.kind() == TokenKind::Yield && !self.yield_starts_statement_expression()
    }

    // A variable named `yield` can start a statement expression.
    fn yield_starts_statement_expression(&self) -> bool {
        match self.nth_kind(1) {
            TokenKind::Dot | TokenKind::LBracket => true,
            kind if kind.is_assignment_operator() || kind.is_update_operator() => true,
            TokenKind::Gt if self.adjacent(1) => {
                self.nth_kind(2) == TokenKind::GtEq
                    || (self.nth_kind(2) == TokenKind::Gt
                        && self.adjacent(2)
                        && self.nth_kind(3) == TokenKind::GtEq)
            }
            _ => false,
        }
    }

    fn labeled_statement(&mut self) -> Result<Statement<'a>> {
        let label = self.label_identifier("a label")?;
        self.expect(TokenKind::Colon, "`:` after the label")?;
        let statement = self.embedded_statement()?;
        Ok(Statement::new_labeled_statement(
            label.span.cover(statement.span()),
            label,
            statement,
            self,
        ))
    }
}

/// Stop qualifier lookahead at top-level operators.
/// Keep `<`, `>`, `?`, and `,` for explicit method type arguments.
fn is_constructor_invocation_lookahead_boundary(kind: TokenKind) -> bool {
    kind.is_assignment_operator()
        || kind.is_update_operator()
        || (kind.is_binary_operator() && !matches!(kind, TokenKind::Lt | TokenKind::Gt))
        || matches!(kind, TokenKind::Arrow | TokenKind::Colon | TokenKind::ColonColon)
}
