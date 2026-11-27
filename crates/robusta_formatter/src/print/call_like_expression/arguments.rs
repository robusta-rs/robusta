// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/print/call_like_expression/arguments.rs

use robusta_allocator::Vec;
use robusta_ast::{
    ArrayCreation, ClassType, Comment, Expression, LambdaExpression, MethodCall, Type,
    match_class_type,
};
use robusta_formatter_core::{
    BestFittingElement, FormatElement, FormatElements, format_args, write,
};
use robusta_span::{GetSpan, Span};

use crate::ast_nodes::{AstNode, AstNodeListElement};
use crate::formatter::prelude::*;
use crate::formatter::separated::format_empty_parentheses;
use crate::formatter::trivia::{format_opening_paren_comment, format_trailing_comments};
use crate::print::array_element_list::can_concisely_print_array_list;
use crate::print::lambda_expression::{
    FormatLambdaExpression, FormatLambdaExpressionOptions, FunctionCacheMode,
    GroupedCallArgumentLayout,
};
use crate::source_text::line_breaks;
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::member_chain::simple_argument::SimpleArgument;

/// An entry of an argument list: an expression in a call, or an element
/// value or pair in an annotation.
pub trait Argument<'a>: AstNodeListElement + Sized {
    /// Whether the arguments compose functions, which puts each on its own line.
    fn is_function_composition(arguments: &[Self]) -> bool;

    /// The argument that can break while the others stay on the `(` line.
    fn grouped_layout(
        arguments: &AstNode<'_, 'a, Vec<'a, Self>>,
        f: &JavaFormatter<'_, 'a>,
    ) -> Option<GroupedCallArgumentLayout>;

    fn as_lambda<'p>(
        argument: &AstNode<'p, 'a, Self>,
    ) -> Option<AstNode<'p, 'a, LambdaExpression<'a>>>;
}

impl<'a> Argument<'a> for Expression<'a> {
    fn is_function_composition(arguments: &[Self]) -> bool {
        is_function_composition_args(arguments)
    }

    fn grouped_layout(
        arguments: &AstNode<'_, 'a, Vec<'a, Self>>,
        f: &JavaFormatter<'_, 'a>,
    ) -> Option<GroupedCallArgumentLayout> {
        arguments_grouped_layout(arguments, f)
    }

    fn as_lambda<'p>(
        argument: &AstNode<'p, 'a, Self>,
    ) -> Option<AstNode<'p, 'a, LambdaExpression<'a>>> {
        argument.as_lambda()
    }
}

/// An argument list whose `)` ends at `close_end`. An empty list keeps a
/// comment between its delimiters, so `foo(/* why */)` does not push the
/// comment out of the call.
pub fn format_arguments<'p, 'a, T: Argument<'a>>(
    f: &mut JavaFormatter<'_, 'a>,
    arguments: &AstNode<'p, 'a, Vec<'a, T>>,
    close_end: u32,
) where
    AstNode<'p, 'a, T>: Format<'a, JavaFormatContext<'a>>,
{
    let Some(first) = arguments.first() else {
        // A class body may follow the `)`.
        format_empty_parentheses(f, Span::new(arguments.parent().span().start, close_end));
        return;
    };

    // A blank line (two line breaks) after the comma between arguments puts every argument on
    // its own line. Line breaks before the comma don't count, as printing drops them.
    let has_empty_line = arguments.windows(2).any(|pair| {
        let comma = f.comments().position_after_character(pair[0].span().end, b',');
        line_breaks(f.source_text().slice_range(comma, pair[1].span().start)).nth(1).is_some()
    });

    if has_empty_line || T::is_function_composition(arguments) {
        format_all_args_broken_out(arguments, true, f);
    } else if !f.comments().opening_paren_line_comment(first.span().start).is_empty() {
        format_all_args_broken_out(arguments, false, f);
    } else if let Some(group_layout) = T::grouped_layout(arguments, f) {
        write_grouped_arguments(arguments, group_layout, f);
    } else {
        format_all_args_broken_out(arguments, false, f);
    }
}

/// Tests if a call has multiple anonymous function like (lambda or anonymous class) arguments.
///
/// ## Examples
///
/// ```java
/// compose(sortBy(x -> x), flatten, map(x -> List.of(x, x * 2)));
/// ```
pub fn is_function_composition_args(args: &[Expression<'_>]) -> bool {
    if args.len() <= 1 {
        return false;
    }

    let mut has_seen_function_like = false;
    let is_call_with_function_argument = |call: &MethodCall| call.args.iter().any(is_function_like);

    for arg in args {
        if is_function_like(arg) {
            if has_seen_function_like {
                return true;
            }
            has_seen_function_like = true;
        } else if let Expression::MethodCall(call) = arg
            && is_call_with_function_argument(call)
        {
            return true;
        }
    }

    false
}

/// A lambda reads as an arrow function and an anonymous class as a function expression.
pub fn is_function_like(expression: &Expression<'_>) -> bool {
    match expression {
        Expression::Lambda(_) => true,
        expression => expression.anonymous_class_body().is_some(),
    }
}

/// The arguments' entries, each on its own line when the list breaks, with
/// the blank lines of the source between them.
fn format_all_args_broken_out<'p, 'a, T: Argument<'a>>(
    node: &AstNode<'p, 'a, Vec<'a, T>>,
    expand: bool,
    f: &mut JavaFormatter<'_, 'a>,
) where
    AstNode<'p, 'a, T>: Format<'a, JavaFormatContext<'a>>,
{
    let last_index = node.len() - 1;
    let first_start = node.first().expect("argument").span().start;
    let last = node.last().expect("argument").span();
    write!(
        f,
        [group(&format_args!(
            "(",
            format_with(|f| {
                format_opening_paren_comment(first_start, f);
            }),
            soft_block_indent(&format_once(|f| {
                for (index, argument) in node.iter().enumerate() {
                    if index > 0 {
                        match f.lines_before(argument.span()) {
                            0 | 1 => soft_line_break_or_space().fmt(f),
                            _ => empty_line().fmt(f),
                        }
                    }

                    if index == last_index {
                        format_last_delimited_entry(f, last, &argument);
                    } else {
                        write!(f, [argument, ","]);
                    }
                }
            })),
            ")",
        ))
        .should_expand(expand)]
    );
}

/// An argument list's last entry, whose span is `last`. The list has no node
/// of its own, so the entry's trailing pass is bounded here, at the `)`.
fn format_last_delimited_entry<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    last: Span,
    entry: &impl Format<'ast, JavaFormatContext<'ast>>,
) {
    format_content_without_comments_after(entry, last.end, f);
    let close = f.comments().position_after_character(last.end, b')') - 1;
    format_trailing_comments(Span::new(last.start, close), last, 0).fmt(f);
}

fn format_all_elements_broken_out<'a>(
    elements: impl Iterator<Item = Option<FormatElement<'a>>>,
    expand: bool,
    f: &mut JavaFormatter<'_, 'a>,
) {
    write!(
        f,
        [group(&format_args!(
            "(",
            soft_block_indent(&format_once(move |f| {
                for (index, element) in elements.enumerate() {
                    if let Some(element) = element {
                        if index > 0 {
                            soft_line_break_or_space().fmt(f);
                        }

                        f.write_element(element);
                    }
                }
            })),
            ")",
        ))
        .should_expand(expand)]
    );
}

pub fn arguments_grouped_layout<'a>(
    args: &AstNode<'_, 'a, Vec<'a, Expression<'a>>>,
    f: &JavaFormatter<'_, 'a>,
) -> Option<GroupedCallArgumentLayout> {
    // For exactly 2 arguments, we need to check both grouping strategies.
    // To avoid redundant `can_group_expression_argument` calls, we handle this case specially.
    if let [first, second] = &args[..] {
        if can_group_expression_argument(second, f) {
            // Check if we should group the last argument (second)
            let second = args.get(1).expect("argument");
            return should_group_last_argument_impl(2, Some(first), &second, f)
                .then_some(GroupedCallArgumentLayout::GroupedLastArgument);
        }

        should_group_first_argument(first, second, f)
            .then_some(GroupedCallArgumentLayout::GroupedFirstArgument)
    } else {
        // For other cases (not exactly 2 arguments), only check last argument grouping
        should_group_last_argument(args, f)
            .then_some(GroupedCallArgumentLayout::GroupedLastArgument)
    }
}

/// Checks if the first argument requires grouping
fn should_group_first_argument(
    first: &Expression,
    second: &Expression,
    f: &JavaFormatter<'_, '_>,
) -> bool {
    match first {
        first if first.anonymous_class_body().is_some() => {}
        // Lambdas with an expression body don't get grouped as the first
        // argument, since they'll either fit entirely on the line or break
        // fully. Only a lambda with a block body can be grouped to collapse
        // the braces.
        Expression::Lambda(lambda) => {
            if lambda.body.is_expression() {
                return false;
            }
        }
        _ => return false,
    }

    if is_function_like(second) || matches!(second, Expression::Conditional(_)) {
        return false;
    }

    let first_span = first.span();

    // Don't group if there are comments around the first argument:
    // - Before the first argument
    // - After the first argument (next non-whitespace is not a comma)
    // - End-of-line comments between the first and second argument
    if f.comments().has_comment_before(first_span.start)
        || !f.source_text().next_non_whitespace_byte_is(first_span.end, b',')
        || f.comments()
            .comments_in_range(first_span.end, second.span().start)
            .iter()
            .any(Comment::followed_by_newline)
    {
        return false;
    }

    is_relatively_short_argument(second)
}

/// Core logic for checking if the last argument should be grouped.
/// Takes the penultimate argument as an Expression for the 2-argument case,
/// or extracts it from the arguments array for other cases.
fn should_group_last_argument_impl<'a>(
    args_len: usize,
    penultimate: Option<&Expression>,
    last: &AstNode<'_, 'a, Expression<'a>>,
    f: &JavaFormatter<'_, 'a>,
) -> bool {
    // Check if penultimate and last are the same kind
    if let Some(penultimate) = penultimate
        && grouped_kind(penultimate).is_some()
        && grouped_kind(penultimate) == grouped_kind(last)
    {
        return false;
    }

    if has_comment_around_last_argument(penultimate.map(GetSpan::span), last.span(), f) {
        return false;
    }

    match grouped_kind(last) {
        Some(GroupedKind::Array) if penultimate.is_some() => {
            // Not after a lambda in a two-argument call
            if args_len == 2 && matches!(penultimate, Some(Expression::Lambda(_))) {
                return false;
            }

            // A text block is never a concise list of numbers.
            let creation = last.as_array_creation();
            creation.as_ref().and_then(AstNode::<ArrayCreation>::init).is_none_or(|init| {
                !can_concisely_print_array_list(
                    init.span,
                    init.elements().iter().map(|element| element.as_expression()),
                    f,
                )
            })
        }
        _ => true,
    }
}

/// Whether a comment around the last argument keeps it from grouping. The
/// argument before it, if any, ends `penultimate`.
pub fn has_comment_around_last_argument(
    penultimate: Option<Span>,
    last_span: Span,
    f: &JavaFormatter<'_, '_>,
) -> bool {
    let has_comment_before_last = if let Some(penultimate) = penultimate {
        // Check for comments between the penultimate and last argument
        let comments = f.comments().comments_in_range(penultimate.end, last_span.start);
        comments.last().is_some_and(|&comment| {
            // Exclude end-of-line comments (treated as previous node's comment)
            // and comments followed by a comma
            !comment.followed_by_newline()
                && !f.context().source_text().next_non_whitespace_byte_is(comment.span.end, b',')
        })
            // A comment that starts its line leads the last argument and keeps
            // the line break before it, which a grouped argument cannot indent
            || comments.iter().any(Comment::preceded_by_newline)
    } else {
        f.comments().has_comment_before(last_span.start)
    };

    has_comment_before_last
        // Check for comments after the last argument
        || f.comments().comments_after(last_span.end).first().is_some_and(|comment| {
            !f.source_text().bytes_contain(last_span.end, comment.span.start, b')')
        })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GroupedKind {
    Array,
    Cast,
    Lambda,
    AnonymousClass,
}

/// The kind of a groupable argument, for comparing the last two arguments.
/// A cast reads as an `as` expression, and a text block as an array literal.
fn grouped_kind(expression: &Expression<'_>) -> Option<GroupedKind> {
    match expression {
        Expression::ArrayCreation(creation) if creation.init.is_some() => Some(GroupedKind::Array),
        Expression::StringLiteral(literal) if literal.is_text_block() => Some(GroupedKind::Array),
        Expression::Cast(_) => Some(GroupedKind::Cast),
        Expression::Lambda(_) => Some(GroupedKind::Lambda),
        expression if expression.anonymous_class_body().is_some() => {
            Some(GroupedKind::AnonymousClass)
        }
        _ => None,
    }
}

/// Checks if the last argument should be grouped.
fn should_group_last_argument<'a>(
    args: &AstNode<'_, 'a, Vec<'a, Expression<'a>>>,
    f: &JavaFormatter<'_, 'a>,
) -> bool {
    let last = args.get(args.len() - 1).expect("argument");
    let penultimate = args.len().checked_sub(2).map(|index| &args[index]);

    can_group_expression_argument(&last, f)
        && should_group_last_argument_impl(args.len(), penultimate, &last, f)
}

/// Check if `ty` is a relatively simple type, allowing a few additional cases
/// through. The simplicity is determined as being a primitive or any
/// reference type with no type arguments. For example:
/// ```text
///     int             => true
///     String          => true
///     Map<K, V>       => false
///     List<String>    => false
/// ```
/// This function also introspects into array and generic types to extract the
/// core type, but only to a limited extent:
/// ```text
///     String[]           => String
///     String[][]         => String
///     String[][][]       => String[]
///     List<String>[][]   => String
///     List<String[]>[]   => String[]
///     List<String[][]>   => String[][]
/// ```
fn is_simple_type(ty: &Type<'_>) -> bool {
    // Reach up to two-levels deep into array types:
    //     String[]     => String
    //     String[][]   => String
    //     String[][][] => String[]
    let extracted_array_type = match ty {
        Type::Array(array) => match &array.element {
            Type::Array(inner_array) => &inner_array.element,
            element_type => element_type,
        },
        _ => ty,
    };

    // Then, extract the first type argument of a generic as long as it's the
    // only argument:
    //     List<Integer> => Integer
    //     Map<K, V>     => Map<K, V>
    let extracted_generic_type = match extracted_array_type.as_class_type().map(ClassType::args) {
        Some([argument]) => argument.as_type().unwrap_or(extracted_array_type),
        _ => extracted_array_type,
    };

    // Any reference with no type arguments
    match extracted_generic_type {
        Type::Primitive(_) => true,
        Type::Array(_) => false,
        it @ match_class_type!(Type) => {
            it.to_class_type().self_and_qualifiers().all(|ty| ty.args().is_empty())
        }
    }
}

/// Checks if `argument` is "short" enough to be groupable.
fn is_relatively_short_argument(argument: &Expression<'_>) -> bool {
    match argument {
        Expression::Binary(binary) => {
            SimpleArgument::from(&binary.lhs).is_simple_with_depth(1)
                && SimpleArgument::from(&binary.rhs).is_simple_with_depth(1)
        }
        Expression::Cast(cast) => {
            cast.additional_bounds.is_empty()
                && is_simple_type(&cast.ty)
                && SimpleArgument::from(&cast.operand).is_simple_with_depth(1)
        }
        Expression::MethodCall(call) => {
            call.args.len() <= 1 && SimpleArgument::from(argument).is_simple()
        }
        Expression::ClassInstanceCreation(creation) => {
            creation.args.len() <= 1 && SimpleArgument::from(argument).is_simple()
        }
        Expression::QualifiedClassInstanceCreation(creation) => {
            creation.args.len() <= 1 && SimpleArgument::from(argument).is_simple()
        }
        _ => SimpleArgument::from(argument).is_simple(),
    }
}

/// Checks if `argument` benefits from grouping in call arguments.
pub fn can_group_expression_argument(argument: &Expression<'_>, f: &JavaFormatter<'_, '_>) -> bool {
    match argument {
        Expression::ArrayCreation(creation) => creation
            .init
            .as_ref()
            .is_some_and(|init| can_group_array_argument(init.span, init.elements.is_empty(), f)),
        Expression::StringLiteral(literal) => literal.is_text_block(),
        Expression::Cast(cast) => can_group_expression_argument(&cast.operand, f),
        Expression::Lambda(lambda) => can_group_lambda_argument(lambda, false),
        expression => expression.anonymous_class_body().is_some(),
    }
}

/// Checks if an array initializer at `span` has elements or comments to
/// break over.
pub fn can_group_array_argument(span: Span, is_empty: bool, f: &JavaFormatter<'_, '_>) -> bool {
    !is_empty || f.comments().has_comment_in_span(span)
}

fn can_group_lambda_argument(lambda: &LambdaExpression, is_lambda_recursion: bool) -> bool {
    lambda.get_expression().is_none_or(|expression| match expression {
        Expression::ArrayCreation(creation) => creation.init.is_some(),
        Expression::StringLiteral(literal) => literal.is_text_block(),
        Expression::Lambda(inner_lambda) => can_group_lambda_argument(inner_lambda, true),
        Expression::Conditional(_) | Expression::MethodCall(_) => !is_lambda_recursion,
        _ => false,
    })
}

fn write_grouped_arguments<'p, 'a, T: Argument<'a>>(
    node: &AstNode<'p, 'a, Vec<'a, T>>,
    group_layout: GroupedCallArgumentLayout,
    f: &mut JavaFormatter<'_, 'a>,
) where
    AstNode<'p, 'a, T>: Format<'a, JavaFormatContext<'a>>,
{
    let last_index = node.len() - 1;
    let last = node.last().expect("argument").span();
    let mut non_grouped_breaks = false;
    let mut grouped_breaks = false;
    let mut has_cached = false;

    // Pre-format the arguments to determine if they can be grouped.
    let elements = node
        .iter()
        .enumerate()
        .map(|(index, argument)| {
            let is_grouped_argument = (group_layout.is_grouped_first() && index == 0)
                || (group_layout.is_grouped_last() && index == last_index);

            let interned = f.intern(&format_once(|f| {
                if is_grouped_argument && let Some(lambda) = T::as_lambda(&argument) {
                    has_cached = true;
                    FormatLambdaExpression::new_with_options(
                        &lambda,
                        FormatLambdaExpressionOptions {
                            cache_mode: FunctionCacheMode::Cache,
                            ..FormatLambdaExpressionOptions::default()
                        },
                    )
                    .fmt(f);
                } else if index == last_index {
                    format_last_delimited_entry(f, last, &argument);
                } else {
                    argument.fmt(f);
                }
                if index != last_index {
                    ",".fmt(f);
                }
            }));

            let break_type =
                if is_grouped_argument { &mut grouped_breaks } else { &mut non_grouped_breaks };

            *break_type = *break_type || interned.as_ref().is_some_and(FormatElement::will_break);

            interned
        })
        .collect::<std::vec::Vec<_>>();

    // If any of the not grouped elements break, then fall back to the variant where
    // all arguments are printed in expanded mode.
    if non_grouped_breaks {
        return format_all_elements_broken_out(elements.into_iter(), true, f);
    }

    // First write the most expanded variant because it needs `arguments`.
    let most_expanded = best_fitting_variant(f, |f| {
        format_all_elements_broken_out(elements.iter().cloned(), true, f);
    });

    // Now reformat the first or last argument if it happens to be a lambda.
    // The grouped lambda removes the soft line breaks from its parameters.
    //
    // This caches the lambda's body when it is first formatted, so a body that
    // contains another call with a lambda as first or last argument is not
    // formatted again for each layout, which would be quadratic.
    let mut grouped = elements;
    if has_cached {
        let (argument, grouped_element) = match group_layout {
            GroupedCallArgumentLayout::GroupedFirstArgument => {
                (node.get(0).unwrap(), grouped.first_mut().unwrap())
            }
            GroupedCallArgumentLayout::GroupedLastArgument => {
                (node.get(last_index).unwrap(), grouped.last_mut().unwrap())
            }
        };
        let lambda = T::as_lambda(&argument).expect("a cached grouped argument is a lambda");
        let params = lambda.params.span;

        // Turns out, using the grouped layout isn't a good fit because some parameters of the
        // grouped lambda break. In that case, fall back to the all args expanded formatting.
        // This back tracking is required because testing if the grouped argument breaks would
        // also return `true` if any content of the lambda body breaks. But, as far as this is
        // concerned, it's only interested if any content in the signature breaks.
        let Some(cached_element) = f.context().get_cached_element(&params) else {
            unreachable!("The parameters should have already been formatted and cached");
        };

        // Remove soft lines from the cached parameters and check if they would break.
        // If they break even without soft lines, we need to use the expanded layout.
        let interned = f.intern(&format_once(|f| {
            f.remove_soft_lines(|f| f.write_element(cached_element));
        }));

        if let Some(interned) = interned {
            if interned.will_break() {
                return format_all_elements_broken_out(grouped.into_iter(), true, f);
            }

            // No break; it should print the element without soft lines.
            f.context_mut().cache_element(&params, interned);
        }

        *grouped_element = f.intern(&format_once(|f| {
            FormatLambdaExpression::new_with_options(
                &lambda,
                FormatLambdaExpressionOptions {
                    cache_mode: FunctionCacheMode::Cache,
                    call_argument_layout: Some(group_layout),
                    ..FormatLambdaExpressionOptions::default()
                },
            )
            .fmt(f);
            if last_index != 0 && group_layout.is_grouped_first() {
                ",".fmt(f);
            }
        }));
    }

    // Write the second variant that forces the group of the first/last argument to expand.
    let middle_variant = best_fitting_variant(f, |f| {
        write!(
            f,
            [
                "(",
                format_with(|f| {
                    let mut joiner = f.join_with(soft_line_break_or_space());

                    for (i, element) in grouped.iter().enumerate() {
                        if (group_layout.is_grouped_first() && i == 0)
                            || (group_layout.is_grouped_last() && i == last_index)
                        {
                            joiner.entry(
                                &group(&format_with(|f| {
                                    if let Some(arg_element) = element.clone() {
                                        f.write_element(arg_element);
                                    }
                                }))
                                .should_expand(true)
                                // The other arguments follow its closing line.
                                .fits_last_line(group_layout.is_grouped_first()),
                            );
                        } else {
                            joiner.entry(&format_with(|f| {
                                if let Some(arg_element) = element.clone() {
                                    f.write_element(arg_element);
                                }
                            }));
                        }
                    }
                }),
                ")"
            ]
        );
    });

    // If the grouped content breaks, then we can skip the most_flat variant,
    // since we already know that it won't be fitting on a single line.
    let variants = if grouped_breaks {
        write!(f, [expand_parent()]);
        Vec::from_iter_in([middle_variant, most_expanded], f)
    } else {
        // Write the most flat variant with the first or last argument grouped.
        let most_flat = best_fitting_variant(f, |f| {
            write!(
                f,
                [
                    "(",
                    format_once(|f| {
                        f.join_with(soft_line_break_or_space()).entries(grouped.into_iter().map(
                            |element| {
                                format_once(move |f| {
                                    if let Some(element) = element {
                                        f.write_element(element);
                                    }
                                })
                            },
                        ));
                    }),
                    ")",
                ]
            );
        });

        Vec::from_iter_in([most_flat, middle_variant, most_expanded], f)
    };

    // SAFETY: Safe because variants is guaranteed to contain at least 2 entries:
    // * most flat (optional)
    // * middle
    // * most expanded
    // ... and best fitting only requires the most flat/and expanded.
    unsafe {
        f.write_element(FormatElement::BestFitting(BestFittingElement::from_vec_unchecked(
            variants,
        )));
    }
}
