// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/utils/assignment_like.rs

use std::iter;

use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::formatter::trivia::{
    FormatTrailingComments, format_dangling_comments, format_trailing_comments,
    is_alignable_block_comment,
};
use crate::print::array_dimension::format_array_type_dimensions;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::member_chain::MemberChain;
use crate::utils::statement_body::write_comments_before_opener;
use robusta_ast::{
    AnnotationInterfaceElementDeclaration, AssignmentExpression, Comment, ElementValuePair,
    Expression, ResourceDeclaration, VariableDeclarator,
};
use robusta_formatter_core::{FormatElements, text_width, write};
use robusta_span::{GetSpan, Span};

#[derive(Clone, Copy)]
pub enum AssignmentLike<'a, 'b> {
    VariableDeclarator(&'b AstNode<'b, 'a, VariableDeclarator<'a>>),
    AssignmentExpression(&'b AstNode<'b, 'a, AssignmentExpression<'a>>),
    ElementValuePair(&'b AstNode<'b, 'a, ElementValuePair<'a>>),
    ResourceDeclaration(&'b AstNode<'b, 'a, ResourceDeclaration<'a>>),
    AnnotationInterfaceElementDeclaration(
        &'b AstNode<'b, 'a, AnnotationInterfaceElementDeclaration<'a>>,
    ),
}

/// How an assignment-like breaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentLikeLayout {
    /// First break the right-hand side, then after the operator.
    Fluid,
    /// First break after the operator, then the right-hand side on its own line.
    BreakAfterOperator,
    /// Break only inside the right-hand side.
    NeverBreakAfterOperator,
    /// A link inside an assignment chain, such as `b` in `a = b = c = 0`.
    Chain,
    /// The value that ends an assignment chain, such as `0` in `a = b = c = 0`.
    ChainTail,
    /// A lambda chain that ends an assignment chain, such as `x -> y -> 0` in
    /// `a = b = x -> y -> 0`. Its signatures break before its body.
    ChainTailLambda,
}

/// The minimum number of overlapping characters between left and right hand side
const MIN_OVERLAP_FOR_BREAK: u8 = 3;

/// The comments from `start` that trail the left side, up to `operator`:
/// those ending its line, or else those before `operator` when none starts a
/// line.
pub fn left_trailing_comments<'a>(
    start: u32,
    operator: u8,
    f: &JavaFormatter<'_, 'a>,
) -> &'a [Comment] {
    let comments = f.comments();
    let end_of_line_comments = comments.end_of_line_comments_after(start);
    if end_of_line_comments.is_empty() {
        let before_operator = comments.comments_before_character(start, operator);
        if before_operator.iter().any(Comment::preceded_by_newline) { &[] } else { before_operator }
    } else {
        end_of_line_comments
    }
}

impl<'a, 'b> AssignmentLike<'a, 'b> {
    /// Writes the left side without the comments before the operator, and returns whether it is short.
    fn write_left(&self, f: &mut JavaFormatter<'_, 'a>) -> bool {
        match self {
            Self::VariableDeclarator(declarator) => {
                let name_end = declarator.name.span.end;
                if declarator.init.is_some() || !declarator.dims.is_empty() {
                    format_content_without_comments_after(&declarator.name(), name_end, f);
                } else {
                    declarator.name().fmt(f);
                }
                format_array_type_dimensions(f, name_end, &declarator.dims());
                false
            }
            Self::AssignmentExpression(assignment) => {
                let target_end = assignment.target.span().end;
                format_content_without_comments_after(&assignment.target(), target_end, f);
                false
            }
            Self::ElementValuePair(pair) => {
                let name = pair.name();
                format_content_without_comments_after(&name, name.span.end, f);
                let text_width_for_break =
                    u32::from(f.options().indent_width.value() + MIN_OVERLAP_FOR_BREAK);
                text_width(name.name, f.options().indent_width) < text_width_for_break
            }
            // The resource prints its modifiers and type.
            Self::ResourceDeclaration(resource) => {
                format_content_without_comments_after(&resource.name(), resource.name.span.end, f);
                false
            }
            // The element prints its modifiers and type.
            Self::AnnotationInterfaceElementDeclaration(element) => {
                let name_end = element.name.span.end;
                format_content_without_comments_after(&element.name(), name_end, f);
                write_comments_before_opener(name_end, b'(', f);
                let open_end = f.comments().position_after_character(name_end, b'(');
                let close_end = f.comments().position_after_character(open_end, b')');
                write!(
                    f,
                    [
                        token("("),
                        format_dangling_comments(Span::new(open_end, close_end))
                            .with_soft_block_indent(),
                        token(")")
                    ]
                );
                format_array_type_dimensions(f, close_end, &element.post_paren_dimensions());
                false
            }
        }
    }

    /// The comments before the operator that stay on the left side. An element's `)` or `]`
    /// has no node, so its trailing pass runs here, stopping at `default`.
    fn write_left_trailing_comments(&self, f: &mut JavaFormatter<'_, 'a>) {
        let left_end = self.left_end(f);
        if let Self::AnnotationInterfaceElementDeclaration(element) = self {
            format_trailing_comments(element.span, Span::empty(left_end), self.right_start())
                .fmt(f);
        } else {
            let comments = left_trailing_comments(left_end, b'=', f);
            // A multi-line block ending the line moves to its own line above the right side.
            if !comments.last().is_some_and(|comment| {
                comment.is_multiline_block() && comment.followed_by_newline()
            }) {
                FormatTrailingComments::Comments(comments).fmt(f);
            }
        }
    }

    fn operator(&self) -> &'static str {
        match self {
            Self::AssignmentExpression(assignment) => assignment.op.as_str(),
            Self::AnnotationInterfaceElementDeclaration(_) => "default",
            _ => "=",
        }
    }

    fn write_operator(&self, f: &mut JavaFormatter<'_, 'a>) {
        write!(f, [space(), token(self.operator())]);
    }

    /// End of the left-hand side, before the operator and any comments around it.
    pub fn left_end(&self, f: &JavaFormatter<'_, 'a>) -> u32 {
        match self {
            Self::VariableDeclarator(declarator) => declarator
                .dims
                .last()
                .map_or(declarator.name.span.end, |dimension| dimension.span.end),
            Self::AssignmentExpression(assignment) => assignment.target.span().end,
            Self::ElementValuePair(pair) => pair.name.span.end,
            Self::ResourceDeclaration(resource) => resource.name.span.end,
            Self::AnnotationInterfaceElementDeclaration(element) => {
                element.post_paren_dimensions.last().map_or_else(
                    || f.comments().position_after_character(element.name.span.end, b')'),
                    |dimension| dimension.span.end,
                )
            }
        }
    }

    fn operator_end(&self, f: &JavaFormatter<'_, 'a>) -> u32 {
        let left_end = self.left_end(f);
        // `default` ends with its only `t`, and every assignment operator with
        // its only `=`.
        let last =
            if let Self::AnnotationInterfaceElementDeclaration(_) = self { b't' } else { b'=' };
        f.comments().position_after_character(left_end, last)
    }

    fn has_only_left_hand_side(&self) -> bool {
        match self {
            Self::VariableDeclarator(declarator) => declarator.init.is_none(),
            Self::AnnotationInterfaceElementDeclaration(element) => element.default_value.is_none(),
            _ => false,
        }
    }

    /// The comments after the operator up to a line comment ending its line (`= /* c */ // d`).
    /// Deferred comments and blocks after a riding line comment lead the right side.
    fn operator_line_run(
        operator_end: u32,
        has_riding_comment: bool,
        f: &JavaFormatter<'_, 'a>,
    ) -> &'a [Comment] {
        let comments = f.comments();
        if comments.has_comment_before(operator_end) {
            return &[];
        }
        let run = comments.end_of_line_comments_after(operator_end);
        if run.last().is_some_and(Comment::is_line) && (!has_riding_comment || run.len() == 1) {
            run
        } else {
            &[]
        }
    }

    fn write_right(&self, f: &mut JavaFormatter<'_, 'a>) {
        match self {
            Self::VariableDeclarator(declarator) => {
                declarator.init().expect("only the left side is written without an init").fmt(f);
            }
            Self::AssignmentExpression(assignment) => assignment.value().fmt(f),
            Self::ElementValuePair(pair) => pair.value().fmt(f),
            Self::ResourceDeclaration(resource) => resource.init().fmt(f),
            Self::AnnotationInterfaceElementDeclaration(element) => {
                element
                    .default_value()
                    .expect("only the left side is written without a default")
                    .fmt(f);
            }
        }
    }

    /// Returns the layout variant for an assignment like depending on right expression and left part length
    /// [Prettier applies]: <https://github.com/prettier/prettier/blob/main/src/language-js/print/assignment.js>
    fn layout(
        &self,
        is_left_short: bool,
        left_may_break: bool,
        has_line_comment_on_operator_line: bool,
        f: &JavaFormatter<'_, 'a>,
    ) -> AssignmentLikeLayout {
        let right = self.right_expression();
        let right_shape = right.map(|right| right.inner());
        if let Some(layout) = self.chain_formatting_layout(right_shape) {
            return layout;
        }

        let comments = f.comments();
        let leading = comments.comments_before(self.right_start());
        if has_line_comment_on_operator_line
            || leading.iter().any(|&comment| is_alignable_block_comment(comment, f))
            || self.should_break_after_operator(right, is_left_short, f)
        {
            return AssignmentLikeLayout::BreakAfterOperator;
        }

        if !left_may_break
            && (is_left_short
                || matches!(
                    right_shape,
                    Some(
                        Expression::BooleanLiteral(_)
                            | Expression::IntLiteral(_)
                            | Expression::FloatLiteral(_)
                    )
                )
                || matches!(right_shape, Some(Expression::StringLiteral(literal)) if literal.is_text_block()))
        {
            return AssignmentLikeLayout::NeverBreakAfterOperator;
        }

        AssignmentLikeLayout::Fluid
    }

    fn right_start(&self) -> u32 {
        match self {
            Self::VariableDeclarator(declarator) => {
                declarator.init.as_ref().map_or(declarator.span.end, |init| init.span().start)
            }
            Self::AssignmentExpression(assignment) => assignment.value.span().start,
            Self::ElementValuePair(pair) => pair.value.span().start,
            Self::ResourceDeclaration(resource) => resource.init.span().start,
            Self::AnnotationInterfaceElementDeclaration(element) => {
                element.default_value.as_ref().map_or(element.span.end, |value| value.span().start)
            }
        }
    }

    /// The right side when it is an expression, not an array initializer or annotation.
    fn right_expression(&self) -> Option<AstNode<'b, 'a, Expression<'a>>> {
        match self {
            Self::VariableDeclarator(declarator) => {
                declarator.init().and_then(|init| init.as_expression())
            }
            Self::AssignmentExpression(assignment) => Some(assignment.value()),
            Self::ElementValuePair(pair) => pair.value().as_expression(),
            Self::ResourceDeclaration(resource) => Some(resource.init()),
            Self::AnnotationInterfaceElementDeclaration(element) => {
                element.default_value().and_then(|value| value.as_expression())
            }
        }
    }

    /// The layout of an assignment inside a chain of at least three, such as `b = c = 0` in `a = b = c = 0`.
    fn chain_formatting_layout(
        &self,
        right_shape: Option<&Expression<'a>>,
    ) -> Option<AssignmentLikeLayout> {
        let right_is_tail = !matches!(right_shape, Some(Expression::Assignment(_)));
        let Self::AssignmentExpression(assignment) = self else {
            return None;
        };
        // Two assignments are not a chain: `a = b = 0;`, `int a = b = 0;`, and `() -> a = b = 0`.
        let upper_chain_is_eligible = match assignment.parent() {
            AstNodes::VariableDeclarator(_) | AstNodes::ResourceDeclaration(_) => !right_is_tail,
            AstNodes::AssignmentExpression(parent) => {
                !right_is_tail
                    || !matches!(
                        parent.parent(),
                        AstNodes::LambdaExpression(_) | AstNodes::ExpressionStatement(_)
                    )
            }
            _ => false,
        };

        upper_chain_is_eligible.then_some(match right_shape {
            Some(Expression::Lambda(lambda))
                if matches!(lambda.get_expression(), Some(Expression::Lambda(_))) =>
            {
                AssignmentLikeLayout::ChainTailLambda
            }
            _ if right_is_tail => AssignmentLikeLayout::ChainTail,
            _ => AssignmentLikeLayout::Chain,
        })
    }

    /// Checks if the current assignment is eligible for [AssignmentLikeLayout::BreakAfterOperator]
    fn should_break_after_operator(
        &self,
        right: Option<AstNode<'_, 'a, Expression<'a>>>,
        is_left_short: bool,
        f: &JavaFormatter<'_, 'a>,
    ) -> bool {
        // Only a comment followed by a newline forces the break.
        if f.comments().has_leading_own_line_comment(self.right_start()) {
            return true;
        }
        right.is_some_and(|right| should_break_after_operator(right, is_left_short, f))
    }
}

/// Checks if the function is entitled to be printed with layout [AssignmentLikeLayout::BreakAfterOperator]
///
/// Based on <https://github.com/prettier/prettier/blob/0273e33fc691e28e4ab3f3c8ee86918b65cf823d/src/language-js/print/assignment.js#L196-L264>
fn should_break_after_operator<'a>(
    right: AstNode<'_, 'a, Expression<'a>>,
    is_left_short: bool,
    f: &JavaFormatter<'_, 'a>,
) -> bool {
    match right.inner() {
        // head is a long chain, meaning that right -> right are both assignment expressions
        Expression::Assignment(assignment) => {
            matches!(assignment.value, Expression::Assignment(_))
        }
        // `&&` and `||` never inline their right side, as no Java operand is an object literal.
        Expression::Binary(_) | Expression::Instanceof(_) => true,
        Expression::Conditional(conditional) => {
            matches!(conditional.condition, Expression::Binary(_) | Expression::Instanceof(_))
        }
        _ if is_left_short => false,
        _ => {
            let inner = innermost_expression(right, f);
            matches!(inner.inner(), Expression::StringLiteral(literal) if !literal.is_text_block())
                || is_poorly_breakable_member_or_call_chain(inner, f)
        }
    }
}

/// Traverses nested unary expressions to find the innermost one.
///
/// Example: `!-value` returns `value`.
fn innermost_expression<'b, 'a>(
    mut expression: AstNode<'b, 'a, Expression<'a>>,
    f: &JavaFormatter<'_, 'a>,
) -> AstNode<'b, 'a, Expression<'a>>
where
    'a: 'b,
{
    let allocator = f.allocator();
    while let Expression::Unary(_) = expression.inner() {
        expression = expression.as_unary().unwrap().in_arena(allocator).operand();
    }
    expression
}

/// A chain that has no calls at all or all of whose calls have no arguments
/// or have only one which [is_short_argument], except for member call chains
/// [Prettier applies]: <https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/assignment.js#L329>
///
/// `expression` is the innermost expression of the right side, see [innermost_expression].
fn is_poorly_breakable_member_or_call_chain<'a>(
    expression: AstNode<'_, 'a, Expression<'a>>,
    f: &JavaFormatter<'_, 'a>,
) -> bool {
    let allocator = f.allocator();
    // The member accesses after the last call print around its chain.
    let mut expression = expression;
    let mut is_member_access = false;
    loop {
        expression = match expression.inner() {
            Expression::FieldAccess(_) => {
                expression.as_field_access().unwrap().in_arena(allocator).target()
            }
            Expression::ArrayAccess(_) => {
                expression.as_array_access().unwrap().in_arena(allocator).array()
            }
            _ => break,
        };
        is_member_access = true;
    }
    let call = match expression.inner() {
        Expression::MethodCall(call) if call.member().is_some() => {
            expression.as_method_call().unwrap().in_arena(allocator)
        }
        // `f(a)` calls an identifier, so it is a chain with a simple head.
        Expression::MethodCall(call) => {
            return !f.comments().has_comment_in_span(call.span) && !is_breakable_call(call, f);
        }
        Expression::Identifier(_) | Expression::This(_) => return is_member_access,
        // `A.class` and `A.this` read as member accesses on an identifier.
        Expression::ClassLiteral(_) | Expression::QualifiedThis(_) => return true,
        _ => return false,
    };
    let chain = MemberChain::new(f, call);

    // Only chains with simple head are poorly breakable. The member accesses
    // that join the head do not make it less simple.
    let head = iter::successors(Some(chain.head()), |head| match head {
        Expression::FieldAccess(access) => Some(&access.target),
        Expression::ArrayAccess(access) => Some(&access.array),
        _ => None,
    })
    .last();
    if !matches!(
        head,
        Some(
            Expression::Identifier(_)
                | Expression::This(_)
                | Expression::QualifiedThis(_)
                | Expression::ClassLiteral(_)
                | Expression::MethodCall(_)
        )
    ) {
        return false;
    }

    let mut calls = chain.calls().peekable();
    let Some(first_call) = calls.peek() else {
        return true;
    };

    if f.comments().has_comment_in_span(first_call.span) {
        return false;
    }

    calls.all(|call| !is_breakable_call(call, f)) && !chain.is_breakable()
}

/// A call with more than one argument or type argument, or one argument that is not short.
fn is_breakable_call(call: &robusta_ast::MethodCall<'_>, f: &JavaFormatter<'_, '_>) -> bool {
    let threshold = f.options().line_width.value() / 4;
    let is_breakable_call = match call.args.len() {
        0 => false,
        1 => !is_short_argument(&call.args[0], threshold, f),
        _ => true,
    };
    // A single Java type argument is never complex: it has no union, intersection, or object type.
    is_breakable_call || call.type_args().len() > 1
}

/// This function checks if [`Expression`] is short
/// We need it to decide if a call with the argument is breakable or not
/// If the argument is short the function call isn't breakable
/// [Prettier applies]: <https://github.com/prettier/prettier/blob/0273e33fc691e28e4ab3f3c8ee86918b65cf823d/src/language-js/utils/index.js#L433-L484>
fn is_short_argument(argument: &Expression<'_>, threshold: u16, f: &JavaFormatter<'_, '_>) -> bool {
    match argument {
        Expression::Identifier(name) => {
            text_width(name.name, f.options().indent_width) <= u32::from(threshold)
        }
        Expression::Unary(unary) => is_short_argument(&unary.operand, threshold, f),
        Expression::MethodCall(call) => {
            call.member().is_none()
                && call.args.is_empty()
                && text_width(call.name(), f.options().indent_width)
                    <= u32::from(threshold.saturating_sub(2))
        }
        Expression::StringLiteral(literal) => {
            let source = literal.span.source_text(f.source_text().as_str());
            text_width(source, f.options().indent_width) <= u32::from(threshold)
                && !source.contains('\n')
        }
        Expression::This(_)
        | Expression::NullLiteral(_)
        | Expression::IntLiteral(_)
        | Expression::FloatLiteral(_)
        | Expression::CharLiteral(_)
        | Expression::BooleanLiteral(_) => true,
        _ => false,
    }
}

impl<'a> Format<'a, JavaFormatContext<'a>> for AssignmentLike<'a, '_> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if self.has_only_left_hand_side() {
            self.write_left(f);
            return;
        }

        // The layout depends on how the left side formats, so it goes to a scratch buffer first.
        // The left side's trailing run stops at the operator: comments past it print after the
        // operator or lead the right side.
        let operator_end = self.operator_end(f);
        let view_limit = f.comments().limit_comments_up_to(operator_end);
        let mut is_left_short = false;
        let formatted_left = f.format_to_vec(|f| {
            is_left_short = self.write_left(f);
            self.write_left_trailing_comments(f);
        });
        f.comments().restore_view_limit(view_limit);
        let left_may_break = formatted_left.may_directly_break();
        let left_end = self.left_end(f);
        let riding_comment = f.comments().printed_line_comment_after(left_end);
        let operator_line_run = Self::operator_line_run(operator_end, riding_comment.is_some(), f);
        // A line comment on the operator's line, before it (printed by the left side) or after
        // it (the run): the pending `line_suffix` must be followed by the break.
        let has_line_comment_on_operator_line = !operator_line_run.is_empty()
            || riding_comment.is_some_and(|comment| {
                comment.span.start > left_end
                    || f.comments().unprinted_comments().first().is_some_and(|next| {
                        next.is_block()
                            && next.span.start >= operator_end
                            && next.span.end <= self.right_start()
                    })
            });
        let layout =
            self.layout(is_left_short, left_may_break, has_line_comment_on_operator_line, f);

        let left = format_once(move |f| {
            for element in formatted_left {
                f.write_element(element);
            }
        });
        let right = format_with(|f| {
            // A lambda chain needs the layout inside its `write`, which is
            // reached through the generated `fmt`, so the span-keyed context
            // slot hands it across that frame.
            if let Some(Expression::Lambda(lambda)) = self.right_expression().as_deref() {
                f.context_mut().set_lambda_assignment_layout(lambda.span, layout);
            }
            self.write_right(f);
        });

        let inner_content = format_once(|f| {
            write!(f, [group(&left)]);
            self.write_operator(f);
            FormatTrailingComments::Comments(operator_line_run).fmt(f);

            match layout {
                AssignmentLikeLayout::Fluid => {
                    let group_id = f.state().group_id("assignment_like");
                    write!(
                        f,
                        [
                            group(&indent(&soft_line_break_or_space()))
                                .with_group_id(Some(group_id)),
                            line_suffix_boundary(),
                            indent_if_group_breaks(&right, group_id)
                        ]
                    );
                }
                AssignmentLikeLayout::BreakAfterOperator => {
                    // Ungrouped, the break follows the enclosing group, expanded by the line comment.
                    if has_line_comment_on_operator_line {
                        soft_line_indent_or_space(&right).fmt(f);
                    } else {
                        group(&soft_line_indent_or_space(&right)).fmt(f);
                    }
                }
                AssignmentLikeLayout::NeverBreakAfterOperator
                | AssignmentLikeLayout::ChainTailLambda => {
                    write!(f, [space(), right]);
                }
                AssignmentLikeLayout::Chain => {
                    write!(f, [soft_line_break_or_space(), right]);
                }
                AssignmentLikeLayout::ChainTail => {
                    write!(f, [soft_line_indent_or_space(&right)]);
                }
            }
        });

        match layout {
            // Layouts that don't need enclosing group
            AssignmentLikeLayout::Chain | AssignmentLikeLayout::ChainTail => {
                inner_content.fmt(f);
            }
            _ => {
                write!(f, [group(&inner_content)]);
            }
        }
    }
}
