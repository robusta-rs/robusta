//! Type, member, constructor, and initializer declarations.

use insta::assert_snapshot;
use robusta_formatter::{JavadocOptions, LineEnding, Options};

use crate::fixtures::{
    assert_examples_print_as_shown, check_contract_at_widths_and_modes, check_formatter_contract,
    format_source as fmt, format_source_with_options as fmt_with_options,
    format_stable_with_options, indent_width, line_width,
};

fixture_tests! {
    golden_methods_file => "declarations/methods";
    golden_class_inheritance_file => "declarations/class-inheritance";
    golden_anonymous_class_bodies_file => "declarations/anonymous-class-bodies";
    golden_initializer_blocks_file => "declarations/initializer-blocks";
    golden_sealed_type_headers_file => "declarations/sealed-type-headers";
    golden_records_file => "declarations/records";
    golden_core_modifiers_file => "declarations/core-modifiers";
    golden_flexible_constructor_bodies_file => "declarations/flexible-constructor-bodies";
    golden_varargs_file => "declarations/varargs";
    golden_receiver_parameters_file => "declarations/receiver-parameters";
    golden_declarator_dimensions_file => "declarations/declarator-dimensions";
    golden_interface_file => "declarations/interface";
    golden_enum_file => "declarations/enum";
    golden_enum_constant_bodies_file => "declarations/enum-constant-bodies";
    golden_nested_types_file => "declarations/nested-types";
    golden_annotation_interface_file => "declarations/annotation-interface";
    golden_local_types_file => "declarations/local-types";
    golden_contextual_keyword_identifiers_file => "declarations/contextual-keyword-identifiers";
}

#[test]
fn empty_initializer_blocks_collapse() {
    let source = "class C { static {} {} }";
    assert_snapshot!(fmt(source), @"
    class C {
      static {}

      {}
    }
    ");
}

/// An escaped contextual keyword is the keyword (JLS §3.2, §3.9), and the
/// formatter prints a keyword in its canonical spelling — the policy it already
/// applies to `cl\u0061ss`. The token stream still has to verify across that
/// rewrite.
#[test]
fn escaped_contextual_keyword_survives_the_formatter_contract() {
    let source = "rec\\u006frd Point(int x) {}\n";
    check_formatter_contract(source).expect("escaped contextual keyword must verify");
}

#[test]
fn comments_after_type_keywords_stay_in_place() {
    let source = "public class /* a */ A<T> {} @B interface /* b */ I {} enum /* c */ E { X } \
                  record /* d */ R() {} @interface /* e */ N {} class // f\nC extends D {}";
    assert_snapshot!(fmt(source), @"
    public class /* a */ A<T> {}

    @B
    interface /* b */ I {}

    enum /* c */ E {
      X,
    }

    record /* d */ R() {}

    @interface /* e */ N {}

    class // f
    C extends D {}
    ");
}

#[test]
fn comment_runs_after_type_clauses_keep_the_brace_on_the_header_line() {
    for head in [
        "class A extends B",
        "class A implements B, C",
        "sealed class A permits B, C",
        "interface A extends B, C",
        "sealed interface A permits B, C",
        "enum A implements B, C",
        "record A(int x) implements B, C",
    ] {
        for (run, formatted) in
            [("/* a */ // c", "/* a */ { // c\n}"), ("/* a */ /* c */", "/* a */ /* c */ {}")]
        {
            let source = format!("{head} {run}\n{{}}");
            assert_eq!(fmt(&source), format!("{head} {formatted}\n"));
            check_contract_at_widths_and_modes(&source);
        }
    }
}

#[test]
fn type_clause_comment_runs_keep_multiline_and_own_line_boundaries() {
    for head in [
        "class A extends B",
        "class A<T> extends B<T> implements C, D",
        "sealed class A extends B implements C permits D, E",
        "interface A extends B, C",
        "sealed interface A extends B permits C, D",
        "enum A implements B, C",
        "record A(int x) implements B, C",
    ] {
        for run in [
            " /* a */ /* b */ // c\n",
            " /* a\n b */ // c\n",
            " /* a\n b */ /* c */\n",
            " /* a */ // b\n// c\n",
            " /* a */\n/* b */ // c\n",
            " /** */ /* a */ // c\n",
            " /* a */\n/** b */\n",
        ] {
            let source = format!("class Outer {{ {head}{run}{{}} }}");
            check_contract_at_widths_and_modes(&source);
        }
    }
}

#[test]
fn comments_before_implements_and_throws_drop_the_blank_line_before_them() {
    let source = "class Order extends Base

    // Marks it as priced.
    implements Priced {

  void pay()

      // Thrown when the card fails.
      throws Failure {}
}";
    assert_snapshot!(fmt(source), @"
    class Order extends Base
        // Marks it as priced.
        implements Priced {

      void pay()
          // Thrown when the card fails.
          throws Failure {}
    }
    ");
}

#[test]
fn type_bodies_keep_a_blank_line_after_their_brace() {
    let source = "class A { // c


  int x;
  void f() {

    Runnable r = () -> {

      g();
    };
    if (r != null) {

      g();
    }
  }
}
interface I {

  // leads f
  void f();
}
enum E {

  A {

    void f() {}
  }
}
record R() {

  static int x;
}
@interface N {

  int v();
}
class B {
  Object o = new Object() {

    int y;
  };
  class Empty {

  }
}";
    assert_snapshot!(fmt(source), @"
    class A { // c

      int x;

      void f() {
        Runnable r = () -> {
          g();
        };
        if (r != null) {
          g();
        }
      }
    }

    interface I {

      // leads f
      void f();
    }

    enum E {

      A {

        void f() {}
      },
    }

    record R() {

      static int x;
    }

    @interface N {

      int v();
    }

    class B {
      Object o = new Object() {

        int y;
      };

      class Empty {}
    }
    ");
}

#[test]
fn comments_before_extends_and_permits_drop_the_blank_line_before_them() {
    let source = "sealed class Order

// Shares the base.
extends Base

/* Lists the kinds. */
permits Online, Store {}
interface Priced

// Adds the currency.
extends Valued {}";
    assert_snapshot!(fmt(source), @"
    sealed class Order
        // Shares the base.
        extends Base
        /* Lists the kinds. */
        permits Online, Store {}

    interface Priced
        // Adds the currency.
        extends Valued {}
    ");
}

#[test]
fn comments_after_a_clause_keyword_lead_its_first_entry() {
    for (head, tail) in [
        ("class D extends", " E {}"),
        ("interface I extends", " J {}"),
        ("class A implements", " B {}"),
        ("sealed class A permits", " B {}"),
        ("class A {\n  void f() throws", " E {}\n}"),
    ] {
        let source = format!("{head} /* c */{tail}");
        assert_eq!(fmt(&source), format!("{source}\n"));
        for comment in [" /* c */", " // c\n", "\n// c\n"] {
            check_contract_at_widths_and_modes(&format!("{head}{comment}{tail}"));
        }
    }
}

#[test]
fn type_parameter_breaks_before_extends_when_the_name_and_keyword_do_not_fit() {
    let source = "class C<FirstTypeParameterIsQuiteLong extends Bound> extends Base {} \
                  class D { <FirstTypeParameterIsQuiteLong extends Bound> void f() {} }";
    let options = Options { line_width: line_width(40), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C<
            FirstTypeParameterIsQuiteLong
                extends Bound>
        extends Base {}

    class D {
      <
        FirstTypeParameterIsQuiteLong
          extends Bound
      > void f() {}
    }
    ");
}

#[test]
fn a_single_clause_entry_can_wrap_after_the_keyword() {
    let source = "class C { \
                  void f(Customer customer, List<Item> items, Discount discount) \
                  throws SomeExceptionWithALongNameX {} \
                  void g() throws SomeExceptionWithALongNameX {} } \
                  record R(Customer customer, List<Item> items, Discount discount) \
                  implements SomeInterfaceWithALongName {}";
    let options = Options { line_width: line_width(40), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f(
        Customer customer,
        List<Item> items,
        Discount discount
      ) throws
          SomeExceptionWithALongNameX {}

      void g()
          throws
              SomeExceptionWithALongNameX {}
    }

    record R(
      Customer customer,
      List<Item> items,
      Discount discount
    ) implements
        SomeInterfaceWithALongName {}
    ");
}

#[test]
fn declaration_clauses_keep_the_keyword_with_the_first_entry() {
    let options =
        Options { line_width: line_width(50), indent_width: indent_width(2), ..Options::default() };
    let examples = [
        "Invoice readInvoice(Path path, Charset charset)
    throws IOException {
  return parse(path, charset);
}
",
        "void transfer(
  Account source,
  Account target,
  Money amount
) throws InsufficientFundsException,
    AccountLockedException {
  ledger.move(source, target, amount);
}
",
        "record OrderLine(
  Item item,
  int quantity,
  Region region
) implements Comparable<OrderLine>,
    Serializable {}
",
    ];
    assert_examples_print_as_shown(&examples, &options);
}

#[test]
fn method_headers_break_after_type_parameters_before_the_return_type() {
    let options =
        Options { line_width: line_width(50), indent_width: indent_width(2), ..Options::default() };
    let examples = [
        "public Map<Customer, List<Invoice>>
    invoicesForRegion(Region region) {
  return groupByCustomer(invoices, region);
}
",
        "static <R extends Region, I>
    Map<R, List<I>> invoicesGroupedByRegion(
      List<I> invoices,
      Function<I, R> regionOf
    ) {
  return groupBy(invoices, regionOf);
}
",
        "static <R extends Region, I extends Invoice>
    Map<R, List<Map.Entry<I, Customer>>>
        invoiceEntriesByRegion(List<I> all)
            throws IOException {
  return groupEntries(all);
}
",
    ];
    assert_examples_print_as_shown(&examples, &options);
}

/// A comment after the type parameters neither breaks them nor the return type.
#[test]
fn comments_after_method_type_parameters_keep_the_break_after_them() {
    let source = "class C { <T> // c\nLongTypeName<T> longMethodName(T value) { return null; } \
                  static <R extends Region, I> /* c */ Map<R, List<I>> \
                  invoicesGroupedByRegion(List<I> invoices) { return null; } }";
    let options =
        Options { line_width: line_width(36), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class C {
      <T> // c
          LongTypeName<T>
              longMethodName(T value) {
        return null;
      }

      static <R extends Region, I>
          /* c */ Map<R, List<I>>
              invoicesGroupedByRegion(
                List<I> invoices
              ) {
        return null;
      }
    }
    ");
}

#[test]
fn clause_after_a_split_list_stays_on_the_closing_paren_line() {
    let source = "class C { Invoice createInvoice(Customer customer, List<Item> items, \
                  Discount discount) throws IOException {} \
                  OrderService(Customer customer, List<Item> items) \
                  throws IOException, TimeoutException {} \
                  C(Customer customer, List<Item> items, Discount discount) throws IOException {} \
                  void read(Customer customer, List<Item> items, Discount discount) \
                  throws IOException, TimeoutException, InterruptedException {} }\n\
                  record Invoice(Customer customer, List<Item> items) \
                  implements Document, Serializable {}\n\
                  record Line(Item item, int quantity, Discount discount) implements Document {}";
    let options = Options { line_width: line_width(50), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      Invoice createInvoice(
        Customer customer,
        List<Item> items,
        Discount discount
      ) throws IOException {}

      OrderService(
        Customer customer,
        List<Item> items
      ) throws IOException, TimeoutException {}

      C(
        Customer customer,
        List<Item> items,
        Discount discount
      ) throws IOException {}

      void read(
        Customer customer,
        List<Item> items,
        Discount discount
      ) throws IOException,
          TimeoutException,
          InterruptedException {}
    }

    record Invoice(
      Customer customer,
      List<Item> items
    ) implements Document, Serializable {}

    record Line(
      Item item,
      int quantity,
      Discount discount
    ) implements Document {}
    ");
    let source = "record R(Customer customer, List<Item> items, Discount discount) \
                  implements Document, Serializable, Comparable<R> {}";
    let options = Options { indent_width: indent_width(4), ..options };
    assert_snapshot!(fmt_with_options(source, &options), @"
    record R(
        Customer customer,
        List<Item> items,
        Discount discount
    ) implements Document,
            Serializable,
            Comparable<R> {}
    ");
}

#[test]
fn comments_before_a_declared_list_stay_before_its_delimiter() {
    let source = "record R<T> /* d */ (int x) {} record S /* e */ () {} \
                  class Y { Y /* f */ (int t) {} <T> void /* g */ m /* h */ (T t) {} \
                  void n /* i */ (/* j */) {} } @interface A { int value /* k */ (); }";
    assert_snapshot!(fmt(source), @"
    record R<T> /* d */(int x) {}

    record S /* e */() {}

    class Y {
      Y /* f */(int t) {}

      <T> void /* g */ m /* h */(T t) {}

      void n /* i */(/* j */) {}
    }

    @interface A {
      int value /* k */();
    }
    ");
}

#[test]
fn line_comments_before_a_declared_list_keep_their_lines() {
    let source = "record R\n// b\n(int x) {}\nclass Z { Z\n/* d */\n() {} }";
    assert_snapshot!(fmt(source), @"
    record R
    // b
    (int x) {}

    class Z {
      Z
      /* d */
      () {}
    }
    ");
}

/// A method's name takes its trailing comments, so a line comment rides to
/// the end of the line. A `///` comment moves after the `(`, since behind the
/// body it would document the next member.
#[test]
fn comments_after_a_method_name_trail_it() {
    let source = "class Z { void m // c\n(int a) {} abstract void n /* d */ // e\n(); \
                  void o /// f\n(int a) {} Z() {} }";
    assert_snapshot!(fmt(source), @"
    class Z {
      void m(int a) {} // c

      abstract void n /* d */(); // e

      void o( /// f
        int a
      ) {}

      Z() {}
    }
    ");
    check_contract_at_widths_and_modes(source);
}

/// The line comment after the name, the type parameters, or a return type's
/// name rides to the body's `{` and stays on its line.
#[test]
fn line_comment_in_a_method_header_follows_the_body_brace() {
    let source = "class Z { void m // c\n(int a) { f(); } <U> // r\nU last(U v) { return v; } \
                  List // t\n<T> g() { return null; } }";
    assert_snapshot!(fmt(source), @"
    class Z {
      void m(int a) { // c
        f();
      }

      <U> U last(U v) { // r
        return v;
      }

      List<T> g() { // t
        return null;
      }
    }
    ");
}

#[test]
fn comments_before_type_parameters_trail_the_name_or_lead_the_first_parameter() {
    let source = "class X // a\n<T> {}\nclass Y\n// b\n<T> {}\nclass Z /* c */ <T> {}\n\
                  interface I /* d */ /* e */ <T> extends J {}\nrecord R /* f */ <T>(int x) {}";
    assert_snapshot!(fmt(source), @"
    class X<T> {} // a

    class Y<
        // b
        T> {}

    class Z</* c */ T> {}

    interface I</* d */ /* e */ T> extends J {}

    record R</* f */ T>(int x) {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn block_and_line_comments_stay_together_before_type_parameters() {
    let source = "class R /* a */ // b\n<T> {}";
    assert_snapshot!(fmt(source), @"
    class R /* a */ // b
    <T> {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn line_comments_after_type_bodies_leave_fitting_clauses_on_the_line() {
    let source = "class X<T> extends Y {} // a\nclass Z extends Y {} // b\n\
                  class W<T> implements Y {} // c\ninterface I extends J {} /* d */";
    assert_snapshot!(fmt(source), @"
    class X<T> extends Y {} // a

    class Z extends Y {} // b

    class W<T> implements Y {} // c

    interface I extends J {} /* d */
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn line_comments_in_type_headers_break_before_the_clauses() {
    let source = "class X // a\n<T> extends Y implements A, B {}\n\
                  class K<T> // b\nextends Y implements A, B { int x; }\n\
                  class D /// c\nextends E { int x; }\n\
                  class G extends // d\nE {}\n\
                  interface I // e\n<T> extends J { int N = 0; }\n\
                  enum Kind // f\nimplements J { A }";
    assert_snapshot!(fmt(source), @"
    class X<T> // a
        extends Y
        implements A, B {}

    class K<T> // b
        extends Y
        implements A, B {
      int x;
    }

    class D /// c
        extends E {
      int x;
    }

    class G // d
        extends E {}

    interface I<T> // e
        extends J {
      int N = 0;
    }

    enum Kind // f
        implements J {
      A,
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn line_comment_stays_before_breaking_type_parameters() {
    let source = "class K // c\n<T extends A & B, U> extends C {}";
    assert_snapshot!(fmt(source), @"
    class K // c
    <T extends A & B, U>
        extends C {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn a_line_comment_before_type_parameters_follows_the_body_brace() {
    let source = "class K // e\n<T> {\n  int x;\n}\nclass L // f\n<T> { /* g */ }\n\
                  interface ReadOnlyCachedRepository // h\n<T> { /* i */ }\n\
                  class M /// documents nothing\n<T> { { init(); } }\n\
                  class N /// would document x\n<T> { int x; }";
    assert_snapshot!(fmt(source), @"
    class K<T> { // e
      int x;
    }

    class L<T> { // f
      /* g */
    }

    interface ReadOnlyCachedRepository<T> { // h
      /* i */
    }

    class M<T> { /// documents nothing
      {
        init();
      }
    }

    class N<T> /// would document x
    {
      int x;
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn comments_before_type_parameters_and_inside_a_clause_keep_their_order() {
    let source = "class C // head\n<T> extends /* first\n * second */ // inheritance\n\
                  S implements I { int count; }";
    assert_snapshot!(fmt(source), @"
    class C // head
    <T> /* first
         * second */ // inheritance
        extends S
        implements I {
      int count;
    }
    ");
    check_contract_at_widths_and_modes(source);
}

/// A `///` run that ends a constructor or method name's line moves after the
/// `(` as a whole. One after a constructor's type parameters or before a type's
/// `<` stays, with the next line indented twice, where a `//` comment trails
/// the header.
#[test]
fn documentation_comments_after_a_declaration_name_keep_their_owner() {
    let source = r"abstract class Order {
  Order /// q
  () {}

  <V> /// q
  Order(V first) {}

  abstract <T> T p /// q
  /// r
  (T first, T second);

  interface Box /// c
  /* d */ <T> extends Base {}

  class Line // c
  <T> {}
}";
    assert_snapshot!(fmt(source), @"
    abstract class Order {
      Order( /// q
      ) {}

      <V> /// q
          Order(V first) {}

      abstract <T> T p( /// q
        /// r
        T first,
        T second
      );

      interface Box /// c
          </* d */ T>
          extends Base {}

      class Line<T> {} // c
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn comment_before_record_type_parameters_respects_the_component_break() {
    let source = "record Sample // note\n<T>(T firstLongName, T anotherLongName, T thirdLongName) \
                  implements I { int value() { return 0; } }";
    let options = Options { line_width: line_width(40), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    record Sample // note
    <T>(
      T firstLongName,
      T anotherLongName,
      T thirdLongName
    ) implements I {
      int value() {
        return 0;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn documentation_comments_before_type_parameters_keep_their_owner() {
    let source = "class K /// e\n<T> {\n  int x;\n}\nrecord R /// f\n<T>(int x) {}\n\
                  interface I\n/// g\n<T> {}\nclass L {}";
    assert_snapshot!(fmt(source), @"
    class K<T> /// e
    {
      int x;
    }

    record R /// f
        <T>(int x) {}

    interface I
    /// g
    <T> {}

    class L {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn documentation_comments_glued_to_a_name_keep_their_owner() {
    let source = "class K///e\n<T> {\n  int x;\n}\ninterface I///g\n<T> {}";
    assert_snapshot!(fmt(source), @"
    class K<T> ///e
    {
      int x;
    }

    interface I ///g
        <T> {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn a_documentation_comment_in_an_empty_type_header_does_not_document_the_next_member() {
    let source = "class Outer { class K /// e\n<T> {} int y; }";
    assert_snapshot!(fmt(source), @"
    class Outer {
      class K /// e
          <T> {}

      int y;
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn documentation_in_wrapped_type_headers_preserves_the_comment_run() {
    for source in [
        "class K /// doc\n<T extends A & B, U> extends C { int x; }",
        "class K /** doc */ // note\n<T> extends C implements I, J {}",
        "class K /// doc  \n/// another\n<T> extends C { int x; }",
    ] {
        for width in [16, 24, 40] {
            for javadoc in [None, Some(JavadocOptions::default())] {
                let options =
                    Options { line_width: line_width(width), javadoc, ..Options::default() };
                assert!(format_stable_with_options(source, &options).contains("doc"));
            }
        }
    }
}

#[test]
fn comments_before_enum_constant_arguments_stay_before_the_paren() {
    let source = "enum E { A /* a */ (1), B /* b */ (), C // c\n(3) { }; E() {} E(int x) {} }";
    assert_snapshot!(fmt(source), @"
    enum E {
      A /* a */(1),
      B /* b */(),
      C // c
      (3) {};

      E() {}

      E(int x) {}
    }
    ");
}

#[test]
fn empty_enum_arguments_keep_comments_inside() {
    assert_snapshot!(fmt("enum E { UNIT(/* c */), B }"), @"
    enum E {
      UNIT(/* c */),
      B,
    }
    ");
}

#[test]
fn empty_enum_arguments_keep_comments_after_the_closer_outside() {
    for source in [
        "enum E { UNIT /* before */ (/* inside */) /* after */, B }",
        r"enum E { UNIT /* before */ \u0028/* inside */\uu0029 /* after */, B }",
    ] {
        insta::allow_duplicates! { assert_snapshot!(fmt(source), @"
        enum E {
          UNIT /* before */(/* inside */) /* after */,
          B,
        }
        "); }
    }
}

#[test]
fn empty_enum_arguments_keep_comments_before_a_body_inside() {
    for source in
        ["enum E { UNIT(/* inside */) {}, B }", r"enum E { UNIT\u0028/* inside */\u0029 {}, B }"]
    {
        insta::allow_duplicates! { assert_snapshot!(fmt(source), @"
        enum E {
          UNIT(/* inside */) {},
          B,
        }
        "); }
    }
}

#[test]
fn enum_constants_take_a_trailing_comma_only_before_the_closing_brace() {
    let source = "enum Region { NORTH, SOUTH }
enum Status { PAID, DUE; Status() {} }
enum Mode { ON, OFF,; Mode() {} }";
    assert_snapshot!(fmt(source), @"
    enum Region {
      NORTH,
      SOUTH,
    }

    enum Status {
      PAID,
      DUE;

      Status() {}
    }

    enum Mode {
      ON,
      OFF;

      Mode() {}
    }
    ");
}

#[test]
fn enum_body_comma_without_constants_is_omitted() {
    let source = "enum Empty { , } enum Marker { ,; } enum Utility { ,; int x; }";
    assert_snapshot!(fmt(source), @"
    enum Empty {}

    enum Marker {
      ;
    }

    enum Utility {
      ;

      int x;
    }
    ");
}

#[test]
fn comments_around_empty_enum_body_commas_stay_in_place() {
    for source in [
        "enum E { /* before */ , /* after */ }",
        "enum E { // before\n, // after\n}",
        "enum E { /* before */ , /* after */ ; int x; }",
        "enum E { // before\n, // after\n; int x; }",
        "enum E { ,; // field\nint x; }",
    ] {
        check_formatter_contract(source).expect("empty enum body comments must verify");
    }
}

#[test]
fn enum_constant_body_boundary_comments_stay_outside_the_body() {
    let source = "enum E { A /* bare */ {}, B() /* empty */ {}, C(1) /* populated */ { int x; } }";
    assert_snapshot!(fmt(source), @"
    enum E {
      A /* bare */ {},
      B() /* empty */ {},
      C(1) /* populated */ {
        int x;
      },
    }
    ");
}

#[test]
fn generic_field_types_break_before_names_that_do_not_fit() {
    for source in [
        "class A { class B { Map<String, List<Section>> NAME_TO_SECTION = make(); } }",
        "class A { class B { Map<String, Section> NAME_TO_SECTION; } }",
    ] {
        for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
            let options = Options { line_width: line_width(40), line_ending, ..Options::default() };
            format_stable_with_options(source, &options);
        }
    }
}

#[test]
fn type_name_breaks_keep_comments_and_dimensions() {
    let sources = [
        "class A { class B { Map<String, Section> /* type */ NAME_TO_SECTION = make(); } }",
        "class A { class B { Map<String, Section> // type\n NAME_TO_SECTION = make(); } }",
        "class A { class B { Map<String, Section>\n /* type */\n NAME_TO_SECTION; } }",
        "class A { class B { Map<String, Section> /* type\n * continued */ NAME_TO_SECTION; } }",
        "class A { class B { Map<String, List<Section>>[] NAME_TO_SECTION[][] = make(); } }",
        "class A { class B { Map<String, Section> /* type */ NAME_TO_SECTION[] /* dims */ []; } }",
        "class A { void f() { Map<String, Section>[] NAME_TO_SECTION = make(), OTHER = make(); } }",
        "class A { void f() { for (Map<String, Section> NAME_TO_SECTION = make();; ) {} } }",
    ];

    for source in sources {
        for width in [40, 100] {
            for step in [2, 4] {
                for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
                    let options = Options {
                        line_width: line_width(width),
                        indent_width: indent_width(step),
                        line_ending,
                        ..Options::default()
                    };
                    format_stable_with_options(source, &options);
                }
            }
        }
    }
}

#[test]
fn erased_type_name_comments_do_not_force_a_continuation() {
    let sources = [
        "class A { class B { Map<String, List<Section>>\n /** */\n NAME_TO_SECTION; } }",
        "class A { class B { int\n /** */\n NAME_TO_SECTION = make(); } }",
        "class A { class B { Map<String, Section> /* type */\n /** */\n NAME_TO_SECTION; } }",
    ];

    for source in sources {
        for width in [40, 100] {
            for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
                let options = Options {
                    line_width: line_width(width),
                    line_ending,
                    javadoc: Some(JavadocOptions::default()),
                    ..Options::default()
                };
                format_stable_with_options(source, &options);
            }
        }
    }
}

#[test]
fn comments_between_modifiers_lead_or_trail_them() {
    let source = "public /* a */ static // b\nabstract\n// c\nclass A {\n\
                  private /* d */\nfinal /* e */ int x;\n\
                  final\n/* f */ int y;\n\
                  public\n\n// g\n\nstatic void m() {} }\n\
                  final // h\nclass B {}\n\
                  final\n// i\nclass C {}\n\
                  final /* j */\nclass D {}\n\
                  final\n/* k */ class E {}\n\
                  final\n\n// l\n\nclass F {}";
    assert_snapshot!(fmt(source), @"
    public /* a */ static // b
    abstract
    // c
    class A {
      private /* d */ final /* e */ int x;
      final
      /* f */
      int y;

      public
      // g

      static void m() {}
    }

    final // h
    class B {}

    final
    // i
    class C {}

    final /* j */ class D {}

    final
    /* k */
    class E {}

    final

    // l
    class F {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn comments_before_the_first_varargs_annotation_lead_it() {
    let source = "class A { void a(int /* b */ @C /* d */ ... e) {} \
                  void f(int // g\n@C ... h) {} \
                  void i(int\n// j\n@C ... k) {} }";
    assert_snapshot!(fmt(source), @"
    class A {
      void a(int /* b */ @C /* d */ ... e) {}

      void f(
        int // g
        @C ... h
      ) {}

      void i(
        int
        // j
        @C ... k
      ) {}
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn annotation_element_default_value_wraps_after_default_when_it_does_not_fit() {
    let source = r#"@interface A {
  String message() default "{org.example.validation.constraints.SomeVeryLongConstraintWithAnEvenLongerName.message}";
  String[] names() default {"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"};
  long count() default 1;
}"#;
    assert_snapshot!(fmt(source), @r#"
    @interface A {
      String message() default
        "{org.example.validation.constraints.SomeVeryLongConstraintWithAnEvenLongerName.message}";

      String[] names() default {
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
      };

      long count() default 1;
    }
    "#);
    check_contract_at_widths_and_modes(source);
}

#[test]
fn type_parameter_bound_wraps_after_extends_when_it_does_not_fit() {
    let source = "class A<TypeParameterWithAnEvenLongerNameThanItNeeds extends \
                  org.example.verylongpackagename.anotherpackagename.SomeVeryLongTypeName> {\n  \
                  <TypeParameterWithAnEvenLongerNameThanItNeeds extends \
                  org.example.verylongpackagename.anotherpackagename.SomeVeryLongTypeName> void \
                  f() {}\n}";
    assert_snapshot!(fmt(source), @"
    class A<
        TypeParameterWithAnEvenLongerNameThanItNeeds extends
            org.example.verylongpackagename.anotherpackagename.SomeVeryLongTypeName> {
      <
        TypeParameterWithAnEvenLongerNameThanItNeeds extends
          org.example.verylongpackagename.anotherpackagename.SomeVeryLongTypeName
      > void f() {}
    }
    ");
    check_contract_at_widths_and_modes(source);
}
