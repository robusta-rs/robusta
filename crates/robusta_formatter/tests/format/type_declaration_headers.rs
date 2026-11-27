//! The type parameters and clauses of class, interface, enum, and record headers.

use insta::assert_snapshot;
use robusta_formatter::Options;

use crate::fixtures::{
    assert_examples_print_as_shown, check_contract_at_widths_and_modes, format_source as fmt,
    format_source_with_options as fmt_with_options, indent_width, line_width,
};

#[test]
fn type_declaration_headers_wrap_their_lists_and_clauses() {
    let options = Options { line_width: line_width(100), ..Options::default() };
    let examples = [
        "abstract class DistributedRepository<
        K, V, RequestContext, TransactionMetadata, SerializationStrategy>
    extends AbstractDistributedRepository<
        K, V, RequestContext, TransactionMetadata, SerializationStrategy>
    implements Reader<K, V>,
        TransactionalWriter<K, V, RequestContext, TransactionMetadata>,
        DistributedChangeListener<K, V, TransactionMetadata, SerializationStrategy>,
        SerializationAwareRepositoryAdapter<
            K, V, RequestContext, TransactionMetadata, SerializationStrategy>,
        AutoCloseable,
        Serializable {}
",
        "class OrderCache<
        OrderKeyTypeWithLongName extends Comparable<OrderKeyTypeWithLongName>,
        OrderValueType extends Serializable>
    extends BaseCache implements Closeable {}
",
        "class OrderRepository
    extends AbstractDistributedRepositoryWithLongName<
        RequestContext, TransactionMetadata, SerializationStrategy>
    implements Closeable {}
",
        "class Registry<
    RegistryKeyTypeWithLongName extends Comparable<RegistryKeyTypeWithLongName>, Value> {}
",
        "class OrderService extends AbstractOrderService
    implements FirstInterfaceName, SecondInterfaceName, ThirdInterfaceName {}
",
        "record OrderLine(Item item, int quantity)
    implements Comparable<OrderLine>,
        DescribableOrderEntry,
        SerializableOrderEntry,
        CloneableOrderEntry {}
",
    ];
    assert_examples_print_as_shown(&examples, &options);
}

#[test]
fn type_declaration_header_lists_follow_their_clause_indent() {
    let source = "class Base<K, V> extends VeryLongBaseClassNameForTheProbeWithArguments<FirstArgumentTypeName, SecondArgumentTypeName, ThirdArgumentType> implements X {} \
                  class Nested extends Map<String, List<VeryLongBoundNameForTheProbeWithArguments<FirstArgumentTypeNameWithAVeryLongName, SecondArgumentTypeName>>> {} \
                  class Wildcards<A, B> extends C<? extends VeryLongBoundNameForTheProbeWithArgumentsNumberOneMoreLong, ? super AnotherVeryLongBoundNameForTheProbe> {} \
                  class WildcardBound extends C<? extends VeryLongBoundNameForTheProbeWithArgumentsNumberOneMoreLongAndLonger<FirstArgumentType, SecondArgumentType>, D> {} \
                  sealed class Fits extends AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA implements BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, CCCCCCCCCCCCCCCC permits D {} \
                  sealed class AfterSplit extends AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA implements BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC permits D {} \
                  sealed interface Store<K extends Comparable<K>, VeryLongValueTypeNameForTheInterfaceProbeWithExtraLengthAndMore> extends A, B permits C, D {} \
                  class Annotated<@Annotated(value = \"first\") VeryLongTypeParameterNameWithAnnotationForTheProbe, @Annotated Other extends Foo> {}";
    let options = Options { line_width: line_width(100), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @r#"
    class Base<K, V>
        extends VeryLongBaseClassNameForTheProbeWithArguments<
            FirstArgumentTypeName, SecondArgumentTypeName, ThirdArgumentType>
        implements X {}

    class Nested
        extends Map<
            String,
            List<
                VeryLongBoundNameForTheProbeWithArguments<
                    FirstArgumentTypeNameWithAVeryLongName, SecondArgumentTypeName>>> {}

    class Wildcards<A, B>
        extends C<
            ? extends VeryLongBoundNameForTheProbeWithArgumentsNumberOneMoreLong,
            ? super AnotherVeryLongBoundNameForTheProbe> {}

    class WildcardBound
        extends C<
            ? extends
                VeryLongBoundNameForTheProbeWithArgumentsNumberOneMoreLongAndLonger<
                    FirstArgumentType, SecondArgumentType>,
            D> {}

    sealed class Fits extends AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        implements BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB, CCCCCCCCCCCCCCCC permits D {}

    sealed class AfterSplit extends AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        implements BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB,
            CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC
        permits D {}

    sealed interface Store<
            K extends Comparable<K>, VeryLongValueTypeNameForTheInterfaceProbeWithExtraLengthAndMore>
        extends A, B permits C, D {}

    class Annotated<
        @Annotated(value = "first") VeryLongTypeParameterNameWithAnnotationForTheProbe,
        @Annotated Other extends Foo> {}
    "#);
    check_contract_at_widths_and_modes(source);
}

#[test]
fn type_parameter_bounds_in_a_type_declaration_fill_after_extends() {
    let source = "class Fill<K extends Comparable<K> & java.io.Serializable & Cloneable & VeryLongInterfaceNameForTheBoundsProbeNumberOne & VeryLongInterfaceNameForTheBoundsProbeNumberTwo, V> {} \
                  class Split<K extends VeryLongBoundNameForTheProbeWithArguments<FirstArgumentTypeNameWithAVeryLongName, SecondArgumentTypeName, ThirdArgumentType> & Serializable, V> extends Base {}";
    let options = Options { line_width: line_width(100), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class Fill<
        K extends
            Comparable<K> & java.io.Serializable & Cloneable
                & VeryLongInterfaceNameForTheBoundsProbeNumberOne
                & VeryLongInterfaceNameForTheBoundsProbeNumberTwo,
        V> {}

    class Split<
            K extends
                VeryLongBoundNameForTheProbeWithArguments<
                        FirstArgumentTypeNameWithAVeryLongName,
                        SecondArgumentTypeName,
                        ThirdArgumentType>
                    & Serializable,
            V>
        extends Base {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn type_declaration_header_indents_scale_with_the_indent_width() {
    let source = "class DistributedRepository<K, V, RequestContext, TransactionMetadata, SerializationStrategy, More> extends AbstractDistributedRepository<K, V, RequestContext, TransactionMetadata, SerializationStrategy> implements Reader<K, V>, SerializationAwareRepositoryAdapter<K, V, RequestContext, TransactionMetadata, SerializationStrategy>, Serializable {} \
                  class Registry<RegistryKeyTypeWithLongName extends Comparable<RegistryKeyTypeWithLongName>, ValueName> {}";
    let options = Options {
        line_width: line_width(100),
        indent_width: indent_width(4),
        ..Options::default()
    };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class DistributedRepository<K, V, RequestContext, TransactionMetadata, SerializationStrategy, More>
            extends AbstractDistributedRepository<
                    K, V, RequestContext, TransactionMetadata, SerializationStrategy>
            implements Reader<K, V>,
                    SerializationAwareRepositoryAdapter<
                            K, V, RequestContext, TransactionMetadata, SerializationStrategy>,
                    Serializable {}

    class Registry<
            RegistryKeyTypeWithLongName extends Comparable<RegistryKeyTypeWithLongName>, ValueName> {}
    ");
}

#[test]
fn an_enum_clause_wraps_as_a_class_clause_does() {
    let source = "enum Status implements VeryLongInterfaceNameForTheEnumProbeNumberOne, VeryLongInterfaceNameForTheEnumProbeNumberTwo, Three { A }";
    let options = Options { line_width: line_width(100), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    enum Status
        implements VeryLongInterfaceNameForTheEnumProbeNumberOne,
            VeryLongInterfaceNameForTheEnumProbeNumberTwo,
            Three {
      A,
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn record_clauses_wrap_by_whether_the_components_split() {
    let source = "record TypeParametersSplit<K extends Comparable<K>, VeryLongValueTypeNameForTheRecordProbeWithExtraLengthAndMoreExtra>(K key) implements A, B {} \
                  record NoClause<K extends Comparable<K>, VeryLongValueTypeNameForTheRecordProbeWithExtraLengthAndMoreExtra>(K key, int value) {} \
                  record Unsplit(int a) implements VeryLongBoundNameForTheProbeWithArguments<FirstArgumentTypeNameWithAVeryLongName, SecondArgumentType> {} \
                  record Split(VeryLongComponentTypeNameForTheRecordProbe first, AnotherVeryLongComponentTypeNameForTheRecord second) implements A, B {} \
                  record SplitLongList(VeryLongComponentTypeNameForTheRecordProbe first, AnotherVeryLongComponentTypeNameForTheRecord second) implements VeryLongInterfaceNameForTheRecordProbeNumberOne, VeryLongInterfaceNameForTheRecordProbeNumberTwo {} \
                  record SplitLongArguments(VeryLongComponentTypeNameForTheRecordProbe first, AnotherVeryLongComponentTypeNameForTheRecord second) implements VeryLongBoundNameForTheProbeWithArguments<FirstArgumentTypeNameWithAVeryLongName, SecondArgumentType> {}";
    let options = Options { line_width: line_width(100), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    record TypeParametersSplit<
            K extends Comparable<K>, VeryLongValueTypeNameForTheRecordProbeWithExtraLengthAndMoreExtra>(
      K key
    ) implements A, B {}

    record NoClause<
        K extends Comparable<K>, VeryLongValueTypeNameForTheRecordProbeWithExtraLengthAndMoreExtra>(
      K key,
      int value
    ) {}

    record Unsplit(int a)
        implements VeryLongBoundNameForTheProbeWithArguments<
            FirstArgumentTypeNameWithAVeryLongName, SecondArgumentType> {}

    record Split(
      VeryLongComponentTypeNameForTheRecordProbe first,
      AnotherVeryLongComponentTypeNameForTheRecord second
    ) implements A, B {}

    record SplitLongList(
      VeryLongComponentTypeNameForTheRecordProbe first,
      AnotherVeryLongComponentTypeNameForTheRecord second
    ) implements VeryLongInterfaceNameForTheRecordProbeNumberOne,
        VeryLongInterfaceNameForTheRecordProbeNumberTwo {}

    record SplitLongArguments(
      VeryLongComponentTypeNameForTheRecordProbe first,
      AnotherVeryLongComponentTypeNameForTheRecord second
    ) implements VeryLongBoundNameForTheProbeWithArguments<
      FirstArgumentTypeNameWithAVeryLongName,
      SecondArgumentType
    > {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn only_type_declaration_headers_take_the_header_list_layout() {
    let source = "class A<T> { <VeryLongMethodTypeParameterNameNumberOne, VeryLongMethodTypeParameterNameNumberTwo extends Comparable<VeryLongMethodTypeParameterNameNumberTwo>> void m() {} \
                  Map<VeryLongFieldTypeArgumentNameNumberOneWithExtraLength, VeryLongFieldTypeArgumentNameNumberTwoWithExtraLength> field; } \
                  @Ann(Map<VeryLongFieldTypeArgumentNameNumberOneWithExtraLength, VeryLongFieldTypeArgumentNameNumberTwoWithExtraLength>[].class) class B {} \
                  class C extends @Ann(Map<VeryLongFieldTypeArgumentNameNumberOneWithExtraLength, VeryLongFieldTypeArgumentNameNumberTwoWithExtraLength>[].class) D {}";
    let options = Options { line_width: line_width(100), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A<T> {
      <
        VeryLongMethodTypeParameterNameNumberOne,
        VeryLongMethodTypeParameterNameNumberTwo extends
          Comparable<VeryLongMethodTypeParameterNameNumberTwo>
      > void m() {}

      Map<
        VeryLongFieldTypeArgumentNameNumberOneWithExtraLength,
        VeryLongFieldTypeArgumentNameNumberTwoWithExtraLength
      > field;
    }

    @Ann(
      Map<
        VeryLongFieldTypeArgumentNameNumberOneWithExtraLength,
        VeryLongFieldTypeArgumentNameNumberTwoWithExtraLength
      >[].class
    )
    class B {}

    class C
        extends @Ann(
          Map<
            VeryLongFieldTypeArgumentNameNumberOneWithExtraLength,
            VeryLongFieldTypeArgumentNameNumberTwoWithExtraLength
          >[].class
        ) D {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn a_clause_entry_that_cannot_fit_after_its_keyword_moves_off_its_line() {
    let source = "class Single implements VeryLongInterfaceNameForTheKeywordProbeNumberOneWithExtraLengthAndMoreAndMoreAndMoreLo {} \
                  class Multi implements VeryLongInterfaceNameForTheKeywordProbeNumberOneWithExtraLengthAndMoreAndMoreAndMoreLo, B {} \
                  class Arguments implements VeryLongInterfaceNameForTheKeywordProbeNumberOneWithExtraLengthAndMoreAndMoreAndMoreLo<First, Second> {} \
                  class Fits extends VeryLongInterfaceNameForTheKeywordProbeNumberOne<FirstArgumentTypeName, SecondArgumentTypeName, Third> {}";
    let options = Options { line_width: line_width(100), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class Single
        implements
            VeryLongInterfaceNameForTheKeywordProbeNumberOneWithExtraLengthAndMoreAndMoreAndMoreLo {}

    class Multi
        implements
            VeryLongInterfaceNameForTheKeywordProbeNumberOneWithExtraLengthAndMoreAndMoreAndMoreLo,
            B {}

    class Arguments
        implements
            VeryLongInterfaceNameForTheKeywordProbeNumberOneWithExtraLengthAndMoreAndMoreAndMoreLo<
                First, Second> {}

    class Fits
        extends VeryLongInterfaceNameForTheKeywordProbeNumberOne<
            FirstArgumentTypeName, SecondArgumentTypeName, Third> {}
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn a_nested_type_declaration_header_wraps_from_its_own_indent() {
    let source = "class Outer { static class Inner<FirstTypeParameter extends Comparable<FirstTypeParameter>, SecondTypeParameter extends Serializable> extends Base implements Closeable {} }";
    let options = Options { line_width: line_width(100), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class Outer {
      static class Inner<
              FirstTypeParameter extends Comparable<FirstTypeParameter>,
              SecondTypeParameter extends Serializable>
          extends Base implements Closeable {}
    }
    ");
    check_contract_at_widths_and_modes(source);
}

#[test]
fn a_clause_a_comment_splits_starts_a_new_line() {
    assert_eq!(
        fmt("class A extends D implements G,\n// c\nI {}"),
        "class A extends D\n    implements G,\n        // c\n        I {}\n"
    );
    for source in [
        "class A<K extends B & C, V> extends D<E, F> implements G<H>, /**  x */ /* c */ I {}",
        "enum A implements B, /**  x */ /* c */ C<D> { X }",
    ] {
        check_contract_at_widths_and_modes(source);
    }
}

#[test]
fn a_comment_sharing_the_first_items_line_joins_the_opening_bracket() {
    for (source, expected) in [
        ("class A<\n/* c */ T, U> {}", "class A</* c */ T, U> {}\n"),
        ("class A extends B<\n/* c */ C, D> {}", "class A extends B</* c */ C, D> {}\n"),
        ("class A<\n/* c */\nT, U> {}", "class A<\n    /* c */\n    T,\n    U> {}\n"),
    ] {
        assert_eq!(fmt(source), expected);
        check_contract_at_widths_and_modes(source);
    }
}

#[test]
fn a_comment_after_a_wildcard_keyword_leads_its_bound() {
    for (source, expected) in [
        (
            "class A extends B<? extends // c\nC> {}",
            "class A\n    extends B<\n        ? extends\n            // c\n            C> {}\n",
        ),
        (
            "class A extends B<? super /// c\nC> {}",
            "class A\n    extends B<\n        ? super\n            /// c\n            C> {}\n",
        ),
        (
            "class A extends B<? extends\n// c\nC> {}",
            "class A\n    extends B<\n        ? extends\n            // c\n            C> {}\n",
        ),
        (
            "class A extends B<? extends /* c */ C> {}",
            "class A extends B<? extends /* c */ C> {}\n",
        ),
        (
            "class A extends B<? extends /* c */\nC> {}",
            "class A extends B<? extends /* c */ C> {}\n",
        ),
    ] {
        assert_eq!(fmt(source), expected);
        check_contract_at_widths_and_modes(source);
    }
}

#[test]
fn a_comment_before_a_type_declaration_lists_close_keeps_its_line() {
    for (source, expected) in [
        ("class A<T /* c */> {}", "class A<T /* c */> {}\n"),
        ("class A<T // c\n> {}", "class A<\n    T // c\n    > {}\n"),
        ("class A<T\n/* c */\n> {}", "class A<\n    T\n    /* c */\n    > {}\n"),
        (
            "class A<T, U // c\n> extends B {}",
            "class A<\n        T,\n        U // c\n        >\n    extends B {}\n",
        ),
        (
            "class A extends B<C, D // c\n> implements E {}",
            "class A\n    extends B<\n        C,\n        D // c\n        >\n    implements E {}\n",
        ),
        (
            "record R(int a, // x\nint b) implements B<C, D // c\n> {}",
            "record R(\n  int a, // x\n  int b\n) implements B<\n  C,\n  D // c\n> {}\n",
        ),
    ] {
        assert_eq!(fmt(source), expected);
        check_contract_at_widths_and_modes(source);
    }
}
