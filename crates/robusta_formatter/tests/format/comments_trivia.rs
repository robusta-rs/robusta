//! Comments, blank lines, and trivia ownership.

use cow_utils::CowUtils;
use insta::assert_snapshot;
use robusta_formatter::{IndentStyle, JavadocOptions, LineEnding, Options};

use crate::fixtures::{
    check_contract_at_widths_and_modes, format_source as fmt,
    format_source_with_options as fmt_with_options, format_stable, format_stable_with_options,
};

#[test]
fn comments_keep_their_place_around_inherited_choice_variants() {
    let source = r#"class A<T> {
  // a
  class B {} // b
  // c
  ; // d
  /* e */ enum C {} /* f */
  // g
  interface D {} // h
  record E() {} // i
  @interface F {} // j
  java.util.Map</* k */ String, /* l */ ? extends T> m; // n
  int[] o = /* p */ {1, /* q */ 2}; // r
  @G(/* s */ "t") int u; // v
  @G(/* w */ {/* x */ "y"}) int z; // aa
  @G(/* ab */ b = 1) int ac; // ad

  void m(Object o) throws Exception {
    // ae
    class L {} // af
    // ag
    enum M {} // ah
    interface N {} // ai
    record O() {} // aj
    if (o instanceof /* ak */ String /* al */) {} // am
    if (o instanceof /* an */ String s) {} // ao
    if (o instanceof /* ap */ P(/* aq */ var q, /* ar */ _)) {} // as
    var t = /* at */ (Runnable) () -> /* au */ true; // av
    Runnable v = () -> /* aw */ {
      // ax
    }; // ay
    Object w = /* az */ String /* bz */ ::valueOf; // bb
    Object x = /* bc */ int[] /* bd */ ::new; // be
    Object y = /* bf */ o /* bg */ ::toString; // bh
    switch (1) {
      case 1 -> /* bi */ o.hashCode(); // bj
      case 2 -> /* bk */ {
      } // bl
      default -> /* bm */ throw new Exception(); // bn
    }
    try (/* bo */ r; /* bp */ this.r; /* bq */ var z = null) {} // br
  }
}
"#;
    assert_snapshot!(fmt(source), @r#"
    class A<T> {
      // a
      class B {} // b

      // c
      ; // d

      /* e */ enum C {} /* f */

      // g
      interface D {} // h

      record E() {} // i

      @interface F {} // j

      java.util.Map</* k */ String, /* l */ ? extends T> m; // n
      int[] o = /* p */ {1, /* q */ 2}; // r
      @G(/* s */ "t") int u; // v
      @G(/* w */ {/* x */ "y"}) int z; // aa
      @G(/* ab */ b = 1) int ac; // ad

      void m(Object o) throws Exception {
        // ae
        class L {} // af
        // ag
        enum M {} // ah
        interface N {} // ai
        record O() {} // aj
        if (o instanceof /* ak */ String /* al */) {} // am
        if (o instanceof /* an */ String s) {} // ao
        if (o instanceof /* ap */ P(/* aq */ var q, /* ar */ _)) {} // as
        var t = /* at */ (Runnable) () -> /* au */ true; // av
        Runnable v = () -> /* aw */ {
          // ax
        }; // ay
        Object w = /* az */ String /* bz */::valueOf; // bb
        Object x = /* bc */ int[] /* bd */::new; // be
        Object y = /* bf */ o /* bg */::toString; // bh
        switch (1) {
          case 1 -> /* bi */ o.hashCode(); // bj
          case 2 -> /* bk */ {} // bl
          default -> /* bm */ throw new Exception(); // bn
        }
        try (/* bo */ r; /* bp */ this.r; /* bq */ var z = null) {} // br
      }
    }
    "#);
    check_contract_at_widths_and_modes(source);
}

#[test]
fn blank_lines_between_body_comments_stay_without_a_following_entry() {
    for marker in ["//", "///", "/*"] {
        let end = if marker == "/*" { " */" } else { "" };
        let comments = format!("{marker} first{end}\n\n\n{marker} last{end}\n");
        for (source, expected) in [
            (
                format!("class A {{\n{comments}void f() {{}}}}"),
                format!(
                    "class A {{\n  {marker} first{end}\n\n  {marker} last{end}\n  void f() {{}}\n}}\n"
                ),
            ),
            (
                format!("class A {{void f() {{\n{comments}}}}}"),
                format!(
                    "class A {{\n  void f() {{\n    {marker} first{end}\n\n    {marker} last{end}\n  }}\n}}\n"
                ),
            ),
            (
                format!("class A {{\n{comments}}}"),
                format!("class A {{\n  {marker} first{end}\n\n  {marker} last{end}\n}}\n"),
            ),
        ] {
            assert_eq!(format_stable(&source), expected);
        }
    }
}

#[test]
fn body_entry_comments_keep_their_code_line_and_blank_lines() {
    let source = r"class Shop {
  int f() {

    // one
    int s = a/* c */+/* d */b;
    g( /* e */ a /* f */ , b );

    // two


    // three
    return s;
  }
  // leads h
  @Deprecated

  /* package */ void h() { /* g */ return; }
}
";
    assert_snapshot!(format_stable(source), @"
    class Shop {
      int f() {
        // one
        int s = a /* c */ + /* d */ b;
        g(/* e */ a /* f */, b);

        // two

        // three
        return s;
      }

      // leads h
      @Deprecated
      /* package */ void h() {
        /* g */ return;
      }
    }
    ");
}

#[test]
fn translated_line_break_keeps_enum_comment_inside_arguments() {
    assert_snapshot!(fmt(r"enum E {UNIT(1\u000a/* in */),B}"), @"
    enum E {
      UNIT(
        1
        /* in */
      ),
      B,
    }
    ");
}

#[test]
fn escaped_line_breaks_around_an_operand_comment_format_as_physical_ones() {
    let source = "class A { void f() { x =GAP/* c */GAPb; y = a +GAP/* c */GAPb; } }";
    let escaped = format_stable(&source.cow_replace("GAP", r"\u000a"));
    assert_eq!(escaped, format_stable(&source.cow_replace("GAP", "\n")));
}

#[test]
fn escaped_line_breaks_inside_a_block_comment_format_as_physical_ones() {
    for source in [
        "class A { void f() { int x /* aBREAK b */\n= 1; } }",
        "class A { void f() { int x\n/* aBREAK b */ = 1; } }",
    ] {
        let physical = format_stable(&source.cow_replace("BREAK", "\n"));
        let escaped = format_stable(&source.cow_replace("BREAK", r"\u000a"));
        assert_eq!(escaped.cow_replace(r"\u000a", "\n"), physical, "{source:?}");
    }
}

/// A block comment ending a line it shares with code joins what follows; one
/// on a line holding only comments keeps that line.
#[test]
fn a_block_comment_ending_a_line_of_code_joins_what_follows() {
    let source = concat!(
        "class A {\n",
        "  void f(/* c */\n",
        "  int x) {}\n",
        "  void g(int /* c */\n",
        "  x) {}\n",
        "  int[] v = {/* c */\n",
        "  1};\n",
        "  void h() {\n",
        "    foo(/* c */\n",
        "    a);\n",
        "    foo(a,\n",
        "    /* b */ /* c */\n",
        "    d);\n",
        "    if (nextChar >= nChars) /* EOF */\n",
        "      break;\n",
        "  }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @"
    class A {
      void f(/* c */ int x) {}

      void g(int /* c */ x) {}

      int[] v = {/* c */ 1};

      void h() {
        foo(/* c */ a);
        foo(
          a,
          /* b */ /* c */
          d
        );
        if (nextChar >= nChars) /* EOF */ break;
      }
    }
    ");
}

#[test]
fn escaped_whitespace_keeps_list_comments_inside_delimiters() {
    for (source, close) in [
        ("enum E {UNIT(1GAP/* in */),B}", ")"),
        ("class E {void m() {call(1GAP/* in */);}}", ")"),
        ("record E(int xGAP/* in */) {}", ")"),
        ("class E {List<StringGAP/* in */> x;}", ">"),
        ("enum E {UNIT(1GAP/* in */GAP/* next */),B}", ")"),
        ("class E {void m() {call(1GAP/* in */GAP/* next */);}}", ")"),
        ("record E(int xGAP/* in */GAP/* next */) {}", ")"),
        ("class E {List<StringGAP/* in */GAP/* next */> x;}", ">"),
        ("enum E {UNIT(1\\u0020/* in */GAP\\u0020/* own */),B}", ")"),
    ] {
        for (gap, physical) in [
            (r"\u0020", " "),
            (r"\uu0009", "\t"),
            (r"\uuu000c", "\u{c}"),
            (r"\u000C\uu0020\uuu0009", "\u{c} \t"),
            (r"\u000a", "\n"),
            (r"\uu000D", "\r"),
            (r"\uuu000d\u000A", "\r\n"),
            ("\\u000d\n", "\r\n"),
            ("\r\\uu000a", "\r\n"),
            (r"\u0020\uu0009\uuu000c\u000a\u0020", " \t\u{c}\n "),
            ("\n", "\n"),
        ] {
            let ordinary =
                source.cow_replace("GAP", physical).cow_replace(r"\u0020", " ").into_owned();
            for escaped_close in [false, true] {
                let escaped = source.cow_replace("GAP", gap).into_owned();
                let escaped = if escaped_close {
                    let comment_end = escaped.rfind("*/").unwrap() + 2;
                    let spelling = if close == ")" { r"\uuu0029" } else { r"\uu003e" };
                    format!(
                        "{}{}{}",
                        &escaped[..comment_end],
                        spelling,
                        &escaped[comment_end + 1..]
                    )
                } else {
                    escaped
                };
                for width in [40, 80] {
                    for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
                        for insert_final_newline in [false, true] {
                            let options = Options {
                                line_width: crate::fixtures::line_width(width),
                                line_ending,
                                insert_final_newline,
                                ..Options::default()
                            };
                            assert_eq!(
                                format_stable_with_options(&escaped, &options),
                                format_stable_with_options(&ordinary, &options),
                                "{escaped:?}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn escaped_list_comments_preserve_raw_token_spellings() {
    let source = r#"enum E {UN\u0049T(1\u0030\u0020/* enum */),B}
        record R(int x\uu0009/* record */) {}
        class Cl\u0061ss {List<Str\u0069ng\uuu000c/* type */> x;
        void m() {c\u0061ll("\u0041"\u0020/* call */);}}"#;
    let output = format_stable(source);
    for spelling in [
        r"UN\u0049T(1\u0030 /* enum */)",
        "int x /* record */)",
        r"Cl\u0061ss",
        r"List<Str\u0069ng /* type */>",
        r#"c\u0061ll("\u0041" /* call */)"#,
    ] {
        assert!(output.contains(spelling), "lost {spelling:?} in {output:?}");
    }
}

#[test]
fn escaped_whitespace_keeps_end_of_line_comments_on_their_line() {
    for source in ["class A { int a =GAP// c\n1; }", "class A { void m() { a +=GAP// c\nb; } }"] {
        for (gap, physical) in [(r"\u0020", " "), (r"\uu0009", "\t")] {
            let escaped = source.cow_replace("GAP", gap);
            let ordinary = source.cow_replace("GAP", physical);
            assert_eq!(format_stable(&escaped), format_stable(&ordinary), "{escaped:?}");
        }
    }
}

#[test]
fn own_line_block_comments_after_control_open_parens_keep_their_line() {
    let source = "class A { void f() { if (\n    /* a\n     b */\n    a) x(); \
                  while (\n    /* c\n     d */\n    a) x(); \
                  switch (\n    /* e\n     f */\n    a) {} \
                  synchronized (\n    /* g\n     h */\n    a) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (
          /* a
         b */
          a
        )
          x();
        while (
          /* c
         d */
          a
        )
          x();
        switch (
          /* e
         f */
          a
        ) {}
        synchronized (
          /* g
         h */
          a
        ) {}
      }
    }
    ");
}

#[test]
fn multiline_block_comments_after_control_open_parens_break_inside_them() {
    let source = "class A { void f() { if (/* a\n b */ a) {} while (/* c\n d */ a) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (
          /* a
     b */ a
        ) {}
        while (
          /* c
     d */ a
        ) {}
      }
    }
    ");
}

#[test]
fn own_line_block_comments_after_for_separators_keep_their_line() {
    let source = "class A { void f() { for (int i = 0;\n    /* a\n     b */\n    i < n;\n    /* c\n     d */\n    i++) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        for (
          int i = 0;
          /* a
         b */
          i < n;
          /* c
         d */
          i++
        ) {}
      }
    }
    ");
}

#[test]
fn own_line_block_comments_after_catch_tokens_and_case_keep_their_line() {
    let source = "class A { void f() { try {} catch (\n    /* a\n     b */\n    B |\n    /* c\n     d */\n    C e) {} \
                  switch (v) { case\n    /* e\n     f */\n    1: break; } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {} catch (
          /* a
         b */
          B
              /* c
         d */
              | C e
        ) {}
        switch (v) {
          case
            /* e
         f */
            1:
            break;
        }
      }
    }
    ");
}

#[test]
fn own_line_block_comments_after_bound_and_clause_separators_keep_their_line() {
    let source = "class A<T extends B &\n    /* a\n     b */\n    C> implements B,\n    /* c\n     d */\n    C {}";
    assert_snapshot!(fmt(source), @"
    class A<
            T extends
                B
                    /* a
         b */
                    & C>
        implements B,
            /* c
         d */
            C {}
    ");
}

#[test]
fn comments_before_wrapped_declaration_clauses_do_not_add_blank_lines() {
    for (head, keyword, tail) in [
        ("class Service extends Base", "implements", "{}"),
        ("interface Service", "extends", "{}"),
        ("sealed class Service", "permits", "{}"),
        ("enum Service", "implements", "{}"),
        ("class Service { void invoke()", "throws", "{} }"),
    ] {
        let source = format!("{head} // head\n// clause\n{keyword} Alpha // entry\n, Beta {tail}");
        format_stable(&source);
    }
}

#[test]
fn own_line_comments_print_before_a_multi_catch_pipe() {
    let source = "class A { void f() { try {} catch (IllegalStateException\n// also\n| IllegalArgumentException e) {} \
                  try {} catch (IllegalStateException |\n\n// after\nIllegalArgumentException e) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {} catch (IllegalStateException
            // also
            | IllegalArgumentException e) {}
        try {} catch (IllegalStateException
            // after
            | IllegalArgumentException e) {}
      }
    }
    ");
}

#[test]
fn comments_at_a_catch_pipe_and_before_a_case_guard() {
    let source = "class A { void f() { try {} catch (A // a\n| B\n/* b */ | C e) {} \
                  switch (o) { case R r, S s // s\nwhen s.ok() -> {} \
                  case T t\n// t\nwhen t.ok() -> {} case U u\n/* u */\nwhen u.ok() -> {} \
                  default -> {} } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {} catch (A // a
            | B
            | /* b */ C e) {}
        switch (o) {
          case R r,
            S s // s
            when s.ok() -> {}
          case T t
            // t
            when t.ok() -> {}
          case U u
            /* u */
            when u.ok() -> {}
          default -> {}
        }
      }
    }
    ");
}

#[test]
fn comments_keep_their_side_of_an_escaped_catch_pipe() {
    for types in ["A /* a */ | /* b */ B", "A // a\n|\n// b\nB"] {
        let source = format!("class A {{ void f() {{ try {{}} catch ({types} e) {{}} }} // c\n}}");
        let escaped = source.cow_replace("|", r"\uu007C").into_owned();
        assert_eq!(format_stable(&escaped), format_stable(&source), "{escaped:?}");
    }
}

#[test]
fn comments_keep_their_side_of_escaped_tokens_without_spans() {
    let template = "enum E { A # , # ; static # {} }
        class C {
            C(int a #) # {}
            void m(#) {
                if (a #) # x(); else # y();
                while (b #) # x();
                do # x(); while (c);
                for (int i = 0; i < n; i++ #) # x();
                for (#;;) {}
                for (int v : vs #) # x();
                switch (s #) # { case 1 # -> x(); }
                switch (s) { case 1 # : x(); }
                synchronized (m #) # { x(); }
                try (R r = a() #) # { x(); } catch (E e #) # { y(); }
                o = outer. # new # <T> Inner();
                receiver. # <T>m();
                r = A:: # <T>m;
            }
            void r(C this #) # {}
        }
    ";
    for comment in ["/* c */", "// c\n", "\n/* c */\n"] {
        let source = template.cow_replace("#", comment);
        for (token, escaped) in [
            (")", r"\u0029"),
            ("{", r"\uu007b"),
            (",", r"\u002c"),
            (";", r"\u003b"),
            (":", r"\uu003a"),
            (".", r"\u002e"),
            ("<", r"\u003c"),
            ("->", r"\u002d\u003e"),
            ("else", r"\u0065lse"),
            ("do ", r"d\u006f "),
            ("for", r"\u0066or"),
            ("new", r"n\u0065w"),
            ("static", r"\u0073tatic"),
        ] {
            let escaped = source.cow_replace(token, escaped).into_owned();
            assert_eq!(format_stable(&escaped), format_stable(&source), "{escaped:?}");
        }
    }
}

#[test]
fn own_line_block_comments_before_catch_modifiers_keep_their_line() {
    let source = "class A { void f() { try {} catch (\n    /* a\n     b */\n    final B e) {} \
                  try {} catch (\n    /* c\n     d */\n    @Nullable final B | C e) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {} catch (
          /* a
         b */
          final B e
        ) {}
        try {} catch (
          /* c
         d */
          @Nullable final B | C e
        ) {}
      }
    }
    ");
}

fixture_tests! {
    multi_line_block_comments_reindent_with_owner => "comments-and-trivia/block-comments";
    golden_blank_line_normalization_file => "comments-and-trivia/blank-line-normalization";
    qualified_type_comments_keep_their_segment_boundary => "comments-and-trivia/qualified-type-comment-ownership";
    modifier_gap_comments_stay_between_their_modifiers => "comments-and-trivia/modifier-gap-comment-ownership";
    trailing_comments_stay_with_their_empty_member => "comments-and-trivia/empty-member-comment-ownership";
    comment_ownership_boundaries_compose => "comments-and-trivia/comment-ownership-combination";
}

#[test]
fn structural_comment_breaks_leave_no_blank_line_after_spaces() {
    let cases = [
        ("class A { // first  \n int x; }", "class A { // first\n  int x;\n}\n"),
        ("class A { void m() { // last  \n } }", "class A {\n  void m() { // last\n  }\n}\n"),
        (
            "class A { public // first  \n void m() {} }",
            "class A {\n  public // first\n  void m() {}\n}\n",
        ),
        (
            "class A { @Test // first  \n void m() {} }",
            "class A {\n  @Test // first\n  void m() {}\n}\n",
        ),
        (
            "class A { void m(// first  \n int a) {} }",
            "class A {\n  void m( // first\n    int a\n  ) {}\n}\n",
        ),
        (
            "class A { void m(int a // last  \n) {} }",
            "class A {\n  void m(\n    int a // last\n  ) {}\n}\n",
        ),
    ];
    for (source, expected) in cases {
        for javadoc in [None, Some(JavadocOptions::default())] {
            assert_eq!(
                fmt_with_options(source, &Options { javadoc, ..Options::default() }),
                expected
            );
        }
    }
}

#[test]
fn line_comment_trailing_spaces_do_not_count_toward_the_line_width() {
    let source = concat!(
        "class A {\n",
        "    void m() {\n",
        "        register(new Route[] {\n",
        "            new Route(\"GET\", \"/bill/{id}\", api::read, Duration.ofSeconds(30)), // read  \n",
        "            new Route(\"PUT\", \"/bill/{id}\", api::write, Duration.ofSeconds(5)), // write  \n",
        "        });\n",
        "        if (request != null && request.isValid() && cache.isEmpty() && queue.size() < limit) // fill  \n",
        "            load();\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @r#"
    class A {
      void m() {
        register(new Route[] {
          new Route("GET", "/bill/{id}", api::read, Duration.ofSeconds(30)), // read
          new Route("PUT", "/bill/{id}", api::write, Duration.ofSeconds(5)), // write
        });
        if (request != null && request.isValid() && cache.isEmpty() && queue.size() < limit)
          // fill
          load();
      }
    }
    "#);
}

#[test]
fn line_comment_loses_only_trailing_spaces_and_tabs() {
    let source = concat!(
        "class A {\n",
        "  int x; // first\u{c}\n",
        "  // second  \u{c}\n",
        "  int y; // third\u{c} \t\n",
        "}\n",
        "// last \t",
    );
    let expected = concat!(
        "class A {\n",
        "  int x; // first\u{c}\n",
        "  // second  \u{c}\n",
        "  int y; // third\u{c}\n",
        "}\n",
        "// last",
    );
    for insert_final_newline in [true, false] {
        let options = Options { insert_final_newline, ..Options::default() };
        let newline = if insert_final_newline { "\n" } else { "" };
        assert_eq!(fmt_with_options(source, &options), format!("{expected}{newline}"));
    }
}

#[test]
fn fall_through_comment_stays_on_its_statement_line() {
    let source =
        "class A { void f() { switch (x) { case 1: f(); // falls through\ncase 2: g(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1:
            f(); // falls through
          case 2:
            g();
        }
      }
    }
    ");
}

#[test]
fn lambda_block_bodies_keep_interior_comments() {
    let source = "class A { void f() { r = x -> { // add\nint y = x + 1; return; }; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        r = x -> { // add
          int y = x + 1;
          return;
        };
      }
    }
    ");
}

#[test]
fn same_line_block_comment_at_a_control_header_stays_inline() {
    // A same-line single-line block comment between a control header and its
    // body is kept at that seam, describing the body it precedes, rather than
    // re-homed after the whole statement.
    let source = "class A { void f() { if (a) /* c */ x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) /* c */ x();
      }
    }
    ");
}

#[test]
fn inline_block_comments_stay_with_argument_list_items() {
    let source = "class C { Object f() { return JdkBackedImmutableMap.create(size, entries, \
                  /* throwIfDuplicateKeys= */ true); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      Object f() {
        return JdkBackedImmutableMap.create(size, entries, /* throwIfDuplicateKeys= */ true);
      }
    }
    ");
}

#[test]
fn inline_block_comments_indent_with_broken_argument_list_items() {
    let source = "class C { void f() { dispatcher.submit(firstExtremelyLongArgumentName, \
                  /* second= */ secondExtremelyLongArgumentName, \
                  thirdExtremelyLongArgumentName); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        dispatcher.submit(
          firstExtremelyLongArgumentName,
          /* second= */ secondExtremelyLongArgumentName,
          thirdExtremelyLongArgumentName
        );
      }
    }
    ");
}

#[test]
fn argument_comments_stay_on_the_line_they_trail() {
    let source = "class C { void f() { int n = grow(size,\n size - used, /* at least */\n size /* doubled */);\n \
                  log(level, message /* raw */ // unformatted\n );\n \
                  submit(first\n // first group\n ,\n second); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        int n = grow(size, size - used /* at least */, size /* doubled */);
        log(
          level,
          message /* raw */ // unformatted
        );
        submit(
          first,
          // first group
          second
        );
      }
    }
    ");
}

#[test]
fn comments_inside_method_bodies_are_kept() {
    let source = "class A { void f() { // first\nint x = 1;\n/* dangling */ } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() { // first
        int x = 1;
        /* dangling */
      }
    }
    ");
}

#[test]
fn comment_only_module_and_switch_bodies_expand_their_braces() {
    assert_snapshot!(fmt("module m { /* only */ }"), @"
    module m {
      /* only */
    }
    ");

    assert_snapshot!(fmt("class A { void f(int x) { switch (x) { /* only */ } } }"), @"
    class A {
      void f(int x) {
        switch (x) {
          /* only */
        }
      }
    }
    ");
}

/// Every braced body sits in the same frame: nothing to write collapses to
/// `{}`, and anything to write — an entry or only a dangling comment — opens
/// the body on its own line and closes the brace on its own line. Module,
/// type, enum, block, and switch bodies are pinned together because one frame
/// writes both of those breaks for all of them.
#[test]
fn braced_bodies_share_one_frame() {
    assert_snapshot!(fmt("module m {}"), @"module m {}");
    assert_snapshot!(fmt("class A {}"), @"class A {}");
    assert_snapshot!(fmt("enum E {}"), @"enum E {}");
    assert_snapshot!(fmt("class A { void f() {} }"), @"
    class A {
      void f() {}
    }
    ");

    assert_snapshot!(fmt("class A { /* only */ }"), @"
    class A {
      /* only */
    }
    ");

    assert_snapshot!(fmt("enum E { /* only */ }"), @"
    enum E {
      /* only */
    }
    ");

    assert_snapshot!(fmt("class A { void f() { /* only */ } }"), @"
    class A {
      void f() {
        /* only */
      }
    }
    ");

    assert_snapshot!(fmt("class A { void f(int x) { switch (x) { case 1: g(); } } }"), @"
    class A {
      void f(int x) {
        switch (x) {
          case 1:
            g();
        }
      }
    }
    ");
}

/// Every braced body joins its entries the same way: a source blank line
/// between two entries survives, a leading comment starts its entry's line, a
/// same-line comment stays with the entry it follows, and a comment before the
/// closing brace keeps the blank line that preceded it. Module, type, enum,
/// block, and switch bodies are pinned together because they share one join.
#[test]
fn braced_bodies_join_their_entries_alike() {
    assert_snapshot!(
        fmt("module m { requires a; // first\n\n// second\nexports b;\n\n/* end */ }"),
        @"
    module m {
      requires a; // first

      // second
      exports b;

      /* end */
    }
    "
    );

    assert_snapshot!(
        fmt("class A { int f; // first\n\n// second\nvoid g() {}\n\n/* end */ }"),
        @"
    class A {
      int f; // first

      // second
      void g() {}

      /* end */
    }
    "
    );

    assert_snapshot!(
        fmt("enum E { A, // first\n\n// second\nB;\n\nint f;\n\n/* end */ }"),
        @"
    enum E {
      A, // first

      // second
      B;

      int f;

      /* end */
    }
    "
    );

    assert_snapshot!(
        fmt("class A { void f() { g(); // first\n\n// second\nh();\n\n/* end */ } }"),
        @"
    class A {
      void f() {
        g(); // first

        // second
        h();

        /* end */
      }
    }
    "
    );

    assert_snapshot!(
        fmt("class A { void f(int x) { switch (x) { case 1: // first\n\n// second\ncase 2: g();\n\n/* end */ } } }"),
        @"
    class A {
      void f(int x) {
        switch (x) {
          case 1: // first

          // second
          case 2:
            g();

          /* end */
        }
      }
    }
    "
    );
}

/// An enum body joins three kinds of entry in one sequence — constants, the
/// body semicolon, and the members after it — and each kind can be absent.
#[test]
fn enum_bodies_join_constants_semicolon_and_members() {
    assert_snapshot!(fmt("enum E { ; int f; void g() {} }"), @"
    enum E {
      ;

      int f;

      void g() {}
    }
    ");

    assert_snapshot!(fmt("enum E { ; /* end */ }"), @"
    enum E {
      ; /* end */
    }
    ");

    assert_snapshot!(fmt("enum E { A }"), @"
    enum E {
      A,
    }
    ");

    assert_snapshot!(fmt("enum E { A; int f; }"), @"
    enum E {
      A;

      int f;
    }
    ");
}

/// The last constant, or the body `;`, keeps the comments before the `}` when
/// a member follows the enum.
#[test]
fn enum_body_end_comments_stay_in_the_body() {
    assert_snapshot!(fmt("class A { enum E { X, Y\n// a\n// b\n} int f; }"), @"
    class A {
      enum E {
        X,
        Y,
        // a
        // b
      }

      int f;
    }
    ");

    assert_snapshot!(fmt("class A { enum E { ;\n// a\n} int f; }"), @"
    class A {
      enum E {
        ;
        // a
      }

      int f;
    }
    ");
}

/// A blank line after an enum constant counts from past its source comma, so a
/// comma on its own line adds none.
#[test]
fn enum_constant_blank_line_counts_from_its_comma() {
    assert_snapshot!(fmt("enum E {\nA //\n, B\n,\nC //\n,\n// end\n}"), @"
    enum E {
      A, //
      B,
      C, //
      // end
    }
    ");

    assert_snapshot!(fmt("enum E {\nA //\n,\n\nB\n\n// c\n, C\n}"), @"
    enum E {
      A, //

      B,

      // c
      C,
    }
    ");
}

/// A generic member without modifiers starts at its `<`, so the blank-line
/// choice before it holds on every pass and a comment after the `<` stays in
/// the list.
#[test]
fn generic_member_boundary_is_its_type_parameter_list() {
    let source = "class A {\nvoid f() {}\n<T // one\n> void g() {}\nint x;\n<T // two\n> A() {}\n\
                  < /* three */ T> void h() {}\n}";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {}

      <
        T // one
      > void g() {}

      int x;

      <
        T // two
      > A() {}

      </* three */ T> void h() {}
    }
    ");
}

#[test]
fn linter_line_suppression_keeps_its_target_line() {
    let source = r"class A {
static public long first=10l; // robusta-disable-line -- generated
long second=20l;
}";

    assert_snapshot!(fmt(source), @"
    class A {
      static public long first = 10l; // robusta-disable-line -- generated
      long second = 20l;
    }
    ");
}

#[test]
fn blank_line_between_type_declarations() {
    assert_snapshot!(fmt("class A {} class B {}"), @"
    class A {}

    class B {}
    ");
}

#[test]
fn top_level_node_boundaries_distinguish_leading_and_trailing_comments() {
    assert_snapshot!(fmt("class A {} /* next */ class B {}"), @"
    class A {}

    /* next */ class B {}
    ");

    assert_snapshot!(fmt("class A {} // owner\nclass B {}"), @"
    class A {} // owner

    class B {}
    ");
}

#[test]
fn statement_comment_is_not_claimed_by_a_nested_argument() {
    let source = "class A { void f() { call(inner(item)).done(); // owner\n} }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        call(inner(item)).done(); // owner
      }
    }
    ");
}

#[test]
fn comment_only_source_is_kept() {
    assert_snapshot!(fmt("// a\n/* b */\n"), @"
    // a
    /* b */
    ");
    assert_eq!(fmt("\n\n// a\n/* b */\n"), "// a\n/* b */\n");
}

#[test]
fn empty_parameter_list_keeps_its_comment_between_the_parentheses() {
    let source = "class A { void i(/* empty */) {} }";
    assert_snapshot!(fmt(source), @"
    class A {
      void i(/* empty */) {}
    }
    ");
}

#[test]
fn empty_constructor_parameter_list_keeps_its_comment_between_the_parentheses() {
    let source = "class A { A(/* none */) {} }";
    assert_snapshot!(fmt(source), @"
    class A {
      A(/* none */) {}
    }
    ");
}

#[test]
fn empty_lambda_parameter_list_keeps_its_comment_between_the_parentheses() {
    let source = "class A { Runnable r = (/* c */) -> {}; }";
    assert_snapshot!(fmt(source), @"
    class A {
      Runnable r = (/* c */) -> {};
    }
    ");
}

#[test]
fn empty_parameter_list_comment_does_not_displace_the_header_boundary_comment() {
    let source = "class A { void i(/* in */) /* after */ {} }";
    assert_snapshot!(fmt(source), @"
    class A {
      void i(/* in */) /* after */ {}
    }
    ");
}

#[test]
fn comments_before_an_opener_trail_the_name_or_keep_their_lines() {
    let source = "class A { void m /* a */\n(int x) {} void n() { foo\n\n// b\n(1); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m /* a */(int x) {}

      void n() {
        foo(
          // b
          1
        );
      }
    }
    ");
}

#[test]
fn a_comment_kept_before_an_opener_indents_as_a_wrapped_line_and_a_calls_moves_inside() {
    let source = "class Orders {
  void load() {
    page = new ArrayList

        // Sized for one page.
        <>(50);
    audit

        // Each order is logged once.
        (orders);
  }
}";
    assert_snapshot!(fmt(source), @"
    class Orders {
      void load() {
        page = new ArrayList
          // Sized for one page.
          <>(50);
        audit(
          // Each order is logged once.
          orders
        );
      }
    }
    ");
}

#[test]
fn a_comment_kept_before_type_arguments_indents_them_twice_in_a_return_type() {
    let source = "class Audit {
  List // unsorted
  <Order> orders;
  List // newest first
  <Order> recent() { return orders; }
}";
    assert_snapshot!(fmt(source), @"
    class Audit {
      List // unsorted
        <Order> orders;

      List // newest first
          <Order> recent() {
        return orders;
      }
    }
    ");
}

#[test]
fn a_line_comment_after_an_opening_parenthesis_keeps_its_line() {
    let source = "class Orders {
  boolean ship() {
    audit // each order once
        (orders, refunds);
    ship( // fastest carrier first
        carriers, priority);
    notify(
        // the customer first
        customer);
    if ( // never due once paid
        paid && !refunded) {
      close();
    }
    return ( // settings before the zone
        lenient == other.lenient);
  }
}";
    assert_snapshot!(format_stable(source), @"
    class Orders {
      boolean ship() {
        audit( // each order once
          orders,
          refunds
        );
        ship( // fastest carrier first
          carriers,
          priority
        );
        notify(
          // the customer first
          customer
        );
        if ( // never due once paid
          paid && !refunded
        ) {
          close();
        }
        return ( // settings before the zone
          lenient == other.lenient
        );
      }
    }
    ");
}

#[test]
fn a_comment_before_an_empty_call_list_moves_inside_it() {
    let source = "class A { void m() { flush // pending writes\n(); reset\n/* state */\n(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        flush( // pending writes
        );
        reset(
          /* state */
        );
      }
    }
    ");
}

#[test]
fn a_comment_kept_before_an_annotation_list_indents_as_a_wrapped_line() {
    let source = "class A { @Timeout // seconds\n(30) void run() {} \
                  void check() { @Retry\n// twice\n(2) int attempts; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      @Timeout // seconds
        (30)
      void run() {}

      void check() {
        @Retry
          // twice
          (2)
        int attempts;
      }
    }
    ");
}

#[test]
fn a_comment_kept_before_a_dimension_indents_as_a_wrapped_line() {
    let source = "class A { int // rows\n[] rows; \
                  void m() { String\n/* names */\n[] names; int cells\n// per row\n[]; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int // rows
        [] rows;

      void m() {
        String
          /* names */
          [] names;
        int cells
          // per row
          [];
      }
    }
    ");
}

/// A line comment after a method's name rides past the empty `()` and any `[]`
/// up to the next comment, so a block comment before a `[` starts the next line,
/// indented as a wrapped line.
#[test]
fn a_comment_after_a_riding_name_comment_indents_a_dimension_as_a_wrapped_line() {
    let source = "class Grid { int[] row // first\n() /* cells */[] { return null; } \
                  int column // last\n()[] /* cells */[] { return null; } \
                  int[] edge() /* cells */ [] { return null; } \
                  int[] corner() // shared\n[] { return null; } }";
    assert_snapshot!(fmt(source), @"
    class Grid {
      int[] row() // first
        /* cells */ [] {
        return null;
      }

      int column()[] // last
        /* cells */ [] {
        return null;
      }

      int[] edge() /* cells */[] {
        return null;
      }

      int[] corner() // shared
        [] {
        return null;
      }
    }
    ");
}

/// The riding name comment meets a comment inside a dimension's brackets,
/// which flushes it after the `[`. The second pass moves it onto its own line.
#[test]
fn a_comment_inside_a_dimension_after_a_riding_name_comment_moves_on_a_second_pass() {
    assert_snapshot!(fmt("class Grid { int row // first\n() [ /* cells */ ] {} }"), @"
    class Grid {
      int row()[ // first
        /* cells */
      ] {}
    }
    ---------- Not idempotent (second pass) ----------
    class Grid {
      int row()[
        // first
        /* cells */
      ] {}
    }
    ");
}

#[test]
fn the_code_after_a_dimension_kept_after_a_comment_continues_its_wrapped_line() {
    let source = "class Grid {\n\
                  String // column titles\n[] titles = {northColumnTitle, eastColumnTitle, \
                  southColumnTitle, westColumnTitle, centreTitle, otherTitle};\n\
                  int corners // clockwise\n[] = {northEastCornerIndex, southEastCornerIndex, \
                  southWestCornerIndex, northWestCornerIndex, centre};\n\
                  void fill() { cells = new int // rows first\n[] {firstRowOfCells, \
                  secondRowOfCells, thirdRowOfCells, fourthRowOfCells, fifthRowOfCells, \
                  sixthRow}; } }\n\
                  @interface Shape { int sides()\n// per face\n[] default {firstFaceSideCount, \
                  secondFaceSideCount, thirdFaceSideCount, fourthFaceSideCount, fifth}; }";
    assert_snapshot!(format_stable(source), @"
    class Grid {
      String // column titles
        [] titles = {
          northColumnTitle,
          eastColumnTitle,
          southColumnTitle,
          westColumnTitle,
          centreTitle,
          otherTitle,
        };
      int corners // clockwise
        [] = {
          northEastCornerIndex,
          southEastCornerIndex,
          southWestCornerIndex,
          northWestCornerIndex,
          centre,
        };

      void fill() {
        cells = new int // rows first
          [] {
            firstRowOfCells,
            secondRowOfCells,
            thirdRowOfCells,
            fourthRowOfCells,
            fifthRowOfCells,
            sixthRow,
          };
      }
    }

    @interface Shape {
      int sides()
        // per face
        [] default {
          firstFaceSideCount,
          secondFaceSideCount,
          thirdFaceSideCount,
          fourthFaceSideCount,
          fifth,
        };
    }
    ");
}

#[test]
fn the_code_after_type_arguments_kept_after_a_comment_continues_their_wrapped_line() {
    let source = "class Cache { void reset() { entries = new LinkedHashMap // evicts the eldest \
                  entry\n<>(initialCapacityForTheCache, defaultLoadFactorForTheCache, accessOrder) \
                  { protected boolean removeEldestEntry(Map.Entry<K, V> eldest) { return size() > \
                  limit; } }; view = entries.new View // newest first\n\
                  <>(firstColumnShownInTheView, secondColumnShownInTheView, \
                  thirdColumnShownInTheView, lastColumn); } }";
    assert_snapshot!(format_stable(source), @"
    class Cache {
      void reset() {
        entries = new LinkedHashMap // evicts the eldest entry
          <>(initialCapacityForTheCache, defaultLoadFactorForTheCache, accessOrder) {
            protected boolean removeEldestEntry(Map.Entry<K, V> eldest) {
              return size() > limit;
            }
          };
        view = entries.new View // newest first
          <>(
            firstColumnShownInTheView,
            secondColumnShownInTheView,
            thirdColumnShownInTheView,
            lastColumn
          );
      }
    }
    ");
}

#[test]
fn a_comment_kept_before_a_variable_arity_ellipsis_indents_as_a_wrapped_line() {
    let source = "class A { void log(String // any number\n... parts) {} \
                  void join(int first, Object\n/* rest */\n... rest) {} }";
    assert_snapshot!(fmt(source), @"
    class A {
      void log(
        String // any number
          ... parts
      ) {}

      void join(
        int first,
        Object
          /* rest */
          ... rest
      ) {}
    }
    ");
}

#[test]
fn comment_run_after_an_empty_opener_keeps_its_space() {
    let source = "class A { void m() { for (/* a */ // b\n;;) {} for (/* a */ /* b */;;) {} \
                  for (/* a */ /* b */ int x : xs) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        for (
          /* a */ // b
          ;;
        ) {}
        for (/* a */ /* b */;;) {}
        for (/* a */ /* b */ int x : xs) {}
      }
    }
    ");
}

#[test]
fn dangling_comments_take_a_line_each_unless_one_fits_inline() {
    let source = "class A { void m() { foo(/* a */); foo(/* a */ // b\n); foo(\n/* a */\n); \
                  foo(\n/* a */\n\n\n/* b */\n); } int[] g = {\n// a\n\n// b\n}; \
                  void n(/* a */ // b\n) {} void p(\n// a\n\n// b\n) {} \
                  @A({/* a */ // b\n}) int z; }";
    assert_snapshot!(format_stable(source), @"
    class A {
      void m() {
        foo(/* a */);
        foo(
          /* a */
          // b
        );
        foo(
          /* a */
        );
        foo(
          /* a */

          /* b */
        );
      }

      int[] g = {
        // a

        // b
      };

      void n(
        /* a */
        // b
      ) {}

      void p(
        // a

        // b
      ) {}

      @A({
        /* a */
        // b
      })
      int z;
    }
    ");
}

#[test]
fn adjacent_aligned_block_comments_keep_their_join() {
    let pair = "/*\n * first\n *//*\n * second\n */";
    let source = format!(
        "{pair}\nclass A {{\n{pair}\nint x; void f() {{ g({pair}); g(x {pair}); g(x\n{pair}\n); }} }}"
    );
    let output = format_stable(&source);
    assert_eq!(output.matches("*//*").count(), 5);
    check_contract_at_widths_and_modes(&source);
}

#[test]
fn adjacent_aligned_block_comments_stay_joined_in_empty_for_clause() {
    let source =
        "class A { void f() { for (int i = 0; /*\n * first\n *//*\n * second\n */ ; ) {} } }";
    assert!(format_stable(source).contains("*//*"));
    check_contract_at_widths_and_modes(source);
}

#[test]
fn adjacent_javadoc_can_still_become_single_line() {
    let source = "class A {\n/*\n * first\n *//**\n * second\n */\nint x; }";
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      /*
       * first
       */ /** second */
      int x;
    }
    ");
}

#[test]
fn annotation_element_keeps_comments_inside_its_parentheses() {
    let source = "@interface A { int value(/* x */); int other /* before */ (/* a */ /* b */); }";
    assert_snapshot!(fmt(source), @"
    @interface A {
      int value(/* x */);

      int other /* before */(
        /* a */
        /* b */
      );
    }
    ");
}

#[test]
fn annotation_element_keeps_line_comments_inside_its_parentheses() {
    let source = "@interface A { int value(\n// x\n); int other(// y\n) default 1; }";
    assert_snapshot!(fmt(source), @"
    @interface A {
      int value(
        // x
      );

      int other( // y
      ) default 1;
    }
    ");
}

#[test]
fn annotation_element_keeps_comments_before_its_obsolete_dimensions() {
    let source = "@interface A { int value() /* y */ []; \
                  int other(/* in */) /* first */ [] /* second */ [] default {{1}}; }";
    assert_snapshot!(fmt(source), @"
    @interface A {
      int value() /* y */[];

      int other(/* in */) /* first */[] /* second */[] default {{1}};
    }
    ");
}

#[test]
fn annotation_element_keeps_own_line_comments_before_its_obsolete_dimensions() {
    let source = "@interface A { int value()\n// x\n[]; int other()\n/* y */\n[]; }";
    assert_snapshot!(fmt(source), @"
    @interface A {
      int value()
        // x
        [];

      int other()
        /* y */
        [];
    }
    ");
}

#[test]
fn record_component_list_keeps_its_comments_between_the_parentheses() {
    let source = "record R(/* x */) implements I {} record S(/* y */) {} record T(\n// z\n) {} \
                  record U(int a\n// w\n) {}";
    assert_snapshot!(fmt(source), @"
    record R(/* x */) implements I {}

    record S(/* y */) {}

    record T(
      // z
    ) {}

    record U(
      int a
      // w
    ) {}
    ");
}

#[test]
fn field_declarator_comment_before_equals_stays_with_its_declarator() {
    let source = "class A { int x /* c */ = 1; int y; }";
    assert_snapshot!(fmt(source), @"
    class A {
      int x /* c */ = 1;
      int y;
    }
    ");
}

#[test]
fn multi_declarator_field_comment_before_equals_stays_with_its_declarator() {
    let source = "class A { int a, b /* c */ = 1; int y; }";
    assert_snapshot!(fmt(source), @"
    class A {
      int a,
        b /* c */ = 1;
      int y;
    }
    ");
}

#[test]
fn local_declarator_comment_before_equals_stays_with_its_declarator() {
    let source = "class A { void f() { int x /* c */ = 1; int y = 2; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        int x /* c */ = 1;
        int y = 2;
      }
    }
    ");
}

#[test]
fn field_and_local_declarator_lists_share_comment_boundaries() {
    let source = "class A { int /* type */ first = 1, /* comma */ second /* equals */ = 2; \
                  void f() { int /* type */ first = 1, /* comma */ second /* equals */ = 2; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int /* type */ first = 1,
        /* comma */ second /* equals */ = 2;

      void f() {
        int /* type */ first = 1,
          /* comma */ second /* equals */ = 2;
      }
    }
    ");
}

#[test]
fn declarator_comments_stay_on_their_side_of_the_comma() {
    let source = "class A { int a /* before */, /* after */ b = 1 /* value */, c; \
                  void f() { int x, // line\n y; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int a /* before */,
        /* after */ b = 1 /* value */,
        c;

      void f() {
        int x, // line
          y;
      }
    }
    ");

    let source = "class A { int a = 1// a\n, b// b\n, c; \
                  void f() { for (int i = 0// i\n, j = 1;;) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int a = 1, // a
        b, // b
        c;

      void f() {
        for (
          int i = 0, // i
            j = 1;
          ;
        ) {}
      }
    }
    ");
}

#[test]
fn block_comments_ending_a_declarator_line_trail_the_declarator() {
    let source = "class A { int width = 4, /* in pixels */\n height = 3; \
                  void f() { long rows = 0, /* skipped */\n cols = 0; \
                  for (int i = 0, /* outer */\n k = 9; i < k; i++) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int width = 4 /* in pixels */,
        height = 3;

      void f() {
        long rows = 0 /* skipped */,
          cols = 0;
        for (int i = 0 /* outer */, k = 9; i < k; i++) {}
      }
    }
    ");
}

#[test]
fn javadoc_after_an_enum_constant_comma_leads_the_next_constant() {
    let source = "enum Level { LOW, /** Raised by an operator. */\n HIGH, /* spare */\n MAX }";
    assert_snapshot!(fmt(source), @"
    enum Level {
      LOW,
      /** Raised by an operator. */
      HIGH /* spare */,
      MAX,
    }
    ");
}

#[test]
fn resource_declarator_comment_before_equals_stays_with_its_declarator() {
    let source = "class A { void f() { try (AutoCloseable x /* c */ = mk()) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try (AutoCloseable x /* c */ = mk()) {}
      }
    }
    ");
}

#[test]
fn block_comment_ending_a_resource_type_line_moves_the_name_as_in_a_local() {
    let declarator = "pooledConnection = connectionFactory.openPooledConnection(primaryArgument, backupArgument)";
    let source = format!(
        "class A {{ void f() throws Exception {{ AutoCloseable /* c */\n{declarator}; \
         try (AutoCloseable /* c */\n{declarator}) {{}} }} }}"
    );
    assert_snapshot!(fmt(&source), @"
    class A {
      void f() throws Exception {
        AutoCloseable /* c */
          pooledConnection = connectionFactory.openPooledConnection(primaryArgument, backupArgument);
        try (
          AutoCloseable /* c */
            pooledConnection = connectionFactory.openPooledConnection(primaryArgument, backupArgument)
        ) {}
      }
    }
    ");
}

#[test]
fn block_comment_ending_a_pattern_type_line_keeps_the_name_beside_it_as_in_a_local() {
    let source = "class A { void f(Object o) { String /* c */\ns // d\n; \
                  if (o instanceof String /* c */\nt // d\n&& t.isEmpty()) {} \
                  switch (o) { case String /* c */\nu // d\nwhen u.isEmpty() -> {} \
                  case Point(int /* c */\nx // d\n, int y) -> {} default -> {} } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(Object o) {
        String /* c */ s; // d
        if (o instanceof String /* c */ t // d
            && t.isEmpty()) {}
        switch (o) {
          case String /* c */ u // d
            when u.isEmpty() -> {}
          case Point(
              int /* c */ x, // d
              int y
            ) -> {}
          default -> {}
        }
      }
    }
    ");
}

/// Every assignment operator in the language claims the comments the source
/// put on each side of it, and keeps them there. The four constructs that
/// carry one — a declarator, an annotation element-value pair, a
/// try-with-resources declaration, and an assignment expression, simple and
/// compound — are asserted together because they share the claim.
#[test]
fn every_assignment_operator_keeps_the_comments_on_both_of_its_sides() {
    let source = "@interface Ann { int x(); } \
                  class A { \
                    int field /* a */ = /* b */ 1; \
                    @Ann(x /* a */ = /* b */ 1) \
                    void f() throws Exception { \
                      int local /* a */ = /* b */ 2; \
                      try (AutoCloseable r /* a */ = /* b */ mk()) {} \
                      local /* a */ = /* b */ 3; \
                      local /* a */ += /* b */ 4; \
                    } \
                    AutoCloseable mk() { return null; } \
                  }";
    assert_snapshot!(fmt(source), @"
    @interface Ann {
      int x();
    }

    class A {
      int field /* a */ = /* b */ 1;

      @Ann(x /* a */ = /* b */ 1)
      void f() throws Exception {
        int local /* a */ = /* b */ 2;
        try (AutoCloseable r /* a */ = /* b */ mk()) {}
        local /* a */ = /* b */ 3;
        local /* a */ += /* b */ 4;
      }

      AutoCloseable mk() {
        return null;
      }
    }
    ");
}

#[test]
fn annotation_element_value_comment_before_equals_stays_with_its_pair() {
    let source = "@interface Ann { int x(); } class A { @Ann(x /* c */ = 1) void f() {} }";
    assert_snapshot!(fmt(source), @"
    @interface Ann {
      int x();
    }

    class A {
      @Ann(x /* c */ = 1)
      void f() {}
    }
    ");
}

#[test]
fn annotation_element_value_comment_spanning_lines_before_equals_moves_after_it() {
    let source = "@interface Ann { int x(); } class A { @Ann(x /* spans\n */\n= 1) void f() {} }";
    assert_snapshot!(fmt(source), @"
    @interface Ann {
      int x();
    }

    class A {
      @Ann(
        x =
          /* spans
           */
          1
      )
      void f() {}
    }
    ");
}

// A comment after a list's last entry and before its closing delimiter trails
// that entry, as the next node is outside the list.

#[test]
fn comment_after_the_last_argument_stays_inside_its_list() {
    let source = "class A { void f() { g(\n  a,\n  b\n  // why\n  );\n} }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        g(
          a,
          b
          // why
        );
      }
    }
    ");
}

#[test]
fn line_comment_after_the_last_entry_breaks_a_list_that_would_otherwise_be_flat() {
    let source = "class A { void f() { g(a\n// why\n); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        g(
          a
          // why
        );
      }
    }
    ");
}

/// The last entry of every delimited list trails the comments before its
/// closer, including the lists whose closer the AST keeps no span for.
#[test]
fn comment_after_the_last_entry_stays_inside_every_delimited_list() {
    let source = "@A(\"x\"\n/* single */\n) class B<T\n/* type */\n> { \
                  List<String\n/* argument */\n> x; \
                  <U\n/* method */\n> void m() {} \
                  enum E { C(1\n/* constant */\n), D } }";
    assert_snapshot!(fmt(source), @r#"
    @A(
      "x"
      /* single */
    )
    class B<
        T
        /* type */
        > {
      List<
        String
        /* argument */
      > x;

      <
        U
        /* method */
      > void m() {}

      enum E {
        C(
          1
          /* constant */
        ),
        D,
      }
    }
    "#);
}

#[test]
fn comments_after_the_last_entry_keep_a_list_closer_on_its_own_line() {
    let cases = [
        (
            "class A { void m(int a // head\n // tail\n) {} }",
            "class A {\n  void m(\n    int a // head\n    // tail\n  ) {}\n}\n",
        ),
        (
            "class A { void m() { f(a\n /// mid  \n /// tail\t\n); } }",
            "class A {\n  void m() {\n    f(\n      a\n      /// mid\n      /// tail\n    );\n  }\n}\n",
        ),
        (
            "@B(a = 1\n // mid\t\n /* tail */\n) class A {}",
            "@B(\n  a = 1\n  // mid\n  /* tail */\n)\nclass A {}\n",
        ),
    ];
    for (source, expected) in cases {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            assert_eq!(fmt_with_options(source, &options), expected);
        }
    }
}

#[test]
fn line_comment_after_the_last_entry_before_an_empty_javadoc_still_ends_its_line() {
    let source = "class A { void m(int a // head  \n /** */\n // tail  \n /** */\n) {} }";
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void m(
        int a // head
        /** */
        // tail
        /** */
      ) {}
    }
    ");
}

/// The claim stops at the closing delimiter, so a comment written past it is
/// left for whatever owns the gap it sits in. Pulling it back inside the list
/// would move it just as far in the other direction.
#[test]
fn a_comment_past_the_closing_delimiter_is_not_claimed_by_the_list() {
    let source = "class A { void f() { g(a, b); /* after */ h(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        g(a, b);
        /* after */ h();
      }
    }
    ");
}

/// Formatting changes the width of the code before a trailing comment, so its
/// continuation lines move to the indent, star lines one space in.
#[test]
fn multi_line_trailing_block_comments_keep_their_lines_on_a_second_pass() {
    let source = concat!(
        "class A {\n",
        "    int x; /* first\n",
        "              second */\n",
        "    int y;     /* star\n",
        "                * line */\n",
        "    void f() {\n",
        "        flags.set(1); /* first bit\n",
        "                   * set before the loop */\n",
        "        call(); /* plain\n",
        "                   continuation\n",
        "                     deeper */\n",
        "        x = y /* op\n",
        "                 comment */ + z;\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @"
    class A {
      int x; /* first
                  second */
      int y; /* star
              * line */

      void f() {
        flags.set(1); /* first bit
                       * set before the loop */
        call(); /* plain
                       continuation
                         deeper */
        x =
          y /* op
                     comment */ + z;
      }
    }
    ");
}

#[test]
fn only_star_aligned_blocks_force_a_break_after_assignment_operators() {
    let source = concat!(
        "class A {\n",
        "    void f() {\n",
        "        int x =\n",
        "            /* a\n",
        "               b */ 5;\n",
        "        y = a +\n",
        "            /* c\n",
        "               d */ b;\n",
        "        z +=\n",
        "            /*\n",
        "             * e\n",
        "             */ 5;\n",
        "        z += /*\n",
        "         * e\n",
        "         */ 5;\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        int x = /* a
                   b */ 5;
        y =
          a
          + /* c
                   d */ b;
        z +=
          /*
           * e
           */ 5;
        z +=
          /*
           * e
           */ 5;
      }
    }
    ");
}

#[test]
fn conditional_colon_comments_have_one_layout_for_both_spellings() {
    let source = "class Test {\n  int f(boolean ok) {\n    return ok ? 1\n      /* before */ :\n      /* after */ 2;\n  }\n}\n";
    let options = Options { line_width: crate::fixtures::line_width(37), ..Options::default() };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class Test {
      int f(boolean ok) {
        return ok
          ? 1
          : /* before */ /* after */ 2;
      }
    }
    ");
    for width in [37, 100] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options {
                line_width: crate::fixtures::line_width(width),
                javadoc,
                ..Options::default()
            };
            assert_eq!(
                format_stable_with_options(&source.cow_replace(':', r"\u003a"), &options),
                format_stable_with_options(source, &options)
            );
        }
    }
}

/// The conditional operators and `instanceof` break the way a binary operator
/// does.
#[test]
fn multiline_blocks_lead_conditional_and_instanceof_operands() {
    let source = concat!(
        "class A {\n",
        "    void f() {\n",
        "        x = c ?\n",
        "            /* a\n",
        "               b */ d : e;\n",
        "        x = c ? d :\n",
        "            /* a\n",
        "               b */ e;\n",
        "        boolean q = o instanceof\n",
        "            /* a\n",
        "               b */ String;\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        x = c
          ? /* a
                   b */ d
          : e;
        x = c
          ? d
          : /* a
                   b */ e;
        boolean q =
          o
          instanceof
          /* a
                   b */ String;
      }
    }
    ");
}

#[test]
fn multiline_blocks_lead_operands_after_casts_unary_operators_and_dots() {
    let source = concat!(
        "class A {\n",
        "    void f() {\n",
        "        Object o = (Object)\n",
        "            /* a\n",
        "               b */ x;\n",
        "        y = !\n",
        "            /* a\n",
        "               b */ z;\n",
        "        foo.\n",
        "            /* a\n",
        "               b */ bar();\n",
        "        y = foo.\n",
        "            /* a\n",
        "               b */ bar;\n",
        "        java.util.\n",
        "            /* a\n",
        "               b */ List<String> l;\n",
        "        o = Outer.\n",
        "            /* a\n",
        "               b */ this;\n",
        "        o = Outer.\n",
        "            /* a\n",
        "               b */ super.f;\n",
        "        k = String.\n",
        "            /* a\n",
        "               b */ class;\n",
        "        o = outer.\n",
        "            /* a\n",
        "               b */ new Inner(1);\n",
        "        r = Outer.\n",
        "            /* a\n",
        "               b */ super::m;\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        Object o = (Object) /* a
                   b */ x;
        y = !/* a
                   b */ z;
        foo
          /* a
                   b */ .bar();
        y =
          foo./* a
                   b */ bar;
        java.util./* a
                   b */ List<String> l;
        o =
          Outer./* a
                   b */ this;
        o = Outer./* a
                   b */ super.f;
        k =
          String./* a
                   b */ class;
        o = outer./* a
                   b */ new Inner(1);
        r = Outer./* a
                   b */ super::m;
      }
    }
    ");
}

/// Comments before a `]` the AST keeps no node for lead it, and those around
/// a `.` before a keyword or `*` keep their own line in front of the `.`.
#[test]
fn comments_around_nodeless_tokens_keep_their_line_and_side() {
    let source = concat!(
        "import java.util.\n",
        "// a\n",
        "*;\n",
        "\n",
        "class A {\n",
        "    void f() {\n",
        "        o = Outer.\n",
        "            // b\n",
        "            this;\n",
        "        k = String\n",
        "            /* c */\n",
        "            .class;\n",
        "        m = new int[3][\n",
        "            // d\n",
        "        ];\n",
        "        r = Box::<String> /* e */ new;\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @"
    import java.util
    // a
    .*;

    class A {
      void f() {
        o =
          Outer
          // b
          .this;
        k =
          String
          /* c */
          .class;
        m = new int[3][
          // d
        ];
        r = Box::<String>/* e */ new;
      }
    }
    ");
}

// After a comment that ends the line, a field name is indented as a member and
// the token is not, so lines compare without their indentation.
#[test]
fn a_dot_before_a_nodeless_token_takes_comments_as_a_dot_before_a_name_does() {
    let lines = |code: &str| code.lines().map(str::trim_start).collect::<Vec<_>>().join("\n");
    for (before, after) in [
        ("", " // c\n"),
        ("", "\n// c\n"),
        ("", "\n// c\n\n"),
        ("", " /* c */\n"),
        ("", " /* c */ "),
        ("", "\n/* c */ "),
        ("", " /* c\n * d */\n"),
        (" /* b */ ", " // c\n"),
        (" // b\n", " /* c */ "),
        (" // b\n// d\n", " // c\n// e\n"),
    ] {
        for (prefix, suffix, name, operands) in [
            (
                "class A { Object f() { return a",
                "; } }",
                "name",
                ["this", "class", "new B()", "super::m"].as_slice(),
            ),
            ("class A { A() { a", "; } }", "new B()", ["super()", "<T>super()"].as_slice()),
            ("import a", ";", "B", ["*"].as_slice()),
        ] {
            let source = |operand: &str| format!("{prefix}{before}.{after}{operand}{suffix}");
            let named = lines(&fmt(&source(name)));
            for operand in operands {
                let source = source(operand);
                assert_eq!(lines(&fmt(&source)), named.cow_replace(name, operand), "{source:?}");
                check_contract_at_widths_and_modes(&source);
            }
        }
    }
}

#[test]
fn multiline_blocks_lead_reference_operands_and_array_lengths() {
    let source = concat!(
        "class A {\n",
        "    void f() {\n",
        "        r = Foo::\n",
        "            /* a\n",
        "               b */ <T>bar;\n",
        "        r = Outer.super::\n",
        "            /* a\n",
        "               b */ m;\n",
        "        r = Foo::\n",
        "            /* a\n",
        "               b */ new;\n",
        "        m = new int[\n",
        "            /* a\n",
        "               b */ 3][\n",
        "            /* a\n",
        "               b */ ];\n",
        "        m = new int[\n",
        "            /* a\n",
        "               b */ 3 // c\n",
        "        ];\n",
        "        m = new int[3\n",
        "          // c\n",
        "        ]\n",
        "        [4];\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        r =
          Foo::/* a
                   b */ <T>bar;
        r = Outer.super
          ::/* a
                   b */ m;
        r =
          Foo::/* a
                   b */ new;
        m = new int[
          /* a
                   b */ 3
        ][
          /* a
                   b */
        ];
        m = new int[
          /* a
                   b */ 3 // c
        ];
        m = new int[
          3
          // c
        ][4];
      }
    }
    ");
}

/// An array access indents its index as a block, wherever the comment starts.
#[test]
fn multiline_block_comment_in_an_array_index_breaks_the_brackets() {
    let source = concat!(
        "class A {\n",
        "    void f() {\n",
        "        z = arr[\n",
        "            /* a\n",
        "               b */ i];\n",
        "        z = arr[/* a\n",
        "         b */ i];\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        z =
          arr[
            /* a
                   b */ i
          ];
        z =
          arr[
            /* a
             b */ i
          ];
      }
    }
    ");
    for index in ["/* a\n b */ index", "\n/* a\n b */ index"] {
        check_contract_at_widths_and_modes(&format!(
            "class A {{ Object longVariableNameForTesting = values[{index}]; }}"
        ));
    }
}

/// After an array creation's `new`, the initializer moves with the type.
#[test]
fn own_line_block_comment_after_an_array_creation_new_moves_with_its_operand() {
    let source = concat!(
        "class A {\n",
        "    void f() {\n",
        "        y = new\n",
        "            /* a\n",
        "               b */ int[3];\n",
        "        s = new\n",
        "            /* a\n",
        "               b */ String[] {\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", \"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\"};\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @r#"
    class A {
      void f() {
        y = new
        /* a
                   b */ int[3];
        s = new
        /* a
                   b */ String[] {
          "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        };
      }
    }
    "#);
}

/// A broken chain already indents its segments, so the comment keeps the
/// chain's indent.
#[test]
fn multiline_blocks_lead_members_of_a_broken_chain() {
    let source = concat!(
        "class A {\n",
        "    void f() {\n",
        "        a.b().\n",
        "            /* a\n",
        "               b */ c().d(1);\n",
        "        a.\n",
        "            /* a\n",
        "               b */ b().c(1);\n",
        "    }\n",
        "}\n",
    );
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        a.b()
          /* a
                   b */ .c()
          .d(1);
        a
          /* a
                   b */ .b()
          .c(1);
      }
    }
    ");
}

#[test]
fn comments_before_a_member_after_a_call_keep_their_side() {
    let source = "class C {
        void f() {
            consume(queue.forRegion(region) /* a */ .poll);
            consume(queue.forRegion(region). /* b */ poll);
            consume(queue.forRegion(region) // c
                .poll);
            consume(queue.forRegion(region). // d
                poll);
            consume(queue.forRegion(region)
                // e
                .poll);
            consume(queue.forRegion(region) // f
                ::poll);
        }
    }";
    let options = Options {
        line_width: crate::fixtures::line_width(30),
        indent_width: crate::fixtures::indent_width(2),
        ..Options::default()
    };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class C {
      void f() {
        consume(
          queue.forRegion(
            region
          ) /* a */.poll
        );
        consume(
          queue.forRegion(region)
            ./* b */ poll
        );
        consume(
          queue.forRegion(region) // c
            .poll
        );
        consume(
          queue.forRegion(region) // d
            .poll
        );
        consume(
          queue.forRegion(region)
            // e
            .poll
        );
        consume(
          queue.forRegion(region) // f
            ::poll
        );
      }
    }
    ");
}

#[test]
fn leading_comments_stay_before_the_dots_of_call_chain_lookups() {
    let source = "\
class A {
  void f() {
    a./* head */ b./* call */ c();
    a./* starts */ b()./* lookup */ field.c(1);
    a()./* lookup */ field.b();
    o = a.b()./* standalone */ field;
    a /* receiver */. /* first */ /* second */ b();
    a. // receiver
       /// lookup
       /* name */ b();
    foo().

       // first lookup
       b();
  }
}
";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        a./* head */ b
          /* call */ .c();
        a
          /* starts */ .b()
          /* lookup */ .field.c(1);
        a()
          /* lookup */ .field.b();
        o = a.b()./* standalone */ field;
        a /* receiver */
          /* first */ /* second */ .b();
        a // receiver
          /// lookup
          /* name */ .b();
        foo()
          // first lookup
          .b();
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn a_line_comment_after_a_chain_head_indents_the_next_member() {
    let source = "\
class A {
  void f() {
    o = items // c
      .stream().filter(predicateForTheEntries).map(mapperForTheEntries).toList();
    if (items // c
      .stream().filter(predicateForTheEntries).map(mapperForTheEntries).toList()) {}
  }
}
";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        o = items // c
          .stream()
          .filter(predicateForTheEntries)
          .map(mapperForTheEntries)
          .toList();
        if (items // c
            .stream()
            .filter(predicateForTheEntries)
            .map(mapperForTheEntries)
            .toList()) {}
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn later_lines_of_a_block_comment_without_stars_stay_as_written() {
    let source = "class A {\n/* a  \n   \n\n\tb \n */\nint x;\n}\n";
    let expected = "class A {\n  /* a  \n   \n\n\tb \n */\n  int x;\n}\n";
    for javadoc in [None, Some(JavadocOptions::default())] {
        let options = Options { javadoc, ..Options::default() };
        assert_eq!(fmt_with_options(source, &options), expected);
    }
}

#[test]
fn later_stars_of_a_block_comment_line_up_under_the_first() {
    let source = "class A {\n/* first  \n        * second  \n\t*third\n */\nint n;\nint m; /* a\n     * b */\nint k=3; /* c\n * d */\n}\n";
    let expected = "class A {\n  /* first\n   * second\n   *third\n   */\n  int n;\n  int m; /* a\n          * b */\n  int k = 3; /* c\n              * d */\n}\n";
    for javadoc in [None, Some(JavadocOptions::default())] {
        let options = Options { javadoc, ..Options::default() };
        assert_eq!(fmt_with_options(source, &options), expected);
    }
    // Tabs keep the indentation and spaces reach the column.
    let options = Options { indent_style: IndentStyle::Tab, ..Options::default() };
    assert_eq!(
        fmt_with_options("class A {\nint m; /* a\n * b */\n}\n", &options),
        "class A {\n\tint m; /* a\n\t        * b */\n}\n"
    );
    // A Javadoc comment after code lines up the same way, formatted or not.
    let source = "class A {\nint n; /** Documents m.\n * @see B */\nint m;\n}\n";
    let kept = "class A {\n  int n; /** Documents m.\n          * @see B */\n  int m;\n}\n";
    let formatted = "class A {\n  int n; /**\n          * Documents m.\n          *\n          * @see B\n          */\n  int m;\n}\n";
    for (javadoc, expected) in [(None, kept), (Some(JavadocOptions::default()), formatted)] {
        let options = Options { javadoc, ..Options::default() };
        assert_eq!(fmt_with_options(source, &options), expected);
        assert_eq!(fmt_with_options(expected, &options), expected);
    }
}

#[test]
fn block_comment_line_endings_follow_the_configured_ending() {
    for (line_ending, newline) in [(LineEnding::Lf, "\n"), (LineEnding::Crlf, "\r\n")] {
        let options = Options { line_ending, ..Options::default() };
        let expected = "class A {\n  /* a\n  b  \n\n */\n  /* c\n   * d\n   */\n  int x;\n}\n"
            .cow_replace('\n', newline);
        let source = "class A {\n/* a\n  b  \n\n */\n/* c\n * d\n */\nint x;\n}\n";
        for ending in ["\n", "\r\n", "\r"] {
            let source = source.cow_replace('\n', ending);
            assert_eq!(fmt_with_options(&source, &options), expected);
        }
    }
}

#[test]
fn name_comments_keep_their_tokens() {
    let templates = [
        "package#a#.#b#; class C {}",
        "import#static#a#.#B#.#c#; import#a#.#*#; class D {}",
        "module#a#.#b {#exports#c#to#d#,#e#; provides#f#.#G#with#h#.#I#,#J#; }",
        "sealed class C#permits#a#.#B#,#D {}",
        "class C {#<#T#>#C#() {} }",
        "class C {#C#() {}#<#T#>#void#m#() {} void n(C#C#.#this) {} }",
        "class C { void m(Object o) { if (o instanceof String#s) {} l#:#for (;;) { break#l#; } } }",
        "class C { Object f = a#.#b#.#c#(); Object g = a#::#b; Object h = A#.#B#.#this; }",
        "class C { void m() { try (R#r = x) {} catch (E#e) {} for (String#x : xs) {} int#y; } }",
        "enum E {#A#(1),#B# }",
        "record R(int x) { public#R# {} }",
        "@interface A { int#v() default 1; }",
        "class C { Object f = (int#x) -> x; Object g =#x#-> x; }",
        "class C { C() { this#(1); } Object f = new D#<>#(); }",
        "class C { void m(Object o) { if (o instanceof R#(var a)) {} } }",
        "@A#(1) record R#<T>#(int x) {}",
    ];
    for template in templates {
        for gap in [" /* a */ // b\n", "\n/* a */\n// b\n", " /* a */ /* b */ "] {
            // `block_comment_after_a_constructor_type_parameter_line_comment_spaces_on_a_second_pass`
            // pins the constructor's second pass.
            if gap == " /* a */ // b\n" && template.starts_with("class C {#<#T#>#C#") {
                continue;
            }
            crate::fixtures::check_contract_at_widths_and_modes(&template.cow_replace('#', gap));
        }
    }
}

/// A line comment that moves behind a member's `;`, or behind the empty `()`
/// and body after a name, ends the member's line, so the block comment after
/// it starts the next line and leads the next member, below the blank line
/// between them.
#[test]
fn block_comment_after_a_member_line_comment_leads_the_next_member() {
    let source = "class C { int x = 1 // c\n; /* field */\nint y = 2 // d\n; /* next field */\n\
                  int z;\nvoid m // e\n() {} /* method */\nabstract void a // f\n(); /* bodyless */\n\
                  <T> // g\nC() {} /* constructor */\nvoid n() {} }\n\
                  enum E { A // h\n; /* constant */\nE() {} }";
    assert_snapshot!(fmt(source), @"
    class C {
      int x = 1; // c
      /* field */
      int y = 2; // d
      /* next field */
      int z;

      void m() {} // e

      /* method */
      abstract void a(); // f

      /* bodyless */
      <T> C() {} // g

      /* constructor */
      void n() {}
    }

    enum E {
      A; // h

      /* constant */
      E() {}
    }
    ");
}

/// The type parameters' line comment rides past the constructor's name, so
/// the block comment after the name starts the next line. The second pass
/// reads it on a line of its own and puts a space before the `(`.
#[test]
fn block_comment_after_a_constructor_type_parameter_line_comment_spaces_on_a_second_pass() {
    assert_snapshot!(fmt("class C { <T> // b\nC /* a */ () {} }"), @"
    class C {
      <T> C // b
      /* a */() {}
    }
    ---------- Not idempotent (second pass) ----------
    class C {
      <T> C // b
      /* a */ () {}
    }
    ");
}

/// A name's line comment that moved after the `(` of a parameter list has
/// ended its line there, so the block comment after the body stays on the
/// body's line.
#[test]
fn block_comment_after_a_parameter_list_holding_the_name_comment_stays_on_its_line() {
    let source = "class C { void methodName // riding\n\
                  (String loooooooooonng, String loooooooooonng) {} /* x */\nvoid n() {} }";
    let expected = "class C {\n  void methodName( // riding\n    String loooooooooonng,\n    \
                    String loooooooooonng\n  ) {} /* x */\n\n  void n() {}\n}\n";
    for width in [30, 100] {
        let options =
            Options { line_width: crate::fixtures::line_width(width), ..Options::default() };
        assert_eq!(format_stable_with_options(source, &options), expected);
    }
    let source = "class C { void m // riding\n(int a) {} /* x */\nvoid n() {} }";
    assert_eq!(
        format_stable(source),
        "class C {\n  void m( // riding\n    int a\n  ) {} /* x */\n\n  void n() {}\n}\n"
    );
}
