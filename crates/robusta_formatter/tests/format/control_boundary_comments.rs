//! Comment ownership at header→body seams.
//!
//! A body's brace stays on its header's line. A block comment sharing the
//! header's line stays before the brace, and a comment that would push the
//! brace to a later line moves after it, with the comments after it. An empty
//! body's `;` and the comments before it stay at the header indent. An
//! unbraced control body that breaks indents one level, with its own-line
//! comments. Shared test support checks parse success, tokens, comments, and
//! final newlines, and shows a second pass that differs.

use insta::assert_snapshot;
use robusta_formatter::{JavadocOptions, Options};

use crate::fixtures::{
    check_contract_at_widths_and_modes, format_source as fmt, format_source_with_options,
    line_width,
};

// Seam 1: `if (...)` `)` → then body.

#[test]
fn if_line_comment_forces_a_braceless_body_break() {
    let source = "class A { void f() { if (a) // c\nx(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a)
          // c
          x();
      }
    }
    ");
}

#[test]
fn if_same_line_block_comment_hugs_a_block_body() {
    let source = "class A { void f() { if (a) /* c */ { x(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) /* c */ {
          x();
        }
      }
    }
    ");
}

#[test]
fn if_line_comment_follows_the_block_brace() {
    let source = "class A { void f() { if (a) // c\n{ x(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) { // c
          x();
        }
      }
    }
    ");
}

#[test]
fn if_comment_before_an_empty_body_keeps_its_line() {
    let source = "class A { void f() { if (a) /* c */ ; if (b) // d\n; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) /* c */ ;
        if (b) // d
        ;
      }
    }
    ");
}

#[test]
fn if_multiline_block_comment_drops_to_its_own_indented_line() {
    let source = "class A { void f() { if (a) /* x\ny */ z(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a)
          /* x
    y */ z();
      }
    }
    ");
}

#[test]
fn if_multiline_block_comment_on_the_header_line_hugs_a_block() {
    let source = "class A { void f() { if (a) /* x\ny */ { z(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) /* x
    y */ {
          z();
        }
      }
    }
    ");
}

#[test]
fn own_line_comments_before_a_brace_move_into_the_body() {
    let source = "class A { void f()\n/* m */\n{ if (a)\n// c\n{ x(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        /* m */
        if (a) {
          // c
          x();
        }
      }
    }
    ");
}

#[test]
fn doc_style_comments_beside_a_header_stay_ordinary_comments() {
    // Each shares its line with the header, so Javadoc formatting leaves it
    // as written, on the first pass and the second.
    let source = "class A { void f() { outer: /**  x */ for (;;) {} \
                  while (a) /** */\n; if (a) /** */\n{} } }";
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_snapshot!(format_source_with_options(source, &options), @"
    class A {
      void f() {
        outer: /**  x */
        for (;;) {}
        while (a) /** */ ;
        if (a) /** */ {}
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn if_block_comment_left_of_the_paren_stays_with_the_header() {
    // The condition owns comments before its closing `)`, so they cannot fall
    // through the header/body seam and escape after the `if` statement.
    let source = "class A { void f() { if (a /* h */) x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a /* h */) x();
      }
    }
    ");
}

#[test]
fn parenthesized_statement_heads_keep_boundary_comments_inside() {
    // The condition cannot break, so the parentheses break around it, and the
    // break before the `)` keeps the second comment from moving past both the
    // `)` and the body's `{`.
    let source = "class A { void f() { \
                  if (// if-open\nready // if-close\n) {} \
                  while (// while-open\nready // while-close\n) {} \
                  synchronized (// synchronized-open\nlock // synchronized-close\n) {} \
                  switch (// switch-open\nvalue // switch-close\n) {} \
                  } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if ( // if-open
          ready // if-close
        ) {}
        while ( // while-open
          ready // while-close
        ) {}
        synchronized ( // synchronized-open
          lock // synchronized-close
        ) {}
        switch ( // switch-open
          value // switch-close
        ) {}
      }
    }
    ");
}

#[test]
fn statement_boundaries_keep_comments_local() {
    // Each comment still documents the construct it was written against. The
    // bare `return` is the one that visibly moves: its comment belongs to the
    // finished statement, so it prints behind the terminator.
    let source = "class A { void f() { while (/* cond */ true) {} \
                  int z = c ? /* yes */ a : b; \
                  r(() -> /* body */ x); return /* r */; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        while (/* cond */ true) {}
        int z = c ? /* yes */ a : b;
        r(() -> /* body */ x);
        return; /* r */
      }
    }
    ");
}

#[test]
fn statement_punctuation_keeps_comments_local_and_stable() {
    // The `for` header's semicolons separate clauses rather than end a
    // statement, so `/* i */` stays where it was written while the `break` and
    // switch-rule terminators carry their comments across.
    let source = "class A { void f() { for (int i = 0 /* i */; /* first */ i < 2; \
                  /* second */ i++ /* update */) {} do {} while (ready /* close */); \
                  break label /* end */; \
                  switch (x) { default /* arrow */ -> value /* end */; } \
                  try {} catch (Exception e /* catch */) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        for (int i = 0 /* i */; /* first */ i < 2; /* second */ i++ /* update */) {}
        do {} while (ready /* close */);
        break label; /* end */
        switch (x) {
          default /* arrow */ -> value; /* end */
        }
        try {} catch (Exception e /* catch */) {}
      }
    }
    ");
}

#[test]
fn inline_comments_stay_before_labeled_and_resource_separators() {
    let source = "class A { void f() { label /* colon */: break label; \
                  try (R r = open() /* resource */;) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        label /* colon */:
        break label;
        try (R r = open() /* resource */;) {}
      }
    }
    ");
}

#[test]
fn own_line_comments_stay_before_label_and_resource_terminators() {
    let source = "class A { void f() { label\n/* label body */\n: break label; \
                  try (R r = open()\n/* resource boundary */\n;) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        label
        /* label body */:
        break label;
        try (
          R r = open()
          /* resource boundary */;
        ) {}
      }
    }
    ");
}

#[test]
fn line_comments_ride_past_terminators() {
    let source = "class A { void f(int x) throws Exception { label // label\n: for (;;) break label; \
                  switch (x) { case 1 // one\n: case 2, 3 // two\n: y(); } \
                  assert x > 0 // condition\n: \"m\"; \
                  try (R r = open() // first\n; S s = open(); // second\n) {} } }";
    assert_snapshot!(fmt(source), @r#"
    class A {
      void f(int x) throws Exception {
        label: // label
        for (;;) break label;
        switch (x) {
          case 1: // one
          case 2, 3: // two
            y();
        }
        assert x > 0 : // condition
        "m";
        try (
          R r = open(); // first
          S s = open(); // second
        ) {}
      }
    }
    "#);
}

#[test]
fn comments_on_the_last_label_colon_line_trail_the_label() {
    let source = "class A { void f(int x) { switch (x) { case 1: // c\nf(x); default: // d\nbreak; } \
                  switch (x) { case 1: /* a */ // b\nf(x); case 2 // c\n: /* d */\nf(x); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(int x) {
        switch (x) {
          case 1: // c
            f(x);
          default: // d
            break;
        }
        switch (x) {
          case 1 /* a */: // b
            f(x);
          case 2: // c
            /* d */
            f(x);
        }
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
    check_contract_at_widths_and_modes(
        "class A { void f(int x) { switch (x) { case 1: /** d */ // e\nclass L {} } } }",
    );
}

#[test]
fn comments_around_a_default_label_stay_on_their_sides() {
    let source = "class A { void f(int x) { switch (x) { /* a */ default /* b */ -> g(); } \
                  switch (x) { // c\ndefault // d\n-> g(); } \
                  switch (x) { case 1: /* e */ default /* f */: g(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(int x) {
        switch (x) {
          /* a */ default /* b */ -> g();
        }
        switch (x) { // c
          default -> g(); // d
        }
        switch (x) {
          case 1:
          /* e */ default /* f */:
            g();
        }
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn documentation_comments_keep_their_side_of_the_colon_before_a_local_type() {
    let source = "class A { void f(int x) { switch (x) { case 1\n/** Documents nothing. */\n: class L {} \
                  case 2 /// Documents nothing either.\n: class M {} \
                  case 3: /** Documents N. */ // n\nclass N {} \
                  case 4 /** Documents nothing. */ /* o */ : /* p */ /** Documents R. */\nrecord R() {} } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(int x) {
        switch (x) {
          case 1
          /** Documents nothing. */:
            class L {}
          case 2 /// Documents nothing either.
          :
            class M {}
          case 3:
            /** Documents N. */ // n
            class N {}
          case 4 /** Documents nothing. */ /* o */:
            /* p */ /** Documents R. */
            record R() {}
        }
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
    check_contract_at_widths_and_modes(
        "class A { void f(int x) { switch (x) { case 1 /** d */ // e\n: f(x); } } }",
    );
}

/// No declaration follows, so a documentation comment moves as any other.
#[test]
fn documentation_comments_cross_the_colon_before_a_statement() {
    let source = "class A { void f(int x) { switch (x) { case 1 /// d\n: f(x); \
                  case 2: /** e */\ng(x); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(int x) {
        switch (x) {
          case 1: /// d
            f(x);
          case 2 /** e */:
            g(x);
        }
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn own_line_comments_defer_across_case_label_colons() {
    // Only an own-line comment moves past the `:`.
    let source = "class A { void f() { switch (x) { case 1\n/* first */\n: case 2 /* two */: \
                  case 3\n// third\n: y(); default\n/* default */\n: \
                  case Integer i when i > 0 /* guard */: z(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1:
          /* first */
          case 2 /* two */:
          case 3:
            // third
            y();
          default:
          /* default */
          case Integer i when i > 0 /* guard */:
            z();
        }
      }
    }
    ");
}

#[test]
fn header_punctuation_comments_stay_on_a_line_with_code() {
    let source = "class A { void f(Object o) { \
                  for (int i = 0 // init\n; i < n // condition\n;) {} \
                  try {} catch (E e // name\n) {} \
                  switch (o) { case 1: /* one */ case 2: break; } \
                  switch (o) { case 3 // three\n-> {} default -> {} } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(Object o) {
        for (
          int i = 0; // init
          i < n; // condition
        ) {}
        try {} catch (
          E e // name
        ) {}
        switch (o) {
          case 1:
          /* one */ case 2:
            break;
        }
        switch (o) {
          case 3 -> { // three
          }
          default -> {}
        }
      }
    }
    ");
}

#[test]
fn header_opener_comments_break_where_they_print_a_break() {
    // A comment before the `(` breaks where the source breaks after it.
    let source = "class A { void f() { try {} catch\n/* x */ (\n/* c */ E e) {} \
                  try {} catch /* x */ (\n/* c */ E e) {} \
                  try {} catch (/** */\nE e) {} try {} catch (E e\n/** */\n) {} \
                  for (/* a */\n;;) {} } }";
    check_contract_at_widths_and_modes(source);
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {} catch (
          /* x */
          /* c */ E e
        ) {}
        try {} catch (
          /* x */
          /* c */ E e
        ) {}
        try {} catch (
          /** */
          E e
        ) {}
        try {} catch (
          E e
          /** */
        ) {}
        for (/* a */;;) {}
      }
    }
    ");
}

#[test]
fn catch_comments_starting_a_line_break_the_parentheses() {
    // The leading pass starts a line before such a comment, so the
    // parentheses break, as they do in an `if` header.
    let source = "class A { void f() { try {} catch (\n/* c */ E e) {} \
                  try {} catch\n/* x */ (A | B e) {} if (\n/* c */ x) {} } }";
    check_contract_at_widths_and_modes(source);
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {} catch (
          /* c */ E e
        ) {}
        try {} catch (
          /* x */ A | B e
        ) {}
        if (
          /* c */ x
        ) {}
      }
    }
    ");
}

#[test]
fn for_header_clauses_keep_comments_before_their_separators() {
    // An own-line comment stays in its clause, before the `;` or `)`.
    let source = "class A { void f() { \
                  for (int i = 0\n/* init */\n; i < n\n/* condition */\n; i++\n/* update */\n) {} \
                  for (i = 0, j = 0\n// init\n; i < n /* condition */; i++, j++\n/* a\n b */ /* c */\n) x(); \
                  for (\n/* init */\n;\n/* condition */\n;\n/* update */\n) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        for (
          int i = 0
          /* init */;
          i < n
          /* condition */;
          i++
          /* update */
        ) {}
        for (
          i = 0, j = 0
          // init
          ;
          i < n /* condition */;
          i++, j++
          /* a
     b */ /* c */
        )
          x();
        for (
          /* init */;
          /* condition */;
          /* update */
        ) {}
      }
    }
    ");
}

// Seam 2: `else` → branch, including `else if`.

#[test]
fn else_block_branch_keeps_an_inline_seam_comment() {
    let source = "class A { void f() { if (a) { x(); } else /* c */ { y(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) {
          x();
        } else /* c */ {
          y();
        }
      }
    }
    ");
}

#[test]
fn else_if_chain_keeps_an_inline_seam_comment() {
    let source = "class A { void f() { if (a) x(); else /* c */ if (b) y(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) x();
        else /* c */ if (b) y();
      }
    }
    ");
}

#[test]
fn else_if_line_comment_breaks_the_chain() {
    let source = "class A { void f() { if (a) { x(); } else // c\nif (b) y(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) {
          x();
        } else // c
        if (b) y();
      }
    }
    ");
}

// Seams 3–5: `while`, `do`, and both `for` forms.

#[test]
fn while_seam_comment_hugs_then_breaks() {
    let source = "class A { void f() { while (a) /* c */ x(); while (b) // d\ny(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        while (a) /* c */ x();
        while (b)
          // d
          y();
      }
    }
    ");
}

#[test]
fn do_keyword_seam_comment_stays_before_the_body() {
    let source = "class A { void f() { do /* c */ { x(); } while (a); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        do /* c */ {
          x();
        } while (a);
      }
    }
    ");
}

#[test]
fn do_keyword_letter_in_a_printed_body_comment_is_not_the_keyword() {
    let source = "class A { void f() { do x(); // while\n/// we\nwhile (c); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        do
          x(); // while
        /// we
        while (c);
      }
    }
    ");
}

#[test]
fn for_seam_comment_covers_basic_and_enhanced() {
    let source = "class A { void f() { for (int i = 0; i < n; i++) /* c */ x(); for (int i = 0; i < n; i++) // e\nx(); for (int v : vs) // d\ng(v); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        for (int i = 0; i < n; i++) /* c */ x();
        for (int i = 0; i < n; i++)
          // e
          x();
        for (int v : vs)
          // d
          g(v);
      }
    }
    ");
}

// Seam 6: `switch (...)` `)` → block.

#[test]
fn switch_block_seam_comment_hugs_the_block() {
    let source = "class A { void f() { switch (x) /* c */ { case 1 -> f(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) /* c */ {
          case 1 -> f();
        }
      }
    }
    ");
}

#[test]
fn switch_block_seam_line_comment_follows_the_brace() {
    let source = "class A { void f() { switch (x) // c\n{ case 1 -> f(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) { // c
          case 1 -> f();
        }
      }
    }
    ");
}

// Seam 7: `synchronized (...)` `)` → block.

#[test]
fn synchronized_seam_comment_hugs_the_block() {
    let source = "class A { void f() { synchronized (a) /* c */ { x(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        synchronized (a) /* c */ {
          x();
        }
      }
    }
    ");
}

// Seams 8–10: `try`, `catch`, and `finally`.

#[test]
fn try_keyword_seam_comment_stays_before_the_block() {
    let source = "class A { void f() { try /* c */ { x(); } catch (E e) { y(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try /* c */ {
          x();
        } catch (E e) {
          y();
        }
      }
    }
    ");
}

#[test]
fn try_with_resources_seam_comment_stays_before_the_block() {
    let source = "class A { void f() { try (R r = a()) /* c */ { x(); } catch (E e) { y(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try (R r = a()) /* c */ {
          x();
        } catch (E e) {
          y();
        }
      }
    }
    ");
}

#[test]
fn catch_seam_comment_stays_before_the_block() {
    let source = "class A { void f() { try { x(); } catch (E e) /* c */ { y(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {
          x();
        } catch (E e) /* c */ {
          y();
        }
      }
    }
    ");
}

#[test]
fn finally_seam_comment_stays_before_the_block() {
    let source = "class A { void f() { try { x(); } finally /* c */ { y(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {
          x();
        } finally /* c */ {
          y();
        }
      }
    }
    ");
}

// Seam 11: switch-rule `->` → expression, block, or throw body.

#[test]
fn switch_rule_expression_body_keeps_an_inline_seam_comment() {
    let source = "class A { void f() { switch (x) { case 1 -> /* c */ f(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1 -> /* c */ f();
        }
      }
    }
    ");
}

#[test]
fn switch_rule_block_body_keeps_an_inline_seam_comment() {
    let source = "class A { void f() { switch (x) { case 1 -> /* c */ { f(); } } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1 -> /* c */ {
            f();
          }
        }
      }
    }
    ");
}

#[test]
fn switch_rule_throw_body_keeps_an_inline_seam_comment() {
    let source = "class A { void f() { switch (x) { case 1 -> /* c */ throw e; } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1 -> /* c */ throw e;
        }
      }
    }
    ");
}

#[test]
fn switch_rule_arrow_line_comment_trails_the_rule() {
    let source = "class A { void f() { switch (x) { case 1 -> // c\nf(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1 -> f(); // c
        }
      }
    }
    ");
}

#[test]
fn switch_rule_line_comment_before_a_block_follows_the_brace() {
    let source = "class A { void f() { switch (x) { case 1 // c\n-> { f(); } \
                  case 2 -> // d\n{ g(); } } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1 -> { // c
            f();
          }
          case 2 -> { // d
            g();
          }
        }
      }
    }
    ");
}

#[test]
fn switch_group_line_comment_before_a_lone_block_follows_the_brace() {
    let source = "class A { void f() { switch (x) { case 1 // c\n: { f(); } \
                  case 2 /* d */\n// e\n: { g(); } } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        switch (x) {
          case 1: { // c
            f();
          }
          case 2 /* d */: {
            // e
            g();
          }
        }
      }
    }
    ");
}

// Seams 12–14: a method's, constructor's, and compact constructor's parameter
// list `)` → body. The seam is the `)`, not the last parameter, so a comment
// still inside the parameter list is left to the list itself.

#[test]
fn method_seam_line_comment_follows_the_body_brace() {
    let source = "class A { void f() // c\n{ x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() { // c
        x();
      }
    }
    ");
}

#[test]
fn method_seam_block_comment_hugs_the_body() {
    let source = "class A { void f() /* c */ { x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() /* c */ {
        x();
      }
    }
    ");
}

#[test]
fn type_body_seam_block_comments_stay_before_the_brace() {
    let source = "class X /* class */ {}\n\
                  class G<T> /* generic */ {}\n\
                  interface I<T> /* interface */ {}\n\
                  enum E /* enum */ { A }\n\
                  record R(int x) /* record */ {}\n\
                  @interface B /* annotation */ {}";
    assert_snapshot!(fmt(source), @"
    class X /* class */ {}

    class G<T> /* generic */ {}

    interface I<T> /* interface */ {}

    enum E /* enum */ {
      A,
    }

    record R(int x) /* record */ {}

    @interface B /* annotation */ {}
    ");
}

#[test]
fn type_body_seam_line_comments_follow_the_brace() {
    let source = "class X // class\n{}\n\
                  class G<T> // generic\n{ int x; }\n\
                  interface I<T> // interface\n{}\n\
                  enum E // enum\n{ A }\n\
                  record R(int x) // record\n{}\n\
                  @interface B // annotation\n{}";
    assert_snapshot!(fmt(source), @"
    class X { // class
    }

    class G<T> { // generic
      int x;
    }

    interface I<T> { // interface
    }

    enum E { // enum
      A,
    }

    record R(int x) { // record
    }

    @interface B { // annotation
    }
    ");
}

#[test]
fn type_body_seam_own_line_comments_move_into_the_body() {
    let source = "class X\n/* head */\n{ /* body */ int x; }\n\
                  record R(int x) /* first\nsecond */ {}\n\
                  class Outer { interface I<T>\n// head\n{ /* body */ } }";
    assert_snapshot!(fmt(source), @"
    class X {
      /* head */
      /* body */ int x;
    }

    record R(int x) /* first
    second */ {}

    class Outer {
      interface I<T> {
        // head
        /* body */
      }
    }
    ");
}

#[test]
fn method_seam_comment_follows_a_throws_clause() {
    let source = "class A { void f() throws E // c\n{ x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() throws E { // c
        x();
      }
    }
    ");
}

#[test]
fn method_seam_leaves_a_parameter_list_comment_to_the_list() {
    let source = "class A { void f(int a /* c */) { x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(int a /* c */) {
        x();
      }
    }
    ");
}

#[test]
fn constructor_seam_line_comment_follows_the_body_brace() {
    let source = "class A { A() // c\n{ x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      A() { // c
        x();
      }
    }
    ");
}

#[test]
fn compact_constructor_seam_line_comment_follows_the_body_brace() {
    let source = "record A(int a) { A // c\n{ x(); } }";
    assert_snapshot!(fmt(source), @"
    record A(int a) {
      A { // c
        x();
      }
    }
    ");
}

// Seam 15: a static initializer's `static` keyword → block. An instance
// initializer has no header, and so no seam.

#[test]
fn static_initializer_seam_line_comment_follows_the_block_brace() {
    let source = "class A { static // c\n{ x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      static { // c
        x();
      }
    }
    ");
}

#[test]
fn static_initializer_seam_block_comment_hugs_the_block() {
    let source = "class A { static /* c */ { x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      static /* c */ {
        x();
      }
    }
    ");
}

// The far side of a continuation seam: the gap between a block's `}` and the
// keyword that continues the same statement. A comment written there documents
// the keyword, not the branch it introduces, so it stays in front of the
// keyword instead of sinking into the following body. An own-line comment puts
// the keyword on its own line, because the alternative is moving the comment.

#[test]
fn own_line_comment_before_else_stays_in_front_of_the_keyword() {
    let source = "class A { void f() { if (a) { x(); }\n// why\nelse { y(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) {
          x();
        }
        // why
        else {
          y();
        }
      }
    }
    ");
}

#[test]
fn inline_block_comment_before_else_stays_between_the_brace_and_the_keyword() {
    let source = "class A { void f() { if (a) { x(); } /* why */ else { y(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) {
          x();
        } /* why */ else {
          y();
        }
      }
    }
    ");
}

#[test]
fn inline_line_comment_before_else_keeps_its_line_and_breaks_the_keyword() {
    let source = "class A { void f() { if (a) { x(); } // why\nelse { y(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        if (a) {
          x();
        } // why
        else {
          y();
        }
      }
    }
    ");
}

#[test]
fn own_line_comment_before_catch_and_finally_stays_in_front_of_each_keyword() {
    let source = "class A { void f() { try { x(); }\n// why catch\ncatch (E e) { y(); }\n// why finally\nfinally { z(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {
          x();
        }
        // why catch
        catch (E e) {
          y();
        }
        // why finally
        finally {
          z();
        }
      }
    }
    ");
}

#[test]
fn comments_after_an_escaped_form_feed_stay_before_catch() {
    let source = r"class A { void f() { try { x(); } \u000c/* c */ catch (E e) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {
          x();
        } /* c */ catch (E e) {}
      }
    }
    ");
}

#[test]
fn inline_line_comments_before_catch_and_finally_keep_each_keyword_on_its_next_line() {
    let source = "class A { void f() { try { x(); } // first catch\n\
                  catch (E e) { y(); } // second catch\n\
                  catch (F e) { z(); } // finally\nfinally {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        try {
          x();
        } // first catch
        catch (E e) {
          y();
        } // second catch
        catch (F e) {
          z();
        } // finally
        finally {}
      }
    }
    ");
}

#[test]
fn new_line_placement_keeps_continuation_seam_comments_in_front_of_the_keyword() {
    use robusta_formatter::{AfterClosingBrace, Options};

    use crate::fixtures::format_source_with_options;

    let options = Options { after_closing_brace: AfterClosingBrace::NewLine, ..Options::default() };
    let source = "class A { void f() { if (a) { x(); } /* why */ else { y(); } \
                  try { x(); } // why catch\ncatch (E e) { y(); }\n// why finally\nfinally { z(); } } }";
    assert_snapshot!(format_source_with_options(source, &options), @"
    class A {
      void f() {
        if (a) {
          x();
        } /* why */
        else {
          y();
        }
        try {
          x();
        } // why catch
        catch (E e) {
          y();
        }
        // why finally
        finally {
          z();
        }
      }
    }
    ");
}

#[test]
fn continuation_seam_comments_are_stable_in_both_placements() {
    use robusta_formatter::{AfterClosingBrace, Options};

    use crate::fixtures::format_stable_with_options;

    for after_closing_brace in [AfterClosingBrace::SameLine, AfterClosingBrace::NewLine] {
        let options = Options { after_closing_brace, ..Options::default() };
        for comment in ["/* c */", "// c\n", "\n// c\n", "\n/* c */\n", "/* c\n * d */"] {
            for source in [
                format!("class A {{ void f() {{ if (a) {{ x(); }} {comment} else {{ y(); }} }} }}"),
                format!(
                    "class A {{ void f() {{ if (a) {{ x(); }} {comment} else if (b) {{ y(); }} }} }}"
                ),
                format!(
                    "class A {{ void f() {{ try {{ x(); }} {comment} catch (E e) {{ y(); }} \
                     {comment} finally {{ z(); }} }} }}"
                ),
            ] {
                format_stable_with_options(&source, &options);
            }
        }
    }
}

#[test]
fn own_line_comment_before_a_declaration_clause_keyword_stays_in_front_of_it() {
    let source = "class A\n// why extends\nextends B {}\n\
                  enum E\n// why implements\nimplements C { X }";
    assert_snapshot!(fmt(source), @"
    class A
        // why extends
        extends B {}

    enum E
        // why implements
        implements C {
      X,
    }
    ");
}

#[test]
fn inline_comment_before_a_declaration_clause_keyword_keeps_its_line() {
    let source = "record R(int a) // note\nimplements I {}";
    assert_snapshot!(fmt(source), @"
    record R(int a) // note
        implements I {}
    ");
}

#[test]
fn inline_comment_after_a_multiline_head_keeps_its_line() {
    let source = "record R(int a,\n    int b) // note\n    implements I {}\n\
                  class A<T,\n    U> // note\n    extends B {}\n\
                  interface J<T,\n    U extends Comparable<U>> // note\n    extends K {}\n\
                  record Address(String firstName, String lastName,\n        \
                  String street, String city,\n        \
                  String postalCode, String country) // fields\n    implements Named {}";
    assert_snapshot!(fmt(source), @"
    record R(int a, int b) // note
        implements I {}

    class A<T, U> // note
        extends B {}

    interface J<T, U extends Comparable<U>> // note
        extends K {}

    record Address(
      String firstName,
      String lastName,
      String street,
      String city,
      String postalCode,
      String country
    ) // fields
        implements Named {}
    ");
}

#[test]
fn comments_between_a_split_list_and_its_clause_keep_their_lines() {
    let source = "class A { void f(Customer customer, List<Item> items) /* why */ throws E {} \
                  void g(Customer customer, List<Item> items) // why\nthrows E {} \
                  void h(Customer customer, List<Item> items)\n// why\nthrows E {} \
                  void i(Customer customer, List<Item> items) throws E /* a */ // c\n{} }";
    let options = Options { line_width: line_width(40), ..Options::default() };
    assert_snapshot!(format_source_with_options(source, &options), @"
    class A {
      void f(
        Customer customer,
        List<Item> items
      ) /* why */ throws E {}

      void g(
        Customer customer,
        List<Item> items
      ) // why
          throws E {}

      void h(
        Customer customer,
        List<Item> items
      )
          // why
          throws E {}

      void i(
        Customer customer,
        List<Item> items
      ) throws E /* a */ { // c
      }
    }
    ");
}

#[test]
fn line_comment_after_a_declaration_clause_keeps_the_next_clause_indented() {
    let source = "class A\n    extends B    // why\n    implements C\n{\n    int x;\n}\n\
                  interface I extends J // why\npermits K {}";
    assert_snapshot!(fmt(source), @"
    class A extends B // why
        implements C {
      int x;
    }

    interface I extends J // why
        permits K {}
    ");
}

#[test]
fn own_line_comment_between_declaration_clauses_stays_at_the_clause_indent() {
    let source = "class A extends B\n// why implements\nimplements C {}";
    assert_snapshot!(fmt(source), @"
    class A extends B
        // why implements
        implements C {}
    ");
}

#[test]
fn own_line_comment_before_throws_stays_in_front_of_the_keyword() {
    let source = "class A { void f()\n// why throws\nthrows E { x(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f()
          // why throws
          throws E {
        x();
      }
    }
    ");
}

#[test]
fn anonymous_class_seam_block_comment_hugs_the_body() {
    let source = "class A { void f() { Foo x = new Foo(a, b) /* why */ { int y; }; } }";
    assert_eq!(
        fmt(source),
        "class A {\n  void f() {\n    Foo x = new Foo(a, b) /* why */ {\n      int y;\n    };\n  }\n}\n"
    );
}
