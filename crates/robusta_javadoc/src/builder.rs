use std::cell::OnceCell;

use robusta_ast::Comment;

use crate::{Javadoc, Structure};

/// Source-order iterator over every traditional or grouped Markdown Javadoc
/// document in a comment side-table.
pub struct Javadocs<'a, 'comments> {
    source: &'a str,
    comments: &'comments [Comment],
    next: usize,
}

impl<'a, 'comments> Javadocs<'a, 'comments> {
    /// Iterate over all documentation comments in `comments`.
    pub fn new(source: &'a str, comments: &'comments [Comment]) -> Self {
        Self { source, comments, next: 0 }
    }
}

impl<'a> Iterator for Javadocs<'a, '_> {
    type Item = Javadoc<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next < self.comments.len() {
            let comments = &self.comments[self.next..];
            if let Some((document, consumed)) = Javadoc::from_comments(self.source, comments) {
                self.next += consumed;
                return Some(document);
            }
            self.next += 1;
        }
        None
    }
}

/// File-local Javadoc documents with lazily cached structure.
///
/// Recognition happens once when the finder is built. Each document's prose,
/// tags, and literal regions are then parsed at most once, even when several
/// consumers inspect the same file.
#[derive(Debug)]
pub struct JavadocFinder<'a> {
    documents: Vec<CachedJavadoc<'a>>,
}

impl<'a> JavadocFinder<'a> {
    /// Recognize every documentation comment in a file.
    pub fn new(source: &'a str, comments: &[Comment]) -> Self {
        Self { documents: Javadocs::new(source, comments).map(CachedJavadoc::new).collect() }
    }

    /// Iterate over each document's [translated
    /// structure](Javadoc::translated_structure) in source order.
    ///
    /// A structure is created on first access and reused thereafter.
    pub fn structures(&self) -> impl Iterator<Item = &Structure> {
        self.documents.iter().map(CachedJavadoc::structure)
    }
}

#[derive(Debug)]
struct CachedJavadoc<'a> {
    document: Javadoc<'a>,
    structure: OnceCell<Structure>,
}

impl<'a> CachedJavadoc<'a> {
    fn new(document: Javadoc<'a>) -> Self {
        Self { document, structure: OnceCell::new() }
    }

    fn structure(&self) -> &Structure {
        self.structure.get_or_init(|| self.document.translated_structure())
    }
}

#[cfg(test)]
mod tests {
    use robusta_ast::{Comment, CommentContent, CommentKind};
    use robusta_span::Span;

    use crate::parser::tests::line;

    use super::*;

    #[test]
    fn markdown_runs_stop_at_blank_lines_and_ordinary_comments() {
        let source = "/// one\n\n/// two\n// ordinary\n/// three";
        let starts = [0usize, 9, 17, 29];
        let comments = [
            line(Span::new(0, 7)),
            line(Span::new(9, 16)),
            Comment { content: CommentContent::None, ..line(Span::new(17, 28)) },
            line(Span::new(29, u32::try_from(source.len()).expect("source fits"))),
        ];
        assert_eq!(starts, comments.map(|comment| comment.span.start as usize));

        let documents = Javadocs::new(source, &comments).collect::<Vec<_>>();
        assert_eq!(documents.len(), 3);
        assert_eq!(
            documents.iter().map(|document| document.lines()).collect::<Vec<_>>(),
            [vec!["one"], vec!["two"], vec!["three"],]
        );
    }

    #[test]
    fn markdown_structure_keeps_reference_names_visible() {
        let source = "/// A demo referencing [List].\n/// @see Map.Entry";
        let comments = [
            line(Span::new(0, 30)),
            line(Span::new(31, u32::try_from(source.len()).expect("source fits"))),
        ];
        let document = Javadocs::new(source, &comments).next().expect("Markdown Javadoc");
        let structure = document.structure();

        assert!(structure.mentions("List"));
        assert!(structure.mentions("Map"));
        assert!(structure.mentions("Entry"));
    }

    #[test]
    fn finder_groups_documents_and_reuses_parsed_structure() {
        let source = "/** First List. */\n/// Second Map.\nclass A {}";
        let comments = [
            Comment { kind: CommentKind::SingleLineBlock, ..line(Span::new(0, 18)) },
            line(Span::new(19, 34)),
        ];
        let finder = JavadocFinder::new(source, &comments);

        let structures = finder.structures().collect::<Vec<_>>();
        assert_eq!(structures.len(), 2);
        assert!(structures[0].mentions("List"));
        assert!(structures[1].mentions("Map"));

        let first_again = finder.structures().next().expect("first Javadoc structure");
        assert!(std::ptr::eq(structures[0], first_again));
    }
}
