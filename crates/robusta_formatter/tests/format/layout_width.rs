//! Line breaking, display width, and compact layout.

use insta::assert_snapshot;
use robusta_formatter::Options;

use crate::fixtures::{
    format_source as fmt, format_source_with_options as fmt_with_options, format_stable,
    format_stable_with_options, indent_width, line_width,
};

#[test]
fn enhanced_for_wraps_with_its_colon_before_a_line_comment() {
    let source = "class A { void f() { for (String value /* before */ : /* after */ allValuesFromRequest) {} \
                  for (String value // before\n: // after\nallValuesFromRequest) {} } }";
    let options = Options { line_width: line_width(56), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        for (String value /* before */ :
            /* after */ allValuesFromRequest) {}
        for (String value : // before
            // after
            allValuesFromRequest) {}
      }
    }
    ");
}

#[test]
fn multi_catch_wraps_around_union_comments() {
    let source = "class A { void f() { try {} catch (FirstException /* before */ | /* after */ SecondException e) {} \
                  try {} catch (FirstException // before\n| // after\nSecondException e) {} } }";
    let options = Options { line_width: line_width(64), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        try {} catch (FirstException /* before */
            | /* after */ SecondException e) {}
        try {} catch (FirstException // before
            // after
            | SecondException e) {}
      }
    }
    ");
}

#[test]
fn long_array_initializers_break_one_per_line() {
    let source = "class C { String[] names = {firstExtremelyLongElementValueName, \
                  secondExtremelyLongElementValueName, thirdExtremelyLongElementValueName}; }";
    assert_snapshot!(fmt(source), @"
    class C {
      String[] names = {
        firstExtremelyLongElementValueName,
        secondExtremelyLongElementValueName,
        thirdExtremelyLongElementValueName,
      };
    }
    ");
}

#[test]
fn long_array_initializers_keep_trailing_commas_when_broken() {
    let source = "class C { String[] names = {firstExtremelyLongElementValueName, \
                  secondExtremelyLongElementValueName, thirdExtremelyLongElementValueName,}; }";
    assert_snapshot!(fmt(source), @"
    class C {
      String[] names = {
        firstExtremelyLongElementValueName,
        secondExtremelyLongElementValueName,
        thirdExtremelyLongElementValueName,
      };
    }
    ");
}

#[test]
fn blank_lines_between_array_elements_break_the_list() {
    let source = "@A({first,\n\nsecond}) class C { String[] names = {first,\n\nsecond}; \
                  int[] numbers = {1, 2,\n\n\n3, 4}; \
                  String[] commented = {first,\n\n// c\nsecond}; }";
    assert_snapshot!(fmt(source), @"
    @A({
      first,

      second,
    })
    class C {
      String[] names = {
        first,

        second,
      };
      int[] numbers = {
        1, 2,

        3, 4,
      };
      String[] commented = {
        first,

        // c
        second,
      };
    }
    ");
}

#[test]
fn long_creation_argument_lists_break_one_per_line() {
    let source = "class C { void f() { cache = new ReplacementCache(firstExtremelyLongArgumentName, \
                  secondExtremelyLongArgumentName, thirdExtremelyLongArgumentName); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        cache = new ReplacementCache(
          firstExtremelyLongArgumentName,
          secondExtremelyLongArgumentName,
          thirdExtremelyLongArgumentName
        );
      }
    }
    ");
}

#[test]
fn long_argument_lists_break_one_per_line() {
    let source = "class C { void f() { dispatcher.submit(firstExtremelyLongArgumentName, \
                  secondExtremelyLongArgumentName, thirdExtremelyLongArgumentName); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        dispatcher.submit(
          firstExtremelyLongArgumentName,
          secondExtremelyLongArgumentName,
          thirdExtremelyLongArgumentName
        );
      }
    }
    ");
}

#[test]
fn blank_lines_between_arguments_break_the_list() {
    let source = "class C { void f() { call(first,\n\nsecond, third); \
                  call(first // after\n\n, second, x -> { return x; }); \
                  call(first, // after\n\nsecond, third); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        call(
          first,

          second,
          third
        );
        call(
          first, // after
          second,
          x -> {
            return x;
          }
        );
        call(
          first, // after

          second,
          third
        );
      }
    }
    ");
}

#[test]
fn only_a_blank_line_after_the_comma_breaks_the_arguments() {
    let source = r#"class C { void f() {
List<String> regions = List.of(
  "north",
  "south"

  ,

  "east",
  "west"
);
List<String> zones = List.of(
  "inner"

  , "outer"
);
} }"#;
    assert_snapshot!(fmt(source), @r#"
    class C {
      void f() {
        List<String> regions = List.of(
          "north",
          "south",

          "east",
          "west"
        );
        List<String> zones = List.of("inner", "outer");
      }
    }
    "#);
}

#[test]
fn line_breaks_before_an_argument_comma_keep_the_list_together() {
    let source = "class C { void f() { call(first\n,\nsecond); \
                  call(first /* a */\n\n, second, x -> { return x; }); } \
                  @A(x = 1\n,\ny = 2) void g() {} }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        call(first, second);
        call(first /* a */, second, x -> {
          return x;
        });
      }

      @A(x = 1, y = 2)
      void g() {}
    }
    ");
}

#[test]
fn comments_between_arguments_keep_their_lines_around_a_blank_line() {
    let source = "class C { void f() { call(first,\n\n// leads second\nsecond); \
                  call(first,\n// leads second\n\nsecond); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        call(
          first,

          // leads second
          second
        );
        call(
          first,
          // leads second

          second
        );
      }
    }
    ");
}

#[test]
fn long_binary_chains_break_before_operators() {
    let source = "class C { boolean f() { return firstExtremelyLongConditionFlag && \
                  secondExtremelyLongConditionFlag && thirdExtremelyLongConditionFlag; } }";
    assert_snapshot!(fmt(source), @"
    class C {
      boolean f() {
        return firstExtremelyLongConditionFlag
          && secondExtremelyLongConditionFlag
          && thirdExtremelyLongConditionFlag;
      }
    }
    ");
}

#[test]
fn short_member_chains_stay_flat() {
    let source = "class C { String f(User user) { return user.profile().displayName(); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      String f(User user) {
        return user.profile().displayName();
      }
    }
    ");
}

#[test]
fn long_member_chains_break_before_selectors() {
    let source = "class C { String f(Source source) { return source.firstExtremelyLongSelectorName()\
                  .secondExtremelyLongSelectorName().thirdExtremelyLongSelectorName(); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      String f(Source source) {
        return source
          .firstExtremelyLongSelectorName()
          .secondExtremelyLongSelectorName()
          .thirdExtremelyLongSelectorName();
      }
    }
    ");
}

#[test]
fn member_accesses_before_the_first_call_stay_with_the_head() {
    let source = "class C { void f() { this.aaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbbbbbbbbbbb\
                  .ccccccccccccccccc(argumentOne).dddddddddddddddd(argumentTwo)\
                  .eeeeeeeeeeeeeee(argumentThree); \
                  values[0].aaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbbbbbbbbbbb\
                  .ccccccccccccccccc(argumentOne).dddddddddddddddd(argumentTwo); \
                  value = this.aaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
                  .cccccccccccccccccc(argumentOne); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        this.aaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbbbbbbbbbbb
          .ccccccccccccccccc(argumentOne)
          .dddddddddddddddd(argumentTwo)
          .eeeeeeeeeeeeeee(argumentThree);
        values[0].aaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbbbbbbbbbbb
          .ccccccccccccccccc(argumentOne)
          .dddddddddddddddd(argumentTwo);
        value =
          this.aaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.cccccccccccccccccc(
            argumentOne
          );
      }
    }
    ");
}

#[test]
fn long_stream_chains_keep_entry_call_with_receiver() {
    let source = "class C { DriverPropertyInfo[] f(Properties properties) { return allProperties().stream()\
                  .filter(property -> property.isValid(properties))\
                  .map(property -> getDriverPropertyInfo(property, properties))\
                  .toArray(DriverPropertyInfo[]::new); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      DriverPropertyInfo[] f(Properties properties) {
        return allProperties()
          .stream()
          .filter(property -> property.isValid(properties))
          .map(property -> getDriverPropertyInfo(property, properties))
          .toArray(DriverPropertyInfo[]::new);
      }
    }
    ");
}

#[test]
fn argument_bearing_chain_entry_breaks_before_selector() {
    let source = "class C { DateTimeFormatter f() { return new DateTimeFormatterBuilder()\
                  .append(ISO_LOCAL_DATE).appendLiteral(' ')\
                  .append(ISO_LOCAL_TIME).toFormatter(); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      DateTimeFormatter f() {
        return new DateTimeFormatterBuilder()
          .append(ISO_LOCAL_DATE)
          .appendLiteral(' ')
          .append(ISO_LOCAL_TIME)
          .toFormatter();
      }
    }
    ");
}

#[test]
fn long_conditionals_break_before_question_and_colon() {
    let source = "class C { String f() { return extremelyLongConditionExpressionFlag ? \
                  firstExtremelyLongResultValue : secondExtremelyLongResultValue; } }";
    assert_snapshot!(fmt(source), @"
    class C {
      String f() {
        return extremelyLongConditionExpressionFlag
          ? firstExtremelyLongResultValue
          : secondExtremelyLongResultValue;
      }
    }
    ");
}

#[test]
fn conditional_branches_align_past_their_operators_after_an_unindented_test() {
    let source = "class C { int f() { \
                  int h = firstExtremelyLongOperandName + secondExtremelyLongOperandName \
                  + thirdExtremelyLongOperandNameXX > 0 ? c : d; \
                  int i = c ? d : firstExtremelyLongOperandName + secondExtremelyLongOperandName \
                  + thirdExtremelyLongOperandNameXX > 0; \
                  call(firstExtremelyLongOperandName + secondExtremelyLongOperandName \
                  + thirdExtremelyLongOperandNameXX > 0 ? c : d); \
                  call((firstExtremelyLongOperandName > 0 || secondExtremelyLongOperandName > 0 \
                  || thirdOperandNameXXXX > 0) ? c : d); \
                  j = cond ? someObject.firstCall(argumentNumberOne, argumentNumberTwo, \
                  argumentNumberThreeXXXXXXXXXXXXXXX) : d; \
                  k = outerConditionName ? consequentName : nestedAlternateFirstOperandNameXXXXXXXXX \
                  + secondOperandNameXXXXXXXXXXXXX + thirdOperandNameXXXXXXXXXXXXXXXXXXXXX \
                  ? consequentOfNestedAlternate : alternateOfNestedAlternate; \
                  l = firstExtremelyLongConditionName ? secondConditionNameXXXXXXXXXXXXXXXX \
                  ? secondConsequentNameXXXXXXXXXXX : secondAlternateName : firstExtremelyLongAlternateName; \
                  m = (firstExtremelyLongOperandName > 0 || secondExtremelyLongOperandName > 0) \
                  ? firstConsequentName : secondAlternateName; \
                  n = (firstExtremelyLongOperandName > 0 && secondExtremelyLongOperandName > 0 \
                  && thirdExtremelyLongOperandNameXX > 0) ? c : d; \
                  o = (isBinary ? argType.erasure().isEquivalentTo(parameters[i].erasure()) \
                  : argType.isEquivalentTo(parameters[i])) ? ACCURATE_MATCH : IMPOSSIBLE_MATCH; \
                  return firstExtremelyLongOperandName + secondExtremelyLongOperandName \
                  + thirdExtremelyLongOperandNameXX > 0 ? c : d; } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      int f() {
        int h =
          firstExtremelyLongOperandName
            + secondExtremelyLongOperandName
            + thirdExtremelyLongOperandNameXX
          > 0
            ? c
            : d;
        int i = c
          ? d
          : firstExtremelyLongOperandName
              + secondExtremelyLongOperandName
              + thirdExtremelyLongOperandNameXX
            > 0;
        call(
          firstExtremelyLongOperandName
            + secondExtremelyLongOperandName
            + thirdExtremelyLongOperandNameXX
            > 0
            ? c
            : d
        );
        call(
          (
            firstExtremelyLongOperandName > 0
              || secondExtremelyLongOperandName > 0
              || thirdOperandNameXXXX > 0
          )
            ? c
            : d
        );
        j = cond
          ? someObject.firstCall(
              argumentNumberOne,
              argumentNumberTwo,
              argumentNumberThreeXXXXXXXXXXXXXXX
            )
          : d;
        k = outerConditionName
          ? consequentName
          : nestedAlternateFirstOperandNameXXXXXXXXX
              + secondOperandNameXXXXXXXXXXXXX
              + thirdOperandNameXXXXXXXXXXXXXXXXXXXXX
            ? consequentOfNestedAlternate
            : alternateOfNestedAlternate;
        l = firstExtremelyLongConditionName
          ? secondConditionNameXXXXXXXXXXXXXXXX
            ? secondConsequentNameXXXXXXXXXXX
            : secondAlternateName
          : firstExtremelyLongAlternateName;
        m = (firstExtremelyLongOperandName > 0 || secondExtremelyLongOperandName > 0)
          ? firstConsequentName
          : secondAlternateName;
        n = (
          firstExtremelyLongOperandName > 0
          && secondExtremelyLongOperandName > 0
          && thirdExtremelyLongOperandNameXX > 0
        )
          ? c
          : d;
        o = (
          isBinary
            ? argType.erasure().isEquivalentTo(parameters[i].erasure())
            : argType.isEquivalentTo(parameters[i])
        )
          ? ACCURATE_MATCH
          : IMPOSSIBLE_MATCH;
        return firstExtremelyLongOperandName
          + secondExtremelyLongOperandName
          + thirdExtremelyLongOperandNameXX
          > 0
          ? c
          : d;
      }
    }
    ");
    let options = Options { indent_width: indent_width(4), ..Options::default() };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class C {
        int f() {
            int h =
                firstExtremelyLongOperandName
                    + secondExtremelyLongOperandName
                    + thirdExtremelyLongOperandNameXX
                > 0
                    ? c
                    : d;
            int i = c
                ? d
                : firstExtremelyLongOperandName
                      + secondExtremelyLongOperandName
                      + thirdExtremelyLongOperandNameXX
                  > 0;
            call(
                firstExtremelyLongOperandName
                    + secondExtremelyLongOperandName
                    + thirdExtremelyLongOperandNameXX
                    > 0
                    ? c
                    : d
            );
            call(
                (
                    firstExtremelyLongOperandName > 0
                        || secondExtremelyLongOperandName > 0
                        || thirdOperandNameXXXX > 0
                )
                    ? c
                    : d
            );
            j = cond
                ? someObject.firstCall(
                      argumentNumberOne,
                      argumentNumberTwo,
                      argumentNumberThreeXXXXXXXXXXXXXXX
                  )
                : d;
            k = outerConditionName
                ? consequentName
                : nestedAlternateFirstOperandNameXXXXXXXXX
                    + secondOperandNameXXXXXXXXXXXXX
                    + thirdOperandNameXXXXXXXXXXXXXXXXXXXXX
                  ? consequentOfNestedAlternate
                  : alternateOfNestedAlternate;
            l = firstExtremelyLongConditionName
                ? secondConditionNameXXXXXXXXXXXXXXXX
                    ? secondConsequentNameXXXXXXXXXXX
                    : secondAlternateName
                : firstExtremelyLongAlternateName;
            m = (firstExtremelyLongOperandName > 0 || secondExtremelyLongOperandName > 0)
                ? firstConsequentName
                : secondAlternateName;
            n = (
                firstExtremelyLongOperandName > 0
                && secondExtremelyLongOperandName > 0
                && thirdExtremelyLongOperandNameXX > 0
            )
                ? c
                : d;
            o = (
                isBinary
                    ? argType.erasure().isEquivalentTo(parameters[i].erasure())
                    : argType.isEquivalentTo(parameters[i])
            )
                ? ACCURATE_MATCH
                : IMPOSSIBLE_MATCH;
            return firstExtremelyLongOperandName
                + secondExtremelyLongOperandName
                + thirdExtremelyLongOperandNameXX
                > 0
                ? c
                : d;
        }
    }
    ");
}

#[test]
fn long_parameter_lists_break_one_per_line() {
    let source = "class C { void configure(ExtremelyLongBuilderTypeName firstParameter, \
                  AnotherExtremelyLongTypeName secondParameter) {} }";
    assert_snapshot!(fmt(source), @"
    class C {
      void configure(
        ExtremelyLongBuilderTypeName firstParameter,
        AnotherExtremelyLongTypeName secondParameter
      ) {}
    }
    ");
}

#[test]
fn declarator_lists_break_one_per_line() {
    // Outside a `for` header, a list with an initializer breaks even when it fits.
    let source = "class C { int firstVariable = 1, secondVariable = 2, thirdVariable = 3, \
                  fourthVariable = 4, fifthVariable = 5; int firstExtremelyLongFieldName, \
                  secondExtremelyLongFieldName, thirdExtremelyLongFieldName, fourth; int x, y; \
                  void f() { int a = 1, b = 2; int c, d = 3; \
                  for (int i = 0, n = size; i < n; i++) {} } }";
    assert_snapshot!(fmt(source), @"
    class C {
      int firstVariable = 1,
        secondVariable = 2,
        thirdVariable = 3,
        fourthVariable = 4,
        fifthVariable = 5;
      int firstExtremelyLongFieldName,
        secondExtremelyLongFieldName,
        thirdExtremelyLongFieldName,
        fourth;
      int x, y;

      void f() {
        int a = 1,
          b = 2;
        int c,
          d = 3;
        for (int i = 0, n = size; i < n; i++) {}
      }
    }
    ");
}

#[test]
fn declarators_break_from_their_type_only_when_they_do_not_fit() {
    let source = "class A { LongTypeName longVariableName; int[] longVariableNames[]; \
                  Map<String, Long> namesToCounts = counts(); int fits = 1; \
                  void f() { LongTypeName longVariableName = 1, other = 2; \
                  for (LongTypeName longVariableName = first(); ; ) {} } }";
    let options = Options { line_width: line_width(30), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      LongTypeName
        longVariableName;
      int[] longVariableNames[];
      Map<String, Long>
        namesToCounts = counts();
      int fits = 1;

      void f() {
        LongTypeName
          longVariableName = 1,
          other = 2;
        for (
          LongTypeName
            longVariableName =
              first();
          ;
        ) {}
      }
    }
    ");
}

#[test]
fn a_lone_simple_type_argument_splits_only_when_no_other_break_fits() {
    let source = "class A { public static final List<ColumnMetadata> TPCH_NATION_SCHEMA_WITH_A_LONG_NAME = ImmutableList.<ColumnMetadata>builder().add(first).add(second).build(); } \
                  class A { public static final List<ColumnMetadata> TPCH_NATION_SCHEMA_WITH_A_LONG_NAME = ImmutableList.<ColumnMetadata>builder().add(firstColumnWithAVeryLongNameForTheNationSchema).add(secondColumnWithAVeryLongNameForTheNationSchema).build(); } \
                  class A { <FirstTypeParameterNameThatIsVeryVeryVeryLong extends Comparable<FirstTypeParameterNameThatIsVeryVeryVeryLong>, SecondTypeParameterNameThatIsAlsoQuiteLongToForceIt> void f() { run(); } } \
                  class B { private static final Optional<ReplicatedOrderRepositoryWithRetryingConnectionPoolAndCachedQueryResultsProviders> cachedOrderRepositoryInstance = null; }";
    assert_snapshot!(format_stable(source), @"
    class A {
      public static final List<ColumnMetadata> TPCH_NATION_SCHEMA_WITH_A_LONG_NAME =
        ImmutableList.<ColumnMetadata>builder().add(first).add(second).build();
    }

    class A {
      public static final List<ColumnMetadata> TPCH_NATION_SCHEMA_WITH_A_LONG_NAME =
        ImmutableList.<ColumnMetadata>builder()
          .add(firstColumnWithAVeryLongNameForTheNationSchema)
          .add(secondColumnWithAVeryLongNameForTheNationSchema)
          .build();
    }

    class A {
      <
        FirstTypeParameterNameThatIsVeryVeryVeryLong extends
          Comparable<FirstTypeParameterNameThatIsVeryVeryVeryLong>,
        SecondTypeParameterNameThatIsAlsoQuiteLongToForceIt
      > void f() {
        run();
      }
    }

    class B {
      private static final Optional<
        ReplicatedOrderRepositoryWithRetryingConnectionPoolAndCachedQueryResultsProviders
      > cachedOrderRepositoryInstance = null;
    }
    ");
}

#[test]
fn comments_before_a_declarator_keep_their_lines_when_it_breaks_from_its_type() {
    let source = "class A { LongTypeName /* c */ longVariableName; \
                  LongTypeName // a comment wider than the line\n longVariableName; \
                  LongTypeName /* a comment wider than the line */\n longVariableName; \
                  LongTypeName\n// own line\nlongVariableName; \
                  LongTypeName\n/* own line */\nlongVariableName; }";
    let options = Options { line_width: line_width(30), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      LongTypeName
        /* c */ longVariableName;
      LongTypeName // a comment wider than the line
        longVariableName;
      LongTypeName /* a comment wider than the line */
        longVariableName;
      LongTypeName
        // own line
        longVariableName;
      LongTypeName
        /* own line */
        longVariableName;
    }
    ");
}

#[test]
fn names_break_from_their_type_only_when_they_do_not_fit() {
    let source = "class A { void f(LongTypeName longVariableName, int fits) { \
                  try (LongTypeName longVariableName = open()) {} \
                  for (LongTypeName longVariableName : values) {} for (int fits : values) {} \
                  try {} catch (LongTypeName longVariableName) {} \
                  Runnable r = (LongTypeName longVariableName) -> {}; } \
                  void g(LongTypeNameOfTheClass this) {} \
                  void h(LongTypeName.Inner Outer.this) {} \
                  record R(LongTypeName longVariableName) {} }";
    let options = Options { line_width: line_width(30), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f(
        LongTypeName
          longVariableName,
        int fits
      ) {
        try (
          LongTypeName
            longVariableName =
              open()
        ) {}
        for (LongTypeName
            longVariableName :
            values) {}
        for (int fits : values) {}
        try {} catch (LongTypeName
            longVariableName) {}
        Runnable r = (
          LongTypeName
            longVariableName
        ) -> {};
      }

      void g(
        LongTypeNameOfTheClass
          this
      ) {}

      void h(
        LongTypeName.Inner
          Outer.this
      ) {}

      record R(
        LongTypeName
          longVariableName
      ) {}
    }
    ");
}

#[test]
fn comments_before_a_name_keep_their_lines_when_it_breaks_from_its_type() {
    let source = "class A { void f(LongTypeName /* c */ longVariableName, \
                  LongTypeName // a comment wider than the line\n longVariableName) { \
                  try (LongTypeName /* c */ longVariableName = open()) {} \
                  for (LongTypeName // line\n longVariableName : values) {} \
                  try {} catch (LongTypeName\n// own line\nlongVariableName) {} \
                  Runnable r = (LongTypeName /* c */ longVariableName) -> {}; } }";
    let options = Options { line_width: line_width(30), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f(
        LongTypeName
          /* c */ longVariableName,
        LongTypeName // a comment wider than the line
          longVariableName
      ) {
        try (
          LongTypeName
            /* c */ longVariableName =
              open()
        ) {}
        for (LongTypeName // line
            longVariableName :
            values) {}
        try {} catch (LongTypeName
            // own line
            longVariableName) {}
        Runnable r = (
          LongTypeName
            /* c */ longVariableName
        ) -> {};
      }
    }
    ");
}

#[test]
fn method_and_element_names_break_from_their_type_only_when_they_do_not_fit() {
    // A moved method name indents twice, so it does not line up with the body.
    let source = "class A { LongReturnTypeName longMethodNameHere() {} int fits() {} \
                  <T> LongTypeName<T> longMethodName(T value) throws Failure { return null; } \
                  abstract LongTypeName longMethodName(int first, int second); \
                  @interface B { LongTypeName longElementName() default x; \
                  LongTypeName longElementName() default {first, second}; } }";
    let options = Options { line_width: line_width(30), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      LongReturnTypeName
          longMethodNameHere() {}

      int fits() {}

      <T>
          LongTypeName<T>
              longMethodName(
                T value
              ) throws Failure {
        return null;
      }

      abstract LongTypeName
          longMethodName(
            int first,
            int second
          );

      @interface B {
        LongTypeName
          longElementName() default
            x;

        LongTypeName
          longElementName() default
            {first, second};
      }
    }
    ");
}

#[test]
fn comments_before_a_method_or_element_name_keep_their_lines() {
    let source = "class A { LongTypeName /* c */ longMethodName() {} \
                  LongTypeName // a comment wider than the line\n longMethodName() {} \
                  LongTypeName\n// own line\nlongMethodName() {} \
                  int /* c */ f(int first, int second) {} \
                  @interface B { LongTypeName /* c */ longElementName(); \
                  LongTypeName // line\n longElementName() default x; } }";
    let options = Options { line_width: line_width(30), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      LongTypeName
          /* c */ longMethodName() {}

      LongTypeName // a comment wider than the line
          longMethodName() {}

      LongTypeName
          // own line
          longMethodName() {}

      int /* c */ f(
        int first,
        int second
      ) {}

      @interface B {
        LongTypeName
          /* c */ longElementName();

        LongTypeName // line
          longElementName() default
            x;
      }
    }
    ");
}

#[test]
fn signature_tail_counts_toward_the_line_width() {
    // The parameters alone fit, but ` throws … {` pushes the line past the
    // width — the signature must still break somewhere. The default clause
    // layout takes the `throws` break, which is the cheaper of the two; the
    // point here is that the tail is measured at all.
    let source = "class C { void write(String path, String data) throws \
                  java.io.IOException, java.lang.IllegalStateException {} }";
    assert_snapshot!(fmt(source), @"
    class C {
      void write(String path, String data)
          throws java.io.IOException, java.lang.IllegalStateException {}
    }
    ");
}

/// A comment between the modifiers and a method's `<` ends at that token, so
/// breaking the type parameters after it does not move the comment onto its
/// own line on the next pass.
#[test]
fn modifier_comment_stays_before_broken_method_type_parameters() {
    let source = "class C { static /* method type parameters */ <U /* method bound */ extends Number> \
                  U identity(U value) { return value; } }";
    let options = Options { line_width: line_width(60), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      static /* method type parameters */ <
        U /* method bound */ extends Number
      > U identity(U value) {
        return value;
      }
    }
    ");
}

#[test]
fn unicode_literal_width_breaks_argument_lists_by_display_column() {
    let source = "class C { void f() { call(\"界界界界界界\", value); } }";
    let options = Options { line_width: line_width(30), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class C {
      void f() {
        call(
          "界界界界界界",
          value
        );
      }
    }
    "#);
}

#[test]
#[expect(clippy::unicode_not_nfc, reason = "the regression requires decomposed combining marks")]
fn combining_comment_width_keeps_fitting_argument_list_flat() {
    let source =
        "class C { void f() { call(first, /* e\u{301}e\u{301}e\u{301}e\u{301} */ second); } }";
    let options = Options { line_width: line_width(40), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        call(first, /* éééé */ second);
      }
    }
    ");
}

#[test]
fn retained_comment_tab_uses_column_aware_indent_width_stops() {
    let source = "class C { void f() { call(a, /* xy\ty */ b); } }";
    let options =
        Options { line_width: line_width(36), indent_width: indent_width(4), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
        void f() {
            call(a, /* xy	y */ b);
        }
    }
    ");
}

#[test]
fn empty_class_collapses() {
    assert_snapshot!(fmt("class  A { }\n"), @"class A {}");
}

#[test]
fn switch_arrow_bodies_break_at_narrow_widths() {
    let source = "class C { int f(int value) { return switch (value) { \
                  case FIRST_CONSTANT -> firstExtremelyLongResultValue; \
                  case SECOND -> calculate(firstArgument, secondArgument); \
                  default -> throw new IllegalStateException(); }; } }";
    let options = Options { line_width: line_width(46), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      int f(int value) {
        return switch (value) {
          case FIRST_CONSTANT ->
            firstExtremelyLongResultValue;
          case SECOND ->
            calculate(
              firstArgument,
              secondArgument
            );
          default ->
            throw new IllegalStateException();
        };
      }
    }
    ");
}

#[test]
fn switch_pattern_lists_wrap_and_keep_block_bodies_attached() {
    let source = "class C { void f(Object value) { switch (value) { \
                  case FirstExtremelyLongType _, SecondExtremelyLongType _ -> {} \
                  default -> {} } } }";
    let options = Options { line_width: line_width(58), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f(Object value) {
        switch (value) {
          case FirstExtremelyLongType _,
            SecondExtremelyLongType _ -> {}
          default -> {}
        }
      }
    }
    ");
}

#[test]
fn switch_arrow_breaks_preserve_comment_boundaries() {
    let source = "class C { int f(int value) { return switch (value) { \
                  case FIRST -> /* value */ firstExtremelyLongResultValue; \
                  case SECOND -> // failure\nthrow new IllegalStateException(); \
                  default -> /* fallback */ { yield 0; } }; } }";
    let options = Options { line_width: line_width(56), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      int f(int value) {
        return switch (value) {
          case FIRST ->
            /* value */ firstExtremelyLongResultValue;
          case SECOND -> throw new IllegalStateException(); // failure
          default -> /* fallback */ {
            yield 0;
          }
        };
      }
    }
    ");
}

#[test]
fn literal_headed_chain_breaks_before_its_only_selector() {
    let source = "class C { String f(String fieldName) { return \
                  \"ARRAY type should be denoted by 'element' in the path; found '%s'\"\
                  .formatted(fieldName); } }";
    let options = Options { line_width: line_width(96), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class C {
      String f(String fieldName) {
        return "ARRAY type should be denoted by 'element' in the path; found '%s'"
          .formatted(fieldName);
      }
    }
    "#);
}

#[test]
fn receiver_headed_chain_keeps_its_only_selector() {
    let source = "class C { void f(Dispatcher dispatcher) { dispatcher.submit(\
                  firstExtremelyLongArgumentName, secondExtremelyLongArgumentName, thirdArgument); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f(Dispatcher dispatcher) {
        dispatcher.submit(
          firstExtremelyLongArgumentName,
          secondExtremelyLongArgumentName,
          thirdArgument
        );
      }
    }
    ");
}

#[test]
fn lambda_expression_body_takes_the_arrow_break_only_when_it_has_to() {
    let source = "class C { void f() { first = value -> value.name(); \
                  second = value -> value.qualifiedName(); } }";
    let options = Options { line_width: line_width(40), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        first = value -> value.name();
        second = value ->
          value.qualifiedName();
      }
    }
    ");
}

#[test]
fn lambda_block_body_keeps_hugging_the_arrow() {
    let source = "class C { void f() { register(registeredValue -> { counter++; }); } }";
    let options = Options { line_width: line_width(40), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        register(registeredValue -> {
          counter++;
        });
      }
    }
    ");
}

#[test]
fn lambda_array_creation_body_hugs_the_arrow() {
    let source = "class C { void f() { names.map(name -> new String[] {name.first(), \
                  name.middle(), name.last(), name.suffix(), name.title()}); \
                  supplier = () -> new int[] {100000, 200000, 300000, 400000, 500000, \
                  600000, 700000, 800000, 900000}; } }";
    let options = Options { line_width: line_width(84), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        names.map(name -> new String[] {
          name.first(),
          name.middle(),
          name.last(),
          name.suffix(),
          name.title(),
        });
        supplier = () -> new int[] {
          100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000, 900000,
        };
      }
    }
    ");
}

#[test]
fn curried_lambdas_break_after_their_arrows_as_a_chain() {
    let source = "class C { void f() { \
                  register(first -> second -> combineTheTwoValuesIntoOne(first, second, defaultThirdValueForAll)); \
                  register(first -> second -> { combine(first, second); }); \
                  register((String... first) -> second -> combine(first, second)); \
                  curried = first -> second -> third -> combineTheTwoValuesIntoOne(first, second, third, fourth); \
                  handler = firstParameterName -> secondParameterName -> thirdParameterName -> fourthParameterName -> fifth; \
                  a = b = c = first -> second -> combineTheTwoValuesIntoOne(first, second, third); } }";
    let options = Options { line_width: line_width(84), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        register(
          first -> second ->
            combineTheTwoValuesIntoOne(first, second, defaultThirdValueForAll)
        );
        register(first -> second -> {
          combine(first, second);
        });
        register(
          (String... first) ->
            second ->
              combine(first, second)
        );
        curried = first -> second -> third ->
          combineTheTwoValuesIntoOne(first, second, third, fourth);
        handler =
          firstParameterName ->
          secondParameterName ->
          thirdParameterName ->
          fourthParameterName ->
            fifth;
        a =
          b =
          c =
            first -> second ->
              combineTheTwoValuesIntoOne(first, second, third);
      }
    }
    ");
}

#[test]
fn own_line_comment_after_a_grouped_lambda_arrow_keeps_the_body_below() {
    let source = "class C { void f() { register(first -> // c\nsecond -> { combine(first, second); }); \
                  names.map(name -> // c\nnew String[] {name.first()}); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        register(first ->
          // c
          second -> {
            combine(first, second);
          }
        );
        names.map(name ->
          // c
          new String[] {name.first()}
        );
      }
    }
    ");
}

#[test]
fn line_comments_before_grouped_lambda_arrows_lead_the_body() {
    let source = "class C { void f() { \
                  handlers.put(key, first // end\n-> { combine(first); }); \
                  handlers.put(key, first -> second // end\n-> { combine(first, second); }); \
                  handlers.put(key, first -> second // middle\n-> third -> { combine(first, second, third); }); \
                  handlers.put(first // first\n-> { combine(first); }, key); \
                  handlers.put(key, first -> second /* inline */ -> { combine(first, second); }); } }";
    let options = Options { line_width: line_width(84), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        handlers.put(key, first -> { // end
          combine(first);
        });
        handlers.put(key, first -> second -> { // end
          combine(first, second);
        });
        handlers.put(key, first -> second ->
          // middle
          third -> {
            combine(first, second, third);
          }
        );
        handlers.put(first -> { // first
          combine(first);
        }, key);
        handlers.put(key, first -> second /* inline */ -> {
          combine(first, second);
        });
      }
    }
    ");
}

#[test]
fn grouped_arguments_break_inside_one_lambda_or_anonymous_class() {
    let source = r#"class C { void f() {
executor.submit(() -> { process(request); });
assertThrows(IllegalArgumentException.class, () -> { parser.parse(input); });
CompletableFuture.supplyAsync(() -> { Order order = load(id); return order.total(); }, executor);
button.addActionListener(new ActionListener() { @Override public void actionPerformed(ActionEvent event) { save(); } });
names.forEach(name -> System.out.println("Processing the request with a fairly long message " + name));
assertEquals(expectedTotalForTheDiscountedOrder, calculator.totalFor(discountedOrderWithFreeShipping), () -> { String summary = describe(order); return "Unexpected total for " + summary; });
assertAll(() -> assertEquals(1, first), () -> assertEquals(2, second));
} }"#;
    let options = Options { indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class C {
      void f() {
        executor.submit(() -> {
          process(request);
        });
        assertThrows(IllegalArgumentException.class, () -> {
          parser.parse(input);
        });
        CompletableFuture.supplyAsync(() -> {
          Order order = load(id);
          return order.total();
        }, executor);
        button.addActionListener(new ActionListener() {
          @Override
          public void actionPerformed(ActionEvent event) {
            save();
          }
        });
        names.forEach(name ->
          System.out.println("Processing the request with a fairly long message " + name)
        );
        assertEquals(
          expectedTotalForTheDiscountedOrder,
          calculator.totalFor(discountedOrderWithFreeShipping),
          () -> {
            String summary = describe(order);
            return "Unexpected total for " + summary;
          }
        );
        assertAll(
          () -> assertEquals(1, first),
          () -> assertEquals(2, second)
        );
      }
    }
    "#);
}

#[test]
fn grouped_first_argument_breaks_out_when_its_closing_line_does_not_fit() {
    let source = r#"class C { void f() {
foo(() -> { run(); }, argumentPastTheClosingLine);
foo(() -> { run(); }, executor);
foo(() -> { log("a message longer than the line"); }, executor);
foo(() -> { for (String name : names // note
) { run(); } }, executor);
if (foo(() -> { run(); // note
}, executor)) {}
foo(() -> { use("""
      text
      """); }, argumentPastTheClosingLine);
} }"#;
    let options = Options { line_width: line_width(30), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class C {
      void f() {
        foo(
          () -> {
            run();
          },
          argumentPastTheClosingLine
        );
        foo(() -> {
          run();
        }, executor);
        foo(() -> {
          log(
            "a message longer than the line"
          );
        }, executor);
        foo(() -> {
          for (String name : names // note
          ) {
            run();
          }
        }, executor);
        if (foo(() -> {
          run(); // note
        }, executor)) {}
        foo(
          () -> {
            use("""
              text
              """);
          },
          argumentPastTheClosingLine
        );
      }
    }
    "#);
}

#[test]
fn function_composition_arguments_break_even_when_they_fit() {
    let source = "class C { void f() { f(a(), g(() -> b())); compose(x -> x, y -> y); \
                  run(first, second); g(() -> b()); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        f(
          a(),
          g(() -> b())
        );
        compose(
          x -> x,
          y -> y
        );
        run(first, second);
        g(() -> b());
      }
    }
    ");
}

#[test]
fn array_creation_arguments_group_as_array_literals() {
    let source = r#"class C { void f() {
accept(key, new String[] {"first value", "second value", "third value", "fourth value"});
accept(key, new int[] {100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000});
} }"#;
    assert_snapshot!(fmt_with_options(source, &Options { line_width: line_width(60), ..Options::default() }), @r#"
    class C {
      void f() {
        accept(key, new String[] {
          "first value",
          "second value",
          "third value",
          "fourth value",
        });
        accept(
          key,
          new int[] {
            100000, 200000, 300000, 400000, 500000, 600000,
            700000, 800000,
          }
        );
      }
    }
    "#);
}

#[test]
fn cast_arguments_group_as_their_operand() {
    let source = "class C { void f() { executor.submit((Runnable) () -> { process(request); }); \
                  call((Runnable) () -> { a(); }, (Runnable) () -> { b(); }); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        executor.submit((Runnable) () -> {
          process(request);
        });
        call(
          (Runnable) () -> {
            a();
          },
          (Runnable) () -> {
            b();
          }
        );
      }
    }
    ");
}

#[test]
fn a_cast_before_a_member_breaks_inside_its_parentheses() {
    let source = "class C { void f() { if (x) { if (y) { \
                  MethodBinding[] methodBindings = \
                  ((ReferenceBinding) this.parent1.type.resolvedType).availableMethods(); \
                  Function<Integer, String> methodFunction = \
                  ((ReferenceBinding) this.parent1.type.resolvedType)::name; \
                  Object innerBinding = \
                  ((ReferenceBinding) this.parent1.type.resolvedType).new InnerBinding(); \
                  MethodBinding[] commentedBindings = \
                  ((ReferenceBinding) this.parent1.type.resolvedType // resolved\n).availableMethods(); \
                  MethodBinding[] uncastBindings = \
                  (ReferenceBinding) this.parent1.type.resolvedType.availableMethods(); \
                  int n = ((String) o).length(); } } } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        if (x) {
          if (y) {
            MethodBinding[] methodBindings = (
              (ReferenceBinding) this.parent1.type.resolvedType
            ).availableMethods();
            Function<Integer, String> methodFunction = (
              (ReferenceBinding) this.parent1.type.resolvedType
            )::name;
            Object innerBinding = (
              (ReferenceBinding) this.parent1.type.resolvedType
            ).new InnerBinding();
            MethodBinding[] commentedBindings = (
              (ReferenceBinding) this.parent1.type.resolvedType // resolved
            ).availableMethods();
            MethodBinding[] uncastBindings =
              (ReferenceBinding) this.parent1.type.resolvedType.availableMethods();
            int n = ((String) o).length();
          }
        }
      }
    }
    ");
}

#[test]
fn binary_and_conditional_member_objects_break_inside_their_parentheses() {
    let source = "class C { void f() { if (x) { \
                  int prefixLength = (\"prefix-for-the-binding-name-\" \
                  + this.parent1.type.resolvedType.name).length(); \
                  int chosenLength = (useTheResolvedType \
                  ? this.parent1.type.resolvedType : fallbackTypeName).length(); \
                  int longerLength = (\"prefix-for-the-binding-name-\" \
                  + this.parent1.type.resolvedType.name + anotherOperandNameForTheBinding).length(); \
                  boolean matches = (bindingForTheResolvedType \
                  instanceof ReferenceBindingForTheType).booleanValue(); \
                  return (useTheResolvedTypeOfTheBinding \
                  ? this.parent1.type.resolvedType.memberOfTheType : fallbackTypeName).length(); \
                  check((useTheResolvedTypeOfTheBinding \
                  ? this.parent1.type.resolvedType.member : fallbackTypeName).length()); \
                  check((useTheResolvedTypeOfTheBinding \
                  ? this.parent1.type.resolvedType.member : fallbackTypeName // fallback\n\
                  ).length()); \
                  if ((useTheResolvedTypeOfTheBinding \
                  ? this.parent1.type.resolvedType.member : fallbackTypeName).flag) {} \
                  if ((useTheResolvedTypeOfTheBinding + this.parent1.type.resolvedType.member \
                  + fallbackTypeName).flag) {} \
                  int sum = this.parent1.type.resolvedType.name.length() \
                  + fallbackTypeName.length() + anotherOperandName.length(); \
                  int n = (a + b).length(); } } }";
    assert_snapshot!(format_stable(source), @r#"
    class C {
      void f() {
        if (x) {
          int prefixLength = (
            "prefix-for-the-binding-name-" + this.parent1.type.resolvedType.name
          ).length();
          int chosenLength = (
            useTheResolvedType ? this.parent1.type.resolvedType : fallbackTypeName
          ).length();
          int longerLength = (
            "prefix-for-the-binding-name-"
            + this.parent1.type.resolvedType.name
            + anotherOperandNameForTheBinding
          ).length();
          boolean matches = (
            bindingForTheResolvedType instanceof ReferenceBindingForTheType
          ).booleanValue();
          return (
            useTheResolvedTypeOfTheBinding
              ? this.parent1.type.resolvedType.memberOfTheType
              : fallbackTypeName
          ).length();
          check(
            (useTheResolvedTypeOfTheBinding
              ? this.parent1.type.resolvedType.member
              : fallbackTypeName
            ).length()
          );
          check(
            (useTheResolvedTypeOfTheBinding
              ? this.parent1.type.resolvedType.member
              : fallbackTypeName // fallback
            ).length()
          );
          if ((useTheResolvedTypeOfTheBinding
              ? this.parent1.type.resolvedType.member
              : fallbackTypeName).flag) {}
          if ((useTheResolvedTypeOfTheBinding
              + this.parent1.type.resolvedType.member
              + fallbackTypeName).flag) {}
          int sum =
            this.parent1.type.resolvedType.name.length()
            + fallbackTypeName.length()
            + anotherOperandName.length();
          int n = (a + b).length();
        }
      }
    }
    "#);
}

#[test]
fn binary_chains_break_at_every_operator_while_one_operator_keeps_its_sides_together() {
    let source = "class C { void f() { if (x) { \
                  int chosenLength = (\"prefix-for-the-binding-name-\" \
                  + this.parent1.type.resolvedType.name + anotherVeryLong).length(); \
                  boolean matches = (bindingForTheResolvedType \
                  instanceof ReferenceBindingForTheType).booleanValue(); } \
                  int z = someMethodCall(firstArgumentIsLong, secondArgumentIsLonger, \
                  thirdArgumentXxxxxxxxxxxxx, fourthArgument) + bbbb; \
                  call(someMethodCall(firstArgumentIsLong, secondArgumentIsLonger, \
                  thirdArgumentXxxxxxxxxxxxx, fourthArgument) + bbbb + cccc); \
                  call(someMethodCall(firstArgumentIsLong, secondArgumentIsLonger, \
                  thirdArgumentXxxxxxxxxxxxx, fourthArgument) // reason\n + bbbb); \
                  call(computeTheFirstValue(firstArgumentForIt, secondArgumentForIt, \
                  thirdArgumentForItXxxxxxxx, fourthArgumentX) * (bbbb + cccc)); \
                  boolean h = c ? firstExtremelyLongOperandName + secondExtremelyLongOperandName \
                  + thirdExtremelyLongOperandNameXX > 0 : d; \
                  Supplier<Boolean> f = () -> firstExtremelyLongOperandName \
                  + secondExtremelyLongOperandName + thirdExtremelyLongOperandNameXX > 0; \
                  for (; firstExtremelyLongOperandName + secondExtremelyLongOperandName \
                  + thirdExtremelyLongOperandNameXX > 0;) {} } }";
    assert_snapshot!(format_stable(source), @r#"
    class C {
      void f() {
        if (x) {
          int chosenLength = (
            "prefix-for-the-binding-name-"
            + this.parent1.type.resolvedType.name
            + anotherVeryLong
          ).length();
          boolean matches = (
            bindingForTheResolvedType instanceof ReferenceBindingForTheType
          ).booleanValue();
        }
        int z =
          someMethodCall(
            firstArgumentIsLong,
            secondArgumentIsLonger,
            thirdArgumentXxxxxxxxxxxxx,
            fourthArgument
          ) + bbbb;
        call(
          someMethodCall(
            firstArgumentIsLong,
            secondArgumentIsLonger,
            thirdArgumentXxxxxxxxxxxxx,
            fourthArgument
          )
            + bbbb
            + cccc
        );
        call(
          someMethodCall(
            firstArgumentIsLong,
            secondArgumentIsLonger,
            thirdArgumentXxxxxxxxxxxxx,
            fourthArgument
          ) // reason
            + bbbb
        );
        call(
          computeTheFirstValue(
            firstArgumentForIt,
            secondArgumentForIt,
            thirdArgumentForItXxxxxxxx,
            fourthArgumentX
          )
            * (bbbb + cccc)
        );
        boolean h = c
          ? firstExtremelyLongOperandName
              + secondExtremelyLongOperandName
              + thirdExtremelyLongOperandNameXX
            > 0
          : d;
        Supplier<Boolean> f = () ->
          firstExtremelyLongOperandName
            + secondExtremelyLongOperandName
            + thirdExtremelyLongOperandNameXX
          > 0;
        for (
          ;
          firstExtremelyLongOperandName
            + secondExtremelyLongOperandName
            + thirdExtremelyLongOperandNameXX
          > 0;
        ) {}
      }
    }
    "#);
}

#[test]
fn a_left_operand_that_does_not_flatten_keeps_its_own_group() {
    let source = "class C { void f() { \
                  int w = aaaaaaaaaaaaaaaaaaaaaaaaaaaa * bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb / cccccccccccccccccccccccccccc; \
                  boolean e = aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa == bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb == ccccccccccccccccccccc; \
                  int s = aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa << bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb >> cccccccccccccccccccccc; \
                  int m = aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa * bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb * cccccccccccccccccccccc; } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        int w =
          aaaaaaaaaaaaaaaaaaaaaaaaaaaa * bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
          / cccccccccccccccccccccccccccc;
        boolean e =
          aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa == bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
          == ccccccccccccccccccccc;
        int s =
          aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa << bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
          >> cccccccccccccccccccccc;
        int m =
          aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
          * bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
          * cccccccccccccccccccccc;
      }
    }
    ");
}

#[test]
fn a_chains_first_call_joins_only_a_this_factory_or_short_statement_head() {
    let source = "class C { void f() { \
                  Object a = items.stream().filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).toList(); \
                  Object b = Stream.of(firstValue).filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries); \
                  Object c = new Foo().aaaaaaaaaaa().bbbbbbbbbbbbbbbbbb().cccccccccccc().dddddddddddddddd().eeeeeeeeeeeeeee(); \
                  Object d = this.entries().filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).toList(); \
                  Object e = Outer.this.filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).toList(abc); \
                  Object g = this.items.stream().filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries); \
                  Object h = Foo.Bar.of(firstValue).filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries); \
                  Object i = super.entries().filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).toList(); \
                  Object j = ((Foo) value)[0].entries().filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).toList(); \
                  Object k = (items).values.entries().filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).toList(); \
                  ab.stream().filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).collectAll(values); \
                  abc.stream().filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).collectAll(xs); } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        Object a = items
          .stream()
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .toList();
        Object b = Stream.of(firstValue)
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries);
        Object c = new Foo()
          .aaaaaaaaaaa()
          .bbbbbbbbbbbbbbbbbb()
          .cccccccccccc()
          .dddddddddddddddd()
          .eeeeeeeeeeeeeee();
        Object d = this.entries()
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .toList();
        Object e = Outer.this.filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .toList(abc);
        Object g = this.items
          .stream()
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries);
        Object h = Foo.Bar.of(firstValue)
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries);
        Object i = super
          .entries()
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .toList();
        Object j = ((Foo) value)[0]
          .entries()
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .toList();
        Object k = (items).values
          .entries()
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .toList();
        ab.stream()
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .collectAll(values);
        abc
          .stream()
          .filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .collectAll(xs);
      }
    }
    ");
}

#[test]
fn a_chain_merged_into_its_head_with_one_call_left_breaks_inside_the_arguments() {
    let source = "class C { void f() { \
                  Object a = Stream.of(firstValueForTheStreamThatIsLong).filterTheEntries(predicateForEntriesLonger); } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        Object a = Stream.of(firstValueForTheStreamThatIsLong).filterTheEntries(
          predicateForEntriesLonger
        );
      }
    }
    ");
}

#[test]
fn the_fields_between_two_calls_break_with_the_next_call() {
    let source = "class C { void f() { \
                  Object a = foo().x.y.filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).toList(abc); \
                  Object b = items.stream().x.y.filterTheEntries(predicateForEntries).mapEachEntry(mapperForEntries).f(); } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        Object a = foo()
          .x.y.filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .toList(abc);
        Object b = items
          .stream()
          .x.y.filterTheEntries(predicateForEntries)
          .mapEachEntry(mapperForEntries)
          .f();
      }
    }
    ");
}

#[test]
fn a_chain_of_more_than_two_calls_with_a_complex_argument_breaks_even_when_it_fits() {
    let source = "class C { void f() { \
                  Object a = items.map(x -> x.value()).filter(y -> y.ok()).count(); \
                  Object s = items.map(value).filter(okay).count(); } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        Object a = items
          .map(x -> x.value())
          .filter(y -> y.ok())
          .count();
        Object s = items.map(value).filter(okay).count();
      }
    }
    ");
}

#[test]
fn a_chain_breaks_when_its_head_breaks() {
    let source = "class C { void f() { \
                  Object h = new Runnable() { public void run() { work(); } }.toString().trim(); \
                  Object t = \"\"\"\n        text\n        \"\"\".strip().trim(); } }";
    assert_snapshot!(format_stable(source), @r#"
    class C {
      void f() {
        Object h = new Runnable() {
          public void run() {
            work();
          }
        }
          .toString()
          .trim();
        Object t = """
          text
          """
          .strip()
          .trim();
      }
    }
    "#);
}

#[test]
fn a_chain_breaks_when_its_last_call_breaks_after_a_call_with_a_lambda() {
    let source = "class C { void f() { \
                  Object r = items.filter(x -> x.ok()).forEach(x -> { use(x); }); \
                  Object q = items.filter(okay).forEach(x -> { use(x); }); } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        Object r = items
          .filter(x -> x.ok())
          .forEach(x -> {
            use(x);
          });
        Object q = items.filter(okay).forEach(x -> {
          use(x);
        });
      }
    }
    ");
}

#[test]
fn a_chain_breaks_when_a_group_before_the_last_breaks() {
    let source = "class C { void f() { Object u = items.forEach(x -> { use(x); }).count(); } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        Object u = items
          .forEach(x -> {
            use(x);
          })
          .count();
      }
    }
    ");
}

#[test]
fn a_comment_or_blank_line_between_members_breaks_a_chain_of_one_group() {
    let source = "class C { void f() { Object c = Stream.of(x) /* c */ .filter(y); \
                  a.b()\n\n.c(); foo.bar().baz()\n\n.qux(); d.e()\n.\nf(); } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        Object c = Stream.of(x) /* c */
          .filter(y);
        a.b()

          .c();
        foo
          .bar()
          .baz()

          .qux();
        d.e().f();
      }
    }
    ");
}

#[test]
fn an_index_after_a_call_starts_a_group_unless_it_is_an_integer() {
    let source = "class C { void f() { \
                  Object b = aaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbbbbbbbbbbbb().cccccccccccccccccccccccccccc()[iiiiiiiiiiiiii].ddddddd(); \
                  Object n = aaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbbbbbbbbbbbb().cccccccccccccccccccccccccccc()[0].dddddddddddddd(); \
                  foo()[0].b(); foo()[i].b(); } }";
    assert_snapshot!(format_stable(source), @"
    class C {
      void f() {
        Object b = aaaaaaaaaaaaaaaaaaaaaaa
          .bbbbbbbbbbbbbbbbbbbbbbbbbb()
          .cccccccccccccccccccccccccccc()
          [iiiiiiiiiiiiii].ddddddd();
        Object n = aaaaaaaaaaaaaaaaaaaaaaa
          .bbbbbbbbbbbbbbbbbbbbbbbbbb()
          .cccccccccccccccccccccccccccc()[0]
          .dddddddddddddd();
        foo()[0].b();
        foo()[i].b();
      }
    }
    ");
}

#[test]
fn chains_after_a_parenthesized_head_break_before_every_call() {
    let source = "class C { void f() { \
                  Object z = (\"prefix-for-the-binding-name-\" + this.parent1.type.resolvedType.name \
                  + anotherVeryLong).aaaaaaaaaaa().bbbbbbbbbbbbbbbbbb().cccccccccccc(); \
                  Object w = ((Foo) this.parent1.type.resolvedType.someMember.fallbackTypeNameXYfallbackTypeNameXY)\
                  .aaaaaaaaaaa().bbbbbbbbbbbbbbbbbb().cccccccccccc(); \
                  Object y = (\"prefix-binding-\" + this.parent1.type.resolvedType.name + anotherVeryLong)\
                  .aaaaaaaaaaa().bbbbbbbbbbbbbbbbbb().cccccccccccc(); \
                  Object v = ((Foo) this.parent1.type.resolvedType.someMember.fallbackTypeNameXY)\
                  .aaaaaaaaaaa().bbbbbbbbbbbbbbbbbb().cccccccccccc(); \
                  Object a = foo(argumentNumberOne, argumentNumberTwo, argumentNumberThree, argumentNumberFourX)\
                  .aaaaaaaaaaa().bbbbbbbbbbbbbbbbbb().cccccccccccc(); } }";
    assert_snapshot!(format_stable(source), @r#"
    class C {
      void f() {
        Object z = (
          "prefix-for-the-binding-name-"
          + this.parent1.type.resolvedType.name
          + anotherVeryLong
        )
          .aaaaaaaaaaa()
          .bbbbbbbbbbbbbbbbbb()
          .cccccccccccc();
        Object w = (
          (Foo) this.parent1.type.resolvedType.someMember.fallbackTypeNameXYfallbackTypeNameXY
        )
          .aaaaaaaaaaa()
          .bbbbbbbbbbbbbbbbbb()
          .cccccccccccc();
        Object y = ("prefix-binding-" + this.parent1.type.resolvedType.name + anotherVeryLong)
          .aaaaaaaaaaa()
          .bbbbbbbbbbbbbbbbbb()
          .cccccccccccc();
        Object v = ((Foo) this.parent1.type.resolvedType.someMember.fallbackTypeNameXY)
          .aaaaaaaaaaa()
          .bbbbbbbbbbbbbbbbbb()
          .cccccccccccc();
        Object a = foo(argumentNumberOne, argumentNumberTwo, argumentNumberThree, argumentNumberFourX)
          .aaaaaaaaaaa()
          .bbbbbbbbbbbbbbbbbb()
          .cccccccccccc();
      }
    }
    "#);
}

#[test]
fn comments_around_a_grouped_argument_keep_the_arguments_apart() {
    let source = "class C { void f() { allOf(anyOf(lastStatement(), lambdaParent()), \
                  // Within a throwing runnable\n(t, s) -> throwingRunnable(s)); \
                  allOf(anyOf(lastStatement(), lambdaParent()),\n\
                  // Within a throwing runnable\n(t, s) -> throwingRunnable(s)); \
                  allOf(isLast(), /* block */ (t, s) -> { run(s); }); \
                  allOf(isLast(), (t, s) -> { run(s); } /* after */); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        allOf(
          anyOf(lastStatement(), lambdaParent()), // Within a throwing runnable
          (t, s) -> throwingRunnable(s)
        );
        allOf(
          anyOf(lastStatement(), lambdaParent()),
          // Within a throwing runnable
          (t, s) -> throwingRunnable(s)
        );
        allOf(
          isLast(),
          /* block */ (t, s) -> {
            run(s);
          }
        );
        allOf(
          isLast(),
          (t, s) -> {
            run(s);
          } /* after */
        );
      }
    }
    ");
}

#[test]
fn comments_starting_their_line_before_a_grouped_argument_keep_the_arguments_apart() {
    let source = "class C { void f() { allOf(isLast(),\n// own\nnew Runnable() { public void run() {} }); \
                  allOf(isLast()\n/* own */ , new Runnable() { public void run() {} }); \
                  allOf(isLast()\n/* own */ , (t, s) -> { run(s); }); } }";
    assert_snapshot!(fmt(source), @"
    class C {
      void f() {
        allOf(
          isLast(),
          // own
          new Runnable() {
            public void run() {}
          }
        );
        allOf(
          isLast(),
          /* own */ new Runnable() {
            public void run() {}
          }
        );
        allOf(
          isLast(),
          /* own */ (t, s) -> {
            run(s);
          }
        );
      }
    }
    ");
}

#[test]
fn instanceof_moves_the_keyword_below_its_operand() {
    let source = "class C { void f() { if (candidateValue instanceof Descriptor descriptor) {} } }";
    let options = Options { line_width: line_width(56), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        if (candidateValue
            instanceof Descriptor descriptor) {}
      }
    }
    ");
}

#[test]
fn instanceof_splits_from_its_type_only_when_the_pair_does_not_fit() {
    let source = "class C { void f() { if (candidateValue instanceof QualifiedDescriptor descriptorValue) {} } }";
    let options = Options { line_width: line_width(56), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        if (candidateValue
            instanceof
            QualifiedDescriptor descriptorValue) {}
      }
    }
    ");
}

#[test]
fn instanceof_type_pattern_breaks_before_its_binding() {
    let source = "class C { void f() { if (candidateValue instanceof QualifiedDescriptorType descriptorValue) {} } }";
    let options = Options { line_width: line_width(50), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        if (candidateValue
            instanceof
            QualifiedDescriptorType
              descriptorValue) {}
      }
    }
    ");
}

#[test]
fn switch_type_pattern_breaks_before_its_binding() {
    let source = "class C { void f(Object value) { switch (value) { \
                  case QualifiedDescriptorType descriptorValue -> {} default -> {} } } }";
    let options = Options { line_width: line_width(44), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f(Object value) {
        switch (value) {
          case QualifiedDescriptorType
              descriptorValue -> {}
          default -> {}
        }
      }
    }
    ");
}

#[test]
fn switch_pattern_guard_can_break_after_the_binding() {
    let source = "class C { void f(Object value) { switch (value) { \
                  case Descriptor descriptorValue when descriptorValue.isReady() -> {} default -> {} } } }";
    let options =
        Options { line_width: line_width(60), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f(Object value) {
        switch (value) {
          case Descriptor descriptorValue
            when descriptorValue.isReady() -> {}
          default -> {}
        }
      }
    }
    ");

    let options = Options { line_width: line_width(42), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f(Object value) {
        switch (value) {
          case Descriptor descriptorValue
            when descriptorValue.isReady()
            -> {}
          default -> {}
        }
      }
    }
    ");
}

#[test]
fn record_type_pattern_breaks_before_its_binding() {
    let source = "class C { void f() { if (value instanceof Box(QualifiedDescriptorType descriptorValue)) {} } }";
    let options = Options { line_width: line_width(48), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        if (value
            instanceof
            Box(
              QualifiedDescriptorType
                descriptorValue
            )) {}
      }
    }
    ");
}

#[test]
fn type_pattern_binding_break_keeps_comments() {
    let source = "class C { void f() { if (value instanceof QualifiedDescriptorType /* binding */ descriptorValue) {} \
                  if (value instanceof QualifiedDescriptorType // binding\n descriptorValue) {} } }";
    let options = Options { line_width: line_width(60), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        if (value
            instanceof
            QualifiedDescriptorType
              /* binding */ descriptorValue) {}
        if (value
            instanceof
            QualifiedDescriptorType // binding
              descriptorValue) {}
      }
    }
    ");
}

#[test]
fn type_pattern_bindings_follow_configured_widths() {
    let sources = [
        "class C { void f() { if (value instanceof Descriptor descriptorValue) {} } }",
        "class C { void f(Object value) { switch (value) { case Descriptor descriptorValue -> {} default -> {} } } }",
        "class C { void f() { if (value instanceof Box(Descriptor descriptorValue)) {} } }",
        "class C { void f() { if (value instanceof final @A Descriptor /* b */ descriptorValue) {} } }",
    ];
    for width in [50, 60, 80, 100] {
        for indent in [2, 4] {
            let options = Options {
                line_width: line_width(width),
                indent_width: indent_width(indent),
                ..Options::default()
            };
            for source in sources {
                let formatted = format_stable_with_options(source, &options);
                assert!(formatted.lines().all(|line| line.len() <= usize::from(width)));
            }
        }
    }
}

#[test]
fn dimensions_assert_messages_and_method_references_wrap() {
    let source = r#"class C {
        int[][] sizes() { return new int[firstDimension][secondDimension]; }
        void check() { assert someCondition : "message text"; }
        Object reference() { return this::referenceMethodName; }
    }"#;
    let options =
        Options { line_width: line_width(30), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class C {
      int[][] sizes() {
        return new int[
          firstDimension
        ][secondDimension];
      }

      void check() {
        assert someCondition
          : "message text";
      }

      Object reference() {
        return this
          ::referenceMethodName;
      }
    }
    "#);
}

#[test]
fn members_after_their_object_break_as_their_context_allows() {
    let source = "class C {
        void f() {
            Runnable r = queue.forRegion(region)::poll;
            consume(queue.forRegion(region)::poll);
            consume(forRegionOf(regionName).poll);
            Object v = forRegionOf(regionName).poll;
            consume(this::someLongMethodReferenceName);
            Runnable s = this::someLongMethodReferenceName;
            consume(queue::poll);
        }
    }";
    let options =
        Options { line_width: line_width(30), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class C {
      void f() {
        Runnable r =
          queue.forRegion(
            region
          )::poll;
        consume(
          queue.forRegion(region)
            ::poll
        );
        consume(
          forRegionOf(regionName)
            .poll
        );
        Object v = forRegionOf(
          regionName
        ).poll;
        consume(
          this
            ::someLongMethodReferenceName
        );
        Runnable s =
          this
            ::someLongMethodReferenceName;
        consume(queue::poll);
      }
    }
    ");
}

#[test]
fn members_after_a_chains_last_call_follow_the_chain() {
    let source = "class C {
        void f() {
            Object v = queue.forRegion(region).poll;
            consume(queue.forRegion(region).poll);
            consume(queues.byRegion(r).byShard(s).poll);
            Runnable p = queues.byRegion(r).byShard(s).poll::run;
            if (queues.byRegion(r).byShard(s).poll) {}
            if (q.forRegion(r).pollingQueueName) {}
            if (q.forRegion(r) // c
                .poll) {}
            consume(this.firstField.second.third);
        }
    }";
    let options =
        Options { line_width: line_width(30), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class C {
      void f() {
        Object v =
          queue.forRegion(
            region
          ).poll;
        consume(
          queue.forRegion(region)
            .poll
        );
        consume(
          queues
            .byRegion(r)
            .byShard(s).poll
        );
        Runnable p = queues
          .byRegion(r)
          .byShard(s).poll::run;
        if (queues
            .byRegion(r)
            .byShard(s).poll) {}
        if (q.forRegion(r)
            .pollingQueueName) {}
        if (q.forRegion(r) // c
            .poll) {}
        consume(
          this.firstField.second
            .third
        );
      }
    }
    ");
}

#[test]
fn method_references_on_a_field_chain_break_after_the_operator() {
    let source = "class C {
        void f() {
            Object o = registry.regions.forRegions.poll::method;
            Object p = this.registry.regions.forRegions.poll::method;
        }
    }";
    let options =
        Options { line_width: line_width(30), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class C {
      void f() {
        Object o =
          registry.regions
            .forRegions.poll
            ::method;
        Object p =
          this.registry.regions
            .forRegions.poll
            ::method;
      }
    }
    ");
}

#[test]
fn fields_break_after_a_parenthesized_object_but_not_in_a_chain_head() {
    let source = "class C {
        void f() {
            Binding binding = (Binding) this.environment.create(ref);
            Binding created = ((Binding) other).environment.create(ref);
            Binding indexed = (Binding) this.bindings[index].create(ref);
            Binding[] types = ((Binding) binding).argumentTypes;
        }
    }";
    let options =
        Options { line_width: line_width(50), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class C {
      void f() {
        Binding binding =
          (Binding) this.environment.create(ref);
        Binding created = (
          (Binding) other
        ).environment.create(ref);
        Binding indexed =
          (Binding) this.bindings[index].create(ref);
        Binding[] types = ((Binding) binding)
          .argumentTypes;
      }
    }
    ");
}

#[test]
fn operators_break_before_the_operator() {
    let statements = "class Example { void run() { \
        if (order.isPaid() && order.isShippable() && !order.isOnHold()) { ship(order); } \
        String label = order.customer().displayName() + suffix; \
        for (ResourceGroupFactory factory : plugin.resourceGroupFactories()) { register(factory); } \
        invoice.setTotal(order.basePrice() * order.quantity() + shippingCostFor(order.region()) \
        - discountFor(order.customer())); \
        Supplier<Order> next = pendingOrderQueues.forRegion(currentRegion)::poll; \
        try { parse(input); } catch (IllegalArgumentException | IllegalStateException exception) { log(exception); } } }";
    let declaration = "class Example { \
        static <T extends Comparable<? super T> & Serializable & Cloneable> T largest(List<T> items) { \
        return Collections.max(items); } }";
    // The class and method add 2 and 4 columns to the design's 50-column examples.
    let at = |width| Options {
        line_width: line_width(width),
        indent_width: indent_width(2),
        ..Options::default()
    };

    assert_snapshot!(format_stable_with_options(statements, &at(54)), @"
    class Example {
      void run() {
        if (order.isPaid()
            && order.isShippable()
            && !order.isOnHold()) {
          ship(order);
        }
        String label =
          order.customer().displayName() + suffix;
        for (ResourceGroupFactory factory :
            plugin.resourceGroupFactories()) {
          register(factory);
        }
        invoice.setTotal(
          order.basePrice() * order.quantity()
            + shippingCostFor(order.region())
            - discountFor(order.customer())
        );
        Supplier<Order> next =
          pendingOrderQueues.forRegion(
            currentRegion
          )::poll;
        try {
          parse(input);
        } catch (IllegalArgumentException
            | IllegalStateException exception) {
          log(exception);
        }
      }
    }
    ");
    assert_snapshot!(format_stable_with_options(declaration, &at(52)), @"
    class Example {
      static <
        T extends Comparable<? super T>
          & Serializable
          & Cloneable
      > T largest(List<T> items) {
        return Collections.max(items);
      }
    }
    ");
}

/// A cast is placed as a prefix operator, so the line breaks after `=` first,
/// then after the cast's `)`, and never inside its parentheses.
#[test]
fn casts_break_after_the_operator_then_after_their_closing_paren() {
    let source = "class Example { void run() { \
        int total = (int) totalFor(order, customer, region); \
        String nameForTheInvoice = (String) store.get(KEY); \
        Map<String, List<OrderLine>> lines = (Map<String, List<OrderLine>>) session.getAttribute(LINES_KEY); \
        Runnable undo = (Runnable & Serializable) () -> { lines.clear(); }; } }";
    // The class and method add 4 columns to the design's 50-column example.
    let options =
        Options { line_width: line_width(54), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class Example {
      void run() {
        int total = (int) totalFor(
          order,
          customer,
          region
        );
        String nameForTheInvoice =
          (String) store.get(KEY);
        Map<String, List<OrderLine>> lines =
          (Map<String, List<OrderLine>>)
            session.getAttribute(LINES_KEY);
        Runnable undo = (Runnable & Serializable) () -> {
          lines.clear();
        };
      }
    }
    ");
}

/// The break after a cast's `)` does not stand in for the break after `=`:
/// the line breaks after `=` when it does not fit up to the operand's `(`.
#[test]
fn a_cast_of_a_call_breaks_after_the_operator_before_its_closing_paren() {
    let source = "class Example { void run() { \
        InvoiceSummary summary = (InvoiceSummary) invoiceRepository.findSummary(customer, region); } }";
    let options =
        Options { line_width: line_width(54), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(format_stable_with_options(source, &options), @"
    class Example {
      void run() {
        InvoiceSummary summary =
          (InvoiceSummary) invoiceRepository.findSummary(
            customer,
            region
          );
      }
    }
    ");
}

#[test]
fn assert_messages_and_method_references_wrap_before_a_multiline_suffix() {
    let source = r#"class C {
        void check() {
            assert pendingJobIsReady : message((Object x) -> { return x; });
            assert isReady // rides past colon
            : "failure";
        }
        Object reference() {
            return longFactoryInstance::</* format
            on two lines */String>create;
        }
    }"#;
    let options =
        Options { line_width: line_width(30), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class C {
      void check() {
        assert pendingJobIsReady
          : message(
            (Object x) -> {
              return x;
            }
          );
        assert isReady : // rides past colon
        "failure";
      }

      Object reference() {
        return longFactoryInstance
          ::<
            /* format
                on two lines */ String
          >create;
      }
    }
    "#);
}

#[test]
fn qualified_names_wrap_after_enclosing_breaks() {
    let source = r"
    package com.example.deliberately.lengthy.packagename;
    import com.example.deliberately.lengthy.ImportedName;
    class C {
      java.util.concurrent.ConcurrentHashMap<String, String> map;
      Object make() { return new java.util.concurrent.ConcurrentHashMap(); }
      Object fields() { return some.deeply.nested.field.access.chain; }
      Object cast(Object o) { return (java.util.concurrent.ConcurrentMap) o; }
      boolean check(Object o) { return o instanceof java.util.concurrent.ConcurrentMap; }
      boolean shorter(Object o) { return o instanceof example.QualifiedName; }
      Object member() { return this.someVeryLongFieldName; }
      Object nestedMember() { return this.someVeryLongFieldName.member; }
      Object commented() { return some.deeply /* left */ . /* right */ nested.field.access.chain; }
      Object commentedOwnLine() { return abcdefghijklmnop
        /* own line */ .secondPortion.middleSuffix.thirdTail; }
      java.util /* left */ . /* right */ concurrent.ConcurrentHashMap<String, String> map2;
    }
    ";
    let options =
        Options { line_width: line_width(30), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    package com.example.deliberately.lengthy.packagename;

    import com.example.deliberately.lengthy.ImportedName;

    class C {
      java.util.concurrent
        .ConcurrentHashMap<
          String,
          String
        > map;

      Object make() {
        return new java.util
          .concurrent
          .ConcurrentHashMap();
      }

      Object fields() {
        return some.deeply.nested
          .field.access.chain;
      }

      Object cast(Object o) {
        return (java.util
          .concurrent
          .ConcurrentMap) o;
      }

      boolean check(Object o) {
        return o
          instanceof
          java.util.concurrent
            .ConcurrentMap;
      }

      boolean shorter(Object o) {
        return o
          instanceof
          example.QualifiedName;
      }

      Object member() {
        return this
          .someVeryLongFieldName;
      }

      Object nestedMember() {
        return this
          .someVeryLongFieldName
          .member;
      }

      Object commented() {
        return some
          .deeply /* left */
          ./* right */ nested
          .field.access.chain;
      }

      Object commentedOwnLine() {
        return abcdefghijklmnop
          /* own line */ .secondPortion
          .middleSuffix.thirdTail;
      }

      java.util /* left */
        ./* right */ concurrent
        .ConcurrentHashMap<
          String,
          String
        > map2;
    }
    "#);
}

#[test]
fn dotted_names_break_as_field_accesses() {
    let source = "
    class C {
      void f() {
        x = registry.regions.forRegions.poll++;
        switch (registry.regions.forRegions.poll) {}
      }
    }
    ";
    let options =
        Options { line_width: line_width(30), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f() {
        x = registry.regions
          .forRegions.poll++;
        switch (registry.regions
            .forRegions.poll) {}
      }
    }
    ");
}

#[test]
fn qualified_type_arguments_wrap_before_dots() {
    let source = "
    class C {
      abcdefgh.ABCDEFGH<IJKL, MNOP>.IJKLMNOP<QRS> value;
      void f() {
        name = new abcde.Example<VeryLongFirstArg, VeryLongSecondArg>();
      }
    }
    ";
    let options =
        Options { line_width: line_width(31), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class C {
      abcdefgh.ABCDEFGH<
        IJKL,
        MNOP
      >.IJKLMNOP<QRS> value;

      void f() {
        name = new abcde.Example<
          VeryLongFirstArg,
          VeryLongSecondArg
        >();
      }
    }
    "#);
}

#[test]
fn qualified_names_stay_together_at_header_and_suffix_breaks() {
    let source = "
    class C {
      void f() { if (!holder.inner.longMethodName()) { consume(); } }
      int read(int value) { return switch (value) {
        case Codes.FIRST, // original
        Codes.SECOND -> 1; default -> 0;
      }; }
    }
    ";
    let options =
        Options { line_width: line_width(36), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class C {
      void f() {
        if (
          !holder.inner.longMethodName()
        ) {
          consume();
        }
      }

      int read(int value) {
        return switch (value) {
          case Codes.FIRST, // original
            Codes.SECOND -> 1;
          default -> 0;
        };
      }
    }
    "#);
}
