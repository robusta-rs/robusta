use crate::ast_nodes::{AstNode, AstNodes};
use crate::formatter::prelude::*;
use crate::formatter::separated::format_undelimited_list;
use crate::formatter::trivia::FormatTrailingComments;
use crate::print::binary_like_expression::format_operator_and_break;
use crate::print::semicolon::FormatContentWithSemicolon;
use crate::print::{FormatWrite, format_header_condition};
use crate::utils::format_node_without_trailing_comments::format_content_without_comments_after;
use crate::utils::statement_body::{
    FormatStatementBody, comments_staying_before_brace, format_attached_braced_body,
    format_head_body,
};
use robusta_ast::{
    CaseLabel, CasePatternLabel, DefaultLabel, Statement, SwitchBlockStatementGroup,
    SwitchExpression, SwitchLabel, SwitchRule, SwitchRuleBody, SwitchStatement, match_expression,
};
use robusta_formatter_core::write;
use robusta_span::GetSpan;

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, SwitchStatement<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_switch(f, self.as_ast_nodes());
    }
}
impl<'a> FormatWrite<'a> for AstNode<'_, 'a, SwitchExpression<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        format_switch(f, self.as_ast_nodes());
    }
}

fn format_switch<'a>(f: &mut JavaFormatter<'_, 'a>, switch: AstNodes<'_, 'a>) {
    let (span, selector, cases) = match switch {
        AstNodes::SwitchStatement(node) => (node.span, node.selector(), node.cases()),
        AstNodes::SwitchExpression(node) => (node.span, node.selector(), node.cases()),
        _ => unreachable!("a switch header belongs to its switch"),
    };
    "switch".fmt(f);
    space().fmt(f);
    format_header_condition(f, span.start, &selector);
    let selector_end = f.comments().position_after_character(selector.span().end, b')');
    format_attached_braced_body(f, selector_end, cases.is_empty(), span, |f| {
        f.join_nodes_with_hardline().entries(cases.iter());
    });
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, SwitchBlockStatementGroup<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let lone_block = matches!(node.statements[..], [Statement::Block(_)]);
        for (index, label) in self.labels().iter().enumerate() {
            if index > 0 {
                hard_line_break().fmt(f);
            }
            if lone_block && index + 1 == node.labels.len() {
                format_label_before_block(&label, b':', f);
                ":".fmt(f);
            } else {
                write!(f, [label, ":"]);
            }
        }
        if node.statements.is_empty() {
            return;
        }

        // A lone block hugs the labels. With more statements it cannot, so the
        // group visibly continues past the block's `}`.
        let statements = self.statements();
        if lone_block {
            FormatStatementBody::new(&statements.get(0).expect("one statement")).fmt(f);
            return;
        }

        // The group has no braces, so it owns the break that puts its first
        // statement on its own line; a braced body gets that from its frame.
        write!(
            f,
            [indent(&format_once(|f| {
                hard_line_break().fmt(f);
                f.join_nodes_with_hardline().entries(statements.iter());
            }))]
        );
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, SwitchRule<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        let label = self.label();
        if matches!(node.body, SwitchRuleBody::Block(_)) {
            format_label_before_block(&label, b'-', f);
        } else {
            label.fmt(f);
        }
        group(&indent(&format_once(|f| {
            write!(f, [soft_line_break_or_space(), "->"]);
        })))
        .fmt(f);
        let body = self.body();
        match &node.body {
            SwitchRuleBody::Block(_) => {
                format_head_body(&body.as_block().expect("block variant"), f);
            }
            SwitchRuleBody::Throw(_) => format_arrow_body(f, |f| body.fmt(f)),
            // The `;` belongs to the rule (JLS 14.11.1).
            it @ match_expression!(SwitchRuleBody) => {
                let content = format_with(|f| format_arrow_body(f, |f| body.fmt(f)));
                let end = it.to_expression().span().end;
                FormatContentWithSemicolon::new(&content, end, node.span.end)
                    .moving_documentation_comments()
                    .with_postfix_comments(it.to_expression(), f)
                    .fmt(f);
            }
        }
    }
}

/// Print the `label` of a block, with the comments before the `separator`
/// that follows it (`:` or `->`) that stay there. Those that would end the
/// label's line move after the block's `{` with the comments after the
/// separator, as [`format_head_body`] places them.
fn format_label_before_block<'a>(
    label: &AstNode<'_, 'a, SwitchLabel<'a>>,
    separator: u8,
    f: &mut JavaFormatter<'_, 'a>,
) {
    let end = label.span().end;
    format_content_without_comments_after(label, end, f);
    let comments = f.comments().comments_before_character(end, separator);
    let staying = comments_staying_before_brace(comments);
    FormatTrailingComments::Comments(&comments[..staying]).fmt(f);
}

/// Format a switch-rule value after `->`: beside the arrow if it fits,
/// otherwise on an indented line.
fn format_arrow_body<'ast>(
    f: &mut JavaFormatter<'_, 'ast>,
    render: impl FnOnce(&mut JavaFormatter<'_, 'ast>),
) {
    group(&soft_line_indent_or_space(&format_once(render))).fmt(f);
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, CaseLabel<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        let node = self.inner();
        write!(f, ["case", space()]);
        let constants = self.constants();
        let mut constants_iter = constants.iter();
        format_undelimited_list(f, false, node.constants.len(), |f, index| {
            let constant = constants_iter.next().expect("case constant");
            if index + 1 < constants.len() || node.has_default {
                write!(f, [constant, ",", line_suffix_boundary()]);
            } else {
                constant.fmt(f);
            }
            if index + 1 == constants.len() && node.has_default {
                write!(f, [space(), "default"]);
            }
        });
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, CasePatternLabel<'a>> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        write!(f, ["case", space()]);
        let count = self.patterns.len();
        let guard = self.guard();
        // The guard follows the patterns as a binary operand follows the
        // operator, so a line comment before `when` breaks the line there.
        group(&format_once(|f| {
            format_undelimited_list(f, false, count, |f, index| {
                let pattern = self.patterns().get(index).expect("pattern");
                pattern.fmt(f);
                if index + 1 < count {
                    ",".fmt(f);
                }
            });
            if let Some(guard) = &guard {
                let when = format_once(|f| {
                    format_operator_and_break("when", guard.span(), f);
                    guard.fmt(f);
                });
                indent(&when).fmt(f);
            }
        }))
        .fmt(f);
    }
}

impl<'a> FormatWrite<'a> for AstNode<'_, 'a, DefaultLabel> {
    fn write(&self, f: &mut JavaFormatter<'_, 'a>) {
        "default".fmt(f);
    }
}
