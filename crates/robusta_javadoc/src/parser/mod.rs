mod javadoc;
pub mod javadoc_tag;
mod parse;
mod utils;

pub use javadoc::{Javadoc, JavadocKind};
pub use javadoc_tag::{Tag, TagForm};
pub use utils::markdown_margin;

#[cfg(test)]
pub mod tests {
    use robusta_ast::{Comment, CommentContent, CommentKind};
    use robusta_span::Span;

    use super::Javadoc;

    /// A documentation comment spanning all of `source`.
    pub fn comment(source: &str, kind: CommentKind) -> Comment {
        let end = u32::try_from(source.len()).expect("test source fits in u32");
        Comment { content: CommentContent::Documentation, ..Comment::new(0, end, kind) }
    }

    /// A `///` documentation line at `span`.
    pub fn line(span: Span) -> Comment {
        Comment {
            content: CommentContent::Documentation,
            ..Comment::new(span.start, span.end, CommentKind::Line)
        }
    }

    pub(super) fn parse(source: &str) -> Javadoc<'_> {
        Javadoc::from_comment(source, &comment(source, CommentKind::MultiLineBlock))
            .expect("test source is Javadoc")
    }
}
