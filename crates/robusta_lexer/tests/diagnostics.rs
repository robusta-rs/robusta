//! Lexical error messages, compared in full. `*_never_flagged` tests lex
//! valid input of the same kind and expect no message.

use robusta_ast::CommentKind;
use robusta_lexer::{
    MAX_LEN, Token,
    TokenKind::{self, *},
    tokenize,
};
use robusta_span::Span;

const NONE: [&str; 0] = [];

fn messages(source: &str) -> Vec<String> {
    tokenize(source).errors.into_iter().map(|error| error.message).collect()
}

/// Lex and return the token kinds (EOF dropped).
fn kinds(source: &str) -> Vec<TokenKind> {
    let mut kinds: Vec<_> = tokenize(source).tokens.iter().map(|token| token.kind).collect();
    assert_eq!(kinds.pop(), Some(Eof));
    kinds
}

/// Lex and return the source text of every token (EOF dropped).
fn spellings(source: &str) -> Vec<&str> {
    let tokens = tokenize(source).tokens;
    tokens[..tokens.len() - 1].iter().map(|token| token.span.source_text(source)).collect()
}

fn comments(source: &str) -> Vec<&str> {
    tokenize(source).comments.iter().map(|comment| comment.span.source_text(source)).collect()
}

#[test]
fn misplaced_byte_order_mark() {
    for source in ["class A {}\u{feff}", r"\uFEFFclass A {}"] {
        assert_eq!(messages(source), [r"unexpected character '\u{feff}'"], "{source}");
        assert!(kinds(source).contains(&Unknown), "{source}");
    }
}

#[test]
fn leading_byte_order_mark_never_flagged() {
    let source = "\u{feff}cl\\u0061ss A {}";
    let tokens = tokenize(source).tokens;
    assert_eq!(messages(source), NONE);
    assert_eq!(kinds(source), [Class, Identifier, LBrace, RBrace]);
    assert_eq!(tokens[0].span, Span::new(3, 13));
    assert_eq!(tokens[0].span.source_text(source), r"cl\u0061ss");
    assert_eq!(tokens.last().unwrap().span, Span::empty(18));
}

#[test]
fn final_sub_never_flagged() {
    for suffix in ["\x1a", r"\u001a", r"\uu001A"] {
        for prefix in ["", "class Z {}", "\u{feff}cl\\u0061ss Z {}\n"] {
            let source = format!("{prefix}{suffix}");
            let result = tokenize(&source);
            assert!(result.errors.is_empty(), "{source:?}: {:?}", result.errors);
            assert_eq!(kinds(&source), kinds(prefix), "{source:?}");
            assert_eq!(
                result.tokens.last().unwrap().span,
                Span::empty(u32::try_from(source.len()).unwrap()),
                "{source:?}"
            );
        }
    }
}

#[test]
fn nonfinal_sub_is_not_an_end_marker() {
    for sub in ["\x1a", r"\u001a"] {
        for following in [" ", "\n", "\r\n", " class Y {}", "\x1a", r"\u000a"] {
            let source = format!("class Z {{}}{sub}{following}");
            assert_eq!(messages(&source), [r"unexpected character '\u{1a}'"], "{source:?}");
            assert!(kinds(&source).contains(&Unknown), "{source:?}");
        }
    }
}

#[test]
fn sub_inside_an_input_element_is_preserved() {
    for sub in ["\x1a", r"\u001a"] {
        for identifier in [format!("a{sub}b"), format!("a{sub}")] {
            let result = tokenize(&identifier);
            assert!(result.errors.is_empty(), "{identifier:?}: {:?}", result.errors);
            assert_eq!(kinds(&identifier), [Identifier]);
            assert_eq!(spellings(&identifier), [identifier.as_str()]);
            assert_eq!(
                result.identifier_names[&0],
                if identifier.ends_with('b') { "ab" } else { "a" }
            );
        }
        let source = format!("// note{sub}");
        assert_eq!(messages(&source), NONE);
        assert_eq!(comments(&source), [source.as_str()]);
        assert_eq!(messages(&format!("/* note{sub}")), ["unterminated block comment"]);
        assert_eq!(messages(&format!("\"note{sub}")), ["unterminated string literal"]);
    }
}

#[test]
fn malformed_integer_literals() {
    assert_eq!(
        messages("long x = 0x;"),
        ["hexadecimal integer literal must contain at least one digit"]
    );
    assert_eq!(messages("int x = 0b_101;"), ["underscore must separate digits in integer literal"]);
    assert_eq!(messages("int x = 1_;"), ["underscore must separate digits in integer literal"]);
    assert_eq!(messages("int x = 08;"), ["invalid digit in octal integer literal"]);
}

#[test]
fn integer_literals_never_flagged() {
    let source = "0 42 1_000 42L 42l 0xCAFE_BABEL 0Xf00d 0777 0_755 0b1010_0101 0B1l";
    assert_eq!(messages(source), NONE);
    assert_eq!(kinds(source), [IntLiteral; 11]);
    assert_eq!(spellings(source), source.split(' ').collect::<Vec<_>>());
}

#[test]
fn malformed_floating_point_literals() {
    assert_eq!(
        messages("double x = 1e+;"),
        ["floating-point exponent must contain at least one digit"]
    );
    assert_eq!(
        messages("double x = 0x1.0;"),
        ["hexadecimal floating-point literal requires a binary exponent"]
    );
    assert_eq!(
        messages("double x = 0x.p1;"),
        ["hexadecimal floating-point literal must contain at least one digit"]
    );
    assert_eq!(
        messages("double x = 1_.0;"),
        ["underscore must separate digits in floating-point literal"]
    );
    assert_eq!(
        messages("double x = .5_;"),
        ["underscore must separate digits in floating-point literal"]
    );
}

#[test]
fn floating_point_literals_never_flagged() {
    let source = "0.0 1. .5 1e10 1e-10F 6_022e+20D 09.0 0x1p4 0x1.fp3 0x.8P-1d 0x1.FP+2";
    assert_eq!(messages(source), NONE);
    assert_eq!(kinds(source), [FloatLiteral; 11]);
    assert_eq!(spellings(source), source.split(' ').collect::<Vec<_>>());
}

#[test]
fn malformed_character_literals() {
    assert_eq!(messages("char c = '';"), ["empty character literal"]);
    assert_eq!(messages("char c = 'ab';"), ["character literal may only contain one character"]);
    assert_eq!(messages(r"char c = '\q';"), ["invalid escape sequence in character literal"]);
    assert_eq!(messages("char c = 'a\n"), ["unterminated character literal"]);
    assert_eq!(
        messages(r"char c = '\u000a';"),
        ["unterminated character literal", "unterminated character literal"]
    );
}

#[test]
fn character_literals_never_flagged() {
    let source =
        r"char c = 'a'; char nl = '\n'; char quote = '\''; char oct = '\123'; char u = '\u0061';";
    assert_eq!(messages(source), NONE);
    assert_eq!(kinds(source), [Char, Identifier, Assign, CharLiteral, Semicolon].repeat(5));
}

#[test]
fn unexpected_character() {
    // Identifier-part characters cannot start an identifier.
    for (source, message) in [
        ("٢name", "unexpected character '٢'"),
        ("\u{0301}name", r"unexpected character '\u{301}'"),
        ("\u{0903}name", "unexpected character '\u{0903}'"),
        ("\u{200c}name", r"unexpected character '\u{200c}'"),
        ("\u{0007}name", r"unexpected character '\u{7}'"),
    ] {
        assert_eq!(messages(source), [message], "{source:?}");
        assert_eq!(kinds(source), [Unknown, Identifier], "{source:?}");
    }

    let source = "class ☃ {}";
    assert_eq!(messages(source), ["unexpected character '☃'"]);
    assert_eq!(spellings(source), ["class", "☃", "{", "}"]);
    assert_eq!(kinds(source), [Class, Unknown, LBrace, RBrace]);
}

#[test]
fn identifiers_never_flagged() {
    for identifier in [
        "Café",
        "变量٢",
        "𝒂",
        "ǅname",
        "ʰname",
        "Ⅰname",
        "$price",
        "€price",
        "‿name",
        "a\u{0301}",
        "a\u{0903}",
        "a\u{200c}b",
        "a\u{0007}b",
    ] {
        assert_eq!(messages(identifier), NONE, "{identifier:?}");
        assert_eq!(kinds(identifier), [Identifier], "{identifier:?}");
    }
}

#[test]
fn unterminated_block_comment() {
    let source = "/* unterminated";
    assert_eq!(messages(source), ["unterminated block comment"]);
    assert_eq!(comments(source), [source]);
    assert_eq!(tokenize(source).tokens, [Token { kind: Eof, span: Span::empty(15) }]);
}

#[test]
fn comments_never_flagged() {
    let source = "// hi\nclass A {} /* block */";
    assert_eq!(messages(source), NONE);
    assert_eq!(
        tokenize(source).comments.iter().map(|comment| comment.kind).collect::<Vec<_>>(),
        [CommentKind::Line, CommentKind::SingleLineBlock]
    );

    let source = "/**/class A {}";
    assert_eq!(messages(source), NONE);
    assert_eq!(comments(source), ["/**/"]);
    assert_eq!(kinds(source), [Class, Identifier, LBrace, RBrace]);

    let source = "// hi\rclass A {}";
    assert_eq!(messages(source), NONE);
    assert_eq!(comments(source), ["// hi"]);
    assert_eq!(kinds(source), [Class, Identifier, LBrace, RBrace]);

    let source = "// café\nString s = \"café\"; /* café */";
    assert_eq!(messages(source), NONE);
    assert_eq!(comments(source), ["// café", "/* café */"]);
}

#[test]
fn unterminated_string_literal() {
    assert_eq!(messages("\"oops"), ["unterminated string literal"]);
    for source in ["\"oops\rclass A {}", "\"\\\nclass A {}"] {
        assert_eq!(messages(source), ["unterminated string literal"], "{source:?}");
        assert_eq!(kinds(source), [StringLiteral, Class, Identifier, LBrace, RBrace], "{source:?}");
    }
    // A line break ends the string, also when the search for it starts after an escape and
    // passes more than 16 bytes, and also when a quote follows on the next line.
    let tail = "x".repeat(40);
    for terminator in ["\r", "\n", "\r\n"] {
        for escape in ["", "\\t"] {
            let literal = format!("\"{escape}{tail}");
            let source = format!("{literal}{terminator}\";");
            assert_eq!(
                messages(&source),
                ["unterminated string literal", "unterminated string literal"],
                "{source:?}"
            );
            assert_eq!(spellings(&source), [literal.as_str(), "\";"], "{source:?}");
        }
    }
    let source = "\"\\t\\n";
    assert_eq!(messages(source), ["unterminated string literal"]);
    assert_eq!(spellings(source), [source]);
    // `\u000a` becomes a line terminator before the string scanner runs.
    assert_eq!(
        messages(r#"String s = "\u000a";"#),
        ["unterminated string literal", "unterminated string literal"]
    );
}

#[test]
fn invalid_escape_in_string_literal() {
    let source = r#""\q""#;
    assert_eq!(messages(source), ["invalid escape sequence in string literal"]);
    assert_eq!(kinds(source), [StringLiteral]);
}

#[test]
fn string_literals_never_flagged() {
    let source = r#"String s = "hi\"there";"#;
    assert_eq!(messages(source), NONE);
    assert_eq!(kinds(source), [Identifier, Identifier, Assign, StringLiteral, Semicolon]);

    // Quotes and escapes before, at, and after the 16-byte batches the search reads.
    for literal in [
        format!(r#""{}""#, "a".repeat(15)),
        format!(r#""{}""#, "a".repeat(16)),
        format!(r#""{}""#, "a".repeat(40)),
        format!(r#""{}\"{}""#, "é".repeat(10), "b".repeat(20)),
        format!(r#""\t{}\n{}\"{}""#, "a".repeat(20), "b".repeat(16), "c".repeat(30)),
    ] {
        let source = format!("String s = {literal};");
        assert_eq!(messages(&source), NONE, "{source}");
        assert_eq!(spellings(&source), ["String", "s", "=", literal.as_str(), ";"], "{source}");
    }

    // `\\u2126` is an escaped backslash then literal `u2126`, never U+2126.
    let source = r#"String s = "\\u2126";"#;
    assert_eq!(messages(source), NONE);
    assert_eq!(spellings(source), ["String", "s", "=", r#""\\u2126""#, ";"]);
}

#[test]
fn unterminated_text_block() {
    assert_eq!(messages("String s = \"\"\"\nmissing"), ["unterminated text block"]);

    let source = "String s = \"\"\"\nnot \"\" closed\\\"\"\"\n";
    assert_eq!(messages(source), ["unterminated text block"]);
    assert_eq!(spellings(source), ["String", "s", "=", &source[11..]]);
}

#[test]
fn text_blocks_never_flagged() {
    let source = "String s = \"\"\"\nhello\n\"\"\";";
    assert_eq!(messages(source), NONE);
    assert_eq!(spellings(source), ["String", "s", "=", "\"\"\"\nhello\n\"\"\"", ";"]);
    assert_eq!(kinds(source), [Identifier, Identifier, Assign, StringLiteral, Semicolon]);

    // Without a line terminator after it, `"""` is an empty string and then a string.
    let source = r#"String s = """not a text block";"#;
    assert_eq!(messages(source), NONE);
    assert_eq!(
        kinds(source),
        [Identifier, Identifier, Assign, StringLiteral, StringLiteral, Semicolon]
    );

    let source = r#"String s = """
\"""
\
""";"#;
    assert_eq!(messages(source), NONE);
    assert_eq!(kinds(source), [Identifier, Identifier, Assign, StringLiteral, Semicolon]);
    assert_eq!(spellings(source)[3], &source[11..source.len() - 1]);

    let source = r#"String s = """
  Grüße, «"quoted"» ""\t€ 😀 \"""
  """;"#;
    assert_eq!(messages(source), NONE);
    assert_eq!(kinds(source), [Identifier, Identifier, Assign, StringLiteral, Semicolon]);
    assert_eq!(spellings(source)[3], &source[11..source.len() - 1]);
}

// Skip on 32-bit systems: a string longer than `isize::MAX` cannot exist.
#[cfg(target_pointer_width = "64")]
#[test]
fn source_too_long() {
    // `vec![0; n]` allocates zeroed pages without writing 4 GiB, and the
    // length guard rejects the source before any byte is read.
    let bytes = vec![0u8; MAX_LEN + 1];
    // SAFETY: a slice consisting entirely of zeros is valid UTF-8.
    let source = unsafe { std::str::from_utf8_unchecked(&bytes) };
    let result = tokenize(source);
    assert_eq!(result.tokens.iter().map(|token| token.kind).collect::<Vec<_>>(), [Eof]);
    assert_eq!(comments(source), NONE);
    assert_eq!(messages(source), ["source length exceeds 4 GiB limit"]);
}
