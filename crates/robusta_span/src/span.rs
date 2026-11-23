use robusta_ast_macros::ast;

/// A half-open byte range `[start, end)` into the source text.
#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[content_eq(skip)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[inline]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// An empty span sitting at `offset`.
    #[inline]
    pub const fn empty(offset: u32) -> Self {
        Self { start: offset, end: offset }
    }

    /// A span of `len` bytes starting at `start`.
    #[inline]
    pub const fn sized(start: u32, len: u32) -> Self {
        Self { start, end: start + len }
    }

    #[inline]
    pub const fn len(self) -> u32 {
        self.end - self.start
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// The slice of `source` this span covers.
    #[inline]
    pub fn source_text(self, source: &str) -> &str {
        &source[self.start as usize..self.end as usize]
    }

    /// Whether `span` lies entirely within `self`. A span always contains
    /// itself, and an empty span sitting on either boundary is contained.
    #[inline]
    pub const fn contains_inclusive(self, span: Span) -> bool {
        self.start <= span.start && span.end <= self.end
    }

    /// The smallest span covering both `self` and `other`.
    #[inline]
    #[must_use]
    pub fn cover(self, other: Span) -> Span {
        Span::new(self.start.min(other.start), self.end.max(other.end))
    }
}

/// Access to an AST node's source span.
pub trait GetSpan {
    fn span(&self) -> Span;
}

impl GetSpan for Span {
    #[inline]
    fn span(&self) -> Span {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_text_and_cover() {
        let src = "package a.b.c;";
        let a = Span::new(0, 7);
        let b = Span::new(8, 13);
        assert_eq!(a.source_text(src), "package");
        assert_eq!(b.source_text(src), "a.b.c");
        assert_eq!(b.len(), 5);
        assert_eq!(a.cover(b), Span::new(0, 13));
    }
}
