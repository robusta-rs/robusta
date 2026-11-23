//! Contextual keywords used as ordinary identifiers (JLS §3.9).
//!
//! Every word here is a legal identifier in the positions the JLS does not
//! restrict, cross-checked against `javac 26.0.2`. These tests pin the
//! admission contexts: a contextual keyword carrying its own [`TokenKind`] must
//! keep every identifier position it has today.

use super::*;

/// The 16 single-word contextual keywords of JLS §3.9, plus `non` — the
/// identifier half of the hyphenated `non-sealed` spelling, which is a keyword
/// only when `-sealed` follows it directly.
const CONTEXTUAL_WORDS: [&str; 17] = [
    "exports",
    "module",
    "non",
    "open",
    "opens",
    "permits",
    "provides",
    "record",
    "requires",
    "sealed",
    "to",
    "transitive",
    "uses",
    "var",
    "when",
    "with",
    "yield",
];

/// The sole class declared by `source`.
fn only_class<'a>(unit: &'a CompilationUnit<'a>, word: &str) -> &'a ClassDeclaration<'a> {
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("{word}: expected a class");
    };
    class
}

#[test]
fn every_contextual_keyword_is_an_ordinary_field_name() {
    for word in CONTEXTUAL_WORDS {
        let allocator = Allocator::new();
        let source = format!("class C {{ int {word}; }}");
        let result = parse(&allocator, &source);
        assert!(result.errors.is_empty(), "{word}: {:?}", result.errors);
        let ClassMember::Field(field) = &only_class(&result.unit, word).body.members[0] else {
            panic!("{word}: expected a field declaration");
        };
        assert_eq!(field.declarators[0].name.name, word);
    }
}

#[test]
fn every_contextual_keyword_is_an_ordinary_method_name() {
    for word in CONTEXTUAL_WORDS {
        let allocator = Allocator::new();
        let source = format!("class C {{ void {word}() {{}} }}");
        let result = parse(&allocator, &source);
        assert!(result.errors.is_empty(), "{word}: {:?}", result.errors);
        let ClassMember::Method(method) = &only_class(&result.unit, word).body.members[0] else {
            panic!("{word}: expected a method declaration");
        };
        assert_eq!(method.name.name, word);
    }
}

#[test]
fn every_contextual_keyword_is_an_ordinary_local_name() {
    for word in CONTEXTUAL_WORDS {
        let allocator = Allocator::new();
        let source = format!("class C {{ void f() {{ int {word} = 0; }} }}");
        let result = parse(&allocator, &source);
        assert!(result.errors.is_empty(), "{word}: {:?}", result.errors);
        let ClassMember::Method(method) = &only_class(&result.unit, word).body.members[0] else {
            panic!("{word}: expected a method declaration");
        };
        let Statement::LocalVariable(local) = &method.body.as_ref().unwrap().statements[0] else {
            panic!("{word}: expected a local variable declaration");
        };
        // `int` is the declared type; the word is only the binder's name.
        assert!(local.ty.is_type(), "{word}: expected an explicit type, not inferred `var`");
        assert_eq!(local.declarators[0].name.name, word);
    }
}

#[test]
fn every_contextual_keyword_is_an_ordinary_parameter_name() {
    for word in CONTEXTUAL_WORDS {
        let allocator = Allocator::new();
        let source = format!("class C {{ void f(int {word}) {{}} }}");
        let result = parse(&allocator, &source);
        assert!(result.errors.is_empty(), "{word}: {:?}", result.errors);
        let ClassMember::Method(method) = &only_class(&result.unit, word).body.members[0] else {
            panic!("{word}: expected a method declaration");
        };
        assert_eq!(method.params.items[0].name.name, word);
    }
}

/// The flip side of the admission tests: each word must still be recognised in
/// the position that makes it a keyword.
#[test]
fn contextual_keywords_still_carry_their_keyword_roles() {
    let allocator = Allocator::new();
    let source = "sealed class Shape permits Circle {}\
                  record Point(int x) {}\
                  class C { void f() { var inferred = 0; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let TypeDeclaration::Class(shape) = &result.unit.types()[0] else {
        panic!("expected a sealed class");
    };
    assert!(matches!(&shape.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::Sealed));
    assert_eq!(shape.permits.len(), 1);

    let TypeDeclaration::Record(point) = &result.unit.types()[1] else {
        panic!("expected a record declaration");
    };
    assert_eq!(point.name.name, "Point");

    let TypeDeclaration::Class(class) = &result.unit.types()[2] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method declaration");
    };
    let Statement::LocalVariable(local) = &method.body.as_ref().unwrap().statements[0] else {
        panic!("expected a local variable declaration");
    };
    assert!(matches!(local.ty, VariableType::Var(_)), "`var` must still infer");
}

/// `non-sealed` is one contextual keyword, but `non` and `sealed` are ordinary
/// identifiers, so the same three characters are subtraction between two
/// variables. Both spellings compile under `javac 26.0.2`; the lexer therefore
/// cannot reduce `non-sealed` to a single token.
#[test]
fn non_sealed_is_a_modifier_but_non_minus_sealed_is_subtraction() {
    let allocator = Allocator::new();
    let source = "non-sealed class FreeShape extends Shape {}\
                  class C { int f() { int non = 10, sealed = 3; return non-sealed; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let TypeDeclaration::Class(free_shape) = &result.unit.types()[0] else {
        panic!("expected a non-sealed class");
    };
    assert!(
        matches!(&free_shape.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::NonSealed)
    );

    let TypeDeclaration::Class(class) = &result.unit.types()[1] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method declaration");
    };
    let Statement::Return(ret) = &method.body.as_ref().unwrap().statements[1] else {
        panic!("expected a return statement");
    };
    let Some(Expression::Binary(binary)) = &ret.value else {
        panic!("expected `non-sealed` to parse as subtraction");
    };
    assert_eq!(binary.op, BinaryOp::Sub);
}

/// JLS §3.2 resolves Unicode escapes in step 1, before §3.5 reduces input
/// characters to input elements — so an escaped contextual keyword *is* the
/// keyword. Every input below compiles under javac 26.0.2.
///
/// The record case is the one that used to fail silently: it parsed as a
/// compact-source method, with a tree that said "method" where the source said
/// "record", and exit status 0.
#[test]
fn an_escaped_record_declares_a_record_not_a_method() {
    let allocator = Allocator::new();
    let source = r"rec\u006frd Point(int x) {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.unit.compact().is_none(), "this is a record, not a compact source file");
    let TypeDeclaration::Record(point) = &result.unit.types()[0] else {
        panic!("expected a record declaration");
    };
    assert_eq!(point.name.name, "Point");
    assert_eq!(point.header.components.len(), 1);
}

#[test]
fn an_escaped_sealed_header_keeps_its_modifier_and_permits_clause() {
    let allocator = Allocator::new();
    let source = r"s\u0065aled interface A p\u0065rmits B {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let TypeDeclaration::Interface(a) = &result.unit.types()[0] else {
        panic!("expected an interface declaration");
    };
    assert!(matches!(&a.modifiers[0], Modifier::Keyword(k) if k.kind == ModifierKind::Sealed));
    assert_eq!(a.permits.len(), 1);
    assert_eq!(type_name_parts(&a.permits[0]), ["B"]);
}

#[test]
fn an_escaped_module_declaration_parses_with_its_directives() {
    let allocator = Allocator::new();
    let source = r"m\u006fdule a.b { r\u0065quires c.d; \u0065xports p; \u006fpens q; }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let module = result.unit.module().expect("expected a module declaration");
    let [
        ModuleDirective::Requires(_),
        ModuleDirective::Package(exports),
        ModuleDirective::Package(opens),
    ] = &module.directives[..]
    else {
        panic!("expected requires, exports, and opens directives");
    };
    assert_eq!(exports.kind, PackageDirectiveKind::Exports);
    assert_eq!(opens.kind, PackageDirectiveKind::Opens);
}
