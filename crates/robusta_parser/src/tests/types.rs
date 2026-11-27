use super::*;

#[test]
fn written_type_names_use_decoded_segments_and_retain_array_dimensions() {
    enum Element<'a> {
        Class(&'a [&'a str]),
        Primitive(PrimitiveTypeKind),
    }

    for (written, expected, dimensions) in [
        ("String", Element::Class(&["String"]), 0),
        (r"L\u0069st<String>", Element::Class(&["List"]), 0),
        (
            r"pkg.\u004futer<String>.@A Inner<Integer>",
            Element::Class(&["pkg", "Outer", "Inner"]),
            0,
        ),
        (
            r"pkg.\u004futer<String>.Inner<Integer> @A [] []",
            Element::Class(&["pkg", "Outer", "Inner"]),
            2,
        ),
        (r"\u0069nt[]", Element::Primitive(PrimitiveTypeKind::Int), 1),
    ] {
        let source = format!("class C {{ {written} field; }}");
        let allocator = Allocator::new();
        let parsed = parse(&allocator, &source);
        assert!(parsed.errors.is_empty(), "{source}: {:?}", parsed.errors);
        let TypeDeclaration::Class(class) = &parsed.unit.types()[0] else { panic!("class") };
        let ClassMember::Field(field) = &class.body.members[0] else { panic!("field") };
        let mut ty = &field.ty;
        let mut found = 0;
        while let Type::Array(array) = ty {
            ty = &array.element;
            found += 1;
        }
        match expected {
            Element::Class(names) => {
                let Some(class_type) = ty.as_class_type() else { panic!("class type") };
                assert_eq!(class_type_names(class_type), names, "{source}");
            }
            Element::Primitive(kind) => {
                let Type::Primitive(primitive) = ty else { panic!("primitive type") };
                assert_eq!(primitive.kind, kind, "{source}");
            }
        }
        assert_eq!(found, dimensions, "{source}");
    }
}

#[test]
fn parses_type_use_annotations() {
    let allocator = Allocator::new();
    let source = "class C<@TypeParam T extends @Nullable Object> {\
                          List<@Nullable T> field;\
                          List<@Readonly ? extends @Nullable Number> wildcard;\
                          char @Nullable [] escape() { return null; }\
                          void f(Object @Nullable ... args) {}\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };

    // Type parameter: `<@TypeParam T ...>`.
    let param = &class.type_params.as_ref().unwrap().params[0];
    assert_eq!(param.annotations.len(), 1);
    assert_eq!(type_name_parts(&param.annotations[0].name), ["TypeParam"]);

    // Type-parameter bound: `T extends @Nullable Object`.
    let Some(bound) = param.bounds[0].as_class_type() else {
        panic!("expected a class type bound");
    };
    assert_eq!(class_type_annotations(bound).len(), 1);
    assert_eq!(type_name_parts(&class_type_annotations(bound)[0].name), ["Nullable"]);

    // Type argument: `List<@Nullable T>`.
    let ClassMember::Field(field) = &class.body.members[0] else {
        panic!("expected a field");
    };
    let Some(list) = field.ty.as_class_type() else {
        panic!("expected a class type");
    };
    let Some(arg) = list.args()[0].as_class_type() else {
        panic!("expected a class type argument");
    };
    assert_eq!(class_type_annotations(arg).len(), 1);

    // Wildcard argument: `List<@Readonly ? extends @Nullable Number>`.
    let ClassMember::Field(wildcard_field) = &class.body.members[1] else {
        panic!("expected a wildcard field");
    };
    let Some(wildcard_list) = wildcard_field.ty.as_class_type() else {
        panic!("expected a class wildcard list type");
    };
    let TypeArgument::Wildcard(wildcard) = &wildcard_list.args()[0] else {
        panic!("expected an annotated wildcard argument");
    };
    assert_eq!(wildcard.annotations.len(), 1);
    assert_eq!(type_name_parts(&wildcard.annotations[0].name), ["Readonly"]);
    let Some(bound) = &wildcard.bound else {
        panic!("expected a wildcard bound");
    };
    let Some(bound_ty) = bound.ty.as_class_type() else {
        panic!("expected a class wildcard bound");
    };
    assert_eq!(class_type_annotations(bound_ty).len(), 1);

    // Array dimension: `char @Nullable []`.
    let ClassMember::Method(method) = &class.body.members[2] else {
        panic!("expected a method");
    };
    let MethodResult::Array(array) = &method.return_type else {
        panic!("expected an array return type");
    };
    assert_eq!(array.annotations.len(), 1);
    let Type::Primitive(element) = &array.element else {
        panic!("expected a primitive element type");
    };
    assert_eq!(element.kind, PrimitiveTypeKind::Char);

    // Varargs dimension: `Object @Nullable ...`.
    let ClassMember::Method(vararg_method) = &class.body.members[3] else {
        panic!("expected a method");
    };
    assert!(vararg_method.params.items[0].varargs);
    assert_eq!(vararg_method.params.items[0].varargs_annotations.len(), 1);
    // The element type is unannotated; the annotation is on the `...`.
    let Some(elem) =
        vararg_method.params.items[0].ty.as_ref().and_then(VariableType::as_class_type)
    else {
        panic!("expected a class varargs element type");
    };
    assert!(class_type_annotations(elem).is_empty());
}

#[test]
fn parses_qualified_inner_type_segments() {
    let allocator = Allocator::new();
    // Interior annotation, interior type args, and a segment bearing both.
    let source = "class C { Map.@Nullable Entry<E, Count> f; \
                      AbstractMultiset<E>.EntrySet g; \
                      java.util.@Nullable Optional<T> h; }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };

    // `Map.@Nullable Entry<E, Count>`: two segments; the annotation and the
    // arguments both sit on the trailing `Entry` segment, not the type root.
    let ClassMember::Field(entry) = &class.body.members[0] else {
        panic!("expected the entry field");
    };
    let Some(entry_ty) = entry.ty.as_class_type() else {
        panic!("expected a class type");
    };
    let segments = class_type_parts(entry_ty);
    assert_eq!(segments.len(), 2);
    assert!(class_type_annotations(segments[0]).is_empty());
    assert_eq!(segments[0].name(), "Map");
    assert_eq!(class_type_annotations(segments[1]).len(), 1);
    assert_eq!(type_name_parts(&class_type_annotations(segments[1])[0].name), ["Nullable"]);
    assert_eq!(segments[1].name(), "Entry");
    assert_eq!(entry_ty.args().len(), 2);

    // `AbstractMultiset<E>.EntrySet`: type arguments on an *interior*
    // segment, which the old flat name could not express.
    let ClassMember::Field(set) = &class.body.members[1] else {
        panic!("expected the set field");
    };
    let Some(set_ty) = set.ty.as_class_type() else {
        panic!("expected a class type");
    };
    let segments = class_type_parts(set_ty);
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0].name(), "AbstractMultiset");
    assert_eq!(segments[0].args().len(), 1);
    assert_eq!(segments[1].name(), "EntrySet");
    assert!(segments[1].args().is_empty());

    // `java.util.@Nullable Optional<T>`: a package-qualified inner type.
    let ClassMember::Field(opt) = &class.body.members[2] else {
        panic!("expected the optional field");
    };
    let Some(opt_ty) = opt.ty.as_class_type() else {
        panic!("expected a class type");
    };
    assert_eq!(class_type_names(opt_ty), ["java", "util", "Optional"]);
    assert_eq!(class_type_annotations(class_type_parts(opt_ty)[2]).len(), 1);
    assert_eq!(opt_ty.args().len(), 1);
}

#[test]
fn interior_type_annotations_continue_type_lookahead() {
    // Each statement is recognized by a lookahead that must read past an
    // annotated later segment (`Map.@A Entry`) to see a type.
    let allocator = Allocator::new();
    let source = "class C { void f(Object o) {\
                      java.util.Map.@A Entry<K, V> e = null;\
                      Object c = (java.util.Map.@A Entry<K, V>) o;\
                      List<Map.@A Entry<K, V>> list = null;\
                      for (Map.@A Entry<K, V> x : list) {}\
                      boolean b = o instanceof Map.@A Entry<?, ?> m;\
                  } }";
    let (statements, _) = first_method_statements(&allocator, source);

    let Statement::LocalVariable(local) = &statements[0] else { panic!("local") };
    let Some(entry) = local.ty.as_class_type() else { panic!("class type") };
    assert_eq!(class_type_names(entry), ["java", "util", "Map", "Entry"]);
    assert_eq!(class_type_annotations(class_type_parts(entry)[3]).len(), 1);

    let Statement::LocalVariable(cast) = &statements[1] else { panic!("local") };
    let Some(VariableInitializer::Cast(cast)) = &cast.declarators[0].init else { panic!("cast") };
    assert!(matches!(&cast.operand, Expression::Identifier(_)));

    // The type argument lookahead reads the annotated argument.
    assert!(matches!(&statements[2], Statement::LocalVariable(_)));
    assert!(matches!(&statements[3], Statement::EnhancedFor(_)));

    let Statement::LocalVariable(test) = &statements[4] else { panic!("local") };
    let Some(VariableInitializer::Instanceof(test)) = &test.declarators[0].init else {
        panic!("instanceof")
    };
    assert!(matches!(&test.rhs, InstanceofRhs::TypePattern(_)));
}

#[test]
fn parses_dimensions_after_declarator_names() {
    let allocator = Allocator::new();
    let source = "class A {\
                      int a, b[], c @T [] [] = {};\
                      void f(String args[], int... rest) {\
                          int x[] = null;\
                          for (final int e[] : m) {}\
                          for (int i = 0, y[] = null;;) {}\
                          g((int p[]) -> 0);\
                      }\
                  }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let TypeDeclaration::Class(class) = &result.unit.types()[0] else { panic!("class") };

    let ClassMember::Field(field) = &class.body.members[0] else { panic!("field") };
    let dims: std::vec::Vec<_> = field.declarators.iter().map(|d| d.dims.len()).collect();
    assert_eq!(dims, [0, 1, 2]);
    assert_eq!(field.declarators[1].span.source_text(source), "b[]");
    assert_eq!(type_name_parts(&field.declarators[2].dims[0].annotations[0].name), ["T"]);
    let Type::Primitive(primitive) = &field.ty else { panic!("primitive type") };
    assert_eq!(primitive.kind, PrimitiveTypeKind::Int);

    let ClassMember::Method(method) = &class.body.members[1] else { panic!("method") };
    assert_eq!(method.params.items[0].span.source_text(source), "String args[]");
    assert!(method.params.items[1].dims.is_empty());
    let statements = &method.body.as_ref().unwrap().statements;
    let Statement::LocalVariable(local) = &statements[0] else { panic!("local") };
    assert_eq!(local.declarators[0].dims.len(), 1);
    let Statement::EnhancedFor(each) = &statements[1] else { panic!("enhanced for") };
    assert_eq!(each.declaration.declarators[0].dims.len(), 1);
    let Statement::For(counted) = &statements[2] else { panic!("for") };
    let Some(ForInit::Declaration(init)) = &counted.init else { panic!("for declaration") };
    assert_eq!(init.declarators[1].dims.len(), 1);
    let Statement::Expression(call) = &statements[3] else { panic!("call") };
    let Expression::MethodCall(call) = &call.expression else { panic!("call") };
    let Expression::Lambda(lambda) = &call.args[0] else { panic!("lambda") };
    assert_eq!(lambda.params.items[0].span.source_text(source), "int p[]");
}

#[test]
fn parses_dimensions_after_method_parameter_lists() {
    let allocator = Allocator::new();
    let source = "interface I { int f()[]; <X> X g() @T [] [] throws E; }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let TypeDeclaration::Interface(interface) = &result.unit.types()[0] else {
        panic!("interface")
    };
    let InterfaceMember::Method(f) = &interface.body.members[0] else { panic!("method") };
    assert_eq!(f.post_paren_dimensions.len(), 1);
    let InterfaceMember::Method(g) = &interface.body.members[1] else { panic!("method") };
    assert_eq!(g.post_paren_dimensions.len(), 2);
    assert_eq!(g.post_paren_dimensions[0].annotations.len(), 1);
    assert_eq!(g.throws.len(), 1);
}

#[test]
fn parses_class_extends_and_implements_clauses() {
    let allocator = Allocator::new();
    let source = "public class Service<T, H extends api.Handler<? super Event> & java.io.Closeable> \
                      extends base.AbstractService<T> implements api.Registry<H> {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(class.name.name, "Service");
    let type_params = class.type_params.as_ref().unwrap();
    assert_eq!(
        &source[type_params.span.start as usize..type_params.span.end as usize],
        "<T, H extends api.Handler<? super Event> & java.io.Closeable>"
    );
    let params = &type_params.params;
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].name.name, "T");
    assert!(params[0].bounds.is_empty());
    assert_eq!(params[1].name.name, "H");
    assert_eq!(params[1].bounds.len(), 2);

    let Some(superclass) = class.superclass.as_ref().and_then(Type::as_class_type) else {
        panic!("expected a superclass");
    };
    assert_eq!(class_type_names(superclass), ["base", "AbstractService"]);
    assert_eq!(superclass.args().len(), 1);

    assert_eq!(class.interfaces.len(), 1);
    let Some(registry) = class.interfaces[0].as_class_type() else {
        panic!("expected the first interface");
    };
    assert_eq!(class_type_names(registry), ["api", "Registry"]);
    assert_eq!(registry.args().len(), 1);
}

#[test]
fn parses_generic_wildcard_and_array_types() {
    let allocator = Allocator::new();
    let source = "class Types {\
                          java.util.Map<String, List<Integer>> byName;\
                          String[] names;\
                          List<?> wildcard;\
                          List<? extends Number> upper;\
                          List<? super T> lower;\
                          void set(final List<?> values, int[][] matrix) {\
                              List<? extends String>[] copy = values;\
                          }\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(class.body.members.len(), 6);

    let ClassMember::Field(map_field) = &class.body.members[0] else {
        panic!("expected the map field");
    };
    let Some(map_ty) = map_field.ty.as_class_type() else {
        panic!("expected a class generic type");
    };
    assert_eq!(map_ty.args().len(), 2);
    let Some(list_ty) = map_ty.args()[1].as_class_type() else {
        panic!("expected a nested generic type argument");
    };
    assert_eq!(list_ty.args().len(), 1);

    let ClassMember::Field(array_field) = &class.body.members[1] else {
        panic!("expected the array field");
    };
    assert!(matches!(&array_field.ty, Type::Array(_)));

    let ClassMember::Field(wildcard_field) = &class.body.members[2] else {
        panic!("expected the wildcard field");
    };
    let Some(wildcard_ty) = wildcard_field.ty.as_class_type() else {
        panic!("expected a class wildcard type");
    };
    let TypeArgument::Wildcard(unbounded) = &wildcard_ty.args()[0] else {
        panic!("expected a wildcard type argument");
    };
    assert!(unbounded.bound.is_none());

    let ClassMember::Field(upper_field) = &class.body.members[3] else {
        panic!("expected the upper-bounded wildcard field");
    };
    let Some(upper_ty) = upper_field.ty.as_class_type() else {
        panic!("expected a class wildcard type");
    };
    let TypeArgument::Wildcard(upper) = &upper_ty.args()[0] else {
        panic!("expected a wildcard type argument");
    };
    let upper_bound = upper.bound.as_ref().unwrap();
    assert_eq!(upper_bound.kind, WildcardBoundKind::Extends);

    let ClassMember::Field(lower_field) = &class.body.members[4] else {
        panic!("expected the lower-bounded wildcard field");
    };
    let Some(lower_ty) = lower_field.ty.as_class_type() else {
        panic!("expected a class wildcard type");
    };
    let TypeArgument::Wildcard(lower) = &lower_ty.args()[0] else {
        panic!("expected a wildcard type argument");
    };
    let lower_bound = lower.bound.as_ref().unwrap();
    assert_eq!(lower_bound.kind, WildcardBoundKind::Super);

    let ClassMember::Method(method) = &class.body.members[5] else {
        panic!("expected the method");
    };
    assert_eq!(method.params.items.len(), 2);
    assert!(matches!(&method.params.items[1].ty, Some(VariableType::Array(_))));
    let Statement::LocalVariable(local) = &method.body.as_ref().unwrap().statements[0] else {
        panic!("expected a local variable");
    };
    assert!(matches!(&local.ty, VariableType::Array(_)));
}

#[test]
fn parses_var_in_inferred_variable_type_positions() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { \
                 var message = read(); \
                 final var count = size(); \
                 for (var item : items) { use(item); } \
                 try (var in = open()) { use(in); } \
                 mapper = (var left, @Nonnull var right) -> left; \
             } }",
    );
    assert_eq!(statements.len(), 5);

    let Statement::LocalVariable(first_local) = &statements[0] else {
        panic!("expected a local variable declaration");
    };
    assert!(matches!(&first_local.ty, VariableType::Var(_)));
    assert_eq!(first_local.declarators[0].name.name, "message");

    let Statement::LocalVariable(final_local) = &statements[1] else {
        panic!("expected a local variable declaration");
    };
    assert_eq!(final_local.modifiers.len(), 1);
    assert!(matches!(&final_local.ty, VariableType::Var(_)));

    let Statement::EnhancedFor(for_each) = &statements[2] else {
        panic!("expected an enhanced for statement");
    };
    assert!(matches!(&for_each.declaration.ty, VariableType::Var(_)));
    assert_eq!(for_each.declaration.declarators[0].name.name, "item");

    let Statement::Try(try_stmt) = &statements[3] else {
        panic!("expected a try statement");
    };
    let Resource::Declaration(resource) = &try_stmt.resources[0] else {
        panic!("expected a resource declaration");
    };
    assert!(matches!(&resource.ty, VariableType::Var(_)));
    assert_eq!(resource.declarators[0].name.name, "in");

    let Expression::Lambda(lambda) = assignment_value(&statements[4]) else {
        panic!("expected a lambda assignment value");
    };
    assert_eq!(lambda.params.items.len(), 2);
    assert!(matches!(&lambda.params.items[0].ty, Some(VariableType::Var(_))));
    assert!(matches!(&lambda.params.items[1].ty, Some(VariableType::Var(_))));
    assert_eq!(lambda.params.items[1].modifiers.len(), 1);
}

#[test]
fn parses_unnamed_variables_in_declarator_id_positions() {
    let allocator = Allocator::new();
    let source = "class A { \
                      int _ = 1; \
                      void f(String _) { \
                          int _ = 0; \
                          for (String _ : values) { use(); } \
                          try (var _ = open()) { read(); } \
                          try { risky(); } catch (IOException _) { recover(); } \
                          r = _ -> 1; \
                          s = (String _) -> 2; \
                          t = (_, value) -> value; \
                      } \
                  }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Field(field) = &class.body.members[0] else {
        panic!("expected a field");
    };
    assert_eq!(field.declarators[0].name.name, "_");

    let ClassMember::Method(method) = &class.body.members[1] else {
        panic!("expected a method");
    };
    assert_eq!(method.params.items[0].name.name, "_");
    let statements = &method.body.as_ref().unwrap().statements;

    let Statement::LocalVariable(local) = &statements[0] else {
        panic!("expected a local variable declaration");
    };
    assert_eq!(local.declarators[0].name.name, "_");

    let Statement::EnhancedFor(for_each) = &statements[1] else {
        panic!("expected an enhanced for statement");
    };
    assert_eq!(for_each.declaration.declarators[0].name.name, "_");

    let Statement::Try(resource_try) = &statements[2] else {
        panic!("expected a try-with-resources statement");
    };
    let Resource::Declaration(resource) = &resource_try.resources[0] else {
        panic!("expected a resource declaration");
    };
    assert_eq!(resource.declarators[0].name.name, "_");

    let Statement::Try(catch_try) = &statements[3] else {
        panic!("expected a try-catch statement");
    };
    assert_eq!(catch_try.catches[0].name.name, "_");

    let Expression::Lambda(concise) = assignment_value(&statements[4]) else {
        panic!("expected a concise lambda");
    };
    assert_eq!(concise.params.items[0].name.name, "_");
    assert!(concise.params.items[0].ty.is_none());

    let Expression::Lambda(typed) = assignment_value(&statements[5]) else {
        panic!("expected a typed lambda");
    };
    assert_eq!(typed.params.items[0].name.name, "_");
    assert!(typed.params.items[0].ty.is_some());

    let Expression::Lambda(parenthesized_concise) = assignment_value(&statements[6]) else {
        panic!("expected a parenthesized concise lambda");
    };
    assert_eq!(parenthesized_concise.params.items.len(), 2);
    assert_eq!(parenthesized_concise.params.items[0].name.name, "_");
    assert!(parenthesized_concise.params.items[0].ty.is_none());
    assert_eq!(parenthesized_concise.params.items[1].name.name, "value");
    assert!(parenthesized_concise.params.items[1].ty.is_none());
}
