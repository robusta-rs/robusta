// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/crates/oxc_formatter/src/formatter/context.rs

use robusta_allocator::Allocator;
use robusta_ast::CompilationUnit;
use robusta_formatter_core::{FormatContext, FormatElement, GroupId, SourceText};
use robusta_span::{GetSpan, Span};
use rustc_hash::FxHashMap;

use crate::Options;
use crate::formatter::Comments;
use crate::formatter::javadoc::JavadocDeclarations;
use crate::utils::assignment_like::AssignmentLikeLayout;

pub struct JavaFormatContext<'a> {
    /// The file being formatted, as mechanical offset-keyed access. It sits
    /// here rather than under the comment cursor because comment state and
    /// source access are siblings: a printer that needs a byte range is not
    /// asking the cursor anything.
    pub source_text: SourceText<'a>,
    pub comments: Comments<'a, 'a>,
    pub options: Options,
    pub(crate) is_snippet: bool,
    /// Content formatted once and written again by a later layout attempt,
    /// whose comments can only be printed once.
    pub cached_elements: FxHashMap<Span, FormatElement<'a>>,
    /// One-shot handoff of the assignment layout to the lambda on the right side of an assignment-like,
    /// keyed by the lambda's span so no other node can consume it.
    /// Set by the assignment-like before formatting the lambda, taken by the lambda's `write`.
    lambda_assignment_layout: Option<(Span, AssignmentLikeLayout)>,
    /// While a record's clause formats, the group of the record's components.
    /// The clause's type argument lists wrap differently once it breaks, and
    /// they sit behind generated node passes that take no arguments.
    record_components_group: Option<GroupId>,
    /// End of the body followed by `else` or a `do`'s `while`.
    pub(crate) continuation_body_end: Option<u32>,
}

impl<'a> JavaFormatContext<'a> {
    pub(crate) fn new(
        allocator: &'a Allocator,
        source: &'a str,
        unit: &'a CompilationUnit<'a>,
        options: &Options,
    ) -> Self {
        let source_text = SourceText::new(source);
        let mut comments = Comments::new(source_text, &unit.comments);
        if options.javadoc.is_some() {
            comments = comments
                .with_declarations(JavadocDeclarations::collect(unit))
                .with_markdown_javadoc_runs(allocator);
        }
        Self {
            source_text,
            comments,
            options: options.clone(),
            is_snippet: false,
            cached_elements: FxHashMap::default(),
            lambda_assignment_layout: None,
            record_components_group: None,
            continuation_body_end: None,
        }
    }

    pub fn comments(&self) -> &Comments<'a, 'a> {
        &self.comments
    }

    pub fn source_text(&self) -> SourceText<'a> {
        self.source_text
    }

    /// Returns the cached formatted element for the given key.
    pub(crate) fn get_cached_element<T: GetSpan>(&self, key: &T) -> Option<FormatElement<'a>> {
        self.cached_elements.get(&key.span()).cloned()
    }

    /// Caches the formatted element for the given key.
    pub(crate) fn cache_element<T: GetSpan>(&mut self, key: &T, formatted: FormatElement<'a>) {
        self.cached_elements.insert(key.span(), formatted);
    }

    /// See the [`Self::lambda_assignment_layout`] field.
    pub(crate) fn set_lambda_assignment_layout(
        &mut self,
        span: Span,
        layout: AssignmentLikeLayout,
    ) {
        debug_assert!(
            self.lambda_assignment_layout.is_none(),
            "a previous lambda assignment layout was not taken"
        );
        self.lambda_assignment_layout = Some((span, layout));
    }

    /// See the [`Self::lambda_assignment_layout`] field.
    pub(crate) fn take_lambda_assignment_layout(
        &mut self,
        span: Span,
    ) -> Option<AssignmentLikeLayout> {
        match self.lambda_assignment_layout {
            Some((key, layout)) if key == span => {
                self.lambda_assignment_layout = None;
                Some(layout)
            }
            _ => None,
        }
    }

    /// See the [`Self::record_components_group`] field.
    pub(crate) fn record_components_group(&self) -> Option<GroupId> {
        self.record_components_group
    }

    /// See the [`Self::record_components_group`] field.
    pub(crate) fn set_record_components_group(&mut self, group: Option<GroupId>) {
        self.record_components_group = group;
    }
}

impl FormatContext for JavaFormatContext<'_> {
    type Options = Options;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_code(&self) -> &str {
        &self.source_text
    }
}
