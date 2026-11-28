//! Expressions, literals, operators, and expression-owned forms.

use cow_utils::CowUtils;
use insta::assert_snapshot;
use robusta_formatter::{JavadocOptions, Options};

use crate::fixtures::{
    check_contract_at_widths_and_modes, check_formatter_contract_with_options, format_stable,
    format_stable_with_options,
};
use crate::fixtures::{format_source as fmt, format_source_with_options as fmt_with_options};

#[test]
fn assignments_break_before_trailing_array_accesses() {
    let source = r"class A {
    private static final TypeVariable<Class<TypeLiteral>> T = TypeLiteral.class.getTypeParameters()[0];
    void m() {
        someVeryLongFieldNameForTheTest = TypeLiteralWithLongerName.class.getTypeParameters()[index];
    }
}";
    let options = Options { line_width: crate::fixtures::line_width(96), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      private static final TypeVariable<Class<TypeLiteral>> T =
        TypeLiteral.class.getTypeParameters()[0];

      void m() {
        someVeryLongFieldNameForTheTest =
          TypeLiteralWithLongerName.class.getTypeParameters()[index];
      }
    }
    ");
}

#[test]
fn array_assignment_layout_respects_breakable_receivers() {
    let source = r"class A {
    Object longVariableNameForTesting = values.get(shortKey)[index];
    Object anotherLongVariableName = getArray(firstLongArgumentName, secondLongArgumentName)[index];
    Object conditionalArrayValue = (condition ? firstArray : secondArray)[index];
    Object chainedArrayValue = source.first(firstArgument).second(secondArgument)[index];
    void m() {
        someVeryLongFieldNameForTheUnaryArrayTest &= ~set.words[i];
    }
}";
    let options = Options { line_width: crate::fixtures::line_width(60), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      Object longVariableNameForTesting =
        values.get(shortKey)[index];
      Object anotherLongVariableName = getArray(
        firstLongArgumentName,
        secondLongArgumentName
      )[index];
      Object conditionalArrayValue = (
        condition ? firstArray : secondArray
      )[index];
      Object chainedArrayValue = source
        .first(firstArgument)
        .second(secondArgument)[index];

      void m() {
        someVeryLongFieldNameForTheUnaryArrayTest &=
          ~set.words[i];
      }
    }
    ");
}

#[test]
fn assignments_break_after_the_operator_before_a_binary_value() {
    let source = r"class A {
    String field = order.customer().displayName() + suffix + more;
    @Size(maximum = firstLimit.value() + secondLimit.value() + offset)
    void m() {
        String label = order.customer().displayName() + suffix;
        String fits = order.customer() + suffix;
        boolean ok = a.isActive() && b.isPaid() && c.isShipped();
        boolean is = order.customer().displayName() instanceof LongerLabel;
        total += basePrice * quantity + shippingCost - discountAmount;
        String state = left > right ? firstValueLonger : secondValueLonger;
        try (Reader reader = first.open() + second.open() + third.open()) {}
    }
}";
    let options = Options {
        line_width: crate::fixtures::line_width(54),
        indent_width: crate::fixtures::indent_width(2),
        ..Options::default()
    };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      String field =
        order.customer().displayName() + suffix + more;

      @Size(
        maximum =
          firstLimit.value()
          + secondLimit.value()
          + offset
      )
      void m() {
        String label =
          order.customer().displayName() + suffix;
        String fits = order.customer() + suffix;
        boolean ok =
          a.isActive() && b.isPaid() && c.isShipped();
        boolean is =
          order.customer().displayName()
          instanceof LongerLabel;
        total +=
          basePrice * quantity
          + shippingCost
          - discountAmount;
        String state =
          left > right
            ? firstValueLonger
            : secondValueLonger;
        try (
          Reader reader =
            first.open() + second.open() + third.open()
        ) {}
      }
    }
    ");
}

#[test]
fn assignment_chains_break_every_link() {
    let source = r"class A {
    void m() {
        first = second = third = order.customer().displayName() + suffix;
        int count = first = second = order.customer().displayName() + suffix;
        first = second = order.customer().displayName() + suffix + other;
    }
}";
    let options = Options { line_width: crate::fixtures::line_width(54), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void m() {
        first =
          second =
          third =
            order.customer().displayName() + suffix;
        int count =
          first =
          second =
            order.customer().displayName() + suffix;
        first = second =
          order.customer().displayName() + suffix + other;
      }
    }
    ");
}

#[test]
fn literal_values_and_short_names_never_break_after_the_operator() {
    let source = r#"class A {
    private static final long SERIAL_VERSION_IDENTIFIER = 1234567890123L;
    private static final boolean SOME_FEATURE_IS_ENABLED_BY_DEFAULT = false;
    private static final String TEMPLATE_WITH_A_LONG_NAME = """
        text
        """;
    @Size(max = someConfiguration.limits().maximumLength())
    String name;
}"#;
    let options = Options { line_width: crate::fixtures::line_width(50), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class A {
      private static final long
        SERIAL_VERSION_IDENTIFIER = 1234567890123L;
      private static final boolean
        SOME_FEATURE_IS_ENABLED_BY_DEFAULT = false;
      private static final String
        TEMPLATE_WITH_A_LONG_NAME = """
          text
          """;
      @Size(
        max = someConfiguration
          .limits()
          .maximumLength()
      )
      String name;
    }
    "#);
}

#[test]
fn assignment_comments_keep_their_lines() {
    let source = r"class A {
    void m() {
        value = // why
            first + second;
        value = /* why */
            first + second;
        value =
            // why
            first + second;
        int count = // why
            1;
        count =
            /* why */ /* and */
            1;
    }
}";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        value = // why
          first + second;
        value = /* why */ first + second;
        value =
          // why
          first + second;
        int count = // why
          1;
        count =
          /* why */ /* and */
          1;
      }
    }
    ");
}

#[test]
fn comments_before_an_assignment_operator_trail_the_left_side_or_lead_the_right() {
    let source = "class A { void m() {
        a /* c */ = b;
        a /* c */
        = b;
        a // c
        = b;
        a /* c */ // d
        = b;
        a // c
        = // d
        b;
        a // c
        = /* d */ b;
        a // c
        = /* d */ // e
        b;
        a // c
        = /* d */
        b;
        a// c
        = /* d */ 1;
        a
        // c
        = b;
        a
        /* c */ = b;
        a

        // c
        = b;
        a /* c */
        // d
        = b;
        a /* c
         */ = b;
        a /* c
         */
        = b;
        int[] e // c
        = {1};
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        a /* c */ = b;
        a /* c */ = b;
        a = // c
          b;
        a /* c */ = // d
          b;
        a // c
          = // d
          b;
        a = // c
          /* d */ b;
        a // c
          = /* d */ // e
          b;
        a = // c
          /* d */
          b;
        a = // c
          /* d */ 1;
        a =
          // c
          b;
        a = /* c */ b;
        a =
          // c
          b;
        a /* c */ =
          // d
          b;
        a /* c
           */ = b;
        a =
          /* c
           */
          b;
        int[] e = // c
          {1};
      }
    }
    ");
}

/// A line comment written against the left side starts where the left side
/// ends, not after it, so the layout does not count it as riding past the
/// operator: a literal right side stays on the line. The comment before a
/// cast's `)` breaks the operand either way.
#[test]
fn line_comments_glued_to_the_left_side_do_not_ride_past_the_operator() {
    let source = "class A { void m() {
        a// c
        = b;
        a// c
        = 1;
        a// c
        += 1;
        x = (T// c
        ) y;
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        a = // c
          b;
        a = 1; // c
        a += 1; // c
        x = (
          T // c
        ) y;
      }
    }
    ");
}

#[test]
fn documentation_comments_before_an_assignment_operator_lay_out_as_line_comments() {
    let source = "class A {
        int a /// B
        = /// C
        1;
        int b /** B */ = 1;
        int c /// B
        ;
    }";
    let formatted = fmt(source);
    assert_snapshot!(formatted, @"
    class A {
      int a /// B
        = /// C
        1;
      int b /** B */ = 1;
      int c /// B
      ;
    }
    ");
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_eq!(fmt_with_options(source, &options), formatted);
}

/// A `///` comment written against the left side trails the right side, and
/// the `;` starts the next line: behind the `;`, the comment would document the
/// declaration that follows.
#[test]
fn documentation_comments_glued_to_the_left_side_stay_before_the_semicolon() {
    let source = "class A {
        int a/// B
        = 1;
        int b;
        @interface C {
            int c()/// D
            default 1;
            int d();
        }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      int a = 1 /// B
      ;
      int b;

      @interface C {
        int c() default 1 /// D
        ;

        int d();
      }
    }
    ");
}

#[test]
fn comments_before_a_lambda_arrow_stay_on_the_signature_line_or_lead_the_body() {
    let source = "class A { void m() {
        x = (a) /* c */ -> b;
        x = (a) /* c */
        -> b;
        x = (a) /* c
         */ -> b;
        x = (a) // c
        -> b;
        x = (a) /* c */ // d
        -> b;
        x = (a)
        // c
        -> b;
        x = (a)
        /* c */ -> b;
        x = (a) /* c
         */
        -> b;
        x = (a) // c
        -> // d
        b;
        x = a // c
        -> b;
        x = (a) // c
        -> { b(); };
        x = (a) /* c */ -> { b(); };
        g((a) /* c */
        -> { b(); });
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        x = (a) /* c */ -> b;
        x = (a) /* c */ -> b;
        x = (a) /* c
                 */ -> b;
        x = (a) ->
          // c
          b;
        x = (a) /* c */ ->
          // d
          b;
        x = (a) ->
          // c
          b;
        x = (a) ->
          /* c */ b;
        x = (a) ->
          /* c
           */
          b;
        x = (a) ->
          // c
          // d
          b;
        x = a ->
          // c
          b;
        x = (a) -> { // c
          b();
        };
        x = (a) /* c */ -> {
          b();
        };
        g((a) /* c */ -> {
          b();
        });
      }
    }
    ");
}

#[test]
fn comments_ending_their_line_before_a_lambda_block_follow_its_brace() {
    let source = "class A { void m() {
        rule = (a) // unused
        -> price;
        check = (a) // unused
        -> { return price; };
        check = (a) /* unused */ -> { return price; };
        check = (a) -> // unused
        { return price; };
        check = (a) -> /* unused */ // later
        {};
        check = (a) -> /** unused */
        { class Price {} };
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        rule = (a) ->
          // unused
          price;
        check = (a) -> { // unused
          return price;
        };
        check = (a) /* unused */ -> {
          return price;
        };
        check = (a) -> { // unused
          return price;
        };
        check = (a) -> /* unused */ { // later
        };
        check = (a) -> /** unused */ {
          class Price {}
        };
      }
    }
    ");
}

#[test]
fn text_blocks_are_placed_as_array_literals_and_their_lines_move_with_them() {
    let source = r#"class Shop {
    String footer = """
        Thanks""";
    String menu = """
    Tea
        Green
Coffee
""";
    String greeting = """
  Hello
  """ + name;
    String footerWithStoreAddressAndOpeningHours = """
        Main Street 1""";

    void send(Order order) {
        log("""
            Paid
            """);
        log(order, """
            Paid
            """);
    }
}"#;
    let options = Options { line_width: crate::fixtures::line_width(50), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class Shop {
      String footer = """
        Thanks""";
      String menu = """
            Tea
                Green
        Coffee
        """;
      String greeting =
        """
        Hello
        """ + name;
      String footerWithStoreAddressAndOpeningHours =
        """
        Main Street 1""";

      void send(Order order) {
        log("""
          Paid
          """);
        log(order, """
          Paid
          """);
      }
    }
    "#);
}

#[test]
fn text_block_lines_keep_the_white_space_the_compiler_keeps() {
    let source = "class A {\n\
        String tab = \"\"\"\n\t\ttab\n\t\t\"\"\";\n\
        String mixed = \"\"\"\n\t  mixed\n\t    deeper\n\t  \"\"\";\n\
        String blank = \"\"\"\n        first\n   \n\n          \n        second\n        \"\"\";\n\
        String opensBlank = \"\"\"\n\n        after\n        \"\"\";\n\
        String escapes = \"\"\"\n        joined \\\n        line\\s\n        \"\"\";\n\
        String closedAfterText = \"\"\"\n            a\n          b\"\"\";\n\
        String lessThanClosing = \"\"\"\n    left\n        \"\"\";\n\
        String moreThanClosing = \"\"\"\n            right\n        \"\"\";\n\
        String ideographic = \"\"\"\n\u{3000}\u{3000}wide\n\u{3000}\u{3000}\"\"\";\n\
        String escaped = \"\"\"\n\\u0020   kept\n    \"\"\";\n\
    }";
    assert_snapshot!(fmt(source), @r#"
    class A {
      String tab = """
        tab
        """;
      String mixed = """
        mixed
          deeper
        """;
      String blank = """
        first



        second
        """;
      String opensBlank = """

        after
        """;
      String escapes = """
        joined \
        line\s
        """;
      String closedAfterText = """
          a
        b""";
      String lessThanClosing = """
        left
        """;
      String moreThanClosing = """
            right
        """;
      String ideographic = """
        wide
        """;
      String escaped = """
    \u0020   kept
        """;
    }
    "#);
}

#[test]
fn text_block_arguments_keep_a_chain_on_one_line_as_array_initializers_do() {
    let source = r#"class A { void m() {
        assertThat(error).hasMessageThat().contains(new String[] {
            "Paid",
        });
        assertThat(error).hasMessageThat().contains("""
            Paid
            """);
        assertThat(error).hasMessageThat().contains(format("""
            Paid
            """));
    } }"#;
    assert_snapshot!(fmt(source), @r#"
    class A {
      void m() {
        assertThat(error).hasMessageThat().contains(new String[] {"Paid"});
        assertThat(error).hasMessageThat().contains("""
          Paid
          """);
        assertThat(error)
          .hasMessageThat()
          .contains(
            format("""
              Paid
              """)
          );
      }
    }
    "#);
}

#[test]
fn text_block_lambda_bodies_stay_on_the_arrow_line() {
    let source = r#"class A { void m() {
        supplier = () -> """
            text
            """;
        supplier = () ->
            """
            text
            """;
        supplier = () ->\u000a """
            text
            """;
        run(() -> /* c */ """
            text
            """);
        chain = a -> () -> """
            text
            """;
        supplier = () -> """\u000a    a text block written on one line that is too wide for it\u000a    """;
    } }"#;
    assert_snapshot!(fmt(source), @r#"
    class A {
      void m() {
        supplier = () -> """
          text
          """;
        supplier = () -> """
          text
          """;
        supplier = () -> """
          text
          """;
        run(() -> /* c */ """
          text
          """);
        chain = a -> () -> """
          text
          """;
        supplier =
          () -> """\u000a    a text block written on one line that is too wide for it\u000a    """;
      }
    }
    "#);
}

#[test]
fn line_comments_before_text_block_lambda_arrows_move_onto_their_own_lines() {
    let source = r#"class A { void m() {
      supplier = () // assignment
      -> """
          text
          """;
      run(() // only
      -> """
          text
          """);
      run(() // first
      -> """
          text
          """, name);
    } }"#;
    let formatted = fmt(source);
    assert_snapshot!(formatted, @r#"
    class A {
      void m() {
        supplier = () ->
          // assignment
          """
          text
          """;
        run(() ->
          // only
          """
          text
          """
        );
        run(
          () ->
            // first
            """
            text
            """,
          name
        );
      }
    }
    "#);
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_eq!(fmt_with_options(source, &options), formatted);
}

#[test]
fn comments_around_a_cast_paren_trail_the_type_or_lead_the_operand() {
    let source = "class A { void m() {
        x = (T /* c */) y;
        x = (T /* c */
        ) y;
        x = (T // c
        ) y;
        x = (T /* c */ // d
        ) y;
        x = (T
        // c
        ) y;
        x = (T
        /* c */) y;
        x = (T /* c
         */) y;
        x = (T /* c
         */
        ) y;
        x = (T // c
        ) // d
        y;
        x = (T // c
        ) /* d */ y;
        x = (T
        // c
        ) /* d */ // e
        y;
        x = (T) /* c */ y;
        x = (T) /* c */
        y;
        x = (T) // c
        y;
        x = (T)
        // c
        y;
        x = (T)
        /* c
         */
        y;
        x = (T)
        /* c
         */ y;
        x = (A & B // c
        ) y;
        f((T // c
        ) y, z);
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        x = (T /* c */) y;
        x = (T /* c */) y;
        x = (
          T // c
        ) y;
        x = (
          T /* c */ // d
        ) y;
        x = (T)
          // c
          y;
        x = (T) /* c */ y;
        x = (
          T /* c
             */
        ) y;
        x = (
          T /* c
             */
        ) y;
        x = (
          T // c
        ) // d
          y;
        x = (
          T // c
        ) /* d */ y;
        x = (T)
          // c
          /* d */ // e
          y;
        x = (T) /* c */ y;
        x = (T) /* c */ y;
        x = (T) // c
          y;
        x = (T)
          // c
          y;
        x = (T)
          /* c
           */
          y;
        x = (T) /* c
                 */ y;
        x = (
          A & B // c
        ) y;
        f(
          (
            T // c
          ) y,
          z
        );
      }
    }
    ");
}

/// The comment on its own line before `)` and the block comment ending the
/// `)` line lead the operand, which joins the `)` line.
#[test]
fn own_line_comment_before_a_cast_paren_joins_the_operand_after_a_block() {
    let source = "class A {
        void m() { x = (T\n/* c */) /* d */\ny; }
        T n() { return (T\n/* c */) /* d */\nvalueFromTheOperation; }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        x = (T) /* c */ /* d */ y;
      }

      T n() {
        return (T) /* c */ /* d */ valueFromTheOperation;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

/// A comment on its own line after a cast's `(` stays there, and the types
/// are indented inside the parentheses.
#[test]
fn own_line_comment_after_a_cast_open_paren_indents_the_types() {
    let source = "class A { void m() {
        x = (
        /* c */
        Object) o;
        x = (
        /* c */
        Comparable<String> & java.io.Serializable) o;
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        x = (
          /* c */
          Object
        ) o;
        x = (
          /* c */
          Comparable<String> & java.io.Serializable
        ) o;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

/// A comment that spans lines before `)` stays after the types when the
/// parentheses break.
#[test]
fn multi_line_comment_before_a_broken_cast_paren_trails_the_types() {
    let source = "class A { void m() {
        x = (B
        /* first */
        & C /* second
         */) x;
        x = (
        // o
        java.util.concurrent.ConcurrentMap /* c
         */) y;
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        x = (
          B
            /* first */
            & C /* second
                 */
        ) x;
        x = (
          // o
          java.util.concurrent.ConcurrentMap /* c
                                              */
        ) y;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn a_cast_indents_deferred_comments_that_need_a_line_break() {
    let source = "class A { void m() {
        x = (T
        /* c */) // d
        y;
        x = (T
        /* c */)
        /* d */ y;
        x = (T
        /* c */)
        y;
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        x = (T)
          /* c */ // d
          y;
        x = (T)
          /* c */
          /* d */ y;
        x = (T)
          /* c */
          y;
      }
    }
    ");
    check_contract_at_widths_and_modes(source);
    let escaped = source.cow_replace("/* c */)", r"/* c */\u0029");
    assert_eq!(fmt(&escaped), fmt(source));
    check_contract_at_widths_and_modes(&escaped);
}

#[test]
fn multi_line_block_comments_around_operator_tokens_are_stable_on_a_second_pass() {
    let source = "class A { void m() {
        a
        /* c */ =
        /* d
         */ b;
        x = (T) /* c
         */ /* d
         */
        y;
        x = (T) /* c */
        /* d
         */ y;
        x = (a) /* c
         */
        -> { b(); };
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        a =
          /* c */ /* d
                   */ b;
        x = (T)
          /* c
           */ /* d
               */
          y;
        x = (T) /* c */ /* d
                         */ y;
        x = (a) -> /* c
                    */ {
          b();
        };
      }
    }
    ");
}

#[test]
fn a_block_comment_ending_the_operator_line_leads_the_value() {
    let source = r"class A {
    static final Map<String, String> DEFAULTS = /* c */
        new HashMap<>();
}";
    let options = Options { line_width: crate::fixtures::line_width(52), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      static final Map<String, String> DEFAULTS =
        /* c */ new HashMap<>();
    }
    ");
}

#[test]
fn block_comments_after_an_assignment_operator_keep_its_line() {
    let source = "class A { void m() {
    pricing = /* per order */ (Order... orders) -> customer -> totalFor(orders, customer);
    totalPrice = /* before tax */ basePrice * quantity + shippingCost + handlingFee;
    Map<String, List<Order>> ordersByCustomer = /* filled lazily */ new HashMap<>();
    rate = /* lead,
   kept */ order -> customer -> order.rate();
} }";
    // A line width of 60 for the statements, indented by four columns.
    let options = Options { line_width: crate::fixtures::line_width(64), ..Options::default() };
    check_formatter_contract_with_options(source, &options).unwrap();
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void m() {
        pricing = /* per order */
          (Order... orders) ->
          customer ->
            totalFor(orders, customer);
        totalPrice = /* before tax */
          basePrice * quantity + shippingCost + handlingFee;
        Map<String, List<Order>> ordersByCustomer =
          /* filled lazily */ new HashMap<>();
        rate = /* lead,
       kept */ order -> customer -> order.rate();
      }
    }
    ");
}

#[test]
fn multiline_block_comments_around_a_lambda_chain_assignment_are_stable() {
    let source = "class A { void f() {\n h = /* a\n  b */ x /* a\n  b */ -> y -> x + y;\n} }\n";
    check_formatter_contract_with_options(source, &Options::default()).unwrap();
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        h = /* a
      b */
          x /* a
      b */ ->
          y ->
            x + y;
      }
    }
    ");
}

#[test]
fn numeric_array_initializers_fill_lines() {
    let options = Options { line_width: crate::fixtures::line_width(60), ..Options::default() };
    for elements in [
        "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20",
        "+1, -2, +3, -4, +5, -6, +7, -8, +9, -10, +11, -12, +13, -14, +15, -16",
        "12345678901L, 12345678902L, 12345678903L, 12345678904L, 12345678905L",
        "1.0, -2.0, +3.0, 4.0f, 5e10, 6e20, 0x1.fp3, 8.0, 9.0, 10.0, 11.0",
    ] {
        for source in [
            format!("class A {{ Object values = new double[] {{{elements}}}; }}"),
            format!("@X({{{elements}}}) class A {{}}"),
        ] {
            let formatted = format_stable_with_options(&source, &options);
            assert!(formatted.lines().any(|line| line.matches(',').count() > 1), "{formatted}");
        }
    }
}

#[test]
fn numeric_array_layout_checks_elements_and_comments() {
    let options = Options { line_width: crate::fixtures::line_width(40), ..Options::default() };
    for (element, fill) in [
        ("-1", true),
        ("+1", true),
        ("/* before */ -1", true),
        ("\n// own line\n-1", true),
        ("(byte) 0x7f", false),
        ("~1", false),
        ("!true", false),
        ("(1)", false),
        ("value", false),
        ("null", false),
        ("'a'", false),
        ("\"text\"", false),
        ("@Y(1)", false),
        ("{1, 2}", false),
        ("- /* inside */ 1", false),
        ("- // inside\n1", false),
        ("1 // trailing\n", false),
    ] {
        let values = format!("11, 12, 13, 14, 15, 16, {element}, 17, 18, 19, 20, 21, 22");
        for source in [
            format!("@X({{{values}}}) class A {{}}"),
            format!("class A {{ Object[] values = {{{values}}}; }}"),
        ] {
            if element.starts_with('@') && !source.starts_with('@') {
                continue;
            }
            let formatted = format_stable_with_options(&source, &options);
            assert_eq!(formatted.contains("11, 12"), fill, "{element}: {formatted}");
        }
    }
}

#[test]
fn numeric_array_opening_comments_keep_the_layout_in_one_pass() {
    let values = "11, 12, 13, 14, 15, 16, 17, 18, 19, 20";
    for leading in [" // header\n", " /* block */ // header\n", "\n/* block */ // header\n"] {
        for source in [
            format!("class A {{ int[] values = {{{leading}{values}}}; }}"),
            format!("@X({{{leading}{values}}}) class A {{}}"),
        ] {
            for line_ending in
                [robusta_formatter::LineEnding::Lf, robusta_formatter::LineEnding::Crlf]
            {
                let options = Options {
                    line_width: crate::fixtures::line_width(40),
                    line_ending,
                    ..Options::default()
                };
                let formatted = format_stable_with_options(&source, &options);
                assert!(formatted.contains("11, 12"), "{formatted}");
                let escaped =
                    source.cow_replacen(&format!("{{{leading}"), &format!(r"\u007b{leading}"), 1);
                assert_eq!(format_stable_with_options(&escaped, &options), formatted);
            }
        }
    }
}

#[test]
fn numeric_array_comment_layout_uses_logical_whitespace() {
    let values = "11, 12, 13, 14, 15, 16, 17, 18, 19, 20";
    let options = Options {
        line_width: crate::fixtures::line_width(40),
        line_ending: robusta_formatter::LineEnding::Lf,
        ..Options::default()
    };
    for (gap, physical) in [
        (r"\u0020", " "),
        (r"\uu0009", "\t"),
        (r"\u000a", "\n"),
        (r"\uuu000d\u000A\u0009", "\r\n\t"),
    ] {
        for source in [
            format!("class A {{ int[] values = {{GAP// header\n{values}}}; }}"),
            format!("@X({{GAP// header\n{values}}}) class A {{}}"),
            format!("class A {{ int[] values = {{{values}, GAP// header\n21, 22}}; }}"),
        ] {
            let ordinary = source.cow_replace("GAP", physical);
            let escaped = source.cow_replace("GAP", gap);
            assert_eq!(
                format_stable_with_options(&escaped, &options),
                format_stable_with_options(&ordinary, &options)
            );
        }
    }
}

#[test]
fn comments_in_signed_numbers_and_trailing_line_comments_stop_the_numeric_fill() {
    for javadoc in [None, Some(JavadocOptions::default())] {
        let options =
            Options { line_width: crate::fixtures::line_width(40), javadoc, ..Options::default() };
        let fills = |values: &str| {
            [
                format!("class A {{ int[] values = {{{values}}}; }}"),
                format!("@X({{{values}}}) class A {{}}"),
            ]
            .map(|source| format_stable_with_options(&source, &options).contains("11, 12"))
        };
        for comment in ["/**\n */", "/** */\n", "\n/** */\n", "/** */", "/*\n */", "/** keep\n */"]
        {
            for sign in ["+", "-"] {
                assert_eq!(
                    fills(&format!("11,12,13,14,15,{sign} {comment} 1,16,17,18,19,20")),
                    [false, false]
                );
            }
        }
        // Before the first element, the comments print on their own line.
        for comment in [
            "/**\n */ // header\n",
            "\n/** */ // header\n",
            "\n/**\n */ // header\n",
            "/*\n */ // header\n",
            "\n/* keep */ // header\n",
        ] {
            assert_eq!(fills(&format!("{comment}11,12,13,14,15,16,17,18,19,20")), [true, true]);
            assert_eq!(fills(&format!("11,12,13,14,15,{comment}16,17,18,19,20")), [false, false]);
        }
    }
}

fixture_tests! {
    golden_expressions_file => "expressions/expressions";
    golden_class_instance_creations_file => "expressions/class-instance-creations";
    golden_class_literals_file => "expressions/class-literals";
    golden_qualified_this_file => "expressions/qualified-this";
    golden_qualified_super_file => "expressions/qualified-super";
    golden_qualified_new_file => "expressions/qualified-new";
    golden_arrays_file => "expressions/arrays";
    golden_increments_file => "expressions/increments";
    golden_record_patterns_file => "expressions/record-patterns";
    golden_operators_file => "expressions/operators";
    golden_instanceof_patterns_file => "expressions/instanceof-patterns";
    golden_character_literals_file => "expressions/character-literals";
    golden_text_block_literals_file => "expressions/text-block-literals";
    golden_integral_literals_file => "expressions/integral-literals";
    golden_floating_point_literals_file => "expressions/floating-point-literals";
    golden_lambdas_file => "expressions/lambdas";
    golden_method_references_file => "expressions/method-references";
    golden_generic_method_invocations_file => "expressions/generic-method-invocations";
    golden_generic_constructor_invocations_file => "expressions/generic-constructor-invocations";
}

#[test]
fn array_member_chains_stay_flat() {
    let source =
        "class A { Object f() { return source.next()[i++].field[j].call(argument())[k()]; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object f() {
        return source.next()[i++].field[j].call(argument())[k()];
      }
    }
    ");
}

#[test]
fn array_member_chains_preserve_bracket_and_selector_comments() {
    let sources = [
        "class A { Object f() { return source /* a */ .next(/* b */) /* c */ [/* d */ index() /* e */] /* f */ .field /* g */ [other()] /* h */ .call(argument()); } }",
        "class A { Object f() { return source.next() // before open\n[// after open\nindex() // before close\n] // before dot\n. // after dot\nfield[other()].call(argument()); } }",
        "class A { Object f() { return source.next()[indices.first().second()][j++].call(argument().next()[k]).field; } }",
        "class A { Object f() { return source.firstLongMethodName()[index()].secondLongMethodName()[other()].thirdLongMethodName(); } }",
        "class A { Object f() { return source[first /* inner */ + second /* close */].field; } }",
        "class A { Object f() { return source[index(/* inner */) /* close */].field; } }",
        "class A { Object f() { return source[indices /* inner */ .first() // close\n].field; } }",
        "class A { Object f() { return source[\n/* first\n * second */\nindex(/* inner */) /* close */].field; } }",
    ];
    for source in sources {
        for width in [40, 80, 120] {
            let options =
                Options { line_width: crate::fixtures::line_width(width), ..Options::default() };
            check_formatter_contract_with_options(source, &options)
                .unwrap_or_else(|error| panic!("width {width}: {error}\n{source}"));
        }
    }
}

#[test]
fn nested_parenthesized_expressions_keep_boundary_comments_inside() {
    let source = "class A { int f(int x) { return (/* open */ (// inner-open\n\
                  x // inner-close\n) /* close */); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int f(int x) {
        return (/* open */ ( // inner-open
          x // inner-close
        ) /* close */);
      }
    }
    ");
}

#[test]
fn empty_record_pattern_keeps_comments_inside() {
    let source = "class A { void f(Object x) { switch(x) { case Empty(/* c */) -> y(); default -> z(); } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(Object x) {
        switch (x) {
          case Empty(/* c */) -> y();
          default -> z();
        }
      }
    }
    ");
}

#[test]
fn diamond_keeps_comments_inside() {
    let source = "class A { Object m = new java.util.HashMap<\n/* c */\n>(); }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object m = new java.util.HashMap<
        /* c */
      >();
    }
    ");
}

#[test]
fn type_argument_lists_keep_their_comments() {
    let source = "class A { void f() {
        a = new ArrayList<>();
        b = new <T> Foo<>();
        c = List.<String>of();
        d = Foo::<T>bar;
        e = new Foo</* c */>();
        g = new Inner<String> /* c */ ();
        h = outer.new Inner<String> /* c */ ();
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        a = new ArrayList<>();
        b = new <T> Foo<>();
        c = List.<String>of();
        d = Foo::<T>bar;
        e = new Foo</* c */>();
        g = new Inner<String> /* c */();
        h = outer.new Inner<String> /* c */();
      }
    }
    ");
}

#[test]
fn comments_before_commented_empty_creation_arguments_break_inside_the_list() {
    let source = "class A {
        Object a = new Inner<String> // before
        ( /* inside */ );
        Object b = outer.new Inner<String> // before
        ( /* inside */ );
        Object c = new Inner<String> // before
        /* next */
        ( /* inside */ );
        Object d = outer.new Inner<> // before
        (
        /* inside */
        );
        Object e = outer.new Inner // before
        ( // inside
        );
        Object f = new Inner<String>
        /* before */
        ( /* inside */ );
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object a = new Inner<String>( // before
        /* inside */
      );
      Object b = outer.new Inner<String>( // before
        /* inside */
      );
      Object c = new Inner<String>( // before
        /* next */
        /* inside */
      );
      Object d = outer.new Inner<>( // before
        /* inside */
      );
      Object e = outer.new Inner( // before
        // inside
      );
      Object f = new Inner<String>(
        /* before */
        /* inside */
      );
    }
    ");
}

#[test]
fn empty_delimiter_comments_follow_configured_layout() {
    for options in [
        Options::default(),
        Options { javadoc: Some(JavadocOptions::default()), ..Options::default() },
        Options { insert_final_newline: false, ..Options::default() },
    ] {
        for comment in [
            "/* c */",
            "// c\n",
            "\n/* c */\n",
            "/* multi\nline */",
            "\n/* multi\nline */\n",
            "/* a */ /* b */",
            "/** doc */",
            "/** */",
        ] {
            for source in [
                format!("enum E {{ UNIT({comment}) {{}}, B }}"),
                format!(
                    "class A {{ void f(Object x) {{ if(x instanceof Empty({comment})) {{}} }} }}"
                ),
                format!("class A {{ Object m = new C<{comment}>(); }}"),
                format!("class A {{ Object m = new C<{comment}>(/* arg */) {{}}; }}"),
                format!(
                    "class A {{ void f(Object x) {{ if(x instanceof Pair(Empty({comment}), int n)) {{}} }} }}"
                ),
            ] {
                check_formatter_contract_with_options(&source, &options)
                    .unwrap_or_else(|err| panic!("{source:?}: {err}"));
            }
        }
    }
}

#[test]
fn record_pattern_keeps_comments_before_its_closer() {
    let source = "class A { void f(Object o) { switch (o) { case P(int a, Q(int b)\n/* own */\n) -> y(); \
                  case P(int a /* inline */) -> z(); default -> {} } } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f(Object o) {
        switch (o) {
          case P(
              int a,
              Q(int b)
              /* own */
            ) -> y();
          case P(int a /* inline */) -> z();
          default -> {}
        }
      }
    }
    ");
}

#[test]
fn empty_array_initializers_drop_the_optional_comma_and_keep_comments() {
    let source = "class A { int[] a = {,}; int[] b = { , }; int[][] c = {{,}, {1,}}; \
                  int[] d = new int[] {,}; int[] e = {/* before */, /* after */}; \
                  int[] f = {, // after\n}; void f() { int[] a = {/* before */,}; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int[] a = {};
      int[] b = {};
      int[][] c = {{}, {1}};
      int[] d = new int[] {};
      int[] e = {
        /* before */
        /* after */
      };
      int[] f = {
        // after
      };

      void f() {
        int[] a = {/* before */};
      }
    }
    ");
}

#[test]
fn array_creation_comments_stay_before_the_initializer_brace() {
    let source = "class A { int[] a = new int[] /* outer */ { /* inner */ 10 }; \
                  int[][] b = new int[][] /* nested */ {{1}}; \
                  int[] c = new int @Y [] /* empty */ {}; }";
    assert_snapshot!(fmt(source), @"
    class A {
      int[] a = new int[] /* outer */ {/* inner */ 10};
      int[][] b = new int[][] /* nested */ {{1}};
      int[] c = new int @Y [] /* empty */ {};
    }
    ");
}

#[test]
fn array_creation_line_comments_stay_before_the_initializer_brace() {
    let source = "class A { int[] a = new int[] // outer\n{10}; \
                  int[][] b = new int[][] // nested\n{{1}}; \
                  int[] c = new int[] // empty\n{}; }";
    assert_snapshot!(fmt(source), @"
    class A {
      int[] a = new int[] // outer
      {10};
      int[][] b = new int[][] // nested
      {{1}};
      int[] c = new int[] // empty
      {};
    }
    ");
}

#[test]
fn array_creation_own_line_comments_stay_before_the_initializer_brace() {
    let source = "class A { int[] a = new int[]\n/* outer */\n{10}; \
                  int[] b = new int[]\n/* first\n   second */ {20}; }";
    assert_snapshot!(fmt(source), @"
    class A {
      int[] a = new int[]
      /* outer */
      {10};
      int[] b = new int[]
      /* first
       second */ {20};
    }
    ");
}

#[test]
fn trailing_array_initializer_commas_are_dropped_when_flat() {
    // The trailing comma is formatter-owned, emitted iff the enclosing group
    // breaks. A flat array initializer never breaks, so a source trailing
    // comma is dropped here even though nothing else about these literals is
    // wrong.
    let source = "class C { int[] xs = {1, 2,}; int[][] ys = new int[][] {{1,}, {2,},}; }";
    assert_snapshot!(fmt(source), @"
    class C {
      int[] xs = {1, 2};
      int[][] ys = new int[][] {{1}, {2}};
    }
    ");
}

#[test]
fn numeric_array_trailing_comma_comments_format_in_one_pass() {
    let options = Options::default();
    let source = "class A { int[] a = {1, 2, /* inline */\n}; \
                  int[] b = {1,\n /* own */\n}; }";
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      int[] a = {1, 2 /* inline */};
      int[] b = {
        1,
        /* own */
      };
    }
    ");
}

#[test]
fn numeric_array_fills_before_own_line_comments_after_its_last_element() {
    let source = "class A { int[] a = {1, 2,\n// c\n}; int[] b = {-1, 2\n// c\n\n// d\n}; }";
    let javadoc = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_eq!(fmt_with_options(source, &javadoc), fmt(source));
    assert_snapshot!(fmt(source), @"
    class A {
      int[] a = {
        1, 2,
        // c
      };
      int[] b = {
        -1, 2,
        // c

        // d
      };
    }
    ");
}

#[test]
fn trailing_array_comma_comments_keep_their_lines() {
    let options = Options { line_width: crate::fixtures::line_width(40), ..Options::default() };
    let source = "class A { E[] a = {new E(1, 2), // a longer comment\n}; \
                  String[] b = {\"One\" // a\n/* b */\n, // c\n}; \
                  int[] c = {1 // a\n, /* b */\n}; \
                  String[] d = {\"One\", // a\n\n// b\n}; }";
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class A {
      E[] a = {
        new E(1, 2), // a longer comment
      };
      String[] b = {
        "One", // a
        /* b */ // c
      };
      int[] c = {
        1, // a
        /* b */
      };
      String[] d = {
        "One", // a

        // b
      };
    }
    "#);
}

#[test]
fn trailing_array_comma_comments_before_the_brace_trail_the_last_element() {
    let source = "class A { int[] a = {1, 2, /* c */}; int[] b = {1, /** d */ /* e */}; \
                  @X({\"a\", /* f */}) int c; }";
    let javadoc = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_eq!(fmt_with_options(source, &javadoc), fmt(source));
    assert_snapshot!(fmt(source), @r#"
    class A {
      int[] a = {1, 2 /* c */};
      int[] b = {1 /** d */ /* e */};
      @X({"a" /* f */}) int c;
    }
    "#);
}

#[test]
fn nested_numeric_arrays_keep_trailing_commas() {
    let options = Options { line_width: crate::fixtures::line_width(40), ..Options::default() };
    let source = "class A { int[][] a = {{1, 2, 3}, \
                  {4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20}}; }";
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      int[][] a = {
        {1, 2, 3},
        {
          4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
          14, 15, 16, 17, 18, 19, 20,
        },
      };
    }
    ");
}

#[test]
fn trailing_initializer_comments_follow_configured_contracts() {
    use robusta_formatter::LineEnding;

    let sources = [
        "class A { int[] a = {1, 2, /* c */\n}; }",
        "class A { int[] a = {1,\n /* c */\n}; }",
        "@X({1, 2, /* c */\n}) class A {}",
        "class A { int[][] a = {{1, /* inner */\n}, {2}, /* outer */\n}; }",
        "class A { int[] a = {1, 2, // c\n}; }",
        "class A { int[] a = {1 /* before */, /* after */\n}; }",
        "class A { int[] a = {1 // before\n, /* after */\n}; }",
        "class A { int[] a = {1,\n/* multi\nline */\n}; }",
        "class A { int[] a = {1, /** doc */\n}; }",
        "class A { int[] a = {1,\n/** */\n/* c */\n}; }",
        "@X(value = {1,\n/* a */ /* b */\n}) class A {}",
    ];
    for source in sources {
        for line_ending in [LineEnding::Lf, LineEnding::Crlf] {
            for javadoc in [None, Some(JavadocOptions::default())] {
                let options = Options { line_ending, javadoc, ..Options::default() };
                check_formatter_contract_with_options(source, &options)
                    .unwrap_or_else(|err| panic!("{source:?}: {err}"));
            }
        }
    }
}

#[test]
fn typed_lambda_parameter_lists_break_like_parameter_lists() {
    let source = "class C { void f() { consume = (ExtremelyLongBuilderTypeName firstParameter, \
                  AnotherExtremelyLongTypeName secondParameter) -> firstParameter; } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        consume = (
          ExtremelyLongBuilderTypeName firstParameter,
          AnotherExtremelyLongTypeName secondParameter
        ) -> firstParameter;
      }
    }
    ");
}

#[test]
fn bare_name_casts_and_non_casts_keep_their_reading() {
    // `(a) - b` is a subtraction and `(a) b` a cast; both must survive the
    // round trip unchanged.
    let source = "class A { void f() { x = (a) - b; y = (a) b; z = (x)++; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void f() {
        x = (a) - b;
        y = (a) b;
        z = (x)++;
      }
    }
    ");
}

#[test]
fn bare_name_casts_keep_switch_and_primitive_headed_operands() {
    let source = "class A { Object f(int x) { \
        o = (Object)int.class; o = (IntFunction)int[]::new; \
        return (Object)switch (x) { case 1 -> \"a\"; default -> \"b\"; }; } }";
    assert_snapshot!(fmt(source), @r#"
    class A {
      Object f(int x) {
        o = (Object) int.class;
        o = (IntFunction) int[]::new;
        return (Object) switch (x) {
          case 1 -> "a";
          default -> "b";
        };
      }
    }
    "#);
}

#[test]
fn intersection_casts_join_their_bounds_with_ampersands() {
    let source = "class A { Object f(Object o) { \
        Runnable r = (Runnable&java.io.Serializable)()->{}; \
        return (Comparable<String>  &  @Immutable Serializable&Cloneable)o; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object f(Object o) {
        Runnable r = (Runnable & java.io.Serializable) () -> {};
        return (Comparable<String> & @Immutable Serializable & Cloneable) o;
      }
    }
    ");
}

#[test]
fn adjacent_same_sign_unary_operators_keep_their_space() {
    // `- -x` fused to `--x` would tokenize as a decrement and change meaning.
    let source = "class A { int f(int x) { return - -x; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int f(int x) {
        return - -x;
      }
    }
    ");
}

#[test]
fn unary_sign_before_an_increment_keeps_its_space() {
    // `- --x` fused to `---x` would tokenize as `-- -x` and change meaning
    // (same for `+ ++x`).
    let source = "class A { int f(int x) { return - --x + + ++x; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int f(int x) {
        return - --x + + ++x;
      }
    }
    ");
}

#[test]
fn comments_ending_their_line_after_a_prefix_operator_stay_and_indent_the_operand() {
    let source = "class A { void m() {
        boolean due = !
            // a paid order is never due
            paid;
        String name = (String) // checked above
            value;
        x = - // c
        y;
        x = -
        /** a */ /* b */
        y;
        f(~
        /* c */
        y);
        x = -/* c */ y;
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        boolean due = !
          // a paid order is never due
          paid;
        String name = (String) // checked above
          value;
        x = - // c
          y;
        x = -
          /** a */ /* b */
          y;
        f(
          ~
            /* c */
            y
        );
        x = -/* c */ y;
      }
    }
    ");
}

#[test]
fn operand_after_a_prefix_operator_comment_in_a_header_takes_its_wrapped_indent() {
    let source = "class A { void m() {
        if (! // c
        ready) { go(); }
        while ((boolean)
        // c
        ready) { go(); }
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        if (! // c
            ready) {
          go();
        }
        while ((boolean)
            // c
            ready) {
          go();
        }
      }
    }
    ");
}

#[test]
fn comments_after_a_prefix_increment_follow_the_prefix_operator_rule() {
    let source = "class A { void m() {
        ++ // c
        x;
        y = --
        /* c */ x;
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        ++ // c
          x;
        y = --/* c */ x;
      }
    }
    ");
}

#[test]
fn assignments_break_after_the_operator_before_a_prefix_increment_in_a_short_chain() {
    let source = "class A { void m() {
        counter = someObject.compute(++index);
        counter = someObject.compute(index++);
        counter = ++holder.compute(argument).count;
        counter = holder.compute(argument).count++;
    } }";
    let options = Options { line_width: crate::fixtures::line_width(40), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void m() {
        counter =
          someObject.compute(++index);
        counter = someObject.compute(
          index++
        );
        counter =
          ++holder.compute(argument).count;
        counter = holder.compute(argument)
          .count++;
      }
    }
    ");
}

#[test]
fn a_prefix_increment_keeps_a_header_and_a_parenthesized_object_layout() {
    let source = "class A { void m() {
        switch (++holder.first().second().third().count) { default -> {} }
        int v = ++(ready ? firstHolder : secondHolder).count;
        int w = (ready ? firstHolder : secondHolder).count++;
    } }";
    let options = Options { line_width: crate::fixtures::line_width(40), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void m() {
        switch (++holder
            .first()
            .second()
            .third().count) {
          default -> {}
        }
        int v = ++(
          ready ? firstHolder : secondHolder
        ).count;
        int w = (ready
          ? firstHolder
          : secondHolder
        ).count++;
      }
    }
    ");
}

#[test]
fn increments_of_names_are_simple_member_chain_arguments() {
    let source = "class A { void m() {
        result = builder.withFirstArgument(++index).withSecondArgument(index++).build().finish().now();
    } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        result = builder.withFirstArgument(++index).withSecondArgument(index++).build().finish().now();
      }
    }
    ");
}

#[test]
fn parentheses_are_preserved_verbatim() {
    let source = "class A { int f(int a, int b) { return (a + b) * (a - b); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int f(int a, int b) {
        return (a + b) * (a - b);
      }
    }
    ");
}

#[test]
fn assignment_targets_keep_their_parentheses_and_the_comments_inside() {
    let source = "class A { int f; void f(int x, int[] a) { (x) = 1; ((x)) += 2; (a[0]) = 3; \
                  (this.f) = 4; (/* a */ x /* b */) = 5; (x // c\n) = 6; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      int f;

      void f(int x, int[] a) {
        (x) = 1;
        ((x)) += 2;
        (a[0]) = 3;
        (this.f) = 4;
        (/* a */ x /* b */) = 5;
        (x // c
        ) = 6;
      }
    }
    ");
}

#[test]
fn expression_boundaries_keep_comments_local() {
    let source = r"class A {
        int[] empty = { /* empty initializer */ };
        Object created = new Foo(/* empty arguments */);
        int called = compute(/* empty call */);
        boolean matched = value instanceof String /* pattern type */ text;
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      int[] empty = {/* empty initializer */};
      Object created = new Foo(/* empty arguments */);
      int called = compute(/* empty call */);
      boolean matched = value instanceof String /* pattern type */ text;
    }
    ");
}

#[test]
fn conditional_arm_comment_is_idempotent_when_the_group_breaks() {
    let source = r"class A {
        Object f(String tableName) {
            int metadataMarkerIndex = tableName.lastIndexOf(39);
            return metadataMarkerIndex <= 0
                ? // marker not found or at the beginning of tableName
                new Object(tableName, optionalValue)
                : new Object(tableName.substring(0, metadataMarkerIndex), optionalValue);
        }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object f(String tableName) {
        int metadataMarkerIndex = tableName.lastIndexOf(39);
        return metadataMarkerIndex <= 0
          // marker not found or at the beginning of tableName
          ? new Object(tableName, optionalValue)
          : new Object(tableName.substring(0, metadataMarkerIndex), optionalValue);
      }
    }
    ");
}

#[test]
fn source_implied_token_block_comments_keep_their_side() {
    let source = r"class A {
        int binary(int a, int b) { return a /* L+ */ + /* R+ */ b; }
        void assignment(int a, int b) { a /* L= */ = /* R= */ b; }
        int unary(int a) { return /* Lpre */ - /* Rpre */ a /* Lpost */ ++; }
        boolean check(Object value) { return value /* Li */ instanceof /* Ri */ String; }
        int conditional(boolean p, int a, int b) {
            return p /* L? */ ? /* R? */ a /* L: */ : /* R: */ b;
        }
        Object member(A value) { return value /* L. */ . /* R. */ field; }
        Object reference() { return A /* L:: */ :: /* R:: */ make; }
        Object cast(Object value) { return (String /* L) */ ) /* R) */ value; }
        Object intersection(Object value) { return (A /* L& */ & /* R& */ B /* L) */ ) value; }
        Object lambda() { return x /* L-> */ -> /* R-> */ x; }
        Object creation() { return new /* Rnew */ Foo(); }
        Object qualifiedCreation(A outer) { return outer /* Lnew. */ . /* Rnew. */ new Inner(); }
        int access(int[] values, int i) {
            return values /* L[ */ [ /* R[ */ i /* L] */ ];
        }
        int[] array(int n) {
            return new /* Rarray-new */ int /* Larray[ */ [ /* Rarray[ */ n /* Larray] */ ];
        }
        Object qualifiedThis() { return A /* Lthis. */ . /* Rthis. */ this; }
        Object classLiteral() { return String /* Lclass. */ . /* Rclass. */ class; }
        int escapedOperator(int a, int b) { return a /* Lu */ \u002b /* Ru */ b; }
        Object superReference() {
            return A /* Lsuper. */ . /* Rsuper. */ super /* Lsuper:: */ :: /* Rsuper:: */ make;
        }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      int binary(int a, int b) {
        return a /* L+ */ + /* R+ */ b;
      }

      void assignment(int a, int b) {
        a /* L= */ = /* R= */ b;
      }

      int unary(int a) {
        return /* Lpre */ -/* Rpre */ a /* Lpost */++;
      }

      boolean check(Object value) {
        return value /* Li */ instanceof /* Ri */ String;
      }

      int conditional(boolean p, int a, int b) {
        return p /* L? */ ? /* R? */ a /* L: */ : /* R: */ b;
      }

      Object member(A value) {
        return value /* L. */./* R. */ field;
      }

      Object reference() {
        return A /* L:: */::/* R:: */ make;
      }

      Object cast(Object value) {
        return (String /* L) */) /* R) */ value;
      }

      Object intersection(Object value) {
        return (A /* L& */ & /* R& */ B /* L) */) value;
      }

      Object lambda() {
        return x /* L-> */ -> /* R-> */ x;
      }

      Object creation() {
        return new /* Rnew */ Foo();
      }

      Object qualifiedCreation(A outer) {
        return outer /* Lnew. */./* Rnew. */ new Inner();
      }

      int access(int[] values, int i) {
        return values /* L[ */[/* R[ */ i /* L] */];
      }

      int[] array(int n) {
        return new /* Rarray-new */ int /* Larray[ */[/* Rarray[ */ n /* Larray] */];
      }

      Object qualifiedThis() {
        return A /* Lthis. */./* Rthis. */ this;
      }

      Object classLiteral() {
        return String /* Lclass. */./* Rclass. */ class;
      }

      int escapedOperator(int a, int b) {
        return a /* Lu */ + /* Ru */ b;
      }

      Object superReference() {
        return A /* Lsuper. */./* Rsuper. */ super /* Lsuper:: */::/* Rsuper:: */ make;
      }
    }
    ");
}

#[test]
fn line_comments_around_operator_tokens_end_their_lines() {
    let source = r"class A {
        int binary(int a, int b) { return a // L+
            + // R+
            b; }
        void assignment(int a, int b) { a // L=
            = // R=
            b; }
        int unary(int a) { return /* Lpre */ - // Rpre
            a // Lpost
            ++; }
        int postfix(int[] a) { return a[0] // Lpost
            --; }
        boolean check(Object value) { return value // Li
            instanceof // Ri
            String; }
        int conditional(boolean p, int a, int b) { return p // L?
            ? // R?
            a // L:
            : // R:
            b; }
        Object member(A value) { return value // L.
            . // R.
            field; }
        Object reference() { return A // L::
            :: // R::
            make; }
        Object cast(Object value) { return (String // L)
            ) // R)
            value; }
        Object lambda() { return x // L->
            -> // R->
            x; }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      int binary(int a, int b) {
        return a // L+
          // R+
          + b;
      }

      void assignment(int a, int b) {
        a // L=
          = // R=
          b;
      }

      int unary(int a) {
        return /* Lpre */ - // Rpre
          a++; // Lpost
      }

      int postfix(int[] a) {
        return a[0]--; // Lpost
      }

      boolean check(Object value) {
        return value // Li
          // Ri
          instanceof String;
      }

      int conditional(boolean p, int a, int b) {
        return p // L?
          // R?
          ? a // L:
          // R:
          : b;
      }

      Object member(A value) {
        return value // L.
          // R.
          .field;
      }

      Object reference() {
        return A // L::
          // R::
          ::make;
      }

      Object cast(Object value) {
        return (
          String // L)
        ) // R)
          value;
      }

      Object lambda() {
        return x ->
          // L->
          // R->
          x;
      }
    }
    ");
}

#[test]
fn line_comments_after_an_operator_ending_its_line_trail_the_left_operand() {
    let source = r#"class A {
        String s = "a"
            + // first
            "b"
            + // second
            "c";
    }"#;
    assert_snapshot!(fmt(source), @r#"
    class A {
      String s =
        "a" // first
        + "b" // second
        + "c";
    }
    "#);
}

#[test]
fn before_binary_operator_comment_spaces_leave_no_blank_line() {
    let source = "class A { int m() { return aaaa // c  \n + bbbb; } }";
    let expected = "class A {\n  int m() {\n    return aaaa // c\n      + bbbb;\n  }\n}\n";
    for javadoc in [None, Some(JavadocOptions::default())] {
        assert_eq!(fmt_with_options(source, &Options { javadoc, ..Options::default() }), expected);
    }
}

#[test]
fn before_conditional_operator_comment_spaces_leave_no_blank_line() {
    let source = "class A { int m() { return cond // c  \n ? x : y; } }";
    let expected = "class A {\n  int m() {\n    return cond // c\n      ? x\n      : y;\n  }\n}\n";
    for javadoc in [None, Some(JavadocOptions::default())] {
        assert_eq!(fmt_with_options(source, &Options { javadoc, ..Options::default() }), expected);
    }
}

#[test]
fn before_member_chain_comment_spaces_leave_no_blank_line() {
    let source = "class A { Object m() { return foo.bar() // c  \n .baz(); } }";
    let expected =
        "class A {\n  Object m() {\n    return foo\n      .bar() // c\n      .baz();\n  }\n}\n";
    for javadoc in [None, Some(JavadocOptions::default())] {
        assert_eq!(fmt_with_options(source, &Options { javadoc, ..Options::default() }), expected);
    }
}

#[test]
fn compound_token_gap_block_comments_keep_every_internal_side() {
    let source = r"class A {
        Object qualified(A outer) {
            return outer /* before dot */ . /* after dot */ new /* after new */ Inner();
        }
        Object genericQualified(A outer) {
            return outer /* generic before dot */ . /* generic after dot */ new /* generic after new */ < /* generic after open */ String> Inner();
        }
        Object call(A receiver) {
            return receiver /* before dot */ . /* after dot */ < /* after open */ String>method();
        }
        Object reference() {
            return A /* before ref */ :: /* after ref */ < /* after ref open */ String>method;
        }
        int choose(int value) {
            return switch (value /* before close */) /* before brace */ {
                /* after brace */ default -> 1;
            };
        }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object qualified(A outer) {
        return outer /* before dot */./* after dot */ new /* after new */ Inner();
      }

      Object genericQualified(A outer) {
        return outer /* generic before dot */./* generic after dot */ new /* generic after new */ <
          /* generic after open */ String
        > Inner();
      }

      Object call(A receiver) {
        return receiver /* before dot */
          /* after dot */ .</* after open */ String>method();
      }

      Object reference() {
        return A /* before ref */::/* after ref */ </* after ref open */ String>method;
      }

      int choose(int value) {
        return switch (value /* before close */) /* before brace */ {
          /* after brace */ default -> 1;
        };
      }
    }
    ");
}

#[test]
fn compound_token_gap_line_comments_keep_every_internal_side() {
    let source = r"class A {
        Object qualified(A outer) { return outer // before dot
            . // after dot
            new // after new
            Inner(); }
        Object genericQualified(A outer) { return outer // generic before dot
            . // generic after dot
            new // generic after new
            < // generic after open
            String> Inner(); }
        Object call(A receiver) { return receiver // before dot
            . // after dot
            < // after open
            String>method(); }
        Object reference() { return A // before ref
            :: // after ref
            < // after ref open
            String>method; }
        int choose(int value) { return switch (value // before close
            ) // before brace
            { // after brace
            default -> 1; }; }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object qualified(A outer) {
        return outer // before dot
        // after dot
        .new // after new
        Inner();
      }

      Object genericQualified(A outer) {
        return outer // generic before dot
        // generic after dot
        .new // generic after new
        <
          // generic after open
          String
        > Inner();
      }

      Object call(A receiver) {
        return receiver // before dot
          // after dot
          .<
            // after open
            String
          >method();
      }

      Object reference() {
        return A // before ref
          // after ref
          ::<
            // after ref open
            String
          >method;
      }

      int choose(int value) {
        return switch (
          value // before close
        ) { // before brace
          // after brace
          default -> 1;
        };
      }
    }
    ");
}

#[test]
fn compound_token_gap_unicode_escapes_use_parser_boundaries() {
    let source = r"class A {
        Object qualified(A outer) {
            return outer\u002e /* after escaped dot */ n\u0065w /* after escaped new */ Inner();
        }
        Object genericQualified(A outer) {
            return outer\u002e /* generic after escaped dot */ n\u0065w /* generic after escaped new */ \u003c /* generic after escaped open */ String> Inner();
        }
        Object call(A receiver) {
            return receiver\u002e /* after escaped dot */ \u003c /* after escaped open */ String>method();
        }
        Object reference() {
            return A\u003a\u003a /* after escaped ref */ \u003c /* after escaped ref open */ String>method;
        }
        int choose(int value) {
            return switch \u0028value /* before escaped close */\u0029 /* before escaped brace */ \u007b /* after escaped brace */ default -> 1; };
        }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object qualified(A outer) {
        return outer./* after escaped dot */ new /* after escaped new */ Inner();
      }

      Object genericQualified(A outer) {
        return outer./* generic after escaped dot */ new /* generic after escaped new */ <
          /* generic after escaped open */ String
        > Inner();
      }

      Object call(A receiver) {
        return receiver
          /* after escaped dot */ .</* after escaped open */ String>method();
      }

      Object reference() {
        return A::/* after escaped ref */ </* after escaped ref open */ String>method;
      }

      int choose(int value) {
        return switch (value /* before escaped close */) /* before escaped brace */ {
          /* after escaped brace */ default -> 1;
        };
      }
    }
    ");
}

#[test]
fn binary_source_gap_comments_survive_flattened_and_nested_chains() {
    let source = r"class A {
        boolean f() {
            return firstExtremelyLongOperandName /* L1 */ + /* R1 */ secondExtremelyLongOperandName
                /* L2 */ + /* R2 */ thirdExtremelyLongOperandName * /* nested */ fourthOperand > 0;
        }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      boolean f() {
        return firstExtremelyLongOperandName /* L1 */
            + /* R1 */ secondExtremelyLongOperandName
            + /* L2 */ /* R2 */ thirdExtremelyLongOperandName * /* nested */ fourthOperand
          > 0;
      }
    }
    ");
}

#[test]
fn field_initializer_expressions_wrap_through_the_flat_ir() {
    let source = r"class A {
        String chained = someRatherLongReceiverName.firstSelector().secondSelector(alpha, beta).thirdSelector();
        int conditional = predicateValueName ? someVeryLongConsequentExpression : someVeryLongAlternative;
    }";
    let options = Options { line_width: crate::fixtures::line_width(98), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      String chained = someRatherLongReceiverName
        .firstSelector()
        .secondSelector(alpha, beta)
        .thirdSelector();
      int conditional = predicateValueName
        ? someVeryLongConsequentExpression
        : someVeryLongAlternative;
    }
    ");
}

#[test]
fn comments_around_a_conditional_operator_trail_the_operand_or_keep_their_line() {
    let source = r"class A {
        int sameLine(boolean p, int a, int b) { return p ? // after question
            a : b; }
        int endsLine(boolean p, int a, int b) { return p ? a : /* ends line */
            b; }
        int leadsBranch(boolean p, int a, int b) { return p
            /* leads a */ ? a : b; }
        int ownLine(boolean p, int a, int b) { return p

            // keeps its line
            ? a : b; }
        int afterOperator(boolean p, int a, int b) { return p ? a
            : // after colon
            b; }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      int sameLine(boolean p, int a, int b) {
        return p // after question
          ? a
          : b;
      }

      int endsLine(boolean p, int a, int b) {
        return p ? a /* ends line */ : b;
      }

      int leadsBranch(boolean p, int a, int b) {
        return p ? /* leads a */ a : b;
      }

      int ownLine(boolean p, int a, int b) {
        return p
          // keeps its line
          ? a
          : b;
      }

      int afterOperator(boolean p, int a, int b) {
        return p
          ? a
          // after colon
          : b;
      }
    }
    ");
}

#[test]
fn own_line_comments_print_before_the_conditional_operators() {
    let source = r"class Order {
  int total = ready

      // cached
      ? cached :
      // computed
      compute();
}";
    assert_snapshot!(fmt(source), @"
    class Order {
      int total = ready
        // cached
        ? cached
        // computed
        : compute();
    }
    ");
}

#[test]
fn an_own_line_comment_before_a_call_dot_stays_there() {
    let source = "class Order { void f() {
this.orders()
.map(Order::total)

// Skip refunds.
.filter(total -> total > 0).count();
} }";
    assert_snapshot!(fmt(source), @"
    class Order {
      void f() {
        this.orders()
          .map(Order::total)

          // Skip refunds.
          .filter(total -> total > 0)
          .count();
      }
    }
    ");
}

#[test]
fn a_line_comment_after_a_members_object_breaks_its_assignment_after_the_operator() {
    let source = "class Order { void f() {
int timeout = config.defaults() // read at startup
.timeout;
Supplier<Order> next = queue // oldest first
::poll;
} }";
    assert_snapshot!(format_stable(source), @"
    class Order {
      void f() {
        int timeout =
          config.defaults() // read at startup
            .timeout;
        Supplier<Order> next =
          queue // oldest first
            ::poll;
      }
    }
    ");
}

#[test]
fn comments_on_both_sides_of_a_member_dot_keep_their_lines() {
    for (source, expected) in [
        ("a // c\n. /* d */\nb", "a // c\n      /* d */\n      .b"),
        ("a // c\n. /* d */ b", "a // c\n      ./* d */ b"),
        ("a. // c\n/* d */ b", "a // c\n      ./* d */ b"),
        ("a. /* c */\n/* d */ b", "a /* c */./* d */ b"),
        ("a. /* c */\n// d\nb", "a /* c */\n      // d\n      .b"),
        ("a\n. /* c */\nb", "a /* c */.b"),
        ("a\n. // c\nb", "a // c\n      .b"),
        ("a /* c */\n. /* d */\nb", "a /* c */\n      /* d */\n      .b"),
        ("a /* c\n */\n. // d\nb", "a /* c\n              */\n      // d\n      .b"),
        ("a.\n// c\n/* d\n * e */ b", "a\n      // c\n      /* d\n       * e */ .b"),
        ("a\n/* c */ . /* d */\nb", "a\n      /* c */\n      /* d */\n      .b"),
        ("a\n/* c */ .\n/* d */\nb", "a\n      /* c */\n      /* d */\n      .b"),
        ("a.\n\n// c\n\n// d\nb", "a\n      // c\n\n      // d\n      .b"),
        ("a. /* c */\n\n// d\nb", "a /* c */\n      // d\n      .b"),
        ("a // c\n.\n\n// d\nb", "a // c\n      // d\n      .b"),
        ("a // c\n\n// d\n.b", "a // c\n      // d\n      .b"),
    ] {
        let source = format!("class A {{ Object f() {{ return {source}; }} }}");
        let expected = format!("class A {{\n  Object f() {{\n    return {expected};\n  }}\n}}\n");
        assert_eq!(fmt(&source), expected, "{source:?}");
        check_contract_at_widths_and_modes(&source);
    }
}

#[test]
fn comments_around_a_member_dot_keep_their_side_or_move_in_front_of_it() {
    let source = r"class A {
        Object beforeDot(A value) { return value // ends its line
            .field; }
        Object afterDot(A value) { return value. // after the dot
            field; }
        Object blockAfterDot(A value) { return value. /* ends its line */
            field; }
        Object ownLineAfterDot(A value) { return value.
            // on its own line
            field; }
        Object ownLine(A value) { return value.call()

            // keeps its line
            .next(); }
        Object inline(A value) { return value /* before */ . /* after */ field; }
        Object name() { return java.util. // after the dot
            List.of(); }
        java. /* a */ // b
        util.List<String> type;
        Object index(int[] values) { return values
            // before the bracket
            [0]; }
        Object reference() { return A // before the separator
            ::method; }
        List
        /* before the opener */
        <String> generic;
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      Object beforeDot(A value) {
        return value // ends its line
          .field;
      }

      Object afterDot(A value) {
        return value // after the dot
          .field;
      }

      Object blockAfterDot(A value) {
        return value /* ends its line */.field;
      }

      Object ownLineAfterDot(A value) {
        return value
          // on its own line
          .field;
      }

      Object ownLine(A value) {
        return value
          .call()

          // keeps its line
          .next();
      }

      Object inline(A value) {
        return value /* before */./* after */ field;
      }

      Object name() {
        return java.util // after the dot
          .List.of();
      }

      java /* a */ // b
      .util.List<String> type;

      Object index(int[] values) {
        return values
        // before the bracket
        [0];
      }

      Object reference() {
        return A // before the separator
          ::method;
      }

      List
        /* before the opener */
        <String> generic;
    }
    ");
}

#[test]
fn comments_around_a_binary_operator_trail_the_left_operand_or_hoist_above_the_operator() {
    let source = r"class A {
        int sameLine(int a, int b) { return a + // after plus
            b; }
        int ownLine(int a, int b) { return a +
            // own line
            b; }
        int leadsRight(int a, int b) { return a
            /* leads b */ + b; }
        int endsLine(int a, int b) { return a + /* ends line */
            b; }
        int bothSides(int a, int b) { return a // before
            + // after
            b; }
        int acrossOperator(int a, int b) { return a
            // above
            +
            // below
            b; }
        boolean test(Object value) { return value instanceof // after keyword
            String; }
        boolean pattern(Object value) { return value instanceof
            // own line
            Point p; }
        boolean leadsType(Object value) { return value
            /* leads String */ instanceof String; }
    }";
    assert_snapshot!(fmt(source), @"
    class A {
      int sameLine(int a, int b) {
        return a // after plus
          + b;
      }

      int ownLine(int a, int b) {
        return a
          // own line
          + b;
      }

      int leadsRight(int a, int b) {
        return a + /* leads b */ b;
      }

      int endsLine(int a, int b) {
        return a /* ends line */ + b;
      }

      int bothSides(int a, int b) {
        return a // before
          // after
          + b;
      }

      int acrossOperator(int a, int b) {
        return a
          // above
          // below
          + b;
      }

      boolean test(Object value) {
        return value // after keyword
          instanceof String;
      }

      boolean pattern(Object value) {
        return value
          // own line
          instanceof Point p;
      }

      boolean leadsType(Object value) {
        return value instanceof /* leads String */ String;
      }
    }
    ");
}

#[test]
fn comments_before_a_call_or_creation_list_stay_before_its_delimiter() {
    let source = "class A { void m() { foo /* a */ (x); foo /* b */ (); \
                  this.<T> /* c */ foo(x); Foo.<T> /* d */ bar(); a.b() /* e */ .c /* f */ (y); \
                  new Foo /* h */ <> /* i */ (); \
                  Object r = Foo::<T> /* j */ bar; \
                  if (o instanceof Point /* k */ (int x, int y)) {} } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        foo /* a */(x);
        foo /* b */();
        this.<T>/* c */ foo(x);
        Foo.<T>/* d */ bar();
        a.b() /* e */
          .c /* f */(y);
        new Foo /* h */<> /* i */();
        Object r = Foo::<T>/* j */ bar;
        if (o instanceof Point /* k */(int x, int y)) {}
      }
    }
    ");
}

#[test]
fn line_comments_before_a_call_list_move_inside_it() {
    let source = "class A { void m() { foo // a\n(x); a.b // b\n(y).c(z); \
                  a.<T> /* t */ d // d\n(); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        foo( // a
          x
        );
        a.b( // b
          y
        ).c(z);
        a.<T>/* t */ d( // d
        );
      }
    }
    ");
}

#[test]
fn comments_before_a_creation_list_lead_its_first_argument_or_trail_the_class() {
    let source = "class A { void m() { new Foo /* g */ (x); new Foo\n/* c */\n(x); \
                  new B\n/* c */ (2); new Foo /* inside */ (); new Foo // c\n(); \
                  o.new Inner /* b */ (x); new Foo<> /* d */ (x); new Foo<> // e\n(x); \
                  o.new Inner<> /* f */ (x); } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        new Foo(/* g */ x);
        new Foo(
          /* c */
          x
        );
        new B(
          /* c */ 2
        );
        new Foo /* inside */();
        new Foo(); // c
        o.new Inner(/* b */ x);
        new Foo<>(/* d */ x);
        new Foo<>(x); // e
        o.new Inner<>(/* f */ x);
      }
    }
    ");
}

#[test]
fn creation_type_and_body_boundary_comments_stay_outside_arguments_and_bodies() {
    let source = "class A { void m() { new <T> /* type */ Foo(a); new Foo(a) /* body */ {}; new Foo(/* inside */) /* empty body */ {}; } }";
    assert_snapshot!(fmt(source), @"
    class A {
      void m() {
        new <T> /* type */ Foo(a);
        new Foo(a) /* body */ {};
        new Foo(/* inside */) /* empty body */ {};
      }
    }
    ");
}
