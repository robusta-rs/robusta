use std::ops::{Bound, RangeBounds};

use robusta_span::Span;

use crate::ast::Comment;

/// The comments that start within `range`, in source order.
///
/// `comments` must be sorted by start position. Comments starting before
/// `range` do not count, even if they overlap it.
pub fn comments_range<R: RangeBounds<u32>>(comments: &[Comment], range: R) -> &[Comment] {
    let first = comments.partition_point(|comment| match range.start_bound() {
        Bound::Included(&start) => comment.span.start < start,
        Bound::Excluded(&start) => comment.span.start <= start,
        Bound::Unbounded => false,
    });
    let rest = &comments[first..];
    &rest[..rest.partition_point(|comment| range.contains(&comment.span.start))]
}

/// Whether a comment starts in the half-open `span`.
///
/// `comments` must be sorted by start position. Comments starting before
/// `span` do not count, even if they overlap it.
pub fn has_comments_between(comments: &[Comment], span: Span) -> bool {
    !comments_range(comments, span.start..span.end).is_empty()
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

    #[test]
    fn ranges_select_comments_by_start() {
        let comments = [comment(0, 4), comment(10, 14), comment(20, 24)];

        assert_eq!(comments_range(&comments, ..), &comments);
        assert_eq!(comments_range(&comments, 1..), &comments[1..]);
        assert_eq!(comments_range(&comments, 10..20), &comments[1..2]);
        assert_eq!(comments_range(&comments, 10..=20), &comments[1..]);
        assert_eq!(comments_range(&comments, ..20), &comments[..2]);
        assert_eq!(comments_range(&comments, 11..20), &[]);
        assert_eq!(comments_range(&[], ..), &[]);
    }
}
