//! Statements and control-flow forms.

use insta::assert_snapshot;

use robusta_formatter::{AfterClosingBrace, IndentStyle, JavadocOptions, LineEnding, Options};

use crate::fixtures::{
    check_contract_at_widths_and_modes, format_source as fmt, format_source_with_options,
    format_stable_with_options, indent_width, line_width,
};

fixture_tests! {
    golden_control_flow_file => "statements/control-flow";
    golden_enhanced_for_wrapping_file => "statements/enhanced-for-wrapping";
    golden_multi_catch_wrapping_file => "statements/multi-catch-wrapping";
    golden_for_expression_lists_file => "statements/for-expression-lists";
    golden_throw_file => "statements/throw";
    golden_assert_file => "statements/assert";
    golden_try_catch_finally_file => "statements/try-catch-finally";
    golden_try_with_resources_file => "statements/try-with-resources";
    golden_synchronized_file => "statements/synchronized";
    golden_switch_file => "statements/switch";
    golden_switch_rules_file => "statements/switch-rules";
    golden_switch_wrapping_file => "statements/switch-wrapping";
    golden_switch_expressions_file => "statements/switch-expressions";
    golden_yield_identifiers_file => "statements/yield-identifiers";
    golden_switch_patterns_file => "statements/switch-patterns";
    golden_switch_null_default_file => "statements/switch-null-default";
    redundant_semicolons_are_preserved => "statements/redundant-semicolons";
}

#[test]
fn switch_expression_bodies_keep_their_comments() {
    let source = "class A { \
        int a = switch (x) {}; \
        int b = switch (x) { /* only */ }; \
        int c = switch (x) { // first\ncase 1 -> 1; /* between */\n// next\ndefault -> 2; // last\n// after\n}; \
        int d = switch (x) { /* first */ case 1: yield 1; /* between */ default: yield 2; /* last */ }; }";
    assert_snapshot!(fmt(source), @"
    class A {
      int a = switch (x) {};
      int b = switch (x) {
        /* only */
      };
      int c = switch (x) { // first
        case 1 -> 1; /* between */
        // next
        default -> 2; // last
        // after
      };
      int d = switch (x) {
        /* first */ case 1:
          yield 1;
        /* between */ default:
          yield 2; /* last */
      };
    }
    ");
}

#[test]
fn switch_statement_comments_after_the_last_case_stay_inside() {
    let source = "class A { void f(int x) { \
        switch (x) { case 1 -> g(); // last\n// after\n} \
        switch (x) { case 1: g(); // last\n// after\n} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(int x) {
        switch (x) {
          case 1 -> g(); // last
          // after
        }
        switch (x) {
          case 1:
            g(); // last
          // after
        }
      }
    }
    ");
}

#[test]
fn empty_switch_block_collapses() {
    assert_snapshot!(fmt("class A { void f() { switch (x) {} } }"), @"
    class A {
      void f() {
        switch (x) {}
      }
    }
    ");
}

#[test]
fn switch_group_terminator_comments_use_the_label_indent() {
    let source = "class A { void f() { switch (x) { case 1: y(); break\n/* c */\n; \
                  default: handle()\n// last\n; } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1:
            y();
            break;
          /* c */
          default:
            handle();
          // last
        }
      }
    }
    ");
}

#[test]
fn comments_after_the_last_switch_group_stay_in_the_switch() {
    let source = "class A { void f() { switch (x) { case 1: y(); // a\n// b\n} z(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1:
            y(); // a
          // b
        }
        z();
      }
    }
    ");
}

#[test]
fn comments_before_a_lone_case_block_that_end_their_line_follow_its_brace() {
    let source = "class A { void f() { switch (x) { case 1: /* a */ { y(); } case 2: // b\n{ y(); } \
                  case 3:\n// c\n{ y(); } default: { y(); } // d\n} } }";
    let expected = "class A {
  void f() {
    switch (x) {
      case 1: /* a */ {
        y();
      }
      case 2: { // b
        y();
      }
      case 3: {
        // c
        y();
      }
      default: {
        y();
      } // d
    }
  }
}
";
    for after_closing_brace in [AfterClosingBrace::SameLine, AfterClosingBrace::NewLine] {
        let options = Options { after_closing_brace, ..Options::default() };
        assert_eq!(format_source_with_options(source, &options), expected);
    }
}

/// The comments moved behind the `;` of a `return`, a `throw`, or a switch
/// rule do not break the group around it, at any width; a block comment after
/// a moved line comment starts the next line.
#[test]
fn comments_moved_behind_a_return_throw_or_switch_rule_semicolon_keep_their_line() {
    let source = "class A { int f() { if (c) return d // c\n; \
                  while (a) throw e // c\n; \
                  if (c) return // c\n; \
                  if (c) return d /* a */ // c\n; \
                  if (c) return d // a comment past the line width of one hundred columns does not break the statement\n; \
                  int y = switch (x) { case 1, 2 -> 10 // c\n; \
                  default -> throw error // error\n; }; \
                  return n // seed\n; /* counted */ } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int f() {
        if (c) return d; // c
        while (a) throw e; // c
        if (c) return; // c
        if (c) return d; /* a */ // c
        if (c) return d; // a comment past the line width of one hundred columns does not break the statement
        int y = switch (x) {
          case 1, 2 -> 10; // c
          default -> throw error; // error
        };
        return n; // seed
        /* counted */
      }
    }
    ");
}

#[test]
fn switch_terminator_comments_preserve_configured_contracts() {
    let comments =
        ["/* c */", "// c", "/* first\n * second */", "/** doc */", "/* c */ // d", "/** */ // c"];
    for width in [40, 80] {
        for (indent, indent_style) in
            [(2, IndentStyle::Space), (4, IndentStyle::Space), (4, IndentStyle::Tab)]
        {
            let options = Options {
                line_width: line_width(width),
                indent_width: indent_width(indent),
                indent_style,
                line_ending: LineEnding::Crlf,
                insert_final_newline: false,
                javadoc: Some(JavadocOptions::default()),
                ..Options::default()
            };
            for comment in comments {
                for statement in ["break", "handle()", "return code", "yield 10"] {
                    for following in ["default: z();", ""] {
                        for gap in [" ", "\n"] {
                            let source = format!(
                                "class A {{ int f() {{ switch (x) {{ case 1: y(); \
                             {statement}{gap}{comment}\n; {following} }} }} }}"
                            );
                            format_stable_with_options(&source, &options);
                            let source = format!(
                                "class A {{ int f() {{ return switch (x) {{ case 1: y(); \
                             {statement}{gap}{comment}\n; {following} }}; }} }}"
                            );
                            format_stable_with_options(&source, &options);
                        }
                    }
                }
                for body in ["compute(first, second, third)", "throw error"] {
                    for gap in [" ", "\n"] {
                        // Pinned in `comments_moved_behind_a_return_or_throw_semicolon_that_break_a_group`.
                        if body == "throw error" && gap == " " && comment.contains('\n') {
                            continue;
                        }
                        let source = format!(
                            "class A {{ int f() {{ return switch (x) {{ case 1, 2 -> \
                             {body}{gap}{comment}\n; default -> 0; }}; }} }}"
                        );
                        format_stable_with_options(&source, &options);
                    }
                }
            }
        }
    }
}

#[test]
fn inline_multiline_terminator_comments_keep_their_line() {
    let source = "class A { int f() { return switch (x) { case 1, 2 -> \
                  compute(first, second, third) /* first\n * second */\n; \
                  default -> 0; }; } \
                  int g() { return switch (x) { default: \
                  yield x /* first\n * second */; }; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int f() {
        return switch (x) {
          case 1, 2 -> compute(first, second, third); /* first
                                                       * second */
          default -> 0;
        };
      }

      int g() {
        return switch (x) {
          default:
            yield x; /* first
                      * second */
        };
      }
    }
    ");
}

#[test]
fn switch_rule_arrow_follows_cast_constants_and_guards() {
    let source = "class A { void f(Object o, int m, boolean b) { \
        switch (m) { case (int) X -> g(); case (int) Y, (int) Z -> (Function<A, B>) a -> b; default -> {} } \
        switch (o) { case Integer i when (b) -> g(); case Long l when b -> g(); \
        case String s when test(x -> x > 0) -> g(); default -> {} } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(Object o, int m, boolean b) {
        switch (m) {
          case (int) X -> g();
          case (int) Y, (int) Z -> (Function<A, B>) a -> b;
          default -> {}
        }
        switch (o) {
          case Integer i when (b) -> g();
          case Long l when b -> g();
          case String s when test(x -> x > 0) -> g();
          default -> {}
        }
      }
    }
    ");
}

#[test]
fn braceless_else_gets_its_own_line() {
    let source = "class A { void f() { if (a) x(); else y(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) x();
        else y();
      }
    }
    ");
}

#[test]
fn braceless_do_body_puts_while_on_its_own_line() {
    let source = "class A { void f() { do x(); while (a); do y() // c\n; while (b); \
                  do z() // d\n; /* t */ while (c); do { w() // e\n; } while (d); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        do x();
        while (a);
        do
          y(); // c
        while (b);
        do
          z(); // d
        /* t */ while (c);
        do {
          w(); // e
        } while (d);
      }
    }
    ");
}

#[test]
fn line_comments_on_a_braceless_do_body_line_trail_the_body() {
    let source = "class A { void f() { do x(); while ( // c\ny); do z(); while // d\n(w); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        do
          x(); // c
        while (y);
        do
          z(); // d
        while (w);
      }
    }
    ");
}

#[test]
fn comments_before_a_postfix_operator_in_an_unbraced_body_are_stable() {
    for body in ["for (;;) k", "if (x) c[k]"] {
        for comment in [" // c\n", "\n// c\n", "\n/* c */\n", " /* c */ // d\n"] {
            let source = format!("class A {{ void f() {{ {body}{comment}++; }} }}");
            check_contract_at_widths_and_modes(&source);
        }
    }
}

#[test]
fn comments_before_a_postfix_operator_in_a_switch_rule_are_stable() {
    for operand in ["k", "c[k]"] {
        for comment in [" // c\n", "\n// c\n", "\n/* c */\n", " /* c */ // d\n"] {
            let source = format!(
                "class A {{ void f(int k) {{ switch (k) {{ default -> {operand}{comment}++; }} }} }}"
            );
            check_contract_at_widths_and_modes(&source);
        }
    }
}

#[test]
fn unbraced_body_that_breaks_starts_an_indented_line() {
    let source = "class A { void f() { if (c) do x(); while (a); else y(); \
                  while (c) if (a) x(); else y(); do while (b) { x(); } while (a); \
                  while (pending > 0) poll(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (c)
          do x();
          while (a);
        else y();
        while (c)
          if (a) x();
          else y();
        do
          while (b) {
            x();
          }
        while (a);
        while (pending > 0) poll();
      }
    }
    ");
}

/// The moved line comment ends the line without breaking the statement's group.
#[test]
fn line_comments_moved_behind_an_unbraced_body_semicolon_keep_its_group_flat() {
    let source = "class A { void f() { while (a) x = 1 // c\n; \
                  if (a) while (b) x = 1 // d\n; \
                  while (a) break // e\n; if (a) continue // f\n; else break; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        while (a) x = 1; // c
        if (a) while (b) x = 1; // d
        while (a) break; // e
        if (a) continue; // f
        else break;
      }
    }
    ");
}

/// A block comment that spans lines, moved behind the `;` of a `return` or
/// `throw`, breaks the group around the statement, and so does a block comment
/// after a moved line comment, which starts the next line. The second pass reads
/// them after the `;`, outside the group.
#[test]
fn comments_moved_behind_a_return_or_throw_semicolon_that_break_a_group() {
    let source = "class A { int f() { while (a) throw e /* first\n * second */; \
                  do return d // c\n; while (a); \
                  { if (c) return n // seed\n; /* counted */ } \
                  return switch (x) { default -> throw error /* first\n * second */; }; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int f() {
        while (a)
          throw e; /* first
                    * second */
        do
          return d; // c
        while (a);
        {
          if (c)
            return n; // seed
          /* counted */
        }
        return switch (x) {
          default ->
            throw error; /* first
                          * second */
        };
      }
    }
    ---------- Not idempotent (second pass) ----------
    class A {
      int f() {
        while (a) throw e; /* first
                            * second */
        do
          return d; // c
        while (a);
        {
          if (c) return n; // seed
          /* counted */
        }
        return switch (x) {
          default -> throw error; /* first
                                   * second */
        };
      }
    }
    ");
}

#[test]
fn comments_before_an_unbraced_then_semicolon_stay_before_else() {
    let source = "class A { void f() { if (a) x() /* c */; else y(); \
                  if (a) return /* d */ /* e */; else return; \
                  if (a) x(); else if (b) y() /* f */; else z(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) x(); /* c */
        else y();
        if (a) return; /* d */ /* e */
        else return;
        if (a) x();
        else if (b) y(); /* f */
        else z();
      }
    }
    ");
}

#[test]
fn comments_after_an_unbraced_body_semicolon_stay_before_the_keyword() {
    let source = "class A { void f() { \
        if (c) foo()\n// own\n;\n// second\nelse bar(); \
        do foo()\n// own\n;\n// second\nwhile (c); \
        if (c) foo()\n// lone\n;\nelse bar(); \
        do foo()\n// lone\n;\nwhile (c); } }";
    assert_snapshot!(format_stable_with_options(source, &Options::default()), @"
    class A {
      void f() {
        if (c) foo();
        // own
        // second
        else bar();
        do foo();
        // own
        // second
        while (c);
        if (c) foo();
        else
          // lone
          bar();
        do foo();
        while (
          // lone
          c
        );
      }
    }
    ");
}

#[test]
fn a_comment_before_else_drops_the_blank_line_before_it() {
    let source = "class Order {
  void ship() {
    if (isPaid()) { send(); }

    // Wait for the payment.
    else { hold(); }
  }
}";
    assert_snapshot!(fmt(source), @"
    class Order {
      void ship() {
        if (isPaid()) {
          send();
        }
        // Wait for the payment.
        else {
          hold();
        }
      }
    }
    ");
}

#[test]
fn comments_before_catch_finally_and_while_drop_the_blank_line_before_them() {
    let source = "class A { void f() {
try { a(); }

// before catch
catch (E e) { b(); }

/* before finally */
finally { c(); }
do { d(); }

// before while
while (e());
do d();

// before while
while (e());
if (g) h();

// before else
else i();
} }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {
          a();
        }
        // before catch
        catch (E e) {
          b();
        }
        /* before finally */
        finally {
          c();
        }
        do {
          d();
        }
        // before while
        while (e());
        do d();
        // before while
        while (e());
        if (g) h();
        // before else
        else i();
      }
    }
    ");
}

#[test]
fn own_line_comments_before_escaped_body_semicolons_keep_their_indent() {
    for semicolon in [";", r"\u003b", r"\u0020\uu003B"] {
        let source = format!(
            "class A {{ void f() {{ \
            do label: work()\n// before\n{semicolon}\n// after\nwhile (waiting); \
            do while (ready) work()\n// before\n{semicolon}\n// after\nwhile (waiting); }} }}"
        );
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            insta::allow_duplicates! { assert_snapshot!(format_stable_with_options(&source, &options), @"
            class A {
              void f() {
                do
                  label:
                  work();
                // before
                // after
                while (waiting);
                do while (ready) work();
                // before
                // after
                while (waiting);
              }
            }
            "); }
        }
    }
}

#[test]
fn block_comment_moved_behind_a_do_body_semicolon_keeps_its_indent() {
    let source = "class A { void f() { do run() /* c\n * d */ ; while (x); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        do
          run(); /* c
                  * d */
        while (x);
      }
    }
    ");
}

#[test]
fn empty_control_bodies_keep_their_semicolon() {
    let source = "class A { void f() { while (poll()); for (;;); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        while (poll());
        for (;;);
      }
    }
    ");
}

#[test]
fn comments_before_a_statement_head_follow_its_opener() {
    let source = "class A { A() { super /* a */ (1); } A(int x) { this /* b */ (x, 1); } \
                  A(long x) { <T> /* c */ this(1); } void m() { \
                  if /* d */ (x) {} else if /* e */ (y) {} while /* f */ (x) {} \
                  do {} while /* g */ (x); switch /* h */ (x) {} synchronized /* i */ (x) {} \
                  try /* j */ (R r = r()) {} catch /* k */ (E e) {} \
                  for /* m */ (int i : xs) {} } \
                  A(short x) { o.super /* n */ (1); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      A() {
        super /* a */(1);
      }

      A(int x) {
        this /* b */(x, 1);
      }

      A(long x) {
        <T>/* c */ this(1);
      }

      void m() {
        if (/* d */ x) {} else if (/* e */ y) {}
        while (/* f */ x) {}
        do {} while (/* g */ x);
        switch (/* h */ x) {}
        synchronized (/* i */ x) {}
        try (/* j */ R r = r()) {} catch (/* k */ E e) {}
        for (/* m */ int i : xs) {}
      }

      A(short x) {
        o.super /* n */(1);
      }
    }
    ");
}

#[test]
fn line_comments_before_a_statement_head_follow_its_opener() {
    let source = "class A { void m() { if // a\n(x) {} try {} catch // c\n(E e) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        if (
          // a
          x
        ) {}
        try {} catch (
          // c
          E e
        ) {}
      }
    }
    ");
}

/// An empty `for` head has no clause to take the comments before its `(`, so
/// they lead the body.
#[test]
fn comments_before_an_empty_for_head_lead_the_body() {
    let source = "class A { void m() { for /* l */ (;;) {} for\n/* b */\n(;;) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        for (;;) /* l */ {}
        for (;;) {
          /* b */
        }
      }
    }
    ");
}

#[test]
fn comments_at_for_and_switch_commas_format_in_one_pass() {
    for comment in [
        "/* owner */",
        "// owner\n",
        "/* block */ // line\n",
        "\n/* owner */\n",
        r"\u000a/* owner */\u000a",
        "\n/** */ // owner\n",
        "\n/** owner */\n",
    ] {
        let sources = [
            format!(
                r"class A {{ void f() {{ for (i = lo\u002c {comment}j = hi; i < j; i++, j--) {{}} }} }}"
            ),
            format!(
                r"class A {{ void f() {{ for (i = lo\uuu002C {comment}j = hi; i < j; i++, j--) {{}} }} }}"
            ),
            format!(
                "class A {{ void f() {{ for (i = lo {comment}, j = hi; i < j; i++, j--) {{}} }} }}"
            ),
            format!(
                "class A {{ void f() {{ for (i = lo, j = hi; i < j; i++ {comment}, j--) {{}} }} }}"
            ),
            format!(
                "class A {{ void f() {{ for (i = lo, {comment}j = hi; i < j; i++, j--) {{}} }} }}"
            ),
            format!(
                "class A {{ void f() {{ for (i = lo, j = hi; i < j; i++, {comment}j--) {{}} }} }}"
            ),
            format!(
                "class A {{ int f() {{ return switch (x) {{ case null, {comment} default -> 0; }}; }} }}"
            ),
            format!(
                "class A {{ int f() {{ return switch (x) {{ case null {comment}, default -> 0; }}; }} }}"
            ),
            format!(
                "class A {{ void f() {{ switch (x) {{ case null {comment}, default: run(); }} }} }}"
            ),
            format!("class A {{ void f() {{ try (factory() {comment}.owned;) {{}} }} }}"),
            format!("class A {{ void f() {{ try (factory().{comment}owned;) {{}} }} }}"),
        ];
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { javadoc, ..Options::default() };
            for source in &sources {
                format_stable_with_options(source, &options);
            }
        }
    }
}

#[test]
fn comments_at_loop_and_constructor_boundaries_keep_their_side() {
    let source = "class A { A() { super() /* a */; } A(int x) { this(/* b */) /* c */; } \
                  void m() { do {} /* d */ while (/* e */ x /* f */) /* g */; \
                  for (/* h */ int i : xs) {} for (/* i */ final int j : xs) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      A() {
        super(); /* a */
      }

      A(int x) {
        this(/* b */); /* c */
      }

      void m() {
        do {} /* d */ while (/* e */ x /* f */); /* g */
        for (/* h */ int i : xs) {}
        for (/* i */ final int j : xs) {}
      }
    }
    ");
}

#[test]
fn comments_ending_an_enhanced_for_colon_line_trail_the_variable() {
    let source = "class A { void f() { for (String x : /* a */ // b\nxs) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        for (String x /* a */ : // b
            xs) {}
      }
    }
    ");
}

#[test]
fn comments_before_an_enhanced_for_close_paren_trail_the_iterable() {
    let source = "class A { void f() { for (int x : xs /* a */) {} for (int x : xs // b\n) {} \
                  for (int x : xs\n/* c */) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        for (int x : xs /* a */) {}
        for (int x : xs // b
        ) {}
        for (int x : xs
        /* c */
        ) {}
      }
    }
    ");
}

#[test]
fn own_line_comments_before_an_enhanced_for_colon_lead_the_statement() {
    let source = "class A { void f() { for (final String value\n// a\n:\n// b\nvalues) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        // a
        for (final String value :
            // b
            values) {}
      }
    }
    ");
}

/// `// own` prints before the statement, so the cast in the variable's
/// annotation sees no line comment riding after its type.
#[test]
fn own_line_comments_leading_an_enhanced_for_leave_its_annotation_unbroken() {
    let source =
        "class A { void f() { for (@A(x = (int) 1L) String value\n// own\n: values) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        // own
        for (@A(x = (int) 1L) String value : values) {}
      }
    }
    ");
}

#[test]
fn enhanced_for_dimension_comments_stay_before_the_colon() {
    let source = "class A { void f() { for (String value // before\n[] : values) {} \
                  for (String value[] /* block */ : values) {} \
                  for (String value[] // after\n: values) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        for (String value // before
          [] :
            values) {}
        for (String value[] /* block */ : values) {}
        for (String value[] // after
        :
            values) {}
      }
    }
    ");
}

#[test]
fn loop_and_constructor_boundary_comments_preserve_configured_contracts() {
    for comment in ["/* c */", "// c\n", "\n/* c */\n", "/* c\n * d */"] {
        let terminator_comment =
            if comment == "/* c\n * d */" { "\n/* c\n * d */\n" } else { comment };
        let sources = [
            format!("class A {{ void m() {{ do {{}} {comment} while (x); }} }}"),
            format!("class A {{ void m() {{ do run(); {comment} while ((x)); }} }}"),
            format!("class A {{ void m() {{ do {{}} while (x) {terminator_comment}; }} }}"),
            format!("class A {{ void m() {{ for ({comment} @B final int i : xs) {{}} }} }}"),
            format!("class A {{ A() {{ super({comment}) {terminator_comment}; }} }}"),
            format!("class A {{ A() {{ <T>this() {terminator_comment}; }} }}"),
            format!("class A {{ A() {{ o.super(x {comment}) {terminator_comment}; }} }}"),
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
fn qualified_constructor_dot_boundary_comments_keep_their_side() {
    let source = "class A { A() { outer /* dot */ .super(1); } A(int x) { outer /* generic */ .<T>super(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      A() {
        outer /* dot */.super(1);
      }

      A(int x) {
        outer /* generic */.<T>super();
      }
    }
    ");
}
