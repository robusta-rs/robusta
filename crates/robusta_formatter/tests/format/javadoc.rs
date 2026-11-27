//! Javadoc formatting and its settings.

#[path = "javadoc/embedded.rs"]
mod embedded;

use cow_utils::CowUtils;
use insta::assert_snapshot;
use robusta_formatter::{
    CommentLineStrategy, DescriptionWithDot, JavadocOptions, LineWrappingStyle, Options,
    SeparateTagGroups,
};

use crate::fixtures::{
    format_source_with_options as fmt_with_options, format_stable_with_options, indent_width,
    line_width,
};

#[test]
fn traditional_javadoc_keeps_star_content_off_the_opening_line() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    for (comment, expected) in [
        ("/**\n * ********\n */", "/**\n * ********\n */"),
        ("/** * * Returns true */", "/**\n * * Returns true\n */"),
    ] {
        let source = format!("{comment}\nclass A {{}}\n");
        assert_eq!(fmt_with_options(&source, &options), format!("{expected}\nclass A {{}}\n"));
    }
}

#[test]
fn traditional_javadoc_strips_contiguous_margin_stars() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    let source = "/**\n **\n ** Returns true\n **/\nclass A {}\n";
    assert_eq!(fmt_with_options(source, &options), "/** Returns true */\nclass A {}\n");
}

#[test]
fn traditional_javadoc_preserves_blank_lines_in_an_unclosed_literal() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    let source = "/**\n * <pre>\n *  example\n *\n *\n *\n * */\nclass A {}\n";
    let expected = "/**\n * <pre>\n *  example\n *\n *\n *\n */\nclass A {}\n";
    assert_eq!(fmt_with_options(source, &options), expected);
}

#[test]
fn javadoc_rewrites_ragged_star_margins() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        "*Ragged summary.\n",
        "     * More text.   \n",
        "*\n",
        "no star line\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Ragged summary. More text.
       *
       * no star line
       */
      int value;
    }
    ");
}

#[test]
fn javadoc_preserves_preformatted_indentation() {
    let source = concat!(
        "class A {\n",
        "  /**\n",
        "   * Example:\n",
        "   * <pre>\n",
        "   *     int x = compute();\n",
        "   *     use(x);\n",
        "  int y = 1;\n",
        "      nested(y);\n",
        "   * </pre>\n",
        "   */\n",
        "  int value;\n",
        "}\n",
    );
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Example:
       *
       * <pre>
       *     int x = compute();
       *     use(x);
       *   int y = 1;
       *       nested(y);
       * </pre>
       */
      int value;
    }
    ");
}

#[test]
fn javadoc_keeps_short_single_line_comments_and_non_doc_comments() {
    let source = concat!(
        "class A {\n",
        "/** Single line doc. */\n",
        "int a;\n",
        "/*\n",
        "*Plain block interior.\n",
        " */\n",
        "int b;\n",
        "}\n",
    );
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /** Single line doc. */
      int a;
      /*
       *Plain block interior.
       */
      int b;
    }
    ");
}

#[test]
fn javadoc_disabled_by_default_preserves_ragged_interior() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        "*Ragged summary.\n",
        "     * More text.\n",
        " */\n",
        "int value;\n",
        "}\n",
    );

    assert_snapshot!(fmt_with_options(source, &Options::default()), @"
    class A {
      /**
       *Ragged summary.
       * More text.
       */
      int value;
    }
    ");
}

#[test]
fn javadoc_wraps_prose_paragraphs_and_block_tags_at_the_output_column() {
    let source = concat!(
        "class A {\n",
        " /**\n",
        "  * This   summary has many spaces and enough words to wrap predictably at width.\n",
        "  * <p>\n",
        "  * A second paragraph stays separate from the first one.\n",
        "  *\n",
        "  * @param value this description also wraps beneath the tag using configured indentation.\n",
        "  * @return the resulting value without changing tag order\n",
        "  */\n",
        " int value;\n",
        "}\n",
    );
    let options = Options { line_width: line_width(50), ..settings(JavadocOptions::default()) };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * This summary has many spaces and enough words
       * to wrap predictably at width.
       *
       * <p>A second paragraph stays separate from the
       * first one.
       *
       * @param value this description also wraps
       *     beneath the tag using configured
       *     indentation.
       * @return the resulting value without changing
       *     tag order
       */
      int value;
    }
    ");
}

#[test]
fn javadoc_preserves_unicode_spaces() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Weight 10\u{a0}kg\u{3000}and a few more words.\n",
        " */\n",
        "int traditional;\n",
        "/// Weight 10\u{a0}kg\u{3000}and a few more words.\n",
        "int markdown;\n",
        "}\n",
    );
    let options = Options {
        line_width: line_width(40),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };

    assert_eq!(
        fmt_with_options(source, &options),
        concat!(
            "class A {\n",
            "  /**\n",
            "   * Weight 10\u{a0}kg\u{3000}and a few more words.\n",
            "   */\n",
            "  int traditional;\n",
            "  /// Weight 10\u{a0}kg\u{3000}and a few more\n",
            "  /// words.\n",
            "  int markdown;\n",
            "}\n",
        )
    );
}

#[test]
fn javadoc_normalizes_horizontal_whitespace() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * This\t\u{c} prose has enough words\t\u{c}to wrap.\n",
        " */\n",
        "int traditional;\n",
        "/// This\t\u{c} prose has enough words\t\u{c}to wrap.\n",
        "int markdown;\n",
        "}\n",
    );
    let options = Options {
        line_width: line_width(40),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * This prose has enough words to
       * wrap.
       */
      int traditional;
      /// This prose has enough words to
      /// wrap.
      int markdown;
    }
    ");
}

#[test]
fn javadoc_preserves_literal_and_preformatted_regions() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Before {@code Map<{A},  B>} after inline literal text that wraps.\n",
        " * <PRE>\n",
        " *     int  x = call(  value );\n",
        " *       keep();\n",
        " * </PrE>\n",
        " * Before {@literal one { two }\n",
        " *   three } after.\n",
        " * ```java\n",
        " *   call(  x );\n",
        " * ```\n",
        " * ~~~\n",
        " *   tilde(  x );\n",
        " * ~~~\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let options = Options {
        line_width: line_width(46),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };
    let formatted = format_stable_with_options(source, &options);

    assert!(formatted.contains("{@code Map<{A},  B>}"));
    assert!(formatted.contains(" *     int  x = call(  value );\n"));
    assert!(formatted.contains(" *       keep();\n"));
    assert!(formatted.contains(" * Before\n"));
    assert!(formatted.contains(" * {@literal one { two }\n"));
    assert!(formatted.contains(" *   three }\n"));
    assert!(formatted.contains(" * after.\n"));
    assert!(formatted.contains(" * ```java\n   *   call(  x );\n   * ```\n"));
    assert!(formatted.contains(" * ~~~\n   *   tilde(  x );\n   * ~~~\n"));
}

#[test]
fn javadoc_normalizes_single_line_comments_but_not_non_doc_comments() {
    let source = concat!(
        "class A {\n",
        "/** Single   line doc. */\n",
        "int a;\n",
        "/* plain   block */\n",
        "// line   comment\n",
        "int b;\n",
        "}\n",
    );
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    let formatted = format_stable_with_options(source, &options);

    assert!(formatted.contains("/** Single line doc. */"));
    assert!(formatted.contains("/* plain   block */"));
    assert!(formatted.contains("// line   comment"));
}

#[test]
fn empty_javadoc_comments_are_kept() {
    let source =
        "class A { /**\n */ int a;\n/**\n *\n */ int b;\n/** */\nint c;\n///\n///\nint d; }";
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /** */ int a;
      /** */ int b;
      /** */
      int c;
      ///
      int d;
    }
    ");
    for (comment_line_strategy, single_line, multiline) in [
        (CommentLineStrategy::SingleLine, "/** */", "/** */"),
        (CommentLineStrategy::Keep, "/** */", "/**\n */"),
        (CommentLineStrategy::Multiline, "/**\n */", "/**\n */"),
    ] {
        let options = settings(JavadocOptions { comment_line_strategy, ..Default::default() });
        for (comment, expected) in [("/**  */", single_line), ("/**\n *\n */", multiline)] {
            let source = format!("{comment}\npublic interface A {{}}\n");
            assert_eq!(
                fmt_with_options(&source, &options),
                format!("{expected}\npublic interface A {{}}\n")
            );
        }
    }
}

#[test]
fn empty_javadoc_keeps_the_blank_lines_around_it() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    let method = |body: &str| format!("class A {{\n  void f() {{\n{body}  }}\n}}\n");

    for source in [
        "/** */\n\n// c\nclass A {}\n".to_owned(),
        "// a\n/** */\n\n// b\nclass A {}\n".to_owned(),
        "class A extends B\n    /** */\n\n    // c\n    implements C {}\n".to_owned(),
        "class A {}\n\n/** */\n".to_owned(),
        "package p;\n\n/** */\n".to_owned(),
        "import a.B;\n\n/** */\n".to_owned(),
        "module m {}\n\n/** */\n".to_owned(),
        "class A {}\n\n/** */\nclass B {}\n".to_owned(),
        "class B {\n  int x;\n\n  /** */\n}\n".to_owned(),
        // A Javadoc of only U+00A0 has content.
        "class B {\n  int x;\n\n  /** \u{a0} */\n}\n".to_owned(),
        method("    if (a) {}\n    /** */\n\n    // c\n    else {}\n"),
        method("    if (a) {}\n    /** */\n    else {}\n"),
        method("    try {}\n    /** */\n\n    // c\n    catch (E e) {}\n"),
        method("    try {}\n    /** */\n    catch (E e) {}\n"),
    ] {
        assert_eq!(fmt_with_options(&source, &options), source);
    }
    for (source, expected) in [
        ("///\n///\n\nclass A {}\n", "///\n\nclass A {}\n"),
        ("class A {}\n/** */\n\nclass B {}\n", "class A {}\n\n/** */\n\nclass B {}\n"),
    ] {
        assert_eq!(fmt_with_options(source, &options), expected);
    }
}

#[test]
fn body_holding_only_an_empty_javadoc_stays_open() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    let method = |body: &str| format!("class A {{\n  void f() {{\n{body}  }}\n}}\n");

    for source in [
        "class A {\n  /** */\n}\n".to_owned(),
        "enum E {\n  /** */\n}\n".to_owned(),
        "module m {\n  /** */\n}\n".to_owned(),
        "class A {\n  /** */\n  // c\n}\n".to_owned(),
        method("    /** */\n"),
        method("    switch (x) {\n      /** */\n    }\n"),
    ] {
        for options in [&Options::default(), &options] {
            assert_eq!(fmt_with_options(&source, options), source);
        }
    }
}

#[test]
fn empty_javadoc_in_a_token_gap_formats_in_one_pass() {
    let method = |body: &str| format!("class A {{\n  void f() {{\n{body}  }}\n}}\n");
    let sources = [
        "class A {\n    int y = a +\n        /** */\n\n        b;\n}\n".to_owned(),
        "class A {\n  int a = 1,\n    /** */\n    b = 2;\n}\n".to_owned(),
        "class A { void f() { if ( /** */\n/* a\n b */ a) {} } }".to_owned(),
        method("    g(a,\n      /** */\n      x);\n"),
        method("    a\n      .b()\n      /** */\n\n      .c();\n"),
        method("    return\n      /** */\n      x;\n"),
        method("    return\n      /** */\n      // c\n      x;\n"),
        method("    g(\n      /** */\n    );\n"),
        method("    g(a, b\n      /** */\n    );\n"),
        method("    g(a /**\n      */);\n"),
        method("    if (a)\n      /** */\n      x();\n"),
        method("    r = () ->\n      /** */\n      1;\n"),
        method("    r = () ->\n      /** */ /* c */\n      1;\n"),
        method("    x = -\n      /** */\n      /* a\n       b */\n      y;\n"),
        method("    x = y\n      /** */\n      ++;\n"),
        "class A { int\n/** */ [] a; }".to_owned(),
        "class A { int[\n/** */ ] a; }".to_owned(),
        "class A { int a\n/** */ []; }".to_owned(),
        "class A { int f() { return - /** */\n1; } }".to_owned(),
        "class A { void f() { count\n/** */ ++; } }".to_owned(),
    ];
    for options in [Options::default(), settings(JavadocOptions::default())] {
        for source in &sources {
            let formatted = format_stable_with_options(source, &options);
            assert_eq!(
                formatted.matches("/**").count(),
                source.matches("/**").count(),
                "{formatted}"
            );
            // An empty comment prints where one with text does.
            let with_text =
                format_stable_with_options(&source.cow_replace("/** */", "/** x */"), &options);
            assert_eq!(formatted, with_text.cow_replace("/** x */", "/** */"), "{source:?}");
        }
    }
}

#[test]
fn javadoc_sharing_its_line_with_another_comment_formats_in_one_pass() {
    let options = settings(JavadocOptions::default());
    let method = |body: &str| format!("class A {{\n  void f() {{\n{body}  }}\n}}\n");

    for (source, expected) in [
        (
            method("    if (a)\n      /**  x */ // c\n      x();\n"),
            method("    if (a)\n      /**  x */ // c\n      x();\n"),
        ),
        (
            method("    g(a\n      /**  x */ /* c */\n    );\n"),
            method("    g(\n      a\n      /**  x */ /* c */\n    );\n"),
        ),
    ] {
        assert_eq!(fmt_with_options(&source, &options), expected);
    }
}

#[test]
fn javadoc_moved_beside_no_code_formats_in_one_pass() {
    let options = settings(JavadocOptions::default());
    let method = |body: &str| format!("class A {{\n  void f() {{\n{body}  }}\n}}\n");

    for (source, expected) in [
        (
            method("    if (a) /* c */\n      /**  x */ x();\n"),
            method("    if (a) /* c */ /**  x */ x();\n"),
        ),
        (method("    if (a)\n      /** */ x();\n"), method("    if (a)\n      /** */ x();\n")),
        (
            method("    g(a\n      /** */,\n      b);\n"),
            method("    g(\n      a,\n      /** */\n      b\n    );\n"),
        ),
        (
            method("    g(a\n      /** */,\n\n      b);\n"),
            method("    g(\n      a,\n      /** */\n\n      b\n    );\n"),
        ),
        (method("    g(/** */\n      a);\n"), method("    g(/** */ a);\n")),
        (
            method("    g(a\n      /**  x */);\n"),
            method("    g(\n      a\n      /**  x */\n    );\n"),
        ),
    ] {
        assert_eq!(fmt_with_options(&source, &options), expected);
    }
}

#[test]
fn javadoc_moved_off_code_at_token_gaps_formats_in_one_pass() {
    let sources = [
        "class A { void f() {\nif (a\n    /** */ > 0) x();\n} }\n",
        "class A { void f() {\nboolean t = o\n    /** */ instanceof String;\n} }\n",
        "class A { void f() {\nif (b\n    /** */ ) y();\n} }\n",
        "class A { void f() {\nint[] q = {a,\n    /** */ b};\n} }\n",
        "class A { void f() {\nfor (var e : /** */\n    list) x();\n} }\n",
        "class A { void f() {\nlist.stream()\n    /** */ .filter(p -> p.ok()).map(Q::r).collect(toList());\n} }\n",
        "class A { void f() {\nlong yield = /** @return x */ /* c */\n0;\n} }\n",
        "class A { void f() {\nfor (int i = 0; i < n; i++\n    /**  x */ /* c */\n    ) x();\n} }\n",
        "class A { void f() {\nint[] q = {1, 2\n/** */ , // second\n};\n} }\n",
        "class A { void f() {\nint[] q = {1,\n/** */\n2, 3 // c\n};\n} }\n",
        "class A { void f() {\nif (a\n/**  x */ > 0) x();\n} }\n",
        "class A { void f() {\nfor (var e : /**  x */\n    list) x();\n} }\n",
        "import java.util.List\n/** */ ;\nclass A {}",
        "package p\n/** */ ;\nclass A {}",
        "class A { <U\n/** */ >\nU last(U v) { return v; } }",
        "import java.util.List\n/** */ ; /* import */\nclass A {}",
        "class A { <U\n/** */ > // r\nU last(U v) { return v; } }",
        "class A { void f() { try (A a = x(); /**  x */\n B b = y()) {} } }",
        "class A\n/**  x */ extends B {}",
        "record A(int x)\n/**  x */ implements B {}",
        "class A { void f() { try /**  x */ /* c */\n(A a = x()) {} } }",
        "class A { int aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa = /** @return x */ /* c */\n0; }",
        "class A { void f() { if (a\n/* a */ /** */ > 0) x(); } }",
        "class A { void f() { int[] q = {a, // c  \n/** */ b}; } }",
        "class A { void f() { int[] q = {a, // c  \n/** */ /* b */ b}; } }",
        "class A { void f() { int[] q = {a,\n/** */ // c  \n/** */ b}; } }",
        "class A { void f() { int[] q = {a,\n/** x */ // c  \n/** */ b}; } }",
        "class A { void f() { int[] q = {a,\n/* a */ /** */ b}; } }",
        "class A { void f() { list.stream()\n/* a */ /** */ .filter(p -> p.ok()); } }",
    ];
    for comment_line_strategy in
        [CommentLineStrategy::SingleLine, CommentLineStrategy::Keep, CommentLineStrategy::Multiline]
    {
        let options = Options {
            javadoc: Some(JavadocOptions { comment_line_strategy, ..Default::default() }),
            ..Options::default()
        };
        for source in sources {
            format_stable_with_options(source, &options);
        }
    }
}

#[test]
fn moved_javadoc_operator_output() {
    let source = "class A { void f() {\nif (a\n/**  x */ > 0) x();\n} }\n";

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      void f() {
        if (a > /**  x */ 0) x();
      }
    }
    ");
}

#[test]
fn moved_javadoc_list_separator_output() {
    let source = "class A { void f() {\nint[] q = {a,\n    /** */ b};\n} }\n";

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      void f() {
        int[] q = {
          a,
          /** */ b,
        };
      }
    }
    ");
}

#[test]
fn moved_javadoc_assignment_output() {
    let source = "class A { int aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa = /** @return x */ /* c */\n0; }";

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      int
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa =
          /** @return x */ /* c */ 0;
    }
    ");
}

#[test]
fn empty_javadoc_before_an_assignment_value_formats_in_one_pass() {
    let source = "class A { void f() {\nsomeVariableNameThatIsLongEnoughToPushTheCallPastTheWidth =\n/** */\nfirstMethodCall(firstArgumentName, secondArgumentName);\n} }";

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      void f() {
        someVariableNameThatIsLongEnoughToPushTheCallPastTheWidth =
          /** */
          firstMethodCall(firstArgumentName, secondArgumentName);
      }
    }
    ");
}

#[test]
fn javadoc_before_an_assignment_operator_formats_in_one_pass() {
    let source = "class A { void f() {\nint local /** x */\n= 1; /* local declaration */\n} }";
    let options = Options { line_width: line_width(40), ..settings(JavadocOptions::default()) };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        int
          local /** x */ = 1; /* local declaration */
      }
    }
    ");
}

#[test]
fn token_gap_javadoc_trims_the_preceding_line_comments_spaces() {
    for comment_line_strategy in
        [CommentLineStrategy::SingleLine, CommentLineStrategy::Keep, CommentLineStrategy::Multiline]
    {
        let options = settings(JavadocOptions { comment_line_strategy, ..Default::default() });
        for gap in [" // c  \n/** */", "\n/** x */ // c  \n/** */"] {
            let source = format!("class A {{ void f() {{ for (int i=0; i<n; i++{gap} ) x(); }} }}");
            let formatted = format_stable_with_options(&source, &options);
            assert!(formatted.contains("// c\n"), "{formatted}");
        }
    }
}

#[test]
fn javadoc_sharing_an_operators_line_keeps_its_text() {
    let options = settings(JavadocOptions::default());
    for source in [
        "class A { int f() { return a /**  x */ +\n b; } }",
        "class A { void f() { count /**  x */ ++; } }",
    ] {
        assert!(format_stable_with_options(source, &options).contains("/**  x */"));
    }
}

#[test]
fn javadoc_before_a_wrapped_catch_union_formats_in_one_pass() {
    for comment in ["/** */", "/**  x */", "/** @return x */"] {
        let source = format!(
            "class A {{ void f() {{ try {{}} catch (InstantiationException {comment}\n| IllegalAccessException | InvocationTargetException exception) {{}} }} }}"
        );
        format_stable_with_options(&source, &settings(JavadocOptions::default()));
    }
}

#[test]
fn javadoc_after_a_stacked_annotation_formats_in_one_pass() {
    for comment_line_strategy in
        [CommentLineStrategy::SingleLine, CommentLineStrategy::Keep, CommentLineStrategy::Multiline]
    {
        let options = settings(JavadocOptions { comment_line_strategy, ..Default::default() });
        for comment in ["/** */", "/**  x */", "/** @return x */"] {
            for source in [
                format!("@Deprecated\n{comment} public class A {{}}"),
                format!("@Deprecated\n{comment} @NullMarked\npackage p;"),
                format!("class A {{ @Override\n/* a */ {comment} public void f() {{}} }}"),
            ] {
                format_stable_with_options(&source, &options);
            }
        }
    }
    // A comment directly before `public` leads it and stays on its line.
    for (source, expected) in [
        ("@Deprecated\n/** */ public class A {}", "@Deprecated\n/** */ public class A {}\n"),
        ("@Deprecated /**  x */ public class A {}", "@Deprecated\n/**  x */ public class A {}\n"),
    ] {
        assert_eq!(fmt_with_options(source, &settings(JavadocOptions::default())), expected);
    }
}

#[test]
fn javadoc_after_the_last_unit_declaration_stays_as_written() {
    for source in [
        "class A {}\n\n/**\n * x\n */\n",
        "class A {}\n/**  x */\n",
        "class A {} // c\n/**  x */\n",
        "package p;\n\n/**  x */\n",
        "module m {}\n\n/**  x */\n",
    ] {
        for options in [Options::default(), settings(JavadocOptions::default())] {
            assert_eq!(fmt_with_options(source, &options), source);
        }
    }
}

#[test]
fn javadoc_in_a_parenthesized_expression_stays_as_written() {
    let source = "class A {\n    int x = (1\n        /**  x */\n    );\n}\n";

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      int x = (1
      /**  x */
      );
    }
    ");
}

#[test]
fn markdown_javadoc_groups_runs_and_normalizes_margins() {
    let source = concat!(
        "class A {\n",
        "/// First.\n",
        "  ///   indented\n",
        "///\n",
        "/// hard break  \n",
        "// ordinary line comment\n",
        "///Separate document.\n",
        "int value;\n",
        "}\n",
    );
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    let formatted = fmt_with_options(source, &options);

    assert_eq!(
        formatted,
        concat!(
            "class A {\n",
            "  /// First.\n",
            "  ///   indented\n",
            "  ///\n",
            "  /// hard break\n",
            "  // ordinary line comment\n",
            "  /// Separate document.\n",
            "  int value;\n",
            "}\n",
        )
    );
}

#[test]
fn four_slash_lines_are_markdown_lines() {
    // javadoc reads `//// a` as the Markdown line `/ a`.
    let javadoc = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    for (source, rewritten) in [
        ("package p; //// a\n//// b\nclass A {}\n", "package p; /// / a / b\n\nclass A {}\n"),
        (
            "class A {\n  int x; /// a  \n  //// b\n  int y;\n}\n",
            "class A {\n  int x; ///  a  \n  /// / b\n  int y;\n}\n",
        ),
        ("class A {\n  ///////////\n  int x;\n}\n", "class A {\n  /// ////////\n  int x;\n}\n"),
    ] {
        assert_eq!(fmt_with_options(source, &Options::default()), source);
        assert_eq!(fmt_with_options(source, &javadoc), rewritten);
        assert_eq!(fmt_with_options(rewritten, &javadoc), rewritten);
    }
}

#[test]
fn markdown_hard_break_keeps_its_spaces_before_a_top_level_type() {
    let class = "/// line one  \n/// line two\nclass A {\n  /// member one  \n  /// member two\n  int x;\n}\n";
    let javadoc = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };

    for source in [class.to_owned(), format!("package p;\n\n{class}")] {
        for options in [&Options::default(), &javadoc] {
            assert_eq!(fmt_with_options(&source, options), source);
        }
    }
}

// A hard break needs a line after it.
#[test]
fn spaces_after_the_last_line_of_a_markdown_comment_are_dropped() {
    for javadoc in [None, Some(JavadocOptions::default())] {
        let options = Options { javadoc, ..Options::default() };
        for (source, expected) in [
            ("/// a  \n/** Doc. */\nclass A {}\n", "/// a\n/** Doc. */\nclass A {}\n"),
            ("/// a\n///\n/// ---  \nclass A {}\n", "/// a\n///\n/// ---\nclass A {}\n"),
            ("/// a  \n\n/// b\nclass A {}\n", "/// a\n\n/// b\nclass A {}\n"),
            (
                "class A {\n  /// a  \n\n  // b\n  void f() {}\n}\n",
                "class A {\n  /// a\n\n  // b\n  void f() {}\n}\n",
            ),
            ("class A {\n  /// a  \n\n  // b\n}\n", "class A {\n  /// a\n\n  // b\n}\n"),
            ("/// ```\n/// b\n/// ```  \nclass A {}\n", "/// ```\n/// b\n/// ```\nclass A {}\n"),
            (
                "class A {\n  public\n  /// first  \n  /* second\n   third */ static int a;\n}\n",
                "class A {\n  public\n  /// first\n  /* second\n   third */ static int a;\n}\n",
            ),
        ] {
            assert_eq!(fmt_with_options(source, &options), expected);
        }
    }
}

// On a line of code they are code text.
#[test]
fn spaces_after_the_last_line_of_markdown_code_are_kept() {
    let javadoc = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    for source in [
        "/// a\n///\n///     code  \nclass A {}\n",
        "/// a\n/// ```text\n/// b  \nclass A {}\n",
        "/// a\n///\n/// \tcode  \nclass A {}\n",
        "class A {\n  void m() {\n    /// a\n    ///\n    ///     code\t\n    int x;\n  }\n}\n",
        "class A {\n  /// a\n  ///\n  ///     code  \n\n  // b\n  void f() {}\n}\n",
        "class A {\n  /// a\n  ///\n  ///     code  \n\n  // b\n}\n",
    ] {
        assert_eq!(format_stable_with_options(source, &Options::default()), source);
    }
    for indent in ["    ", "\t"] {
        assert_eq!(
            format_stable_with_options(
                &format!("/// a\n///\n/// {indent}code  \nclass A {{}}\n"),
                &javadoc
            ),
            "/// a\n///\n/// ```\n/// code  \n/// ```\nclass A {}\n"
        );
    }
    // Each block tag ends its own Markdown, wherever it moves.
    assert_eq!(
        format_stable_with_options(
            concat!(
                "class A {\n",
                "  /// a\n  ///\n  /// @return r\n  ///\n  /// \tcode  \n",
                "  /// @param x d\n  ///\n  /// # h  \n",
                "  int f(int x) {}\n}\n",
            ),
            &javadoc,
        ),
        concat!(
            "class A {\n",
            "  /// a\n  ///\n  /// @param x d\n  ///\n  /// # h\n",
            "  /// @return r\n  ///\n  /// ```\n  /// code  \n  /// ```\n",
            "  int f(int x) {}\n}\n",
        )
    );
}

#[test]
fn markdown_hard_breaks_survive_inside_comment_gaps() {
    let cases = [
        "class A {\n  int m() {\n    return a\n      /// first  \n      /// second\n      + b;\n  }\n}\n",
        "class A {\n  void m() {\n    /// first  \n    /// second\n  }\n}\n",
        "class A {\n  void m() {\n    /// first  \n    /// second\n\n    // third\n  }\n}\n",
        "class A {\n  void m(\n    /// first  \n    /// second\n    int a\n  ) {}\n}\n",
        "class A {\n  /// ```\n  /// code  \n  ///\n  /// \ttab\t\n  /// ```\n  int x;\n}\n",
        "class A {\n  int x; /// first  \n  /// second\n  int y;\n}\n",
        "class A {\n  int x; /// first\t\n  /// second  \n  /// third\n  int y;\n}\n",
        "enum E {\n  A, /// first  \n  /// second\n  B,\n}\n",
    ];
    for source in cases {
        for javadoc in [None, Some(JavadocOptions::default())] {
            assert_eq!(
                fmt_with_options(source, &Options { javadoc, ..Options::default() }),
                source
            );
        }
    }
    // Rewritten whole, a run after a method, a type, or the package is
    // followed by the blank line that follows them.
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    for (source, rewritten) in [
        (
            "class A {\n  void m() {} /// first  \n  /// second\n  int y;\n}\n",
            "class A {\n  void m() {} /// first  \n  /// second\n\n  int y;\n}\n",
        ),
        (
            "class A {} /// first  \n/// second\nclass B {}\n",
            "class A {} /// first  \n/// second\n\nclass B {}\n",
        ),
        (
            "package p; /// first  \n/// second\nclass A {}\n",
            "package p; /// first  \n/// second\n\nclass A {}\n",
        ),
    ] {
        assert_eq!(fmt_with_options(source, &Options::default()), source);
        assert_eq!(fmt_with_options(source, &options), rewritten);
    }
}

// javadoc reads the run as one comment, so the package's blank line may not
// split it before a declaration it documents. Before an import it documents
// nothing and is laid out as `//` comments are.
#[test]
fn a_markdown_run_on_the_package_line_stays_whole_before_a_declaration() {
    let source = "package p; /// first\n/// second\nclass A {}\n";
    assert_eq!(fmt_with_options(source, &Options::default()), source);
    assert_eq!(
        fmt_with_options(
            source,
            &Options { javadoc: Some(JavadocOptions::default()), ..Options::default() }
        ),
        "package p; /// first second\n\nclass A {}\n"
    );
    for (source, expected) in [
        (
            "package p; /// first  \n/// second\nimport a.B;\n",
            "package p; /// first\n\n/// second\nimport a.B;\n",
        ),
        (
            "package p; /// first  \n/// second  \n/// third\n\nimport a.B;\n",
            "package p; /// first\n\n/// second  \n/// third\n\nimport a.B;\n",
        ),
        (
            "package p; /// first\n/// second\nimport z.Z;\nimport a.A;\nclass A {}\n",
            "package p; /// first\n\nimport a.A;\n/// second\nimport z.Z;\n\nclass A {}\n",
        ),
        (
            "package p; // first\n// second\nimport a.B;\n",
            "package p; // first\n\n// second\nimport a.B;\n",
        ),
        (
            "package p; /// first\n\n/// second\nclass A {}\n",
            "package p; /// first\n\n/// second\nclass A {}\n",
        ),
        (
            "package p; // first\n// second\nclass A {}\n",
            "package p; // first\n\n// second\nclass A {}\n",
        ),
    ] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(fmt_with_options(source, &options), expected);
        }
    }
}

#[test]
fn escaped_newlines_keep_markdown_documentation_runs_together() {
    for (gap, blank) in [
        (r"\u000a", ""),
        (r"\u000d", ""),
        (r"\u000d\u000a", ""),
        ("\r\\u000a", ""),
        ("\\u000d\n", ""),
        (r"\uu000a\u0020\u0009\u000c", ""),
        (r"\u000a\u000a", "\n"),
        ("\n\\u0020\n", "\n"),
    ] {
        for (source, expected, joined) in [
            (
                format!("package p; /// a{gap}/// b\nclass A {{}}\n"),
                format!("package p; /// a\n{blank}/// b\nclass A {{}}\n"),
                "package p; /// a b\n\nclass A {}\n",
            ),
            (
                format!("class A {{void f() {{}} /// a{gap}/// b\\u000avoid g() {{}}}}"),
                format!("class A {{\n  void f() {{}} /// a\n{blank}  /// b\n  void g() {{}}\n}}\n"),
                "class A {\n  void f() {} /// a b\n\n  void g() {}\n}\n",
            ),
        ] {
            assert_eq!(fmt_with_options(&source, &Options::default()), expected, "{source:?}");
            // Rewritten, one run is one paragraph.
            let rewritten = if blank.is_empty() { joined } else { &expected };
            let options =
                Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
            assert_eq!(fmt_with_options(&source, &options), rewritten, "{source:?}");
        }
    }
}

#[test]
fn escaped_markdown_boundaries_keep_own_line_comments_and_hard_breaks() {
    for gap in [r"\u000a", r"\u000d\u000a", "\r\\u000a", "\\u000d\n"] {
        let source = format!("class A {{\n/// first  {gap}/// last\nvoid f() {{}}}}");
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(
                fmt_with_options(&source, &options),
                "class A {\n  /// first  \n  /// last\n  void f() {}\n}\n"
            );
        }
    }
    for gap in ["\n\n", r"\u000a\u000a", r"\u000d\u000a\u000a"] {
        let source = format!("class A {{\n/// first{gap}/// last\nvoid f() {{}}}}");
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(
                fmt_with_options(&source, &options),
                "class A {\n  /// first\n\n  /// last\n  void f() {}\n}\n"
            );
        }
    }
}

#[test]
fn escaped_markdown_boundaries_keep_fences_and_raw_content() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_eq!(
        fmt_with_options(
            "package p; /// ```java\\u000a/// int x=1;\\u000a/// ```\nclass A {}",
            &options
        ),
        "package p; /// ```java\n/// int x = 1;\n/// ```\n\nclass A {}\n"
    );
    assert_eq!(
        fmt_with_options("class A {\n/// first  \\u000a/// \\u005cu000a\nvoid f() {}}", &options),
        "class A {\n  /// first  \n  /// \\u005cu000a\n  void f() {}\n}\n"
    );
}

// A `///` after a modifier documents nothing, so its trailing spaces mean nothing.
#[test]
fn a_markdown_comment_after_a_modifier_is_an_ordinary_comment() {
    let source = "class A {\n  public /// first  \n  /// second\n  void m() {}\n}\n";
    for javadoc in [None, Some(JavadocOptions::default())] {
        assert_eq!(
            fmt_with_options(source, &Options { javadoc, ..Options::default() }),
            "class A {\n  public /// first\n  /// second\n  void m() {}\n}\n"
        );
    }
}

// A `///` comment that trails an operand documents nothing, so its spaces mean nothing in
// either pass.
#[test]
fn a_markdown_comment_after_an_operand_loses_its_spaces() {
    let source = "class A { int m() { return a ? /// h  \n/// i\nb : c; } }\n";
    let expected =
        "class A {\n  int m() {\n    return a /// h\n      /// i\n      ? b\n      : c;\n  }\n}\n";
    for javadoc in [None, Some(JavadocOptions::default())] {
        let options = Options { javadoc, ..Options::default() };
        let formatted = fmt_with_options(source, &options);
        assert_eq!(formatted, expected);
        assert_eq!(fmt_with_options(&formatted, &options), expected);
    }
}

// javadoc reads a `///` run as one comment even when its first line follows
// code, so it is rewritten whole: its summary is the whole first paragraph,
// and an unclosed fence is closed once.
#[test]
fn a_markdown_run_starting_after_code_is_rewritten_whole() {
    let options = Options {
        javadoc: Some(JavadocOptions {
            capitalize_descriptions: true,
            description_with_dot: DescriptionWithDot::Summary,
            ..JavadocOptions::default()
        }),
        ..Options::default()
    };
    for (source, expected) in [
        (
            "class A {\n  void f() {} /// first part\n  /// second part\n  void g() {}\n}\n",
            "class A {\n  void f() {} /// First part second part.\n\n  void g() {}\n}\n",
        ),
        (
            "class A {\n  int a; /// ```\n  /// int x;\n  int b;\n}\n",
            "class A {\n  int a; /// ```\n  /// int x;\n  /// ```\n  int b;\n}\n",
        ),
        (
            "class A {\n  int x; /// first  \n  /// \n  /// third\n  int y;\n}\n",
            "class A {\n  int x; /// First.\n  ///\n  /// Third\n  int y;\n}\n",
        ),
        (
            "class A {\n  void m() {\n    f(); /// ```\n    /// int x;\n    class L {}\n  }\n}\n",
            "class A {\n  void m() {\n    f(); /// ```\n    /// int x;\n    /// ```\n    class L {}\n  }\n}\n",
        ),
        // The later lines follow the label's trailing comment.
        (
            "class A {\n  void m(int x) {\n    switch (x) {\n      case 1: /// ```\n        /// int x;\n        class M {}\n    }\n  }\n}\n",
            "class A {\n  void m(int x) {\n    switch (x) {\n      case 1: /// ```\n      /// int x;\n      /// ```\n        class M {}\n    }\n  }\n}\n",
        ),
        (
            "package p; /// ```\n/// int x;\nclass A {}\n",
            "package p; /// ```\n/// int x;\n/// ```\n\nclass A {}\n",
        ),
    ] {
        assert_eq!(fmt_with_options(source, &Options::default()), source);
        assert_eq!(fmt_with_options(source, &options), expected);
        assert_eq!(fmt_with_options(expected, &options), expected);
    }
}

#[test]
fn trailing_spaces_before_syntax_or_a_comment_leave_no_blank_line() {
    let list = "class A {\n    int[] a = {\n        1, // one  \n        2,\n    };\n}\n";
    assert_eq!(
        fmt_with_options(list, &Options::default()),
        "class A {\n  int[] a = {\n    1, // one\n    2,\n  };\n}\n"
    );

    let resource = "class A {\n    void m() {\n        try (// a  \n            /** */\n            // b\n            Foo f = open()) {\n        }\n    }\n}\n";
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_eq!(
        fmt_with_options(resource, &options),
        "class A {\n  void m() {\n    try ( // a\n      /** */\n      // b\n      Foo f = open()\n    ) {}\n  }\n}\n"
    );
}

#[test]
fn javadoc_rewrites_skip_banner_comments_and_unattached_markdown_runs() {
    let source = concat!(
        "/*******************************************************************************\n",
        " * Copyright (c) 2026 Example\n",
        " * All rights reserved. This program\n",
        " * and the accompanying materials are made available.\n",
        " *******************************************************************************/\n",
        "package p;\n",
        "\n",
        "class A {\n",
        "  void m() {\n",
        "    ///statement   run one\n",
        "    ///statement   run two\n",
        "    doThing();\n",
        "  }\n",
        "\n",
        "  ///end   of body\n",
        "}\n",
    );
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_eq!(fmt_with_options(source, &options), source);
}

#[test]
fn javadoc_is_formatted_only_where_it_documents_a_declaration() {
    let source = concat!(
        "class Probe {\n",
        "  void m() {\n",
        "    /** dangling   inside   a method body\n",
        "     * @exception Foo bar */\n",
        "    x();\n",
        "    /**\n",
        "     *\n",
        "     */\n",
        "    y();\n",
        "  }\n",
        "\n",
        "  /** dangling before another doc   comment */\n",
        "  /** attached   one */\n",
        "  int f;\n",
        "  /*** banner   style   doc   comment */\n",
        "  int g;\n",
        "  /// separated   by a line comment\n",
        "  // note\n",
        "  int h;\n",
        "}\n",
    );

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class Probe {
      void m() {
        /** dangling   inside   a method body
         * @exception Foo bar */
        x();
        /**
         *
         */
        y();
      }

      /** dangling before another doc   comment */
      /** attached one */
      int f;
      /** banner style doc comment */
      int g;
      /// separated by a line comment
      // note
      int h;
    }
    ");
}

#[test]
fn javadoc_with_escaped_delimiters_stays_as_written() {
    // Rewritten, `/** a *\u002f` would end before its text.
    let source = concat!(
        r"\u002f** escaped   opener */",
        "\nclass A {\n",
        r"  /** escaped   closer *\u002f",
        "\n  int f;\n\n",
        r"  \u002f// escaped   line",
        "\n  int g;\n\n  /// literal   line\n",
        r"  \u002f// escaped   line",
        "\n  int h;\n\n",
        r"  \u002f** {@snippet :",
        "\n   call(  x );\n   }\n   */\n  void m() {}\n}\n",
    );

    assert_eq!(fmt_with_options(source, &settings(JavadocOptions::default())), source);
}

#[test]
fn compact_empty_javadoc_takes_precedence_without_reformatting() {
    let source = concat!(
        "class A {\n",
        "  /** earlier   field comment */\n",
        "  /**/\n",
        "  int x;\n",
        "\n",
        "  /** instance   initializer */\n",
        "  {}\n",
        "\n",
        "  /** static   initializer */\n",
        "  static {}\n",
        "\n",
        "  /** active   field comment */\n",
        "  /* ordinary comment */\n",
        "  int y;\n",
        "}\n",
    );

    assert_eq!(fmt_with_options(source, &Options::default()), source);
    assert_eq!(
        fmt_with_options(source, &settings(JavadocOptions::default())),
        source.cow_replace("active   field comment", "active field comment")
    );
}

#[test]
fn markdown_javadoc_before_declarations_is_rewritten() {
    let source = concat!(
        "///Class   doc.\n",
        "@Deprecated\n",
        "class A {\n",
        "///Field   doc.\n",
        "int a;\n",
        "///Blank   line.\n",
        "\n",
        "int b;\n",
        "///Method   doc.\n",
        "public void m() {\n",
        "///Local   class.\n",
        "class L {}\n",
        "}\n",
        "///Generic   method.\n",
        "<T> T g() { return null; }\n",
        "///Generic   constructor.\n",
        "< T > A() {}\n",
        "}\n",
    );
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    /// Class doc.
    @Deprecated
    class A {
      /// Field doc.
      int a;
      /// Blank line.

      int b;

      /// Method doc.
      public void m() {
        /// Local class.
        class L {}
      }

      /// Generic method.
      <T> T g() {
        return null;
      }

      /// Generic constructor.
      <T> A() {}
    }
    ");
}

#[test]
fn markdown_javadoc_before_an_annotated_package_is_rewritten() {
    let source = "///Package   doc.\n@NullMarked\npackage p;\n";
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };

    assert_eq!(fmt_with_options(source, &options), "/// Package doc.\n@NullMarked\npackage p;\n");
}

#[test]
fn markdown_javadoc_ending_in_less_than_documents_the_generic_member() {
    let source = "class A {\n///Less   than <\n<T> T g() { return null; }\n\
                  ///Less   than <\n<T> A() {}\n}\n";
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /// Less than <
      <T> T g() {
        return null;
      }

      /// Less than <
      <T> A() {}
    }
    ");
}

#[test]
fn markdown_javadoc_wraps_plain_prose_and_preserves_markdown_lines() {
    let source = concat!(
        "class A {\n",
        "/// This plain paragraph has enough words to wrap across several physical lines.\n",
        "/// It remains one Markdown paragraph.\n",
        "///\n",
        "/// # Heading\n",
        "/// - [List] item\n",
        "/// ```java\n",
        "/// call(  value );\n",
        "/// ```\n",
        "int value;\n",
        "}\n",
    );
    let options = Options {
        line_width: line_width(52),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /// This plain paragraph has enough words to wrap
      /// across several physical lines. It remains one
      /// Markdown paragraph.
      ///
      /// # Heading
      /// - [List] item
      /// ```java
      /// call(value);
      /// ```
      int value;
    }
    ");
}

#[test]
fn javadoc_keeps_at_words_in_prose() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Send questions to the team box @support for help.\n",
        " * @since the first release of the library @version one\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let options = Options {
        line_width: line_width(40),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Send questions to the team
       * box @support for help.
       *
       * @since the first release of the
       *     library @version one
       */
      int value;
    }
    ");
}

#[test]
fn javadoc_keeps_at_words_after_forced_boundaries_in_prose() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Before.<p>@support answers questions.\n",
        " * <pre>\n",
        " * code\n",
        " * </pre> @support answers more.\n",
        " * @since one<p>@version is prose here.\n",
        " * @deprecated a real tag\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let options = Options {
        line_width: line_width(40),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Before.
       *
       * <p>@support answers questions.
       *
       * <pre>
       * code
       * </pre> @support answers more.
       *
       * @since one
       *     <p>@version is prose here.
       * @deprecated a real tag
       */
      int value;
    }
    ");
}

#[test]
fn markdown_javadoc_keeps_markup_whose_rewrite_would_change_its_meaning() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    let formatted = "// Underscores would make one emphasis around one__two.
/// _This_ uses underscores, but *one*_two_ keeps its asterisks.
class Emphasis {}

// One marker would merge the two lists.
/// - first list
///
/// * second list
class Lists {}

// 1000000000 is too long for a list marker.
/// 999999999. last number a marker allows
/// 999999999. second item
class Numbers {}

// A backtick fence cannot have a backtick in its info string.
/// ~~~ text title=`a`
/// kept fence
/// ~~~
class Fence {}
";
    assert_eq!(fmt_with_options(formatted, &options), formatted);

    let source = [
        ("/// _This_", "/// *This*"),
        ("/// - first", "/// + first"),
        ("/// * second", "/// - second"),
        ("999999999.", "999999999)"),
    ]
    .into_iter()
    .fold(formatted.to_owned(), |text, (from, to)| text.cow_replace(from, to).into_owned());
    assert_eq!(fmt_with_options(&source, &options), formatted);
}

#[test]
fn markdown_javadoc_keeps_line_start_markers_in_prose() {
    let options = Options {
        line_width: line_width(40),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };

    for marker in [
        "@since", "-", "+", "*", "#", "######", "1.", "1)", ">", ">=", "```", "~~~", "---", "===",
        "___", "***", "* * *", "_ _ _", "<div>", "<!--",
    ] {
        for suffix in ["", " for this example."] {
            let source = format!(
                "class A {{\n/// Values are positive or negative {marker}{suffix}\nint value;\n}}\n"
            );
            let formatted = format_stable_with_options(&source, &options);
            assert!(formatted.contains(&format!("negative {marker}")), "{marker}:\n{formatted}");
            assert!(!formatted.contains(&format!("/// {marker}")), "{marker}:\n{formatted}");
        }
    }

    let source = concat!(
        "class A {\n",
        "/// Values are positive or negative - or + for sign.\n",
        "/// @since the first release of the library @version one\n",
        "int value;\n",
        "}\n",
    );
    let formatted = format_stable_with_options(source, &options);
    assert!(formatted.contains("negative -"));
    assert!(formatted.contains("or +"));
    assert!(formatted.contains("/// @since"));
    assert!(formatted.contains("library @version"));
}

#[test]
fn markdown_javadoc_preserves_an_indented_marker_after_a_link_definition() {
    let source = concat!(
        "class A {\n",
        "/// Text *b*.\n",
        "///\n",
        "///   [a]: b\n",
        "///     > >     @Override\n",
        "/// @since 3\n",
        "/// @return the value\n",
        "int m() { return 0; }\n",
        "}\n",
    );
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /// Text _b_.
      ///
      ///   [a]: b
      ///     > >     @Override
      ///
      /// @return the value
      /// @since 3
      int m() {
        return 0;
      }
    }
    ");
}

#[test]
fn javadoc_keeps_indivisible_hazard_runs_even_when_they_exceed_the_width() {
    let options = Options {
        line_width: line_width(40),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Contact extraordinarily_long_team_name @support for help.\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let formatted = fmt_with_options(source, &options);
    assert!(formatted.contains("extraordinarily_long_team_name @support"));

    let widths = robusta_formatter::check_line_width(&formatted, &options);
    assert_eq!(widths.wide_lines().len(), 1);
    assert_eq!(widths.wide_lines()[0].cause, robusta_formatter::WideLineCause::Comment);
}

fn settings(javadoc: JavadocOptions) -> Options {
    Options { javadoc: Some(javadoc), ..Options::default() }
}

#[test]
fn capitalize_descriptions_capitalizes_paragraphs_but_not_tag_descriptions() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * returns the label.\n",
        " * <p>\n",
        " * second paragraph.\n",
        " *\n",
        " * {@code label} stays as written.\n",
        " *\n",
        " * @return the label of this button\n",
        " */\n",
        "String label() { return null; }\n",
        "/// returns the label.\n",
        "///\n",
        "/// * first item\n",
        "/// * [List] item\n",
        "///\n",
        "/// @return the label\n",
        "String markdown() { return null; }\n",
        "}\n",
    );
    let options = settings(JavadocOptions { capitalize_descriptions: true, ..Default::default() });

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Returns the label.
       *
       * <p>Second paragraph.
       *
       * {@code label} stays as written.
       *
       * @return the label of this button
       */
      String label() {
        return null;
      }

      /// Returns the label.
      ///
      /// - First item
      /// - [List] item
      ///
      /// @return the label
      String markdown() {
        return null;
      }
    }
    ");

    let example = concat!(
        "/**\n",
        " * returns the label of this button.\n",
        " *\n",
        " * @return the label of this button\n",
        " */\n",
        "class A {}\n",
    );
    assert_snapshot!(fmt_with_options(example, &options), @"
    /**
     * Returns the label of this button.
     *
     * @return the label of this button
     */
    class A {}
    ");
}

#[test]
fn description_with_dot_summary_ends_only_the_summary() {
    let source = concat!(
        "/**\n",
        " * Returns the label of this button\n",
        " *\n",
        " * <p>The label is never null\n",
        " *\n",
        " * @return the label of this button\n",
        " */\n",
        "class A {}\n",
    );
    for (dot, summary, rest) in [
        (DescriptionWithDot::Never, "", ""),
        (DescriptionWithDot::Summary, ".", ""),
        (DescriptionWithDot::Always, ".", "."),
    ] {
        let options = settings(JavadocOptions { description_with_dot: dot, ..Default::default() });
        let expected = format!(
            "/**\n * Returns the label of this button{summary}\n *\n \
             * <p>The label is never null{rest}\n *\n \
             * @return the label of this button\n */\nclass A {{}}\n"
        );
        assert_eq!(fmt_with_options(source, &options), expected, "{dot:?}");
    }
}

#[test]
fn description_with_dot_summary_ends_where_javadoc_ends_it() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Returns the label. It is never null\n",
        " */\n",
        "int periodFirst;\n",
        "/**\n",
        " * Returns e.g. the label\n",
        " */\n",
        "int abbreviation;\n",
        "/**\n",
        " * Returns the label\n",
        " *\n",
        " * It is never null\n",
        " *\n",
        " * @return the label\n",
        " */\n",
        "int noParagraphTag;\n",
        "/**\n",
        " * Returns the label\n",
        " * @return the label\n",
        " */\n",
        "int toBlockTag;\n",
        "/**\n",
        " * <p>Returns the label</p> It is never null\n",
        " */\n",
        "int endTag;\n",
        "/**\n",
        " * Returns the label {@code a}\n",
        " */\n",
        "int inlineTag;\n",
        "/**\n",
        " * Returns the label {@code a. b} as text\n",
        " */\n",
        "int periodInCode;\n",
        "/**\n",
        " * Requires a<p and p is positive\n",
        " */\n",
        "int lessThan;\n",
        "/**\n",
        " * Returns the label. @hidden stays in the text\n",
        " */\n",
        "int atSignInText;\n",
        "/**\n",
        " * {@return the label} It is never null\n",
        " */\n",
        "int returnTag;\n",
        "/**\n",
        " * {@summary Returns the label} It is never null\n",
        " */\n",
        "int summaryTag;\n",
        "/// Returns the label\n",
        "///\n",
        "/// It is never null\n",
        "int markdown;\n",
        "/// Returns the label\n",
        "/// - first item\n",
        "int markdownList;\n",
        "}\n",
    );
    let options = settings(JavadocOptions {
        description_with_dot: DescriptionWithDot::Summary,
        ..Default::default()
    });

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /** Returns the label. It is never null */
      int periodFirst;
      /** Returns e.g. the label */
      int abbreviation;
      /**
       * Returns the label
       *
       * It is never null.
       *
       * @return the label
       */
      int noParagraphTag;
      /**
       * Returns the label.
       *
       * @return the label
       */
      int toBlockTag;
      /** <p>Returns the label.</p> It is never null */
      int endTag;
      /** Returns the label {@code a} */
      int inlineTag;
      /** Returns the label {@code a. b} as text. */
      int periodInCode;
      /** Requires a<p and p is positive. */
      int lessThan;
      /** Returns the label. @hidden stays in the text */
      int atSignInText;
      /** {@return the label} It is never null */
      int returnTag;
      /** {@summary Returns the label} It is never null */
      int summaryTag;
      /// Returns the label.
      ///
      /// It is never null
      int markdown;
      /// Returns the label.
      ///
      /// - first item
      int markdownList;
    }
    ");
}

#[test]
fn description_with_dot_summary_ends_at_inserted_paragraph_tags() {
    let source = "/**\n * Returns the label\n *\n * It is never null\n */\nclass A {}\n";
    let options = settings(JavadocOptions {
        description_with_dot: DescriptionWithDot::Summary,
        paragraph_html_tags: true,
        ..Default::default()
    });

    assert_eq!(
        fmt_with_options(source, &options),
        "/**\n * Returns the label.\n *\n * <p>It is never null\n */\nclass A {}\n"
    );
}

#[test]
fn description_with_dot_ends_paragraphs_but_not_tag_descriptions() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Returns the label\n",
        " * <p>\n",
        " * Second paragraph 2\n",
        " *\n",
        " * Ends with {@code label}\n",
        " *\n",
        " * Ends with punctuation:\n",
        " * @return the label of this button\n",
        " */\n",
        "String label() { return null; }\n",
        "/// Returns the label\n",
        "///\n",
        "/// @return the label\n",
        "String markdown() { return null; }\n",
        "}\n",
    );
    let options = settings(JavadocOptions {
        description_with_dot: DescriptionWithDot::Always,
        ..Default::default()
    });

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Returns the label.
       *
       * <p>Second paragraph 2.
       *
       * Ends with {@code label}
       *
       * Ends with punctuation:
       *
       * @return the label of this button
       */
      String label() {
        return null;
      }

      /// Returns the label.
      ///
      /// @return the label
      String markdown() {
        return null;
      }
    }
    ");
}

#[test]
fn comment_line_strategy_decides_the_single_line_form() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Short.\n",
        " */\n",
        "int multiline;\n",
        "/** Short. */\n",
        "int single;\n",
        "/** @return a value */\n",
        "int tagged() { return 0; }\n",
        "/**\n",
        " * A description that does not fit on a single line together with its delimiters.\n",
        " */\n",
        "int wide;\n",
        "int trailing; /**   after   code */\n",
        "}\n",
    );
    let format = |comment_line_strategy| {
        let javadoc = JavadocOptions { comment_line_strategy, ..Default::default() };
        let options = Options { line_width: line_width(60), ..settings(javadoc) };
        fmt_with_options(source, &options)
    };

    assert_snapshot!(format(CommentLineStrategy::SingleLine), @"
    class A {
      /** Short. */
      int multiline;
      /** Short. */
      int single;

      /**
       * @return a value
       */
      int tagged() {
        return 0;
      }

      /**
       * A description that does not fit on a single line
       * together with its delimiters.
       */
      int wide;
      int trailing; /**   after   code */
    }
    ");
    assert_snapshot!(format(CommentLineStrategy::Multiline), @"
    class A {
      /**
       * Short.
       */
      int multiline;
      /**
       * Short.
       */
      int single;

      /**
       * @return a value
       */
      int tagged() {
        return 0;
      }

      /**
       * A description that does not fit on a single line
       * together with its delimiters.
       */
      int wide;
      int trailing; /**   after   code */
    }
    ");
    assert_snapshot!(format(CommentLineStrategy::Keep), @"
    class A {
      /**
       * Short.
       */
      int multiline;
      /** Short. */
      int single;

      /**
       * @return a value
       */
      int tagged() {
        return 0;
      }

      /**
       * A description that does not fit on a single line
       * together with its delimiters.
       */
      int wide;
      int trailing; /**   after   code */
    }
    ");

    let example = "/**\n * Returns the label of this button.\n */\nclass A {}\n";
    assert_snapshot!(fmt_with_options(example, &settings(JavadocOptions::default())), @"
    /** Returns the label of this button. */
    class A {}
    ");
}

#[test]
fn single_line_comments_are_rewritten_only_without_code_on_their_line() {
    let source = concat!(
        "class A {\n",
        "int a; /** moved   before b */ int b;\n",
        "/** beside   c */ int c;\n",
        "/** above   d */\n",
        "int d;\n",
        "int e; /** after   e */\n",
        "int f;\n",
        "}\n",
    );
    let javadoc = JavadocOptions {
        comment_line_strategy: CommentLineStrategy::Multiline,
        ..Default::default()
    };

    assert_snapshot!(fmt_with_options(source, &settings(javadoc)), @"
    class A {
      int a;
      /** moved   before b */ int b;
      /** beside   c */ int c;
      /**
       * above d
       */
      int d;
      int e; /** after   e */
      int f;
    }
    ");
}

#[test]
fn line_wrapping_style_keep_keeps_breaks_only_when_every_line_fits() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * This description was wrapped by hand\n",
        " * into short lines that all fit.\n",
        " *\n",
        " * This paragraph has one line that is much too long for the width\n",
        " * and a short one.\n",
        " *\n",
        " * @param value kept tag\n",
        " * description lines\n",
        " */\n",
        "void m(int value) {}\n",
        "/// This description was wrapped by hand\n",
        "/// into short lines that all fit.\n",
        "void markdown() {}\n",
        "}\n",
    );
    let keep =
        JavadocOptions { line_wrapping_style: LineWrappingStyle::Keep, ..Default::default() };
    let options = Options { line_width: line_width(60), ..settings(keep) };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * This description was wrapped by hand
       * into short lines that all fit.
       *
       * This paragraph has one line that is much too long for
       * the width and a short one.
       *
       * @param value kept tag
       *     description lines
       */
      void m(int value) {}

      /// This description was wrapped by hand
      /// into short lines that all fit.
      void markdown() {}
    }
    ");

    let greedy = Options { line_width: line_width(60), ..settings(JavadocOptions::default()) };
    assert_snapshot!(fmt_with_options(source, &greedy), @"
    class A {
      /**
       * This description was wrapped by hand into short lines
       * that all fit.
       *
       * This paragraph has one line that is much too long for
       * the width and a short one.
       *
       * @param value kept tag description lines
       */
      void m(int value) {}

      /// This description was wrapped by hand into short lines
      /// that all fit.
      void markdown() {}
    }
    ");

    let example = concat!(
        "/**\n",
        " * This description was wrapped by hand\n",
        " * into short lines that all fit.\n",
        " */\n",
        "class A {}\n",
    );
    let multiline = JavadocOptions {
        comment_line_strategy: CommentLineStrategy::Multiline,
        ..Default::default()
    };
    assert_snapshot!(fmt_with_options(example, &settings(multiline)), @"
    /**
     * This description was wrapped by hand into short lines that all fit.
     */
    class A {}
    ");
}

#[test]
fn line_wrapping_style_keep_measures_lines_at_the_output_indentation() {
    let source = concat!(
        "class A {\n",
        "class B {\n",
        "/**\n",
        " * This source line fits at column zero only.\n",
        " * Short.\n",
        " */\n",
        "int value;\n",
        "}\n",
        "}\n",
    );
    let keep =
        JavadocOptions { line_wrapping_style: LineWrappingStyle::Keep, ..Default::default() };
    let options = Options { line_width: line_width(50), ..settings(keep) };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      class B {
        /**
         * This source line fits at column zero only.
         * Short.
         */
        int value;
      }
    }
    ");
}

#[test]
fn separate_tag_groups_decides_blank_lines_between_block_tags() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Reads one entry.\n",
        " *\n",
        " *\n",
        " * @param index the position to read\n",
        " *\n",
        " * @param count the number to read\n",
        " * @return the entry at that position\n",
        " * @throws IndexOutOfBoundsException if the position is out of range\n",
        " */\n",
        "Object read(int index, int count) { return null; }\n",
        "}\n",
    );
    let format = |separate_tag_groups| {
        fmt_with_options(
            source,
            &settings(JavadocOptions { separate_tag_groups, ..Default::default() }),
        )
    };

    assert_snapshot!(format(SeparateTagGroups::Never), @"
    class A {
      /**
       * Reads one entry.
       *
       * @param index the position to read
       * @param count the number to read
       * @return the entry at that position
       * @throws IndexOutOfBoundsException if the position is out of range
       */
      Object read(int index, int count) {
        return null;
      }
    }
    ");
    assert_snapshot!(format(SeparateTagGroups::BeforeReturn), @"
    class A {
      /**
       * Reads one entry.
       *
       * @param index the position to read
       * @param count the number to read
       *
       * @return the entry at that position
       * @throws IndexOutOfBoundsException if the position is out of range
       */
      Object read(int index, int count) {
        return null;
      }
    }
    ");
    assert_snapshot!(format(SeparateTagGroups::Always), @"
    class A {
      /**
       * Reads one entry.
       *
       * @param index the position to read
       * @param count the number to read
       *
       * @return the entry at that position
       *
       * @throws IndexOutOfBoundsException if the position is out of range
       */
      Object read(int index, int count) {
        return null;
      }
    }
    ");

    let example = concat!(
        "/**\n",
        " * Reads one entry.\n",
        " *\n",
        " * @param index the position to read\n",
        " * @return the entry at that position\n",
        " * @throws IndexOutOfBoundsException if the position is out of range\n",
        " */\n",
        "class A {}\n",
    );
    let format = |separate_tag_groups| {
        fmt_with_options(
            example,
            &settings(JavadocOptions { separate_tag_groups, ..Default::default() }),
        )
    };
    assert_snapshot!(format(SeparateTagGroups::Always), @"
    /**
     * Reads one entry.
     *
     * @param index the position to read
     *
     * @return the entry at that position
     *
     * @throws IndexOutOfBoundsException if the position is out of range
     */
    class A {}
    ");
    assert_snapshot!(format(SeparateTagGroups::BeforeReturn), @"
    /**
     * Reads one entry.
     *
     * @param index the position to read
     *
     * @return the entry at that position
     * @throws IndexOutOfBoundsException if the position is out of range
     */
    class A {}
    ");
}

#[test]
fn markdown_javadoc_separates_the_description_from_block_tags() {
    let source = concat!(
        "class A {\n",
        "/// Reads one entry.\n",
        "/// @param index the position to read\n",
        "/// @return the entry at that position\n",
        "Object read(int index) { return null; }\n",
        "}\n",
    );
    let options = settings(JavadocOptions {
        separate_tag_groups: SeparateTagGroups::BeforeReturn,
        ..Default::default()
    });

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /// Reads one entry.
      ///
      /// @param index the position to read
      ///
      /// @return the entry at that position
      Object read(int index) {
        return null;
      }
    }
    ");
}

#[test]
fn markdown_javadoc_normalizes_its_markup() {
    let source = concat!(
        "class A {\n",
        "/// Reads *one* __entry__ from a*b*c.\n",
        "/// The steps:\n",
        "/// 1) find it\n",
        "/// 5) read it\n",
        "///\n",
        "/// Details\n",
        "/// ---\n",
        "///\n",
        "/// |key|value|\n",
        "/// |:-|-|\n",
        "/// |a|`x\\|y`|\n",
        "///\n",
        "///     read(  index );\n",
        "///\n",
        "/// ***\n",
        "///\n",
        "/// See [java.util.List] and {@code *raw*}.\n",
        "///\n",
        "/// @param index the *position* to read\n",
        "Object read(int index) { return null; }\n",
        "}\n",
    );

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @r"
    class A {
      /// Reads _one_ **entry** from a*b*c.
      /// The steps:
      ///
      /// 1. find it
      /// 2. read it
      ///
      /// ## Details
      ///
      /// | key | value  |
      /// | :-- | ------ |
      /// | a   | `x\|y` |
      ///
      /// ```
      /// read(index);
      /// ```
      ///
      /// ***
      ///
      /// See [java.util.List] and {@code *raw*}.
      ///
      /// @param index the _position_ to read
      Object read(int index) {
        return null;
      }
    }
    ");
}

#[test]
fn inline_tag_name_and_reference_stay_together() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Delegates to {@link java.util.Map#computeIfAbsent(Object, Function) the map} here.\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let options = Options { line_width: line_width(50), ..settings(JavadocOptions::default()) };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Delegates to
       * {@link java.util.Map#computeIfAbsent(Object,
       * Function) the map} here.
       */
      int value;
    }
    ");
}

#[test]
fn block_tags_follow_the_documented_order() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Summary.\n",
        " *\n",
        " * @custom stays first\n",
        " * @throws ZipException second\n",
        " * @exception IOException first\n",
        " * @return the result\n",
        " * @see Other\n",
        " * @param b second\n",
        " * @apiNote follows the parameters\n",
        " * @param a first\n",
        " * @param <T> the type\n",
        " * @author someone\n",
        " * @deprecated use another\n",
        " * @serialData the data\n",
        " * @since 1.0\n",
        " * @see Another\n",
        " */\n",
        "<T> int m(int a, int b) throws Exception { return 0; }\n",
        "}\n",
    );

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      /**
       * Summary.
       *
       * @custom stays first
       * @author someone
       * @param <T> the type
       * @param a first
       * @param b second
       * @apiNote follows the parameters
       * @return the result
       * @throws IOException first
       * @throws ZipException second
       * @see Other
       * @see Another
       * @since 1.0
       * @serialData the data
       * @deprecated use another
       */
      <T> int m(int a, int b) throws Exception {
        return 0;
      }
    }
    ");
}

#[test]
fn param_tags_follow_the_declaration_only_when_they_name_it_exactly() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * @param <V> value\n",
        " * @param <K> key\n",
        " */\n",
        "class Pair<K, V> {}\n",
        "/**\n",
        " * @param second the second\n",
        " * @param <T> the type\n",
        " * @param first the first\n",
        " */\n",
        "record Entry<T>(T first, T second) {}\n",
        "/**\n",
        " * @param b missing a\n",
        " * @param c not declared\n",
        " */\n",
        "A(int a, int b) {}\n",
        "/**\n",
        " * @param b named twice\n",
        " * @param b again\n",
        " */\n",
        "void twice(int a, int b) {}\n",
        "/**\n",
        " * @param <T> tagged as a type parameter\n",
        " * @param a first\n",
        " */\n",
        "void untyped(int a, int T) {}\n",
        "/**\n",
        " * @param b second\n",
        " * @param a first\n",
        " */\n",
        "int field;\n",
        "}\n",
    );

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      /**
       * @param <K> key
       * @param <V> value
       */
      class Pair<K, V> {}

      /**
       * @param <T> the type
       * @param first the first
       * @param second the second
       */
      record Entry<T>(T first, T second) {}

      /**
       * @param b missing a
       * @param c not declared
       */
      A(int a, int b) {}

      /**
       * @param b named twice
       * @param b again
       */
      void twice(int a, int b) {}

      /**
       * @param <T> tagged as a type parameter
       * @param a first
       */
      void untyped(int a, int T) {}

      /**
       * @param b second
       * @param a first
       */
      int field;
    }
    ");
}

#[test]
fn param_tags_name_type_parameters_spelled_like_block_html() {
    let source = concat!(
        "abstract class A {\n",
        "/**\n",
        " * Finds one.\n",
        " *\n",
        " * @param c the class\n",
        " * <p>Any class.\n",
        " * @param <P> the provider type\n",
        " */\n",
        "abstract <P> P find(Class<P> c);\n",
        "/**\n",
        " * @param <H1>\n",
        " * @param <LI> the item type\n",
        " */\n",
        "abstract <LI, H1> void m();\n",
        "/**\n",
        " * @return the count\n",
        " * @param <PRE> the prefix type\n",
        " */\n",
        "abstract <PRE> int n();\n",
        "}\n",
    );
    let format = |paragraph_html_tags| {
        let options = JavadocOptions { paragraph_html_tags, ..Default::default() };
        fmt_with_options(source, &settings(options))
    };

    let formatted = format(false);
    assert_eq!(format(true), formatted);
    assert_snapshot!(formatted, @"
    abstract class A {
      /**
       * Finds one.
       *
       * @param <P> the provider type
       * @param c the class
       *     <p>Any class.
       */
      abstract <P> P find(Class<P> c);

      /**
       * @param <LI> the item type
       * @param <H1>
       */
      abstract <LI, H1> void m();

      /**
       * @param <PRE> the prefix type
       * @return the count
       */
      abstract <PRE> int n();
    }
    ");
}

#[test]
fn param_tags_name_type_parameters_on_the_next_line() {
    let source = concat!(
        "abstract class A {\n",
        "/**\n",
        " * Finds one.\n",
        " *\n",
        " * @param\n",
        " *     <P> the provider type\n",
        " * @param c the class\n",
        " */\n",
        "abstract <P> P find(Class<P> c);\n",
        "/**\n",
        " * @return the count\n",
        " * @param\n",
        " *     <PRE> the prefix type\n",
        " * @param\n",
        " *\n",
        " *     <TABLE> the table type\n",
        " */\n",
        "abstract <TABLE, PRE> int n();\n",
        "/// @param\n",
        "/// <PRE> the prefix type\n",
        "/// @return the *count*\n",
        "abstract <PRE> int m();\n",
        "}\n",
    );
    let format = |paragraph_html_tags| {
        let options = JavadocOptions { paragraph_html_tags, ..Default::default() };
        fmt_with_options(source, &settings(options))
    };

    let formatted = format(false);
    assert_eq!(format(true), formatted);
    assert_snapshot!(formatted, @"
    abstract class A {
      /**
       * Finds one.
       *
       * @param <P> the provider type
       * @param c the class
       */
      abstract <P> P find(Class<P> c);

      /**
       * @param <TABLE> the table type
       * @param <PRE> the prefix type
       * @return the count
       */
      abstract <TABLE, PRE> int n();

      /// @param
      /// <PRE> the prefix type
      /// @return the _count_
      abstract <PRE> int m();
    }
    ");
}

#[test]
fn block_tag_continuation_ignores_indent_width() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * @param value a description long enough to wrap beneath the tag\n",
        " */\n",
        "void traditional(int value) {}\n",
        "/// @param value a description long enough to wrap beneath the tag\n",
        "void markdown(int value) {}\n",
        "}\n",
    );
    let options = Options {
        line_width: line_width(50),
        indent_width: indent_width(8),
        ..settings(JavadocOptions::default())
    };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
            /**
             * @param value a description long enough
             *     to wrap beneath the tag
             */
            void traditional(int value) {}

            /// @param value a description long enough
            ///   to wrap beneath the tag
            void markdown(int value) {}
    }
    ");
}

#[test]
fn tag_ordering_leaves_code_and_literal_text_alone() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * {@code @return} stays in the description.\n",
        " *\n",
        " * @return the value, as in\n",
        " * <pre>\n",
        " * @param b not a tag\n",
        " * @author nobody\n",
        " * </pre>\n",
        " * @param a first\n",
        " */\n",
        "int m(int a) { return a; }\n",
        "/// Example:\n",
        "///\n",
        "/// ```java\n",
        "/// @return not a tag\n",
        "/// @param a not a tag\n",
        "/// ```\n",
        "///\n",
        "/// @return the value\n",
        "/// @param a first\n",
        "int n(int a) { return a; }\n",
        "}\n",
    );

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      /**
       * {@code @return} stays in the description.
       *
       * @param a first
       * @return the value, as in
       *     <pre>
       * @param b not a tag
       * @author nobody
       * </pre>
       */
      int m(int a) {
        return a;
      }

      /// Example:
      ///
      /// ```java
      /// @return not a tag
      /// @param a not a tag
      /// ```
      ///
      /// @param a first
      /// @return the value
      int n(int a) {
        return a;
      }
    }
    ");
}

#[test]
fn paragraph_before_backticks_formats_in_one_pass() {
    for indent in ["", " ", "  ", "\t"] {
        let source = format!(
            "class A {{ /** Text.\n * {indent}<p>\n * {indent}```\n *\n * @param <T> data\n */ <T> void f() {{}} }}"
        );

        insta::allow_duplicates! { assert_snapshot!(fmt_with_options(&source, &settings(JavadocOptions::default())), @"
        class A {
          /**
           * Text.
           *
           * <p>```
           *
           * @param <T> data
           */ <T> void f() {}
        }
        "); }
    }
}

#[test]
fn html_start_tags_keep_fenced_literal_content() {
    for marker in ["```", "~~~"] {
        for tag in ["<p>", "<p class=\"note\">", "<li>", "<h2>"] {
            for indent in ["", " ", "  ", "\t"] {
                for separator in ["", "\n * "] {
                    let source = format!(
                        "class A {{ /** Text.\n * {tag}{separator}{indent}{marker}other\n *   code  stays\n *   @return literal\n * {marker}\n *\n * @param <T> data\n */ <T> void f() {{}} }}"
                    );
                    let formatted =
                        format_stable_with_options(&source, &settings(JavadocOptions::default()));
                    assert!(formatted.contains(&format!("{tag}{marker}other")));
                    assert!(formatted.contains(" *   code  stays\n"));
                    assert!(formatted.contains(" *   @return literal\n"));
                    assert!(formatted.contains("@param <T> data"));
                }
            }
        }
    }
}

#[test]
fn html_boundaries_keep_fenced_literal_content_after_prose() {
    for marker in ["```", "~~~"] {
        for tag in ["<p>", "<li>", "<h2>", "<blockquote>"] {
            let close = if tag == "<blockquote>" { " * </blockquote>\n" } else { "" };
            for prefix in ["Text. ", "Text.\n * "] {
                for indent in ["", " ", "  ", "\t"] {
                    let source = format!(
                        "class A {{ /** {prefix}{tag}{indent}{marker}other\n *   code  stays\n *   @return literal\n * {marker}\n{close} *\n * @param value data\n */ void f(int value) {{}} }}"
                    );
                    let formatted =
                        format_stable_with_options(&source, &settings(JavadocOptions::default()));
                    assert!(formatted.contains(" *   code  stays\n"));
                    assert!(formatted.contains(" *   @return literal\n"));
                    assert!(formatted.contains("@param value data"));
                }
            }
        }
    }
}

#[test]
fn multiline_html_tags_keep_fenced_literal_content() {
    let source = "class A { /** Summary. <p class=\"note\"\n * >```other\n *   code  stays\n *   @return literal\n * ```\n * @param value data\n */ void f(int value) {} }";
    let options = Options { line_width: line_width(60), ..settings(JavadocOptions::default()) };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class A {
      /**
       * Summary.
       *
       * <p class="note" >```other
       *   code  stays
       *   @return literal
       * ```
       *
       * @param value data
       */ void f(int value) {}
    }
    "#);

    for marker in ["```", "~~~"] {
        for (tag, suffix) in [
            ("<p class=\"note\"\n * >", ""),
            ("<p\n * class=\"note\"\n * >", ""),
            ("<ul\n * class=\"note\"\n * >", "</ul>"),
            ("<li class=\"note\"\n * >", ""),
            ("<h2\n * class=\"note\"\n * >", "</h2>"),
            ("<blockquote\n * cite=example\n * >", "</blockquote>"),
            ("<p\n * title='note > value'\n * >", ""),
            ("<li\n * title='note > value'\n * >", ""),
            ("<h2\n * title='note > value'\n * >", "</h2>"),
            ("<p title='note > value'>", ""),
            ("<li title='note > value'>", ""),
            ("<h2 title='note > value'>", "</h2>"),
        ] {
            for prefix in ["Summary. ", "Summary.\n * "] {
                for margin in ["", " ", "  ", "\t", "\n * ", "\n *  ", "\n *   ", "\n * \t"] {
                    let source = format!(
                        "class A {{ /** {prefix}{tag}{margin}{marker}other\n *   code  stays\n *   @return literal\n * {marker}\n * {suffix}\n * @param value data\n */ void f(int value) {{}} }}"
                    );
                    for crlf in [false, true] {
                        let source = if crlf {
                            source.cow_replace('\n', "\r\n").into_owned()
                        } else {
                            source.clone()
                        };
                        for width in [40, 60, 100] {
                            for style in [LineWrappingStyle::Greedy, LineWrappingStyle::Keep] {
                                let options = Options {
                                    line_width: line_width(width),
                                    ..settings(JavadocOptions {
                                        line_wrapping_style: style,
                                        ..JavadocOptions::default()
                                    })
                                };
                                let formatted = format_stable_with_options(&source, &options);
                                assert!(
                                    formatted.contains(" *   code  stays"),
                                    "{source:?} => {formatted:?}"
                                );
                                assert!(formatted.contains(" *   @return literal"));
                                assert!(formatted.contains(" * @param value data"));
                                if tag.contains("note > value") {
                                    assert!(formatted.contains("note > value"));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn list_fence_openers_include_their_preserved_margin() {
    let source = "class A { /** Summary. <ul>```other\n *   code  stays\n *   @return literal\n * ```\n * @param value data\n */ void f(int value) {} }";
    let options = Options { line_width: line_width(40), ..settings(JavadocOptions::default()) };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Summary.
       *
       * <ul>
       *   ```other
       *   code  stays
       *   @return literal
       * ```
       *
       * @param value data
       */ void f(int value) {}
    }
    ");
}

#[test]
fn list_fences_after_blank_lines_use_paragraph_margins() {
    let source = "class A { /** Summary. <ul><li>Item.\n *\n *  ```other\n *   code  stays\n *   @return literal\n * ```\n * </ul>\n * @param value data\n */ void f(int value) {} }";
    let options = Options { line_width: line_width(40), ..settings(JavadocOptions::default()) };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Summary.
       *
       * <ul>
       *   <li>Item.
       *
       *       ```other
       *   code  stays
       *   @return literal
       * ```
       * </ul>
       *
       * @param value data
       */ void f(int value) {}
    }
    ");
}

#[test]
fn html_lists_keep_fenced_literal_margins_stable() {
    for marker in ["```", "~~~"] {
        for (prefix, suffix) in [
            ("<ul>", "</ul>"),
            ("<ol>", "</ol>"),
            ("<dl>", "</dl>"),
            ("<ul><li>Item.", "</ul>"),
            ("<ol><li>", "</ol>"),
            ("<dl><dt>Term.<dd>", "</dl>"),
            ("<ul><li>Item.<ol>", "</ol></ul>"),
        ] {
            for margin in ["", " ", "  ", "    ", "\t"] {
                for separator in ["", "\n * ", "\n *\n * "] {
                    if prefix.ends_with('.') && separator.is_empty() {
                        continue;
                    }
                    let source = format!(
                        "class A {{ /** Summary. {prefix}{separator}{margin}{marker}other\n *   code  stays\n *   @return literal\n * {marker}\n * {suffix}\n * @param value data\n */ void f(int value) {{}} }}"
                    );
                    for width in [40, 60, 100] {
                        for style in [LineWrappingStyle::Greedy, LineWrappingStyle::Keep] {
                            for paragraph_html_tags in [false, true] {
                                let options = Options {
                                    line_width: line_width(width),
                                    javadoc: Some(JavadocOptions {
                                        line_wrapping_style: style,
                                        paragraph_html_tags,
                                        ..JavadocOptions::default()
                                    }),
                                    ..Options::default()
                                };
                                let formatted = format_stable_with_options(&source, &options);
                                assert!(formatted.contains(" *   code  stays\n"));
                                assert!(formatted.contains(" *   @return literal\n"));
                                assert!(formatted.contains(" * @param value data\n"));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn paragraphs_start_with_p_before_their_first_word() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * <p>First paragraph.\n",
        " *\n",
        " * Second paragraph.\n",
        " * <P>\n",
        " * Third paragraph.<p class=\"note\">Fourth paragraph.\n",
        " *\n",
        " * <p>\n",
        " *\n",
        " * Fifth paragraph.\n",
        " */\n",
        "int value;\n",
        "}\n",
    );

    let options = settings(JavadocOptions { paragraph_html_tags: true, ..Default::default() });

    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class A {
      /**
       * First paragraph.
       *
       * <p>Second paragraph.
       *
       * <p>Third paragraph.
       *
       * <p class="note">Fourth paragraph.
       *
       * <p>Fifth paragraph.
       */
      int value;
    }
    "#);
}

#[test]
fn paragraph_html_tags_start_later_paragraphs_with_p() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Returns the label of this button.\n",
        " *\n",
        " * The label is never null.\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let options = settings(JavadocOptions { paragraph_html_tags: true, ..Default::default() });

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Returns the label of this button.
       *
       * <p>The label is never null.
       */
      int value;
    }
    ");
    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      /**
       * Returns the label of this button.
       *
       * The label is never null.
       */
      int value;
    }
    ");
}

#[test]
fn paragraph_tags_stay_as_written_by_default() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * <p>First paragraph.\n",
        " *\n",
        " * <P>\n",
        " *\n",
        " * Second paragraph.\n",
        " * <p>\n",
        " * <ul><li>one</li></ul>\n",
        " * <p>\n",
        " *\n",
        " * @param value data\n",
        " */\n",
        "void f(int value) {}\n",
        "}\n",
    );

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      /**
       * <p>First paragraph.
       *
       * <p>Second paragraph.
       *
       * <p>
       *
       * <ul>
       *   <li>one</li>
       * </ul>
       *
       * <p>
       *
       * @param value data
       */
      void f(int value) {}
    }
    ");
}

#[test]
fn paragraph_html_tags_never_precede_block_html() {
    let options = settings(JavadocOptions { paragraph_html_tags: true, ..Default::default() });
    for block in [
        "<ul><li>one</ul>",
        "<ol><li>one</ol>",
        "<dl><dt>term</dl>",
        "<table><tr><td>cell</table>",
        "<pre>code</pre>",
        "<blockquote>quote</blockquote>",
        "<h2>Heading</h2>",
    ] {
        let source = format!(
            "class A {{\n/**\n * Summary.\n *\n * {block}\n * <p>\n * {block}\n */\nint value;\n}}\n"
        );
        let formatted = format_stable_with_options(&source, &options);
        assert_eq!(formatted.matches(block.split('>').next().unwrap()).count(), 2, "{formatted}");
        assert!(!formatted.contains("<p>"), "{formatted}");
    }
}

#[test]
fn block_html_gets_blank_lines_and_no_paragraph_tag() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Summary.<h2>Usage</h2>Text after the heading.\n",
        " * <p>\n",
        " * <blockquote>Quoted text.</blockquote>\n",
        " *\n",
        " * <ul><li>one</li><li>two</li></ul> text after the list stays in its paragraph.\n",
        " */\n",
        "int value;\n",
        "}\n",
    );

    let options = settings(JavadocOptions { paragraph_html_tags: true, ..Default::default() });

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Summary.
       *
       * <h2>Usage</h2>
       *
       * Text after the heading.
       *
       * <blockquote>
       *
       * Quoted text.
       *
       * </blockquote>
       *
       * <ul>
       *   <li>one</li>
       *   <li>two</li>
       * </ul>
       *
       * text after the list stays in its paragraph.
       */
      int value;
    }
    ");
}

#[test]
fn lists_indent_nested_contents_and_align_wrapped_items() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Options:\n",
        " * <ol>\n",
        " * <li> a first item whose text is long enough to wrap onto another line\n",
        " * <li>a nested list:\n",
        " * <ul><li>inner\n",
        " *\n",
        " * second inner paragraph</ul>\n",
        " * </ol>\n",
        " * <dl><dt>term<dd>definition</dl>\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let options = Options {
        line_width: line_width(50),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * Options:
       *
       * <ol>
       *   <li>a first item whose text is long enough
       *       to wrap onto another line
       *   <li>a nested list:
       *       <ul>
       *         <li>inner
       *
       *             second inner paragraph
       *       </ul>
       * </ol>
       *
       * <dl>
       *   <dt>term
       *   <dd>definition
       * </dl>
       */
      int value;
    }
    ");
}

#[test]
fn preformatted_html_keeps_its_lines_and_spacing() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * See <table><tr><td>a   b</td></tr></table> and\n",
        " * <table>\n",
        " *   <tr><td>c    d</td></tr>\n",
        " * </table>\n",
        " * <ul><li>Example:<pre>\n",
        " *   keep(  this );\n",
        " *\n",
        " *   and(  this );\n",
        " * </pre></ul>\n",
        " * {@snippet :\n",
        " *   call(  x );\n",
        " * }\n",
        " */\n",
        "int value;\n",
        "}\n",
    );

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      /**
       * See
       *
       * <table><tr><td>a   b</td></tr></table>
       *
       * and
       *
       * <table>
       *   <tr><td>c    d</td></tr>
       * </table>
       *
       * <ul>
       *   <li>Example:
       *       <pre>
       *   keep(  this );
       *
       *   and(  this );
       * </pre>
       * </ul>
       *
       * {@snippet :
       * call(x);
       * }
       */
      int value;
    }
    ");
}

#[test]
fn text_next_to_block_html_stays_in_its_paragraph_for_case_and_period() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * first paragraph\n",
        " * <ul><li>item</ul>\n",
        " * after the list\n",
        " * <pre>\n",
        " *   code\n",
        " * </pre>\n",
        " * after the code\n",
        " *\n",
        " * second paragraph\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let options = settings(JavadocOptions {
        capitalize_descriptions: true,
        description_with_dot: DescriptionWithDot::Always,
        ..Default::default()
    });

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * First paragraph
       *
       * <ul>
       *   <li>item
       * </ul>
       *
       * after the list
       *
       * <pre>
       *   code
       * </pre>
       *
       * after the code.
       *
       * Second paragraph.
       */
      int value;
    }
    ");
}

#[test]
fn kept_empty_paragraph_tag_ends_the_paragraph_before_it() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * first paragraph\n",
        " * <p>\n",
        " * <ul><li>item</ul>\n",
        " */\n",
        "int value;\n",
        "}\n",
    );
    let options = settings(JavadocOptions {
        description_with_dot: DescriptionWithDot::Always,
        ..Default::default()
    });

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /**
       * first paragraph.
       *
       * <p>
       *
       * <ul>
       *   <li>item
       * </ul>
       */
      int value;
    }
    ");
}

#[test]
fn empty_paragraph_before_a_block_tag_leaves_tag_spacing_to_the_setting() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Sets both.\n",
        " *\n",
        " * @param x a\n",
        " *\n",
        " * <p>\n",
        " * @param y b\n",
        " *\n",
        " * <p>\n",
        " * @return the sum\n",
        " */\n",
        "int set(int x, int y) { return x + y; }\n",
        "}\n",
    );
    let format = |paragraph_html_tags| {
        let options = JavadocOptions {
            separate_tag_groups: SeparateTagGroups::BeforeReturn,
            paragraph_html_tags,
            ..Default::default()
        };
        fmt_with_options(source, &settings(options))
    };

    assert_snapshot!(format(false), @"
    class A {
      /**
       * Sets both.
       *
       * @param x a
       *     <p>
       * @param y b
       *     <p>
       *
       * @return the sum
       */
      int set(int x, int y) {
        return x + y;
      }
    }
    ");
    assert_snapshot!(format(true), @"
    class A {
      /**
       * Sets both.
       *
       * @param x a
       * @param y b
       *
       * @return the sum
       */
      int set(int x, int y) {
        return x + y;
      }
    }
    ");
}

#[test]
fn multiline_code_stays_after_its_start_tag_and_before_the_next_paragraph() {
    let source = concat!(
        "class A {\n",
        "/**\n",
        " * Examples.\n",
        " * <p>{@code first(\n",
        " *     line)} continues\n",
        " * <ul>\n",
        " * <li>{@code item(\n",
        " *     line)}\n",
        " * </ul>\n",
        " * Uses {@code last(\n",
        " *     line)}\n",
        " *\n",
        " * next paragraph\n",
        " */\n",
        "int value;\n",
        "}\n",
    );

    assert_snapshot!(fmt_with_options(source, &settings(JavadocOptions::default())), @"
    class A {
      /**
       * Examples.
       *
       * <p>{@code first(
       *     line)}
       * continues
       *
       * <ul>
       *   <li>{@code item(
       *     line)}
       * </ul>
       *
       * Uses
       * {@code last(
       *     line)}
       *
       * next paragraph
       */
      int value;
    }
    ");
}

#[test]
fn markdown_javadoc_normalizes_quoted_blocks_and_block_boundaries() {
    let source = concat!(
        "/// > Title\n",
        "/// > continued\n",
        "/// > ===\n",
        "/// >\n",
        "/// > > |a|bb|\n",
        "/// > > |-|--|\n",
        "/// > > |x|long|\n",
        "/// >\n",
        "/// > ~~~ text\n",
        "/// > has ``` inside\n",
        "/// > ~~~\n",
        "///\n",
        "/// # Heading\n",
        "/// |a|bb|\n",
        "/// |-|--|\n",
        "/// |x|long|\n",
        "///\n",
        "/// -     int   x=1;\n",
        "public class Probe {}\n",
    );
    let options = settings(JavadocOptions::default());
    let formatted = fmt_with_options(source, &options);
    assert_eq!(fmt_with_options(&formatted, &options), formatted);
    assert_snapshot!(formatted, @"
    /// > # Title continued
    /// >
    /// > > | a   | bb   |
    /// > > | --- | ---- |
    /// > > | x   | long |
    /// >
    /// > ```` text
    /// > has ``` inside
    /// > ````
    ///
    /// # Heading
    /// | a   | bb   |
    /// | --- | ---- |
    /// | x   | long |
    ///
    /// - ```
    ///   int x = 1;
    ///   ```
    public class Probe {}
    ");
}

#[test]
fn markdown_javadoc_drops_whitespace_lines_after_its_last_text() {
    let options = settings(JavadocOptions::default());
    for (source, expected) in [
        ("/// - a\n///   \n///\nclass A {}\n", "/// - a\nclass A {}\n"),
        ("/// 1. ```\n///\n///    \n///\nclass A {}\n", "/// 1. ```\n///    ```\nclass A {}\n"),
    ] {
        assert_eq!(format_stable_with_options(source, &options), expected, "{source:?}");
    }
}

#[test]
fn markdown_ordered_lists_reindent_when_numbers_change_width() {
    let options = settings(JavadocOptions::default());
    for (body, expected) in [
        ("8) a\n8) b\n8) c\n   - child", "8. a\n9. b\n10. c\n    - child"),
        (
            "1) a\n100) b\n     continuation\n\n     - child",
            "1. a\n2. b\n   continuation\n\n   - child",
        ),
        (
            "8) a\n8) b\n8)     raw\n\n       continuation",
            "8. a\n9. b\n10. ```\n    raw\n\n    continuation\n    ```",
        ),
    ] {
        for tag in ["", "@deprecated Steps:\n\n"] {
            let comment = |text: &str| {
                format!("{tag}{text}")
                    .lines()
                    .map(
                        |line| {
                            if line.is_empty() { "///\n".into() } else { format!("/// {line}\n") }
                        },
                    )
                    .collect::<String>()
            };
            let source = format!("{}public class Example {{}}\n", comment(body));
            let formatted = fmt_with_options(&source, &options);
            assert_eq!(formatted, format!("{}public class Example {{}}\n", comment(expected)));
        }
    }
}

/// A Markdown documentation comment holding the lines of `text`.
fn markdown_comment(text: &str) -> String {
    text.split('\n')
        .map(|line| if line.is_empty() { "///\n".into() } else { format!("/// {line}\n") })
        .collect()
}

#[test]
fn markdown_ordered_lists_normalize_before_closing_empty_fences() {
    let options = settings(JavadocOptions::default());
    for (body, expected) in [
        ("- 01.\n* 01. ```java", "- 1.\n* 1. ```java\n     ```"),
        ("- 1.\n* 01. ```java", "- 1.\n* 1. ```java\n     ```"),
        ("  - 01.\n* ~j\n\n  01. ```java", "  - 1.\n* ~j\n\n  1. ```java\n     ```"),
        ("  - 01.\n*\n  01. ```java", "  - 1.\n*\n  1. ```java\n     ```"),
    ] {
        let source = format!("{}class Example {{}}\n", markdown_comment(body));
        assert_eq!(
            format_stable_with_options(&source, &options),
            format!("{}class Example {{}}\n", markdown_comment(expected)),
        );
    }
}

#[test]
fn markdown_ordered_markers_before_underindented_prose_normalize_on_the_first_pass() {
    let options = settings(JavadocOptions::default());
    for (body, expected) in [
        ("001) a\n\n   b", "1. a\n\nb"),
        ("01. # H\n   a b", "1. # H\na b"),
        ("01. ```text\n   a b", "1. ```text\n   ```\na b"),
        ("01. {@code x}\n\n   a b", "1. {@code x}\n\na b"),
    ] {
        let source = format!("{}class Example {{}}\n", markdown_comment(body));
        assert_eq!(
            format_stable_with_options(&source, &options),
            format!("{}class Example {{}}\n", markdown_comment(expected)),
        );
    }
}

#[test]
fn nested_markdown_code_keeps_its_literal_text_and_blank_lines() {
    for capitalize_descriptions in [false, true] {
        let options = settings(JavadocOptions { capitalize_descriptions, ..Default::default() });
        for gap in ["\n\n", "\n  \n", "\n  \t\n"] {
            for literal in ["raw", "*raw*", "__raw__", r"\*raw\*", "[ref]: url", "[^note]: *raw*"] {
                let body = format!("- Outer\n  -     {literal}{gap}        continuation");
                let comment = body
                    .split('\n')
                    .map(
                        |line| {
                            if line.is_empty() { "///\n".into() } else { format!("/// {line}\n") }
                        },
                    )
                    .collect::<String>();
                let source = format!("{comment}public class Probe {{}}\n");
                let formatted = fmt_with_options(&source, &options);
                let expected = format!(
                    "/// - Outer\n///   - ```\n///     {literal}\n///\n///     continuation\n///     ```\npublic class Probe {{}}\n"
                );
                assert_eq!(formatted, expected);
                assert_eq!(fmt_with_options(&formatted, &options), formatted);
            }
        }
    }
}
