use crate::*;

enum FieldOrMethodTail<'a> {
    Field(Box<'a, FieldDeclaration<'a>>),
    Method(Box<'a, MethodDeclaration<'a>>),
}

enum FieldOrAnnotationElementTail<'a> {
    Field(Box<'a, FieldDeclaration<'a>>),
    Element(Box<'a, AnnotationInterfaceElementDeclaration<'a>>),
}

impl<'a> Parser<'a> {
    pub(crate) fn compact_compilation_unit(&mut self) -> Result<CompactCompilationUnit<'a>> {
        let mut members = Vec::new_in(self);
        let mut start = None;
        let mut end = None;
        let mut saw_method = false;
        while self.kind() != TokenKind::Eof {
            match self.compact_source_member() {
                Ok(member) => {
                    if start.is_none() {
                        start = Some(member.span());
                    }
                    end = Some(member.span());
                    saw_method |= matches!(member, ClassMember::Method(_));
                    members.push(member);
                }
                Err(error) if self.recovery_enabled => {
                    self.record_error(error);
                    self.recover_body_item(false);
                    // Skip stray `}` in the implicit class body.
                    self.eat(TokenKind::RBrace);
                }
                Err(error) => return Err(error),
            }
        }
        if !saw_method {
            return Err(diagnostics::message(
                "expected at least one method declaration in a compact source file",
                self.cur().span,
            ));
        }
        let span = start.unwrap().cover(end.unwrap());
        // `compilation_unit` supplies the imports parsed before this body.
        Ok(CompactCompilationUnit::new(span, [], members, self))
    }

    /// Parse implicit class members, excluding constructors and initializers (JLS §8.1.8).
    fn compact_source_member(&mut self) -> Result<ClassMember<'a>> {
        let start = self.cur().span.start;
        if let Some(semi) = self.eat(TokenKind::Semicolon) {
            return Ok(ClassMember::new_empty_statement(semi.span, self));
        }
        if self.kind() == TokenKind::LBrace
            || (self.kind() == TokenKind::Static && self.nth_kind(1) == TokenKind::LBrace)
        {
            return Err(diagnostics::message(
                "initializer blocks are not valid in compact source files",
                self.cur().span,
            ));
        }
        if self.at_type_declaration_start_with_modifiers() {
            let modifiers = self.type_modifiers()?;
            return Ok(ClassMember::from(self.type_declaration_rest(modifiers)?));
        }
        let modifiers = self.modifiers()?;
        if self.at_type_declaration_start() {
            return Ok(ClassMember::from(self.type_declaration_rest(modifiers)?));
        }
        let type_params = self.type_parameters()?;
        if self.kind().is_identifier() && self.nth_kind(1) == TokenKind::LParen {
            return Err(diagnostics::message(
                "constructors are not valid in compact source files",
                self.cur().span,
            ));
        }
        match self.field_or_method_tail(
            start,
            modifiers,
            type_params,
            "a compact source member name",
            "a field name",
            "type parameters are only allowed on methods",
        )? {
            FieldOrMethodTail::Field(field) => Ok(ClassMember::Field(field)),
            FieldOrMethodTail::Method(method) => Ok(ClassMember::Method(method)),
        }
    }

    pub(crate) fn type_declaration(&mut self) -> Result<TypeDeclaration<'a>> {
        let modifiers = if self.at_type_declaration_start_with_modifiers() {
            self.type_modifiers()?
        } else {
            self.modifiers()?
        };
        self.type_declaration_rest(modifiers)
    }

    /// Parse a type declaration after its modifiers.
    pub(crate) fn type_declaration_rest(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
    ) -> Result<TypeDeclaration<'a>> {
        match self.kind() {
            TokenKind::Class | TokenKind::Interface | TokenKind::Enum | TokenKind::Record => {
                self.local_type_declaration_rest(modifiers).map(TypeDeclaration::from)
            }
            TokenKind::At if self.nth_kind(1) == TokenKind::Interface => {
                Ok(TypeDeclaration::AnnotationInterface(
                    self.annotation_interface_declaration(modifiers)?,
                ))
            }
            _ => Err(diagnostics::message(
                "expected a class, interface, enum, record, or annotation interface declaration",
                self.cur().span,
            )),
        }
    }

    fn class_declaration(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
    ) -> Result<Box<'a, ClassDeclaration<'a>>> {
        let class_token = self.expect(TokenKind::Class, "a class declaration")?;
        let start = modifiers.first().map_or(class_token.span, robusta_span::GetSpan::span);
        let name = self.binding_identifier("a class name")?;
        let type_params = self.type_parameters()?;
        let superclass =
            if self.eat(TokenKind::Extends).is_some() { Some(self.class_type()?) } else { None };
        let interfaces = self.implements_clause()?;
        let permits = self.permits_clause()?;
        let body = self.type_body()?;

        Ok(ClassDeclaration::boxed(
            start.cover(body.span),
            modifiers,
            name,
            type_params,
            superclass,
            interfaces,
            permits,
            body,
            self,
        ))
    }

    /// Parse interface members separately from class members (JLS §9.1).
    fn interface_declaration(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
    ) -> Result<Box<'a, InterfaceDeclaration<'a>>> {
        let interface_token = self.expect(TokenKind::Interface, "an interface declaration")?;
        let start = modifiers.first().map_or(interface_token.span, robusta_span::GetSpan::span);
        let name = self.binding_identifier("an interface name")?;
        let type_params = self.type_parameters()?;
        let extends = if self.eat(TokenKind::Extends).is_some() {
            self.separated_list(
                TokenKind::Comma,
                |parser| parser.kind() == TokenKind::Permits || parser.kind() == TokenKind::LBrace,
                Self::class_type,
            )?
        } else {
            Vec::new_in(self)
        };
        let permits = self.permits_clause()?;
        let body = self.interface_body()?;

        Ok(InterfaceDeclaration::boxed(
            start.cover(body.span),
            modifiers,
            name,
            type_params,
            extends,
            permits,
            body,
            self,
        ))
    }

    /// Parse constants, then class members after `;` (JLS §8.9).
    fn enum_declaration(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
    ) -> Result<Box<'a, EnumDeclaration<'a>>> {
        let enum_token = self.expect(TokenKind::Enum, "an enum declaration")?;
        let start = modifiers.first().map_or(enum_token.span, robusta_span::GetSpan::span);
        let name = self.binding_identifier("an enum name")?;
        let interfaces = self.implements_clause()?;
        let body = self.enum_body()?;

        Ok(EnumDeclaration::boxed(start.cover(body.span), modifiers, name, interfaces, body, self))
    }

    /// Parse a record, allowing compact constructors (JLS §8.10).
    fn record_declaration(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
    ) -> Result<Box<'a, RecordDeclaration<'a>>> {
        if self.kind() != TokenKind::Record {
            return Err(diagnostics::message("expected `record`", self.cur().span));
        }
        let record_token = self.bump();
        let start = modifiers.first().map_or(record_token.span, robusta_span::GetSpan::span);
        let name = self.binding_identifier("a record name")?;
        let type_params = self.type_parameters()?;
        let header = self.record_header()?;
        let interfaces = self.implements_clause()?;
        let body = self.record_body()?;

        Ok(RecordDeclaration::boxed(
            start.cover(body.span),
            modifiers,
            name,
            type_params,
            header,
            interfaces,
            body,
            self,
        ))
    }

    fn record_header(&mut self) -> Result<RecordHeader<'a>> {
        let open = self.expect(TokenKind::LParen, "`(` to open the record header")?;
        let mut components = Vec::new_in(self);
        if self.kind() != TokenKind::RParen {
            loop {
                let component = self.record_component()?;
                let varargs = component.varargs;
                components.push(component);
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
                if varargs {
                    return Err(diagnostics::message_with_help(
                        "a varargs record component must be the final component",
                        self.cur().span,
                        "Move the varargs component to the end of the record header.",
                    ));
                }
            }
        }
        let close = self.expect(TokenKind::RParen, "`)` to close the record header")?;
        Ok(RecordHeader::new(open.span.cover(close.span), components, self))
    }

    fn record_component(&mut self) -> Result<RecordComponent<'a>> {
        let modifiers = self.record_component_modifiers()?;
        let ty = self.ty()?;
        let (varargs_annotations, varargs) = self.varargs_marker()?;
        let name = self.binding_identifier("a record component name")?;
        let start = modifiers.first().map_or_else(|| ty.span(), robusta_span::GetSpan::span);
        Ok(RecordComponent::new(
            start.cover(name.span),
            modifiers,
            ty,
            varargs,
            varargs_annotations,
            name,
            self,
        ))
    }

    fn implements_clause(&mut self) -> Result<Vec<'a, Type<'a>>> {
        if self.eat(TokenKind::Implements).is_some() {
            return self.separated_list(
                TokenKind::Comma,
                |parser| parser.kind() == TokenKind::Permits || parser.kind() == TokenKind::LBrace,
                Self::class_type,
            );
        }
        Ok(Vec::new_in(self))
    }

    fn permits_clause(&mut self) -> Result<Vec<'a, TypeName<'a>>> {
        if self.kind() == TokenKind::Permits {
            self.bump();
            return self.separated_list(
                TokenKind::Comma,
                |parser| parser.kind() == TokenKind::LBrace,
                Self::type_name,
            );
        }
        Ok(Vec::new_in(self))
    }

    fn enum_body(&mut self) -> Result<EnumBody<'a>> {
        let open = self.expect(TokenKind::LBrace, "`{` to open the enum body")?.span;
        let mut constants = Vec::new_in(self);

        if self.eat(TokenKind::Comma).is_none()
            && !matches!(self.kind(), TokenKind::Semicolon | TokenKind::RBrace)
        {
            loop {
                constants.push(self.enum_constant()?);
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
                if matches!(self.kind(), TokenKind::Semicolon | TokenKind::RBrace) {
                    break;
                }
            }
        }

        let has_body_semicolon = self.eat(TokenKind::Semicolon).is_some();
        let members = if has_body_semicolon {
            self.member_list(ClassMember::Empty, Self::class_member)?
        } else {
            Vec::new_in(self)
        };

        let close = self.expect(TokenKind::RBrace, "`}` to close the enum body")?.span;
        Ok(EnumBody::new(open.cover(close), constants, has_body_semicolon, members, self))
    }

    fn enum_constant(&mut self) -> Result<EnumConstant<'a>> {
        let modifiers = self.enum_constant_modifiers()?;
        let name = self.binding_identifier("an enum constant name")?;
        let start = modifiers.first().map_or(name.span, robusta_span::GetSpan::span);
        let mut end = name.span;
        let args = if self.kind() == TokenKind::LParen {
            let (args, close) = self.argument_list()?;
            end = close;
            Some(args)
        } else {
            None
        };
        let body = if self.kind() == TokenKind::LBrace {
            let body = self.type_body()?;
            end = body.span;
            Some(body)
        } else {
            None
        };
        Ok(EnumConstant::new(start.cover(end), modifiers, name, args, body, self))
    }

    /// Parse `@interface`, allowing element defaults (JLS §9.6).
    fn annotation_interface_declaration(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
    ) -> Result<Box<'a, AnnotationInterfaceDeclaration<'a>>> {
        let at = self.expect(TokenKind::At, "`@` in an annotation interface")?;
        self.expect(TokenKind::Interface, "`interface` after `@`")?;
        let start = modifiers.first().map_or(at.span, robusta_span::GetSpan::span);
        let name = self.binding_identifier("an annotation interface name")?;
        let body = self.annotation_interface_body()?;

        Ok(AnnotationInterfaceDeclaration::boxed(
            start.cover(body.span),
            modifiers,
            name,
            body,
            self,
        ))
    }

    fn annotation_interface_body(&mut self) -> Result<AnnotationInterfaceBody<'a>> {
        let (span, members) = self.member_body(
            "`{` to open the annotation interface body",
            "`}` to close the annotation interface body",
            AnnotationInterfaceMemberDeclaration::Empty,
            Self::annotation_interface_member,
        )?;
        Ok(AnnotationInterfaceBody::new(span, members, self))
    }

    fn annotation_interface_member(&mut self) -> Result<AnnotationInterfaceMemberDeclaration<'a>> {
        if self.at_type_declaration_start_with_modifiers() {
            let modifiers = self.type_modifiers()?;
            return Ok(AnnotationInterfaceMemberDeclaration::from(
                self.type_declaration_rest(modifiers)?,
            ));
        }
        let modifiers = self.modifiers()?;
        if self.at_type_declaration_start() {
            return Ok(AnnotationInterfaceMemberDeclaration::from(
                self.type_declaration_rest(modifiers)?,
            ));
        }
        let ty = self.ty()?;
        match self.field_or_annotation_element_tail(
            modifiers,
            ty,
            "an annotation interface member name",
        )? {
            FieldOrAnnotationElementTail::Field(field) => {
                Ok(AnnotationInterfaceMemberDeclaration::Field(field))
            }
            FieldOrAnnotationElementTail::Element(element) => {
                Ok(AnnotationInterfaceMemberDeclaration::Element(element))
            }
        }
    }

    fn annotation_interface_element_rest(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
        ty: Type<'a>,
        name: BindingIdentifier<'a>,
    ) -> Result<Box<'a, AnnotationInterfaceElementDeclaration<'a>>> {
        let start = modifiers.first().map_or_else(|| ty.span(), robusta_span::GetSpan::span);
        self.expect(TokenKind::LParen, "`(` after the annotation interface element name")?;
        self.expect(TokenKind::RParen, "`)` after the annotation interface element name")?;
        let post_paren_dimensions = self.array_type_dimensions()?;
        let default_value =
            if self.eat(TokenKind::Default).is_some() { Some(self.element_value()?) } else { None };
        let end =
            self.expect(TokenKind::Semicolon, "`;` after the annotation interface element")?.span;
        Ok(AnnotationInterfaceElementDeclaration::boxed(
            start.cover(end),
            modifiers,
            ty,
            name,
            post_paren_dimensions,
            default_value,
            self,
        ))
    }

    fn interface_body(&mut self) -> Result<InterfaceBody<'a>> {
        let (span, members) = self.member_body(
            "`{` to open the interface body",
            "`}` to close the interface body",
            InterfaceMember::Empty,
            Self::interface_member,
        )?;
        Ok(InterfaceBody::new(span, members, self))
    }

    fn interface_member(&mut self) -> Result<InterfaceMember<'a>> {
        let start = self.cur().span.start;
        if self.kind() == TokenKind::LBrace
            || (self.kind() == TokenKind::Static && self.nth_kind(1) == TokenKind::LBrace)
        {
            return Err(diagnostics::message(
                "expected an interface member declaration",
                self.cur().span,
            ));
        }

        if self.at_type_declaration_start_with_modifiers() {
            let modifiers = self.type_modifiers()?;
            return Ok(InterfaceMember::from(self.type_declaration_rest(modifiers)?));
        }
        let modifiers = self.modifiers()?;
        if self.at_type_declaration_start() {
            return Ok(InterfaceMember::from(self.type_declaration_rest(modifiers)?));
        }
        let type_params = self.type_parameters()?;
        if self.kind().is_identifier() && self.nth_kind(1) == TokenKind::LParen {
            return Err(diagnostics::message(
                "constructors are not valid interface members",
                self.cur().span,
            ));
        }
        match self.field_or_method_tail(
            start,
            modifiers,
            type_params,
            "an interface member name",
            "an interface constant name",
            "type parameters are only allowed on interface methods",
        )? {
            FieldOrMethodTail::Field(field) => Ok(InterfaceMember::Constant(field)),
            FieldOrMethodTail::Method(method) => Ok(InterfaceMember::Method(method)),
        }
    }

    /// Parse `{ member* }`, wrapping stray `;` via `empty`.
    /// Return the span from `{` to `}` and the members (JLS §8.1.7, §8.9.2, §8.10.2, §9.1.5, §9.6.1).
    fn member_body<T>(
        &mut self,
        open_what: &'static str,
        close_what: &'static str,
        empty: impl Fn(Box<'a, EmptyStatement>) -> T,
        member: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<(Span, Vec<'a, T>)> {
        let open = self.expect(TokenKind::LBrace, open_what)?.span;
        let members = self.member_list(empty, member)?;
        let close = self.expect(TokenKind::RBrace, close_what)?.span;
        Ok((open.cover(close), members))
    }

    /// Parse members, leaving `}` or EOF for the caller.
    fn member_list<T>(
        &mut self,
        empty: impl Fn(Box<'a, EmptyStatement>) -> T,
        mut member: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<'a, T>> {
        let mut members = Vec::new_in(self);
        loop {
            if let Some(semi) = self.eat(TokenKind::Semicolon) {
                members.push(empty(EmptyStatement::boxed(semi.span, self)));
                continue;
            }
            if matches!(self.kind(), TokenKind::RBrace | TokenKind::Eof) {
                break;
            }
            match member(self) {
                Ok(parsed) => members.push(parsed),
                Err(error) if self.recovery_enabled => {
                    self.record_error(error);
                    self.recover_body_item(false);
                }
                Err(error) => return Err(error),
            }
        }
        Ok(members)
    }

    /// Parse a class body, which an enum constant or anonymous class also has.
    pub(crate) fn type_body(&mut self) -> Result<ClassBody<'a>> {
        let (span, members) = self.member_body(
            "`{` to open the type body",
            "`}` to close the type body",
            ClassMember::Empty,
            Self::class_member,
        )?;
        Ok(ClassBody::new(span, members, self))
    }

    /// Parse class members and compact constructors.
    fn record_body(&mut self) -> Result<ClassBody<'a>> {
        let (span, members) = self.member_body(
            "`{` to open the record body",
            "`}` to close the record body",
            ClassMember::Empty,
            Self::record_member,
        )?;
        Ok(ClassBody::new(span, members, self))
    }

    fn at_type_declaration_start(&self) -> bool {
        self.is_type_declaration_start_at(0)
    }

    pub(crate) fn is_type_declaration_start_at(&self, n: usize) -> bool {
        matches!(self.nth_kind(n), TokenKind::Class | TokenKind::Interface | TokenKind::Enum)
            || self.is_record_declaration_start_at(n)
            || (self.nth_kind(n) == TokenKind::At && self.nth_kind(n + 1) == TokenKind::Interface)
    }

    fn is_record_declaration_start_at(&self, n: usize) -> bool {
        if self.nth_kind(n) != TokenKind::Record || !self.nth_kind(n + 1).is_identifier() {
            return false;
        }
        let mut after_name = n + 2;
        if self.nth_kind(after_name) == TokenKind::Lt {
            let Some(after_params) = self.balanced_type_parameter_list_end_from(after_name) else {
                return false;
            };
            after_name = after_params;
        }
        self.nth_kind(after_name) == TokenKind::LParen
    }

    fn balanced_type_parameter_list_end_from(&self, mut n: usize) -> Option<usize> {
        debug_assert_eq!(self.nth_kind(n), TokenKind::Lt);
        let mut depth = 0usize;
        loop {
            match self.nth_kind(n) {
                TokenKind::Lt => depth += 1,
                TokenKind::Gt => {
                    depth -= 1;
                    n += 1;
                    if depth == 0 {
                        return Some(n);
                    }
                    continue;
                }
                TokenKind::Eof => return None,
                _ => {}
            }
            n += 1;
        }
    }

    pub(crate) fn at_local_type_declaration_start(&self) -> bool {
        if self.at_type_declaration_start_with_modifiers() {
            return true;
        }
        let n = self.modifier_list_end_from(0);
        matches!(self.nth_kind(n), TokenKind::Class | TokenKind::Interface | TokenKind::Enum)
            || self.is_record_declaration_start_at(n)
    }

    /// Parse a local type, excluding annotation interfaces (JLS §14.3).
    pub(crate) fn local_type_declaration_rest(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
    ) -> Result<LocalClassOrInterfaceDeclaration<'a>> {
        match self.kind() {
            TokenKind::Class => {
                Ok(LocalClassOrInterfaceDeclaration::Class(self.class_declaration(modifiers)?))
            }
            TokenKind::Interface => Ok(LocalClassOrInterfaceDeclaration::Interface(
                self.interface_declaration(modifiers)?,
            )),
            TokenKind::Enum => {
                Ok(LocalClassOrInterfaceDeclaration::Enum(self.enum_declaration(modifiers)?))
            }
            TokenKind::Record => {
                Ok(LocalClassOrInterfaceDeclaration::Record(self.record_declaration(modifiers)?))
            }
            _ => Err(diagnostics::message(
                "expected a local class, enum, record, or interface declaration",
                self.cur().span,
            )),
        }
    }

    fn class_member(&mut self) -> Result<ClassMember<'a>> {
        self.class_like_member(false)
    }

    fn record_member(&mut self) -> Result<ClassMember<'a>> {
        self.class_like_member(true)
    }

    fn class_like_member(&mut self, allow_compact_constructor: bool) -> Result<ClassMember<'a>> {
        let start = self.cur().span.start;
        // Initializer blocks have no declarator (JLS §8.6, §8.7).
        if self.kind() == TokenKind::LBrace
            || (self.kind() == TokenKind::Static && self.nth_kind(1) == TokenKind::LBrace)
        {
            return Ok(ClassMember::Initializer(self.initializer_block()?));
        }
        if self.at_type_declaration_start_with_modifiers() {
            let modifiers = self.type_modifiers()?;
            return Ok(ClassMember::from(self.type_declaration_rest(modifiers)?));
        }
        let modifiers = self.modifiers()?;
        // Member types start with their declaration keyword.
        if self.at_type_declaration_start() {
            return Ok(ClassMember::from(self.type_declaration_rest(modifiers)?));
        }
        if allow_compact_constructor
            && self.kind().is_identifier()
            && self.nth_kind(1) == TokenKind::LBrace
        {
            return Ok(ClassMember::CompactConstructor(self.compact_constructor_rest(modifiers)?));
        }
        let type_params = self.type_parameters()?;
        // Constructors omit the return type before `(`.
        if self.kind().is_identifier() && self.nth_kind(1) == TokenKind::LParen {
            let ctor = self.constructor_rest(start, modifiers, type_params)?;
            return Ok(ClassMember::Constructor(ctor));
        }
        match self.field_or_method_tail(
            start,
            modifiers,
            type_params,
            "a member name",
            "a field name",
            "type parameters are only allowed on methods and constructors",
        )? {
            FieldOrMethodTail::Field(field) => Ok(ClassMember::Field(field)),
            FieldOrMethodTail::Method(method) => Ok(ClassMember::Method(method)),
        }
    }

    fn field_or_method_tail(
        &mut self,
        start: u32,
        modifiers: Vec<'a, Modifier<'a>>,
        type_params: Option<Box<'a, TypeParameters<'a>>>,
        name_context: &'static str,
        underscore_name_context: &'static str,
        type_params_on_field_message: &'static str,
    ) -> Result<FieldOrMethodTail<'a>> {
        let ty = self.method_result()?;
        if self.kind() == TokenKind::Underscore {
            let name = self.variable_declarator_id(underscore_name_context)?;
            let ty = Self::field_type(ty)?;
            Self::reject_field_type_params(type_params.as_deref(), type_params_on_field_message)?;
            return Ok(FieldOrMethodTail::Field(self.field_rest(modifiers, ty, name)?));
        }

        let name = self.binding_identifier(name_context)?;
        if self.kind() == TokenKind::LParen {
            let method = self.method_rest(start, modifiers, type_params, ty, name)?;
            Ok(FieldOrMethodTail::Method(method))
        } else {
            let ty = Self::field_type(ty)?;
            Self::reject_field_type_params(type_params.as_deref(), type_params_on_field_message)?;
            Ok(FieldOrMethodTail::Field(self.field_rest(modifiers, ty, name)?))
        }
    }

    fn field_type(ty: MethodResult<'a>) -> Result<Type<'a>> {
        let span = ty.span();
        Type::try_from(ty).map_err(|()| {
            diagnostics::message("`void` is only valid as a method return type", span)
        })
    }

    fn field_or_annotation_element_tail(
        &mut self,
        modifiers: Vec<'a, Modifier<'a>>,
        ty: Type<'a>,
        name_context: &'static str,
    ) -> Result<FieldOrAnnotationElementTail<'a>> {
        let name = self.binding_identifier(name_context)?;
        if self.kind() == TokenKind::LParen {
            let element = self.annotation_interface_element_rest(modifiers, ty, name)?;
            Ok(FieldOrAnnotationElementTail::Element(element))
        } else {
            Ok(FieldOrAnnotationElementTail::Field(self.field_rest(modifiers, ty, name)?))
        }
    }

    fn reject_field_type_params(
        type_params: Option<&TypeParameters<'a>>,
        message: &'static str,
    ) -> Result<()> {
        if let Some(first) = type_params.and_then(|list| list.params.first()) {
            Err(diagnostics::message(message, first.span))
        } else {
            Ok(())
        }
    }

    /// Parse `{ … }` or `static { … }`.
    fn initializer_block(&mut self) -> Result<Box<'a, InitializerBlock<'a>>> {
        let start = self.cur().span;
        let is_static = self.eat(TokenKind::Static).is_some();
        let block = self.block()?;
        Ok(InitializerBlock::boxed(start.cover(block.span), is_static, block, self))
    }
}
