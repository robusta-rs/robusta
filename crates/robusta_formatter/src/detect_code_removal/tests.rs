use super::*;
use crate::{DescriptionWithDot, JavadocOptions, Options, format};

/// Formats `input` in both modes, checks each result, and returns the default output.
fn format_checked(input: &str) -> String {
    let check = |javadoc| {
        let options = Options { javadoc, ..Options::default() };
        let output = format(input, &options).unwrap_or_else(|error| panic!("{input}: {error:?}"));
        detect_code_removal(input, &output).unwrap_or_else(|error| panic!("{input}: {error}"));
        output
    };
    check(Some(JavadocOptions::default()));
    check(None)
}

#[test]
fn token_mismatch_reports_first_kind_difference() {
    let before = SourceSnapshot::new("class A { int x = 1; }");
    let after = SourceSnapshot::new("class A { int x == 1; }");

    assert_eq!(
        before.compare_tokens(&after).unwrap_err(),
        "token kind changed at index 5: input Assign, output EqEq"
    );
}

#[test]
fn token_mismatch_reports_first_spelling_difference() {
    let before = SourceSnapshot::new("class A { int x = 1; }");
    let after = SourceSnapshot::new("class A { int y = 1; }");

    assert_eq!(
        before.compare_tokens(&after).unwrap_err(),
        "token spelling changed at index 4: Identifier input Some(\"x\"), output Some(\"y\")"
    );
}

#[test]
fn a_text_block_may_move_its_lines_but_not_change_them() {
    let input = "class A { String s = \"\"\"\n        a\n          b\n        \"\"\"; }";
    let moved = "class A { String s = \"\"\"\n  a\n    b\n  \"\"\"; }";
    let changed = "class A { String s = \"\"\"\n  a\n  b\n  \"\"\"; }";

    assert!(detect_code_removal(input, moved).is_ok());
    assert!(detect_code_removal(input, changed).is_err());
}

#[test]
fn a_dropped_token_is_caught() {
    assert_eq!(
        detect_code_removal("class A { int x; int y; }", "class A { int x; }").unwrap_err(),
        "token kind changed at index 6: input Int, output RBrace"
    );
}

#[test]
fn dropping_a_repeated_import_is_caught() {
    for import in [
        "import a.B;\n",
        "import a.b.*;\n",
        "import static a.B.c;\n",
        "import static a.B.*;\n",
        "import module java.base;\n",
    ] {
        let input = import.repeat(2);
        assert!(detect_code_removal(&input, import).is_err(), "{import}");
    }
}

#[test]
fn import_regions_are_normalized_for_formatter_sorting() {
    let before = SourceSnapshot::new(
        r"import z.Z;
import static z.Tools.zed;
import a.A;
import a.A;

class C {}
",
    );
    let after = SourceSnapshot::new(
        r"import static z.Tools.zed;

import a.A;
import a.A;
import z.Z;

class C {}
",
    );

    before.compare_tokens(&after).unwrap();
}

#[test]
fn a_trailing_comma_is_not_a_token_change() {
    detect_code_removal("enum E { A }", "enum E { A, }").unwrap();
    detect_code_removal("enum E { A; }", "enum E { A,; }").unwrap();
    detect_code_removal("class A { int[] a = {1, 2,}; }", "class A { int[] a = {1, 2}; }").unwrap();
    assert!(
        detect_code_removal(
            "class A { void f(int a, int b) {} }",
            "class A { void f(int a int b) {} }"
        )
        .is_err()
    );
}

#[test]
fn a_dropped_line_comment_is_caught() {
    assert_eq!(
        detect_code_removal("class A {\n  // gone\n  int x;\n}\n", "class A {\n  int x;\n}\n")
            .unwrap_err(),
        "LineComment mismatch: `// gone` not found"
    );
}

#[test]
fn a_changed_line_comment_is_caught() {
    assert_eq!(
        detect_code_removal(
            "class A {\n  // old\n  int x;\n}\n",
            "class A {\n  // new\n  int x;\n}\n"
        )
        .unwrap_err(),
        "LineComment mismatch: `// old` not found"
    );
    for output in [
        "class A { int y = 1; // first // changed\n}",
        "class A { int y = 1; // changed // second\n}",
    ] {
        assert!(
            detect_code_removal("class A { int y = 1 // first\n; // second\n}", output).is_err()
        );
    }
}

#[test]
fn a_dropped_block_comment_is_caught() {
    for (input, output, message) in [
        ("class A { /* gone */ int x; }", "class A { int x; }", "1 -> 0"),
        ("class A { /** gone */ }", "class A { }", "1 -> 0"),
        ("class A { /** a */ /** gone */ int x; }", "class A { /** a */ int x; }", "2 -> 1"),
    ] {
        assert_eq!(
            detect_code_removal(input, output).unwrap_err(),
            format!("BlockComment count mismatch: {message}"),
            "{input}"
        );
    }
}

#[test]
fn a_documentation_comment_that_documents_another_declaration_is_caught() {
    for (input, output) in [
        // It documents the next declaration instead.
        ("class A { /** a */ int x; int y; }", "class A { int x; /** a */ int y; }"),
        ("class A { /// a\nint x; int y; }", "class A { int x; /// a\nint y; }"),
        // It stops documenting one.
        ("enum E { A, /** a */\nB }", "enum E { A /** a */,\nB }"),
        ("class A { /** a */ int x; }", "class A { int /** a */ x; }"),
        ("class A { /// a\nint x; }", "class A { int /// a\nx; }"),
        ("/// a\npackage p;", "package /// a\np;"),
        // It starts to come before one.
        ("class A { int x /** a */; int y; }", "class A { int x; /** a */ int y; }"),
        ("class A { int x\n/** a */; int y; }", "class A { int x;\n/** a */\nint y; }"),
        (
            "class A { int x /** a */; /** b */ int y; }",
            "class A { int x; /** a */ /** b */ int y; }",
        ),
        ("class A { int x; /// a\nint y; }", "class A { int x;\n/// a\n\n/// b\nint y; }"),
        // A blank line decides which `///` comment documents the declaration.
        ("/// a\n\n/// b\nclass A {}", "/// a\n/// b\nclass A {}"),
        ("/// a\n/// b\nclass A {}", "/// a\n\n/// b\nclass A {}"),
        (r"/// a\u000a\u000a/// b\u000aclass A {}", "/// a\n/// b\nclass A {}"),
        // A `/**/` comment supersedes the comment before it.
        ("class A { /** a */ int x /**/; }", "class A { /** a */ /**/ int x; }"),
    ] {
        let error = detect_code_removal(input, output).unwrap_err();
        assert!(error.starts_with("documentation comment "), "{input:?} -> {output:?}: {error}");
    }
}

/// Javadoc formatting leaves a documentation comment that documents nothing as
/// it is, so it compares as a line comment, wherever it moves.
#[test]
fn a_markdown_comment_before_no_declaration_compares_by_text() {
    for (input, output) in [
        ("class A { int x /// a\n\n/// b\n= 1; }", "class A { int x = /// a\n/// b\n1; }"),
        ("import b.B; /// x\n\n/// y\nimport a.A;\n", "/// y\nimport a.A;\n/// x\nimport b.B;\n"),
        ("class A { int x /// a\n; }", "class A { int x; /// a\n}"),
        ("class A { /// a\n}", "class A {\n  /// a\n}\n"),
    ] {
        detect_code_removal(input, output)
            .unwrap_or_else(|error| panic!("{input:?} -> {output:?}: {error}"));
    }
    for (input, output, missing) in [
        // It is dropped, or changed.
        ("class A { /// a\n}", "class A { }", "/// a"),
        ("class A { /// a\n}", "class A { /// b\n}", "/// a"),
        // It joins the run before the next declaration, or an ordinary comment becomes one.
        (
            "class A { int x /// a\n; /// b\nint y; }",
            "class A { int x; /// a\n/// b\nint y; }",
            "/// a",
        ),
        ("class A { int x /// a\n; int y; }", "class A { int x; /// a\nint y; }", "/// a"),
        (
            "class A { int x /// a\n; /** b */ int y; }",
            "class A { int x; /** b */ /// a\nint y; }",
            "/// a",
        ),
        ("class A { // a\nint x; }", "class A { /// a\nint x; }", "// a"),
    ] {
        assert_eq!(
            detect_code_removal(input, output).unwrap_err(),
            format!("LineComment mismatch: `{missing}` not found"),
            "{input:?} -> {output:?}"
        );
    }
}

/// A line comment merging into a documentation comment would change its text.
#[test]
fn a_line_comment_cannot_join_a_documentation_comment_before_a_declaration() {
    assert_eq!(
        detect_code_removal(
            "class A { int x; /// a\n// b\nint y; }",
            "class A { int x; /// a // b\nint y; }",
        )
        .unwrap_err(),
        "LineComment mismatch: `// b` not found"
    );
}

#[test]
fn a_comment_that_only_moved_passes() {
    for (input, output) in [
        ("class A { int x /* c */; }", "class A { int x; /* c */ }"),
        ("class A { // c\nint x; }", "class A { int x; // c\n}"),
        ("@S(\"x\") // c\nclass A {}", "@S(\"x\")\nclass // c\nA {}"),
        ("import z.Z;\nimport a.A; // c\n", "import a.A;\nimport z.Z; // c\n"),
    ] {
        detect_code_removal(input, output)
            .unwrap_or_else(|error| panic!("{input:?} -> {output:?}: {error}"));
    }
}

#[test]
fn line_comments_merged_onto_one_line_pass() {
    format_checked("class A { int y = 1 // first\n; // second\n}");
    detect_code_removal(
        "class A { int y = 1 // first\n; // second\n}",
        "class A { int y = 1; // first // second\n}",
    )
    .unwrap();
    detect_code_removal(
        "class A { int y = 1 // first  \n; // second\n}",
        "class A { int y = 1; // first // second\n}",
    )
    .unwrap();
    // Including a `///` comment that documents nothing.
    detect_code_removal(
        "class A { int y /// first\n= // second\n1; }",
        "class A { int y = /// first // second\n1; }",
    )
    .unwrap();
    // Which is then no documentation comment.
    format_checked("class A { void f() { for (int i = 0 // x\n; /// d\ni < 1;) {} } }");
}

#[test]
fn a_trimmed_line_comment_passes() {
    detect_code_removal("class A { // c  \t\n}", "class A { // c\n}").unwrap();
    assert_eq!(format_checked("class A { // c  \t\n}"), "class A { // c\n}\n");
}

/// Comments the formatter moves where the placement rules put them, whatever
/// they cross. A build's guarded CLI would abort on these if they were rejected.
#[test]
fn comments_the_formatter_moves_pass() {
    for input in [
        // A block comment before a binary operator.
        "class A { void f() { int x = 1\n/* a\n b */ + 2; } }",
        // A block comment run hoisted before a bound's `&`.
        "class A<K extends B // one\n/* a\n b */\n& /* c\n d */ C, V> {}",
        // Comments around an enum's `,` and `;`.
        "enum E { A // x\n, /* b */ ; void f() {} }",
        "enum E { A // x\n, // y\n; /* t */ }",
        // A comment after an unbraced body's `;`.
        "class A { void f() { if (c) foo()\n// own\n;\n// second\nelse bar(); } }",
        "class A { void f() { do foo()\n// own\n;\n// second\nwhile (c); } }",
        // A `///` comment after a line comment moved behind a `;`.
        "module m { requires a /// D\n; /// d\n}",
        "class A { void f() { switch (x) { case 1 -> foo() // x\n; /// d\ndefault -> {} } } }",
        "class A { void f() { if (c) foo() // x\n; /// d\nelse bar(); } }",
        "class A { void f() { for (int i = 0 // x\n; /// d\ni < 1;) {} } }",
    ] {
        format_checked(input);
    }
}

#[test]
fn a_javadoc_rewrite_passes() {
    let javadoc = JavadocOptions {
        capitalize_descriptions: true,
        description_with_dot: DescriptionWithDot::Summary,
        ..JavadocOptions::default()
    };
    let options = Options { javadoc: Some(javadoc), ..Options::default() };
    for (input, rewritten) in [
        ("class A {\nint a; /// first part\nint b;\n}\n", "/// First part."),
        ("class A {\nvoid f() {} /// a\nvoid g() {}\n}\n", "/// A."),
        ("/** first part */\nclass A {}\n", "/** First part. */"),
    ] {
        let output = format(input, &options).unwrap();
        assert!(output.contains(rewritten), "{output}");
        detect_code_removal(input, &output).unwrap();
    }
}
