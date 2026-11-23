use std::{cell::OnceCell, ops::Range};

use crate::Options;
use crate::utils::member_chain::{
    MemberChainLayout, MemberGroups, MemberKind, follows_blank_line, head_has_member_comment,
    head_takes_member_accesses, merges_first_group,
};
use robusta_allocator::Allocator;
use robusta_ast::{AstKind, Comment, Expression};
use robusta_ast_visit::Visit;
use robusta_formatter_core::{IndentWidth, LineWidth, text_width};
use robusta_lexer::{LexResult, Token, TokenKind, tokenize};
use robusta_span::{GetSpan, Span};
use robusta_syntax::line_terminator::split_lines_with_endings;
use rustc_hash::FxHashSet;

/// Why one output line exceeds the configured `line_width`.
///
/// Every cause but [`WideLineCause::Miss`] is an excuse: no break would have
/// helped, or the layout never takes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WideLineCause {
    /// An `import` or `package` declaration, whose name no Java formatter breaks.
    Declaration,
    /// The line fits without its comments.
    Comment,
    /// The widest run of tokens with no break position overflows on its own.
    IndivisibleRun,
    /// A break would fit, but only where the layout deliberately never breaks.
    KeptTogether,
    /// The layout left a break position unused, which may or may not have helped.
    Miss,
}

/// One output line wider than the configured budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WideLine {
    /// One-based line number in the formatted output.
    pub line: u32,
    /// The line's display width, in the same columns the printer counts.
    pub width: u32,
    pub cause: WideLineCause,
}

/// Every output line wider than the configured budget, classified.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LineWidthReport {
    lines: Vec<WideLine>,
}

impl LineWidthReport {
    /// Every over-budget line, in output order.
    #[must_use]
    pub fn wide_lines(&self) -> &[WideLine] {
        &self.lines
    }

    /// Lines the layout could have broken and did not.
    pub fn misses(&self) -> impl Iterator<Item = &WideLine> {
        self.lines.iter().filter(|line| line.cause == WideLineCause::Miss)
    }

    /// How many over-budget lines carry `cause`.
    #[must_use]
    pub fn count(&self, cause: WideLineCause) -> usize {
        self.lines.iter().filter(|line| line.cause == cause).count()
    }

    /// Whether the layout took every break that would have helped.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.misses().next().is_none()
    }

    /// The shared user-facing failure text, naming the first miss.
    #[must_use]
    pub fn failure_message(&self, line_width: LineWidth) -> Option<String> {
        let first = self.misses().next()?;
        let total = self.misses().count();
        let budget = line_width.value();
        Some(format!(
            "{total} line(s) exceed the {budget}-column budget with a break available; \
             first at line {} ({} columns)",
            first.line, first.width
        ))
    }
}

/// Classify every formatted line wider than `line_width`.
///
/// A tab counts `indent_width` columns, as in the printer.
#[must_use]
pub fn check_line_width(formatted: &str, options: &Options) -> LineWidthReport {
    let budget = u32::from(options.line_width.value());
    let indent_width = options.indent_width;
    let audit =
        WidthAudit { source: formatted, lexed: tokenize(formatted), closed: OnceCell::new() };
    let mut lines = Vec::new();
    let mut start = 0usize;

    for (index, raw) in split_lines_with_endings(formatted).enumerate() {
        let content = raw.trim_end_matches(['\n', '\r']);
        let end = start + content.len();
        let width = text_width(content, indent_width);
        if width > budget {
            lines.push(WideLine {
                #[expect(clippy::cast_possible_truncation)]
                line: index as u32 + 1,
                width,
                cause: classify_wide_line(content, start..end, &audit, budget, indent_width),
            });
        }
        start += raw.len();
    }

    LineWidthReport { lines }
}

struct WidthAudit<'s> {
    source: &'s str,
    lexed: LexResult,
    closed: OnceCell<FxHashSet<u32>>,
}

fn classify_wide_line(
    content: &str,
    line: Range<usize>,
    audit: &WidthAudit<'_>,
    budget: u32,
    indent_width: IndentWidth,
) -> WideLineCause {
    let tokens = intersecting(&audit.lexed.tokens, &line, |token| token.span);
    if tokens
        .first()
        .is_some_and(|token| matches!(token.kind, TokenKind::Import | TokenKind::Package))
    {
        return WideLineCause::Declaration;
    }

    let indent = content.len() - content.trim_start().len();
    let indent_columns = text_width(&content[..indent], indent_width);
    let comments = intersecting(&audit.lexed.comments, &line, |comment| comment.span);
    let code = code_without_comments(content, &line, comments);
    let code = code.trim();
    if code.is_empty() || indent_columns + text_width(code, indent_width) <= budget {
        return WideLineCause::Comment;
    }

    let narrowest =
        |policy| narrowest_achievable(content, &line, tokens, indent_width, indent_columns, policy);
    if narrowest(None) > budget {
        return WideLineCause::IndivisibleRun;
    }
    let closed = audit.closed.get_or_init(|| closed_selectors(audit, indent_width));
    if narrowest(Some(closed)) > budget {
        return WideLineCause::KeptTogether;
    }

    WideLineCause::Miss
}

/// The narrowest this line could become however its breaks were placed.
///
/// A run of tokens with no break position stays whole. Every run after the
/// first can reach at best one indent past the line's own.
fn narrowest_achievable(
    content: &str,
    line: &Range<usize>,
    tokens: &[Token],
    indent_width: IndentWidth,
    indent_columns: u32,
    closed: Option<&FxHashSet<u32>>,
) -> u32 {
    let continuation = u32::from(indent_width.value());
    let mut runs = Vec::new();
    let mut run: Option<Range<usize>> = None;
    let mut previous = None;
    for (index, token) in tokens.iter().enumerate() {
        let span = clamp_to_line(token.span, line);
        let opens = breaks_between(previous, token.kind)
            && !closed.is_some_and(|closed| layout_keeps_closed(closed, tokens, index));
        match &mut run {
            Some(open) if !opens => open.end = span.end,
            open => {
                if let Some(open) = open.take() {
                    runs.push(open);
                }
                *open = Some(span);
            }
        }
        previous = Some(token.kind);
    }
    runs.extend(run);

    runs.iter()
        .enumerate()
        .map(|(index, run)| {
            let floor = if index == 0 { indent_columns } else { indent_columns + continuation };
            floor + text_width(&content[run.clone()], indent_width)
        })
        .max()
        .unwrap_or(0)
}

/// Whether the gap before `tokens[index]` is one the layout never breaks at.
///
/// Selectors stay with their operand, and a trailing operator moves no text.
fn layout_keeps_closed(closed: &FxHashSet<u32>, tokens: &[Token], index: usize) -> bool {
    if closed.contains(&tokens[index].span.start)
        || (index > 0 && matches!(tokens[index - 1].kind, TokenKind::Dot | TokenKind::ColonColon))
    {
        return true;
    }
    index + 1 == tokens.len() && is_operator(tokens[index].kind)
}

fn closed_selectors(audit: &WidthAudit<'_>, indent_width: IndentWidth) -> FxHashSet<u32> {
    let allocator = Allocator::new();
    let parsed = robusta_parser::parse(&allocator, audit.source);
    let mut selectors = ClosedSelectors {
        source: audit.source,
        tokens: &audit.lexed.tokens,
        comments: &audit.lexed.comments,
        indent_width,
        seen: FxHashSet::default(),
        closed: FxHashSet::default(),
    };
    if parsed.errors.is_empty() {
        selectors.visit_compilation_unit(&parsed.unit);
    }
    selectors.closed
}

struct ClosedSelectors<'t> {
    source: &'t str,
    tokens: &'t [Token],
    comments: &'t [Comment],
    indent_width: IndentWidth,
    seen: FxHashSet<Span>,
    closed: FxHashSet<u32>,
}

impl ClosedSelectors<'_> {
    fn close_after(&mut self, target_end: u32) {
        let dot = self.tokens.partition_point(|token| token.span.end <= target_end);
        if let Some(token) = self.tokens.get(dot).filter(|token| token.kind == TokenKind::Dot) {
            self.closed.insert(token.span.start);
        }
    }

    fn collect_chain(&mut self, mut member: AstKind<'_>, starts_statement: bool) {
        // The member accesses after the last call print around its chain.
        loop {
            let object = match member {
                AstKind::FieldAccess(access) => &access.target,
                AstKind::ArrayAccess(access) => &access.array,
                _ => break,
            };
            self.seen.insert(member.span());
            member = match object {
                Expression::FieldAccess(access) => AstKind::FieldAccess(access),
                Expression::ArrayAccess(access) => AstKind::ArrayAccess(access),
                Expression::MethodCall(call) if call.member().is_some() => {
                    AstKind::MethodCall(call)
                }
                _ => return,
            };
        }
        // The members from the root: what each is, its target, and where it
        // starts after its `.`, or at its target's end for an array access.
        let mut members = Vec::new();
        // The members before the first call, which join a head that is not a call.
        let mut head_members = 0;
        let head = loop {
            let target = match member {
                AstKind::FieldAccess(access) => {
                    head_members += 1;
                    members.push((MemberKind::Field, &access.target, access.name.span.start));
                    &access.target
                }
                AstKind::MethodCall(call) => {
                    let Some(callee) = call.member() else { return };
                    head_members = 0;
                    let start = callee
                        .type_args
                        .as_ref()
                        .map_or(callee.name.span.start, |args| args.span.start);
                    members.push((MemberKind::Call, &callee.target, start));
                    &callee.target
                }
                AstKind::ArrayAccess(access) => {
                    head_members += 1;
                    let literal = matches!(access.index, Expression::IntLiteral(_));
                    let kind = MemberKind::Index { literal };
                    members.push((kind, &access.array, access.array.span().end));
                    &access.array
                }
                _ => return,
            };
            self.seen.insert(member.span());
            member = match target {
                Expression::FieldAccess(access) => AstKind::FieldAccess(access),
                Expression::MethodCall(call) if call.member().is_some() => {
                    AstKind::MethodCall(call)
                }
                Expression::ArrayAccess(access) => AstKind::ArrayAccess(access),
                _ => break target,
            };
        };
        if head_takes_member_accesses(head) {
            members.truncate(members.len() - head_members);
        }
        let Some(&(_, head, start)) = members.last() else { return };
        let merges = merges_first_group(
            head,
            self.has_comment_between(head.span().end, start),
            starts_statement,
            self.indent_width,
        );
        let mut separated =
            head_has_member_comment(head, |start, end| self.has_comment_between(start, end));
        let mut groups = MemberGroups::default();
        let starts: Vec<_> = members
            .iter()
            .rev()
            .map(|&(kind, target, start)| {
                let end = target.span().end;
                let comment = self.has_comment_between(end, start);
                let group = groups.push(kind, comment);
                separated |= comment
                    || (group.is_some_and(|group| group > 0 || !merges)
                        && !matches!(kind, MemberKind::Index { .. })
                        && follows_blank_line(target, &self.source[end as usize..start as usize]));
                group
            })
            .collect();
        let layout = MemberChainLayout::new(head, groups.count(), merges);
        let breaks = layout.can_break(separated);
        for (&(_, target, _), group) in members.iter().rev().zip(starts) {
            if !(breaks && group.is_some_and(|group| layout.starts_line(group))) {
                self.close_after(target.span().end);
            }
        }
    }

    fn has_comment_between(&self, start: u32, end: u32) -> bool {
        let first = self.comments.partition_point(|comment| comment.span.start < start);
        self.comments.get(first).is_some_and(|comment| comment.span.end <= end)
    }
}

impl<'a> Visit<'a> for ClosedSelectors<'_> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        match kind {
            AstKind::QualifiedThis(node) => self.close_after(node.qualifier.span().end),
            AstKind::QualifiedSuper(node) => self.close_after(node.qualifier.span().end),
            AstKind::ClassLiteral(node) => self.close_after(node.ty.span().end),
            AstKind::QualifiedClassInstanceCreation(node) => {
                self.close_after(node.qualifier.span().end);
            }
            AstKind::ConstructorInvocation(node) => {
                if let Some(qualifier) = &node.qualifier {
                    self.close_after(qualifier.span().end);
                }
            }
            AstKind::ExpressionStatement(statement) => {
                if let Expression::MethodCall(call) = &statement.expression {
                    self.collect_chain(AstKind::MethodCall(call), true);
                }
            }
            AstKind::FieldAccess(_) | AstKind::MethodCall(_) | AstKind::ArrayAccess(_)
                if !self.seen.contains(&kind.span()) =>
            {
                self.collect_chain(kind, false);
            }
            _ => {}
        }
    }
}

/// Whether a Java layout may put a line break between `previous` and `next`.
///
/// Never between a name and its `(`, so a method name that fills the budget is
/// not counted as a miss.
fn breaks_between(previous: Option<TokenKind>, next: TokenKind) -> bool {
    let Some(previous) = previous else {
        return false;
    };
    if matches!(
        (previous, next),
        (TokenKind::LParen, TokenKind::RParen)
            | (TokenKind::LBracket, TokenKind::RBracket)
            | (TokenKind::LBrace, TokenKind::RBrace)
    ) {
        // An empty pair holds nothing to move below the line.
        return false;
    }
    ends_a_line(previous) || starts_a_line(next)
}

/// Token kinds a wrapped line may end on: an open bracket, `,`, or an operator.
fn ends_a_line(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace | TokenKind::Comma)
        || is_operator(kind)
}

/// Token kinds a wrapped line may begin with: a closing bracket, an operator, or
/// a clause keyword.
fn starts_a_line(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::RParen
            | TokenKind::RBracket
            | TokenKind::RBrace
            | TokenKind::Extends
            | TokenKind::Implements
            | TokenKind::Permits
            | TokenKind::Throws
    ) || is_operator(kind)
}

fn is_operator(kind: TokenKind) -> bool {
    kind.is_binary_operator()
        || kind.is_assignment_operator()
        || matches!(
            kind,
            TokenKind::Dot
                | TokenKind::ColonColon
                | TokenKind::Question
                | TokenKind::Colon
                | TokenKind::Arrow
        )
}

/// The slice of sorted `items` whose spans intersect `line`.
///
/// Intersect, not contain: a block comment or text block can span lines.
fn intersecting<'items, T>(
    items: &'items [T],
    line: &Range<usize>,
    span: impl Fn(&T) -> Span,
) -> &'items [T] {
    let first = items.partition_point(|item| (span(item).end as usize) <= line.start);
    let rest = &items[first..];
    let len = rest.partition_point(|item| (span(item).start as usize) < line.end);
    &rest[..len]
}

/// `content` without its comments, interior ones included: where a comment
/// sits is another check's concern.
fn code_without_comments(content: &str, line: &Range<usize>, comments: &[Comment]) -> String {
    let mut code = String::with_capacity(content.len());
    let mut cut = 0;
    for comment in comments {
        let span = clamp_to_line(comment.span, line);
        code.push_str(&content[cut..span.start]);
        cut = span.end;
    }
    code.push_str(&content[cut..]);
    code
}

/// `span` as a byte range into the line's own content.
fn clamp_to_line(span: Span, line: &Range<usize>) -> Range<usize> {
    let start = (span.start as usize).clamp(line.start, line.end) - line.start;
    let end = (span.end as usize).clamp(line.start, line.end) - line.start;
    start..end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn narrow_options() -> Options {
        Options {
            line_width: LineWidth::try_from(20).expect("test budget is valid"),
            indent_width: IndentWidth::try_from(4).expect("test indent is valid"),
            ..Options::default()
        }
    }

    fn closed(source: &str) -> (Vec<Token>, FxHashSet<u32>) {
        let audit = WidthAudit { source, lexed: tokenize(source), closed: OnceCell::new() };
        let closed = closed_selectors(&audit, narrow_options().indent_width);
        (audit.lexed.tokens, closed)
    }

    fn causes(source: &str) -> Vec<WideLineCause> {
        check_line_width(source, &narrow_options())
            .wide_lines()
            .iter()
            .map(|line| line.cause)
            .collect()
    }

    #[test]
    fn a_line_within_the_budget_is_not_reported() {
        assert_eq!(causes("class A {\n    int x = 1;\n}\n").len(), 0);
    }

    #[test]
    fn a_long_import_is_excused_as_a_declaration() {
        assert_eq!(
            causes("import a.b.c.d.e.F;\nimport a.b.c.d.e.f.G;\n"),
            [WideLineCause::Declaration]
        );
    }

    #[test]
    fn a_trailing_comment_that_overflows_a_fitting_line_is_excused() {
        assert_eq!(
            causes("class A {\n    int x; // a wide trailing note\n}\n"),
            [WideLineCause::Comment]
        );
    }

    #[test]
    fn a_leading_block_comment_does_not_pad_the_remaining_code() {
        assert_eq!(
            causes("class A {\n    /* note */ int memberNames;\n}\n"),
            [WideLineCause::Comment]
        );
    }

    #[test]
    fn a_line_whose_widest_run_alone_overflows_is_excused() {
        assert_eq!(
            causes("class A {\n    void aVeryLongMethodName() {}\n}\n"),
            [WideLineCause::IndivisibleRun]
        );
    }

    #[test]
    fn a_one_segment_chain_is_kept_together() {
        // Each half fits, but a one-call chain prints inline at any width.
        assert_eq!(
            causes("class A {\nvoid m() {\n    aaaaaaaaaaaa.bbbb(c);\n}\n}\n"),
            [WideLineCause::KeptTogether]
        );
    }

    #[test]
    fn selector_breaks_follow_ast_chains() {
        for (expression, breaks) in [
            ("receiver.call()", vec![false]),
            ("receiver.<T>call()", vec![false]),
            ("a.b.call()", vec![true, false]),
            ("new C().call()", vec![false]),
            ("a.b().c()", vec![true, true]),
            ("Stream.of(a).b()", vec![false, false]),
            ("Stream.of(a).b().c()", vec![false, true, true]),
            ("this.a(b).c(d).e()", vec![false, true, true]),
            ("a().x.y.b()", vec![false, false, false]),
            ("a().x.b().c()", vec![true, false, true]),
            ("a.b(c).d()", vec![true, true]),
            ("\"text\".call(c)", vec![true]),
            ("\"text\".call()", vec![true]),
            ("a.b().field", vec![false, true]),
            ("a.field.b()", vec![true, false]),
            ("this.a.b(c)", vec![true, false]),
            ("a[0].b.c(d)", vec![true, false]),
            ("a[0].call()", vec![false]),
            ("a.b()[0].c()", vec![true, true]),
            ("a.b()[i].c()", vec![true, false]),
            ("Stream.of(a) /* c */ .b()", vec![false, true]),
            ("Stream.of(a)\n\n.b()", vec![false, true]),
            ("Stream.of(a)\n.\nb()", vec![false, false]),
            ("this.a.b.c(d)", vec![true, true, false]),
            ("this.a /* c */ .b.c(d)", vec![true, true, true]),
            ("f(a.b(), c.d())", vec![false, false]),
            ("new C(a.b()).c()", vec![false, false]),
            ("(a.b()).c()", vec![false, false]),
            ("a.b.C.class", vec![true, true, false]),
            ("a.b::method", vec![true, true]),
            ("a.b(c.d()).e(f.g())", vec![true, false, true, false]),
        ] {
            let source = format!("class A {{\nvoid m() {{\n    Object x = {expression};\n}}\n}}\n");
            crate::format(&source, &narrow_options()).expect("valid source");
            let (tokens, closed) = closed(&source);
            let actual: Vec<_> = tokens
                .iter()
                .enumerate()
                .filter(|(_, token)| matches!(token.kind, TokenKind::Dot | TokenKind::ColonColon))
                .map(|(index, _)| !layout_keeps_closed(&closed, &tokens, index))
                .collect();
            assert_eq!(actual, breaks, "{expression}");
        }
    }

    #[test]
    fn explicit_constructor_type_arguments_keep_the_keyword_dot_closed() {
        let source = "class A {\nA() {\n    outer.<T>super();\n}\n}\n";
        crate::format(source, &narrow_options()).expect("valid source");
        let (tokens, closed) = closed(source);
        let dot = tokens.iter().find(|token| token.kind == TokenKind::Dot).expect("qualifier dot");
        assert!(closed.contains(&dot.span.start));
    }

    #[test]
    fn constructor_receivers_and_merged_first_calls_keep_their_selectors() {
        for source in [
            "class A {\nvoid m() {\n    new C().bbbbbbbb();\n}\n}\n",
            "class A {\nvoid m() {\n    Aaaaaaaaaa.bbbb()\n        .c()\n        .d();\n}\n}\n",
        ] {
            assert_eq!(causes(source), [WideLineCause::KeptTogether], "{source}");
        }
    }

    #[test]
    fn a_qualified_name_with_an_unused_break_is_a_miss() {
        assert_eq!(causes("class A {\n    aaa.bbb.ccc.ddd eee;\n}\n"), [WideLineCause::Miss]);
    }

    #[test]
    fn a_qualified_constructor_type_with_an_unused_break_is_a_miss() {
        assert_eq!(causes("class A {\n    x = new aaaa.bbbb.Cccc();\n}\n"), [WideLineCause::Miss]);
    }

    #[test]
    fn qualified_names_and_fields_report_unused_dot_breaks() {
        for source in [
            "class A {\n    aaaaaaaa.Bbbb value;\n}\n",
            "class A {\n    Object x = new aaaa.Bbbb();\n}\n",
            "class A {\n    Object x = aaaaaaaa.bbbb;\n}\n",
            "class A {\n    Object x = this.bbbb;\n}\n",
            "class A {\n    Object x = f().bbbb;\n}\n",
            "class A {\n    aaaa.Bbbb.Cccc value;\n}\n",
        ] {
            assert_eq!(causes(source), [WideLineCause::Miss], "{source}");
            let formatted = crate::format(source, &narrow_options()).expect("valid source");
            assert!(causes(&formatted).is_empty(), "{formatted}");
        }
    }

    #[test]
    fn keyword_dots_and_wide_tokens_keep_their_exemptions() {
        for source in [
            "class A {\n    Object x = aaaaaaaaaaa.class;\n}\n",
            "class A {\n    Object x = aaaaaaaaaaaa.this;\n}\n",
            "class A {\n    Object x = aaaaaaaaaaa.super.b;\n}\n",
            "class A {\n    Object x = aaaaaaaaaaaa.new C();\n}\n",
        ] {
            assert_eq!(causes(source), [WideLineCause::KeptTogether], "{source}");
        }
        assert_eq!(
            causes("package aaaaaaaa.bbbbbbbb;\nimport aaaaaaaa.bbbbbbbb.C;\n"),
            [WideLineCause::Declaration; 2]
        );
        assert_eq!(causes("    aaaaaaaaaaaaaaaaaaaaa;\n"), [WideLineCause::IndivisibleRun]);
    }

    #[test]
    fn a_two_segment_chain_is_still_a_miss() {
        assert_eq!(causes("class A {\n    aaaaaaaa.bbbb(c).dddd();\n}\n"), [WideLineCause::Miss]);
    }

    #[test]
    fn an_operator_that_ends_the_line_is_kept_together() {
        // Breaking before the trailing `=` would leave it alone below.
        assert_eq!(causes("class A {\n    int aaaaaaaaaaaa =\n}\n"), [WideLineCause::KeptTogether]);
    }

    #[test]
    fn a_line_with_a_break_position_inside_the_wide_part_is_a_miss() {
        assert_eq!(causes("class A {\n    int x = aaa + bbb + ccc;\n}\n"), [WideLineCause::Miss]);
    }

    #[test]
    fn an_empty_parameter_list_is_not_a_break_position() {
        // Counting the gap in `()` would make every long method name a miss.
        assert!(!breaks_between(Some(TokenKind::LParen), TokenKind::RParen));
        assert!(breaks_between(Some(TokenKind::LParen), TokenKind::Identifier));
    }

    #[test]
    fn a_name_and_the_paren_after_it_are_one_run() {
        assert!(!breaks_between(Some(TokenKind::Identifier), TokenKind::LParen));
        assert!(breaks_between(Some(TokenKind::Identifier), TokenKind::Lt));
    }

    #[test]
    fn the_failure_message_counts_misses_and_names_the_first() {
        let report =
            check_line_width("class A {\n    int x = aaa + bbb + ccc;\n}\n", &narrow_options());

        assert!(!report.is_clean());
        assert_eq!(
            report.failure_message(narrow_options().line_width),
            Some(
                "1 line(s) exceed the 20-column budget with a break available; first at line 2 (28 columns)"
                    .to_string()
            )
        );
    }
}
