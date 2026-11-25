// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/print/arrow_function_expression.rs

use crate::ast_nodes::AstNode;
use crate::ast_nodes::generated::ast_nodes::AstNodeKind;
use crate::formatter::comments::stays_on_operator_line;
use crate::formatter::prelude::*;
use crate::formatter::trivia::FormatTrailingComments;
use crate::print::FormatWrite;
use crate::utils::assignment_like::AssignmentLikeLayout;
use crate::utils::statement_body::format_head_body;
use robusta_ast::{Comment, Expression, LambdaBody, LambdaExpression, match_expression};
use robusta_formatter_core::{format_args, write};
use robusta_span::{GetSpan, Span};
use robusta_syntax::line_terminator::contains_line_terminator;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, LambdaExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let options = FormatLambdaExpressionOptions {
            // Handed over by the assignment-like whose right side this lambda is.
            // The span key ensures only this lambda can consume it.
            assignment_layout: f.context_mut().take_lambda_assignment_layout(self.span),
            ..FormatLambdaExpressionOptions::default()
        };
        FormatLambdaExpression::new_with_options(self, options).fmt(f);
    }
}

#[derive(Clone, Copy)]
pub struct FormatLambdaExpression<'a, 'b> {
    lambda: &'b AstNode<'b, 'a, LambdaExpression<'a>>,
    options: FormatLambdaExpressionOptions,
}

#[derive(Default, Clone, Copy)]
pub struct FormatLambdaExpressionOptions {
    pub assignment_layout: Option<AssignmentLikeLayout>,
    pub call_argument_layout: Option<GroupedCallArgumentLayout>,
    // Determine whether the signature and body should be cached.
    pub cache_mode: FunctionCacheMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupedCallArgumentLayout {
    /// Group the first call argument.
    GroupedFirstArgument,

    /// Group the last call argument.
    GroupedLastArgument,
}

impl GroupedCallArgumentLayout {
    pub fn is_grouped_first(self) -> bool {
        matches!(self, GroupedCallArgumentLayout::GroupedFirstArgument)
    }

    pub fn is_grouped_last(self) -> bool {
        matches!(self, GroupedCallArgumentLayout::GroupedLastArgument)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum FunctionCacheMode {
    /// Format the body without caching it or retrieving it from the cache.
    #[default]
    NoCache,

    /// Cache the body during the next [formatting](Format::fmt).
    Cache,
}

impl<'a, 'b> FormatLambdaExpression<'a, 'b> {
    pub fn new_with_options(
        lambda: &'b AstNode<'b, 'a, LambdaExpression<'a>>,
        options: FormatLambdaExpressionOptions,
    ) -> Self {
        Self { lambda, options }
    }
}

impl<'a> Format<'a, JavaFormatContext<'a>> for FormatLambdaExpression<'a, '_> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let lambda = match LambdaLayout::for_lambda(self.lambda, self.options, f) {
            LambdaLayout::Chain(chain) => return chain.fmt(f),
            LambdaLayout::Single(lambda) => lambda,
        };
        let cache_mode = self.options.cache_mode;
        write!(
            f,
            [
                format_signature(
                    lambda,
                    self.options.call_argument_layout.is_some(),
                    true,
                    cache_mode
                ),
                space(),
                "->"
            ]
        );
        let format_body = format_body(lambda, cache_mode);

        // A lambda breaks after its `->` and an array creation inside its
        // braces, so neither needs a soft line break after the `->`:
        //
        //   names.map(name -> new String[] {
        //     name.first(),
        //     name.last(),
        //   });
        let body_has_soft_line_break = lambda.get_expression().is_none_or(|expression| {
            (is_hugging_body(expression)
                || is_multiline_text_block_starting_on_same_line(expression, f))
                && !has_own_line_comment_before_body(lambda, f)
        });

        if body_has_soft_line_break {
            write_space_before_body(lambda, f);
            format_body.fmt(f);
        } else {
            let is_last_call_arg = matches!(
                self.options.call_argument_layout,
                Some(GroupedCallArgumentLayout::GroupedLastArgument)
            );
            write!(
                f,
                [group(&format_args!(
                    soft_line_indent_or_space(&format_body),
                    is_last_call_arg.then_some(soft_line_break())
                ))]
            );
        }
    }
}

enum LambdaLayout<'a, 'b> {
    /// A lambda whose body is not a lambda.
    Single(&'b AstNode<'b, 'a, LambdaExpression<'a>>),

    /// A chain of at least two lambdas, each the body of the one before.
    ///
    /// A chain breaks after the `->` tokens:
    ///
    /// ```java
    /// Function<A, Function<B, C>> combine =
    ///   first ->
    ///   second ->
    ///     merge(first, second);
    /// ```
    Chain(LambdaChain<'a, 'b>),
}

impl<'a, 'b> LambdaLayout<'a, 'b> {
    /// Determines the layout for the passed lambda. See [LambdaLayout] for a
    /// description of the different layouts.
    fn for_lambda(
        lambda: &'b AstNode<'b, 'a, LambdaExpression<'a>>,
        options: FormatLambdaExpressionOptions,
        f: &JavaFormatter<'_, 'a>,
    ) -> LambdaLayout<'a, 'b> {
        let mut head = None;
        let mut middle = Vec::new();
        let mut current = lambda;
        let mut should_break = false;
        let is_non_grouped_or_grouped_last_argument = matches!(
            options.call_argument_layout,
            None | Some(GroupedCallArgumentLayout::GroupedLastArgument)
        );

        loop {
            if is_non_grouped_or_grouped_last_argument
                && let AstNodeKind::LambdaExpression(next) = current.body().as_kind()
            {
                should_break = should_break || Self::should_break_chain(current);

                should_break = should_break || Self::should_break_chain(&next);

                if head.is_none() {
                    head = Some(current);
                } else {
                    middle.push(current);
                }

                // In the arena, as it is the parent of the next lambda
                current = next.in_arena(f.allocator());
                continue;
            }
            break match head {
                None => LambdaLayout::Single(current),
                Some(head) => LambdaLayout::Chain(LambdaChain {
                    head,
                    middle,
                    tail: current,
                    expand_signatures: should_break,
                    options,
                }),
            };
        }
    }

    /// Whether the lambda's parameters are too complex to print a chain on
    /// one line: a varargs parameter reads as a rest parameter.
    fn should_break_chain(lambda: &LambdaExpression<'a>) -> bool {
        lambda.params.items.iter().any(|param| param.varargs)
    }
}

struct LambdaChain<'a, 'b> {
    /// The top most lambda in the chain
    head: &'b AstNode<'b, 'a, LambdaExpression<'a>>,

    /// The lambdas in the chain that are neither the first nor the last.
    /// Empty for chains consisting only of two lambdas.
    middle: Vec<&'b AstNode<'b, 'a, LambdaExpression<'a>>>,

    /// The last lambda in the chain
    tail: &'b AstNode<'b, 'a, LambdaExpression<'a>>,

    options: FormatLambdaExpressionOptions,

    /// Whether the group wrapping the signatures should be expanded or not.
    expand_signatures: bool,
}

impl<'a, 'b> LambdaChain<'a, 'b> {
    /// Returns an iterator over all lambdas in this chain
    fn lambdas(&self) -> impl Iterator<Item = &'b AstNode<'b, 'a, LambdaExpression<'a>>> {
        use std::iter::once;
        once(self.head).chain(self.middle.iter().copied()).chain(once(self.tail))
    }
}

impl<'a> Format<'a, JavaFormatContext<'a>> for LambdaChain<'a, '_> {
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        let LambdaChain { tail, expand_signatures, .. } = *self;

        let is_grouped_call_arg_layout = self.options.call_argument_layout.is_some();

        // An array creation or a block body opens with a brace, which stays
        // after the last `->` and saves a line holding only the punctuation:
        //
        //   foo -> bar -> new int[] {
        //     a,
        //     b,
        //   }
        //
        // Any other body takes a line of its own when it breaks.
        let body_on_separate_line = !tail.get_expression().is_none_or(is_hugging_body);

        // An own-line comment before an expression tail body forces the body
        // onto its own line. Before a block, it moves after the `{`.
        let tail_body_has_leading_own_line_comment =
            tail.get_expression().is_some() && has_own_line_comment_before_body(tail, f);
        let body_on_separate_line = body_on_separate_line || tail_body_has_leading_own_line_comment;

        // Each non-tail lambda's "body" is the next lambda in the chain,
        // so this covers own-line comments before any chain link as well as before the tail body.
        let should_expand_grouped_tail = is_grouped_call_arg_layout
            && (tail_body_has_leading_own_line_comment
                || std::iter::once(self.head)
                    .chain(self.middle.iter().copied())
                    .any(|lambda| has_own_line_comment_before_body(lambda, f)));

        // As the right side of an assignment chain, the signatures break first.
        let break_signatures =
            matches!(self.options.assignment_layout, Some(AssignmentLikeLayout::ChainTailLambda));

        // Lambda chains as the right side of an assignment indent the entire
        // signature chain a single level and do _not_ indent a second level
        // for additional signatures after the first:
        //   foo =
        //     a ->
        //     b ->
        //     c ->
        //       0;
        // This tracks that state and is used to prevent the insertion of
        // additional indents under `format_lambda_signatures`, then also to
        // add the outer indent under `format_inner`.
        let has_initial_indent = self
            .options
            .assignment_layout
            .is_some_and(|layout| layout != AssignmentLikeLayout::BreakAfterOperator);

        let format_lambda_signatures = format_with(|f| {
            let join_signatures = format_with(|f| {
                let mut is_first_in_chain = true;
                for lambda in self.lambdas() {
                    // The first comment in the chain gets formatted by the
                    // parent (the generated lambda `fmt`), but the rest of the
                    // lambdas in the chain need to format their comments
                    // manually, since they won't have their own Format node to
                    // handle it.
                    let should_format_comments = !is_first_in_chain
                        && f.context().comments().has_comment_before(lambda.span.start);
                    let is_first = is_first_in_chain;

                    let formatted_signature = format_with(|f| {
                        let format_leading_comments = format_with(|f| {
                            if should_format_comments {
                                // A grouped layout implies that the lambda chain is trying to be rendered
                                // in a condensed, single-line format (at least the signatures, not
                                // necessarily the body). In that case, we _need_ to prevent the leading
                                // comments from inserting line breaks. But if it's _not_ a grouped layout,
                                // then we want to _force_ the line break so that the leading comments
                                // don't inadvertently end up on the previous line after the arrow.
                                if is_grouped_call_arg_layout {
                                    space().fmt(f);
                                } else {
                                    soft_line_break_or_space().fmt(f);
                                }
                                lambda.format_leading_comments(f);
                            }
                        });

                        let start = lambda.span.start;
                        write!(
                            f,
                            [
                                FormatContentWithCacheMode::new(
                                    Span::empty(start),
                                    format_leading_comments,
                                    self.options.cache_mode,
                                ),
                                format_signature(
                                    lambda,
                                    is_grouped_call_arg_layout,
                                    is_first,
                                    self.options.cache_mode
                                )
                            ]
                        );
                    });

                    // Lambda chains indent a second level for every item other than the first:
                    //   a ->
                    //     b ->
                    //     c ->
                    //       0
                    // Because the chain is printed as a flat list, each entry needs to set
                    // its own indention. This ensures that the first item keeps the same
                    // level as the surrounding content, and then each subsequent item has
                    // one additional level, as shown above.
                    if is_first_in_chain || has_initial_indent {
                        is_first_in_chain = false;
                        write!(f, [formatted_signature]);
                    } else {
                        write!(f, [indent(&formatted_signature)]);
                    }

                    // The arrow of the tail is formatted outside of the group to ensure it never
                    // breaks from the body
                    if !std::ptr::eq(lambda, tail) {
                        write!(f, [space(), "->"]);
                    }
                }
            });

            group(&join_signatures).should_expand(expand_signatures).fmt(f);
        });

        let format_tail_body_inner = format_body(tail, self.options.cache_mode);

        let format_tail_body = format_with(|f| {
            if body_on_separate_line {
                write!(f, [soft_line_indent_or_space(&format_tail_body_inner)]);
            } else {
                write_space_before_body(tail, f);
                format_tail_body_inner.fmt(f);
            }
        });

        let group_id = f.state().group_id("lambda-chain");

        let format_inner = format_with(|f| {
            if has_initial_indent {
                write!(
                    f,
                    [group(&indent(&format_args!(soft_line_break(), format_lambda_signatures)))
                        .with_group_id(Some(group_id))
                        .should_expand(break_signatures)]
                );
            } else {
                write!(
                    f,
                    group(&format_lambda_signatures)
                        .with_group_id(Some(group_id))
                        .should_expand(break_signatures)
                );
            }

            write!(f, [space(), "->"]);

            if is_grouped_call_arg_layout {
                // An own-line comment in the chain breaks the body below the
                // last `->` and puts the closing `)` on its own line, as an
                // ungrouped argument would.
                if should_expand_grouped_tail {
                    write!(
                        f,
                        [group(&format_args!(
                            indent_if_group_breaks(&format_tail_body, group_id),
                            soft_line_break()
                        ))
                        .should_expand(true)]
                    );
                } else {
                    write!(f, [group(&format_tail_body)]);
                }
            } else {
                write!(f, [indent_if_group_breaks(&format_tail_body, group_id)]);
            }
        });

        write!(f, [group(&format_inner)]);
    }
}

/// Whether the body breaks inside itself and can start after the `->`: a
/// lambda, or an array creation with an initializer.
fn is_hugging_body(expression: &Expression<'_>) -> bool {
    match expression {
        Expression::Lambda(_) => true,
        Expression::ArrayCreation(creation) => creation.init.is_some(),
        _ => false,
    }
}

/// Whether `expression` is a text block that starts on the line of the token
/// before it and spans lines as written, not only through `\u000a` escapes.
///
/// ```java
/// () -> """
///     some content
///     """
/// ```
fn is_multiline_text_block_starting_on_same_line(
    expression: &Expression<'_>,
    f: &JavaFormatter<'_, '_>,
) -> bool {
    let Expression::StringLiteral(literal) = expression else {
        return false;
    };
    literal.is_text_block()
        && contains_line_terminator(literal.raw)
        && !f.source_text().starts_line(expression.span().start)
}

/// Whether an own-line comment sits between the lambda's signature and its
/// body, other than one staying before the `->`.
///
/// Queried by position because as a grouped call argument this decision runs
/// again in the re-format pass, after the first pass claimed the comments.
fn has_own_line_comment_before_body<'a>(
    lambda: &AstNode<'_, 'a, LambdaExpression<'a>>,
    f: &JavaFormatter<'_, 'a>,
) -> bool {
    let comments = f.comments();
    let signature_end = lambda.params.span.end;
    let arrow_start = comments.position_after_character(signature_end, b'-') - 1;
    comments
        .all_comments_in_range(signature_end, body_span(lambda).start)
        .skip_while(|comment| comment.span.start < arrow_start && stays_before_arrow(comment))
        .any(Comment::followed_by_newline)
}

/// Whether `comment`, from the run before a lambda's `->`, stays there. A line
/// comment would need a line break before the `->`, so it leads the body with
/// the comments after the `->`.
fn stays_before_arrow(comment: &Comment) -> bool {
    comment.is_block() && stays_on_operator_line(comment)
}

/// The separator before an expression body that starts after the `->`. A
/// block body writes its own.
fn write_space_before_body(lambda: &LambdaExpression<'_>, f: &mut JavaFormatter<'_, '_>) {
    if lambda.get_expression().is_some() {
        space().fmt(f);
    }
}

/// The lambda's body. A block places the comments before its `{` through
/// [`format_head_body`].
fn format_body<'a, 'b>(
    lambda: &'b AstNode<'b, 'a, LambdaExpression<'a>>,
    cache_mode: FunctionCacheMode,
) -> impl Format<'a, JavaFormatContext<'a>> + 'b {
    let content = format_with(move |f| {
        let body = lambda.body();
        match body.as_block() {
            Some(block) => format_head_body(&block, f),
            None => body.fmt(f),
        }
    });
    FormatContentWithCacheMode::new(body_span(lambda), content, cache_mode)
}

fn body_span(lambda: &LambdaExpression<'_>) -> Span {
    match &lambda.body {
        LambdaBody::Block(block) => block.span,
        it @ match_expression!(LambdaBody) => it.to_expression().span(),
    }
}

/// Writes the lambda's parameters and the comments before its `->`.
///
/// Formats the parameters without any soft line breaks if `is_grouped_call_argument` is `true`
/// so that the parameters are kept on the same line.
fn format_signature<'a, 'b>(
    lambda: &'b AstNode<'b, 'a, LambdaExpression<'a>>,
    is_grouped_call_argument: bool,
    is_first_in_chain: bool,
    cache_mode: FunctionCacheMode,
) -> impl Format<'a, JavaFormatContext<'a>> + 'b {
    format_with(move |f| {
        let header_span = lambda.params.span;
        let content = format_with(|f| group(&lambda.params()).fmt(f));
        let format_head = FormatContentWithCacheMode::new(header_span, content, cache_mode);

        if is_grouped_call_argument {
            // The first lambda's soft lines have already been removed in the call arguments.
            if is_first_in_chain {
                write!(f, format_head);
            } else {
                write!(f, [space()]);
                f.remove_soft_lines(|f| write!(f, format_head));
            }
        } else {
            write!(
                f,
                [
                    // This soft break is placed outside of the group to ensure
                    // that the parameter group only tries to write on a single
                    // line and can't break pre-emptively without also causing
                    // the parent (i.e., this LambdaChain) to break first.
                    (!is_first_in_chain).then_some(soft_line_break_or_space()),
                    format_head
                ]
            );
        }

        // Comments before `->` that stay on the signature's line trail it, and the rest lead the body.
        let comments_before_arrow = format_with(|f| {
            let comments = f.comments().comments_before_character(header_span.end, b'-');
            let count = comments.iter().take_while(|comment| stays_before_arrow(comment)).count();
            FormatTrailingComments::Comments(&comments[..count]).fmt(f);
        });
        write!(
            f,
            [FormatContentWithCacheMode::new(lambda.span, comments_before_arrow, cache_mode)]
        );
    })
}

// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/print/function.rs
/// A wrapper that formats content and caches the result based on the given cache mode.
///
/// A grouped call argument is formatted again for each layout the arguments try,
/// but a comment can only be printed once. The first formatting caches its
/// result, and the later ones reuse it.
pub struct FormatContentWithCacheMode<T> {
    key: Span,
    content: T,
    cache_mode: FunctionCacheMode,
}

impl<T> FormatContentWithCacheMode<T> {
    pub fn new(key: Span, content: T, cache_mode: FunctionCacheMode) -> Self {
        Self { key, content, cache_mode }
    }
}

impl<'a, T> Format<'a, JavaFormatContext<'a>> for FormatContentWithCacheMode<T>
where
    T: Format<'a, JavaFormatContext<'a>>,
{
    fn fmt(&self, f: &mut JavaFormatter<'_, 'a>) {
        if matches!(self.cache_mode, FunctionCacheMode::NoCache) {
            self.content.fmt(f);
        } else if let Some(grouped) = f.context().get_cached_element(&self.key) {
            f.write_element(grouped);
        } else if let Some(grouped) = f.intern(&self.content) {
            f.context_mut().cache_element(&self.key, grouped.clone());
            f.write_element(grouped);
        }
    }
}
