//! Comment-placement policy encoded as paired input and expected-output files.
//!
//! The output checks ask only that nothing is lost; these fixtures decide
//! whether a comment's placement is correct. They replace the byte-exact
//! external formatter oracle Robusta deliberately does not use.

use insta::assert_snapshot;

use crate::fixtures::{
    assert_java_fixture, check_contract_at_widths_and_modes, format_source as fmt, format_stable,
};

pub const FIXTURES: &[&str] = &[
    "comments-and-trivia/annotation-array-comment-policy",
    "comments-and-trivia/declaration-generics-comment-policy",
    "comments-and-trivia/separator-comment-policy",
    "comments-and-trivia/terminator-comment-policy",
    "comments-and-trivia/type-position-comment-policy",
];

#[test]
fn multiline_comments_lead_the_bound_after_its_ampersand() {
    for source in [
        "class A<K extends B // line\n/* first\n second */ & C, V> {}",
        "class A { <K extends B // line\n/* first\n second */ & C> void m() {} }",
        "class A { Object x = (B // line\n/* first\\u000a second */ & C) value; }",
    ] {
        let output = format_stable(source);
        assert!(output.contains("& /* first"), "{output}");
        check_contract_at_widths_and_modes(source);
    }
}

#[test]
fn a_multiline_block_can_share_the_binary_operators_line() {
    for newline in ["\n", "\\u000a"] {
        let source = format!("class A {{ int value = 1\n/* a{newline} b */ + 2; }}");
        let output = format_stable(&source);
        assert!(output.contains(&format!("+ /* a{newline} b */ 2;")), "{output}");
        check_contract_at_widths_and_modes(&source);
    }
}

#[test]
fn annotation_array_comments_keep_their_element_and_declaration_boundaries() {
    const NAME: &str = "comments-and-trivia/annotation-array-comment-policy";
    assert_java_fixture(
        NAME,
        include_str!("../fixtures/java/comments-and-trivia/annotation-array-comment-policy.java"),
        include_str!(
            "../fixtures/java/comments-and-trivia/annotation-array-comment-policy.expected.java"
        ),
    );
}

#[test]
fn generic_declaration_comments_keep_their_constructor_and_method_boundaries() {
    const NAME: &str = "comments-and-trivia/declaration-generics-comment-policy";
    assert_java_fixture(
        NAME,
        include_str!(
            "../fixtures/java/comments-and-trivia/declaration-generics-comment-policy.java"
        ),
        include_str!(
            "../fixtures/java/comments-and-trivia/declaration-generics-comment-policy.expected.java"
        ),
    );
}

/// Inline comments stay before the separator they precede: a comma, an
/// enhanced-`for` colon, or an `assert` colon. Each comment keeps documenting
/// the element before that separator.
#[test]
fn separator_comments_stay_on_the_element_they_follow() {
    const NAME: &str = "comments-and-trivia/separator-comment-policy";
    assert_java_fixture(
        NAME,
        include_str!("../fixtures/java/comments-and-trivia/separator-comment-policy.java"),
        include_str!("../fixtures/java/comments-and-trivia/separator-comment-policy.expected.java"),
    );
}

/// A comment sharing a line with the semicolon that ends a construct documents
/// the finished construct, so it prints behind the terminator. A comment on its
/// own line before the same semicolon describes what follows and leads the next
/// construct instead, and a `for` header's semicolons are separators that keep
/// their comments in place.
#[test]
fn terminator_comments_follow_the_semicolon_that_ends_their_construct() {
    const NAME: &str = "comments-and-trivia/terminator-comment-policy";
    assert_java_fixture(
        NAME,
        include_str!("../fixtures/java/comments-and-trivia/terminator-comment-policy.java"),
        include_str!(
            "../fixtures/java/comments-and-trivia/terminator-comment-policy.expected.java"
        ),
    );
}

#[test]
fn line_comments_merge_after_their_terminating_semicolon() {
    use crate::fixtures::format_source_with_options;
    use robusta_formatter::{JavadocOptions, Options};

    for (input, expected) in [
        (
            "class A { void f() { call() // c\n; // keep\n} }",
            "class A {\n  void f() {\n    call(); // c // keep\n  }\n}\n",
        ),
        (
            "class A { void f() { int value = 42 // c\n; // keep\n} }",
            "class A {\n  void f() {\n    int value = 42; // c // keep\n  }\n}\n",
        ),
        (
            "class A { int value = 42 // c\n; // keep\n}",
            "class A {\n  int value = 42; // c // keep\n}\n",
        ),
        // Joining the comments changes the directive's rule list.
        (
            "class A { long n = 2l // robusta-disable-line\n; // why\n}",
            "class A {\n  long n = 2l; // robusta-disable-line // why\n}\n",
        ),
        ("import a.A // c\n; // keep\nclass B {}", "import a.A; // c // keep\n\nclass B {}\n"),
        ("package p // c\n; // keep\nclass A {}", "package p; // c // keep\n\nclass A {}\n"),
        ("enum E { A // c\n; // keep\n}", "enum E {\n  A,; // c // keep\n}\n"),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(input, &options), expected);
        }
    }
}

#[test]
fn enum_semicolon_comments_keep_their_order_behind_a_moved_line_comment() {
    use crate::fixtures::format_source_with_options;
    use robusta_formatter::{JavadocOptions, Options};

    for (input, expected) in [
        (
            "enum E { ONLY, // x\n; /* after */ /* more */ }",
            "enum E {\n  ONLY,; // x\n  /* after */ /* more */\n}\n",
        ),
        (
            "enum E { ONLY // x\n; /* after */ /* more */ }",
            "enum E {\n  ONLY,; // x\n  /* after */ /* more */\n}\n",
        ),
        ("enum E { ONLY, // x\n; /// d\n}", "enum E {\n  ONLY,; // x\n  /// d\n}\n"),
        (
            "enum E { ONLY, // x\n; /// d\nvoid f() {} }",
            "enum E {\n  ONLY,; // x\n\n  /// d\n  void f() {}\n}\n",
        ),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(input, &options), expected);
        }
    }
}

#[test]
fn enum_semicolon_comments_before_a_member_trail_it_as_a_field_terminator_keeps_them() {
    use crate::fixtures::format_source_with_options;
    use robusta_formatter::{JavadocOptions, Options};

    for (input, expected) in [
        (
            "enum E { A // c\n; // keep\nvoid f() {} }",
            "enum E {\n  A,; // c // keep\n\n  void f() {}\n}\n",
        ),
        (
            "class C { int x = 1 // c\n; // keep\nvoid f() {} }",
            "class C {\n  int x = 1; // c // keep\n\n  void f() {}\n}\n",
        ),
        (
            "enum E { A, /* c */\n; /* after */\nvoid f() {} }",
            "enum E {\n  A,; /* c */ /* after */\n\n  void f() {}\n}\n",
        ),
        (
            "class C { int x = 1 /* c */\n; /* after */\nvoid f() {} }",
            "class C {\n  int x = 1; /* c */ /* after */\n\n  void f() {}\n}\n",
        ),
        ("enum E { A,\n; // keep\nvoid f() {} }", "enum E {\n  A,; // keep\n\n  void f() {}\n}\n"),
        (
            "enum E { A // c\n, ; // keep\nvoid f() {} }",
            "enum E {\n  A,; // c // keep\n\n  void f() {}\n}\n",
        ),
        // A `///` on the line would become part of the moved comment.
        (
            "enum E { A // c\n, ; /// d\nvoid f() {} }",
            "enum E {\n  A,; // c\n\n  /// d\n  void f() {}\n}\n",
        ),
        // A comment on its own line before the `;` leads the member, with what follows it.
        (
            "enum E { A\n// own\n; // keep\nvoid f() {} }",
            "enum E {\n  A,;\n\n  // own\n  // keep\n  void f() {}\n}\n",
        ),
        // An escaped `;` ends the constant's claim like any other.
        ("enum E { A\\u003b // y\n}", "enum E {\n  A,; // y\n}\n"),
        (
            "enum E { A // c\n\\u002c \\u003b /// d\nvoid f() {} }",
            "enum E {\n  A,; // c\n\n  /// d\n  void f() {}\n}\n",
        ),
        // With no member, it stays in the body, whatever follows the enum.
        (
            "class C { enum E { A\n// own\n; /* t */ } void h() {} }",
            "class C {\n  enum E {\n    A,;\n    // own\n    /* t */\n  }\n\n  void h() {}\n}\n",
        ),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(input, &options), expected);
        }
    }
}

#[test]
fn a_line_comment_inside_a_member_may_precede_a_multibyte_token() {
    use crate::fixtures::format_source_with_options;
    use robusta_formatter::{JavadocOptions, Options};

    for (input, expected) in [
        (
            "class B { void f( // c\né x) {} void g() {} }",
            "class B {\n  void f(\n    // c\n    é x\n  ) {}\n\n  void g() {}\n}\n",
        ),
        (
            "class B { void f(int a // c\n, é x) {} void g() {} }",
            "class B {\n  void f(\n    int a, // c\n    é x\n  ) {}\n\n  void g() {}\n}\n",
        ),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(input, &options), expected);
        }
    }
}

#[test]
fn documentation_comments_stay_before_a_terminator_a_declaration_could_follow() {
    use crate::fixtures::format_source_with_options;
    use robusta_formatter::{JavadocOptions, Options};

    for (input, expected) in [
        ("class A { int x /** doc */;\nint y; }", "class A {\n  int x /** doc */;\n  int y;\n}\n"),
        ("class A { int x /** */;\nint y; }", "class A {\n  int x /** */;\n  int y;\n}\n"),
        (
            "class A { int x /*** @deprecated Old */; int y; }",
            "class A {\n  int x /*** @deprecated Old */;\n  int y;\n}\n",
        ),
        (
            "class A { int x \\u002f** @deprecated Old */; int y; }",
            "class A {\n  int x \\u002f** @deprecated Old */;\n  int y;\n}\n",
        ),
        ("class A { int x /// doc\n; int y; }", "class A {\n  int x /// doc\n  ;\n  int y;\n}\n"),
        (
            "class A { int x\n/** Doc. */;\nint y; }",
            "class A {\n  int x\n  /** Doc. */;\n  int y;\n}\n",
        ),
        (
            "class A { int x /* c */ /** doc */ /* d */;\n}",
            "class A {\n  int x /* c */ /** doc */; /* d */\n}\n",
        ),
        ("package p /** doc */;\nclass Q {}", "package p /** doc */;\n\nclass Q {}\n"),
        ("package p\n/** Doc. */;\nclass Q {}", "package p\n/** Doc. */;\n\nclass Q {}\n"),
        ("import a.A /** semi */;\nclass Q {}", "import a.A /** semi */;\n\nclass Q {}\n"),
        ("import a.A\n/** Doc. */;\nclass Q {}", "import a.A\n/** Doc. */;\n\nclass Q {}\n"),
        ("interface I { void m() /** doc */; }", "interface I {\n  void m() /** doc */;\n}\n"),
        (
            "class A { void f() { g() /** doc */; } }",
            "class A {\n  void f() {\n    g() /** doc */;\n  }\n}\n",
        ),
        (
            "class A { void f() { while (a) continue /** doc */; } }",
            "class A {\n  void f() {\n    while (a) continue /** doc */;\n  }\n}\n",
        ),
        (
            "class A { void f() { if (a) g() /** doc */ // c\n; int y; } }",
            "class A {\n  void f() {\n    if (a) g() /** doc */; // c\n    int y;\n  }\n}\n",
        ),
        (
            "class A { void f() { for (;;) g() /// doc\n; } }",
            "class A {\n  void f() {\n    for (;;)\n      g() /// doc\n      ;\n  }\n}\n",
        ),
        (
            "enum E { A, /** Stray. */\n; void m() {} }",
            "enum E {\n  A, /** Stray. */;\n\n  void m() {}\n}\n",
        ),
        ("enum E { A,\n/**/;\nE() {} }", "enum E {\n  A,\n  /**/;\n\n  E() {}\n}\n"),
        ("enum E { A /// e\n, B }", "enum E {\n  A /// e\n  ,\n  B,\n}\n"),
        // After the `,`, it documents the next constant, so the `,` stays first.
        ("enum E { A(1), /// b\nB }", "enum E {\n  A(1), /// b\n  B,\n}\n"),
        ("enum E { A\n/** Doc. */, B }", "enum E {\n  A\n  /** Doc. */,\n  B,\n}\n"),
        (
            "enum E { A /* a */ /** b */ // c\n, B, /* d */ /** e */\nC }",
            "enum E {\n  A /* a */ /** b */, // c\n  B,\n  /* d */ /** e */\n  C,\n}\n",
        ),
        (
            "class A { int value = 42 /// c\n; // keep\n}",
            "class A {\n  int value = 42 /// c\n  ; // keep\n}\n",
        ),
        (
            "class A { int value = 42 /// c\n; /* t\n u */ // keep\n}",
            "class A {\n  int value = 42 /// c\n  ; /* t\n u */ // keep\n}\n",
        ),
        // Joining the moved line comment would make the documentation part of it.
        (
            "class A { int value = 42 // c\n; /// keep\nint next; }",
            "class A {\n  int value = 42; // c\n  /// keep\n  int next;\n}\n",
        ),
        // No declaration follows a module directive or a switch rule.
        ("module m { requires a /** doc */; }", "module m {\n  requires a; /** doc */\n}\n"),
        (
            "class A { int f() { return switch (x) { case 1 -> g() /** doc */; default -> 0; }; } }",
            "class A {\n  int f() {\n    return switch (x) {\n      case 1 -> g(); /** doc */\n      default -> 0;\n    };\n  }\n}\n",
        ),
        ("class A { int x /* c */;\nint y; }", "class A {\n  int x; /* c */\n  int y;\n}\n"),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(input, &options), expected, "{input:?}");
        }
    }
}

#[test]
fn a_documentation_comment_after_a_moved_line_comment_leads_the_next_member() {
    use crate::fixtures::format_source_with_options;
    use robusta_formatter::{JavadocOptions, Options};

    for (input, expected) in [
        (
            "class A { int x = 1 // x\n; /// d\nvoid f() {} }",
            "class A {\n  int x = 1; // x\n\n  /// d\n  void f() {}\n}\n",
        ),
        (
            "class A { int x = 1 // x\n; /** d */\nvoid f() {} }",
            "class A {\n  int x = 1; // x\n\n  /** d */\n  void f() {}\n}\n",
        ),
        (
            "abstract class A { abstract void g() // x\n; /// d\nclass B {} }",
            "abstract class A {\n  abstract void g(); // x\n\n  /// d\n  class B {}\n}\n",
        ),
        (
            "@interface A { int a() // x\n; /// d\nint b(); }",
            "@interface A {\n  int a(); // x\n\n  /// d\n  int b();\n}\n",
        ),
        ("package p // x\n; /// d\nclass A {}", "package p; // x\n\n/// d\nclass A {}\n"),
        (
            "class A { void g() { foo() // x\n; /// d\n\nbar(); } }",
            "class A {\n  void g() {\n    foo(); // x\n    /// d\n\n    bar();\n  }\n}\n",
        ),
        (
            "class A { int x = 1 // x\n\\u003b /// d\nvoid f() {} }",
            "class A {\n  int x = 1; // x\n\n  /// d\n  void f() {}\n}\n",
        ),
        // No line comment moves behind a `}`.
        (
            "class A { void f() { g(); // x\n} /// d\nvoid h() {} }",
            "class A {\n  void f() {\n    g(); // x\n  } /// d\n\n  void h() {}\n}\n",
        ),
        // With no member to lead, it starts a line of its own.
        ("class A { int x = 1 // x\n; /// d\n}", "class A {\n  int x = 1; // x\n  /// d\n}\n"),
        // The comments after it on later lines stay inside the body.
        (
            "class A { int x = 1 // x\n; /// d\n// e\n}",
            "class A {\n  int x = 1; // x\n  /// d\n  // e\n}\n",
        ),
        (
            "class A { void g() { foo() // x\n; /** d */\n\n// e\n} }",
            "class A {\n  void g() {\n    foo(); // x\n    /** d */\n\n    // e\n  }\n}\n",
        ),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(input, &options), expected, "{input:?}");
        }
    }
}

#[test]
fn a_documentation_comment_after_a_moved_line_comment_goes_to_what_follows_a_body() {
    use crate::fixtures::format_source_with_options;
    use robusta_formatter::{JavadocOptions, Options};

    for (input, expected) in [
        // The next statement leads with it, as if written on a line of its own.
        ("if (c) foo() // x\n; /// d\n\nbar();", "if (c) foo(); // x\n/// d\n\nbar();\n"),
        ("if (c) foo() // x\n; /** d */\n\nbar();", "if (c) foo(); // x\n/** d */\n\nbar();\n"),
        (
            "if (c) a(); else foo() // x\n; /// d\n\nbar();",
            "if (c) a();\nelse foo(); // x\n/// d\n\nbar();\n",
        ),
        ("while (c) foo() // x\n; /// d\n\nbar();", "while (c) foo(); // x\n/// d\n\nbar();\n"),
        ("for (;;) foo() // x\n; /// d\n\nbar();", "for (;;) foo(); // x\n/// d\n\nbar();\n"),
        (
            "for (int i : a) foo() // x\n; /// d\n\nbar();",
            "for (int i : a) foo(); // x\n/// d\n\nbar();\n",
        ),
        ("l: foo() // x\n; /// d\n\nbar();", "l:\nfoo(); // x\n/// d\n\nbar();\n"),
        (
            "l: while (c) if (e) foo() // x\n; /// d\n\nbar();",
            "l:\nwhile (c) if (e) foo(); // x\n/// d\n\nbar();\n",
        ),
        (
            r"if (c) foo() // x
\u003b /// d

bar();",
            "if (c) foo(); // x\n/// d\n\nbar();\n",
        ),
        // No declaration can follow the semicolon before a continuation keyword.
        ("do foo() // x\n; /// d\nwhile (c);", "do\n  foo(); // x /// d\nwhile (c);\n"),
        ("do foo() // x\n; /** d */\nwhile (c);", "do\n  foo(); // x\n/** d */\nwhile (c);\n"),
        ("if (c) foo() // x\n; /// d\nelse bar();", "if (c) foo(); // x /// d\nelse bar();\n"),
        ("if (c) foo() // x\n; /** d */ else bar();", "if (c) foo(); // x\n/** d */ else bar();\n"),
        (
            "if (c) foo() // x\n; /* c */ /// d\n\nelse bar();",
            "if (c) foo(); // x\n/* c */ /// d\n\nelse bar();\n",
        ),
        (
            "if (c) while (a) foo(1l) /* note */ // robusta-disable-line\n; /// d\nelse bar();",
            "if (c) while (a) foo(1l); /* note */ // robusta-disable-line\n/// d\nelse bar();\n",
        ),
        (
            "do foo() // robusta-disable-next-line -- reason\n; /// d\nwhile (c);",
            "do\n  foo(); // robusta-disable-next-line -- reason\n/// d\nwhile (c);\n",
        ),
        // The next group or rule leads with it, at the label's indent.
        (
            "switch (x) { case 1: foo() // x\n; /// d\ncase 2: bar(); }",
            "switch (x) {\n  case 1:\n    foo(); // x\n  /// d\n  case 2:\n    bar();\n}\n",
        ),
        (
            "switch (x) { case 1: if (c) foo() // x\n; /// d\ncase 2: bar(); }",
            "switch (x) {\n  case 1:\n    if (c) foo(); // x\n  /// d\n  case 2:\n    bar();\n}\n",
        ),
        (
            "switch (x) { case 1 -> foo() // x\n; /// d\n\ndefault -> {} }",
            "switch (x) {\n  case 1 -> foo(); // x /// d\n\n  default -> {}\n}\n",
        ),
        (
            "switch (x) { case 1 -> throw e() // x\n; /// d\n\ndefault -> {} }",
            "switch (x) {\n  case 1 -> throw e(); // x /// d\n\n  default -> {}\n}\n",
        ),
        (
            "switch (x) { case 1 -> foo() // x\n; /// d  \n/// more\ndefault -> {} }",
            "switch (x) {\n  case 1 -> foo(); // x\n  /// d  \n  /// more\n  default -> {}\n}\n",
        ),
        // A `///` comment moves behind this `;` too, as no declaration can follow it.
        (
            "switch (x) { case 1 -> foo() /// D\n; /** d */\ndefault -> {} }",
            "switch (x) {\n  case 1 -> foo(); /// D\n  /** d */\n  default -> {}\n}\n",
        ),
        // With nothing to lead, it follows the entry that ends the body.
        ("if (c) foo() // x\n; /// d\n", "if (c) foo(); // x\n/// d\n"),
        ("foo() // x\n; /* c */ /// d\n", "foo(); // x\n/* c */ /// d\n"),
        ("l: foo() // x\n; /// d\n", "l:\nfoo(); // x\n/// d\n"),
        (
            "switch (x) { case 1: foo() // x\n; /// d\n}",
            "switch (x) {\n  case 1:\n    foo(); // x\n  /// d\n}\n",
        ),
        (
            "switch (x) { case 1: foo() // x\n; /** d */ /* e */\n}",
            "switch (x) {\n  case 1:\n    foo(); // x\n  /** d */ /* e */\n}\n",
        ),
        (
            "switch (x) { default -> foo() // x\n; /// d\n}",
            "switch (x) {\n  default -> foo(); // x /// d\n}\n",
        ),
        // A line comment that leads the `;` stays before it, so nothing joins it.
        ("while (c) // x\n; /// d\nbar();", "while (c) // x\n; /// d\nbar();\n"),
    ] {
        let input = format!("class A {{ void g() {{ {input} }} }}");
        let mut output = String::from("class A {\n  void g() {\n");
        for line in expected.lines() {
            if !line.is_empty() {
                output.push_str("    ");
                output.push_str(line);
            }
            output.push('\n');
        }
        output.push_str("  }\n}\n");
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(&input, &options), output, "{input:?}");
        }
    }

    for (input, expected) in [
        (
            "module m { requires a /// x\n; /// d\n\nrequires b; }",
            "module m {\n  requires a; /// x /// d\n\n  requires b;\n}\n",
        ),
        (
            "module m { requires a /// x\n; /// d\n// own\n}",
            "module m {\n  requires a; /// x /// d\n  // own\n}\n",
        ),
        ("module m { requires a // x\n; /// d\n}", "module m {\n  requires a; // x /// d\n}\n"),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(input, &options), expected, "{input:?}");
        }
    }
}

#[test]
fn markdown_comments_cross_semicolons_before_continuation_keywords() {
    use crate::fixtures::{format_stable_with_options, line_width};
    use robusta_formatter::{JavadocOptions, Options};

    for (statement, expected) in [
        (
            "if (c) foo() /// first\n; /// second\nelse bar();",
            "    if (c) foo(); /// first /// second\n    else bar();\n",
        ),
        (
            "do foo() /// first\n; /// second\nwhile (c);",
            "    do\n      foo(); /// first /// second\n    while (c);\n",
        ),
        (
            "if (c) again: while (d) foo() /// first\n\\u003b /// second\nelse bar();",
            "    if (c)\n      again:\n      while (d) foo(); /// first /// second\n    else bar();\n",
        ),
        (
            "do while (d) foo() /// first\n; /* second */\nwhile (c);",
            "    do\n      while (d) foo(); /// first\n    /* second */\n    while (c);\n",
        ),
        (
            "if (c) { foo() /// first\n; /// second\nclass Local {} } else bar();",
            "    if (c) {\n      foo() /// first\n      ; /// second\n      class Local {}\n    } else bar();\n",
        ),
    ] {
        let input = format!("class A {{ void f() {{ {statement} }} }}");
        let expected = format!("class A {{\n  void f() {{\n{expected}  }}\n}}\n");
        for width in [30, 90] {
            for javadoc in [None, Some(JavadocOptions::default())] {
                let options =
                    Options { line_width: line_width(width), javadoc, ..Options::default() };
                assert_eq!(format_stable_with_options(&input, &options), expected, "{input:?}");
            }
        }
    }
}

#[test]
fn a_documentation_comment_after_a_member_keeps_the_lines_that_continue_it() {
    use crate::fixtures::format_source_with_options;
    use cow_utils::CowUtils;
    use robusta_formatter::{JavadocOptions, Options};

    // `/// m` and `/// n` are one comment for `g`, so no blank line splits
    // them. Rewritten onto one line, the comment takes the blank line a `//`
    // comment there would.
    let javadoc = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    for (input, rewritten) in [
        (
            "class A {\n  void f() {} /// m\n  /// n\n  void g() {}\n}\n",
            "class A {\n  void f() {} /// m n\n\n  void g() {}\n}\n",
        ),
        ("class A {} /// m\n/// n\nclass B {}\n", "class A {} /// m n\n\nclass B {}\n"),
    ] {
        assert_eq!(format_source_with_options(input, &Options::default()), input);
        assert_eq!(format_source_with_options(input, &javadoc), rewritten, "{input:?}");
        assert_eq!(format_source_with_options(rewritten, &javadoc), rewritten);
        let spaced = input.cow_replace("/// n\n", "/// n\n\n");
        assert_eq!(format_source_with_options(&spaced, &javadoc), rewritten, "{spaced:?}");
    }
    for (input, expected) in [
        // javadoc reads a line with an escaped `///` as part of the run.
        (
            "class A {\n  void f() {} /// m\n  \\u002f// n\n  void g() {}\n}\n",
            "class A {\n  void f() {} /// m\n  \\u002f// n\n  void g() {}\n}\n",
        ),
        (
            "class A {\n  void f() {} /// m\n\n  /// n\n  void g() {}\n}\n",
            "class A {\n  void f() {} /// m\n\n  /// n\n  void g() {}\n}\n",
        ),
        (
            "class A {\n  void f() {} // m\n  /// n\n  void g() {}\n}\n",
            "class A {\n  void f() {} // m\n\n  /// n\n  void g() {}\n}\n",
        ),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(input, &options), expected, "{input:?}");
        }
    }
}

#[test]
fn multiline_block_comments_after_a_line_comment_moved_behind_a_semicolon_start_the_next_line() {
    use crate::fixtures::format_source_with_options;
    use cow_utils::CowUtils;
    use robusta_formatter::{JavadocOptions, LineEnding, Options};

    for (input, expected) in [
        (
            "class A { int value = 42 // c\n; /* t\n u */ // keep\n}",
            "class A {\n  int value = 42; // c\n  /* t\n u */ // keep\n}\n",
        ),
        (
            "class A { int value = 42 // c\n; /* t\n * u */\n}",
            "class A {\n  int value = 42; // c\n  /* t\n   * u */\n}\n",
        ),
        (
            "class A { void f() { call() // c\n; /* t\n u */\n} }",
            "class A {\n  void f() {\n    call(); // c\n    /* t\n u */\n  }\n}\n",
        ),
        (
            "class A { void f() { int value = 42 // c\n; /* t\n * u */\n} }",
            "class A {\n  void f() {\n    int value = 42; // c\n    /* t\n     * u */\n  }\n}\n",
        ),
        (
            "class A { void f() { if (a) x = 1 // c\n; /* t\n u */ else y(); } }",
            "class A {\n  void f() {\n    if (a) x = 1; // c\n    /* t\n u */ else y();\n  }\n}\n",
        ),
    ] {
        for (line_ending, newline) in [(LineEnding::Lf, "\n"), (LineEnding::Crlf, "\r\n")] {
            for javadoc in [None, Some(JavadocOptions::default())] {
                let options = Options { javadoc, line_ending, ..Options::default() };
                assert_eq!(
                    format_source_with_options(&input.cow_replace('\n', newline), &options),
                    expected.cow_replace('\n', newline)
                );
            }
        }
    }
}

#[test]
fn block_comments_after_a_line_comment_start_the_next_line() {
    use crate::fixtures::{format_source_with_options, line_width};
    use robusta_formatter::Options;

    let source = "class A { void f() {
foo(a, // c
  b, c, d);
int y = 1 // first
; // second
int n = 2 // seed
; /* counted */
int total = totalFor(order, customer, shippingRegion); // note
} }";
    // The class and method take 4 columns, leaving 50 for the statements.
    let options = Options { line_width: line_width(54), ..Options::default() };
    assert_snapshot!(format_source_with_options(source, &options), @"
    class A {
      void f() {
        foo(
          a, // c
          b,
          c,
          d
        );
        int y = 1; // first // second
        int n = 2; // seed
        /* counted */
        int total = totalFor(
          order,
          customer,
          shippingRegion
        ); // note
      }
    }
    ");
    assert_snapshot!(fmt("class A { int f() { return 2 // seed\n; /* one\ntwo */ } }"), @"
    class A {
      int f() {
        return 2; // seed
        /* one
    two */
      }
    }
    ");
}

#[test]
fn a_line_comment_after_a_body_brace_does_not_break_its_header() {
    use crate::fixtures::{format_source_with_options, line_width};
    use robusta_formatter::Options;

    let source = "class A { void f() {
if (ready) { // checked by the caller before each run
  run();
}
} }";
    // The class and method take 4 columns, leaving 50 for the statements.
    let options = Options { line_width: line_width(54), ..Options::default() };
    assert_snapshot!(format_source_with_options(source, &options), @"
    class A {
      void f() {
        if (ready) { // checked by the caller before each run
          run();
        }
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn body_braces_stay_on_their_header_line() {
    let source = "class Stock {
  void refill() { // called hourly
    if (ready) // checked by the caller
    { run(); }
    else { /* retried later */ queue(); }
    while (more)
    // drained in order
    { next(); }
    if (full) /* by count */ // or by weight
    { stop(); }
    for (Item item : items)
    /* each */ /* once */ { count(item); }
  }

  class Shelf
  /// Holds one product.
  {
    int units;
  }
}";
    assert_snapshot!(fmt(source), @"
    class Stock {
      void refill() { // called hourly
        if (ready) { // checked by the caller
          run();
        } else {
          /* retried later */ queue();
        }
        while (more) {
          // drained in order
          next();
        }
        if (full) /* by count */ { // or by weight
          stop();
        }
        for (Item item : items) {
          /* each */ /* once */ count(item);
        }
      }

      class Shelf
      /// Holds one product.
      {
        int units;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn documentation_comments_before_a_brace_move_unless_they_would_document_a_declaration() {
    let source = "class A {
  void f()
  /** moves */
  { run(); }
  void g()
  /** stays */
  { class Local {} }
  void h() /// moves
  { int count; }
}";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        /** moves */
        run();
      }

      void g()
      /** stays */
      {
        class Local {}
      }

      void h() { /// moves
        int count;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn a_documentation_comment_that_a_blank_line_ends_leaves_the_brace_line() {
    // On the brace's line it would lose the blank line and join the next `///`
    // comment, which documents `cells` alone.
    let source = "class Grid { /// h\n\n/// i\nint cells; }";
    assert_snapshot!(fmt(source), @"
    class Grid {
      /// h

      /// i
      int cells;
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn documentation_comments_lay_out_as_ordinary_comments_unless_that_changes_what_they_document() {
    let source = r"class Order {
  int a /// B
  = /// C
  1 /// D
  ;
  int y = 1 // first
  ; /// Documents z.
  int z;

  enum Kind {
    NEW /// e
    , PAID
  }

  void f() {} /// m
  /// n
  void g() {}
}";
    assert_snapshot!(fmt(source), @"
    class Order {
      int a = /// B /// C
        1 /// D
      ;
      int y = 1; // first
      /// Documents z.
      int z;

      enum Kind {
        NEW /// e
        ,
        PAID,
      }

      void f() {} /// m
      /// n
      void g() {}
    }
    ");
}

#[test]
fn trailing_comments_before_a_semicolon_follow_it_unless_they_document() {
    let source = r"class Order {
  void f() {
    foo() /* c */;
    while (pending()) poll() // retry
    ;
    int n = switch (k) {
      case 1 -> k /* f */;
      default -> 0;
    };
  }

  int x /* d */;
  int id /** e */;
}";
    assert_snapshot!(fmt(source), @"
    class Order {
      void f() {
        foo(); /* c */
        while (pending()) poll(); // retry
        int n = switch (k) {
          case 1 -> k; /* f */
          default -> 0;
        };
      }

      int x; /* d */
      int id /** e */;
    }
    ");
}

#[test]
fn a_comment_spanning_lines_before_an_assignment_operator_moves_after_it() {
    let source = "class Order { void f() {
total /* base
 */
= price;
count /* kept */
= 1;
} }";
    assert_snapshot!(fmt(source), @"
    class Order {
      void f() {
        total =
          /* base
           */
          price;
        count /* kept */ = 1;
      }
    }
    ");
}

#[test]
fn multiline_block_comments_after_a_moved_package_or_import_line_comment_start_the_next_line() {
    use crate::fixtures::format_source_with_options;
    use robusta_formatter::{JavadocOptions, Options};

    for (input, expected) in [
        (
            "import a.A // c\n; /* t\n u */\nclass B {}",
            "import a.A; // c\n/* t\n u */\n\nclass B {}\n\
             ---------- Not idempotent (second pass) ----------\n\
             import a.A; // c\n\n/* t\n u */\n\nclass B {}\n",
        ),
        (
            "package p // c\n; /* t\n * u */\nclass A {}",
            "package p; // c\n/* t\n * u */\n\nclass A {}\n\
             ---------- Not idempotent (second pass) ----------\n\
             package p; // c\n\n/* t\n * u */\n\nclass A {}\n",
        ),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(format_source_with_options(input, &options), expected);
        }
    }
}

/// The comments around the `;` of a bare `break`, `continue`, or `return` keep
/// their source order.
#[test]
fn comments_after_a_bare_keyword_statement_keep_their_order() {
    let source = "class A { void f() { return // c\n; /* t\n u */ } }";
    assert_eq!(fmt(source), "class A {\n  void f() {\n    return; // c\n    /* t\n u */\n  }\n}\n");
}

#[test]
fn type_position_comments_keep_their_qualified_and_annotated_boundaries() {
    const NAME: &str = "comments-and-trivia/type-position-comment-policy";
    assert_java_fixture(
        NAME,
        include_str!("../fixtures/java/comments-and-trivia/type-position-comment-policy.java"),
        include_str!(
            "../fixtures/java/comments-and-trivia/type-position-comment-policy.expected.java"
        ),
    );
}

#[test]
fn expression_type_and_body_boundary_comments_preserve_configured_contracts() {
    use crate::fixtures::{format_stable_with_options, indent_width, line_width};
    use robusta_formatter::{LineEnding, Options};

    for comment in ["/* c */", "// c\n", "\n/* c */\n", "/* c\n * d */"] {
        let sources = [
            format!("class A {{ void m() {{ new <T> {comment} Foo(x); }} }}"),
            format!("class A {{ A() {{ outer {comment} .super(1); }} }}"),
            format!("class A {{ A() {{ outer {comment} .<T>super(); }} }}"),
            format!("enum E {{ A {comment} {{}}, B }}"),
            format!("enum E {{ A(/* inside */) {comment} {{ int x; }}, B }}"),
            format!("class A {{ void m() {{ new Foo(x) {comment} {{}}; }} }}"),
            format!("class A {{ void m() {{ new Foo(/* inside */) {comment} {{ int x; }}; }} }}"),
            format!("class A {{ List<? {comment} extends T> x; }}"),
            format!("class A {{ List<@B ? {comment} super T> x; }}"),
            format!("class A {{ @ {comment} B(1) int x; }}"),
            format!("class A {{ @ {comment} B int x; }}"),
        ];
        for width in [40, 80] {
            for indent in [2, 4] {
                for ending in [LineEnding::Lf, LineEnding::Crlf] {
                    for final_newline in [false, true] {
                        let options = Options {
                            line_width: line_width(width),
                            indent_width: indent_width(indent),
                            line_ending: ending,
                            insert_final_newline: final_newline,
                            ..Options::default()
                        };
                        for source in &sources {
                            format_stable_with_options(source, &options);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn multiline_terminator_comments_preserve_configured_contracts() {
    use crate::fixtures::{format_stable_with_options, indent_width, line_width};
    use robusta_formatter::{LineEnding, Options};

    for ending in [LineEnding::Lf, LineEnding::Crlf] {
        for width in [40, 80] {
            for indent in [2, 4] {
                for final_newline in [false, true] {
                    let options = Options {
                        line_width: line_width(width),
                        indent_width: indent_width(indent),
                        line_ending: ending,
                        insert_final_newline: final_newline,
                        ..Options::default()
                    };
                    for gap in [" ", "\n"] {
                        for statement in [
                            "compute(first, second, third)",
                            "return code",
                            "throw error",
                            "int value = code",
                            "do {} while (ready)",
                        ] {
                            let source = format!(
                                "class A {{ int f() {{ {statement}{gap}/* first\n * second */\n; \
                                 next(); }} }}"
                            );
                            format_stable_with_options(&source, &options);
                        }
                        let source = format!(
                            "class A {{ A() {{ o.super(x){gap}/* first\n * second */; }} }}"
                        );
                        format_stable_with_options(&source, &options);
                        let source =
                            format!("class A {{ native void f(){gap}/* first\n * second */; }}");
                        format_stable_with_options(&source, &options);
                    }
                }
            }
        }
    }
}

#[test]
fn annotation_element_default_comments_lay_out_as_an_assignment() {
    for (source, expected) in [
        ("int f() // c\ndefault 1;", "int f() default // c\n    1;"),
        ("int f()\n// c\ndefault 1;", "int f() default\n    // c\n    1;"),
        ("int f()\n/* c */\ndefault 1;", "int f() default\n    /* c */\n    1;"),
        ("int f() /* c */\ndefault 1;", "int f() /* c */ default 1;"),
        ("int f() /* a\n b */\ndefault 1;", "int f() /* a\n b */ default 1;"),
        ("int f() default // c\n1;", "int f() default // c\n    1;"),
        ("int f() default\n// c\n1;", "int f() default\n    // c\n    1;"),
        ("int f() /* a */ // b\ndefault 1;", "int f() /* a */ default // b\n    1;"),
        ("int[] f() // c\ndefault {1};", "int[] f() default // c\n    {1};"),
        ("@B\nint f() // c\ndefault 1;", "@B\n  int f() default // c\n    1;"),
    ] {
        let source = format!("@interface A {{ {source} }}");
        assert_eq!(fmt(&source), format!("@interface A {{\n  {expected}\n}}\n"));
        check_contract_at_widths_and_modes(&source);
    }
}

#[test]
fn annotation_arguments_keep_their_comments_inside_the_parentheses() {
    let source = "@A(/* empty */) @B() @C /* marker */ @D(/* before */ 1 /* after */) \
                  @E(x = 1 /* first */, /* second */ y = @F({2 /* nested */}) /* last */) class G {}";
    assert_eq!(
        fmt(source),
        "@A(/* empty */)\n@B()\n@C\n/* marker */ @D(/* before */ 1 /* after */)\n\
         @E(x = 1 /* first */, /* second */ y = @F({2 /* nested */}) /* last */)\nclass G {}\n"
    );
    check_contract_at_widths_and_modes(source);
}

#[test]
fn type_parameter_name_comments_lay_out_as_extends_prints_them() {
    for (source, expected) in [
        ("T /* c */ extends B", "<T /* c */ extends B>"),
        ("T /* c */\nextends B", "<T /* c */ extends B>"),
        ("T // c\nextends B", "<\n    T extends // c\n        B>"),
        ("T /* a */ // b\nextends B", "<\n    T /* a */ extends // b\n        B>"),
        ("T\n// c\nextends B", "<\n    T extends\n        // c\n        B>"),
        ("T\n/* c */\nextends B", "<\n    T extends\n        /* c */\n        B>"),
        ("T\n/* c */ extends B", "<T extends /* c */ B>"),
        ("T\n/* a\n * b */ extends B", "<\n    T extends\n        /* a\n         * b */ B>"),
        (
            "T\n/* c */ extends\n/* a\n * b */ B",
            "<\n    T extends\n        /* c */ /* a\n                 * b */ B>",
        ),
        ("T\n// c\n\nextends B", "<\n    T extends\n        // c\n\n        B>"),
        ("T extends /* c */ B", "<T extends /* c */ B>"),
        ("T extends /* c */\nB", "<T /* c */ extends B>"),
        ("T extends // c\nB", "<\n    T extends // c\n        B>"),
        ("T extends\n// c\nB", "<\n    T extends\n        // c\n        B>"),
        ("@D T // c\nextends B & C", "<\n    @D T extends // c\n        B & C>"),
        (
            "T // a\nextends // b\nB // c\n& C",
            "<\n    T extends // a\n        // b\n        B // c\n            & C>",
        ),
    ] {
        let source = format!("class A<{source}> {{}}");
        assert_eq!(fmt(&source), format!("class A{expected} {{}}\n"));
        check_contract_at_widths_and_modes(&source);
    }
}

#[test]
fn receiver_comments_before_this_lead_it_as_a_name() {
    for (source, expected) in [
        ("A /* c */ this", "(A /* c */ this)"),
        ("A // c\nthis", "(\n    A // c\n      this\n  )"),
        ("A\n// c\nthis", "(\n    A\n      // c\n      this\n  )"),
        ("A\n// c\n\nthis", "(\n    A\n      // c\n\n      this\n  )"),
        ("@B A /* c */ this", "(@B A /* c */ this)"),
        ("A.B // c\nA.this", "(\n    A.B // c\n      A.this\n  )"),
    ] {
        let source = format!("class A {{ void f({source}) {{}} }}");
        assert_eq!(fmt(&source), format!("class A {{\n  void f{expected} {{}}\n}}\n"));
        check_contract_at_widths_and_modes(&source);
    }
}

#[test]
fn instance_creation_keyword_comments_lead_what_follows_them() {
    for (source, expected) in [
        ("new /* c */ B()", "new /* c */ B()"),
        ("new // c\nB()", "new // c\n  B()"),
        ("new\n// c\nB()", "new\n  // c\n  B()"),
        ("new\n// c\n\nB()", "new\n  // c\n\n  B()"),
        ("new /* c */ <T> B()", "new /* c */ <T> B()"),
        ("new // c\n<T> B()", "new // c\n  <T> B()"),
        ("new\n// c\n<T> B()", "new\n  // c\n  <T> B()"),
        ("new\n\n// c\n<T> B()", "new\n  // c\n  <T> B()"),
        ("new <T> /* c */ B()", "new <T> /* c */ B()"),
        ("new <T> // c\nB()", "new <T> // c\n  B()"),
        ("new <T>\n// c\nB()", "new <T>\n  // c\n  B()"),
        ("new <T>\n// c\n\nB()", "new <T>\n  // c\n\n  B()"),
        ("a.new /* c */ <T> /* d */ B()", "a.new /* c */ <T> /* d */ B()"),
    ] {
        let source = format!("class A {{ Object x = {source}; }}");
        assert_eq!(fmt(&source), format!("class A {{\n  Object x = {expected};\n}}\n"));
        check_contract_at_widths_and_modes(&source);
    }
}
