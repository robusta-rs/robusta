use robusta_ast::Comment;
use robusta_span::Span;

use crate::structure::{self, Structure};

use super::utils;

/// The source form of a Java documentation comment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JavadocKind {
    /// A traditional `/** ... */` documentation comment.
    Traditional,
    /// A consecutive run of `///` end-of-line documentation comments.
    Markdown,
}

/// A documentation comment borrowed from a Java source file.
///
/// [`Javadoc::lines`], [`Javadoc::structure`], and [`Javadoc::literal_regions`]
/// read the delimiters as written, so a comment that spells one with a Unicode
/// escape is read through [`Javadoc::translated_structure`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Javadoc<'a> {
    pub(super) text: &'a str,
    pub(super) span: Span,
    pub(super) kind: JavadocKind,
    /// The start of the token the comment attaches to.
    pub(super) attached_to: u32,
}

impl<'a> Javadoc<'a> {
    /// The complete source slice. For Markdown Javadoc, this spans every `///`
    /// line in the run, including intervening line terminators and indentation.
    pub fn text(self) -> &'a str {
        self.text
    }

    /// The complete half-open source span of this documentation comment.
    pub fn span(self) -> Span {
        self.span
    }

    /// Whether this document uses traditional block or Markdown line syntax.
    pub fn kind(self) -> JavadocKind {
        self.kind
    }

    /// Physical content lines with the source form's conventional margin
    /// removed.
    pub fn lines(self) -> Vec<&'a str> {
        match self.kind {
            JavadocKind::Traditional => {
                let inner = &self.text[3..];
                utils::traditional_lines(inner.strip_suffix("*/").unwrap_or(inner))
            }
            JavadocKind::Markdown => utils::markdown_lines(self.text),
        }
    }

    /// Content lines with literal delimiters, after translating only the
    /// intervening line terminators and indentation. Content keeps its raw spelling.
    pub fn literal_lines(self) -> Option<Vec<&'a str>> {
        match self.kind {
            JavadocKind::Traditional => {
                (self.text.starts_with("/**") && self.text.ends_with("*/")).then(|| self.lines())
            }
            JavadocKind::Markdown => utils::markdown_literal_lines(self.text)
                .collect::<Option<Vec<_>>>()
                .map(utils::markdown_margin),
        }
    }

    /// Whether every comment delimiter is written literally in the source.
    pub fn has_literal_delimiters(self) -> bool {
        match self.kind {
            JavadocKind::Traditional => self.text.starts_with("/**") && self.text.ends_with("*/"),
            JavadocKind::Markdown => {
                utils::markdown_literal_lines(self.text).all(|line| line.is_some())
            }
        }
    }

    /// Whether this comment is Javadoc where it stands: the token it attaches
    /// to begins one of `declaration_starts`, the sorted start offsets of the
    /// file's declarations, annotations and modifiers included.
    ///
    /// `comments` is the file's comment side-table. A comment followed by a
    /// statement, the end of a body, or another documentation comment is
    /// ordinary, since javadoc reads only the last before a declaration.
    pub fn is_attached(
        self,
        source: &str,
        comments: &[Comment],
        declaration_starts: &[u32],
    ) -> bool {
        self.documented_offset(source, comments)
            .is_some_and(|next| declaration_starts.binary_search(&next).is_ok())
    }

    /// Where the declaration this comment documents would start: the token it
    /// attaches to. `None` when another documentation comment comes first.
    pub fn documented_offset(self, source: &str, comments: &[Comment]) -> Option<u32> {
        let first = comments.partition_point(|comment| comment.span.start < self.span.end);
        let superseded = comments[first..]
            .iter()
            .take_while(|comment| comment.attached_to == self.attached_to)
            // The empty `/**/` takes precedence without being reformatted.
            .any(|comment| {
                comment.is_documentation()
                    || (comment.is_block()
                        && robusta_lexer::translate(comment.span.source_text(source)).cooked
                            == "/**/")
            });
        (!superseded).then_some(self.attached_to)
    }

    /// Parse the prose, block tags, inline tags, and literal regions.
    pub fn structure(self) -> Structure {
        Structure::parse(self)
    }

    /// [`Self::structure`] of the text javadoc reads, after Unicode escape
    /// translation (JLS §3.3).
    pub fn translated_structure(self) -> Structure {
        let text = robusta_lexer::translate(self.text).cooked;
        Javadoc { text: &text, ..self }.structure()
    }

    /// Code, literal, preformatted, and fenced regions kept indivisible.
    pub fn literal_regions(self) -> Vec<String> {
        structure::literal_regions(self)
    }

    /// Whether the last line is Markdown code, so that its trailing whitespace
    /// is code text.
    pub fn ends_in_code(self) -> bool {
        self.kind == JavadocKind::Markdown
            && self.literal_lines().is_some_and(|lines| structure::ends_in_code(&lines.join("\n")))
    }
}

#[cfg(test)]
mod tests {
    use robusta_ast::CommentKind;

    use crate::TagForm;
    use crate::parser::tests::{comment, parse};

    use super::*;

    #[test]
    fn exposes_margin_stripped_lines_for_all_java_terminators() {
        let document = parse("/**\r\n * first\r *   second\n */");

        assert_eq!(document.lines(), ["", "first", "  second", ""]);
    }

    #[test]
    fn a_comment_is_attached_when_a_declaration_is_the_next_token() {
        let attached = |source: &str, declaration: Option<&str>| {
            let comments = robusta_lexer::tokenize(source).comments;
            let first = comments.iter().position(Comment::is_documentation).expect("Javadoc");
            let (document, _) =
                Javadoc::from_comments(source, &comments[first..]).expect("Javadoc");
            let starts = declaration.map(|needle| {
                u32::try_from(source.find(needle).expect("declaration in source")).expect("fits")
            });
            document.is_attached(source, &comments, starts.as_slice())
        };

        assert!(attached("/// one\n/// two\n  @Deprecated class A {}", Some("@Deprecated")));
        assert!(attached("/// one\r\nclass A {}", Some("class")));
        assert!(attached("/// one\n\nclass A {}", Some("class")));
        assert!(attached("/// one\n// note\nclass A {}", Some("class")));
        assert!(attached(r"/// one\u000a\u0020class A {}", Some("class")));
        assert!(attached(r"/** one */\u000a/* note */\uu0009class A {}", Some("class")));
        assert!(attached("/** one */ /* note */ class A {}", Some("class")));
        assert!(attached("/** one */ // note\nclass A {}", Some("class")));
        assert!(attached("int x; /** one */ // note\nclass A {}", Some("class")));
        assert!(attached("int x; /// one\n/// two\nclass A {}", Some("class")));
        assert!(attached("int x; /* note */ /// one\nclass A {}", Some("class")));
        assert!(attached(r"int x; /** one */\u000a\u0063lass A {}", Some(r"\u0063lass")));
        assert!(attached(r"/** one */\u0020\u000a@Deprecated class A {}", Some("@Deprecated")));
        assert!(attached(r"/// one\u000a\u0040Deprecated class A {}", Some(r"\u0040Deprecated")));
        assert!(attached(r"class A { int x; /** one */\u000a\u0020int y; }", Some("int y")));
        assert!(attached("/*** banner */\nclass A {}", Some("class")));
        assert!(!attached("/// note one\n/// note two\ndoThing();", None));
        assert!(!attached("/** note */ doThing();", None));
        assert!(!attached("/// trailing\n}", None));
        assert!(!attached("/// one", None));
        assert!(!attached("/** one */\n/** two */\nclass A {}", Some("class")));
        assert!(!attached("/** one */ /**/ class A {}", Some("class")));
        assert!(!attached(r"/** one */ /\u002a*/ class A {}", Some("class")));
        assert!(!attached("/// one\n\n/// two\nclass A {}", Some("class")));
        assert!(!attached("/** one */ /// two\nclass A {}", Some("class")));
        assert!(!attached("/** one */ /* note */ /** two */ class A {}", Some("class")));
        assert!(!attached("/** one */ int x; class A {}", Some("class")));
    }

    #[test]
    fn documented_offset_reads_the_token_a_comment_attaches_to() {
        let source = "int x; /** one */ // note\n/* note */ class A {}";
        let comments = robusta_lexer::tokenize(source).comments;
        let document = Javadoc::from_comment(source, &comments[0]).expect("Javadoc");

        assert_eq!(
            document.documented_offset(source, &comments),
            source.find("class").map(|start| u32::try_from(start).expect("fits"))
        );
    }

    #[test]
    fn exposes_block_and_inline_tag_subjects() {
        let document = parse(
            "/**\n * A {@link List#of(Object) linked list}.\n * @param value the input\n * @see Map.Entry\n * @throws IOException on failure\n */",
        );
        let structure = document.structure();
        let tags = structure.tags();

        assert_eq!(
            tags.iter()
                .map(|tag| (tag.form(), tag.name(), tag.referenced_name()))
                .collect::<Vec<_>>(),
            [
                (TagForm::Inline, "link", Some("List#of(Object)")),
                (TagForm::Block, "param", Some("value")),
                (TagForm::Block, "see", Some("Map.Entry")),
                (TagForm::Block, "throws", Some("IOException")),
            ]
        );
    }

    #[test]
    fn name_lookup_uses_tags_and_preserves_the_conservative_prose_fallback() {
        let structure =
            parse("/** A List in prose, not Lists. {@link Map.Entry}. @see IOException */")
                .structure();

        assert!(structure.mentions("List"));
        assert!(structure.mentions("Map"));
        assert!(structure.mentions("Entry"));
        assert!(structure.mentions("IOException"));
        assert!(!structure.mentions("Set"));
    }

    #[test]
    fn only_the_translated_structure_reads_unicode_escapes() {
        let document = parse(r"/** {@link M\u0061p} */");

        assert!(document.translated_structure().mentions("Map"));
        assert!(!document.structure().mentions("Map"));
    }

    #[test]
    fn exposes_literal_regions_separately_from_reflowable_prose() {
        let document =
            parse("/**\n * Before {@code Map<{A}, B>} after.\n * <pre>call(  x);</pre>\n */");

        assert_eq!(document.literal_regions(), ["{@code Map<{A}, B>}", "<pre>call(  x);</pre>"]);
    }

    #[test]
    fn exposes_markdown_code_regions_without_parsing_tags_inside_them() {
        let source = "/// Before `{@link List}`.\n/// ```java\n/// call(  value );\n/// ```";
        let document = Javadoc::from_comment(source, &comment(source, CommentKind::Line))
            .expect("test source is Markdown Javadoc");

        assert_eq!(document.literal_regions(), ["`{@link List}`", "```java\ncall(  value );\n```"]);
        assert_eq!(document.structure().tags(), []);
    }
}
