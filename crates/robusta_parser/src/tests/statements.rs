use super::*;

#[test]
fn parses_explicit_constructor_invocation_type_arguments() {
    let allocator = Allocator::new();
    let source = "class A {\
                          A(Object value) { <String>this(value, 1); }\
                          A(String value, int count) { <java.util.List<String>, Integer>super(value); }\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };

    let ClassMember::Constructor(first_ctor) = &class.body.members[0] else {
        panic!("expected a constructor");
    };
    let Statement::ConstructorInvocation(this_invocation) = &first_ctor.body.statements[0] else {
        panic!("expected a constructor invocation");
    };
    assert_eq!(this_invocation.kind, ConstructorInvocationKind::This);
    assert_eq!(this_invocation.type_args.as_ref().unwrap().args.len(), 1);
    assert!(this_invocation.qualifier.is_none());
    assert_eq!(this_invocation.args.len(), 2);

    let ClassMember::Constructor(second_ctor) = &class.body.members[1] else {
        panic!("expected a constructor");
    };
    let Statement::ConstructorInvocation(super_invocation) = &second_ctor.body.statements[0] else {
        panic!("expected a constructor invocation");
    };
    assert_eq!(super_invocation.kind, ConstructorInvocationKind::Super);
    assert_eq!(super_invocation.type_args.as_ref().unwrap().args.len(), 2);
    assert!(super_invocation.qualifier.is_none());
    assert_eq!(super_invocation.args.len(), 1);
}

#[test]
fn parses_qualified_explicit_constructor_invocations() {
    let allocator = Allocator::new();
    let source = "class Child {\
                          Child(Outer owner, Object value) { owner.super(value); init(); }\
                          Child() { factory().<String>super(name()); }\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };

    let ClassMember::Constructor(first_ctor) = &class.body.members[0] else {
        panic!("expected a constructor");
    };
    let Statement::ConstructorInvocation(named_invocation) = &first_ctor.body.statements[0] else {
        panic!("expected a constructor invocation");
    };
    assert_eq!(named_invocation.kind, ConstructorInvocationKind::Super);
    assert!(named_invocation.type_args.is_none());
    assert_eq!(named_invocation.args.len(), 1);
    assert!(matches!(&named_invocation.qualifier, Some(Expression::Identifier(_))));
    assert!(matches!(&first_ctor.body.statements[1], Statement::Expression(_)));

    let ClassMember::Constructor(second_ctor) = &class.body.members[1] else {
        panic!("expected a constructor");
    };
    let Statement::ConstructorInvocation(primary_invocation) = &second_ctor.body.statements[0]
    else {
        panic!("expected a constructor invocation");
    };
    assert_eq!(primary_invocation.kind, ConstructorInvocationKind::Super);
    assert_eq!(primary_invocation.type_args.as_ref().unwrap().args.len(), 1);
    assert_eq!(primary_invocation.args.len(), 1);
    assert!(matches!(&primary_invocation.qualifier, Some(Expression::MethodCall(_))));
}

#[test]
fn parses_flexible_constructor_bodies() {
    let allocator = Allocator::new();
    let result = parse(
        &allocator,
        "class A { A(int x) { int y = x + 1; this(y); check(); } A() { before(); super(); after(); } }",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };

    let ClassMember::Constructor(this_ctor) = &class.body.members[0] else {
        panic!("expected a constructor");
    };
    assert_eq!(this_ctor.body.statements.len(), 3);
    assert!(matches!(&this_ctor.body.statements[0], Statement::LocalVariable(_)));
    let Statement::ConstructorInvocation(this_invocation) = &this_ctor.body.statements[1] else {
        panic!("expected a constructor invocation after the prologue");
    };
    assert_eq!(this_invocation.kind, ConstructorInvocationKind::This);
    assert!(matches!(&this_ctor.body.statements[2], Statement::Expression(_)));

    let ClassMember::Constructor(super_ctor) = &class.body.members[1] else {
        panic!("expected a constructor");
    };
    assert_eq!(super_ctor.body.statements.len(), 3);
    assert!(matches!(&super_ctor.body.statements[0], Statement::Expression(_)));
    let Statement::ConstructorInvocation(super_invocation) = &super_ctor.body.statements[1] else {
        panic!("expected a constructor invocation after the prologue");
    };
    assert_eq!(super_invocation.kind, ConstructorInvocationKind::Super);
    assert!(matches!(&super_ctor.body.statements[2], Statement::Expression(_)));
}

#[test]
fn parses_if_else_chains() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { \
                 if (a < 0) { neg(); } else if (a == 0) zero(); else { pos(); } \
             } }",
    );
    let Statement::If(outer) = stmt else {
        panic!("expected an if");
    };
    assert!(matches!(&outer.condition, Expression::Binary(_)));
    assert!(matches!(&outer.then_branch, Statement::Block(_)));
    let Some(Statement::If(else_if)) = &outer.else_branch else {
        panic!("expected an else-if chain");
    };
    // The middle branch is braceless: the statement itself, not a block.
    assert!(matches!(&else_if.then_branch, Statement::Expression(_)));
    assert!(matches!(&else_if.else_branch, Some(Statement::Block(_))));
}

#[test]
fn else_binds_to_the_nearest_if() {
    let allocator = Allocator::new();
    let (stmt, _) =
        first_statement(&allocator, "class A { void f() { if (a) if (b) x(); else y(); } }");
    let Statement::If(outer) = stmt else {
        panic!("expected an if");
    };
    assert!(outer.else_branch.is_none());
    let Statement::If(inner) = &outer.then_branch else {
        panic!("expected a nested if");
    };
    assert!(inner.else_branch.is_some());
}

#[test]
fn parses_while_and_do_loops() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 while (running) tick(); \
                 do { tick(); } while (running); \
                 while (x()); \
             } }",
    );
    let Statement::While(while_stmt) = stmt else {
        panic!("expected a while");
    };
    assert!(matches!(&while_stmt.body, Statement::Expression(_)));

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;
    let Statement::Do(do_stmt) = &statements[1] else {
        panic!("expected a do statement");
    };
    assert!(matches!(&do_stmt.body, Statement::Block(_)));
    let Statement::While(empty_body) = &statements[2] else {
        panic!("expected a while with an empty body");
    };
    assert!(matches!(&empty_body.body, Statement::Empty(_)));
}

#[test]
fn parses_basic_for_variants() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 for (int i = 0, n = size(); i < n; i += 1) use(i); \
                 for (i = 0, j = 9; i < j; i += 1, j -= 1) swap(i, j); \
                 for (;;) { spin(); } \
             } }",
    );
    let Statement::For(declared) = stmt else {
        panic!("expected a for");
    };
    let Some(ForInit::Declaration(init)) = &declared.init else {
        panic!("expected a declaration initializer");
    };
    assert_eq!(init.declarators.len(), 2);
    assert!(declared.condition.is_some());
    assert_eq!(declared.update.as_ref().unwrap().expressions.len(), 1);
    assert!(matches!(&declared.body, Statement::Expression(_)));

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;
    let Statement::For(expressions) = &statements[1] else {
        panic!("expected a for");
    };
    let Some(ForInit::Expressions(init)) = &expressions.init else {
        panic!("expected an expression-list initializer");
    };
    assert_eq!(init.expressions.len(), 2);
    assert_eq!(expressions.update.as_ref().unwrap().expressions.len(), 2);

    let Statement::For(bare) = &statements[2] else {
        panic!("expected a for");
    };
    assert!(bare.init.is_none());
    assert!(bare.condition.is_none());
    assert!(bare.update.is_none());
}

#[test]
fn statement_expression_lists_span_their_expressions() {
    let allocator = Allocator::new();
    let source = r"class A { void f() { for ( (i)++ \u002c j = 9 ; ; i += 1 , (j)-- ) ; } }";
    let (stmt, _) = first_statement(&allocator, source);
    let Statement::For(statement) = stmt else {
        panic!("expected a for");
    };
    let Some(ForInit::Expressions(init)) = &statement.init else {
        panic!("expected an expression-list initializer");
    };
    assert_eq!(init.span.source_text(source), r"(i)++ \u002c j = 9");
    assert_eq!(init.expressions.len(), 2);
    let update = statement.update.as_ref().unwrap();
    assert_eq!(update.span.source_text(source), "i += 1 , (j)--");
    assert_eq!(update.expressions.len(), 2);
}

#[test]
fn for_declaration_ends_before_the_header_separator() {
    let allocator = Allocator::new();
    let source = r"class A { void f() { for (int i = 0, j[] = {} /* c */ \u003b ; ) ; } }";
    let (stmt, _) = first_statement(&allocator, source);
    let Statement::For(statement) = stmt else {
        panic!("expected a for");
    };
    let Some(ForInit::Declaration(init)) = &statement.init else {
        panic!("expected a declaration initializer");
    };
    assert_eq!(init.span.source_text(source), "int i = 0, j[] = {}");

    let source = r"class A { void f() { int i = 0 \u003b } }";
    let (stmt, _) = first_statement(&allocator, source);
    let Statement::LocalVariable(local) = stmt else {
        panic!("expected a local variable");
    };
    assert_eq!(local.span.source_text(source), r"int i = 0 \u003b");
}

#[test]
fn parses_enhanced_for() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { for (final Map.Entry<String, Integer> e : map.entrySet()) { use(e); } } }",
    );
    let Statement::EnhancedFor(for_each) = stmt else {
        panic!("expected an enhanced for");
    };
    assert_eq!(for_each.declaration.modifiers.len(), 1);
    assert!(matches!(&for_each.declaration.ty, VariableType::Qualified(_)));
    assert_eq!(for_each.declaration.declarators[0].name.name, "e");
    assert!(matches!(&for_each.iterable, Expression::MethodCall(_)));
}

#[test]
fn parses_labels_break_and_continue() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { \
                 outer: \
                 while (true) { \
                     if (a) break outer; \
                     if (b) continue; \
                     break; \
                 } \
             } }",
    );
    let Statement::Labeled(labeled) = stmt else {
        panic!("expected a labeled statement");
    };
    assert_eq!(labeled.label.name, "outer");
    let Statement::While(while_stmt) = &labeled.statement else {
        panic!("expected a labeled while");
    };
    let Statement::Block(body) = &while_stmt.body else {
        panic!("expected a block body");
    };
    let Statement::If(first) = &body.statements[0] else {
        panic!("expected an if");
    };
    let Statement::Break(labeled_break) = &first.then_branch else {
        panic!("expected a break");
    };
    assert_eq!(labeled_break.label.as_ref().unwrap().name, "outer");
    let Statement::If(second) = &body.statements[1] else {
        panic!("expected an if");
    };
    let Statement::Continue(bare_continue) = &second.then_branch else {
        panic!("expected a continue");
    };
    assert!(bare_continue.label.is_none());
    assert!(matches!(&body.statements[2], Statement::Break(_)));
}

#[test]
fn parses_throw_statements() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 throw new IllegalStateException(message); \
                 if (bad) throw problem; \
             } }",
    );
    let Statement::Throw(throw_stmt) = stmt else {
        panic!("expected a throw statement");
    };
    assert!(matches!(&throw_stmt.value, Expression::ClassInstanceCreation(_)));

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let Statement::If(if_stmt) = &method.body.as_ref().unwrap().statements[1] else {
        panic!("expected an if");
    };
    assert!(matches!(&if_stmt.then_branch, Statement::Throw(_)));
}

#[test]
fn parses_assert_statements() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 assert ready; \
                 assert count > 0 : \"count\"; \
             } }",
    );
    let Statement::Assert(assert_stmt) = stmt else {
        panic!("expected an assert statement");
    };
    assert!(matches!(&assert_stmt.condition, Expression::Identifier(_)));
    assert!(assert_stmt.message.is_none());

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let Statement::Assert(with_message) = &method.body.as_ref().unwrap().statements[1] else {
        panic!("expected an assert statement");
    };
    assert!(matches!(&with_message.condition, Expression::Binary(_)));
    assert!(matches!(&with_message.message, Some(Expression::StringLiteral(_))));
}

#[test]
fn parses_try_catch_finally_statements() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { void f() { \
                 try { read(); } \
                 catch (final java.io.IOException | IllegalStateException ex) { throw ex; } \
                 catch (RuntimeException ex) { recover(ex); } \
                 finally { close(); } \
                 try { cleanup(); } finally { close(); } \
             } }",
    );
    let Statement::Try(try_stmt) = stmt else {
        panic!("expected a try statement");
    };
    assert_eq!(try_stmt.block.statements.len(), 1);
    assert_eq!(try_stmt.catches.len(), 2);
    assert!(try_stmt.finally_block.is_some());

    let first_catch = &try_stmt.catches[0];
    assert_eq!(first_catch.modifiers.len(), 1);
    assert_eq!(first_catch.types.len(), 2);
    assert_eq!(first_catch.name.name, "ex");
    assert_eq!(first_catch.block.statements.len(), 1);

    let Some(first_type) = first_catch.types[0].as_class_type() else {
        panic!("expected a class catch type");
    };
    assert_eq!(class_type_names(first_type), ["java", "io", "IOException"]);

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let Statement::Try(finally_only) = &method.body.as_ref().unwrap().statements[1] else {
        panic!("expected a try-finally statement");
    };
    assert!(finally_only.catches.is_empty());
    assert!(finally_only.finally_block.is_some());
}

#[test]
fn parses_try_with_resources() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { \
                 try (final InputStream in = open(); var out = create()) { copy(in, out); } \
             } }",
    );
    let Statement::Try(try_stmt) = stmt else {
        panic!("expected a try statement");
    };
    assert_eq!(try_stmt.resources.len(), 2);
    assert!(!try_stmt.resources_trailing_semicolon);
    // A resource-only try needs no catch or finally.
    assert!(try_stmt.catches.is_empty());
    assert!(try_stmt.finally_block.is_none());

    let Resource::Declaration(first) = &try_stmt.resources[0] else {
        panic!("expected a resource declaration");
    };
    assert_eq!(first.modifiers.len(), 1);
    assert_eq!(first.declarators[0].name.name, "in");
    let Resource::Declaration(second) = &try_stmt.resources[1] else {
        panic!("expected a resource declaration");
    };
    assert_eq!(second.declarators[0].name.name, "out");
}

#[test]
fn try_with_resources_keeps_catch_finally_and_trailing_semicolon() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { \
                 try (lock; this.other; factory().owned;) { work(); } catch (IOException e) {} finally { done(); } \
             } }",
    );
    let Statement::Try(try_stmt) = stmt else {
        panic!("expected a try statement");
    };
    assert_eq!(try_stmt.resources.len(), 3);
    assert!(try_stmt.resources_trailing_semicolon);
    assert_eq!(try_stmt.catches.len(), 1);
    assert!(try_stmt.finally_block.is_some());
    let Resource::Identifier(lock) = &try_stmt.resources[0] else {
        panic!("expected an expression-name resource access");
    };
    assert_eq!(lock.name, "lock");

    let Resource::Field(other) = &try_stmt.resources[1] else {
        panic!("expected a field-access resource access");
    };
    assert_eq!(other.name.name, "other");
    assert!(matches!(other.target, Expression::This(_)));

    let Resource::Field(owned) = &try_stmt.resources[2] else {
        panic!("expected a primary field-access resource access");
    };
    assert_eq!(owned.name.name, "owned");
    assert!(matches!(owned.target, Expression::MethodCall(_)));
}

#[test]
fn parses_synchronized_statements() {
    let allocator = Allocator::new();
    let (stmt, unit) = first_statement(
        &allocator,
        "class A { synchronized void f() { \
                 synchronized (this.lock) { count++; } \
                 synchronized (Registry.global()) {} \
             } }",
    );
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    assert_eq!(method.modifiers.len(), 1);
    let Modifier::Keyword(keyword) = &method.modifiers[0] else {
        panic!("expected a keyword modifier");
    };
    assert_eq!(keyword.kind, ModifierKind::Synchronized);

    let Statement::Synchronized(sync) = stmt else {
        panic!("expected a synchronized statement");
    };
    assert!(matches!(&sync.lock, Expression::FieldAccess(_)));
    assert_eq!(sync.body.statements.len(), 1);

    let statements = &method.body.as_ref().unwrap().statements;
    let Statement::Synchronized(second) = &statements[1] else {
        panic!("expected a second synchronized statement");
    };
    assert!(matches!(&second.lock, Expression::MethodCall(_)));
    assert!(second.body.statements.is_empty());
}

#[test]
fn parses_switch_statements() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f(int code) { \
                 switch (code) { \
                     case 1: \
                     case 2: \
                         prepare(); \
                         break; \
                     case 3, LIMIT - 1: \
                         handle(code); \
                     default: \
                         fallback(); \
                 } \
             } }",
    );
    let Statement::Switch(switch_stmt) = stmt else {
        panic!("expected a switch statement");
    };
    assert!(matches!(&switch_stmt.selector, Expression::Identifier(_)));
    let cases = switch_colon_cases(&switch_stmt.cases);
    assert_eq!(cases.len(), 4);

    assert!(cases[..2].iter().all(
        |case| matches!(&case.label, Some(SwitchLabel::Case(case)) if case.constants.len() == 1)
    ));
    assert!(cases[0].statements.is_empty());
    assert_eq!(cases[1].statements.len(), 2);

    let Some(SwitchLabel::Case(case)) = &cases[2].label else {
        panic!("expected a case label");
    };
    assert_eq!(case.constants.len(), 2);
    assert!(matches!(&case.constants[1], Expression::Binary(_)));
    assert_eq!(cases[2].statements.len(), 1);

    assert!(cases[3].label.is_none());
    assert_eq!(cases[3].statements.len(), 1);
}

#[test]
fn stacked_switch_labels_are_cases_without_statements() {
    let allocator = Allocator::new();
    let source = "class A { void f() { switch (x) { case 1: case 2: default: f(); } \
                  switch (x) { case 1: f(); case 2: default: } } }";
    let (statements, _) = first_method_statements(&allocator, source);
    let spans = |statement: &Statement<'_>| {
        let Statement::Switch(switch_stmt) = statement else {
            panic!("expected a switch statement");
        };
        switch_colon_cases(&switch_stmt.cases)
            .iter()
            .map(|case| (case.span.source_text(source), case.statements.len()))
            .collect::<std::vec::Vec<_>>()
    };
    assert_eq!(spans(&statements[0]), [("case 1:", 0), ("case 2:", 0), ("default: f();", 1)]);
    assert_eq!(spans(&statements[1]), [("case 1: f();", 1), ("case 2:", 0), ("default:", 0)]);
}

#[test]
fn parses_switch_arrow_rules() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f(int code) { \
                 switch (code) { \
                     case 1, 2 -> handle(code); \
                     case 3 -> { prepare(); break; } \
                     default -> throw bad(); \
                 } \
             } }",
    );
    let Statement::Switch(switch_stmt) = stmt else {
        panic!("expected a switch statement");
    };
    let rules = switch_rules(&switch_stmt.cases);
    assert_eq!(rules.len(), 3);

    let Some(SwitchLabel::Case(first_case)) = &rules[0].label else {
        panic!("expected a case label");
    };
    assert_eq!(first_case.constants.len(), 2);
    assert!(matches!(&rules[0].body, SwitchRuleBody::MethodCall(_)));

    let SwitchRuleBody::Block(block) = &rules[1].body else {
        panic!("expected a block rule body");
    };
    assert_eq!(block.statements.len(), 2);

    assert!(rules[2].label.is_none());
    assert!(matches!(&rules[2].body, SwitchRuleBody::Throw(_)));
}

#[test]
fn parses_switch_type_pattern_labels() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f(Object value) { \
                 switch (value) { \
                     case @Nonnull String name when name.length() > 0 -> handle(name); \
                     case final var item -> handle(item); \
                     default -> throw bad(); \
                 } \
             } }",
    );
    let Statement::Switch(switch_stmt) = stmt else {
        panic!("expected a switch statement");
    };
    let rules = switch_rules(&switch_stmt.cases);
    assert_eq!(rules.len(), 3);

    let Some(SwitchLabel::Pattern(first_case)) = &rules[0].label else {
        panic!("expected a pattern switch label");
    };
    assert_eq!(first_case.patterns.len(), 1);
    assert!(first_case.guard.is_some());
    let Pattern::TypePattern(first_pattern) = &first_case.patterns[0] else {
        panic!("expected a type pattern");
    };
    assert_eq!(first_pattern.declaration.modifiers.len(), 1);
    assert_eq!(first_pattern.declaration.declarators[0].name.name, "name");

    let Some(SwitchLabel::Pattern(second_case)) = &rules[1].label else {
        panic!("expected a pattern switch label");
    };
    let Pattern::TypePattern(second_pattern) = &second_case.patterns[0] else {
        panic!("expected a type pattern");
    };
    assert!(
        matches!(&second_pattern.declaration.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::Final)
    );
    assert!(matches!(&second_pattern.declaration.ty, VariableType::Var(_)));
    assert_eq!(second_pattern.declaration.declarators[0].name.name, "item");
}

#[test]
fn parses_colon_switch_pattern_label_lists() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f(Object value) { \
                 switch (value) { \
                     case String name, Integer count: \
                         handle(name); \
                     default: \
                 } \
             } }",
    );
    let Statement::Switch(switch_stmt) = stmt else {
        panic!("expected a switch statement");
    };
    let cases = switch_colon_cases(&switch_stmt.cases);
    assert_eq!(cases.len(), 2);
    let Some(SwitchLabel::Pattern(pattern_case)) = &cases[0].label else {
        panic!("expected a pattern switch label");
    };
    assert_eq!(pattern_case.patterns.len(), 2);
    assert!(pattern_case.guard.is_none());
    assert_eq!(cases[0].statements.len(), 1);
}

#[test]
fn parses_switch_record_pattern_labels() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f(Object value) { \
                 switch (value) { \
                     case Box<Point>(Point(int x, _)) when x > 0 -> handle(x); \
                     case Empty() -> handleEmpty(); \
                     default -> handleDefault(); \
                 } \
                 switch (value) { \
                     case Point(int x, int y), Line(Point(var start, _), Point(var end, _)): \
                         handle(x); \
                     default: \
                 } \
             } }",
    );
    assert_eq!(statements.len(), 2);

    let Statement::Switch(rule_switch) = &statements[0] else {
        panic!("expected a switch statement");
    };
    let rules = switch_rules(&rule_switch.cases);
    let Some(SwitchLabel::Pattern(first_case)) = &rules[0].label else {
        panic!("expected a pattern switch label");
    };
    assert!(first_case.guard.is_some());
    let Pattern::RecordPattern(box_pattern) = &first_case.patterns[0] else {
        panic!("expected a record pattern");
    };
    let Some(box_ty) = box_pattern.ty.as_class_type() else {
        panic!("expected a class generic record pattern type");
    };
    assert_eq!(box_ty.args().len(), 1);
    let ComponentPattern::RecordPattern(point) = &box_pattern.components[0] else {
        panic!("expected a nested record pattern");
    };
    assert!(matches!(&point.components[1], ComponentPattern::MatchAll(_)));

    let Some(SwitchLabel::Pattern(empty_case)) = &rules[1].label else {
        panic!("expected a pattern switch label");
    };
    let Pattern::RecordPattern(empty_pattern) = &empty_case.patterns[0] else {
        panic!("expected an empty record pattern");
    };
    assert!(empty_pattern.components.is_empty());

    let Statement::Switch(group_switch) = &statements[1] else {
        panic!("expected a switch statement");
    };
    let cases = switch_colon_cases(&group_switch.cases);
    let Some(SwitchLabel::Pattern(pattern_case)) = &cases[0].label else {
        panic!("expected a pattern switch label");
    };
    assert_eq!(pattern_case.patterns.len(), 2);
    assert!(matches!(&pattern_case.patterns[1], Pattern::RecordPattern(_)));
}

#[test]
fn parses_switch_null_default_labels() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f(Object value) { \
                 switch (value) { \
                     case null, default -> handle(); \
                     case String name -> handle(name); \
                 } \
                 switch (value) { \
                     case null, default: handle(); \
                 } \
             } }",
    );
    assert_eq!(statements.len(), 2);

    let Statement::Switch(rule_switch) = &statements[0] else {
        panic!("expected a switch statement");
    };
    let rules = switch_rules(&rule_switch.cases);
    let Some(SwitchLabel::Case(null_default)) = &rules[0].label else {
        panic!("expected a case label");
    };
    assert_eq!(null_default.constants.len(), 1);
    assert!(matches!(&null_default.constants[0], Expression::NullLiteral(_)));
    assert!(null_default.has_default);

    let Statement::Switch(group_switch) = &statements[1] else {
        panic!("expected a switch statement");
    };
    let cases = switch_colon_cases(&group_switch.cases);
    let Some(SwitchLabel::Case(null_default)) = &cases[0].label else {
        panic!("expected a case label");
    };
    assert_eq!(null_default.constants.len(), 1);
    assert!(matches!(&null_default.constants[0], Expression::NullLiteral(_)));
    assert!(null_default.has_default);
}

#[test]
fn arrow_after_a_case_constant_or_guard_ends_the_label() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f(Object o, int m, boolean b) { \
                 switch (m) { \
                     case (int) X -> g(); \
                     case (int) Y, (int) Z -> (Function<A, B>) a -> b; \
                     default -> (Runnable) () -> {}; \
                 } \
                 switch (o) { \
                     case Integer i when (b) -> g(); \
                     case Long l when b -> g(); \
                     case String s when test(x -> x > 0) -> g(); \
                     default -> g(); \
                 } \
             } }",
    );
    let Statement::Switch(constant_switch) = &statements[0] else {
        panic!("expected a switch statement");
    };
    let rules = switch_rules(&constant_switch.cases);
    let Some(SwitchLabel::Case(one)) = &rules[0].label else {
        panic!("expected a case label");
    };
    let Some(SwitchLabel::Case(two)) = &rules[1].label else {
        panic!("expected a case label");
    };
    assert_eq!(two.constants.len(), 2);
    for constant in one.constants.iter().chain(&two.constants) {
        let Expression::Cast(cast) = constant else {
            panic!("expected a cast constant");
        };
        assert!(matches!(&cast.operand, Expression::Identifier(_)));
    }
    for rule in &rules[1..] {
        let SwitchRuleBody::Cast(cast) = &rule.body else {
            panic!("expected a cast rule body");
        };
        assert!(matches!(&cast.operand, Expression::Lambda(_)));
    }

    let Statement::Switch(pattern_switch) = &statements[1] else {
        panic!("expected a switch statement");
    };
    let rules = switch_rules(&pattern_switch.cases);
    let guards: std::vec::Vec<_> = rules[..3]
        .iter()
        .map(|rule| {
            let Some(SwitchLabel::Pattern(label)) = &rule.label else {
                panic!("expected a pattern switch label");
            };
            label.guard.as_ref().expect("expected a guard")
        })
        .collect();
    assert!(matches!(guards[0], Expression::Parenthesized(_)));
    assert!(matches!(guards[1], Expression::Identifier(_)));
    let Expression::MethodCall(call) = guards[2] else {
        panic!("expected a call guard");
    };
    assert!(matches!(&call.args[0], Expression::Lambda(_)));
}

#[test]
fn yield_identifier_can_start_statement_expressions() {
    let allocator = Allocator::new();
    let result = parse(
        &allocator,
        "class A { boolean[] yield; void f() { \
             yield.set(true); \
             yield = other; \
             yield[0] = true; \
             yield++; \
             yield--; \
             yield += 1; \
             yield -= 1; \
             yield *= 1; \
             yield /= 1; \
             yield %= 1; \
             yield &= 1; \
             yield |= 1; \
             yield ^= 1; \
             yield <<= 1; \
             yield >>= 1; \
             yield >>>= 1; \
         } }",
    );
    assert!(result.errors.is_empty(), "parse errors: {:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Field(field) = &class.body.members[0] else {
        panic!("expected a field");
    };
    assert_eq!(field.declarators[0].name.name, "yield");

    let ClassMember::Method(method) = &class.body.members[1] else {
        panic!("expected a method");
    };
    let statements = &method.body.as_ref().unwrap().statements;
    assert_eq!(statements.len(), 16);

    let Statement::Expression(call_stmt) = &statements[0] else {
        panic!("expected a method-call statement");
    };
    assert!(matches!(&call_stmt.expression, Expression::MethodCall(_)));

    let Statement::Expression(assign_stmt) = &statements[1] else {
        panic!("expected an assignment statement");
    };
    assert!(matches!(&assign_stmt.expression, Expression::Assignment(_)));

    let Statement::Expression(array_assign_stmt) = &statements[2] else {
        panic!("expected an array assignment statement");
    };
    let Expression::Assignment(array_assign) = &array_assign_stmt.expression else {
        panic!("expected an array assignment");
    };
    assert!(matches!(&array_assign.target, AssignmentTarget::ArrayAccess(_)));

    let Statement::Expression(increment_stmt) = &statements[3] else {
        panic!("expected an increment statement");
    };
    assert!(matches!(&increment_stmt.expression, Expression::Update(_)));

    let Statement::Expression(decrement_stmt) = &statements[4] else {
        panic!("expected a decrement statement");
    };
    assert!(matches!(&decrement_stmt.expression, Expression::Update(_)));

    let assignment_ops = statements[5..]
        .iter()
        .map(|statement| {
            let Statement::Expression(statement) = statement else {
                panic!("expected an assignment statement");
            };
            let Expression::Assignment(assignment) = &statement.expression else {
                panic!("expected an assignment expression");
            };
            assignment.op
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        assignment_ops,
        [
            AssignOp::Add,
            AssignOp::Sub,
            AssignOp::Mul,
            AssignOp::Div,
            AssignOp::Rem,
            AssignOp::BitAnd,
            AssignOp::BitOr,
            AssignOp::BitXor,
            AssignOp::Shl,
            AssignOp::Shr,
            AssignOp::Ushr,
        ]
    );
}

#[test]
fn switch_rules_and_colon_cases_do_not_mix() {
    for (source, expected) in [
        ("class A { void f() { switch (x) { case 1 -> g(); case 2: h(); } } }", "`->`"),
        ("class A { void f() { switch (x) { case 1: g(); case 2 -> h(); } } }", "`:`"),
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert_eq!(result.errors.len(), 1, "{source}: {:?}", result.errors);
        assert!(result.errors[0].message.contains(expected), "{source}: {:?}", result.errors);
    }
}

#[test]
fn empty_switch_block_parses() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { switch (x) {} } }");
    let Statement::Switch(switch_stmt) = stmt else {
        panic!("expected a switch statement");
    };
    assert!(switch_stmt.cases.is_empty());
}

#[test]
fn nested_bare_blocks_are_statements() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(&allocator, "class A { void f() { { int x = 1; } } }");
    let Statement::Block(block) = stmt else {
        panic!("expected a block statement");
    };
    assert_eq!(block.statements.len(), 1);
}

#[test]
fn parses_local_class_interface_and_enum_declarations() {
    let allocator = Allocator::new();
    let (statements, _) = first_method_statements(
        &allocator,
        "class A { void f() {\
                @Marker final class Local { int value; }\
                interface Helper { void help(); }\
                enum Kind { ONE, TWO }\
                int after = 1;\
            } }",
    );
    assert_eq!(statements.len(), 4);
    let Statement::Class(local) = &statements[0] else {
        panic!("expected a local class");
    };
    assert_eq!(local.name.name, "Local");
    assert_eq!(local.modifiers.len(), 2);
    assert_eq!(local.body.members.len(), 1);

    let Statement::Interface(helper) = &statements[1] else {
        panic!("expected a local interface");
    };
    assert_eq!(helper.name.name, "Helper");
    assert_eq!(helper.body.members.len(), 1);

    let Statement::Enum(kind) = &statements[2] else {
        panic!("expected a local enum");
    };
    assert_eq!(kind.name.name, "Kind");
    assert_eq!(kind.body.constants.len(), 2);

    assert!(matches!(&statements[3], Statement::LocalVariable(_)));
}

#[test]
fn empty_statements_in_blocks_are_preserved() {
    let allocator = Allocator::new();
    let source = "class A { void f() { ; int x = 1; ; } }";
    let (statements, _) = first_method_statements(&allocator, source);
    assert_eq!(statements.len(), 3);
    let Statement::Empty(first_empty) = &statements[0] else {
        panic!("expected an empty statement");
    };
    assert_eq!(first_empty.span.source_text(source), ";");
    assert!(matches!(&statements[1], Statement::LocalVariable(_)));
    let Statement::Empty(second_empty) = &statements[2] else {
        panic!("expected an empty statement");
    };
    assert_eq!(second_empty.span.source_text(source), ";");
}

#[test]
fn empty_constructor_body_statements_are_preserved() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "class A { A() { ; this(1); ; } A(int x) {} }");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Constructor(ctor) = &class.body.members[0] else {
        panic!("expected a constructor");
    };
    assert_eq!(ctor.body.statements.len(), 3);
    assert!(matches!(&ctor.body.statements[0], Statement::Empty(_)));
    assert!(matches!(&ctor.body.statements[1], Statement::ConstructorInvocation(_)));
    assert!(matches!(&ctor.body.statements[2], Statement::Empty(_)));
}

#[test]
fn empty_statements_in_a_switch_case_are_preserved() {
    let allocator = Allocator::new();
    let (stmt, _) = first_statement(
        &allocator,
        "class A { void f() { switch (x) { case 1: ; f(); ; default: ; } } }",
    );
    let Statement::Switch(switch_stmt) = stmt else {
        panic!("expected a switch statement");
    };
    let cases = switch_colon_cases(&switch_stmt.cases);
    assert_eq!(cases.len(), 2);
    assert_eq!(cases[0].statements.len(), 3);
    assert!(matches!(&cases[0].statements[0], Statement::Empty(_)));
    assert!(matches!(&cases[0].statements[1], Statement::Expression(_)));
    assert!(matches!(&cases[0].statements[2], Statement::Empty(_)));
    assert_eq!(cases[1].statements.len(), 1);
    assert!(matches!(&cases[1].statements[0], Statement::Empty(_)));
}
