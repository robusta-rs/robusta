use super::*;

#[test]
fn parses_sealed_class_and_interface_headers() {
    let allocator = Allocator::new();
    let source = "public sealed class Shape permits Circle, geometry.Square {}\
                  sealed interface Node extends Base permits Leaf, tree.Branch {}\
                  non-sealed class FreeShape extends Shape {}\
                  class Uses { sealed value; void f() { sealed = sealed - 1; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    assert_eq!(unit.types().len(), 4);

    let TypeDeclaration::Class(shape) = &unit.types()[0] else {
        panic!("expected a sealed class");
    };
    assert!(matches!(&shape.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::Public));
    assert!(matches!(&shape.modifiers[1], Modifier::Keyword(k) if k.kind == ModifierKind::Sealed));
    assert_eq!(shape.permits.len(), 2);
    assert_eq!(type_name_parts(&shape.permits[0]), ["Circle"]);
    assert_eq!(type_name_parts(&shape.permits[1]), ["geometry", "Square"]);

    let TypeDeclaration::Interface(node) = &unit.types()[1] else {
        panic!("expected a sealed interface");
    };
    assert!(matches!(&node.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::Sealed));
    assert_eq!(node.extends.len(), 1);
    assert_eq!(node.permits.len(), 2);

    let TypeDeclaration::Class(free_shape) = &unit.types()[2] else {
        panic!("expected a non-sealed class");
    };
    assert!(
        matches!(&free_shape.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::NonSealed)
    );
    assert!(free_shape.superclass.is_some());

    let TypeDeclaration::Class(uses) = &unit.types()[3] else {
        panic!("expected a class");
    };
    let ClassMember::Field(field) = &uses.body.members[0] else {
        panic!("expected a field");
    };
    let Some(ty) = field.ty.as_class_type() else {
        panic!("expected a class type");
    };
    assert_eq!(class_type_names(ty), ["sealed"]);
    assert_eq!(field.declarators[0].name.name, "value");
}

#[test]
fn parses_record_declarations() {
    let allocator = Allocator::new();
    let source = "@Entity public record Pair<T>(@Nonnull String left, java.util.List<T> right, String @Nullable ... tags) \
                  implements Named, java.io.Serializable {\
                      Pair(String left, java.util.List<T> right, String... tags) { this.left = left; }\
                      record Meta(int code) {}\
                  }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;

    let TypeDeclaration::Record(record) = &unit.types()[0] else {
        panic!("expected a record");
    };
    assert_eq!(record.name.name, "Pair");
    assert_eq!(record.modifiers.len(), 2);
    assert_eq!(record.type_params.as_ref().unwrap().params.len(), 1);
    assert_eq!(record.header.components.len(), 3);
    assert_eq!(record.header.components[0].modifiers.len(), 1);
    assert_eq!(record.header.components[0].name.name, "left");
    assert_eq!(record.header.components[1].name.name, "right");
    assert!(record.header.components[2].varargs);
    assert_eq!(record.header.components[2].varargs_annotations.len(), 1);
    assert_eq!(
        type_name_parts(&record.header.components[2].varargs_annotations[0].name),
        ["Nullable"]
    );
    assert_eq!(record.interfaces.len(), 2);
    assert_eq!(record.body.members.len(), 2);
    assert!(matches!(&record.body.members[0], ClassMember::Constructor(_)));
    let ClassMember::Record(meta) = &record.body.members[1] else {
        panic!("expected a nested record");
    };
    assert_eq!(meta.name.name, "Meta");
    assert_eq!(meta.header.span.source_text(source), "(int code)");
    assert_eq!(meta.header.components.len(), 1);
}

#[test]
fn parses_local_record_declarations() {
    let allocator = Allocator::new();
    let source = "class A { void f() { record Pair(int left, int right) {} use(); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let body = method.body.as_ref().unwrap();
    let Statement::Record(record) = &body.statements[0] else {
        panic!("expected a local record");
    };
    assert_eq!(record.name.name, "Pair");
    assert_eq!(record.header.components.len(), 2);
}

#[test]
fn record_remains_a_plain_identifier_outside_declaration_starts() {
    let allocator = Allocator::new();
    let result = parse(
        &allocator,
        "class record { record value; int record; void f() { record local = value; record = record + 1; } }",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(class.name.name, "record");
    let ClassMember::Field(field) = &class.body.members[0] else {
        panic!("expected a field");
    };
    let Some(ty) = field.ty.as_class_type() else {
        panic!("expected a class field type");
    };
    assert_eq!(class_type_names(ty), ["record"]);
}

#[test]
fn parses_compact_record_constructors() {
    let allocator = Allocator::new();
    let result = parse(
        &allocator,
        "record Point(int x, int y) { @Deprecated public Point { if (x < 0) { throw bad(); } } }",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Record(record) = &unit.types()[0] else {
        panic!("expected a record");
    };
    assert_eq!(record.body.members.len(), 1);
    let ClassMember::CompactConstructor(ctor) = &record.body.members[0] else {
        panic!("expected a compact constructor");
    };
    assert_eq!(ctor.name.name, "Point");
    assert_eq!(ctor.modifiers.len(), 2);
    assert!(matches!(&ctor.modifiers[1], Modifier::Keyword(k) if k.kind == ModifierKind::Public));
    assert!(matches!(&ctor.body.statements[0], Statement::If(_)));
}

#[test]
fn parses_methods_constructors_and_fields() {
    let allocator = Allocator::new();
    let source = "class Greeter {\
                          int count;\
                          Greeter(String greeting) throws A.B, C { this(greeting, 0); String g = greeting; }\
                          <T> Greeter(T ignored) { super(); return; }\
                          final String greet(final String name) { return name; }\
                          static <T extends CharSequence> T identity(final T value) throws E { return value; }\
                          abstract void reset();\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(class.body.members.len(), 6);

    assert!(matches!(&class.body.members[0], ClassMember::Field(_)));

    let ClassMember::Constructor(ctor) = &class.body.members[1] else {
        panic!("expected a constructor");
    };
    assert_eq!(ctor.name.name, "Greeter");
    assert!(ctor.type_params.is_none());
    assert_eq!(ctor.params.items.len(), 1);
    assert_eq!(ctor.throws.len(), 2);
    assert_eq!(ctor.body.statements.len(), 2);
    let Statement::ConstructorInvocation(invocation) = &ctor.body.statements[0] else {
        panic!("expected a constructor invocation");
    };
    assert_eq!(invocation.kind, ConstructorInvocationKind::This);
    assert!(invocation.type_args.is_none());
    assert!(invocation.qualifier.is_none());
    assert_eq!(invocation.args.len(), 2);
    assert!(matches!(&ctor.body.statements[1], Statement::LocalVariable(_)));

    let ClassMember::Constructor(generic_ctor) = &class.body.members[2] else {
        panic!("expected a generic constructor");
    };
    let type_params = &generic_ctor.type_params.as_ref().unwrap().params;
    assert_eq!(type_params.len(), 1);
    assert_eq!(type_params[0].name.name, "T");
    let Statement::ConstructorInvocation(invocation) = &generic_ctor.body.statements[0] else {
        panic!("expected a constructor invocation");
    };
    assert_eq!(invocation.kind, ConstructorInvocationKind::Super);
    assert!(invocation.type_args.is_none());
    assert!(invocation.qualifier.is_none());

    let ClassMember::Method(method) = &class.body.members[3] else {
        panic!("expected a method");
    };
    assert_eq!(method.name.name, "greet");
    assert!(method.type_params.is_none());
    assert_eq!(method.params.items.len(), 1);
    assert_eq!(method.params.items[0].modifiers.len(), 1);
    let body = method.body.as_ref().unwrap();
    let Statement::Return(ret) = &body.statements[0] else {
        panic!("expected a return statement");
    };
    assert!(ret.value.is_some());

    let ClassMember::Method(generic_method) = &class.body.members[4] else {
        panic!("expected a generic method");
    };
    assert_eq!(generic_method.name.name, "identity");
    let type_params = &generic_method.type_params.as_ref().unwrap().params;
    assert_eq!(type_params.len(), 1);
    assert_eq!(type_params[0].bounds.len(), 1);

    let ClassMember::Method(bodyless) = &class.body.members[5] else {
        panic!("expected a method");
    };
    assert!(bodyless.body.is_none());
}

#[test]
fn throws_types_carry_annotations_and_qualified_names() {
    let allocator = Allocator::new();
    // A thrown `ClassType` may carry type-use annotations, on the leading
    // segment (`@Critical IOException`) or an interior one
    // (`java.io.@Nullable IOException`); JLS §8.4.6 / §9.7.4.
    let source =
        "class C { void f() throws @Critical IOException, java.io.@Nullable IOException {} }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    assert_eq!(method.throws.len(), 2);

    // Leading annotation on a single-segment exception type.
    let Some(first) = method.throws[0].as_class_type() else {
        panic!("expected a class exception type");
    };
    let first_segments = class_type_parts(first);
    assert_eq!(class_type_annotations(first_segments[0]).len(), 1);
    assert_eq!(type_name_parts(&class_type_annotations(first_segments[0])[0].name), ["Critical"]);
    assert_eq!(first_segments.len(), 1);
    assert_eq!(first_segments[0].name(), "IOException");

    // Interior annotation on a qualified exception type: the annotation sits on
    // the final `IOException` segment, not the `java`/`io` package segments.
    let Some(second) = method.throws[1].as_class_type() else {
        panic!("expected a qualified exception type");
    };
    let second_segments = class_type_parts(second);
    assert_eq!(second_segments.len(), 3);
    assert!(class_type_annotations(second_segments[0]).is_empty());
    assert_eq!(class_type_annotations(second_segments[2]).len(), 1);
    assert_eq!(type_name_parts(&class_type_annotations(second_segments[2])[0].name), ["Nullable"]);
}

#[test]
fn parses_instance_and_static_initializer_blocks() {
    let allocator = Allocator::new();
    let source = "class Registry {\
                          static final java.util.Map<String, String> DEFAULTS;\
                          static { DEFAULTS.put(\"a\", \"1\"); }\
                          int count;\
                          { count = 0; log(); }\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(class.body.members.len(), 4);

    let ClassMember::Initializer(static_init) = &class.body.members[1] else {
        panic!("expected a static initializer");
    };
    assert!(static_init.is_static);
    assert_eq!(static_init.block.statements.len(), 1);

    let ClassMember::Initializer(instance_init) = &class.body.members[3] else {
        panic!("expected an instance initializer");
    };
    assert!(!instance_init.is_static);
    assert_eq!(instance_init.block.statements.len(), 2);
}

#[test]
fn parses_interface_declaration() {
    let allocator = Allocator::new();
    let source = "public interface Cache<K, V> extends Store<K, V>, AutoCloseable {\
                          int LIMIT = 64;\
                          V get(K key);\
                          default V getOrNull(K key) { return get(key); }\
                          static <T> Cache<T, T> identity() { return null; }\
                          private void evict() {}\
                          ;\
                          class Support {}\
                          @interface Tag { String value(); }\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Interface(interface) = &unit.types()[0] else {
        panic!("expected an interface");
    };
    assert_eq!(interface.name.name, "Cache");
    assert_eq!(interface.modifiers.len(), 1);
    assert_eq!(interface.type_params.as_ref().unwrap().params.len(), 2);
    assert_eq!(interface.extends.len(), 2);
    assert_eq!(interface.body.members.len(), 8);

    assert!(matches!(&interface.body.members[0], InterfaceMember::Constant(_)));

    let InterfaceMember::Method(abstract_method) = &interface.body.members[1] else {
        panic!("expected a method");
    };
    assert!(abstract_method.body.is_none());

    let InterfaceMember::Method(default_method) = &interface.body.members[2] else {
        panic!("expected a method");
    };
    assert!(matches!(
        &default_method.modifiers[0],
        Modifier::Keyword(KeywordModifier { kind: ModifierKind::Default, .. })
    ));
    assert!(default_method.body.is_some());

    let InterfaceMember::Method(static_method) = &interface.body.members[3] else {
        panic!("expected a method");
    };
    assert_eq!(static_method.type_params.as_ref().unwrap().params.len(), 1);

    assert!(matches!(&interface.body.members[4], InterfaceMember::Method(_)));
    assert!(matches!(&interface.body.members[5], InterfaceMember::Empty(_)));
    assert!(matches!(&interface.body.members[6], InterfaceMember::Class(_)));
    assert!(matches!(&interface.body.members[7], InterfaceMember::AnnotationInterface(_)));
}

#[test]
fn parses_enum_declaration() {
    let allocator = Allocator::new();
    let source = "public enum Color implements Code, Named {\
                          @Deprecated RED(1),\
                          GREEN(),\
                          BLUE,;\
                          private final int code;\
                          Color(int code) { this.code = code; }\
                          int code() { return code; }\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Enum(enum_decl) = &unit.types()[0] else {
        panic!("expected an enum");
    };
    assert_eq!(enum_decl.name.name, "Color");
    assert_eq!(enum_decl.modifiers.len(), 1);
    assert_eq!(enum_decl.interfaces.len(), 2);
    assert_eq!(enum_decl.body.constants.len(), 3);
    assert!(enum_decl.body.has_body_semicolon);
    assert_eq!(enum_decl.body.members.len(), 3);

    assert_eq!(enum_decl.body.constants[0].name.name, "RED");
    assert!(matches!(&enum_decl.body.constants[0].modifiers[0], Modifier::Annotation(_)));
    assert_eq!(enum_decl.body.constants[0].args.as_ref().unwrap().len(), 1);
    assert_eq!(enum_decl.body.constants[1].name.name, "GREEN");
    assert!(enum_decl.body.constants[1].args.as_ref().unwrap().is_empty());
    assert!(enum_decl.body.constants[2].args.is_none());

    assert!(matches!(&enum_decl.body.members[0], ClassMember::Field(_)));
    assert!(matches!(&enum_decl.body.members[1], ClassMember::Constructor(_)));
    assert!(matches!(&enum_decl.body.members[2], ClassMember::Method(_)));
}

#[test]
fn enum_body_comma_does_not_require_constants() {
    for (source, has_semicolon, member_count) in [
        ("enum Empty { , }", false, 0),
        ("enum Empty { ,; }", true, 0),
        ("enum Utility { ,; int x; }", true, 1),
        ("enum Utility { /* before */ , // after\n; int x; }", true, 1),
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        let TypeDeclaration::Enum(enum_decl) = &result.unit.types()[0] else {
            panic!("expected an enum");
        };
        assert!(enum_decl.body.constants.is_empty());
        assert_eq!(enum_decl.body.has_body_semicolon, has_semicolon);
        assert_eq!(enum_decl.body.members.len(), member_count);
    }
}

#[test]
fn parses_annotation_interface_declaration() {
    let allocator = Allocator::new();
    let source = "@Doc public @interface Route {\
                          String value() default \"/\";\
                          int[] codes() default {200, 201,};\
                          Class<?> type();\
                          int LIMIT = 64;\
                          @interface Nested { String name(); }\
                          enum Mode { GET, POST }\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::AnnotationInterface(annotation_interface) = &unit.types()[0] else {
        panic!("expected an annotation interface");
    };
    assert_eq!(annotation_interface.name.name, "Route");
    assert_eq!(annotation_interface.modifiers.len(), 2);
    assert_eq!(annotation_interface.body.members.len(), 6);

    let AnnotationInterfaceMemberDeclaration::Element(value) =
        &annotation_interface.body.members[0]
    else {
        panic!("expected an annotation element");
    };
    assert_eq!(value.name.name, "value");
    assert!(matches!(&value.default_value, Some(ElementValue::StringLiteral(_))));

    let AnnotationInterfaceMemberDeclaration::Element(codes) =
        &annotation_interface.body.members[1]
    else {
        panic!("expected an annotation element");
    };
    assert_eq!(codes.name.name, "codes");
    assert!(matches!(&codes.ty, Type::Array(_)));
    let Some(ElementValue::Array(default_codes)) = &codes.default_value else {
        panic!("expected an array default");
    };
    assert_eq!(default_codes.values.len(), 2);
    assert!(default_codes.trailing_comma);

    assert!(matches!(
        &annotation_interface.body.members[2],
        AnnotationInterfaceMemberDeclaration::Element(_)
    ));
    assert!(matches!(
        &annotation_interface.body.members[3],
        AnnotationInterfaceMemberDeclaration::Field(_)
    ));
    assert!(matches!(
        &annotation_interface.body.members[4],
        AnnotationInterfaceMemberDeclaration::AnnotationInterface(_)
    ));
    assert!(matches!(
        &annotation_interface.body.members[5],
        AnnotationInterfaceMemberDeclaration::Enum(_)
    ));
}

#[test]
fn parses_post_paren_annotation_element_dimensions() {
    let allocator = Allocator::new();
    let source = "@interface A {\
                          String value()[];\
                          String tags() @Dim [] @Other [] default {\"a\",};\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::AnnotationInterface(annotation_interface) = &unit.types()[0] else {
        panic!("expected an annotation interface");
    };
    assert_eq!(annotation_interface.body.members.len(), 2);

    let AnnotationInterfaceMemberDeclaration::Element(value) =
        &annotation_interface.body.members[0]
    else {
        panic!("expected an annotation element");
    };
    assert!(matches!(&value.ty, Type::Simple(_)));
    assert_eq!(value.post_paren_dimensions.len(), 1);
    assert!(value.post_paren_dimensions[0].annotations.is_empty());
    assert!(value.default_value.is_none());

    let AnnotationInterfaceMemberDeclaration::Element(tags) = &annotation_interface.body.members[1]
    else {
        panic!("expected an annotation element");
    };
    assert_eq!(tags.post_paren_dimensions.len(), 2);
    assert_eq!(type_name_parts(&tags.post_paren_dimensions[0].annotations[0].name), ["Dim"]);
    assert_eq!(type_name_parts(&tags.post_paren_dimensions[1].annotations[0].name), ["Other"]);
    let Some(ElementValue::Array(default_tags)) = &tags.default_value else {
        panic!("expected an array default");
    };
    assert_eq!(default_tags.values.len(), 1);
    assert!(default_tags.trailing_comma);
}

#[test]
fn parses_enum_constant_class_bodies() {
    let allocator = Allocator::new();
    let source = "enum Op {\
                          PLUS { int apply(int a, int b) { return a + b; } },\
                          TIMES() { int apply(int a, int b) { return a * b; } },\
                          UNIT {};\
                          abstract int apply(int a, int b);\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Enum(enum_decl) = &unit.types()[0] else {
        panic!("expected an enum");
    };
    assert_eq!(enum_decl.body.constants.len(), 3);

    let plus = &enum_decl.body.constants[0];
    assert_eq!(plus.name.name, "PLUS");
    assert!(plus.args.is_none());
    let plus_body = plus.body.as_ref().expect("PLUS has a class body");
    assert_eq!(plus_body.members.len(), 1);
    assert!(matches!(&plus_body.members[0], ClassMember::Method(_)));

    let times = &enum_decl.body.constants[1];
    assert!(times.args.as_ref().unwrap().is_empty());
    assert!(times.body.is_some());

    // `A {}` is an empty body, distinct from a body-less constant.
    let unit_const = &enum_decl.body.constants[2];
    let unit_body = unit_const.body.as_ref().expect("UNIT has an empty body");
    assert!(unit_body.members.is_empty());

    assert_eq!(enum_decl.body.members.len(), 1);
}

#[test]
fn parses_class_interface_enum_and_annotation_interface_side_by_side() {
    let allocator = Allocator::new();
    let source = "interface Id {}\nenum Kind {}\n@interface Route {}\nclass Impl implements Id {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    assert_eq!(unit.types().len(), 4);
    assert!(matches!(&unit.types()[0], TypeDeclaration::Interface(_)));
    assert!(matches!(&unit.types()[1], TypeDeclaration::Enum(_)));
    assert!(matches!(&unit.types()[2], TypeDeclaration::AnnotationInterface(_)));
    assert!(matches!(&unit.types()[3], TypeDeclaration::Class(_)));
}

#[test]
fn parses_nested_member_types() {
    let allocator = Allocator::new();
    let source = "class Outer {\
                          private int x;\
                          @Frozen public static final class Inner extends Base {\
                              void f() {}\
                              static class Deeper { int y; }\
                          }\
                          interface Listener { void onEvent(); }\
                          private enum State { ON, OFF }\
                          Outer() {}\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(outer) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(outer.body.members.len(), 5);

    let ClassMember::Class(inner) = &outer.body.members[1] else {
        panic!("expected a nested class");
    };
    assert_eq!(inner.name.name, "Inner");
    assert_eq!(inner.modifiers.len(), 4);
    assert!(matches!(&inner.modifiers[0], Modifier::Annotation(_)));
    assert!(inner.superclass.is_some());
    assert_eq!(inner.body.members.len(), 2);
    let ClassMember::Class(deeper) = &inner.body.members[1] else {
        panic!("expected a doubly nested class");
    };
    assert_eq!(deeper.name.name, "Deeper");

    let ClassMember::Interface(listener) = &outer.body.members[2] else {
        panic!("expected a nested interface");
    };
    assert_eq!(listener.name.name, "Listener");

    let ClassMember::Enum(state) = &outer.body.members[3] else {
        panic!("expected a nested enum");
    };
    assert_eq!(state.body.constants.len(), 2);

    // The `Name(` constructor lookahead still wins after the nested types.
    assert!(matches!(&outer.body.members[4], ClassMember::Constructor(_)));
}

#[test]
fn parses_varargs_formal_parameters() {
    let allocator = Allocator::new();
    let source = "class Log {\
                          Log(String... names) {}\
                          void f(final String prefix, int[]... counts) {}\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };

    let ClassMember::Constructor(ctor) = &class.body.members[0] else {
        panic!("expected a constructor");
    };
    assert_eq!(ctor.params.items.len(), 1);
    assert!(ctor.params.items[0].varargs);
    assert_eq!(ctor.params.items[0].name.name, "names");

    let ClassMember::Method(method) = &class.body.members[1] else {
        panic!("expected a method");
    };
    assert_eq!(method.params.items.len(), 2);
    assert!(!method.params.items[0].varargs);
    assert!(method.params.items[1].varargs);
    assert!(matches!(&method.params.items[1].ty, Some(VariableType::Array(_))));
}

#[test]
fn parses_receiver_parameters() {
    let allocator = Allocator::new();
    let source = "class A {\
                      void f(A this) {}\
                      void g(@Ann A this, int value) {}\
                      class B {\
                          B(A A.this, String name) {}\
                          void h(A.@Ann B this) {}\
                      }\
                  }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let TypeDeclaration::Class(class) = &result.unit.types()[0] else {
        panic!("expected a class");
    };

    let ClassMember::Method(f) = &class.body.members[0] else { panic!("expected a method") };
    assert_eq!(f.params.span.source_text(source), "(A this)");
    let receiver = f.params.receiver.as_ref().expect("receiver");
    assert!(receiver.qualifier.is_none());
    assert_eq!(receiver.span.source_text(source), "A this");
    assert!(f.params.items.is_empty());

    let ClassMember::Method(g) = &class.body.members[1] else { panic!("expected a method") };
    let receiver = g.params.receiver.as_ref().expect("receiver");
    assert_eq!(receiver.span.source_text(source), "@Ann A this");
    assert_eq!(g.params.items.len(), 1);
    assert_eq!(g.params.items[0].name.name, "value");

    let ClassMember::Class(inner) = &class.body.members[2] else {
        panic!("expected a class");
    };
    let ClassMember::Constructor(ctor) = &inner.body.members[0] else {
        panic!("expected a constructor");
    };
    let receiver = ctor.params.receiver.as_ref().expect("receiver");
    assert_eq!(receiver.qualifier.as_ref().map(|qualifier| qualifier.name), Some("A"));
    assert_eq!(receiver.span.source_text(source), "A A.this");
    assert_eq!(ctor.params.items.len(), 1);

    let ClassMember::Method(h) = &inner.body.members[1] else { panic!("expected a method") };
    assert!(h.params.receiver.is_some());
    assert!(h.params.items.is_empty());
}

#[test]
fn generic_members_start_at_their_type_parameter_list() {
    let allocator = Allocator::new();
    let source = "class A { <T> void f() {} <T> A() {} interface I { <T> void g(); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let TypeDeclaration::Class(class) = &result.unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else { panic!("expected a method") };
    assert_eq!(method.span.source_text(source), "<T> void f() {}");
    let ClassMember::Constructor(ctor) = &class.body.members[1] else {
        panic!("expected a constructor");
    };
    assert_eq!(ctor.span.source_text(source), "<T> A() {}");
    let ClassMember::Interface(interface) = &class.body.members[2] else {
        panic!("expected an interface");
    };
    let InterfaceMember::Method(method) = &interface.body.members[0] else {
        panic!("expected a method");
    };
    assert_eq!(method.span.source_text(source), "<T> void g();");

    let source = "<T> void f() {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let compact = result.unit.compact().expect("expected a compact unit");
    let ClassMember::Method(method) = &compact.members[0] else { panic!("expected a method") };
    assert_eq!(method.span.source_text(source), "<T> void f() {}");
}

#[test]
fn tolerates_stray_semicolons() {
    let allocator = Allocator::new();
    let source = ";;\nclass A { ; int x; ; }\n;";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    assert_eq!(unit.types().len(), 4);
    let TypeDeclaration::Empty(first_empty) = &unit.types()[0] else {
        panic!("expected an empty type declaration");
    };
    assert_eq!(first_empty.span.source_text(source), ";");
    assert!(matches!(&unit.types()[1], TypeDeclaration::Empty(_)));
    assert!(matches!(&unit.types()[3], TypeDeclaration::Empty(_)));

    let TypeDeclaration::Class(class) = &unit.types()[2] else {
        panic!("expected a class");
    };
    assert_eq!(class.body.members.len(), 3);
    let ClassMember::Empty(member_empty) = &class.body.members[0] else {
        panic!("expected an empty class member");
    };
    assert_eq!(member_empty.span.source_text(source), ";");
    assert!(matches!(&class.body.members[1], ClassMember::Field(_)));
    assert!(matches!(&class.body.members[2], ClassMember::Empty(_)));
}

#[test]
fn annotation_interface_empty_member_declarations_are_preserved() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "@interface A { ; String value(); ; }");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::AnnotationInterface(annotation_interface) = &unit.types()[0] else {
        panic!("expected an annotation interface");
    };
    assert_eq!(annotation_interface.body.members.len(), 3);
    assert!(matches!(
        &annotation_interface.body.members[0],
        AnnotationInterfaceMemberDeclaration::Empty(_)
    ));
    assert!(matches!(
        &annotation_interface.body.members[1],
        AnnotationInterfaceMemberDeclaration::Element(_)
    ));
    assert!(matches!(
        &annotation_interface.body.members[2],
        AnnotationInterfaceMemberDeclaration::Empty(_)
    ));
}

#[test]
fn record_body_empty_member_declarations_are_preserved() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "record R(int x) { ; void f() {} ; }");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Record(record) = &unit.types()[0] else {
        panic!("expected a record");
    };
    assert_eq!(record.body.members.len(), 3);
    assert!(matches!(&record.body.members[0], ClassMember::Empty(_)));
    assert!(matches!(&record.body.members[1], ClassMember::Method(_)));
    assert!(matches!(&record.body.members[2], ClassMember::Empty(_)));
}

#[test]
fn enum_body_empty_member_declarations_are_preserved() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "enum E { A;; ; int x; }");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Enum(enum_decl) = &unit.types()[0] else {
        panic!("expected an enum");
    };
    assert_eq!(enum_decl.body.constants.len(), 1);
    assert!(enum_decl.body.has_body_semicolon);
    assert_eq!(enum_decl.body.members.len(), 3);
    assert!(matches!(&enum_decl.body.members[0], ClassMember::Empty(_)));
    assert!(matches!(&enum_decl.body.members[1], ClassMember::Empty(_)));
    assert!(matches!(&enum_decl.body.members[2], ClassMember::Field(_)));
}

#[test]
fn parses_core_keyword_modifiers() {
    let allocator = Allocator::new();
    let result = parse(
        &allocator,
        "strictfp class A { transient volatile int flags; native synchronized void ping(); }",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert!(
        matches!(&class.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::Strictfp)
    );

    let ClassMember::Field(field) = &class.body.members[0] else {
        panic!("expected a field");
    };
    assert!(
        matches!(&field.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::Transient)
    );
    assert!(
        matches!(&field.modifiers[1], Modifier::Keyword(k) if k.kind == ModifierKind::Volatile)
    );

    let ClassMember::Method(method) = &class.body.members[1] else {
        panic!("expected a method");
    };
    assert!(matches!(&method.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::Native));
    assert!(
        matches!(&method.modifiers[1], Modifier::Keyword(k) if k.kind == ModifierKind::Synchronized)
    );
    assert!(method.body.is_none());
}
