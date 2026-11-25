use robusta_allocator::Allocator;
use robusta_ast::Comment;
use robusta_javadoc::Javadoc;

use crate::formatter::javadoc::JavadocDeclarations;

/// A documentation comment before a declaration, which it documents unless a
/// later comment supersedes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Documented {
    /// The declaration's place among the file's.
    declaration: usize,
    superseded: bool,
}

/// Where the declarations a documentation comment can document start. Empty when
/// there is no such comment, which spares the parse.
pub(super) fn declaration_starts(source: &str, comments: &[Comment]) -> Vec<u32> {
    if !comments.iter().any(Comment::is_documentation) {
        return Vec::new();
    }
    let allocator = Allocator::new();
    let parsed = robusta_parser::parse(&allocator, source);
    JavadocDeclarations::collect(&parsed.unit).starts().to_vec()
}

/// Whether `comment` is a documentation comment before a declaration, the only
/// place Javadoc formatting rewrites one.
pub(super) fn is_before_declaration(comment: &Comment, starts: &[u32]) -> bool {
    comment.is_documentation() && starts.binary_search(&comment.attached_to).is_ok()
}

/// The documentation comments before a declaration, a `///` run counting once,
/// in source order.
pub(super) fn documented_declarations(
    source: &str,
    comments: &[Comment],
    starts: &[u32],
) -> Vec<Documented> {
    let mut documented = Vec::new();
    let mut index = 0;
    while index < comments.len() {
        let Some((document, consumed)) = Javadoc::from_comments(source, &comments[index..]) else {
            index += 1;
            continue;
        };
        if let Ok(declaration) = starts.binary_search(&comments[index].attached_to) {
            let superseded = document.documented_offset(comments).is_none();
            documented.push(Documented { declaration, superseded });
        }
        index += consumed;
    }
    documented
}
