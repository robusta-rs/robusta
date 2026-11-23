// Adapted from https://github.com/oxc-project/oxc/blob/566b11b2db6b664e787e67e2199d4ffcaa31c30a/crates/oxc_formatter/src/formatter/formatter_js.rs

use robusta_formatter_core::builders::{Line, hard_line_break, soft_line_break_or_space};
use robusta_formatter_core::{Formatter, SourceText};
use robusta_span::Span;

use super::{Comments, JavaFormatContext, builders::JoinNodesBuilder};
use crate::source_text::SourceTextExt;

/// The Java-specialized [`Formatter`].
pub type JavaFormatter<'buf, 'ast> = Formatter<'buf, 'ast, JavaFormatContext<'ast>>;

/// Java-specific methods for the formatter defined in `robusta_formatter_core`.
pub trait JavaFormatterExt<'buf, 'ast> {
    fn source_text(&self) -> SourceText<'ast>;
    fn comments(&self) -> &Comments<'ast, 'ast>;
    fn lines_before(&self, span: Span) -> usize;
    fn join_nodes_with_soft_line<'fmt>(&'fmt mut self) -> JoinNodesBuilder<'fmt, 'buf, 'ast, Line>;
    fn join_nodes_with_hardline<'fmt>(&'fmt mut self) -> JoinNodesBuilder<'fmt, 'buf, 'ast, Line>;
}

impl<'buf, 'ast> JavaFormatterExt<'buf, 'ast> for JavaFormatter<'buf, 'ast> {
    /// Returns the source text from the context.
    #[inline]
    fn source_text(&self) -> SourceText<'ast> {
        self.context().source_text()
    }

    #[inline]
    fn comments(&self) -> &Comments<'ast, 'ast> {
        self.context().comments()
    }

    /// Returns the number of line breaks before `span`, counting the leading trivia
    /// of the first not-yet-printed comment. See [`SourceTextExt::get_lines_before`].
    #[inline]
    fn lines_before(&self, span: Span) -> usize {
        self.source_text().get_lines_before(span, self.comments().first_unprinted_span())
    }

    fn join_nodes_with_soft_line<'fmt>(&'fmt mut self) -> JoinNodesBuilder<'fmt, 'buf, 'ast, Line> {
        JoinNodesBuilder::new(soft_line_break_or_space(), self)
    }

    fn join_nodes_with_hardline<'fmt>(&'fmt mut self) -> JoinNodesBuilder<'fmt, 'buf, 'ast, Line> {
        JoinNodesBuilder::new(hard_line_break(), self)
    }
}
