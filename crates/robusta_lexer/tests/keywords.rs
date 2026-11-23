use robusta_lexer::{
    TokenKind::{self, *},
    tokenize,
};

/// Every keyword spelling the lexer recognizes, written out so a typo here
/// or in the lexer fails the test.
const KEYWORDS: [(&str, TokenKind); 71] = [
    ("package", Package),
    ("_", Underscore),
    ("import", Import),
    ("class", Class),
    ("interface", Interface),
    ("enum", Enum),
    ("boolean", Boolean),
    ("byte", Byte),
    ("char", Char),
    ("double", Double),
    ("float", Float),
    ("int", Int),
    ("long", Long),
    ("short", Short),
    ("void", Void),
    ("const", Const),
    ("goto", Goto),
    ("public", Public),
    ("protected", Protected),
    ("private", Private),
    ("static", Static),
    ("final", Final),
    ("abstract", Abstract),
    ("native", Native),
    ("strictfp", Strictfp),
    ("transient", Transient),
    ("volatile", Volatile),
    ("extends", Extends),
    ("implements", Implements),
    ("new", New),
    ("super", Super),
    ("instanceof", Instanceof),
    ("return", Return),
    ("throw", Throw),
    ("assert", Assert),
    ("try", Try),
    ("catch", Catch),
    ("finally", Finally),
    ("synchronized", Synchronized),
    ("if", If),
    ("else", Else),
    ("switch", Switch),
    ("case", Case),
    ("default", Default),
    ("while", While),
    ("do", Do),
    ("for", For),
    ("break", Break),
    ("continue", Continue),
    ("throws", Throws),
    ("this", This),
    ("true", True),
    ("false", False),
    ("null", Null),
    ("exports", Exports),
    ("module", Module),
    ("non", Non),
    ("open", Open),
    ("opens", Opens),
    ("permits", Permits),
    ("provides", Provides),
    ("record", Record),
    ("requires", Requires),
    ("sealed", Sealed),
    ("to", To),
    ("transitive", Transitive),
    ("uses", Uses),
    ("var", Var),
    ("when", When),
    ("with", With),
    ("yield", Yield),
];

/// Lex and return the token kinds (EOF dropped).
fn kinds(source: &str) -> Vec<TokenKind> {
    let mut kinds: Vec<_> = tokenize(source).tokens.iter().map(|token| token.kind).collect();
    assert_eq!(kinds.pop(), Some(Eof));
    kinds
}

#[test]
fn keywords_resolve_to_their_own_kinds() {
    for (word, kind) in KEYWORDS {
        assert_eq!(kinds(word), [kind], "{word}");
    }
}

#[test]
fn escaped_keywords_resolve_and_keep_their_spelling() {
    for (source, kind) in [(r"cl\u0061ss", Class), (r"rec\u006frd", Record), (r"\u0069nt", Int)] {
        let result = tokenize(source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        assert_eq!(result.tokens[0].kind, kind, "{source}");
        assert_eq!(result.tokens[0].span.source_text(source), source);
    }
    let source = r"cl\u0061ss A {}";
    assert_eq!(tokenize(source).tokens[1].span.source_text(source), "A");
}

#[test]
fn near_misses_stay_identifiers() {
    assert_eq!(kinds("inté int\u{0301}"), [Identifier, Identifier]);
}

#[test]
fn tokenizes_package_and_class() {
    let source =
        "package a.b;\npublic final class Foo extends Bar implements I { private int x = 5; }";
    assert_eq!(
        kinds(source),
        [
            Package, Identifier, Dot, Identifier, Semicolon, //
            Public, Final, Class, Identifier, Extends, Identifier, Implements, Identifier,
            LBrace, //
            Private, Int, Identifier, Assign, IntLiteral, Semicolon, //
            RBrace,
        ]
    );
}
