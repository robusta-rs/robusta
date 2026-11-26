//! Cross-cutting formatter contract regressions.

use insta::assert_snapshot;

use crate::fixtures::{
    assert_java_fixtures_covered, check_formatter_contract, format_source as fmt,
};

#[test]
fn empty_source_formats_to_nothing() {
    assert_snapshot!(fmt(""), @"");
}

/// Sources that exercise the formatter contract (a stable second pass, tokens
/// and comments preserved) without needing a golden snapshot of their own. Golden
/// tests already enforce the contract through `fmt`, so only add a source
/// here when its exact output is not worth pinning.
const FORMATTER_CONTRACT_EXTRA_SOURCES: &[&str] = &[
    "class A { void f() { submit(new Runnable() { public void run() { step(); } }); } }",
    "class A { void f() { do; while (a); } }",
    "class A { void f() { switch (x) { case 1: f(); default: } } }",
    "class A { Object f(int x) { return switch (x) { case 1 -> \"one\"; default -> null; }; } }",
    "class A { void f(Object x) { switch (x) { case String s when s.isBlank() -> f(); default -> g(); } } }",
    "class A {}\n",
    ";;\nclass A { ; int x = 1; ; }\n",
    "class A { void f() { ; return; ; } }\n",
    "package a.b; class C { int x; }",
    "class C { int[] xs = {1, 2}; int f(int[][] m) { m[0] = new int[xs[0]]; \
         return m[xs[0]][xs.length - 1]; } }",
    "class C { C(final int a) throws E { return; } int f() { return 1; } }",
    "class C { void f() { x = a ^ b | c & ~d; ok = !(a == b) != (c < d); } }",
    "class C { int f() { return (a + b) * (a - b) / (a % b); } }",
];

#[test]
fn formatter_contract() {
    assert_java_fixtures_covered(&[
        crate::compilation_units::FIXTURES,
        crate::declarations::FIXTURES,
        crate::expressions::FIXTURES,
        crate::statements::FIXTURES,
        crate::types_annotations::FIXTURES,
        crate::comments_trivia::FIXTURES,
        crate::comment_policy::FIXTURES,
    ])
    .unwrap_or_else(|err| panic!("Java fixture coverage failed: {err}"));

    for &source in FORMATTER_CONTRACT_EXTRA_SOURCES {
        check_formatter_contract(source)
            .unwrap_or_else(|err| panic!("formatter contract failed for {source:?}: {err}"));
    }
}

/// An unstable input fails the contract, and its output shows the second pass
/// so a snapshot can pin it. Fixing the input fails this test.
#[test]
fn an_unstable_input_shows_its_second_pass() {
    let source = "class A { void g() { if (a) g() /** doc */ // c\n; int y; } }";
    assert!(check_formatter_contract(source).unwrap_err().starts_with("second pass differs"));
    assert!(fmt(source).contains("---------- Not idempotent (second pass) ----------"));
}

#[test]
fn unicode_escapes_preserve_names_and_literals_but_normalize_keywords() {
    // Formatter policy: identifiers and literals keep their original
    // spelling (the identifier `\u0078` survives), while structural tokens such
    // as the keyword `class` are reprinted canonically.
    let src = r"cl\u0061ss A { int \u0078 = 1; }";
    assert_snapshot!(fmt(src), @r"
    class A {
      int \u0078 = 1;
    }
    ");
}

#[test]
fn every_literal_kind_keeps_its_unicode_escape_spelling() {
    let src = r#"class A { boolean a = \u0074rue; boolean b = \u0066alse; Object c = nul\u006c;
        int d = 1\u0030; double e = 1\u002e5; char f = '\u0041'; String g = "\u0041"; }"#;
    assert_snapshot!(fmt(src), @r#"
    class A {
      boolean a = \u0074rue;
      boolean b = \u0066alse;
      Object c = nul\u006c;
      int d = 1\u0030;
      double e = 1\u002e5;
      char f = '\u0041';
      String g = "\u0041";
    }
    "#);
}

#[test]
fn unicode_identifiers_round_trip_while_hard_type_keywords_normalize() {
    let src = r"class  Caf\u00e9 { \u0069nt 𝒂 ; \u0076oid méthode ( int 变量٢ ) { } }";
    assert_snapshot!(fmt(src), @r"
    class Caf\u00e9 {
      int 𝒂;

      void méthode(int 变量٢) {}
    }
    ");
}

#[test]
fn every_identifier_owner_preserves_its_unicode_escape_spelling() {
    for source in [
        r"package \u0070.\u0071; import java.util.L\u0069st; class \u0043<\u0054> {}",
        r"interface \u0049 { void \u006d(); }",
        r"enum \u0045 { \u0041; }",
        r"record \u0052(int \u0078) { \u0052 {} }",
        r"@interface \u0041 { int \u0076() default 1; }",
        r"@\u0041(\u0076 = 1) class C {}",
        r"class \u0043 { \u0043(int \u0078) {} void \u006d(String \u0070) {} }",
        r"class C { \u0054 \u0078; void f() { \u0054 \u0079 = \u0078; } }",
        r"class C { void f() { \u0061: while (true) { if (p) break \u0061; continue \u0061; } } }",
        r"class C { void f() { for (\u0054 \u0078 : xs) use(\u0078); } }",
        r"class C { void f() { try (\u0054 \u0072 = open()) {} catch (\u0045 \u0065) {} } }",
        r"class C { void f() { try (this.\u0072) {} } }",
        r"class C { Object f() { return \u0078 -> \u0078 + 1; } }",
        r"class C { Object f() { return (\u0054 \u0078) -> \u0078; } }",
        r"class C { void f(Object x) { if (x instanceof \u0054 \u0074) use(\u0074); } }",
        r"class C { void f() { \u006d(); y = x.\u006d().\u0066; } }",
        r"class C { Object f() { return \u0043::\u006d; } }",
        r"class C { Object f() { return \u0043.this; } }",
        r"class C { Object f() { return \u0043.super.\u006d(); } }",
        r"module \u006d { requires \u006e; exports \u0070 to \u0071; uses \u0054; }",
        r"class C { int f() { int \uD835\uDC82 = 1; return 𝒂; } }",
    ] {
        check_formatter_contract(source).unwrap_or_else(|error| panic!("{source}: {error}"));
    }
}
