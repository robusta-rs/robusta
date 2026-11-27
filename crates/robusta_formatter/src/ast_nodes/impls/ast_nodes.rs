use robusta_ast::{Comment, SwitchCase};
use robusta_span::{GetSpan, Span};

use crate::formatter::trivia::{
    format_documentation_comments_before, format_leading_comments, format_line_after_moved_comment,
    format_trailing_comments,
};
use crate::formatter::{JavaFormatter, prelude::*};
use crate::print::FormatWrite;
use crate::print::type_parameters::starts_type_declaration_list;
use crate::utils::is_header_condition;

use crate::ast_nodes::{AstNode, AstNodes};

impl<'a, T: GetSpan> AstNode<'_, 'a, T> {
    pub fn enclosing_span(&self) -> Span {
        match self.parent() {
            AstNodes::OrdinaryCompilationUnit(parent) => parent.parent().span(),
            AstNodes::CompactCompilationUnit(parent) => parent.parent().span(),
            AstNodes::ModularCompilationUnit(parent) => parent.parent().span(),
            _ => self.parent.span(),
        }
    }

    pub fn format_leading_comments(&self, f: &mut JavaFormatter<'_, 'a>)
    where
        Self: FormatWrite<'a>,
    {
        format_node_leading_comments(self.parent(), self.leading_comments_start(), f);
    }

    pub fn format_trailing_comments(&self, f: &mut JavaFormatter<'_, 'a>) {
        let following_start = self.following_span_start;
        if is_import(self.parent(), self.span()) {
            format_import_trailing_comments(self.enclosing_span(), self.span(), following_start, f);
            return;
        }
        if let Some(separator) = separator_before_declaration(self.parent(), following_start) {
            format_trailing_comments_before_separator(
                self.enclosing_span(),
                self.span(),
                separator,
                following_start,
                f,
            );
        } else {
            format_node_trailing_comments(
                self.enclosing_span(),
                self.span(),
                following_start,
                declaration_may_follow(self.parent(), self.span()),
                f,
            );
        }
    }
}

impl AstNodes<'_, '_> {
    /// Whether the node at `span` is the object of a member: the `a` of
    /// `a.b`, `a.b()`, `a[b]`, `a::b`, or `a.new B()`.
    pub fn is_member_object_span(&self, span: Span) -> bool {
        match self {
            AstNodes::FieldAccess(node) => node.target.span() == span,
            AstNodes::MethodMember(node) => node.target.span() == span,
            AstNodes::ArrayAccess(node) => node.array.span() == span,
            AstNodes::MethodReference(node) => node.target.span() == span,
            AstNodes::QualifiedClassInstanceCreation(node) => node.qualifier.span() == span,
            _ => false,
        }
    }

    /// Whether this is a parenthesized expression that is the object of a member: `(a + b).c`.
    pub fn is_parenthesized_member_object(&self) -> bool {
        matches!(self, AstNodes::ParenthesizedExpression(_))
            && self.parent().is_member_object_span(self.span())
    }

    /// Whether this is a parenthesized expression that is a conditional's test
    /// outside a header's condition: `(a || b) ? c : d`.
    pub fn is_parenthesized_conditional_test(&self) -> bool {
        if !matches!(self, AstNodes::ParenthesizedExpression(_)) {
            return false;
        }
        match self.parent() {
            AstNodes::ConditionalExpression(conditional) => {
                conditional.condition.span() == self.span()
                    && !is_header_condition(conditional.parent(), conditional.span)
            }
            _ => false,
        }
    }
}

/// The trailing comments of the import at `span`. Sorting moves them with it,
/// so those on the line of a line comment moved behind its `;` stay its own.
/// It could leave a documentation comment after an import documenting the
/// declaration after the imports, though, so it and the comments after it lead
/// whatever follows the import. After the last import, with nothing following,
/// the unit prints them.
fn format_import_trailing_comments(
    enclosing_span: Span,
    span: Span,
    mut following_start: u32,
    f: &mut JavaFormatter<'_, '_>,
) {
    if let Some(comment) = f
        .comments()
        .unprinted_comments()
        .iter()
        .take_while(|comment| following_start == 0 || comment.span.start < following_start)
        .find(|comment| comment.is_documentation())
    {
        following_start = comment.span.start;
    }
    format_trailing_comments(enclosing_span, span, following_start).fmt(f);
}

/// Whether the child of `parent` at `span` is an import.
fn is_import(parent: &AstNodes<'_, '_>, span: Span) -> bool {
    let imports = match parent {
        AstNodes::OrdinaryCompilationUnit(unit) => &unit.imports,
        AstNodes::CompactCompilationUnit(unit) => &unit.imports,
        AstNodes::ModularCompilationUnit(unit) => &unit.imports,
        _ => return false,
    };
    imports.binary_search_by_key(&span.start, |import| import.span.start).is_ok()
}

/// Whether a declaration could follow the `;` ending the child of `parent` at
/// `span`. None follows a module directive (JLS §7.7) or a switch rule
/// (JLS §14.11.1).
fn declaration_may_follow(parent: &AstNodes<'_, '_>, span: Span) -> bool {
    let is_rule = |cases: &[SwitchCase<'_>]| {
        cases
            .binary_search_by_key(&span.start, |case| case.span().start)
            .is_ok_and(|index| matches!(cases[index], SwitchCase::Rule(_)))
    };
    match parent {
        AstNodes::ModuleDeclaration(_) => false,
        AstNodes::SwitchStatement(parent) => !is_rule(&parent.cases),
        AstNodes::SwitchExpression(parent) => !is_rule(&parent.cases),
        _ => true,
    }
}

/// The separator before the child of `parent` starting at `following_start`
/// when that child is a declaration behind one: the `,` before an enum
/// constant, or a switch group's `:` before a local type declaration.
fn separator_before_declaration(parent: &AstNodes<'_, '_>, following_start: u32) -> Option<u8> {
    match parent {
        AstNodes::EnumBody(body) => body
            .constants
            .binary_search_by_key(&following_start, |constant| constant.span.start)
            .is_ok()
            .then_some(b','),
        AstNodes::SwitchBlockStatementGroup(group) => group
            .statements
            .first()
            .is_some_and(|statement| {
                statement.span().start == following_start
                    && statement.is_local_class_or_interface_declaration()
            })
            .then_some(b':'),
        _ => None,
    }
}

/// [`format_node_trailing_comments`] before the `separator` that follows the
/// node at `span` and the declaration after it, keeping each documentation
/// comment on its side of the separator: those before it print in place, and
/// a block one after it leads the declaration.
fn format_trailing_comments_before_separator(
    enclosing_span: Span,
    span: Span,
    separator: u8,
    following_start: u32,
    f: &mut JavaFormatter<'_, '_>,
) {
    let separator = f.comments().position_after_character(span.end, separator) - 1;
    let end = format_documentation_comments_before(span.end, separator, f);
    // A line comment rides past the separator, so it stays behind it.
    let following_start = f
        .comments()
        .unprinted_comments()
        .iter()
        .take_while(|comment| comment.span.start < following_start)
        .find(|comment| {
            comment.span.start > separator && comment.is_block() && comment.is_documentation()
        })
        .map_or(following_start, |comment| comment.span.start);
    format_node_trailing_comments(
        enclosing_span,
        Span::new(span.start, end),
        following_start,
        true,
        f,
    );
}

/// The trailing comments of the node at `span`, which covers the `;` ending
/// it, if any. The comments on the line of a line comment moved behind the
/// node that cannot join it start lines of their own, as if written there:
/// they go to whatever follows the node, or follow it at the end of its
/// parent. A node that ends its parent leaves them to that parent. Unless
/// `declaration_may_follow`, a lone `///` line may join instead.
pub fn format_node_trailing_comments(
    enclosing_span: Span,
    mut span: Span,
    mut following_start: u32,
    declaration_may_follow: bool,
    f: &mut JavaFormatter<'_, '_>,
) {
    let count = f.comments().line_after_moved_comment(span.end, declaration_may_follow);
    if count > 0 {
        if following_start != 0 {
            following_start = f
                .comments()
                .unprinted_comments()
                .first()
                .map_or(following_start, |it| it.span.start);
        } else if span.end < enclosing_span.end {
            span.end = format_line_after_moved_comment(count, f);
        }
    }
    format_trailing_comments(enclosing_span, span, following_start).fmt(f);
}

/// The leading comments of the child of `parent` that starts at `start`.
pub fn format_node_leading_comments<'a>(
    parent: &AstNodes<'_, 'a>,
    start: u32,
    f: &mut JavaFormatter<'_, 'a>,
) {
    // An operand after its operator or `[`: the parent's printer
    // decides the break, so a comment starting its line may join it.
    let after_operator = match parent {
        AstNodes::BinaryExpression(_)
        | AstNodes::ConditionalExpression(_)
        | AstNodes::AssignmentExpression(_)
        | AstNodes::ElementValuePair(_)
        | AstNodes::UnaryExpression(_)
        | AstNodes::CastExpression(_) => true,
        // A length after its `[`, not an annotation before it.
        AstNodes::ArrayDimension(parent) => {
            parent.annotations.last().is_none_or(|last| start >= last.span.end)
        }
        // A default value after its `default`.
        AstNodes::AnnotationInterfaceElementDeclaration(parent) => {
            parent.default_value.as_ref().is_some_and(|value| start >= value.span().start)
        }
        // The resource's type and name come before its `=`.
        AstNodes::ResourceDeclaration(parent) => start >= parent.init.span().start,
        // As do the declarator's name and dimensions.
        AstNodes::VariableDeclarator(parent) => {
            parent.init.as_ref().is_some_and(|init| start >= init.span().start)
        }
        AstNodes::InstanceofExpression(parent) => start >= parent.rhs.span().start,
        // A multi-catch type after its `|`.
        AstNodes::CatchClause(parent) => {
            parent.types.iter().skip(1).any(|ty| ty.span().start == start)
        }
        // A bound after `extends` or `&`.
        AstNodes::TypeParameter(parent) => {
            parent.bounds.first().is_some_and(|first| start >= first.span().start)
        }
        // The first item after a type declaration list's `<`.
        parent => starts_type_declaration_list(parent, start),
    };
    let comments = f.comments();
    let leading = comments.comments_before(start);
    if !after_operator && leading.first().is_some_and(Comment::preceded_by_newline) {
        hard_line_break().fmt(f);
    }
    format_leading_comments(Span::empty(start)).fmt(f);
}
