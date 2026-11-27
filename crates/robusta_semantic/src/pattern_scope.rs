use robusta_allocator::GetAddress;
use robusta_ast::{
    AstKind, DoStatement, Expression, IdentifierReference, LabeledStatement, Statement, SwitchCase,
    SwitchLabel, SwitchRuleBody, SwitchStatement, match_expression,
};
use robusta_span::{GetSpan, Span};
use robusta_syntax::operator::{BinaryOp, UnaryOp};

use crate::{AstNodes, NodeId, ScopeId};

/// What introduces a pattern variable on the way out of its pattern (JLS 6.3.1–6.3.2).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Introduced {
    WhenTrue,
    WhenFalse,
    ByStatement,
}

/// Where a pattern variable is in scope (JLS 6.3).
pub struct PatternScope<'a> {
    /// The scope the variable binds in.
    pub scope_id: ScopeId,
    /// The parts of it where the variable is definitely matched, in source order.
    pub parts: Vec<Span>,
    /// The unmatched branch of an `if`, when the last part holds only if the
    /// branch cannot complete normally. That can depend on any declaration in
    /// the file, so it is decided later.
    pub unmatched: Option<&'a Statement<'a>>,
}

/// Where the variable of the pattern at `pattern_id` is in scope. `scope_id` is
/// the scope the pattern is in.
///
/// Every statement counts as reachable, as it is in code that compiles.
pub fn pattern_scope<'a>(
    nodes: &AstNodes<'a>,
    pattern_id: NodeId,
    mut scope_id: ScopeId,
) -> PatternScope<'a> {
    let mut parts = Vec::new();
    let mut unmatched = None;
    let mut introduced = Introduced::WhenTrue;
    let mut child = nodes.kind(pattern_id);
    for (id, node) in nodes.ancestors_enumerated(pattern_id) {
        match node.kind() {
            AstKind::RecordPattern(_) | AstKind::ParenthesizedExpression(_) => {}
            AstKind::InstanceofExpression(instanceof)
                if instanceof.operand.address() != child.address() => {}
            AstKind::UnaryExpression(unary) if unary.op == UnaryOp::Not => {
                introduced = match introduced {
                    Introduced::WhenTrue => Introduced::WhenFalse,
                    _ => Introduced::WhenTrue,
                };
            }
            AstKind::BinaryExpression(binary)
                if matches!(binary.op, BinaryOp::And | BinaryOp::Or) =>
            {
                // `a && b` introduces only what holds when true, `a || b` when false.
                let kept = if binary.op == BinaryOp::And {
                    Introduced::WhenTrue
                } else {
                    Introduced::WhenFalse
                };
                if introduced != kept {
                    break;
                }
                if binary.lhs.address() == child.address() {
                    parts.push(binary.rhs.span());
                }
            }
            AstKind::ConditionalExpression(conditional)
                if conditional.condition.address() == child.address() =>
            {
                let branch = if introduced == Introduced::WhenTrue {
                    &conditional.when_true
                } else {
                    &conditional.when_false
                };
                parts.push(branch.span());
                break;
            }
            AstKind::CasePatternLabel(label) => {
                // The label's patterns, and its guard when true, reach the rest of
                // its rule or group.
                let is_guard =
                    label.guard.as_ref().is_some_and(|guard| guard.address() == child.address());
                if !is_guard || introduced == Introduced::WhenTrue {
                    parts.push(Span::new(child.span().end, nodes.parent_kind(id).span().end));
                }
                break;
            }
            AstKind::IfStatement(statement) if statement.condition.address() == child.address() => {
                // Code after an abrupt `if` is unreachable, so the unmatched branch decides.
                let branch = match (&statement.else_branch, introduced) {
                    (None, Introduced::WhenTrue) => {
                        parts.push(statement.then_branch.span());
                        break;
                    }
                    (None, _) => &statement.then_branch,
                    (Some(else_branch), Introduced::WhenTrue) => {
                        parts.push(statement.then_branch.span());
                        else_branch
                    }
                    (Some(else_branch), _) => {
                        parts.push(else_branch.span());
                        &statement.then_branch
                    }
                };
                unmatched = Some((branch, parts.len()));
                introduced = Introduced::ByStatement;
            }
            AstKind::WhileStatement(statement)
                if statement.condition.address() == child.address() =>
            {
                if introduced == Introduced::WhenTrue {
                    parts.push(statement.body.span());
                    break;
                }
                if has_jump(&statement.body, JumpTarget::BREAK, &Unresolved) {
                    break;
                }
                introduced = Introduced::ByStatement;
            }
            AstKind::DoStatement(statement) if statement.condition.address() == child.address() => {
                if introduced == Introduced::WhenTrue
                    || has_jump(&statement.body, JumpTarget::BREAK, &Unresolved)
                {
                    break;
                }
                introduced = Introduced::ByStatement;
            }
            AstKind::ForStatement(statement)
                if statement
                    .condition
                    .as_ref()
                    .is_some_and(|condition| condition.address() == child.address()) =>
            {
                if introduced == Introduced::WhenTrue {
                    let start =
                        statement.update.as_ref().map_or(statement.body.span(), GetSpan::span);
                    parts.push(Span::new(start.start, statement.body.span().end));
                    break;
                }
                if has_jump(&statement.body, JumpTarget::BREAK, &Unresolved) {
                    break;
                }
                introduced = Introduced::ByStatement;
                // The statement's own scope ends with it.
                scope_id = node.scope_id();
            }
            AstKind::LabeledStatement(statement) if introduced == Introduced::ByStatement => {
                let labels = [statement.label.name];
                if has_jump(&statement.statement, JumpTarget::labeled_break(&labels), &Unresolved) {
                    break;
                }
            }
            AstKind::Block(block) if introduced == Introduced::ByStatement => {
                parts.push(Span::new(child.span().end, block.span.end));
                break;
            }
            AstKind::SwitchBlockStatementGroup(group) if introduced == Introduced::ByStatement => {
                parts.push(Span::new(child.span().end, group.span.end));
                break;
            }
            _ => break,
        }
        child = node.kind();
    }
    // Only a part after the `if` waits on its unmatched branch.
    let unmatched = unmatched.and_then(|(branch, len)| (parts.len() > len).then_some(branch));
    PatternScope { scope_id, parts, unmatched }
}

/// What a simple name in a constant expression refers to.
pub enum NameValue<'a> {
    /// A `final` variable of primitive or `String` type declared at `start`: a
    /// constant variable when its initializer is a constant expression
    /// (JLS 4.12.4).
    Initialized { start: u32, initializer: &'a Expression<'a> },
    /// A variable that is never a constant variable.
    Variable,
    /// A name the file does not settle, which may be a constant variable.
    Unknown,
}

/// Follows simple names to their file-local declarations.
pub trait NameContext<'a> {
    fn name_value(&self, _name: &IdentifierReference<'a>) -> NameValue<'a> {
        NameValue::Unknown
    }
}

/// Resolves no name, leaving each one possibly a constant.
pub struct Unresolved;

impl NameContext<'_> for Unresolved {}

/// Whether `statement` is known to complete normally (JLS 14.22).
pub fn can_complete_normally<'a>(statement: &Statement<'a>, names: &impl NameContext<'a>) -> bool {
    match statement {
        Statement::Block(block) => block_can_complete_normally(&block.statements, names),
        Statement::If(statement) => statement.else_branch.as_ref().is_none_or(|else_branch| {
            can_complete_normally(&statement.then_branch, names)
                || can_complete_normally(else_branch, names)
        }),
        Statement::Switch(statement) => switch_can_complete_normally(statement, names),
        Statement::While(statement) => {
            !may_be_constant_true(&statement.condition, names)
                || has_jump(&statement.body, JumpTarget::EXITING_BREAK, names)
        }
        Statement::Do(statement) => do_can_complete_normally(statement, &[], names),
        Statement::For(statement) => {
            statement
                .condition
                .as_ref()
                .is_some_and(|condition| !may_be_constant_true(condition, names))
                || has_jump(&statement.body, JumpTarget::EXITING_BREAK, names)
        }
        Statement::Labeled(statement) => labeled_can_complete_normally(statement, names),
        Statement::Try(statement) => {
            (block_can_complete_normally(&statement.block.statements, names)
                || statement
                    .catches
                    .iter()
                    .any(|catch| block_can_complete_normally(&catch.block.statements, names)))
                && statement
                    .finally_block
                    .as_ref()
                    .is_none_or(|block| block_can_complete_normally(&block.statements, names))
        }
        Statement::Synchronized(statement) => {
            block_can_complete_normally(&statement.body.statements, names)
        }
        Statement::Break(_)
        | Statement::Continue(_)
        | Statement::Throw(_)
        | Statement::Return(_)
        | Statement::Yield(_) => false,
        Statement::Empty(_)
        | Statement::Class(_)
        | Statement::Interface(_)
        | Statement::Enum(_)
        | Statement::Record(_)
        | Statement::ConstructorInvocation(_)
        | Statement::LocalVariable(_)
        | Statement::Expression(_)
        | Statement::EnhancedFor(_)
        | Statement::Assert(_) => true,
    }
}

/// Whether a block holding `statements` is known to complete normally (JLS 14.22).
pub fn block_can_complete_normally<'a>(
    statements: &[Statement<'a>],
    names: &impl NameContext<'a>,
) -> bool {
    statements.last().is_none_or(|statement| can_complete_normally(statement, names))
}

/// `L1: L2: do …` gives the `do` statement both labels.
fn labeled_can_complete_normally<'a>(
    statement: &LabeledStatement<'a>,
    names: &impl NameContext<'a>,
) -> bool {
    let mut labels = vec![statement.label.name];
    let mut inner = &statement.statement;
    while let Statement::Labeled(next) = inner {
        labels.push(next.label.name);
        inner = &next.statement;
    }
    let inner_completes = match inner {
        Statement::Do(inner) => do_can_complete_normally(inner, &labels, names),
        inner => can_complete_normally(inner, names),
    };
    inner_completes
        || has_jump(
            &statement.statement,
            JumpTarget { exits: true, ..JumpTarget::labeled_break(&labels) },
            names,
        )
}

fn do_can_complete_normally<'a>(
    statement: &DoStatement<'a>,
    labels: &[&str],
    names: &impl NameContext<'a>,
) -> bool {
    let continues = JumpTarget { is_continue: true, unlabeled: true, labels, exits: true };
    (!may_be_constant_true(&statement.condition, names)
        && (can_complete_normally(&statement.body, names)
            || has_jump(&statement.body, continues, names)))
        || has_jump(&statement.body, JumpTarget::EXITING_BREAK, names)
}

fn switch_can_complete_normally<'a>(
    statement: &SwitchStatement<'a>,
    names: &impl NameContext<'a>,
) -> bool {
    let breaks = |statement: &Statement<'a>| has_jump(statement, JumpTarget::EXITING_BREAK, names);
    let cases = &statement.cases;
    cases.last().is_none_or(|case| match case {
        SwitchCase::Rule(_) => false,
        SwitchCase::Group(group) => block_can_complete_normally(&group.statements, names),
    }) || cases.iter().any(|case| match case {
        SwitchCase::Rule(rule) => match &rule.body {
            match_expression!(SwitchRuleBody) => true,
            SwitchRuleBody::Block(block) => {
                block_can_complete_normally(&block.statements, names)
                    || block.statements.iter().any(breaks)
            }
            SwitchRuleBody::Throw(_) => false,
        },
        SwitchCase::Group(group) => group.statements.iter().any(breaks),
    }) || may_match_no_label(
        cases.iter().flat_map(|case| match case {
            SwitchCase::Rule(rule) => std::slice::from_ref(&rule.label),
            SwitchCase::Group(group) => &group.labels[..],
        }),
        names,
    )
}

/// A `switch` statement that is neither enhanced nor has a `default` label may
/// match none (JLS 14.11.2). Without the selector's type, only a constant
/// expression among the case constants shows it is not enhanced. A name may be
/// an enum constant unless it refers to a constant variable.
fn may_match_no_label<'s, 'a: 's>(
    labels: impl Iterator<Item = &'s SwitchLabel<'a>>,
    names: &impl NameContext<'a>,
) -> bool {
    let mut not_enhanced = false;
    for label in labels {
        let SwitchLabel::Case(case) = label else {
            return false;
        };
        if case.has_default {
            return false;
        }
        for constant in &case.constants {
            match constant {
                Expression::NullLiteral(_) => return false,
                Expression::Identifier(name) => {
                    not_enhanced |= matches!(names.name_value(name), NameValue::Initialized { .. });
                }
                // A dotted name stays unknown.
                it if it.is_dotted_name() => {}
                _ => not_enhanced = true,
            }
        }
    }
    not_enhanced
}

/// Whether `expression` may be a constant expression with value `true` (JLS 15.29).
fn may_be_constant_true<'a>(expression: &Expression<'a>, names: &impl NameContext<'a>) -> bool {
    constant_boolean(expression, names, u32::MAX)
        .unwrap_or_else(|| may_be_constant(expression, names, u32::MAX))
}

/// Whether `expression` holds only what a constant expression may hold.
///
/// Names are followed only to declarations that start before `before`, which
/// ends a chain of initializers even in code that does not compile.
fn may_be_constant<'a>(
    expression: &Expression<'a>,
    names: &impl NameContext<'a>,
    before: u32,
) -> bool {
    match expression {
        Expression::IntLiteral(_)
        | Expression::FloatLiteral(_)
        | Expression::CharLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_) => true,
        Expression::Identifier(name) => match names.name_value(name) {
            NameValue::Initialized { start, initializer } if start < before => {
                may_be_constant(initializer, names, start)
            }
            NameValue::Variable => false,
            _ => true,
        },
        // A dotted name stays unknown.
        it if it.is_dotted_name() => true,
        Expression::Parenthesized(parenthesized) => {
            may_be_constant(&parenthesized.expression, names, before)
        }
        Expression::Cast(cast) => may_be_constant(&cast.operand, names, before),
        Expression::Unary(unary) => may_be_constant(&unary.operand, names, before),
        Expression::Binary(binary) => {
            may_be_constant(&binary.lhs, names, before)
                && may_be_constant(&binary.rhs, names, before)
        }
        Expression::Conditional(conditional) => {
            may_be_constant(&conditional.condition, names, before)
                && may_be_constant(&conditional.when_true, names, before)
                && may_be_constant(&conditional.when_false, names, before)
        }
        _ => false,
    }
}

fn constant_boolean<'a>(
    expression: &Expression<'a>,
    names: &impl NameContext<'a>,
    before: u32,
) -> Option<bool> {
    match expression {
        Expression::BooleanLiteral(literal) => Some(literal.value),
        Expression::Identifier(name) => match names.name_value(name) {
            NameValue::Initialized { start, initializer } if start < before => {
                constant_boolean(initializer, names, start)
            }
            _ => None,
        },
        Expression::Parenthesized(parenthesized) => {
            constant_boolean(&parenthesized.expression, names, before)
        }
        Expression::Unary(unary) if unary.op == UnaryOp::Not => {
            constant_boolean(&unary.operand, names, before).map(|value| !value)
        }
        Expression::Binary(binary) => {
            let lhs = constant_boolean(&binary.lhs, names, before)?;
            let rhs = constant_boolean(&binary.rhs, names, before)?;
            match binary.op {
                BinaryOp::And | BinaryOp::BitAnd => Some(lhs && rhs),
                BinaryOp::Or | BinaryOp::BitOr => Some(lhs || rhs),
                BinaryOp::BitXor | BinaryOp::NotEq => Some(lhs != rhs),
                BinaryOp::Eq => Some(lhs == rhs),
                _ => None,
            }
        }
        _ => None,
    }
}

/// The `break` or `continue` statements that reach one statement (JLS 14.15–14.16).
#[derive(Clone, Copy)]
struct JumpTarget<'l> {
    is_continue: bool,
    /// Whether a jump without a label still reaches the target.
    unlabeled: bool,
    labels: &'l [&'l str],
    /// Count only a jump that exits the target (JLS 14.22).
    exits: bool,
}

impl<'l> JumpTarget<'l> {
    const BREAK: Self = Self { is_continue: false, unlabeled: true, labels: &[], exits: false };
    const EXITING_BREAK: Self = Self { exits: true, ..Self::BREAK };

    const fn labeled_break(labels: &'l [&'l str]) -> Self {
        Self { unlabeled: false, labels, ..Self::BREAK }
    }

    fn reaches(self, is_continue: bool, label: Option<&str>) -> bool {
        self.is_continue == is_continue
            && label.map_or(self.unlabeled, |label| self.labels.contains(&label))
    }

    /// Inside a nested loop, or a `switch` for `break`, an unlabeled jump stops there.
    fn nested(self, is_loop: bool) -> Self {
        Self { unlabeled: self.unlabeled && !is_loop && self.is_continue, ..self }
    }
}

fn has_jump<'a>(
    statement: &Statement<'a>,
    target: JumpTarget<'_>,
    names: &impl NameContext<'a>,
) -> bool {
    let in_block = |statements: &[Statement<'a>], target| {
        statements.iter().any(|statement| has_jump(statement, target, names))
    };
    match statement {
        Statement::Break(statement) => {
            target.reaches(false, statement.label.as_ref().map(|label| label.name))
        }
        Statement::Continue(statement) => {
            target.reaches(true, statement.label.as_ref().map(|label| label.name))
        }
        Statement::Block(block) => in_block(&block.statements, target),
        Statement::If(statement) => {
            has_jump(&statement.then_branch, target, names)
                || statement
                    .else_branch
                    .as_ref()
                    .is_some_and(|branch| has_jump(branch, target, names))
        }
        Statement::Labeled(statement) => has_jump(&statement.statement, target, names),
        Statement::Synchronized(statement) => in_block(&statement.body.statements, target),
        Statement::Try(statement) => {
            // An abrupt `finally` block stops a jump out of the `try` block.
            let finally_completes = statement
                .finally_block
                .as_ref()
                .is_none_or(|block| block_can_complete_normally(&block.statements, names));
            ((finally_completes || !target.exits) && in_block(&statement.block.statements, target))
                || statement.catches.iter().any(|catch| in_block(&catch.block.statements, target))
                || statement
                    .finally_block
                    .as_ref()
                    .is_some_and(|block| in_block(&block.statements, target))
        }
        Statement::Switch(statement) => {
            let target = target.nested(false);
            statement.cases.iter().any(|case| match case {
                SwitchCase::Rule(rule) => match &rule.body {
                    SwitchRuleBody::Block(block) => in_block(&block.statements, target),
                    _ => false,
                },
                SwitchCase::Group(group) => in_block(&group.statements, target),
            })
        }
        Statement::While(statement) => has_jump(&statement.body, target.nested(true), names),
        Statement::Do(statement) => has_jump(&statement.body, target.nested(true), names),
        Statement::For(statement) => has_jump(&statement.body, target.nested(true), names),
        Statement::EnhancedFor(statement) => has_jump(&statement.body, target.nested(true), names),
        _ => false,
    }
}
