use robusta_syntax::operator::UpdateOp;

use super::*;

fn expression_stmt<'a>(stmt: &'a Statement<'a>) -> &'a Expression<'a> {
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    &expr_stmt.expression
}

fn method_reference_assignment<'a>(stmt: &'a Statement<'a>) -> &'a MethodReference<'a> {
    let Expression::MethodReference(reference) = assignment_value(stmt) else {
        panic!("expected a method reference");
    };
    reference
}

#[test]
fn parses_text_block_as_raw_string_literal() {
    let allocator = Allocator::new();
    let source = "class Texts { String body = \"\"\"\nhello\n\"\"\"; }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Field(field) = &class.body.members[0] else {
        panic!("expected a field");
    };
    let Some(VariableInitializer::StringLiteral(literal)) = &field.declarators[0].init else {
        panic!("expected a text block string literal initializer");
    };
    assert_eq!(literal.raw, "\"\"\"\nhello\n\"\"\"");
}

#[test]
fn multiplication_binds_tighter_than_addition() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { int f() { return 1 + 2 * 3; } }");
    let Statement::Return(ret) = stmt else {
        panic!("expected a return");
    };
    let Some(Expression::Binary(add)) = &ret.value else {
        panic!("expected a binary expression");
    };
    assert_eq!(add.op, BinaryOp::Add);
    assert!(matches!(&add.lhs, Expression::IntLiteral(_)));
    let Expression::Binary(mul) = &add.rhs else {
        panic!("expected the multiplication on the right");
    };
    assert_eq!(mul.op, BinaryOp::Mul);
}

#[test]
fn same_precedence_associates_left() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { int f() { return a - b - c; } }");
    let Statement::Return(ret) = stmt else {
        panic!("expected a return");
    };
    let Some(Expression::Binary(outer)) = &ret.value else {
        panic!("expected a binary expression");
    };
    assert_eq!(outer.op, BinaryOp::Sub);
    assert!(matches!(&outer.lhs, Expression::Binary(_)));
    assert!(matches!(&outer.rhs, Expression::Identifier(_)));
}

#[test]
fn constructor_field_assignment() {
    let allocator = Allocator::new();
    let source = "class A { A(int x) { this.x = x; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Constructor(ctor) = &class.body.members[0] else {
        panic!("expected a constructor");
    };
    let Statement::Expression(stmt) = &ctor.body.statements[0] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &stmt.expression else {
        panic!("expected an assignment");
    };
    assert_eq!(assignment.op, AssignOp::Assign);
    let AssignmentTarget::FieldAccess(access) = &assignment.target else {
        panic!("expected a field access target");
    };
    assert!(matches!(access.target, Expression::This(_)));
    assert_eq!(access.name.name, "x");
}

#[test]
fn assignment_is_right_associative() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { a = b = c; } }");
    let Statement::Expression(outer) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &outer.expression else {
        panic!("expected an assignment");
    };
    assert!(matches!(&assignment.value, Expression::Assignment(_)));
}

#[test]
fn assignment_target_keeps_its_parentheses() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f(int x, int[] a) { x = 1; ((x)) = 2; (a[0]) = 3; } }",
    );
    let Expression::Assignment(plain) = expression_stmt(&statements[0]) else {
        panic!("expected an assignment");
    };
    assert!(matches!(&plain.target, AssignmentTarget::Identifier(name) if name.name == "x"));
    let Expression::Assignment(name) = expression_stmt(&statements[1]) else {
        panic!("expected an assignment");
    };
    let AssignmentTarget::Parenthesized(outer) = &name.target else {
        panic!("expected a parenthesized target");
    };
    assert!(matches!(&outer.expression, Expression::Parenthesized(_)));
    assert_eq!(name.target.get_identifier_reference().map(|name| name.name), Some("x"));
    let Expression::Assignment(element) = expression_stmt(&statements[2]) else {
        panic!("expected an assignment");
    };
    assert!(matches!(&element.target, AssignmentTarget::Parenthesized(_)));
    assert!(element.target.get_identifier_reference().is_none());
}

#[test]
fn qualified_calls_split_receiver_and_method() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { a.b.f(x, 1); } }");
    let Statement::Expression(stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::MethodCall(call) = &stmt.expression else {
        panic!("expected a call");
    };
    assert_eq!(call.name(), "f");
    assert_eq!(call.args.len(), 2);
    let Some(receiver) = call.target() else {
        panic!("expected a name receiver");
    };
    assert_eq!(expression_names(receiver), ["a", "b"]);
}

#[test]
fn parses_explicit_method_invocation_type_arguments() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 java.util.Collections.<String>emptyList(); \
                 this.<Integer>accept(1); \
                 result = source.<java.util.Map<String, ? extends Number>>convert(value).size(); \
             } }",
    );
    let Statement::Expression(first_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::MethodCall(empty_list) = &first_stmt.expression else {
        panic!("expected a generic method call");
    };
    assert_eq!(empty_list.name(), "emptyList");
    assert_eq!(empty_list.type_args().len(), 1);
    let Some(receiver) = empty_list.target() else {
        panic!("expected a named receiver");
    };
    assert_eq!(expression_names(receiver), ["java", "util", "Collections"]);

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;

    let Statement::Expression(this_stmt) = &statements[1] else {
        panic!("expected an expression statement");
    };
    let Expression::MethodCall(accept) = &this_stmt.expression else {
        panic!("expected a generic method call");
    };
    assert!(matches!(accept.target(), Some(Expression::This(_))));
    assert_eq!(accept.type_args().len(), 1);

    let Statement::Expression(assign_stmt) = &statements[2] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &assign_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::MethodCall(size) = &assignment.value else {
        panic!("expected the chained size call");
    };
    let Some(Expression::MethodCall(convert)) = size.target() else {
        panic!("expected the generic convert call receiver");
    };
    assert_eq!(convert.name(), "convert");
    assert_eq!(convert.type_args().len(), 1);
}

#[test]
fn call_chains_and_field_access_after_calls() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { int f() { return g().h().i; } }");
    let Statement::Return(ret) = stmt else {
        panic!("expected a return");
    };
    let Some(Expression::FieldAccess(access)) = &ret.value else {
        panic!("expected a field access");
    };
    assert_eq!(access.name.name, "i");
    let Expression::MethodCall(h) = &access.target else {
        panic!("expected a call");
    };
    assert_eq!(h.name(), "h");
    assert!(matches!(h.target(), Some(Expression::MethodCall(_))));
}

#[test]
fn super_field_and_method_access() {
    let allocator = Allocator::new();
    let source = "class A { void f() { super.reset(); value = super.value; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;
    let Statement::Expression(call_stmt) = &statements[0] else {
        panic!("expected a call statement");
    };
    let Expression::MethodCall(call) = &call_stmt.expression else {
        panic!("expected a method call");
    };
    assert!(matches!(call.target(), Some(Expression::Super(_))));

    let Statement::Expression(assign_stmt) = &statements[1] else {
        panic!("expected an assignment statement");
    };
    let Expression::Assignment(assignment) = &assign_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::FieldAccess(access) = &assignment.value else {
        panic!("expected a field access value");
    };
    assert!(matches!(access.target, Expression::Super(_)));
}

#[test]
fn qualified_super_field_and_method_access() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { \
                 Outer.super.reset(); \
                 value = Outer.super.value; \
                 result = a.b.Outer.super.<String>adapt(input).size(); \
             } }",
    );
    assert_eq!(statements.len(), 3);

    let Statement::Expression(call_stmt) = &statements[0] else {
        panic!("expected a call statement");
    };
    let Expression::MethodCall(call) = &call_stmt.expression else {
        panic!("expected a method call");
    };
    let Some(Expression::QualifiedSuper(target)) = call.target() else {
        panic!("expected a qualified-super target");
    };
    assert_eq!(type_name_parts(&target.qualifier), ["Outer"]);

    let Expression::FieldAccess(access) = assignment_value(&statements[1]) else {
        panic!("expected a field access value");
    };
    assert!(matches!(&access.target, Expression::QualifiedSuper(_)));

    let Expression::MethodCall(size) = assignment_value(&statements[2]) else {
        panic!("expected a chained call");
    };
    let Some(Expression::MethodCall(adapt)) = size.target() else {
        panic!("expected the generic qualified-super call receiver");
    };
    assert_eq!(adapt.type_args().len(), 1);
    let Some(Expression::QualifiedSuper(target)) = adapt.target() else {
        panic!("expected a qualified-super target");
    };
    assert_eq!(type_name_parts(&target.qualifier), ["a", "b", "Outer"]);
}

#[test]
fn parses_class_instance_creations() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 java.util.List<String> xs = new java.util.ArrayList<>(); \
                 Point p = new geo.Point<Integer>(1, 2); \
                 new Runner(p).start(); \
                 register(new Handler()); \
             } }",
    );
    let Statement::LocalVariable(diamond_local) = stmt else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::ClassInstanceCreation(diamond)) =
        &diamond_local.declarators[0].init
    else {
        panic!("expected a class instance creation initializer");
    };
    assert!(diamond.args.is_empty());
    assert_eq!(class_type_names(&diamond.ty), ["java", "util", "ArrayList"]);
    let empty = diamond.ty.type_arguments().unwrap();
    assert!(empty.is_diamond());
    assert_eq!(empty.span.end - empty.span.start, 2);

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;

    let Statement::LocalVariable(explicit_local) = &statements[1] else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::ClassInstanceCreation(explicit)) =
        &explicit_local.declarators[0].init
    else {
        panic!("expected a class instance creation initializer");
    };
    assert!(!explicit.ty.type_arguments().unwrap().is_diamond());
    assert_eq!(explicit.args.len(), 2);
    assert_eq!(explicit.ty.args().len(), 1);

    let Statement::Expression(chained_stmt) = &statements[2] else {
        panic!("expected an expression statement");
    };
    let Expression::MethodCall(chained) = &chained_stmt.expression else {
        panic!("expected a chained call");
    };
    assert_eq!(chained.name(), "start");
    assert!(matches!(chained.target(), Some(Expression::ClassInstanceCreation(_))));

    let Statement::Expression(arg_stmt) = &statements[3] else {
        panic!("expected an expression statement");
    };
    let Expression::MethodCall(register) = &arg_stmt.expression else {
        panic!("expected a call");
    };
    assert!(matches!(register.args[0], Expression::ClassInstanceCreation(_)));
}

#[test]
fn parses_qualified_inner_class_creations() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { \
                 outer.new Inner(); \
                 this.new Node<String>(value); \
                 a.b.getScope().new Entry() {}; \
                 outer.new Inner().start(); \
             } }",
    );
    assert_eq!(statements.len(), 4);

    // `outer.new Inner()` — a plain expression-name qualifier.
    let Statement::Expression(stmt) = &statements[0] else {
        panic!("expected an expression statement");
    };
    let Expression::QualifiedClassInstanceCreation(creation) = &stmt.expression else {
        panic!("expected a qualified creation");
    };
    let Expression::Identifier(qualifier) = &creation.qualifier else {
        panic!("expected an expression-name qualifier");
    };
    assert_eq!(qualifier.name, "outer");
    assert!(creation.body.is_none());

    // `this.new Node<String>(value)` — a `this` qualifier, explicit type
    // arguments and a constructor argument.
    let Expression::QualifiedClassInstanceCreation(node) = expression_stmt(&statements[1]) else {
        panic!("expected a qualified creation");
    };
    assert!(matches!(node.qualifier, Expression::This(_)));
    assert_eq!(node.type_args.as_ref().unwrap().args.len(), 1);
    assert_eq!(node.args.len(), 1);

    // `a.b.getScope().new Entry() {}` — a method-call qualifier and an
    // anonymous class body on the qualified creation.
    let Expression::QualifiedClassInstanceCreation(entry) = expression_stmt(&statements[2]) else {
        panic!("expected a qualified creation");
    };
    assert!(matches!(entry.qualifier, Expression::MethodCall(_)));
    assert!(entry.body.as_ref().expect("expected a body").members.is_empty());

    // `outer.new Inner().start()` — the qualified creation is itself the
    // receiver of a trailing call.
    let Expression::MethodCall(started) = expression_stmt(&statements[3]) else {
        panic!("expected a chained call");
    };
    assert_eq!(started.name(), "start");
    assert!(matches!(started.target(), Some(Expression::QualifiedClassInstanceCreation(_))));
}

#[test]
fn parses_class_instance_creation_type_arguments() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { \
                 Box box = new <String> Box(value); \
                 Holder holder = new <java.util.List<String>> Holder<>(value); \
                 outer.new <String> Inner(value); \
             } }",
    );
    assert_eq!(statements.len(), 3);

    let Statement::LocalVariable(box_local) = &statements[0] else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::ClassInstanceCreation(boxed)) = &box_local.declarators[0].init
    else {
        panic!("expected a class instance creation initializer");
    };
    assert_eq!(boxed.constructor_type_args.as_ref().unwrap().args.len(), 1);
    assert!(boxed.ty.type_arguments().is_none());
    assert_eq!(boxed.args.len(), 1);

    let Statement::LocalVariable(holder_local) = &statements[1] else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::ClassInstanceCreation(holder)) =
        &holder_local.declarators[0].init
    else {
        panic!("expected a class instance creation initializer");
    };
    assert_eq!(holder.constructor_type_args.as_ref().unwrap().args.len(), 1);
    assert!(holder.ty.type_arguments().unwrap().is_diamond());

    let Expression::QualifiedClassInstanceCreation(inner) = expression_stmt(&statements[2]) else {
        panic!("expected a qualified creation");
    };
    assert!(matches!(inner.qualifier, Expression::Identifier(_)));
    assert_eq!(inner.constructor_type_args.as_ref().unwrap().args.len(), 1);
    assert_eq!(inner.args.len(), 1);
}

#[test]
fn parses_anonymous_class_bodies() {
    let allocator = Allocator::new();
    let (stmt, _unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 Runnable r = new Runnable() { \
                     public void run() { step(); } \
                 }; \
             } }",
    );
    let Statement::LocalVariable(local) = stmt else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::ClassInstanceCreation(creation)) = &local.declarators[0].init
    else {
        panic!("expected a class instance creation initializer");
    };
    assert!(creation.args.is_empty());
    let body = creation.body.as_ref().expect("expected an anonymous class body");
    assert_eq!(body.members.len(), 1);
    assert!(matches!(body.members[0], ClassMember::Method(_)));
}

#[test]
fn parses_empty_anonymous_class_body_and_ctor_args() {
    let allocator = Allocator::new();
    let (stmt, _unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 Thread t = new Thread(task) {}; \
             } }",
    );
    let Statement::LocalVariable(local) = stmt else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::ClassInstanceCreation(creation)) = &local.declarators[0].init
    else {
        panic!("expected a class instance creation initializer");
    };
    assert_eq!(creation.args.len(), 1);
    // `Some` with no members distinguishes `new Thread(task) {}` from a
    // plain `new Thread(task)`.
    let body = creation.body.as_ref().expect("expected an (empty) anonymous class body");
    assert!(body.members.is_empty());
}

#[test]
fn parses_array_creations_accesses_and_initializers() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 int[][] matrix = new int[rows][cols + 1]; \
                 int[] mixed = new int[2][]; \
                 String[] names = {\"a\", \"b\",}; \
                 int[][] table = new int[][] {{1, 2}, {3}}; \
                 matrix[0][1] = names[i].length(); \
             } }",
    );
    let Statement::LocalVariable(matrix_local) = stmt else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::ArrayCreation(sized)) = &matrix_local.declarators[0].init else {
        panic!("expected an array creation initializer");
    };
    let Type::Primitive(sized_element) = &sized.element else {
        panic!("expected a primitive element type");
    };
    assert_eq!(sized_element.kind, PrimitiveTypeKind::Int);
    assert_eq!(sized.dims.len(), 2);
    assert!(sized.dims[0].length.is_some());
    assert!(matches!(sized.dims[1].length, Some(Expression::Binary(_))));
    assert!(sized.init.is_none());

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;

    let Statement::LocalVariable(mixed_local) = &statements[1] else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::ArrayCreation(mixed)) = &mixed_local.declarators[0].init else {
        panic!("expected an array creation initializer");
    };
    assert!(mixed.dims[0].length.is_some());
    assert!(mixed.dims[1].length.is_none());
    assert!(mixed.init.is_none());

    let Statement::LocalVariable(names_local) = &statements[2] else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::Array(names_init)) = &names_local.declarators[0].init else {
        panic!("expected an array initializer");
    };
    assert_eq!(names_init.elements.len(), 2);

    let Statement::LocalVariable(table_local) = &statements[3] else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::ArrayCreation(with_init)) = &table_local.declarators[0].init
    else {
        panic!("expected an array creation initializer");
    };
    assert_eq!(with_init.dims.len(), 2);
    assert!(with_init.dims.iter().all(|dim| dim.length.is_none()));
    let init = with_init.init.as_ref().unwrap();
    assert_eq!(init.elements.len(), 2);
    let VariableInitializer::Array(first_row) = &init.elements[0] else {
        panic!("expected a nested array initializer");
    };
    assert_eq!(first_row.elements.len(), 2);

    let Statement::Expression(assign_stmt) = &statements[4] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &assign_stmt.expression else {
        panic!("expected an assignment");
    };
    let AssignmentTarget::ArrayAccess(outer) = &assignment.target else {
        panic!("expected an array access target");
    };
    assert!(matches!(&outer.array, Expression::ArrayAccess(_)));
    let Expression::MethodCall(length_call) = &assignment.value else {
        panic!("expected a call value");
    };
    assert!(matches!(length_call.target(), Some(Expression::ArrayAccess(_))));
}

#[test]
fn parses_annotated_array_creation_dimensions() {
    let allocator = Allocator::new();
    let source = "class A { void f() { \
                      i = new @Y int @Y [4]; \
                      j = new @Y int @Y [] { 10 }; \
                      t = new String @Y [5] @Y @Z []; \
                  } }";
    let (statements, _) = first_method_statements(&allocator, source);

    let Expression::ArrayCreation(sized) = assignment_value(&statements[0]) else {
        panic!("expected an array creation");
    };
    let Type::Primitive(element) = &sized.element else {
        panic!("expected a primitive element type");
    };
    assert_eq!(element.kind, PrimitiveTypeKind::Int);
    assert_eq!(element.annotations.len(), 1);
    assert_eq!(sized.dims[0].annotations.len(), 1);
    assert_eq!(sized.dims[0].span.source_text(source), "@Y [4]");
    assert!(sized.dims[0].length.is_some());

    let Expression::ArrayCreation(with_init) = assignment_value(&statements[1]) else {
        panic!("expected an array creation");
    };
    assert_eq!(with_init.dims[0].annotations.len(), 1);
    assert!(with_init.dims[0].length.is_none());
    assert!(with_init.init.is_some());

    let Expression::ArrayCreation(class_element) = assignment_value(&statements[2]) else {
        panic!("expected an array creation");
    };
    assert_eq!(class_element.dims[0].annotations.len(), 1);
    assert_eq!(class_element.dims[1].annotations.len(), 2);
    assert!(class_element.dims[1].length.is_none());
}

#[test]
fn parses_class_literals() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { \
                 string = String.class; \
                 array = int[][].class; \
                 none = void.class; \
                 name = java.util.Map.Entry.class.getName(); \
             } }",
    );
    assert_eq!(statements.len(), 4);

    let Expression::ClassLiteral(string_literal) = assignment_value(&statements[0]) else {
        panic!("expected a class literal");
    };
    let Some(string_ty) = string_literal.ty.as_class_type() else {
        panic!("expected a class literal type");
    };
    assert_eq!(class_type_names(string_ty), ["String"]);

    let Expression::ClassLiteral(array_literal) = assignment_value(&statements[1]) else {
        panic!("expected an array class literal");
    };
    assert!(matches!(&array_literal.ty, MethodResult::Array(_)));

    let Expression::ClassLiteral(void_literal) = assignment_value(&statements[2]) else {
        panic!("expected a void class literal");
    };
    assert!(matches!(&void_literal.ty, MethodResult::Void(_)));

    let Expression::MethodCall(get_name) = assignment_value(&statements[3]) else {
        panic!("expected a chained call");
    };
    assert!(matches!(get_name.target(), Some(Expression::ClassLiteral(_))));
}

#[test]
fn parses_qualified_this() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { \
                 self = Outer.this; \
                 nested = a.b.Outer.this; \
                 field = Outer.this.value; \
             } }",
    );
    assert_eq!(statements.len(), 3);

    let Expression::QualifiedThis(plain) = assignment_value(&statements[0]) else {
        panic!("expected a qualified this");
    };
    assert_eq!(type_name_parts(&plain.qualifier), ["Outer"]);

    let Expression::QualifiedThis(nested) = assignment_value(&statements[1]) else {
        panic!("expected a qualified this");
    };
    assert_eq!(type_name_parts(&nested.qualifier), ["a", "b", "Outer"]);

    let Expression::FieldAccess(access) = assignment_value(&statements[2]) else {
        panic!("expected a field access on a qualified this");
    };
    assert!(matches!(&access.target, Expression::QualifiedThis(_)));
}

#[test]
fn conditional_expression_nests_right() {
    let allocator = Allocator::new();
    let (stmt, _) =
        first_statement(&allocator, "class A { int f() { return a > 0 ? 1 : a == 0 ? 0 : -1; } }");
    let Statement::Return(ret) = stmt else {
        panic!("expected a return");
    };
    let Some(Expression::Conditional(cond)) = &ret.value else {
        panic!("expected a conditional");
    };
    assert!(matches!(&cond.condition, Expression::Binary(_)));
    assert!(matches!(&cond.when_false, Expression::Conditional(_)));
}

#[test]
fn qualified_type_declaration_vs_qualified_call_statement() {
    let allocator = Allocator::new();
    let (stmt, unit) =
        first_statement(&allocator, "class A { void f() { a.b.C x = null; a.b.f(); x = 1; } }");
    assert!(matches!(stmt, Statement::LocalVariable(_)));
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;
    assert_eq!(statements.len(), 3);
    assert!(matches!(&statements[1], Statement::Expression(_)));
    assert!(matches!(&statements[2], Statement::Expression(_)));
}

#[test]
fn parses_prefix_and_postfix_increments() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 i++; \
                 --this.count; \
                 cells[i]--; \
                 for (int k = 0; k < n; k++) use(k); \
             } }",
    );
    let Statement::Expression(post_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Update(post) = &post_stmt.expression else {
        panic!("expected an update expression");
    };
    assert_eq!(post.op, UpdateOp::Increment);
    assert!(!post.prefix);
    assert!(matches!(&post.target, AssignmentTarget::Identifier(_)));

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;

    let Statement::Expression(pre_stmt) = &statements[1] else {
        panic!("expected an expression statement");
    };
    let Expression::Update(pre) = &pre_stmt.expression else {
        panic!("expected an update expression");
    };
    assert_eq!(pre.op, UpdateOp::Decrement);
    assert!(pre.prefix);
    assert!(matches!(&pre.target, AssignmentTarget::FieldAccess(_)));

    let Statement::Expression(element_stmt) = &statements[2] else {
        panic!("expected an expression statement");
    };
    let Expression::Update(element) = &element_stmt.expression else {
        panic!("expected an update expression");
    };
    assert_eq!(element.op, UpdateOp::Decrement);
    assert!(!element.prefix);
    assert!(matches!(&element.target, AssignmentTarget::ArrayAccess(_)));

    let Statement::For(loop_stmt) = &statements[3] else {
        panic!("expected a for");
    };
    assert!(matches!(&loop_stmt.update.as_ref().unwrap().expressions[0], Expression::Update(_)));
}

#[test]
fn update_target_keeps_its_parentheses() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f(int x, int[] a) { (x)++; ++(a[0]); } }",
    );
    let Expression::Update(postfix) = expression_stmt(&statements[0]) else {
        panic!("expected an update expression");
    };
    let AssignmentTarget::Parenthesized(name) = &postfix.target else {
        panic!("expected a parenthesized target");
    };
    assert!(matches!(&name.expression, Expression::Identifier(_)));
    let Expression::Update(prefix) = expression_stmt(&statements[1]) else {
        panic!("expected an update expression");
    };
    let AssignmentTarget::Parenthesized(element) = &prefix.target else {
        panic!("expected a parenthesized target");
    };
    assert!(matches!(&element.expression, Expression::ArrayAccess(_)));
}

#[test]
fn postfix_increment_binds_tighter_than_addition() {
    // `x+++y` lexes as `x ++ + y` (maximal munch) and parses as
    // `(x++) + y`.
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { r = x+++y; } }");
    let Statement::Expression(stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Binary(add) = &assignment.value else {
        panic!("expected a binary expression");
    };
    assert_eq!(add.op, BinaryOp::Add);
    assert!(matches!(&add.lhs, Expression::Update(_)));
    assert!(matches!(&add.rhs, Expression::Identifier(_)));
}

#[test]
fn parses_shift_operators_and_assignments() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 x = 1 << offset + 1; \
                 x = value >> 2 >>> 3; \
                 x <<= 1; \
                 x >>= 2; \
                 x >>>= 3; \
             } }",
    );
    let Statement::Expression(shl_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(shl_assign) = &shl_stmt.expression else {
        panic!("expected an assignment");
    };
    // Additive binds tighter than shift: `1 << (offset + 1)`.
    let Expression::Binary(shl) = &shl_assign.value else {
        panic!("expected a binary expression");
    };
    assert_eq!(shl.op, BinaryOp::Shl);
    assert!(matches!(&shl.rhs, Expression::Binary(add) if add.op == BinaryOp::Add));

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;

    let Statement::Expression(shr_stmt) = &statements[1] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(shr_assign) = &shr_stmt.expression else {
        panic!("expected an assignment");
    };
    // Same precedence associates left: `(value >> 2) >>> 3`.
    let Expression::Binary(ushr) = &shr_assign.value else {
        panic!("expected a binary expression");
    };
    assert_eq!(ushr.op, BinaryOp::Ushr);
    assert!(matches!(&ushr.lhs, Expression::Binary(shr) if shr.op == BinaryOp::Shr));

    let ops: std::vec::Vec<_> = statements[2..]
        .iter()
        .map(|stmt| {
            let Statement::Expression(stmt) = stmt else {
                panic!("expected an expression statement");
            };
            let Expression::Assignment(assign) = &stmt.expression else {
                panic!("expected an assignment");
            };
            assign.op
        })
        .collect();
    assert_eq!(ops, [AssignOp::Shl, AssignOp::Shr, AssignOp::Ushr]);
}

#[test]
fn shift_binds_tighter_than_relational() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { ok = a < b >> c; } }");
    let Statement::Expression(stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assign) = &stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Binary(lt) = &assign.value else {
        panic!("expected a binary expression");
    };
    assert_eq!(lt.op, BinaryOp::Lt);
    assert!(matches!(&lt.rhs, Expression::Binary(shr) if shr.op == BinaryOp::Shr));
}

#[test]
fn parses_cast_expressions() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 Object o = (Object) value; \
                 int i = (int) -count; \
                 long total = (long) get() * 2; \
                 Object list = (java.util.List<String>) names; \
                 int[] copy = (int[]) raw; \
                 int d = (a) - b; \
             } }",
    );
    let Statement::LocalVariable(plain_local) = stmt else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::Cast(plain)) = &plain_local.declarators[0].init else {
        panic!("expected a cast initializer");
    };
    assert!(matches!(&plain.ty, Type::Simple(_)));
    assert!(matches!(&plain.operand, Expression::Identifier(_)));

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;

    let inits: std::vec::Vec<_> = statements
        .iter()
        .map(|stmt| {
            let Statement::LocalVariable(local) = stmt else {
                panic!("expected a local variable");
            };
            let init = local.declarators[0].init.as_ref().and_then(|init| init.as_expression());
            init.expect("expected an expression initializer")
        })
        .collect();

    // A primitive cast takes a `-` operand: `(int) -count` is a cast.
    let Expression::Cast(primitive) = inits[1] else {
        panic!("expected a cast");
    };
    assert!(matches!(&primitive.operand, Expression::Unary(_)));

    // The cast binds tighter than `*`: `((long) get()) * 2`.
    let Expression::Binary(mul) = inits[2] else {
        panic!("expected a binary expression");
    };
    assert_eq!(mul.op, BinaryOp::Mul);
    let Expression::Cast(call_cast) = &mul.lhs else {
        panic!("expected a cast on the left");
    };
    assert!(matches!(&call_cast.operand, Expression::MethodCall(_)));

    let Expression::Cast(generic) = inits[3] else {
        panic!("expected a generic cast");
    };
    let Some(generic_ty) = generic.ty.as_class_type() else {
        panic!("expected a class cast type");
    };
    assert_eq!(generic_ty.args().len(), 1);

    let Expression::Cast(array) = inits[4] else {
        panic!("expected an array cast");
    };
    assert!(matches!(&array.ty, Type::Array(_)));

    // `(a) - b` is a subtraction: a bare name before `-` never casts.
    let Expression::Binary(sub) = inits[5] else {
        panic!("expected a binary expression");
    };
    assert_eq!(sub.op, BinaryOp::Sub);
    assert!(matches!(&sub.lhs, Expression::Parenthesized(_)));
}

#[test]
fn parses_intersection_casts() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { \
                 r = (Runnable & java.io.Serializable) () -> {}; \
                 c = (Comparable<String> & @Immutable Serializable & Cloneable) o; \
                 b = (Foo & Bar) o; \
                 a = (int[] & Cloneable) raw; \
                 n = (a & b) - c; \
             } }",
    );

    let Expression::Cast(lambda) = assignment_value(&statements[0]) else {
        panic!("expected a cast");
    };
    let [bound] = &lambda.additional_bounds[..] else {
        panic!("expected one bound");
    };
    let Some(bound) = bound.as_class_type() else {
        panic!("expected a class bound");
    };
    assert_eq!(class_type_names(bound), ["java", "io", "Serializable"]);
    assert!(matches!(&lambda.operand, Expression::Lambda(_)));

    let Expression::Cast(generic) = assignment_value(&statements[1]) else {
        panic!("expected a cast");
    };
    let [annotated, other] = &generic.additional_bounds[..] else {
        panic!("expected two bounds");
    };
    let (Some(annotated), Some(_)) = (annotated.as_class_type(), other.as_class_type()) else {
        panic!("expected two class bounds");
    };
    assert_eq!(class_type_annotations(class_type_parts(annotated)[0]).len(), 1);
    assert!(matches!(&generic.operand, Expression::Identifier(_)));

    // Bare names cast when an operand follows `)`.
    let Expression::Cast(names) = assignment_value(&statements[2]) else {
        panic!("expected a cast");
    };
    assert_eq!(names.additional_bounds.len(), 1);

    // `int[]` is a reference type, so the grammar admits bounds after it.
    // Type checking, not parsing, rejects this cast.
    let Expression::Cast(array) = assignment_value(&statements[3]) else {
        panic!("expected a cast");
    };
    assert!(matches!(&array.ty, Type::Array(_)));
    assert_eq!(array.additional_bounds.len(), 1);

    // `(a & b) - c` is a subtraction: bare names before `-` never cast.
    let Expression::Binary(sub) = assignment_value(&statements[4]) else {
        panic!("expected a binary expression");
    };
    assert_eq!(sub.op, BinaryOp::Sub);
    assert!(matches!(&sub.lhs, Expression::Parenthesized(_)));
}

#[test]
fn bare_name_casts_take_switch_and_primitive_headed_operands() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { \
                 o = (Object) switch (x) { default -> 1; }; \
                 i = (int) switch (x) { default -> 1; }; \
                 o = (Object) int.class; \
                 o = (Object) void.class; \
                 o = (IntFunction) int[]::new; \
                 if (a) switch (x) { default -> {} } \
                 while (a) switch (x) { default -> {} } \
             } }",
    );

    for statement in &statements[..2] {
        let Expression::Cast(cast) = assignment_value(statement) else {
            panic!("expected a cast");
        };
        assert!(matches!(&cast.operand, Expression::Switch(_)));
    }
    for statement in &statements[2..4] {
        let Expression::Cast(cast) = assignment_value(statement) else {
            panic!("expected a cast");
        };
        assert!(matches!(&cast.operand, Expression::ClassLiteral(_)));
    }
    let Expression::Cast(reference) = assignment_value(&statements[4]) else {
        panic!("expected a cast");
    };
    assert!(matches!(&reference.operand, Expression::MethodReference(_)));

    // A parenthesized condition before a `switch` statement is not a cast.
    let Statement::If(if_stmt) = &statements[5] else {
        panic!("expected an if");
    };
    assert!(matches!(&if_stmt.condition, Expression::Identifier(_)));
    assert!(matches!(&if_stmt.then_branch, Statement::Switch(_)));
    let Statement::While(while_stmt) = &statements[6] else {
        panic!("expected a while");
    };
    assert!(matches!(&while_stmt.body, Statement::Switch(_)));
}

#[test]
fn parses_instanceof() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 ok = o instanceof java.util.List; \
                 ok = o instanceof List<String> && ready; \
                 ok = o instanceof int[] == other; \
                 ok = o instanceof @Nonnull String; \
             } }",
    );
    let Statement::Expression(stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assign) = &stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Instanceof(plain) = &assign.value else {
        panic!("expected an instanceof");
    };
    assert!(matches!(&plain.operand, Expression::Identifier(_)));
    assert!(matches!(&plain.rhs, InstanceofRhs::Qualified(_)));

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;

    // `instanceof` binds tighter than `&&`.
    let Statement::Expression(and_stmt) = &statements[1] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(and_assign) = &and_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Binary(and) = &and_assign.value else {
        panic!("expected a binary expression");
    };
    assert_eq!(and.op, BinaryOp::And);
    let Expression::Instanceof(generic) = &and.lhs else {
        panic!("expected an instanceof on the left");
    };
    let Some(generic_ty) = generic.rhs.as_class_type() else {
        panic!("expected a class type");
    };
    assert_eq!(generic_ty.args().len(), 1);

    // …and tighter than `==`; arrays are reference types, so
    // `o instanceof int[]` is legal.
    let Statement::Expression(eq_stmt) = &statements[2] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(eq_assign) = &eq_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Binary(eq) = &eq_assign.value else {
        panic!("expected a binary expression");
    };
    assert_eq!(eq.op, BinaryOp::Eq);
    let Expression::Instanceof(array) = &eq.lhs else {
        panic!("expected an instanceof on the left");
    };
    assert!(matches!(&array.rhs, InstanceofRhs::Array(_)));

    let Statement::Expression(annotated_stmt) = &statements[3] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(annotated_assign) = &annotated_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Instanceof(annotated) = &annotated_assign.value else {
        panic!("expected an instanceof");
    };
    let Some(annotated_ty) = annotated.rhs.as_class_type() else {
        panic!("expected a plain annotated type test");
    };
    assert_eq!(class_type_annotations(class_type_parts(annotated_ty)[0]).len(), 1);
}

#[test]
fn parses_instanceof_type_patterns() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f(Object o) { \
                 ok = o instanceof @Nonnull String name; \
                 ok = o instanceof final var value; \
                 ok = o instanceof java.util.Map<String, Integer> _; \
                 ok = o instanceof int count; \
             } }",
    );
    let Statement::Expression(stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assign) = &stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Instanceof(annotated) = &assign.value else {
        panic!("expected an instanceof");
    };
    let InstanceofRhs::TypePattern(annotated_pattern) = &annotated.rhs else {
        panic!("expected a type pattern");
    };
    assert_eq!(annotated_pattern.modifiers.len(), 1);
    assert_eq!(annotated_pattern.name.name, "name");

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;

    let Statement::Expression(var_stmt) = &statements[1] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(var_assign) = &var_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Instanceof(var_instanceof) = &var_assign.value else {
        panic!("expected an instanceof");
    };
    let InstanceofRhs::TypePattern(var_pattern) = &var_instanceof.rhs else {
        panic!("expected a type pattern");
    };
    assert!(
        matches!(&var_pattern.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::Final)
    );
    assert!(matches!(&var_pattern.ty, VariableType::Var(_)));
    assert_eq!(var_pattern.name.name, "value");

    let Statement::Expression(unnamed_stmt) = &statements[2] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(unnamed_assign) = &unnamed_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Instanceof(unnamed_instanceof) = &unnamed_assign.value else {
        panic!("expected an instanceof");
    };
    let InstanceofRhs::TypePattern(unnamed_pattern) = &unnamed_instanceof.rhs else {
        panic!("expected a type pattern");
    };
    assert_eq!(unnamed_pattern.name.name, "_");
    let Some(unnamed_ty) = unnamed_pattern.ty.as_class_type() else {
        panic!("expected a class pattern type");
    };
    assert_eq!(unnamed_ty.args().len(), 2);

    let Statement::Expression(primitive_stmt) = &statements[3] else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(primitive_assign) = &primitive_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Instanceof(primitive_instanceof) = &primitive_assign.value else {
        panic!("expected an instanceof");
    };
    let InstanceofRhs::TypePattern(primitive_pattern) = &primitive_instanceof.rhs else {
        panic!("expected a type pattern");
    };
    let VariableType::Primitive(primitive_ty) = &primitive_pattern.ty else {
        panic!("expected a primitive pattern type");
    };
    assert_eq!(primitive_ty.kind, PrimitiveTypeKind::Int);
    assert_eq!(primitive_pattern.name.name, "count");
}

#[test]
fn parses_instanceof_record_patterns() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f(Object o) { \
                 ok = o instanceof Point(int x, int y); \
                 ok = o instanceof Box<Point>(Point(var x, _)); \
             } }",
    );
    assert_eq!(statements.len(), 2);

    let Expression::Instanceof(point_instanceof) = assignment_value(&statements[0]) else {
        panic!("expected an instanceof");
    };
    let InstanceofRhs::RecordPattern(point) = &point_instanceof.rhs else {
        panic!("expected a record pattern");
    };
    let Some(point_ty) = point.ty.as_class_type() else {
        panic!("expected a class record pattern type");
    };
    assert_eq!(class_type_names(point_ty), ["Point"]);
    assert_eq!(point.components.len(), 2);
    let ComponentPattern::TypePattern(x_pattern) = &point.components[0] else {
        panic!("expected the first record component to be a type pattern");
    };
    assert_eq!(x_pattern.name.name, "x");
    let ComponentPattern::TypePattern(y_pattern) = &point.components[1] else {
        panic!("expected the second record component to be a type pattern");
    };
    assert_eq!(y_pattern.name.name, "y");

    let Expression::Instanceof(box_instanceof) = assignment_value(&statements[1]) else {
        panic!("expected an instanceof");
    };
    let InstanceofRhs::RecordPattern(box_pattern) = &box_instanceof.rhs else {
        panic!("expected a record pattern");
    };
    let Some(box_ty) = box_pattern.ty.as_class_type() else {
        panic!("expected a class generic record pattern type");
    };
    assert_eq!(box_ty.args().len(), 1);
    let ComponentPattern::RecordPattern(nested_point) = &box_pattern.components[0] else {
        panic!("expected a nested record pattern component");
    };
    let ComponentPattern::TypePattern(var_component) = &nested_point.components[0] else {
        panic!("expected a var type pattern component");
    };
    assert!(matches!(&var_component.ty, VariableType::Var(_)));
    assert_eq!(var_component.name.name, "x");
    assert!(matches!(&nested_point.components[1], ComponentPattern::MatchAll(_)));
}

#[test]
fn parses_integral_literal_spellings() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 long hex = 0xCAFE_BABEL; \
                 int binary = 0b1010_0101; \
                 int octal = 0_755; \
                 int decimal = 1_000_000; \
             } }",
    );
    let Statement::LocalVariable(hex_local) = stmt else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::IntLiteral(hex)) = &hex_local.declarators[0].init else {
        panic!("expected an integer literal initializer");
    };
    assert_eq!(hex.raw, "0xCAFE_BABEL");

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let raw: std::vec::Vec<_> = method
        .body
        .as_ref()
        .unwrap()
        .statements
        .iter()
        .map(|stmt| {
            let Statement::LocalVariable(local) = stmt else {
                panic!("expected a local variable");
            };
            let Some(VariableInitializer::IntLiteral(literal)) = &local.declarators[0].init else {
                panic!("expected an integer literal initializer");
            };
            literal.raw
        })
        .collect();
    assert_eq!(raw, ["0xCAFE_BABEL", "0b1010_0101", "0_755", "1_000_000"]);
}

#[test]
fn each_literal_kind_has_its_own_node() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        r#"class A { void f() {
             int i = 1;
             double d = .5;
             char c = 'x';
             String s = "x";
             boolean b = true;
             Object n = null;
        } }"#,
    );
    let kinds: std::vec::Vec<_> = statements
        .iter()
        .map(|stmt| {
            let Statement::LocalVariable(local) = stmt else {
                panic!("expected a local variable");
            };
            match local.declarators[0].init.as_ref().expect("an initializer") {
                VariableInitializer::IntLiteral(_) => "int",
                VariableInitializer::FloatLiteral(_) => "float",
                VariableInitializer::CharLiteral(_) => "char",
                VariableInitializer::StringLiteral(_) => "string",
                VariableInitializer::BooleanLiteral(_) => "boolean",
                VariableInitializer::NullLiteral(_) => "null",
                _ => panic!("expected a literal expression"),
            }
        })
        .collect();
    assert_eq!(kinds, ["int", "float", "char", "string", "boolean", "null"]);
}

#[test]
fn a_boolean_literal_holds_its_value_whatever_its_spelling() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        r"class A { void f() {
             boolean a = true;
             boolean b = false;
             boolean c = \u0074rue;
             boolean d = \u0066alse;
        } }",
    );
    let values: std::vec::Vec<_> = statements
        .iter()
        .map(|stmt| {
            let Statement::LocalVariable(local) = stmt else {
                panic!("expected a local variable");
            };
            let Some(VariableInitializer::BooleanLiteral(literal)) = &local.declarators[0].init
            else {
                panic!("expected a boolean literal");
            };
            literal.value
        })
        .collect();
    assert_eq!(values, [true, false, true, false]);
}

#[test]
fn parses_floating_point_literal_spellings() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 double decimal = 1.; \
                 double leading = .5e-2; \
                 float suffixed = 6_022e+20F; \
                 double hex = 0x1.fp3; \
                 double leading_zero = 09.0; \
             } }",
    );
    let Statement::LocalVariable(decimal_local) = stmt else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::FloatLiteral(decimal)) = &decimal_local.declarators[0].init
    else {
        panic!("expected a floating-point literal initializer");
    };
    assert_eq!(decimal.raw, "1.");

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let raw: std::vec::Vec<_> = method
        .body
        .as_ref()
        .unwrap()
        .statements
        .iter()
        .map(|stmt| {
            let Statement::LocalVariable(local) = stmt else {
                panic!("expected a local variable");
            };
            let Some(VariableInitializer::FloatLiteral(literal)) = &local.declarators[0].init
            else {
                panic!("expected a floating-point literal initializer");
            };
            literal.raw
        })
        .collect();
    assert_eq!(raw, ["1.", ".5e-2", "6_022e+20F", "0x1.fp3", "09.0"]);
}

#[test]
fn parses_character_literals() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        r"class A { char f() { char quote = '\''; Object boxed = (Character) 'x'; return '\123'; } }",
    );
    let Statement::LocalVariable(char_local) = stmt else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::CharLiteral(quote)) = &char_local.declarators[0].init else {
        panic!("expected a character literal initializer");
    };
    assert_eq!(quote.raw, r"'\''");

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;
    let Statement::LocalVariable(boxed_local) = &statements[1] else {
        panic!("expected a local variable");
    };
    let Some(VariableInitializer::Cast(cast)) = &boxed_local.declarators[0].init else {
        panic!("expected a cast initializer");
    };
    assert!(matches!(&cast.operand, Expression::CharLiteral(_)));

    let Statement::Return(ret) = &statements[2] else {
        panic!("expected a return");
    };
    assert!(matches!(&ret.value, Some(Expression::CharLiteral(_))));
}

#[test]
fn parses_concise_lambdas() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { r = x -> x + 1; } }");
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Lambda(lambda) = &assignment.value else {
        panic!("expected a lambda");
    };
    assert!(!lambda.params.has_parentheses());
    assert_eq!(lambda.params.items.len(), 1);
    assert_eq!(lambda.params.span, lambda.params.items[0].span);
    assert_eq!(lambda.params.items[0].name.name, "x");
    assert!(lambda.params.items[0].ty.is_none());
    assert!(lambda.body.is_expression());
}

#[test]
fn parenthesized_lambda_parameters_are_kept_distinct() {
    let allocator = Allocator::new();
    let source = "class A { void f() { r = (x) -> x; s = () -> {}; t = (a, b) -> a + b; } }";
    let (stmt, _) = first_statement(&allocator, source);
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Lambda(lambda) = &assignment.value else {
        panic!("expected a lambda");
    };
    assert!(lambda.params.has_parentheses());
    assert_eq!(lambda.params.span.source_text(source), "(x)");
    assert_eq!(lambda.params.items.len(), 1);
}

#[test]
fn parses_typed_lambda_parameters_and_block_bodies() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { r = (final int a, java.util.List<String> b) -> { return a; }; } }",
    );
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Lambda(lambda) = &assignment.value else {
        panic!("expected a lambda");
    };
    assert_eq!(lambda.params.items.len(), 2);
    assert_eq!(lambda.params.items[0].modifiers.len(), 1);
    assert!(lambda.params.items[0].ty.is_some());
    assert!(lambda.params.items[1].ty.is_some());
    let LambdaBody::Block(block) = &lambda.body else {
        panic!("expected a block body");
    };
    assert_eq!(block.statements.len(), 1);
}

#[test]
fn typed_lambda_parameters_allow_annotation_arguments() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { r = (@A(value = 1, names = {X, Y}) String name) -> name; } }",
    );
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Lambda(lambda) = &assignment.value else {
        panic!("expected a lambda");
    };
    assert_eq!(lambda.params.items.len(), 1);
    assert_eq!(lambda.params.items[0].modifiers.len(), 1);
    assert!(lambda.params.items[0].ty.is_some());
}

#[test]
fn parses_typed_lambda_varargs_parameters() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { r = (String... names) -> names.length; } }",
    );
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Lambda(lambda) = &assignment.value else {
        panic!("expected a lambda");
    };
    assert_eq!(lambda.params.items.len(), 1);
    assert!(lambda.params.items[0].varargs);
    assert_eq!(lambda.params.items[0].name.name, "names");
    assert!(lambda.params.items[0].ty.is_some());
}

#[test]
fn typed_lambda_varargs_parameters_allow_annotations_on_the_ellipsis() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { r = (String @Nullable ... names) -> names.length; } }",
    );
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Lambda(lambda) = &assignment.value else {
        panic!("expected a lambda");
    };
    assert_eq!(lambda.params.items.len(), 1);
    assert!(lambda.params.items[0].varargs);
    assert_eq!(lambda.params.items[0].varargs_annotations.len(), 1);
    assert_eq!(type_name_parts(&lambda.params.items[0].varargs_annotations[0].name), ["Nullable"]);
    let Some(element) = lambda.params.items[0].ty.as_ref().and_then(VariableType::as_class_type)
    else {
        panic!("expected a class element type");
    };
    assert!(class_type_annotations(element).is_empty());
}

#[test]
fn lambdas_stand_as_arguments_and_cast_operands() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { run((Runnable) () -> {}, x -> g(x)); } }",
    );
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::MethodCall(call) = &expr_stmt.expression else {
        panic!("expected a call");
    };
    let Expression::Cast(cast) = &call.args[0] else {
        panic!("expected a cast");
    };
    assert!(matches!(&cast.operand, Expression::Lambda(_)));
    assert!(matches!(&call.args[1], Expression::Lambda(_)));
}

#[test]
fn conditional_branches_can_be_lambdas() {
    let allocator = Allocator::new();
    let (stmt, _) =
        first_statement(&allocator, "class A { void f() { r = flag ? x -> a : (y) -> b; } }");
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Conditional(conditional) = &assignment.value else {
        panic!("expected a conditional");
    };
    assert!(matches!(&conditional.when_true, Expression::Lambda(_)));
    assert!(matches!(&conditional.when_false, Expression::Lambda(_)));
}

#[test]
fn lambda_bodies_nest_lambdas_and_assignments() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { r = x -> y -> v = x; } }");
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    let Expression::Lambda(outer) = &assignment.value else {
        panic!("expected a lambda");
    };
    let LambdaBody::Lambda(inner) = &outer.body else {
        panic!("expected a nested lambda body");
    };
    assert!(matches!(&inner.body, LambdaBody::Assignment(_)));
}

#[test]
fn parenthesized_expressions_are_not_mistaken_for_lambdas() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { r = (a) - (b + c); } }");
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    assert!(matches!(&assignment.value, Expression::Binary(_)));
}

#[test]
fn parses_name_and_primary_method_references() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { r = a.b.Foo::bar; s = this::g; t = f(x)::h; u = super::m; v = Outer.Inner.super::<String>adapt; } }",
    );
    assert_eq!(statements.len(), 5);

    let reference = method_reference_assignment(&statements[0]);
    assert!(matches!(&reference.target, MethodReferenceTarget::FieldAccess(_)));
    assert_eq!(expression_names(reference.target.to_expression()), ["a", "b", "Foo"]);
    assert!(reference.type_args.is_none());
    assert_eq!(reference.method.as_ref().unwrap().name, "bar");

    let reference = method_reference_assignment(&statements[1]);
    assert!(matches!(&reference.target, MethodReferenceTarget::This(_)));
    assert_eq!(reference.method.as_ref().unwrap().name, "g");

    let reference = method_reference_assignment(&statements[2]);
    assert!(matches!(&reference.target, MethodReferenceTarget::MethodCall(_)));
    assert_eq!(reference.method.as_ref().unwrap().name, "h");

    let reference = method_reference_assignment(&statements[3]);
    assert!(matches!(&reference.target, MethodReferenceTarget::Super(_)));
    assert_eq!(reference.method.as_ref().unwrap().name, "m");

    let reference = method_reference_assignment(&statements[4]);
    let MethodReferenceTarget::QualifiedSuper(target) = &reference.target else {
        panic!("expected a qualified-super target");
    };
    assert_eq!(type_name_parts(&target.qualifier), ["Outer", "Inner"]);
    assert_eq!(reference.type_args.as_ref().unwrap().args.len(), 1);
    assert_eq!(reference.method.as_ref().unwrap().name, "adapt");
}

#[test]
fn parses_constructor_references() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { r = Foo::new; s = int[]::new; t = java.util.List<String>::new; } }",
    );
    assert_eq!(statements.len(), 3);

    let reference = method_reference_assignment(&statements[0]);
    // A plain name target stays an expression name, even before `::new`.
    assert!(matches!(&reference.target, MethodReferenceTarget::Identifier(_)));
    assert!(reference.method.is_none());

    let reference = method_reference_assignment(&statements[1]);
    assert!(matches!(&reference.target, MethodReferenceTarget::Array(_)));
    assert!(reference.method.is_none());

    let reference = method_reference_assignment(&statements[2]);
    let Some(named) = reference.target.as_class_type() else {
        panic!("expected a generic type target");
    };
    assert_eq!(named.args().len(), 1);
    assert!(reference.method.is_none());
}

#[test]
fn type_shaped_method_reference_targets_parse_as_types() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { r = java.util.List<String>::size; s = int[]::new; } }",
    );
    assert_eq!(statements.len(), 2);

    let reference = method_reference_assignment(&statements[0]);
    let Some(named) = reference.target.as_class_type() else {
        panic!("expected a class type target");
    };
    assert_eq!(named.args().len(), 1);
    assert_eq!(reference.method.as_ref().unwrap().name, "size");

    let reference = method_reference_assignment(&statements[1]);
    assert!(matches!(&reference.target, MethodReferenceTarget::Array(_)));
    assert!(reference.method.is_none());
}

#[test]
fn annotated_method_reference_targets_parse_as_types() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() { r = @A String::length; s = @A String @B []::new; \
         t = String @B []::new; u = @A int @B []::new; v = @A List<String>::size; } }",
    );
    assert_eq!(statements.len(), 5);

    let reference = method_reference_assignment(&statements[0]);
    let Some(named) = reference.target.as_class_type() else {
        panic!("expected a class type target");
    };
    assert_eq!(class_type_annotations(class_type_parts(named)[0]).len(), 1);
    assert_eq!(reference.method.as_ref().unwrap().name, "length");

    for (stmt, element_annotations) in
        [(&statements[1], 1), (&statements[2], 0), (&statements[3], 1)]
    {
        let reference = method_reference_assignment(stmt);
        let MethodReferenceTarget::Array(array) = &reference.target else {
            panic!("expected an array type target");
        };
        assert_eq!(array.annotations.len(), 1);
        assert_eq!(leading_annotations(&array.element).len(), element_annotations);
        assert!(reference.method.is_none());
    }

    let reference = method_reference_assignment(&statements[4]);
    let Some(named) = reference.target.as_class_type() else {
        panic!("expected a generic type target");
    };
    assert_eq!(class_type_annotations(class_type_parts(named)[0]).len(), 1);
    assert_eq!(class_type_parts(named)[0].args().len(), 1);
}

#[test]
fn parses_method_reference_type_arguments_and_chains() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { run(list.stream().map(Foo::<String>convert), Foo::bar); } }",
    );
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::MethodCall(run) = &expr_stmt.expression else {
        panic!("expected a call");
    };
    let Expression::MethodCall(map) = &run.args[0] else {
        panic!("expected a chained call");
    };
    let Expression::MethodReference(reference) = &map.args[0] else {
        panic!("expected a method reference argument");
    };
    assert_eq!(reference.type_args.as_ref().unwrap().args.len(), 1);
    assert_eq!(reference.method.as_ref().unwrap().name, "convert");
    assert!(matches!(&run.args[1], Expression::MethodReference(_)));
}

#[test]
fn comparisons_are_not_mistaken_for_type_method_references() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { r = a < b; s = c[i]; } }");
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    assert!(matches!(&assignment.value, Expression::Binary(_)));
}

#[test]
fn parses_switch_expressions_with_arrow_rules() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { int f(int code) { \
                 return switch (code) { \
                     case 1, 2 -> 10; \
                     case 3 -> { yield compute(code); } \
                     default -> throw bad(); \
                 }; \
             } }",
    );
    let Statement::Return(ret) = stmt else {
        panic!("expected a return statement");
    };
    let Some(Expression::Switch(switch_expr)) = &ret.value else {
        panic!("expected a switch expression");
    };
    let rules = switch_rules(&switch_expr.cases);
    assert_eq!(rules.len(), 3);
    assert!(matches!(&rules[0].body, SwitchRuleBody::IntLiteral(_)));

    let SwitchRuleBody::Block(block) = &rules[1].body else {
        panic!("expected a block rule body");
    };
    assert_eq!(block.statements.len(), 1);
    assert!(matches!(&block.statements[0], Statement::Yield(_)));

    assert!(rules[2].label.is_none());
    assert!(matches!(&rules[2].body, SwitchRuleBody::Throw(_)));
}

#[test]
fn parses_colon_form_switch_expressions_with_yield() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { int f(int code) { \
                 return switch (code) { \
                     case 1: yield 10; \
                     default: yield 0; \
                 }; \
             } }",
    );
    let Statement::Return(ret) = stmt else {
        panic!("expected a return statement");
    };
    let Some(Expression::Switch(switch_expr)) = &ret.value else {
        panic!("expected a switch expression");
    };
    let cases = switch_colon_cases(&switch_expr.cases);
    assert_eq!(cases.len(), 2);
    assert!(matches!(&cases[0].statements[0], Statement::Yield(_)));
    assert!(cases[1].label.is_none());
    assert!(matches!(&cases[1].statements[0], Statement::Yield(_)));
}
