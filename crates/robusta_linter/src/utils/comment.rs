use robusta_ast::Comment;
use robusta_span::Span;

/// Whether a documentation comment attaches to the token that starts `span`,
/// as javac gives it to that token.
pub fn follows_documentation(comments: &[Comment], span: Span) -> bool {
    let before = &comments[..comments.partition_point(|comment| comment.span.start < span.start)];
    before
        .iter()
        .rev()
        .take_while(|comment| comment.attached_to == span.start)
        .any(Comment::is_documentation)
}
