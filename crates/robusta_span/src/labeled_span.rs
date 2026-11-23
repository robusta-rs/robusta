// Adapted from https://github.com/oxc-project/oxc/blob/82c924496c2e1c79e05b29e40377f6b703a2cd6a/crates/oxc_span/src/labeled_span.rs

use crate::Span;

/// A labeled source [`Span`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabeledSpan {
    label: Option<String>,
    span: Span,
    primary: bool,
}

impl LabeledSpan {
    /// Creates a labeled span.
    #[must_use]
    pub const fn new(label: Option<String>, offset: u32, len: u32) -> Self {
        Self { label, span: Span::sized(offset, len), primary: false }
    }

    /// Creates a labeled span from an existing span.
    #[must_use]
    pub const fn new_with_span(label: Option<String>, span: Span) -> Self {
        Self { label, span, primary: false }
    }

    /// Creates a primary labeled span from an existing span.
    #[must_use]
    pub const fn new_primary_with_span(label: Option<String>, span: Span) -> Self {
        Self { label, span, primary: true }
    }

    /// Creates a label for the specified span.
    #[must_use]
    pub fn at(span: Span, label: impl Into<String>) -> Self {
        Self::new_with_span(Some(label.into()), span)
    }

    /// Creates an unlabeled underline for the specified span.
    #[must_use]
    pub const fn underline(span: Span) -> Self {
        Self::new_with_span(None, span)
    }

    /// Returns the label text, if present.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the source [`Span`].
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Replaces the source [`Span`], keeping the label and primary state.
    pub const fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    /// Returns the 0-based starting byte offset.
    #[must_use]
    pub const fn offset(&self) -> u32 {
        self.span.start
    }

    /// Returns the number of bytes this `LabeledSpan` covers.
    #[must_use]
    pub const fn len(&self) -> u32 {
        self.span.len()
    }

    /// True if this `LabeledSpan` is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// True if this `LabeledSpan` is a primary span.
    #[must_use]
    pub const fn primary(&self) -> bool {
        self.primary
    }
}

#[cfg(test)]
mod tests {
    use super::LabeledSpan;
    use crate::Span;

    #[test]
    fn labeled_spans_preserve_label_and_primary_state() {
        let label = LabeledSpan::new_primary_with_span(Some("here".to_string()), Span::new(3, 7));
        assert_eq!(label.label(), Some("here"));
        assert_eq!(label.offset(), 3);
        assert_eq!(label.len(), 4);
        assert!(label.primary());
        assert_eq!(label.span(), Span::new(3, 7));
    }

    #[test]
    fn moving_a_label_preserves_its_label_text() {
        let mut label = LabeledSpan::at(Span::new(3, 7), "here");
        label.set_span(Span::new(10, 14));
        assert_eq!(label.span(), Span::new(10, 14));
        assert_eq!(label.label(), Some("here"));
        assert!(!label.primary());
        assert!(!label.is_empty());
    }
}
