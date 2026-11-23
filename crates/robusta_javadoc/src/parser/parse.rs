use robusta_ast::Comment;
use robusta_span::Span;
use robusta_syntax::{
    identifier::is_horizontal_whitespace,
    line_terminator::{is_line_terminator, terminator_count},
};

use super::{Javadoc, JavadocKind};

impl<'a> Javadoc<'a> {
    /// Recognize and borrow `comment` as Javadoc.
    pub fn from_comment(source: &'a str, comment: &Comment) -> Option<Self> {
        Self::from_comments(source, std::slice::from_ref(comment)).map(|(document, _)| document)
    }

    /// Recognize the documentation comment beginning at the front of
    /// `comments`, returning it and the number of lexer comments it consumes.
    ///
    /// Traditional Javadoc consumes one block comment. Markdown Javadoc
    /// consumes the complete consecutive `///` run; a blank line, an ordinary
    /// comment, or Java syntax ends the run. Use [`Javadoc::is_attached`] to
    /// require the comment to document a declaration.
    pub fn from_comments(source: &'a str, comments: &[Comment]) -> Option<(Self, usize)> {
        let first = *comments.first()?;
        if !first.is_documentation() {
            return None;
        }
        let attached_to = first.attached_to;
        if first.is_block() {
            let text = first.span.source_text(source);
            let kind = JavadocKind::Traditional;
            return Some((Self { text, span: first.span, kind, attached_to }, 1));
        }

        let mut consumed = 1usize;
        while let Some(next) = comments.get(consumed).copied() {
            let previous = comments[consumed - 1];
            if !next.is_line()
                || !next.is_documentation()
                || !markdown_lines_are_consecutive(source, previous, next)
            {
                break;
            }
            consumed += 1;
        }

        let last = comments[consumed - 1];
        let span = Span::new(first.span.start, last.span.end);
        let text = span.source_text(source);
        let kind = JavadocKind::Markdown;
        Some((Self { text, span, kind, attached_to }, consumed))
    }
}

fn markdown_lines_are_consecutive(source: &str, previous: Comment, next: Comment) -> bool {
    if previous.span.end > next.span.start {
        return false;
    }
    let gap =
        robusta_lexer::translate(&source[previous.span.end as usize..next.span.start as usize])
            .cooked;
    terminator_count(&gap) == 1
        && gap.bytes().all(|byte| is_horizontal_whitespace(byte) || is_line_terminator(byte))
}

#[cfg(test)]
mod tests {
    use robusta_ast::{CommentContent, CommentKind};

    use crate::parser::tests::{comment, line};

    use super::*;

    #[test]
    fn recognizes_documentation_comments_as_the_lexer_classifies_them() {
        let document = |source, kind| Javadoc::from_comment(source, &comment(source, kind));

        assert!(document("/**\n * text\n */", CommentKind::MultiLineBlock).is_some());
        assert!(document("/*****\n * Copyright\n *****/", CommentKind::MultiLineBlock).is_some());
        assert!(document("/// text", CommentKind::Line).is_some());
        assert!(document(r"\u002f** text */", CommentKind::SingleLineBlock).is_some());
        assert!(document(r"/** text *\u002f", CommentKind::SingleLineBlock).is_some());
        assert!(document(r"\u002f// text", CommentKind::Line).is_some());
        let plain = "/* text */";
        let ordinary = Comment {
            content: CommentContent::None,
            ..comment(plain, CommentKind::SingleLineBlock)
        };
        assert!(Javadoc::from_comment(plain, &ordinary).is_none());
    }

    #[test]
    fn groups_consecutive_markdown_lines_into_one_document() {
        let source = "/// First.\r\n  ///   indented\r/// Last.";
        let comments = [
            line(Span::new(0, 10)),
            line(Span::new(14, 28)),
            line(Span::new(29, u32::try_from(source.len()).expect("source fits"))),
        ];

        let (document, consumed) =
            Javadoc::from_comments(source, &comments).expect("Markdown Javadoc run");

        assert_eq!(consumed, 3);
        assert_eq!(document.kind(), JavadocKind::Markdown);
        assert_eq!(document.lines(), ["First.", "  indented", "Last."]);
        assert_eq!(document.span(), Span::new(0, u32::try_from(source.len()).expect("fits")));
    }

    #[test]
    fn four_slash_lines_continue_the_run_as_markdown_lines() {
        let source = "/// a\n//// b\n////";
        let comments = robusta_lexer::tokenize(source).comments;

        let (document, consumed) = Javadoc::from_comments(source, &comments).expect("Javadoc");

        assert_eq!(consumed, 3);
        assert_eq!(document.lines(), [" a", "/ b", "/"]);
    }

    #[test]
    fn groups_markdown_lines_after_translating_whitespace_escapes() {
        for (gap, consumed) in [
            (r"\u000a", 2),
            (r"\u000d", 2),
            (r"\u000d\u000a", 2),
            ("\r\\u000a", 2),
            ("\\u000d\n", 2),
            (r"\uu000a\u0020\u0009\u000c", 2),
            ("\n\n", 1),
            (r"\u000a\u000a", 1),
            (r"\u000d\u000a\u000a", 1),
            ("\n\\u0020\n", 1),
            (r"\u000a\u003b", 1),
        ] {
            let source = format!("/// First.{gap}/// Last.");
            let comments = robusta_lexer::tokenize(&source).comments;
            let (document, count) = Javadoc::from_comments(&source, &comments).expect("Javadoc");

            assert_eq!(count, consumed, "{gap:?}");
            let expected_end = comments[consumed - 1].span.end;
            assert_eq!(document.span(), Span::new(0, expected_end), "{gap:?}");
            if consumed == 2 {
                let translated = robusta_lexer::translate(document.text());
                assert_eq!(
                    Javadoc { text: &translated.cooked, ..document }.lines(),
                    ["First.", "Last."],
                    "{gap:?}"
                );
            }
        }
    }
}
