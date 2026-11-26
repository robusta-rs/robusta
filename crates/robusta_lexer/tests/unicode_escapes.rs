use robusta_ast::CommentKind;
use robusta_lexer::{
    TokenKind::{self, *},
    tokenize,
};

/// Lex and return the token kinds (EOF dropped).
fn kinds(source: &str) -> Vec<TokenKind> {
    let mut kinds: Vec<_> = tokenize(source).tokens.iter().map(|token| token.kind).collect();
    assert_eq!(kinds.pop(), Some(Eof));
    kinds
}

#[test]
fn raw_and_escaped_identifiers_preserve_source_spans() {
    let source = r"𝒂 Caf\u00e9 \uD835\uDC82 int \u0061";
    let result = tokenize(source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(kinds(source), [Identifier, Identifier, Identifier, Int, Identifier]);
    assert_eq!(
        result.tokens.iter().map(|token| token.span.source_text(source)).collect::<Vec<_>>(),
        ["𝒂", r"Caf\u00e9", r"\uD835\uDC82", "int", r"\u0061", ""]
    );
}

#[test]
fn decoded_identifier_table_is_sparse_and_uses_original_offsets() {
    let source = "\\u0020 cl\\u0061ss f\\u006fo plain \\u0072ecord \\uD835\\uDC82 co\u{200c}unt co\\u200Cunt";
    let result = tokenize(source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(result.identifier_names.len(), 5);
    for (raw, cooked) in [
        (r"f\u006fo", "foo"),
        (r"\u0072ecord", "record"),
        (r"\uD835\uDC82", "𝒂"),
        ("co\u{200c}unt", "count"),
        (r"co\u200Cunt", "count"),
    ] {
        let token = result
            .tokens
            .iter()
            .find(|token| token.span.source_text(source) == raw)
            .expect("identifier token at its original source span");
        assert_eq!(result.identifier_names[&token.span.start], cooked);
    }
    for source in ["class C { int plain; }", r#"class C { String s = "\u0061"; }"#] {
        assert!(tokenize(source).identifier_names.is_empty());
    }
}

#[test]
fn escaped_slashes_form_a_line_comment() {
    let source = r"\u002f\u002f c
class A {}";
    let result = tokenize(source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(result.comments.len(), 1);
    assert_eq!(result.comments[0].kind, CommentKind::Line);
    assert_eq!(result.comments[0].span.source_text(source), r"\u002f\u002f c");
    assert_eq!(kinds(source), [Class, Identifier, LBrace, RBrace]);
}
