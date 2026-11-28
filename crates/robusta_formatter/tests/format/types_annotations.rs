//! Types, annotations, and variable-type forms.

use cow_utils::CowUtils;
use insta::assert_snapshot;

use crate::fixtures::{check_contract_at_widths_and_modes, format_source as fmt, format_stable};

fixture_tests! {
    golden_generic_and_array_types_file => "types-and-annotations/generic-and-array-types";
    golden_var_inferred_variable_types_file => "types-and-annotations/var-inferred-variable-types";
    golden_unnamed_variables_file => "types-and-annotations/unnamed-variables";
    golden_annotations_file => "types-and-annotations/annotations";
    golden_type_annotations_file => "types-and-annotations/type-annotations";
    golden_qualified_inner_types_file => "types-and-annotations/qualified-inner-types";
}

#[test]
fn primitive_and_void_keywords_print_without_escapes_and_names_keep_theirs() {
    let source = "class A { \\u0069nt a; Object b = \\u0076oid.class; L\\u0069st c; \
                  \\u0076oid f() { Sys\\u0074em.\\u006fut.println(a); } }";
    assert_snapshot!(fmt(source), @r"
    class A {
      int a;
      Object b = void.class;
      L\u0069st c;

      void f() {
        Sys\u0074em.\u006fut.println(a);
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn annotation_interface_comments_after_the_at_sign_lead_the_name() {
    let source = "@ /* block */ interface A {}\n\
                  @\n// own line\ninterface B {}\n\
                  @Deprecated public \\u0040 // trailing\n\\u0069nterface /* after */ C {}";
    assert_snapshot!(fmt(source), @"
    @interface /* block */ A {}

    @interface
    // own line
    B {}

    @Deprecated
    public @interface // trailing
    /* after */ C {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn empty_annotation_arrays_drop_the_optional_comma_and_keep_comments() {
    let source = "@interface A { int[] value() default {/* before */, /* after */}; } \
                  class C { @SuppressWarnings({,}) int e = 0; \
                  @A(value = {/* before */, // after\n}) int f; }";
    assert_snapshot!(fmt(source), @"
    @interface A {
      int[] value() default {
        /* before */
        /* after */
      };
    }

    class C {
      @SuppressWarnings({}) int e = 0;
      @A(
        value = {
          /* before */
          // after
        }
      )
      int f;
    }
    ");
}

#[test]
fn annotation_element_comments_stay_on_each_side_of_default() {
    let source = "@interface A { int value() /* before */ default /* after */ 1; \
                  int other() /* dimension */ [] /* before */ default /* after */ {1}; \
                  int escaped\\u0028/* inside */\\u0029 /* before */ default /* after */ 1; }";
    assert_snapshot!(fmt(source), @"
    @interface A {
      int value() /* before */ default /* after */ 1;

      int other() /* dimension */[] /* before */ default /* after */ {1};

      int escaped(/* inside */) /* before */ default /* after */ 1;
    }
    ");
}

#[test]
fn annotation_element_default_comments_follow_configured_contracts() {
    use crate::fixtures::{format_stable_with_options, line_width};
    use robusta_formatter::{JavadocOptions, LineEnding, Options};

    for (ty, dimensions, value) in [
        ("int", "", "1"),
        ("int[]", "", "{1, 2}"),
        ("int", " /* dimension */ []", "{1, 2}"),
        ("int[][]", "", "{{1}, {2}}"),
        ("B", "", "@B(x = 1)"),
    ] {
        for (before, after) in [
            (" /* before */ ", " /* after */ "),
            (r"\u0020/* before */ ", " /* after */ "),
            (r"\uu0009/* before */ ", " /* after */ "),
            (" // before\n", " // after\n"),
            ("\n/* before */\n", "\n/* after */\n"),
            ("\n/* before\n * continued */\n", "\n/* after\n * continued */\n"),
            ("\n/** before */\n", "\n/** after */\n"),
            ("\n/** */\n", "\n/** */\n"),
        ] {
            for keyword in ["default", r"\u0064efault", r"def\uu0061ult"] {
                let source = format!(
                    "@interface A {{ {ty} value(){dimensions}{before}{keyword}{after}{value}; }}"
                );
                for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
                    for javadoc in [None, Some(JavadocOptions::default())] {
                        let options = Options {
                            line_width: line_width(60),
                            line_ending,
                            javadoc,
                            ..Options::default()
                        };
                        format_stable_with_options(&source, &options);
                    }
                }
            }
        }
    }
}

#[test]
fn a_markdown_comment_before_an_annotation_default_does_not_break_the_header() {
    use crate::fixtures::{format_stable_with_options, line_width};
    use robusta_formatter::{JavadocOptions, Options};

    let source = "@interface Ann { int count()/// first\ndefault 2; }";
    for javadoc in [None, Some(JavadocOptions::default())] {
        let options = Options { line_width: line_width(30), javadoc, ..Options::default() };
        assert_eq!(
            format_stable_with_options(source, &options),
            "@interface Ann {\n  int count() default 2 /// first\n  ;\n}\n"
        );
    }
}

#[test]
fn a_markdown_hard_break_before_a_terminator_stays_with_its_comment() {
    use crate::fixtures::{format_stable_with_options, line_width};
    use robusta_formatter::{JavadocOptions, Options};

    let source = "@interface Ann { int count() default 2 /// first  \n/// second\n; }";
    for javadoc in [None, Some(JavadocOptions::default())] {
        let options = Options { line_width: line_width(30), javadoc, ..Options::default() };
        assert_eq!(
            format_stable_with_options(source, &options),
            "@interface Ann {\n  int count() default 2 /// first  \n  /// second\n  ;\n}\n"
        );
    }
}

#[test]
fn type_bound_comments_stay_before_their_ampersands() {
    let source = "class A<T extends B\n/* first */\n& C\n// second\n& D> {}";
    assert_snapshot!(fmt(source), @"
    class A<
        T extends
            B
                /* first */
                & C
                // second
                & D> {}
    ");
}

#[test]
fn a_comment_with_code_after_it_on_its_line_leads_the_bound_after_its_ampersand() {
    let source = "class A<T extends B\n/* before */ & C, U extends D &\n/* after */ E> {}";
    assert_snapshot!(fmt(source), @"class A<T extends B & /* before */ C, U extends D & /* after */ E> {}");
}

#[test]
fn intersection_cast_comments_stay_before_their_ampersands() {
    let source = "class A { Object m(Object x) { \
        return (B\n/* first */\n& C\n// second\n& D) x; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object m(Object x) {
        return (
          B
            /* first */
            & C
            // second
            & D
        ) x;
      }
    }
    ");
}

#[test]
fn comments_before_a_dimension_bracket_stay_before_it_and_a_cast_paren_keeps_only_same_line_ones() {
    let source = "class A { Object m(Object x) { Object a = (Runnable /* why */\n) x; \
        Object b = new int @A /* sized */\n[3]; int[] c = new int[\n// sized\n3]; \
        return (B\n/* last */\n) x; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object m(Object x) {
        Object a = (Runnable /* why */) x;
        Object b = new int @A /* sized */ [3];
        int[] c = new int[
          // sized
          3
        ];
        return (B)
          /* last */
          x;
      }
    }
    ");
}

#[test]
fn type_bound_comments_follow_configured_contracts() {
    use robusta_formatter::{JavadocOptions, LineEnding, Options};

    let sources = [
        "class A<T extends B\n/* first */\n& C\n// second\n& D> {}",
        "class A<T extends B /* before */ & /* after */ C, U extends D> {}",
        "class A<T extends B\n/* first\nsecond */\n& @X C> {}",
        "class A<T extends B &\n/* first\nsecond */\nC> {}",
        "class A<T extends B\n/** */\n& C> {}",
        "class A<T extends B\n/** bound */\n& C> {}",
        "class A { Object m(Object x) { return (B\n/* first */\n& C\n// second\n& D) x; } }",
        "class A { Object m(Object x) { return (B & C\n/* first\nsecond */\n& @X D) x; } }",
        "class A { Object m(Object x) { return (B /* before */ & /* after */ C) x; } }",
        "class A { Object m(Object x) { return (B\n/** */\n& C) x; } }",
    ];
    for source in sources {
        for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
            for javadoc in [None, Some(JavadocOptions::default())] {
                let options = Options { line_ending, javadoc, ..Options::default() };
                crate::fixtures::check_formatter_contract_with_options(source, &options)
                    .unwrap_or_else(|err| panic!("{source:?}: {err}"));
            }
        }
    }
}

#[test]
fn field_marker_annotations_break_one_per_line_when_too_wide() {
    let source = "class M { int before; \
         @Alpha @Bravo @Charlie @Delta @Echo @Foxtrot @Golf @Hotel @India @Juliett \
         ExtremelyLongDataLoaderTypeName fieldWithAnExtremelyLongName; int after; }";
    assert_snapshot!(fmt(source), @"
    class M {
      int before;
      @Alpha
      @Bravo
      @Charlie
      @Delta
      @Echo
      @Foxtrot
      @Golf
      @Hotel
      @India
      @Juliett
      ExtremelyLongDataLoaderTypeName fieldWithAnExtremelyLongName;
      int after;
    }
    ");
}

#[test]
fn field_annotations_stack_after_a_source_line_break() {
    let source = r#"class C { int first; @GuardedBy("this") int second; @First(1)
                      @Second int third; @A @B
                      int fourth; @A\u000aint fifth; int sixth; }"#;
    assert_snapshot!(fmt(source), @r#"
    class C {
      int first;
      @GuardedBy("this") int second;
      @First(1)
      @Second
      int third;
      @A
      @B
      int fourth;
      @A
      int fifth;
      int sixth;
    }
    "#);
}

#[test]
fn parameterless_field_annotations_stay_inline_and_compact() {
    let source = "class C { int first; @Nullable int second; @First @Second int third; \
                  @Empty() int fourth; int fifth; }";
    assert_snapshot!(fmt(source), @"
    class C {
      int first;
      @Nullable int second;
      @First @Second int third;
      @Empty() int fourth;
      int fifth;
    }
    ");
}

#[test]
fn annotated_field_spacing_keeps_comment_owners() {
    let source = r#"class C { int first; // first trailing
                      // second leading
                      @GuardedBy("this") int second; // second trailing
                      // third leading
                      int third; }"#;
    assert_snapshot!(fmt(source), @r#"
    class C {
      int first; // first trailing
      // second leading
      @GuardedBy("this") int second; // second trailing
      // third leading
      int third;
    }
    "#);
}

#[test]
fn comments_between_annotations_lead_or_trail_them() {
    let source = "class C {\n\
                  @A // a\n@B\nvoid a() {}\n\
                  @A\n// b\n@B\nvoid b() {}\n\
                  @A /* c */ @B\nvoid c() {}\n\
                  @A /* d */ public void d() {}\n\
                  @A\n\n// e\n\n@B\nvoid e() {}\n\
                  @A\n\n// f\n\nvoid f() {}\n\
                  @A // g\n@B int g;\n\
                  @A /* h */ @B int h; }";
    assert_snapshot!(fmt(source), @"
    class C {
      @A // a
      @B
      void a() {}

      @A
      // b
      @B
      void b() {}

      @A
      /* c */ @B
      void c() {}

      @A
      /* d */ public void d() {}

      @A
      // e

      @B
      void e() {}

      @A
      // f

      void f() {}

      @A // g
      @B
      int g;
      @A /* h */ @B int h;
    }
    ");
}

#[test]
fn a_comment_starting_a_line_after_the_last_annotation_leads_the_code_on_its_line() {
    let source = "/** Shop. */\n@Deprecated\n\n/* c */ class C {\n\
                  @Deprecated\n\n/* m */ void m() {\n\
                  @SuppressWarnings(\"x\")\n\n/* l */ int l = 1;\n\
                  @SuppressWarnings(\"x\")\n// l\nint k = 2; }\n\
                  @Deprecated\n\n/* f */ int f;\n\
                  @Deprecated\n// g\nint g; }";
    assert_snapshot!(format_stable(source), @"
    /** Shop. */
    @Deprecated
    /* c */ class C {
      @Deprecated
      /* m */ void m() {
        @SuppressWarnings(\"x\")
        /* l */ int l = 1;
        @SuppressWarnings(\"x\")
        // l
        int k = 2;
      }

      @Deprecated
      /* f */ int f;
      @Deprecated
      // g
      int g;
    }
    ");
}

#[test]
fn comments_after_inline_annotations_lead_the_code_on_their_line() {
    use crate::fixtures::format_stable_with_options;
    use robusta_formatter::{JavadocOptions, Options};

    let source = "record R(@A\n\n/* r */ int x) {}\n\
                  enum E { @A\n\n/* e */ ONE }\n\
                  class C { void m(@A\n\n/* p */ int x, final @A\n/* q */ int y) {} }";
    let expected = format_stable(source);
    assert_snapshot!(expected, @"
        record R(
          @A
          /* r */ int x
        ) {}

        enum E {
          @A
          /* e */ ONE,
        }

        class C {
          void m(
            @A
            /* p */ int x,
            final @A
            /* q */ int y
          ) {}
        }
        ");
    for javadoc in [None, Some(JavadocOptions::default())] {
        let options = Options { javadoc, ..Options::default() };
        assert_eq!(format_stable_with_options(source, &options), expected);
    }
}

#[test]
fn comments_after_inline_annotations_and_keyword_modifiers_keep_their_lines() {
    let source = "class C { void m(@A\n\n// a\nint x, final\n/* b */ int y) {\n\
                  for (@A\n\n/* c */ int i = 0; i < 1; i++) {}\n\
                  for (@A\n/* d */ String s : values) {}\n\
                  try (@A\n/* e */ Resource r = open()) {}\n\
                  catch (@A\n/* f */ Exception e) {}\n\
                  List<@A\n/* g */ String> names; } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void m(
        @A
        // a
        int x,
        final
        /* b */
        int y
      ) {
        for (
          @A
          /* c */ int i = 0;
          i < 1;
          i++
        ) {}
        for (@A
        /* d */ String s : values) {}
        try (
          @A
          /* e */ Resource r = open()
        ) {} catch (@A
            /* f */ Exception e) {}
        List<
          @A
          /* g */ String
        > names;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn annotation_array_does_not_claim_a_field_trailing_comment() {
    let source = "class C { @Values({1}) int first; // first trailing\nint second; }";
    assert_snapshot!(fmt(source), @"
    class C {
      @Values({1}) int first; // first trailing
      int second;
    }
    ");
}

#[test]
fn numeric_annotation_array_trailing_comma_comment_formats_in_one_pass() {
    use robusta_formatter::Options;

    let options = Options::default();
    let source = "@X({1, 2, /* c */\n}) class A {}";
    assert_snapshot!(crate::fixtures::format_source_with_options(source, &options), @"
    @X({1, 2 /* c */})
    class A {}
    ");
}

#[test]
fn local_variable_annotations_take_the_same_three_directions_as_fields() {
    let source = "class C { void m() { int first = 0; \
                  @Nullable int second = 0; \
                  @GuardedBy(\"this\") int third = 0; \
                  @Alpha /* between */ final int fourth = 0; @Alpha\n@Bravo int fifth = 0; \
                  @Alpha @Bravo @Charlie @Delta @Echo @Foxtrot @Golf @Hotel @India @Juliett \
                  ExtremelyLongDataLoaderTypeName sixthWithAnExtremelyLongName = null; } }";
    assert_snapshot!(fmt(source), @r#"
    class C {
      void m() {
        int first = 0;
        @Nullable int second = 0;
        @GuardedBy("this") int third = 0;
        @Alpha /* between */ final int fourth = 0;
        @Alpha
        @Bravo
        int fifth = 0;
        @Alpha
        @Bravo
        @Charlie
        @Delta
        @Echo
        @Foxtrot
        @Golf
        @Hotel
        @India
        @Juliett
        ExtremelyLongDataLoaderTypeName sixthWithAnExtremelyLongName = null;
      }
    }
    "#);
}

#[test]
fn comments_before_the_semicolon_leave_annotations_inline() {
    let source = "class C {\n\
                  @A @B Loader loader // c\n;\n\
                  @A @B int y /* c */ // d\n;\n\
                  @A @B int id /// e\n;\n\
                  void m() {\n\
                  @A final int x = 1 /* c\n * d */ ;\n\
                  @A int z = 2 // c\n;\n\
                  }\n}";
    assert_snapshot!(fmt(source), @"
    class C {
      @A @B Loader loader; // c
      @A @B int y; /* c */ // d
      @A @B int id /// e
      ;

      void m() {
        @A final int x = 1; /* c
                             * d */
        @A int z = 2; // c
      }
    }
    ");
}

#[test]
fn annotated_field_spacing_covers_every_field_member_container() {
    let source = r"class C { Object value = new Object() { int first; @A(1) int second; int third; }; }
                      interface I { int FIRST = 1; @A(1) int SECOND = 2; int THIRD = 3; }
                      enum E { VALUE; int first; @A(1) int second; int third; }
                      record R() { int first; @A(1) int second; int third; }
                      @interface Ann { int FIRST = 1; @A(1) int SECOND = 2; int THIRD = 3; }";
    assert_snapshot!(fmt(source), @"
    class C {
      Object value = new Object() {
        int first;
        @A(1) int second;
        int third;
      };
    }

    interface I {
      int FIRST = 1;
      @A(1) int SECOND = 2;
      int THIRD = 3;
    }

    enum E {
      VALUE;

      int first;
      @A(1) int second;
      int third;
    }

    record R() {
      int first;
      @A(1) int second;
      int third;
    }

    @interface Ann {
      int FIRST = 1;
      @A(1) int SECOND = 2;
      int THIRD = 3;
    }
    ");
}

#[test]
fn annotation_arguments_wrap_like_argument_lists() {
    let source = "class M { @Configuration(firstElementName = firstExtremelyLongElementValue, \
                  secondElementName = secondExtremelyLongElementValue) void f() {} }";
    assert_snapshot!(fmt(source), @"
    class M {
      @Configuration(
        firstElementName = firstExtremelyLongElementValue,
        secondElementName = secondExtremelyLongElementValue
      )
      void f() {}
    }
    ");
}

#[test]
fn a_lone_annotation_array_hugs_the_parentheses() {
    let names = "{\"unchecked\", \"rawtypes\", \"deprecation\", \"serial\", \"fallthrough\", \
                 \"cast\", \"static\"}";
    let source = format!(
        "class M {{ @SuppressWarnings({names}) void f() {{}} \
         @SuppressWarnings(/* why */ {names}) void g() {{}} \
         @SuppressWarnings(value = {names}) void h() {{}} }}"
    );
    assert_snapshot!(fmt(&source), @r#"
    class M {
      @SuppressWarnings({
        "unchecked",
        "rawtypes",
        "deprecation",
        "serial",
        "fallthrough",
        "cast",
        "static",
      })
      void f() {}

      @SuppressWarnings(
        /* why */ {"unchecked", "rawtypes", "deprecation", "serial", "fallthrough", "cast", "static"}
      )
      void g() {}

      @SuppressWarnings(
        value = {"unchecked", "rawtypes", "deprecation", "serial", "fallthrough", "cast", "static"}
      )
      void h() {}
    }
    "#);
}

#[test]
fn blank_lines_between_annotation_arguments_break_the_list() {
    let source = "class M { @Configuration(first = 1,\n\nsecond = 2, third = 3) void f() {} }";
    assert_snapshot!(fmt(source), @"
    class M {
      @Configuration(
        first = 1,

        second = 2,
        third = 3
      )
      void f() {}
    }
    ");
}

#[test]
fn throws_types_keep_type_use_annotations() {
    // A thrown `ClassType` may carry type-use annotations (JLS §8.4.6 / §9.7.4),
    // including on an interior segment of a qualified name.
    let source = "class C { void f() throws @Critical IOException, \
                  java.io.@Nullable IOException {} }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() throws @Critical IOException, java.io.@Nullable IOException {}
    }
    ");
}

#[test]
fn local_and_cast_types_keep_interior_type_use_annotations() {
    let source = "class C { void f(Object o) { java.util.Map . @Nullable Entry<K,V> e = \
                  (java.util.Map.@Nullable Entry<K,V>) o; } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f(Object o) {
        java.util.Map.@Nullable Entry<K, V> e = (java.util.Map.@Nullable Entry<K, V>) o;
      }
    }
    ");
}

#[test]
fn method_reference_types_keep_type_use_annotations() {
    let source = "class C { Function<String,Integer> length = @Y  String :: length; \
                  IntFunction<String[]> arrays = /* a */ @Y /* b */ String @Y /* c */ [ ]::new; \
                  IntFunction<int[]> ints = int @Y []::new; }";
    assert_snapshot!(fmt(source), @"
    class C {
      Function<String, Integer> length = @Y String::length;
      IntFunction<String[]> arrays = /* a */ @Y /* b */ String @Y /* c */ []::new;
      IntFunction<int[]> ints = int @Y []::new;
    }
    ");
}

#[test]
fn array_creation_dimensions_keep_type_use_annotations() {
    let source = "class C { void f() { i = new @Y int @Y [ 4 ]; j = new int @Y [ ] { 10 }; \
                  t = new String /* a */ @Y /* b */ @Z /* c */ [5] @Y [ ]; } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        i = new @Y int @Y [4];
        j = new int @Y [] {10};
        t = new String /* a */ @Y /* b */ @Z /* c */ [5] @Y [];
      }
    }
    ");
}

#[test]
fn shared_type_annotation_and_list_boundaries_keep_comments_local() {
    let source = r"class A<T /* type parameter */> {
        @Deprecated /* declaration annotation */ void first(
            @Marker /* parameter annotation */ String... /* varargs */ values) {}
        Map<String /* type argument */, Integer> entries;
        @Empty(/* empty arguments */) int marker;
        int /* array dimension */ [] dimensions;
    }";
    assert_snapshot!(fmt(source), @"
    class A<T /* type parameter */> {
      @Deprecated /* declaration annotation */
      void first(@Marker /* parameter annotation */ String... /* varargs */ values) {}

      Map<String /* type argument */, Integer> entries;
      @Empty(/* empty arguments */) int marker;
      int /* array dimension */[] dimensions;
    }
    ");
}

/// A comment that ends its line before a class type's `<` stays there, and
/// the type arguments start the next line, indented as a wrapped line.
#[test]
fn a_comment_ending_the_line_before_type_arguments_stays_before_the_opener() {
    let source = "class A {\n  Map // e\n  <K, V> z;\n  List /// d\n  <T> x;\n  \
                  java.util.List // c\n  <@NonNull /* annotated argument */ String> values;\n  \
                  boolean f(Object o) {\n    return o instanceof java.util.Map // m\n    \
                  <String, Integer> m;\n  }\n}\n";
    assert_snapshot!(fmt(source), @"
    class A {
      Map // e
        <K, V> z;
      List /// d
        <T> x;
      java.util.List // c
        <@NonNull /* annotated argument */ String> values;

      boolean f(Object o) {
        return o
          instanceof
          java.util.Map // m
            <String, Integer> m;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

/// Type arguments moved after a comment are indented twice in a return type or
/// a declaration clause, and the code after them continues their line.
#[test]
fn type_arguments_moved_after_a_comment_start_a_wrapped_line_for_the_code_after_them() {
    let source = "class Ledger<E> extends Base // base
<E>.Entry implements Comparable // order
<Ledger<E>>, Serializable {
  java.util.List // c
  <String> values = computeTheValues(alphaArgument, betaArgument, gammaArgument, deltaArgument, e);
  Object entry = new Outer // o
  <String>.Inner<Integer>(alphaArgument, betaArgument, gammaArgument, deltaArgument, epsilonArgument);
  <T> List // t
  <T> entries(int alphaArgument, int betaArgument, int gammaArgument, int deltaArgument, int epsilon) throws Failure // f
  <T>.Kind { return null; }
}
";
    assert_snapshot!(fmt(source), @"
    class Ledger<E>
        extends Base // base
            <E>.Entry
        implements Comparable // order
                <Ledger<E>>,
            Serializable {
      java.util.List // c
        <String> values = computeTheValues(
          alphaArgument,
          betaArgument,
          gammaArgument,
          deltaArgument,
          e
        );
      Object entry = new Outer // o
        <String>.Inner<Integer>(
          alphaArgument,
          betaArgument,
          gammaArgument,
          deltaArgument,
          epsilonArgument
        );

      <T> List // t
          <T> entries(
            int alphaArgument,
            int betaArgument,
            int gammaArgument,
            int deltaArgument,
            int epsilon
          ) throws Failure // f
              <T>.Kind {
        return null;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

/// An array creation's initializer and a record pattern's components continue
/// the line of type arguments moved after a comment.
#[test]
fn an_initializer_or_record_components_continue_type_arguments_moved_after_a_comment() {
    let source = "class A {
  Object kinds = new Class // c
  <?>[] {String.class, Integer.class, Long.class, Double.class, Float.class, Short.class, Byte.class};
  boolean f(Object o) {
    return o instanceof Box // b
    <String>(var alphaComponent, var betaComponent, var gammaComponent, var deltaComponent, var e);
  }
}
";
    assert_snapshot!(fmt(source), @"
    class A {
      Object kinds = new Class // c
        <?>[] {
          String.class,
          Integer.class,
          Long.class,
          Double.class,
          Float.class,
          Short.class,
          Byte.class,
        };

      boolean f(Object o) {
        return o
          instanceof
          Box // b
            <String>(
              var alphaComponent,
              var betaComponent,
              var gammaComponent,
              var deltaComponent,
              var e
            );
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn block_comments_ending_the_line_before_type_arguments_trail_the_name() {
    let source = "class A {\n  List /* c */\n  <T> x;\n  \
                  Object o = new Outer /** d */\n  <T>.Inner<U>();\n  \
                  List /* a\n  b */\n  <T> m(Map /* e */ // f\n  <K, V> p) {\n    \
                  return (List /** g */\n  <T>) p;\n  }\n}\n";
    assert_snapshot!(fmt(source), @"
    class A {
      List /* c */<T> x;
      Object o = new Outer /** d */<T>.Inner<U>();

      List /* a
      b */<T> m(
        Map /* e */ // f
          <K, V> p
      ) {
        return (List /** g */<T>) p;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn comments_before_annotation_and_type_argument_lists_stay_before_the_delimiter() {
    let source = "class A { @B /* a */ (1) @C /* b */ (k = 2) @D /* c */ () List /* d */ <T> x; \
                  Map.Entry /* e */ <K, List /* f */ <V>> y; }";
    assert_snapshot!(fmt(source), @"
    class A {
      @B /* a */(1) @C /* b */(k = 2) @D /* c */() List /* d */<T> x;
      Map.Entry /* e */<K, List /* f */<V>> y;
    }
    ");
}

#[test]
fn line_comments_before_annotation_and_type_argument_lists_keep_their_lines() {
    let source = "class A { @B // a\n(1) int x; List\n/* b */\n<T> y; }";
    assert_snapshot!(fmt(source), @"
    class A {
      @B // a
        (1)
      int x;
      List
        /* b */
        <T> y;
    }
    ");
}

#[test]
fn wildcard_and_annotation_boundary_comments_keep_their_tokens() {
    let source = "class A { List<? /* extends */ extends T> x; List<@B ? /* super */ super T> y; @ /* name */ B(1) int z; }";
    assert_snapshot!(fmt(source), @"
    class A {
      List<? /* extends */ extends T> x;
      List<@B ? /* super */ super T> y;
      @/* name */ B(1) int z;
    }
    ");
}

#[test]
fn comments_inside_and_before_dimension_brackets_stay_there() {
    let source = "class A { int[/* type */] a; int b[/* name */]; int @A /* c */\n[] c; \
                  List<@B /* d */\n?> d; }";
    assert_snapshot!(fmt(source), @"
    class A {
      int[/* type */] a;
      int b[/* name */];
      int @A /* c */ [] c;
      List<@B /* d */ ?> d;
    }
    ");
}

#[test]
fn each_dimension_of_an_array_type_keeps_its_annotations_and_the_comments_before_it() {
    let source = "class A { String @A [] @B [] a; int @A [] [] @B [] b; \
                  int /* c */ [] /* d */ [] c; int @A [] /* e */ @B [] e; \
                  int[]\n// f\n[] f = {}, ff = {}; List<int @A [] @B []> g; \
                  void h(String @A [] /* i */ [] i, String @A [] @B ... j) {} }";
    assert_snapshot!(fmt(source), @"
    class A {
      String @A [] @B [] a;
      int @A [][] @B [] b;
      int /* c */[] /* d */[] c;
      int @A [] /* e */ @B [] e;
      int[]
        // f
        [] f = {},
          ff = {};
      List<int @A [] @B []> g;

      void h(String @A [] /* i */[] i, String @A [] @B ... j) {}
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn comments_before_a_first_dimension_annotation_lead_it() {
    let source = "class A { int\n\n// b\n\n@C [] d; int /* e */\n@C [] f; \
                  int g\n/* h */ @C [] = null; int i\n\n// j\n\n@C []; \
                  Object k = new int\n\n// l\n\n@C [5]; int m()\n// n\n@C [] { return null; } \
                  void o(int p\n/* q */ @C []) {} }";
    assert_snapshot!(fmt(source), @"
    class A {
      int
      // b

      @C [] d;
      int /* e */ @C [] f;
      int g
      /* h */ @C [] = null;
      int i
      // j

      @C [];
      Object k = new int
      // l

      @C [5];

      int m()
      // n
      @C [] {
        return null;
      }

      void o(
        int p
        /* q */ @C []
      ) {}
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn comments_between_type_annotations_lead_or_trail_them() {
    let source = "class A { List<@B /* a */ @C /* b */ T> c; \
                  List<@B // d\n@C // e\nT> f; \
                  List<@B\n// g\n@C\n// h\nT> i; \
                  List<@B /* j */\nT> k; \
                  List<@B\n/* l */ T> m; \
                  Object n = new int @B\n// o\n@C [1]; \
                  List<@B(1) // p\nT> q; }";
    assert_snapshot!(fmt(source), @"
    class A {
      List<@B /* a */ @C /* b */ T> c;
      List<
        @B // d
        @C // e
        T
      > f;
      List<
        @B
        // g
        @C
        // h
        T
      > i;
      List<@B /* j */ T> k;
      List<
        @B
        /* l */ T
      > m;
      Object n = new int @B
      // o
      @C [1];
      List<
        @B(1) // p
        T
      > q;
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn comments_starting_a_line_after_a_type_annotation_keep_it_before_the_token() {
    for (gap, kept) in [
        ("\n/* c */\n", "@D\n{i}/* c */\n{i}"),
        ("\n/* c */ ", "@D\n{i}/* c */ "),
        (" // a\n/* b */\n", "@D // a\n{i}/* b */\n{i}"),
        ("\n/* a\n * b */\n", "@D\n{i}/* a\n{i} * b */\n{i}"),
        ("\n\n// c\n", "@D\n{i}// c\n{i}"),
    ] {
        let kept = |indent: &str| kept.cow_replace("{i}", indent).into_owned();
        let (two, four) = (kept("  "), kept("    "));
        for (source, expected) in [
            (
                format!("class A {{ int @D{gap}[] x; }}"),
                format!("class A {{\n  int {two}[] x;\n}}\n"),
            ),
            (
                format!("class A {{ Object x = new int @D{gap}[1]; }}"),
                format!("class A {{\n  Object x = new int {two}[1];\n}}\n"),
            ),
            (
                format!("class A {{ List<@D{gap}?> x; }}"),
                format!("class A {{\n  List<\n    {four}?\n  > x;\n}}\n"),
            ),
            (
                format!("class A {{ void f(int @D{gap}... x) {{}} }}"),
                format!("class A {{\n  void f(\n    int {four}... x\n  ) {{}}\n}}\n"),
            ),
        ] {
            assert_eq!(fmt(&source), expected, "{source}");
            check_contract_at_widths_and_modes(&source);
        }
    }
}

#[test]
fn child_comments_stay_inside_type_and_pattern_delimiters() {
    let sources = [
        "class A { int[/* type */] a; int b[/* detached */]; }",
        "class A { void m(A this) /* head */ {} }",
        "class A { F f = (int x) /* arrow */ -> x; }",
        "class A { void m(int a, int b, int c) {} }",
        "record A(int x /* one */ /* two */) {}",
        r"record A(int x\u0020/* one */\u0020/* two */\uuu0029 {}",
        "class A<T> /* head */ {}",
        "class A { @interface B { int value()[] // before\ndefault // after\n{1}; } }",
        "class A { void m(Object o) { if (o instanceof Empty(/* empty */)) {} if (o instanceof Pair(var x, _ /* component */)) {} } }",
    ];
    for source in sources {
        check_contract_at_widths_and_modes(source);
    }
}

#[test]
fn paired_comments_at_type_and_pattern_boundaries_preserve_the_contract() {
    let templates = [
        "class C<T #extends# Number #&# Runnable> {}",
        "class C { <T #extends# Number #&# Runnable> T m(T a) { return a; } }",
        "class C { java.#util.#List<#?#extends# Number> v; }",
        "class C { Object x = (#A #&# B#) value; }",
        "class C { int#[]#a#[]; }",
        "class C { Object x = new# int#[#3#]#[]; }",
        "class C { Object x = new# A<#>#(); }",
        "class C { Object x = new#<A>#Thing(); }",
        "class C { Object x = owner.#<A>#call(); }",
        "class C { Object x = A#::#<B>#new; }",
        "class C { boolean x = value instanceof#A#other; }",
        "class C { boolean x = value instanceof#R#(#var#x,#_#); }",
        "class C { void m(Object x) { switch (x) { case R(#var#a,#_#) -> {} default -> {} } } }",
        "class C { void m(int#x,#String#y) {} }",
        "class C { void m(C#this,#String#x) {} }",
        "record C(int#x,#String#y) {}",
        "class C { Object x = (#var#a,#var#b) -> a; }",
        "class C { String a @Dims#[] = null; int b#[]#[]#= 1; }",
        "class C { java.util.#@Nullable#List<String> v; }",
        "class C { void m() { try {} catch (A#ex) {} } }",
        "class C { Object x = new java.util.HashMap#<>(); }",
        "class C { <U> U#first(U value) { return value; } }",
        "class C { void m(final String args#[]#, int#x) {} }",
    ];
    for template in templates {
        for gap in [" /* a */ // b\n", "/** a *//* b */", "\n/* a */\n// b\n"] {
            check_contract_at_widths_and_modes(&template.cow_replace('#', gap));
        }
    }
}

#[test]
fn annotation_children_and_varargs_comments_keep_their_tokens() {
    let templates = [
        "@#A#(#{#@B#(name#=#1#),#@C#()#}#) class D {}",
        "@A#(a#=#@B#(1#),#b#=#{#1,#2#}#) class C {}",
        "@A#()#@B#(1#) public#final#class C {}",
        "@A#(1#) package p; class C {}",
        "@A#(1#) open module m {}",
        "class C { @A#(1#) public#static#final#int x = 1; }",
        "class C { void m(@A#(1#) final#String#@B#...#args) {} }",
        "record C(@A#(1#) String#@B#...#args) {}",
        "class C { Object f = (@A#(1#) String#@B#...#args) -> args; }",
        "@interface C { A value() default#@A#(x#=#1#); }",
    ];
    for template in templates {
        for gap in [" /* a */ // b\n", "\n/* a */\n// b\n", " /* a */ /* b */ "] {
            check_contract_at_widths_and_modes(&template.cow_replace('#', gap));
        }
    }
}

#[test]
fn block_comment_after_an_annotation_pair_operator_starts_the_next_line() {
    assert_snapshot!(fmt("@A(name // b\n= /* a */ // b\n1) class D {}"), @"
    @A(
      name // b
        = /* a */ // b
        1
    )
    class D {}
    ");
}
