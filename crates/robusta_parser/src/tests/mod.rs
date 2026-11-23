use robusta_ast::{KeywordModifier, MethodReference, SwitchBlockStatementGroup, SwitchRule};
use robusta_syntax::node::NodeId;

use super::*;

mod annotations;
mod compilation_units;
mod contextual_keywords;
mod declarations;
mod expressions;
mod recovery;
mod statements;
mod types;

/// The single statement of `void f() { … }` in `source`.
fn first_statement<'a>(
    allocator: &'a Allocator,
    source: &'a str,
) -> (&'a Statement<'a>, &'a CompilationUnit<'a>) {
    let result = parse(allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = allocator.alloc(result.unit);
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    (&method.body.as_ref().unwrap().statements[0], unit)
}

/// The statements of the first `void f() { ... }`-style method in `source`.
fn first_method_statements<'a>(
    allocator: &'a Allocator,
    source: &'a str,
) -> (&'a [Statement<'a>], &'a CompilationUnit<'a>) {
    let result = parse(allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = allocator.alloc(result.unit);
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    (&method.body.as_ref().unwrap().statements[..], unit)
}

/// The arrow rules of a switch whose cases are all rules.
fn switch_rules<'s, 'a>(cases: &'s [SwitchCase<'a>]) -> std::vec::Vec<&'s SwitchRule<'a>> {
    let rule = |case: &'s SwitchCase<'a>| match case {
        SwitchCase::Rule(rule) => rule.as_ref(),
        SwitchCase::Group(_) => panic!("expected an arrow rule"),
    };
    cases.iter().map(rule).collect()
}

/// The colon groups of a switch whose cases are all groups.
fn switch_groups<'s, 'a>(
    cases: &'s [SwitchCase<'a>],
) -> std::vec::Vec<&'s SwitchBlockStatementGroup<'a>> {
    let group = |case: &'s SwitchCase<'a>| match case {
        SwitchCase::Group(group) => group.as_ref(),
        SwitchCase::Rule(_) => panic!("expected a colon group"),
    };
    cases.iter().map(group).collect()
}

/// The names of a package, import, or module name in source order.
fn name_parts<'a>(name: &Name<'a>) -> std::vec::Vec<&'a str> {
    let mut parts: std::vec::Vec<_> = name.names_from_last().collect();
    parts.reverse();
    parts
}

/// The names of an annotation, `permits`, `uses`, or `provides` name in source
/// order.
fn type_name_parts<'a>(name: &TypeName<'a>) -> std::vec::Vec<&'a str> {
    let mut parts = std::vec::Vec::new();
    let mut name = name;
    while let TypeName::Qualified(qualified) = name {
        parts.push(qualified.name.name);
        name = &qualified.qualifier;
    }
    parts.push(name.last());
    parts.reverse();
    parts
}

/// The names of a dotted expression such as `a.b.c` in source order.
///
/// Panics on anything but identifiers joined by field accesses.
fn expression_names<'a>(expression: &Expression<'a>) -> std::vec::Vec<&'a str> {
    let mut names = std::vec::Vec::new();
    let mut expression = expression;
    while let Expression::FieldAccess(access) = expression {
        names.push(access.name.name);
        expression = &access.target;
    }
    let Expression::Identifier(identifier) = expression else {
        panic!("expected a dotted name, found {expression:?}");
    };
    names.push(identifier.name);
    names.reverse();
    names
}

/// The names of a class type in source order, excluding annotations and
/// arguments: `java.util.Map.Entry<K, V>` gives `java`, `util`, `Map`, `Entry`.
fn class_type_names<'a>(ty: &ClassType<'a>) -> std::vec::Vec<&'a str> {
    let mut names: std::vec::Vec<_> = ty.self_and_qualifiers().map(ClassType::name).collect();
    names.reverse();
    names
}

/// A class type and those it is qualified by in source order, so that
/// `Map.Entry<K, V>` gives `Map` and `Map.Entry<K, V>`.
///
/// A segment's `name()`, `args()`, and [`class_type_annotations`] are those of
/// the name it ends with.
fn class_type_parts<'s, 'a>(ty: &'s ClassType<'a>) -> std::vec::Vec<&'s ClassType<'a>> {
    let mut segments: std::vec::Vec<_> = ty.self_and_qualifiers().collect();
    segments.reverse();
    segments
}

/// The annotations before the last name of a class type: `@A` in `@A Entry` and
/// in `Map.@A Entry`.
fn class_type_annotations<'s, 'a>(ty: &'s ClassType<'a>) -> &'s [Annotation<'a>] {
    match ty {
        ClassType::Simple(ty) => &ty.annotations,
        ClassType::Qualified(ty) => &ty.annotations,
    }
}

/// The annotations before a primitive or class type: `@A` in `@A int` and in
/// `@A Object`. Panics on an array type.
fn leading_annotations<'s, 'a>(ty: &'s Type<'a>) -> &'s [Annotation<'a>] {
    match ty {
        Type::Primitive(primitive) => &primitive.annotations,
        Type::Array(_) => panic!("expected a primitive or class type"),
        Type::Simple(_) | Type::Qualified(_) => {
            class_type_annotations(class_type_parts(ty.to_class_type())[0])
        }
    }
}

fn assignment_value<'a>(stmt: &'a Statement<'a>) -> &'a Expression<'a> {
    let Statement::Expression(expr_stmt) = stmt else {
        panic!("expected an expression statement");
    };
    let Expression::Assignment(assignment) = &expr_stmt.expression else {
        panic!("expected an assignment");
    };
    &assignment.value
}
