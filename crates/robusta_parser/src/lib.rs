// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_parser/src/lib.rs

//! Recursive-descent parser for Java 25 syntax.
//!
//! Consumes [`robusta_lexer`] tokens into an arena-allocated [`robusta_ast`] tree.
//! Syntax errors recover at declaration, member, statement, and selected-list
//! boundaries. Reject results with diagnostics before formatting or linting.
//!
//! # Examples
//!
//! <https://github.com/robusta-rs/robusta/blob/main/crates/robusta_parser/examples/parser.rs>
//!
//! ```rust,ignore
#![doc = include_str!("../examples/parser.rs")]
//! ```
//!
//! # Visitor
//!
//! <https://github.com/robusta-rs/robusta/blob/main/crates/robusta_parser/examples/visitor.rs>

use robusta_allocator::{Allocator, Box, GetAllocator, Vec};
use robusta_ast::{
    Annotation, AnnotationArgument, AnnotationInterfaceBody, AnnotationInterfaceDeclaration,
    AnnotationInterfaceElementDeclaration, AnnotationInterfaceMemberDeclaration, ArrayDimension,
    ArrayInitializer, ArrayTypeDimension, BindingIdentifier, Block, Callee, CatchClause, ClassBody,
    ClassDeclaration, ClassMember, ClassType, Comment, CompactCompilationUnit,
    CompactConstructorDeclaration, CompilationUnit, CompilationUnitContent, ComponentPattern,
    ConstructorDeclaration, ConstructorInvocation, ConstructorInvocationKind, ElementValue,
    ElementValueArrayInitializer, ElementValuePair, EmptyDeclaration, EnumBody, EnumConstant,
    EnumDeclaration, Expression, FieldDeclaration, ForInit, FormalParameter, FormalParameters,
    IdentifierName, IdentifierReference, ImportDeclaration, ImportKind, InitializerBlock,
    InstanceofRhs, InterfaceBody, InterfaceDeclaration, InterfaceMember, LabelIdentifier,
    LambdaBody, LocalClassOrInterfaceDeclaration, LocalVariableDeclaration, MethodDeclaration,
    MethodReferenceTarget, MethodResult, Modifier, ModifierKind, ModuleDeclaration,
    ModuleDirective, Name, PackageDeclaration, PackageDirectiveKind, Pattern, PrimitiveTypeKind,
    ReceiverParameter, RecordComponent, RecordDeclaration, RecordHeader, RecordPattern,
    RequiresModifier, RequiresModifierKind, Resource, Statement, StatementExpressionList,
    SwitchCase, SwitchLabel, SwitchRuleBody, Type, TypeArgument, TypeArguments, TypeDeclaration,
    TypeName, TypeParameter, TypeParameters, TypePattern, VariableDeclarator, VariableInitializer,
    VariableType, WildcardBound, WildcardBoundKind,
    builder::{AstBuilder, GetAstBuilder},
};
use robusta_diagnostics::RobustaDiagnostic;
use robusta_lexer::{Token, TokenKind};
use robusta_span::{GetSpan, Span};
use robusta_syntax::{
    operator::{AssignOp, BinaryOp, UnaryOp},
    precedence::{GetPrecedence, Precedence},
};

mod ast_builder;
mod cursor;
mod error_handler;
mod modifiers;

mod java;

mod diagnostics;

use ast_builder::ParserAstBuilder;
use diagnostics::{ParserDiagnostic, Result};

pub struct ParseResult<'a> {
    /// Whether parsing was clean, recovered, or fatal.
    pub status: ParseStatus,
    /// The tree, owning comments in source order. Fatal results carry an
    /// empty unit. Check `status` and `errors` before using it.
    pub unit: CompilationUnit<'a>,
    /// Lexical and syntax errors, in source order.
    pub errors: std::vec::Vec<RobustaDiagnostic>,
}

/// Parser outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseStatus {
    /// A complete tree without errors.
    Clean,
    /// A structurally valid tree retained after errors.
    Recovered,
    /// No structurally valid compilation unit could be parsed.
    Fatal,
}

/// Parse into an arena-allocated unit owning its comments.
///
/// Sources exceeding [`robusta_lexer::MAX_LEN`] return a fatal result with
/// an empty unit and lexer diagnostic. Reject results with errors
/// before formatting or linting. A fatal unit has span `0..0`.
pub fn parse<'a>(allocator: &'a Allocator, source: &'a str) -> ParseResult<'a> {
    let lexed = robusta_lexer::tokenize(source);
    if source.len() > robusta_lexer::MAX_LEN {
        return ParseResult {
            status: ParseStatus::Fatal,
            unit: empty_unit(allocator, &lexed.comments),
            errors: lexed.errors,
        };
    }
    let lexical_error_spans = lexed.errors.iter().map(RobustaDiagnostic::primary_span).collect();
    let mut parser = Parser {
        ast: ParserAstBuilder::new(allocator),
        source,
        tokens: lexed.tokens,
        identifier_names: lexed
            .identifier_names
            .into_iter()
            .map(|(start, name)| {
                let name: &'a str = allocator.alloc_str(&name);
                (start, name)
            })
            .collect(),
        comments: lexed.comments,
        idx: 0,
        errors: lexed.errors.into_iter().map(ParserDiagnostic::eager).collect(),
        lexical_error_spans,
        recovery_enabled: true,
        in_case_label: false,
    };
    let unit = match parser.compilation_unit() {
        Ok(unit) => Some(unit),
        Err(diagnostic) => {
            parser.record_error(diagnostic);
            None
        }
    };
    let mut errors = std::mem::take(&mut parser.errors)
        .into_iter()
        .map(ParserDiagnostic::into_diagnostic)
        .collect::<std::vec::Vec<_>>();
    errors.sort_by_key(|error| {
        let span = error.primary_span();
        (span.start, span.end)
    });
    let status = match (unit.is_some(), errors.is_empty()) {
        (true, true) => ParseStatus::Clean,
        (true, false) => ParseStatus::Recovered,
        (false, _) => ParseStatus::Fatal,
    };
    let unit = unit.unwrap_or_else(|| empty_unit(allocator, &parser.comments));
    ParseResult { status, unit, errors }
}

/// Build an empty fatal unit retaining lexed comments.
fn empty_unit<'a>(allocator: &'a Allocator, comments: &[Comment]) -> CompilationUnit<'a> {
    let comments = Vec::from_iter_in(comments.iter().copied(), &allocator);
    let content = CompilationUnitContent::new_ordinary_compilation_unit(
        Span::empty(0),
        None,
        [],
        [],
        &ParserAstBuilder::new(allocator),
    );
    CompilationUnit::new(Span::new(0, 0), comments, content, &AstBuilder::new(allocator))
}

struct Parser<'a> {
    ast: ParserAstBuilder<'a>,
    source: &'a str,
    tokens: std::vec::Vec<Token>,
    /// Decoded names use the arena. Ordinary names borrow `source`.
    identifier_names: rustc_hash::FxHashMap<u32, &'a str>,
    /// Lexed comments for the compilation unit.
    comments: std::vec::Vec<Comment>,
    idx: usize,
    errors: std::vec::Vec<ParserDiagnostic>,
    lexical_error_spans: std::vec::Vec<Span>,
    recovery_enabled: bool,
    /// Treat `->` as a rule arrow (JLS §14.11.1).
    /// [`Parser::expression`] clears this for nested expressions.
    in_case_label: bool,
}

struct OrdinaryOrCompactUnit<'a> {
    compact: Option<CompactCompilationUnit<'a>>,
    types: Vec<'a, TypeDeclaration<'a>>,
}

impl<'a> Parser<'a> {
    fn compilation_unit(&mut self) -> Result<CompilationUnit<'a>> {
        // Leading annotations may belong to `package` (JLS §7.4.1).
        let package = if self.kind() == TokenKind::Package
            || (self.kind() == TokenKind::At
                && self.nth_kind(self.annotations_end_from(0)) == TokenKind::Package)
        {
            match self.package_declaration() {
                Ok(package) => Some(package),
                Err(error) if self.recovery_enabled => {
                    self.record_error(error);
                    self.recover_body_item(false);
                    None
                }
                Err(error) => return Err(error),
            }
        } else {
            None
        };

        let mut imports = Vec::new_in(self);
        while self.kind() == TokenKind::Import {
            match self.import_declaration() {
                Ok(import) => imports.push(import),
                Err(error) if self.recovery_enabled => {
                    self.record_error(error);
                    self.recover_body_item(false);
                }
                Err(error) => return Err(error),
            }
        }

        let content = if package.is_none() && self.at_module_declaration() {
            // Modular units contain only imports and a module (JLS §7.3/§7.7).
            let module = self.module_declaration()?;
            if self.kind() != TokenKind::Eof {
                return Err(diagnostics::message(
                    "expected end of file after the module declaration",
                    self.cur().span,
                ));
            }
            let span = imports.first().map_or(module.span, |import| import.span.cover(module.span));
            CompilationUnitContent::new_modular_compilation_unit(span, imports, module, self)
        } else if package.is_none() {
            let unit = self.ordinary_or_compact_compilation_unit()?;
            match unit.compact {
                Some(mut compact) => {
                    compact.span = imports
                        .first()
                        .map_or(compact.span, |import| import.span.cover(compact.span));
                    compact.imports = imports;
                    CompilationUnitContent::Compact(compact)
                }
                None => self.ordinary_content(None, imports, unit.types),
            }
        } else {
            let types = self.ordinary_type_declarations()?;
            self.ordinary_content(package, imports, types)
        };

        // Cannot overflow `u32`: `parse` rejects sources longer than `MAX_LEN`.
        #[expect(clippy::cast_possible_truncation)]
        let end = self.source.len() as u32;
        let comments = Vec::from_iter_in(self.comments.iter().copied(), self);
        Ok(CompilationUnit::new(
            Span::new(0, end),
            comments,
            content,
            // The default builder assigns ID zero (`NodeId::ROOT`) to the unit.
            // `ParserAstBuilder` starts at one, keeping every child ID distinct.
            &AstBuilder::new(self.allocator()),
        ))
    }

    /// Build ordinary content with a span excluding surrounding trivia.
    fn ordinary_content(
        &self,
        package: Option<PackageDeclaration<'a>>,
        imports: Vec<'a, ImportDeclaration<'a>>,
        types: Vec<'a, TypeDeclaration<'a>>,
    ) -> CompilationUnitContent<'a> {
        let start = package
            .as_ref()
            .map(|package| package.span)
            .or_else(|| imports.first().map(|import| import.span))
            .or_else(|| types.first().map(GetSpan::span));
        let span = start.map_or_else(
            || Span::empty(0),
            |start| {
                let end = types
                    .last()
                    .map(GetSpan::span)
                    .or_else(|| imports.last().map(|import| import.span))
                    .or_else(|| package.as_ref().map(|package| package.span))
                    .unwrap_or(start);
                start.cover(end)
            },
        );
        CompilationUnitContent::new_ordinary_compilation_unit(span, package, imports, types, self)
    }

    fn ordinary_or_compact_compilation_unit(&mut self) -> Result<OrdinaryOrCompactUnit<'a>> {
        let unit_start = self.idx;
        let recovery_enabled = self.recovery_enabled;
        self.recovery_enabled = false;
        let starts_like_ordinary =
            self.kind() == TokenKind::Semicolon || self.at_type_declaration_start_with_modifiers();
        match self.ordinary_type_declarations() {
            Ok(types) => {
                self.recovery_enabled = recovery_enabled;
                Ok(OrdinaryOrCompactUnit { compact: None, types })
            }
            Err(ordinary_error) => {
                let ordinary_error_idx = self.idx;
                self.idx = unit_start;
                match self.compact_compilation_unit() {
                    Ok(compact) => {
                        self.recovery_enabled = recovery_enabled;
                        Ok(OrdinaryOrCompactUnit {
                            compact: Some(compact),
                            types: Vec::new_in(self),
                        })
                    }
                    Err(compact_error) => {
                        let compact_error_idx = self.idx;
                        // Prefer the parse that reached farther. Break ties by the source start.
                        let ordinary_wins = ordinary_error_idx > compact_error_idx
                            || (ordinary_error_idx == compact_error_idx && starts_like_ordinary);
                        self.idx = unit_start;
                        self.recovery_enabled = recovery_enabled;
                        if !recovery_enabled {
                            return if ordinary_wins {
                                Err(ordinary_error)
                            } else {
                                Err(compact_error)
                            };
                        }
                        if ordinary_wins {
                            self.ordinary_type_declarations()
                                .map(|types| OrdinaryOrCompactUnit { compact: None, types })
                        } else {
                            self.compact_compilation_unit().map(|compact| OrdinaryOrCompactUnit {
                                compact: Some(compact),
                                types: Vec::new_in(self),
                            })
                        }
                    }
                }
            }
        }
    }

    fn ordinary_type_declarations(&mut self) -> Result<Vec<'a, TypeDeclaration<'a>>> {
        let mut types = Vec::new_in(self);
        loop {
            // A lone `;` is an empty declaration (JLS §7.6).
            if let Some(semi) = self.eat(TokenKind::Semicolon) {
                types.push(TypeDeclaration::new_empty_declaration(semi.span, self));
                continue;
            }
            if self.kind() == TokenKind::Eof {
                break;
            }
            match self.type_declaration() {
                Ok(ty) => types.push(ty),
                Err(error) if self.recovery_enabled => {
                    if self.recover_top_level_declaration() {
                        self.record_error(error);
                    } else {
                        return Err(error);
                    }
                }
                Err(error) => return Err(error),
            }
        }
        Ok(types)
    }

    /// Parse a package, import, or module name, which is never looked up.
    fn name(&mut self) -> Result<Name<'a>> {
        let first = self.identifier_name("a name")?;
        let mut name = Name::Identifier(self.alloc(first));
        // Leave `.*` for on-demand imports.
        while self.kind() == TokenKind::Dot && self.nth_kind(1).is_identifier() {
            self.bump();
            let last = self.identifier_name("a name")?;
            name = Name::new_qualified_name(name.span().cover(last.span), name, last, self);
        }
        Ok(name)
    }

    /// Parse the name of a type, whose first name is looked up.
    fn type_name(&mut self) -> Result<TypeName<'a>> {
        let first = self.identifier_reference("a name")?;
        let mut name = TypeName::Identifier(self.alloc(first));
        while self.kind() == TokenKind::Dot && self.nth_kind(1).is_identifier() {
            self.bump();
            let last = self.identifier_name("a name")?;
            name =
                TypeName::new_qualified_type_name(name.span().cover(last.span), name, last, self);
        }
        Ok(name)
    }

    /// Parse post-name dimensions, rejecting them after `_` (JLS §8.3).
    fn variable_declarator_dims(
        &mut self,
        name: &BindingIdentifier<'a>,
    ) -> Result<Vec<'a, ArrayTypeDimension<'a>>> {
        let dims = self.array_type_dimensions()?;
        if let Some(first) = dims.first().filter(|_| name.name == "_") {
            return Err(diagnostics::message(
                "`_` cannot be followed by array dimensions",
                first.span,
            ));
        }
        Ok(dims)
    }

    /// Leave forbidden post-name varargs dimensions for caller diagnostics (JLS §8.4.1).
    fn parameter_dims(
        &mut self,
        varargs: bool,
        name: &BindingIdentifier<'a>,
    ) -> Result<Vec<'a, ArrayTypeDimension<'a>>> {
        if varargs { Ok(Vec::new_in(self)) } else { self.variable_declarator_dims(name) }
    }

    /// `var` forbids array dimensions (JLS §14.4).
    fn reject_var_dims(ty: &VariableType<'a>, dims: &[ArrayTypeDimension<'a>]) -> Result<()> {
        match (ty, dims.first()) {
            (VariableType::Var(_), Some(first)) => Err(diagnostics::message_with_help(
                "`var` is not allowed as an array element type",
                first.span,
                "Use an explicit array type.",
            )),
            _ => Ok(()),
        }
    }

    #[inline]
    fn alloc<T>(&self, value: T) -> Box<'a, T> {
        Box::new_in(value, self)
    }
}

impl<'a> GetAllocator<'a> for Parser<'a> {
    #[inline]
    fn allocator(&self) -> &'a Allocator {
        self.ast.allocator()
    }
}

impl<'a> GetAstBuilder<'a> for Parser<'a> {
    type Builder = ParserAstBuilder<'a>;

    #[inline]
    fn builder(&self) -> &ParserAstBuilder<'a> {
        &self.ast
    }
}

#[cfg(test)]
mod tests;
