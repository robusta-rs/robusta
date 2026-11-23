use robusta_span::Span;

use crate::ast::Comment;

/// Whether a comment starts in the half-open `span`.
///
/// `comments` must be sorted by start position. Comments starting before
/// `span` do not count, even if they overlap it.
pub fn has_comments_between(comments: &[Comment], span: Span) -> bool {
    let first = comments.partition_point(|comment| comment.span.start < span.start);
    comments.get(first).is_some_and(|comment| comment.span.start < span.end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::CommentKind;

    fn comment(start: u32, end: u32) -> Comment {
        Comment::new(start, end, CommentKind::SingleLineBlock)
    }

    #[test]
    fn finds_only_comments_starting_inside_the_span() {
        let comments = [comment(0, 4), comment(10, 14), comment(20, 24)];

        assert!(has_comments_between(&comments, Span::new(10, 11)));
        assert!(has_comments_between(&comments, Span::new(5, 15)));
        assert!(!has_comments_between(&comments, Span::new(4, 10)));
        assert!(!has_comments_between(&comments, Span::new(11, 20)));
        assert!(!has_comments_between(&comments, Span::new(24, 30)));
        assert!(!has_comments_between(&comments, Span::new(12, 12)));
        assert!(!has_comments_between(&[], Span::new(0, 100)));
    }
}
