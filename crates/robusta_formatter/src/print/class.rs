use crate::ast_nodes::AstNode;
use crate::ast_nodes::impls::ast_nodes::format_node_trailing_comments;
use crate::formatter::prelude::*;
use crate::formatter::separated::argument_list_end;
use crate::formatter::trivia::{
    documentation_comment_ends_line, format_documentation_comments_before, format_leading_comments,
    format_trailing_comments,
};
use crate::print::FormatWrite;
use crate::print::annotations::{format_modifiers_inline, format_modifiers_stacked};
use crate::print::call_like_expression::arguments::format_arguments;
use crate::print::parameters::format_record_header;
use crate::print::semicolon::FormatContentWithSemicolon;
use crate::print::type_parameters::format_type_declaration_parameters;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::{
    Body, format_braced_body, format_head_body, write_comments_before_opener,
    write_comments_between_blocks, write_same_line_comments_before_opener,
};
use robusta_allocator::Vec;
use robusta_ast::{
    AnnotationInterfaceBody, AnnotationInterfaceDeclaration, AnnotationInterfaceMemberDeclaration,
    BindingIdentifier, ClassBody, ClassDeclaration, ClassMember, Comment, EmptyDeclaration,
    EnumBody, EnumConstant, EnumDeclaration, InitializerBlock, InterfaceBody, InterfaceDeclaration,
    InterfaceMember, Modifier, RecordDeclaration, RecordHeader, Type, TypeName, TypeParameters,
};
use robusta_formatter_core::{GroupId, format_args, write};
use robusta_span::{GetSpan, Span};
use std::cell::Cell;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, EmptyDeclaration> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        ";".fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ClassDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let clauses = [
            FormatDeclarationClause::superclass(self.superclass()),
            FormatDeclarationClause::types("implements", self.interfaces()),
            FormatDeclarationClause::names("permits", self.permits()),
        ];
        FormatTypeHead::new("class", self.modifiers(), self.name(), self.type_params())
            .format_header(f, &clauses, None, &node.body);
        format_head_body(&self.body(), f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, ClassBody<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_type_body(f, self.span, &self.members());
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, InterfaceDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let clauses = [
            FormatDeclarationClause::types("extends", self.extends()),
            FormatDeclarationClause::names("permits", self.permits()),
        ];
        FormatTypeHead::new("interface", self.modifiers(), self.name(), self.type_params())
            .format_header(f, &clauses, None, &node.body);
        format_head_body(&self.body(), f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, InterfaceBody<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_type_body(f, self.span, &self.members());
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, RecordDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let clauses = [FormatDeclarationClause::types("implements", self.interfaces())];
        FormatTypeHead::new("record", self.modifiers(), self.name(), self.type_params())
            .format_header(f, &clauses, Some(self.header()), &node.body);
        format_head_body(&self.body(), f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, AnnotationInterfaceDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        FormatTypeHead::new("@interface", self.modifiers(), self.name(), None)
            .format_name(f, false);
        format_head_body(&self.body(), f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, AnnotationInterfaceBody<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_type_body(f, self.span, &self.members());
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, EnumDeclaration<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let clauses = [FormatDeclarationClause::types("implements", self.interfaces())];
        FormatTypeHead::new("enum", self.modifiers(), self.name(), None)
            .format_header(f, &clauses, None, &node.body);
        format_head_body(&self.body(), f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, EnumBody<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let body_semicolon = node.has_body_semicolon.then(|| {
            let constants_end = node.constants.last().map_or_else(
                || f.comments().position_after_character(node.span.start, b'{'),
                |constant| constant.span().end,
            );
            let end = f.comments().position_after_character(constants_end, b';');
            Span::new(end - 1, end)
        });
        let first_member = node.members.first().map_or(0, |member| member.span().start);
        format_braced_body(f, node.is_empty(), node.span, |f| {
            write_blank_line_at_body_start(node.span, f);
            let mut join = f.join_nodes_with_hardline();
            let constants = self.constants();
            for (index, constant) in constants.iter().enumerate() {
                let is_last = index + 1 == constants.len();
                let entry = format_with(|f| match body_semicolon {
                    // The constant's trailing comments follow the body's `;`.
                    Some(semicolon) if is_last => {
                        FormatContentWithSemicolon::new(
                            &constant,
                            constant.span().end,
                            semicolon.end,
                        )
                        .fmt(f);
                        format_node_trailing_comments(
                            node.span,
                            Span::new(constant.span().start, semicolon.end),
                            first_member,
                            true,
                            f,
                        );
                    }
                    _ if is_last => write!(f, [constant, if_group_breaks(&token(","))]),
                    _ => {
                        constant.fmt(f);
                        ",".fmt(f);
                    }
                });
                join.entry(constant.span(), &entry);
            }

            // The enum body's `;` has no node to carry its comments.
            if node.constants.is_empty()
                && let Some(semicolon) = body_semicolon
            {
                let entry = format_with(|f| {
                    format_leading_comments(semicolon).fmt(f);
                    ";".fmt(f);
                    format_trailing_comments(node.span, semicolon, first_member).fmt(f);
                });
                join.entry(semicolon, &entry);
            }

            join_type_members(&mut join, &self.members());
        });
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, EnumConstant<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        format_modifiers_inline(f, &self.modifiers());
        format_content_without_comments_after(&self.name(), node.name.span.end, f);
        if let Some(arguments) = self.args() {
            write_comments_before_opener(node.name.span.end, b'(', f);
            let close_end = argument_list_end(f, &arguments, node.name.span.end);
            format_arguments(f, &arguments, close_end);
        }
        if let Some(body) = self.body() {
            format_head_body(&body, f);
        }
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, InitializerBlock<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        if self.is_static {
            "static".fmt(f);
            format_head_body(&self.block(), f);
        } else {
            self.block().fmt(f);
        }
    }
}

struct FormatTypeHead<'parent, 'ast> {
    keyword: &'static str,
    modifiers: AstNode<'parent, 'ast, Vec<'ast, Modifier<'ast>>>,
    name: AstNode<'parent, 'ast, BindingIdentifier<'ast>>,
    type_params: Option<AstNode<'parent, 'ast, TypeParameters<'ast>>>,
}

impl<'parent, 'ast> FormatTypeHead<'parent, 'ast> {
    fn new(
        keyword: &'static str,
        modifiers: AstNode<'parent, 'ast, Vec<'ast, Modifier<'ast>>>,
        name: AstNode<'parent, 'ast, BindingIdentifier<'ast>>,
        type_params: Option<AstNode<'parent, 'ast, TypeParameters<'ast>>>,
    ) -> Self {
        Self { keyword, modifiers, name, type_params }
    }

    fn format_name(&self, f: &mut JavaFormatter<'_, 'ast>, has_clauses: bool) {
        format_modifiers_stacked(f, &self.modifiers);
        write!(f, [self.keyword, space()]);
        let name_end = self.name.span.end;
        if self.type_params.is_some() {
            format_content_without_comments_after(&self.name, name_end, f);
            write_same_line_comments_before_opener(name_end, b'<', f);
        } else if has_clauses {
            self.name.fmt(f);
        } else {
            format_content_without_comments_after(&self.name, name_end, f);
        }
    }

    fn format_header(
        &self,
        f: &mut JavaFormatter<'_, 'ast>,
        clauses: &[FormatDeclarationClause<'_, 'ast>],
        record_header: Option<AstNode<'_, 'ast, RecordHeader<'ast>>>,
        body: &impl Body,
    ) {
        let empty_body = body.is_empty();
        let body_starts_with_declaration = body.starts_with_declaration();
        let name_end = self.name.span.end;
        let params_end = self.type_params.as_ref().map_or(name_end, |params| params.span.end);
        let head_end = record_header.as_ref().map_or(params_end, |header| header.span.end);
        let first_clause = clauses.iter().find_map(|clause| clause.entries.first_start());
        let has_clauses = first_clause.is_some();
        let before_params = self
            .type_params
            .as_ref()
            .map_or(&[][..], |params| f.comments().comments_in_range(name_end, params.span.start));
        let has_documentation = before_params.iter().any(Comment::is_documentation);
        // An empty body could leave the comment before the next member.
        let keep_javadoc = record_header.is_some()
            || empty_body && !has_clauses
            || (before_params.len() > 1 && has_documentation);
        // Without a clause, the comments after the head are the body's to place.
        let after_params = match first_clause {
            Some(first_clause) if record_header.is_none() => {
                f.comments().comments_in_range(head_end, first_clause)
            }
            _ => &[],
        };
        let name_has_suffix = has_trailing_line_comment(before_params);
        let params_have_suffix = has_trailing_line_comment(after_params);
        // Without a clause, a line comment after the name rides past the `{`, unless it
        // would come to document the body's first declaration.
        let grouped = record_header.is_none()
            && (has_clauses || has_documentation && body_starts_with_declaration)
            && (name_has_suffix || params_have_suffix);
        let heritage = grouped.then(|| f.state().group_id("heritage"));
        // After a `///` comment that ends the name's line, a `<` that starts the
        // next line is indented twice, as a moved method name is.
        let indent_head = documentation_comment_ends_line(before_params);
        let header = format_once(|f| {
            self.format_name(f, has_clauses && record_header.is_none());
            format_type_clauses(f, head_end, clauses, heritage, |f| {
                let mut components = None;
                let head = format_once(|f| {
                    if keep_javadoc && let Some(params) = &self.type_params {
                        format_documentation_comments_before(name_end, params.span.start, f);
                    }
                    if let Some(params) = &self.type_params {
                        let parameters = format_once(|f| {
                            format_type_declaration_parameters(f, params, has_clauses);
                        });
                        // Keep an inline block with the name's line comment.
                        if name_has_suffix
                            && (has_documentation && keep_javadoc
                                || before_params.first().is_some_and(Comment::is_block))
                        {
                            write!(f, [line_suffix_boundary(), parameters]);
                        } else if name_has_suffix {
                            write!(f, [group(&format_args!(soft_line_break(), parameters))]);
                        } else {
                            parameters.fmt(f);
                        }
                        if params_have_suffix {
                            params.format_trailing_comments(f);
                        }
                    }
                    components = record_header.as_ref().and_then(|header| {
                        if name_has_suffix {
                            line_suffix_boundary().fmt(f);
                        }
                        format_record_header(f, header)
                    });
                });
                if indent_head {
                    wrapped_indent(true, &head).fmt(f);
                } else {
                    head.fmt(f);
                }
                components
            });
        });
        if let Some(heritage) = heritage {
            group(&header).with_group_id(Some(heritage)).fmt(f);
            // Without a clause, the header is grouped only to keep a documentation
            // comment before the `{`, which then starts the next line.
            if !has_clauses {
                if_group_breaks(&hard_line_break()).with_group_id(Some(heritage)).fmt(f);
            }
        } else {
            header.fmt(f);
        }
    }
}

fn has_trailing_line_comment(comments: &[Comment]) -> bool {
    comments.iter().take_while(|comment| !comment.preceded_by_newline()).any(Comment::is_line)
}

#[derive(Clone, Copy)]
enum FormatDeclarationEntries<'parent, 'ast> {
    Types(AstNode<'parent, 'ast, Vec<'ast, Type<'ast>>>),
    Superclass(Option<AstNode<'parent, 'ast, Type<'ast>>>),
    Names(AstNode<'parent, 'ast, Vec<'ast, TypeName<'ast>>>),
}

impl FormatDeclarationEntries<'_, '_> {
    fn len(self) -> usize {
        match self {
            Self::Types(entries) => entries.len(),
            Self::Superclass(entry) => usize::from(entry.is_some()),
            Self::Names(entries) => entries.len(),
        }
    }

    fn first_start(self) -> Option<u32> {
        match self {
            Self::Types(entries) => entries.first().map(|entry| entry.span().start),
            Self::Superclass(entry) => entry.map(|entry| entry.span().start),
            Self::Names(entries) => entries.first().map(|entry| entry.span().start),
        }
    }

    /// Where the last named type ends, which starts the gap after its clause.
    fn last_end(self) -> Option<u32> {
        match self {
            Self::Types(entries) => entries.last().map(|entry| entry.span().end),
            Self::Superclass(entry) => entry.map(|entry| entry.span().end),
            Self::Names(entries) => entries.last().map(|entry| entry.span().end),
        }
    }
}

#[derive(Clone, Copy)]
pub struct FormatDeclarationClause<'parent, 'ast> {
    keyword: &'static str,
    entries: FormatDeclarationEntries<'parent, 'ast>,
}

impl<'parent, 'ast> FormatDeclarationClause<'parent, 'ast> {
    pub const fn types(
        keyword: &'static str,
        entries: AstNode<'parent, 'ast, Vec<'ast, Type<'ast>>>,
    ) -> Self {
        Self { keyword, entries: FormatDeclarationEntries::Types(entries) }
    }

    const fn superclass(entry: Option<AstNode<'parent, 'ast, Type<'ast>>>) -> Self {
        Self { keyword: "extends", entries: FormatDeclarationEntries::Superclass(entry) }
    }

    const fn names(
        keyword: &'static str,
        entries: AstNode<'parent, 'ast, Vec<'ast, TypeName<'ast>>>,
    ) -> Self {
        Self { keyword, entries: FormatDeclarationEntries::Names(entries) }
    }
}

/// Format what follows a method or constructor's name: `format_head`'s `(…)`
/// list, then its `throws` clause.
///
/// `head_end` is where what `format_head` prints ends. The clause's comment
/// gap starts there, so a comment on the `)` line trails it however many lines
/// the list spans. `format_head` returns the list's group.
///
/// Returns where the header ends, past its clause.
pub fn format_declaration_header<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    head_end: u32,
    clause: FormatDeclarationClause<'_, 'ast>,
    format_head: impl FnOnce(&mut JavaFormatter<'_, 'ast>) -> Option<GroupId>,
) -> u32 {
    let clause_end = clause.entries.last_end();
    let content = format_once(|f| {
        let list = format_head(f);
        if clause_end.is_some() {
            format_declaration_clause(f, head_end, list, clause);
        }
    });
    if clause_end.is_some() {
        write!(f, [group(&content)]);
    } else {
        content.fmt(f);
    }
    clause_end.unwrap_or(head_end)
}

/// Format what follows a type declaration's name: `format_head`'s type
/// parameters or record components, then the `extends`, `implements` and
/// `permits` clauses, filled as google-java-format fills them. A clause stays on
/// the line when it fits and starts a new one otherwise, as it does after what
/// split.
///
/// `head_end` is where what `format_head` prints ends, or the declared name
/// when it prints nothing. `format_head` returns the group of a record's
/// components.
fn format_type_clauses<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    head_end: u32,
    clauses: &[FormatDeclarationClause<'_, 'ast>],
    heritage: Option<GroupId>,
    format_head: impl FnOnce(&mut JavaFormatter<'_, 'ast>) -> Option<GroupId>,
) {
    let mut components = None;
    let mut previous_end = head_end;
    let mut filler = f.fill();
    filler.entry(&soft_line_break_or_space(), &format_once(|f| components = format_head(f)));
    for clause in clauses.iter().copied() {
        let Some(end) = clause.entries.last_end() else { continue };
        let owes_separator = Cell::new(true);
        let separator = format_with(|f| {
            owes_separator.set(format_clause_comments(f, previous_end, clause.keyword));
            if owes_separator.get() {
                let line = format_with(|f| {
                    if let Some(heritage) = heritage {
                        write!(
                            f,
                            [
                                if_group_breaks(&hard_line_break()).with_group_id(Some(heritage)),
                                if_group_fits_on_line(&soft_line_break_or_space())
                                    .with_group_id(Some(heritage))
                            ]
                        );
                    } else {
                        soft_line_break_or_space().fmt(f);
                    }
                });
                let line = wrapped_indent(true, &line);
                // After split components the clause writes its own separator.
                match components {
                    Some(group) => {
                        if_group_fits_on_line(&line).with_group_id(Some(group)).fmt(f);
                    }
                    None => line.fmt(f),
                }
            }
        });
        let entry =
            format_with(|f| format_type_clause(f, components, owes_separator.get(), clause));
        filler.entry(&separator, &entry);
        previous_end = end;
    }
    filler.finish();
}

/// One clause of a type declaration. After split `components`, the clause stays
/// on the `)` line, as a `throws` clause does.
fn format_type_clause<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    components: Option<GroupId>,
    owes_separator: bool,
    clause: FormatDeclarationClause<'_, 'ast>,
) {
    let Some(components) = components else {
        let entries = format_once(|f| format_clause_entries(f, clause.entries));
        let content = format_once(|f| format_clause_content(f, clause, &entries));
        group(&wrapped_indent(true, &content)).fmt(f);
        return;
    };

    f.context_mut().set_record_components_group(Some(components));
    let interned = f
        .intern(&format_once(|f| format_clause_entries(f, clause.entries)))
        .expect("a clause has entries");
    f.context_mut().set_record_components_group(None);
    let entries = format_with(|f| f.write_element(interned.clone()));
    let content = format_with(|f| format_clause_content(f, clause, &entries));
    let on_own_line = format_with(|f| group(&wrapped_indent(true, &content)).fmt(f));
    let after_split = format_with(|f| {
        if owes_separator {
            write!(f, [space(), group(&content)]);
        } else {
            on_own_line.fmt(f);
        }
    });
    write!(
        f,
        [
            if_group_breaks(&after_split).with_group_id(Some(components)),
            if_group_fits_on_line(&on_own_line).with_group_id(Some(components)),
        ]
    );
}

/// A clause's keyword and entries as google-java-format wraps them: the first
/// stays after the keyword and each other takes its own line. A first entry that
/// cannot fit there moves off the line, unless a comment after the keyword fixes
/// where it starts.
fn format_clause_content<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    clause: FormatDeclarationClause<'_, 'ast>,
    entries: &impl Format<'ast, JavaFormatContext<'ast>>,
) {
    clause.keyword.fmt(f);
    let single = clause.entries.len() == 1;
    let keyword_comment = clause
        .entries
        .first_start()
        .is_some_and(|first_start| f.comments().has_comment_before(first_start));
    if keyword_comment {
        space().fmt(f);
        if single {
            entries.fmt(f);
        } else {
            wrapped_indent(true, entries).fmt(f);
        }
        return;
    }
    let gap = f.state().group_id("clause_keyword");
    write!(
        f,
        [group(&wrapped_indent(true, &soft_line_break_or_space()))
            .with_group_id(Some(gap))
            .measure_flat()]
    );
    if single {
        indent_if_group_breaks(&indent_if_group_breaks(entries, gap), gap).fmt(f);
    } else {
        wrapped_indent(true, entries).fmt(f);
    }
}

/// The `throws` clause after a method or constructor's parameters.
fn format_declaration_clause<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    previous_end: u32,
    list: Option<GroupId>,
    clause: FormatDeclarationClause<'_, 'ast>,
) {
    let owes_separator = format_clause_comments(f, previous_end, clause.keyword);
    let entries = format_once(|f| format_clause_entries(f, clause.entries));
    let content = format_once(|f| format_clause_content(f, clause, &entries));
    let content = group(&content);
    if let Some(list) = list.filter(|_| owes_separator) {
        // A clause after a split list stays on the `)` line.
        let interned = f.intern(&content).expect("a clause has a keyword");
        let clause = format_with(|f| f.write_element(interned.clone()));
        write!(
            f,
            [
                if_group_breaks(&format_args!(space(), clause)).with_group_id(Some(list)),
                if_group_fits_on_line(&wrapped_indent(
                    true,
                    &format_args!(soft_line_break_or_space(), clause)
                ))
                .with_group_id(Some(list)),
            ]
        );
    } else {
        let separator = owes_separator.then_some(soft_line_break_or_space());
        write!(f, [wrapped_indent(true, &format_args!(separator, content))]);
    }
}

/// The comments before a clause's keyword. Their same-line half trails what
/// precedes the keyword, and an own-line comment leads it at the clause indent.
/// A comment after the keyword leads the first entry: `extends /* c */ E`.
///
/// Returns whether the clause still owes its separator.
fn format_clause_comments(f: &mut JavaFormatter<'_, '_>, previous_end: u32, keyword: &str) -> bool {
    // Lexical scan for the keyword's first byte: the gap holds only trivia and the keyword
    let keyword_start =
        f.comments().position_after_character(previous_end, keyword.as_bytes()[0]) - 1;
    let comments = f.comments().comments_before(keyword_start);
    let mut owes_separator = true;
    if !comments.is_empty() {
        write!(
            f,
            [wrapped_indent(
                true,
                &format_once(|f| {
                    owes_separator = write_comments_between_blocks(comments, f);
                })
            )]
        );
    }
    owes_separator
}

fn format_clause_entries<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    entries: FormatDeclarationEntries<'_, 'ast>,
) {
    for index in 0..entries.len() {
        if index > 0 {
            soft_line_break_or_space().fmt(f);
        }
        let has_next = index + 1 < entries.len();
        format_declaration_entry(f, entries, index, has_next);
        if has_next {
            ",".fmt(f);
        }
    }
}

/// One clause entry, carrying the two comment roles every list entry gets: a
/// comment written before the entry leads it, and one written after it trails
/// it. Without them a comment between two entries has no owner, so it surfaces
/// at the next claimed position and documents whatever the declaration prints
/// there.
///
/// `claim_trailing` is false for the entry that ends a clause: what follows it
/// is either the next clause's keyword, whose gap that clause already claims,
/// or the declaration body, whose separator claims the gap outside the clause
/// alignment.
fn format_declaration_entry<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    entries: FormatDeclarationEntries<'_, 'ast>,
    index: usize,
    claim_trailing: bool,
) {
    match entries {
        FormatDeclarationEntries::Types(entries) => {
            format_clause_entry(f, &entries.get(index).expect("clause type"), claim_trailing);
        }
        FormatDeclarationEntries::Superclass(entry) => {
            format_clause_entry(f, &entry.expect("superclass"), claim_trailing);
        }
        FormatDeclarationEntries::Names(entries) => {
            format_clause_entry(f, &entries.get(index).expect("clause name"), claim_trailing);
        }
    }
}

fn format_clause_entry<'ast, T>(f: &mut JavaFormatter<'_, 'ast>, entry: &T, claim_trailing: bool)
where
    T: Format<'ast, JavaFormatContext<'ast>> + GetSpan,
{
    if claim_trailing {
        entry.fmt(f);
    } else {
        format_content_without_comments_after(entry, entry.span().end, f);
    }
}

/// A type body member. Fields alone keep a source gap between them.
pub trait TypeMember: GetSpan + crate::ast_nodes::AstNodeListElement {
    fn is_field(&self) -> bool;
}

impl TypeMember for ClassMember<'_> {
    fn is_field(&self) -> bool {
        matches!(self, Self::Field(_))
    }
}

impl TypeMember for InterfaceMember<'_> {
    fn is_field(&self) -> bool {
        matches!(self, Self::Constant(_))
    }
}

impl TypeMember for AnnotationInterfaceMemberDeclaration<'_> {
    fn is_field(&self) -> bool {
        matches!(self, Self::Field(_))
    }
}

/// Format the braces at `span` around a type body's members.
fn format_type_body<'parent, 'a: 'parent, M: TypeMember + 'a>(
    f: &mut JavaFormatter<'_, 'a>,
    span: Span,
    members: &AstNode<'parent, 'a, Vec<'a, M>>,
) where
    AstNode<'parent, 'a, M>: Format<'a, JavaFormatContext<'a>>,
{
    format_braced_body(f, members.is_empty(), span, |f| {
        write_blank_line_at_body_start(span, f);
        join_type_members(&mut f.join_nodes_with_hardline(), members);
    });
}

/// Keep one blank line at the start of the type body at `span` where the
/// source has one after its `{`, or after the line comment kept on the `{`'s
/// line. Other bodies drop it.
fn write_blank_line_at_body_start(span: Span, f: &mut JavaFormatter<'_, '_>) {
    let brace_end = f.comments().position_after_character(span.start, b'{');
    let start = f
        .comments()
        .printed_line_comment_after(brace_end)
        .map_or(brace_end, |comment| comment.span.end);
    if f.source_text().lines_after_skipping_terminators(start, None) > 1 {
        empty_line().fmt(f);
    }
}

/// Separate members, preserving source gaps only between fields.
pub fn join_type_members<'parent, 'a: 'parent, M: TypeMember + 'a>(
    join: &mut JoinNodesBuilder<'_, '_, 'a, Line>,
    members: &AstNode<'parent, 'a, Vec<'a, M>>,
) where
    AstNode<'parent, 'a, M>: Format<'a, JavaFormatContext<'a>>,
{
    let mut previous_was_field = false;
    for member in members {
        let is_field = member.is_field();
        if previous_was_field && is_field {
            join.entry(member.span(), &member);
        } else {
            join.entry_with_empty_line(true, &member);
        }
        previous_was_field = is_field;
    }
}
