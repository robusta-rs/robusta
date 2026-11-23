// Adapted from https://github.com/oxc-project/oxc/blob/732a8e42617296af7aed3b0b374cb76863ac0efc/crates/oxc_parser/src/lexer/trivia_builder.rs
use robusta_ast::{
    Comment,
    CommentContent::{self, Documentation, None},
    CommentKind::{self, MultiLineBlock, SingleLineBlock},
    CommentPosition::{self, Leading, Trailing},
};
use robusta_lexer::tokenize;

fn comments(source: &str) -> Vec<Comment> {
    let result = tokenize(source);
    assert!(result.errors.is_empty(), "{source:?}: {:?}", result.errors);
    result.comments
}

fn contents(source: &str) -> Vec<(&str, CommentContent)> {
    comments(source)
        .iter()
        .map(|comment| (comment.span.source_text(source), comment.content))
        .collect()
}

#[test]
fn documentation_content_follows_the_opening_delimiter() {
    assert_eq!(
        contents("/** a */ /*** b */ /** */ /**/ /* c */ /// d\n///\n//// e\n// f\nint x;"),
        [
            ("/** a */", Documentation),
            ("/*** b */", Documentation),
            ("/** */", Documentation),
            ("/**/", None),
            ("/* c */", None),
            ("/// d", Documentation),
            ("///", Documentation),
            ("//// e", Documentation),
            ("// f", None),
        ]
    );
    // Delimiters are judged after Unicode escapes are translated.
    assert_eq!(
        contents(r"\u002f** a */ /\u002a b */ \u002f\u002f/ c"),
        [
            (r"\u002f** a */", Documentation),
            (r"/\u002a b */", None),
            (r"\u002f\u002f/ c", Documentation)
        ]
    );
}

fn block_kind(source: &str) -> CommentKind {
    let comments = tokenize(source).comments;
    assert_eq!(comments.len(), 1, "{source:?}");
    comments[0].kind
}

#[test]
fn block_comment_line_terminators() {
    for (separator, kind) in [
        ("\n", MultiLineBlock),
        ("\r", MultiLineBlock),
        ("\r\n", MultiLineBlock),
        (r"\u000a", MultiLineBlock),
        (r"\uu000D", MultiLineBlock),
        (r"\u000d\u000a", MultiLineBlock),
        ("*\n*", MultiLineBlock),
        ("", SingleLineBlock),
        (r"\\u000a", SingleLineBlock),
        (r"\u0020", SingleLineBlock),
        ("*", SingleLineBlock),
        ("\u{c}", SingleLineBlock),
        ("\u{85}", SingleLineBlock),
        ("\u{2028}", SingleLineBlock),
        ("\u{2029}", SingleLineBlock),
    ] {
        // A newline before the comment must not affect its kind.
        for prefix in ["", "int a;", "int a;\n"] {
            let source = format!("{prefix}/* first{separator}second */ int b;");
            assert_eq!(block_kind(&source), kind, "{source:?}");
        }
    }
    assert_eq!(block_kind("/**/"), SingleLineBlock);
    assert_eq!(block_kind("/***/"), SingleLineBlock);
    assert_eq!(block_kind(r"/* a *\u002f"), SingleLineBlock);
    assert_eq!(block_kind(r"\u002f* a\u000a *\u002f"), MultiLineBlock);
    // An unterminated comment runs to the end of the file.
    assert_eq!(block_kind("/* a *"), SingleLineBlock);
    assert_eq!(block_kind("/* a\n"), MultiLineBlock);
}

/// Each comment's text, whether a newline precedes it, and whether one follows it.
fn newlines(source: &str) -> Vec<(&str, bool, bool)> {
    comments(source)
        .iter()
        .map(|comment| {
            (
                comment.span.source_text(source),
                comment.preceded_by_newline(),
                comment.followed_by_newline(),
            )
        })
        .collect()
}

#[test]
fn comment_newlines_at_tokens() {
    assert_eq!(newlines("int x; /* a */ int y;"), [("/* a */", false, false)]);
    assert_eq!(newlines("int x; /* a */\nint y;"), [("/* a */", false, true)]);
    assert_eq!(newlines("int x;\n/* a */ int y;"), [("/* a */", true, false)]);
    assert_eq!(newlines("int x;\n  /* a */  \nint y;"), [("/* a */", true, true)]);
    assert_eq!(newlines("int x; // a\nint y;"), [("// a", false, true)]);
    assert_eq!(newlines("/* a */ int x\n;"), [("/* a */", true, false)]);
    assert_eq!(
        newlines("String s = \"\"\"\n  a\n  \"\"\"; /* c */ int y;"),
        [("/* c */", false, false)]
    );
}

#[test]
fn comment_newlines_between_comments() {
    assert_eq!(
        newlines("int x;\n/* a */ /* b */\nint y;"),
        [("/* a */", true, false), ("/* b */", false, true)]
    );
    assert_eq!(
        newlines("int x; /* a */ // b\n/* c */ int y;"),
        [("/* a */", false, false), ("// b", false, true), ("/* c */", true, false)]
    );
    assert_eq!(newlines("// a\n\n// b\n"), [("// a", true, true), ("// b", true, true)]);
    // The comment's own line breaks do not count.
    assert_eq!(
        newlines("int x; /* a\n b */ /* c */ int y;"),
        [("/* a\n b */", false, false), ("/* c */", false, false)]
    );
}

#[test]
fn comment_newlines_at_file_ends() {
    assert_eq!(newlines("/* a */ int x;"), [("/* a */", true, false)]);
    assert_eq!(newlines(" \t\u{c}/* a */ int x;"), [("/* a */", true, false)]);
    assert_eq!(newlines("\u{feff}/* a */ int x;"), [("/* a */", true, false)]);
    assert_eq!(newlines("int x; // a"), [("// a", false, true)]);
    assert_eq!(newlines("int x; /* a */"), [("/* a */", false, false)]);
    assert_eq!(newlines("int x; /* a */  "), [("/* a */", false, false)]);
    assert_eq!(newlines("int x; /* a */\u{1a}"), [("/* a */", false, false)]);
    assert_eq!(newlines("int x; /* a */\n"), [("/* a */", false, true)]);
}

#[test]
fn comment_newlines_read_translated_line_terminators() {
    for terminator in ["\r", "\r\n", r"\u000a", r"\uu000D", r"\u000d\u000a", "\\u000d\n"] {
        let source = format!("int x;{terminator}/* a */{terminator}int y;");
        assert_eq!(newlines(&source), [("/* a */", true, true)], "{source:?}");
    }
    assert_eq!(newlines(r"int x;\u0020/* a */\u0009\u000Cint y;"), [("/* a */", false, false)]);
    // The span stays on the original source, with its escaped delimiters.
    assert_eq!(
        newlines(r"\u000a\u002f* a *\u002f\u000a\u002f\u002f b\u000a"),
        [(r"\u002f* a *\u002f", true, true), (r"\u002f\u002f b", true, true)]
    );
}

/// Each comment's text and position, with the source it attaches to: what
/// starts at a leading comment's offset, what ends at a trailing comment's.
fn attachments(source: &str) -> Vec<(&str, CommentPosition, &str)> {
    comments(source)
        .iter()
        .map(|comment| {
            let attached_to = comment.attached_to as usize;
            let attached = match comment.position {
                Leading => &source[attached_to..],
                Trailing => &source[..attached_to],
            };
            (comment.span.source_text(source), comment.position, attached)
        })
        .collect()
}

#[test]
fn comment_attachments() {
    assert_eq!(
        attachments("/* a */\n// b\n/* c */ x /* d */ // e\n// f\n"),
        [
            ("/* a */", Leading, "x /* d */ // e\n// f\n"),
            ("// b", Leading, "x /* d */ // e\n// f\n"),
            ("/* c */", Leading, "x /* d */ // e\n// f\n"),
            ("/* d */", Trailing, "/* a */\n// b\n/* c */ x"),
            ("// e", Trailing, "/* a */\n// b\n/* c */ x"),
            ("// f", Leading, ""),
        ]
    );
    assert_eq!(attachments("\u{feff}/* a */ x"), [("/* a */", Leading, "x")]);
    assert_eq!(attachments("// a"), [("// a", Leading, "")]);
}

#[test]
fn comments_around_binary_operands_attach_to_their_adjacent_tokens() {
    assert_eq!(
        attachments("/* a */ x /* b */ + /* c */ y /* d */"),
        [
            ("/* a */", Leading, "x /* b */ + /* c */ y /* d */"),
            ("/* b */", Trailing, "/* a */ x"),
            ("/* c */", Leading, "y /* d */"),
            ("/* d */", Trailing, "/* a */ x /* b */ + /* c */ y"),
        ]
    );
}

#[test]
fn infix_comment_attachment_respects_line_and_unary_boundaries() {
    assert_eq!(attachments("x /* a */\n+ y"), [("/* a */", Trailing, "x")]);
    assert_eq!(attachments("x\n/* a */ +y"), [("/* a */", Leading, "+y")]);
    assert_eq!(attachments("return /* a */ +y"), [("/* a */", Leading, "+y")]);
    assert_eq!(attachments("x /* a */ y"), [("/* a */", Leading, "y")]);
    assert_eq!(attachments("f(); /* a */"), [("/* a */", Trailing, "f();")]);
    assert_eq!(attachments("x /* a */ += y"), [("/* a */", Trailing, "x")]);
    assert_eq!(attachments("x /* a */ instanceof y"), [("/* a */", Trailing, "x")]);
}

#[test]
fn java_expression_ends_take_a_comment_before_an_operator() {
    for source in [
        "this /* a */ == y",
        "A.class /* a */ == y",
        "\"s\" /* a */ + y",
        "1 /* a */ + y",
        "null /* a */ == y",
        "true /* a */ && y",
        "f() /* a */ + y",
        "x[0] /* a */ + y",
        "new int[] {} /* a */ == y",
        "x++ /* a */ + y",
        "x-- /* a */ + y",
        "record /* a */ + y",
        "x instanceof List<?> /* a */ && y",
    ] {
        let token = source.split(" /*").next().unwrap();
        assert_eq!(attachments(source), [("/* a */", Trailing, token)], "{source:?}");
    }
}

#[test]
fn line_comments_after_assign_and_open_tokens_stay_leading() {
    for source in ["x = // a\ny", "f( // a\ny", "x ? y : // a\nz", "case 1: // a\ny"] {
        let next = &source[source.rfind('\n').unwrap() + 1..];
        assert_eq!(attachments(source), [("// a", Leading, next)], "{source:?}");
    }
    assert_eq!(attachments("x = /* a */\ny"), [("/* a */", Trailing, "x =")]);
}

#[test]
fn documentation_comments_stay_leading() {
    for source in ["int x; /** a */ int y;", "int x; /** a */\nint y;", "int x; /// a\nint y;"] {
        assert!(attachments(source).iter().all(|(_, position, _)| *position == Leading));
    }
    assert_eq!(
        attachments("int x; /// a\n/// b\nint y;"),
        [("/// a", Leading, "int y;"), ("/// b", Leading, "int y;")]
    );
    assert_eq!(attachments("x /** a */ + y"), [("/** a */", Leading, "+ y")]);
    assert_eq!(attachments("int x; /** a */"), [("/** a */", Leading, "")]);
    // Delimiters are judged after Unicode escapes are translated.
    assert_eq!(attachments("int x; \\u002f** a */\ny"), [(r"\u002f** a */", Leading, "y")]);
}

#[test]
fn comments_after_a_documentation_comment_attach_to_the_same_token() {
    assert_eq!(
        attachments("int x; /** a */ // b\nint y;"),
        [("/** a */", Leading, "int y;"), ("// b", Leading, "int y;")]
    );
    assert_eq!(
        attachments("int x; /** a */ /* b */\nint y;"),
        [("/** a */", Leading, "int y;"), ("/* b */", Leading, "int y;")]
    );
    assert_eq!(
        attachments("x /** a */ /* b */ + y"),
        [("/** a */", Leading, "+ y"), ("/* b */", Leading, "+ y")]
    );
}

#[test]
fn comments_before_a_documentation_comment_on_a_line() {
    assert_eq!(
        attachments("int x; /* a */ /** b */ /* c */\nint y;"),
        [
            ("/* a */", Trailing, "int x;"),
            ("/** b */", Leading, "int y;"),
            ("/* c */", Leading, "int y;")
        ]
    );
    assert_eq!(
        attachments("int x; /* a */ /** b */\nint y;"),
        [("/* a */", Leading, "int y;"), ("/** b */", Leading, "int y;")]
    );
    assert_eq!(
        attachments("int x; /* a */ /** b */ // c\nint y;"),
        [
            ("/* a */", Trailing, "int x;"),
            ("/** b */", Leading, "int y;"),
            ("// c", Leading, "int y;")
        ]
    );
}

#[test]
fn attachments_map_to_the_original_source_through_escapes() {
    assert_eq!(
        attachments(r"/** a */\u000a/* b */\uu0009\u0063lass A {}"),
        [("/** a */", Leading, r"\u0063lass A {}"), ("/* b */", Leading, r"\u0063lass A {}"),]
    );
    assert_eq!(
        attachments(r"int x\u003b\u0020/* a */\u000aint y\u003b\u0020// b"),
        [
            ("/* a */", Trailing, r"int x\u003b"),
            ("// b", Trailing, r"int x\u003b\u0020/* a */\u000aint y\u003b"),
        ]
    );
    // An attachment equals the offset of the token it names.
    let source = r"\u0078\u0020/* a */\u0020\u002b\u0020/* b */\u0020\u0079";
    let result = tokenize(source);
    assert_eq!(result.comments[0].attached_to, result.tokens[0].span.end);
    assert_eq!(result.comments[1].attached_to, result.tokens[2].span.start);
}
