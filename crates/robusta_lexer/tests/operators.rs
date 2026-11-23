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
fn tokenizes_annotation_generic_and_array_type_punctuation() {
    assert_eq!(
        kinds("@A Map<String, List<? extends Integer>>[] names"),
        [
            At, Identifier, Identifier, Lt, Identifier, Comma, Identifier, Lt, Question, Extends,
            Identifier, Gt, Gt, LBracket, RBracket, Identifier,
        ]
    );
}

#[test]
fn tokenizes_ellipsis_for_varargs() {
    assert_eq!(
        kinds("void f(String... names, double .5)"),
        [
            Void,
            Identifier,
            LParen,
            Identifier,
            Ellipsis,
            Identifier,
            Comma,
            Double,
            FloatLiteral,
            RParen,
        ]
    );
}

#[test]
fn operators_munch_maximally() {
    assert_eq!(
        kinds("a == b != c <= d >= e < f > g"),
        [
            Identifier, EqEq, Identifier, BangEq, Identifier, LtEq, Identifier, GtEq, Identifier,
            Lt, Identifier, Gt, Identifier,
        ]
    );
    assert_eq!(
        kinds("a && b & c || d | e ^ f"),
        [
            Identifier, AmpAmp, Identifier, Amp, Identifier, PipePipe, Identifier, Pipe,
            Identifier, Caret, Identifier,
        ]
    );
    assert_eq!(
        kinds("x += 1; x -= 1; x *= 2; x /= 2"),
        [
            Identifier, PlusEq, IntLiteral, Semicolon, //
            Identifier, MinusEq, IntLiteral, Semicolon, //
            Identifier, StarEq, IntLiteral, Semicolon, //
            Identifier, SlashEq, IntLiteral,
        ]
    );
    assert_eq!(
        kinds("x %= 2; x &= 1; x |= 1; x ^= 1"),
        [
            Identifier, PercentEq, IntLiteral, Semicolon, //
            Identifier, AmpEq, IntLiteral, Semicolon, //
            Identifier, PipeEq, IntLiteral, Semicolon, //
            Identifier, CaretEq, IntLiteral,
        ]
    );
    assert_eq!(
        kinds("!ok ? -a : ~b % c"),
        [
            Bang, Identifier, Question, Minus, Identifier, Colon, Tilde, Identifier, Percent,
            Identifier
        ]
    );
}

#[test]
fn left_shifts_munch_maximally_and_right_angles_stay_single() {
    assert_eq!(
        kinds("a << b <<= c < d <= e"),
        [Identifier, LtLt, Identifier, LtLtEq, Identifier, Lt, Identifier, LtEq, Identifier]
    );
    // `>`-led operators lex as single `>`s (and a final `>=`) so generics close one at a time.
    assert_eq!(
        kinds("a >> b >>> c >>= d >>>= e"),
        [
            Identifier, Gt, Gt, Identifier, Gt, Gt, Gt, Identifier, Gt, GtEq, Identifier, Gt, Gt,
            GtEq, Identifier,
        ]
    );
}

#[test]
fn increment_and_decrement_munch_maximally() {
    assert_eq!(
        kinds("++i; --j; i++; j--"),
        [
            PlusPlus, Identifier, Semicolon, //
            MinusMinus, Identifier, Semicolon, //
            Identifier, PlusPlus, Semicolon, //
            Identifier, MinusMinus,
        ]
    );
    // Maximal munch: `x+++y` is `x++ + y`, never `x + ++y` (JLS §3.2).
    assert_eq!(kinds("x+++y"), [Identifier, PlusPlus, Plus, Identifier]);
    assert_eq!(kinds("x---y"), [Identifier, MinusMinus, Minus, Identifier]);
}

#[test]
fn arrow_munches_after_minus() {
    assert_eq!(kinds("x -> y"), [Identifier, Arrow, Identifier]);
    // `-->` munches maximally: `--` then `>` (JLS §3.2), never `- ->`.
    assert_eq!(kinds("x-->y"), [Identifier, MinusMinus, Gt, Identifier]);
    assert_eq!(kinds("x->-y"), [Identifier, Arrow, Minus, Identifier]);
}

#[test]
fn colon_colon_munches_maximally() {
    assert_eq!(kinds("Foo::bar"), [Identifier, ColonColon, Identifier]);
    assert_eq!(kinds("a ? b : c"), [Identifier, Question, Identifier, Colon, Identifier]);
    // Adjacent colons munch maximally (JLS §3.2): `:::` is `::` then `:`.
    assert_eq!(kinds("a:::b"), [Identifier, ColonColon, Colon, Identifier]);
}

#[test]
fn slash_outside_a_comment_is_an_operator() {
    assert_eq!(kinds("a / b // real comment"), [Identifier, Slash, Identifier]);
}
