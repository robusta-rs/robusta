use super::*;

#[test]
fn annotations_interleave_with_keyword_modifiers_in_source_order() {
    let allocator = Allocator::new();
    let result = parse(
        &allocator,
        "@Entity public @Deprecated final class A { @Override public void f() {} }",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(class.modifiers.len(), 4);
    assert_eq!(type_name_parts(&class.modifiers[0].as_annotation().unwrap().name), ["Entity"]);
    assert!(matches!(&class.modifiers[1], Modifier::Keyword(k) if k.kind == ModifierKind::Public));
    assert_eq!(type_name_parts(&class.modifiers[2].as_annotation().unwrap().name), ["Deprecated"]);
    assert!(matches!(&class.modifiers[3], Modifier::Keyword(k) if k.kind == ModifierKind::Final));
}

#[test]
fn marker_and_empty_argument_annotations_stay_distinct() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "class A { @Foo int x; @Bar() int y; }");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Field(x) = &class.body.members[0] else {
        panic!("expected a field");
    };
    assert!(x.modifiers[0].as_annotation().unwrap().is_marker());
    let ClassMember::Field(y) = &class.body.members[1] else {
        panic!("expected a field");
    };
    let bar = y.modifiers[0].as_annotation().unwrap();
    assert!(!bar.is_marker());
    assert!(matches!(bar.args.as_deref(), Some([])));
}

#[test]
fn normal_annotation_parses_pairs_nested_annotations_and_arrays() {
    let allocator = Allocator::new();
    let result = parse(
        &allocator,
        "@Named(value = \"x\", scopes = {@Scope(SINGLETON), @Scope(REQUEST),}) class A {}",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let named = class.modifiers[0].as_annotation().unwrap();
    let Some([AnnotationArgument::Pair(value), AnnotationArgument::Pair(scopes)]) =
        named.args.as_deref()
    else {
        panic!("expected two pairs");
    };
    assert_eq!(value.name.name, "value");
    assert!(matches!(&value.value, ElementValue::StringLiteral(_)));
    let ElementValue::Array(scopes) = &scopes.value else {
        panic!("expected an element value array");
    };
    assert_eq!(scopes.values.len(), 2);
    assert!(scopes.trailing_comma);
    let ElementValue::Annotation(scope) = &scopes.values[0] else {
        panic!("expected a nested annotation");
    };
    assert!(matches!(scope.args.as_deref(), Some([argument]) if argument.is_element_value()));
}

#[test]
fn annotation_value_is_the_single_element_or_the_value_pair() {
    let allocator = Allocator::new();
    let source = "@A(1) @B(x = 2, value = 3) @C(x = 4) @D() @E class F {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let TypeDeclaration::Class(class) = &result.unit.types()[0] else {
        panic!("expected a class");
    };
    let values = class
        .modifiers
        .iter()
        .map(|modifier| {
            let annotation = modifier.as_annotation().unwrap();
            annotation.value().map(|value| value.span().source_text(source))
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(values, [Some("1"), Some("3"), None, None, None]);
}

#[test]
fn annotation_arrays_accept_a_comma_without_values() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "@A({,}) class C {}");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let TypeDeclaration::Class(class) = &result.unit.types()[0] else {
        panic!("expected a class");
    };
    let annotation = class.modifiers[0].as_annotation().unwrap();
    let Some([AnnotationArgument::Array(array)]) = annotation.args.as_deref() else {
        panic!("expected an annotation array");
    };
    assert!(array.values.is_empty());
    assert!(array.trailing_comma);

    for source in [
        "@A(value = {,}) class C {}",
        "@interface A { int[] value() default {,}; }",
        "class C { @SuppressWarnings({,}) int e = 0; }",
    ] {
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
    }
}

#[test]
fn annotated_locals_are_recognized_as_declarations() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { @a.b.Qual(order = {1, 2}) final int x = 1; } }",
    );
    let Statement::LocalVariable(local) = stmt else {
        panic!("expected a local variable declaration");
    };
    assert_eq!(local.modifiers.len(), 2);
    let qual = local.modifiers[0].as_annotation().unwrap();
    assert_eq!(type_name_parts(&qual.name), ["a", "b", "Qual"]);
}
