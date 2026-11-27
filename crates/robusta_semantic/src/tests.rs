use std::cell::Cell;

use robusta_allocator::Allocator;
use robusta_ast::{AstKind, ClassMember, ImportKind, TypeDeclaration};
use robusta_ast_visit::Visit;
use robusta_parser::parse;
use robusta_span::{GetSpan, Span};

use crate::{
    NodeId, Reference, ReferenceId, ReferenceKind, ScopeId, SemanticBuilder, Symbol, SymbolId,
    SymbolKind,
};

#[test]
fn analysis_replaces_parser_ids_with_the_dense_table_index() {
    for source in [
        "package a; import java.util.List; class C { int f; void m() { return; } }",
        "import java.util.List; int f; void main() { return; }",
        "import java.util.List; open module m { requires transitive java.base; }",
    ] {
        let allocator = Allocator::new();
        let parsed = parse(&allocator, source);
        assert!(parsed.errors.is_empty(), "{source}: {:?}", parsed.errors);
        let parsed_form_id = parsed.unit.content.node_id();
        parsed.unit.set_node_id(NodeId::new(9));

        let semantic = SemanticBuilder::new().build(source, &parsed.unit);
        let nodes = semantic.nodes();
        assert!(nodes.len() > 2, "{source}");
        for (id, node) in nodes.iter_enumerated() {
            assert_eq!(node.id(), id, "{source}: {}", node.kind().debug_name());
        }
        assert_eq!(parsed.unit.node_id(), NodeId::ROOT, "{source}");
        assert_ne!(parsed_form_id, NodeId::new(1), "{source}: the parser builds the form last");
        assert_eq!(parsed.unit.content.node_id(), NodeId::new(1), "{source}");
        assert_eq!(nodes.parent_id(parsed.unit.content.node_id()), NodeId::ROOT, "{source}");
        assert!(
            nodes.iter_enumerated().skip(1).all(|(id, _)| nodes.parent_id(id) < id),
            "{source}: a parent comes before its children"
        );
    }
}

#[test]
fn a_choice_reads_the_id_of_the_node_it_holds() {
    let source = "class C { void m() { return; } }";
    let allocator = Allocator::new();
    let parsed = parse(&allocator, source);
    assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
    let semantic = SemanticBuilder::new().build(source, &parsed.unit);

    let TypeDeclaration::Class(class) = &parsed.unit.types()[0] else {
        panic!("expected a class");
    };
    let ClassMember::Method(method) = &class.body.members[0] else {
        panic!("expected a method");
    };
    let statement = &method.body.as_ref().unwrap().statements[0];
    assert!(matches!(semantic.nodes().kind(statement.node_id()), AstKind::ReturnStatement(_)));
    assert!(matches!(
        semantic.nodes().kind(class.body.members[0].node_id()),
        AstKind::MethodDeclaration(_)
    ));
    assert!(matches!(
        semantic.nodes().kind(parsed.unit.types()[0].node_id()),
        AstKind::ClassDeclaration(_)
    ));

    statement.set_node_id(NodeId::new(0));
    assert_eq!(statement.node_id(), NodeId::new(0));
}

#[test]
fn long_binary_chains_keep_preorder_and_parent_links() {
    let source = format!("class A {{ int n = 0{}; }}", " + 1".repeat(20_000));
    let allocator = Allocator::new();
    let parsed = parse(&allocator, &source);
    assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
    let semantic = SemanticBuilder::new().build(&source, &parsed.unit);
    let nodes = semantic.nodes();
    let binaries = nodes
        .iter_enumerated()
        .filter(|(_, node)| matches!(node.kind(), AstKind::BinaryExpression(_)))
        .collect::<Vec<_>>();
    assert_eq!(binaries.len(), 20_000);
    for pair in binaries.windows(2) {
        assert_eq!(nodes.parent_id(pair[1].0), pair[0].0);
    }
    let literals = nodes
        .iter_enumerated()
        .filter_map(|(id, node)| matches!(node.kind(), AstKind::IntLiteral(_)).then_some(id))
        .collect::<Vec<_>>();
    assert_eq!(literals.len(), 20_001);
    assert_eq!(nodes.parent_id(literals[0]), binaries[19_999].0);
    for (&literal, (parent, _)) in literals[1..].iter().zip(binaries.iter().rev()) {
        assert_eq!(nodes.parent_id(literal), *parent);
    }
}

#[test]
fn long_receiver_chains_keep_preorder_and_parent_links() {
    for (tail, repeat, links) in [
        (".f()", 20_000, 2),
        (".field", 20_000, 1),
        ("[0]", 20_000, 1),
        (".new A()", 20_000, 1),
        ("::f", 20_000, 1),
        (" instanceof A", 20_000, 1),
        (".f()[0].field.new A()::f", 4_000, 6),
    ] {
        let source = format!("class A {{ Object n = f(){}; }}", tail.repeat(repeat));
        let allocator = Allocator::new();
        let parsed = parse(&allocator, &source);
        assert!(parsed.errors.is_empty(), "{tail}: {:?}", parsed.errors);
        let semantic = SemanticBuilder::new().build(&source, &parsed.unit);
        let nodes = semantic.nodes();
        let chains = nodes
            .iter_enumerated()
            .filter(|(_, node)| {
                matches!(
                    node.kind(),
                    AstKind::FieldAccess(_)
                        | AstKind::MethodCall(_)
                        | AstKind::MethodMember(_)
                        | AstKind::QualifiedClassInstanceCreation(_)
                        | AstKind::ArrayAccess(_)
                        | AstKind::InstanceofExpression(_)
                        | AstKind::MethodReference(_)
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(chains.len(), repeat * links + 1, "{tail}");
        for pair in chains.windows(2) {
            assert_eq!(nodes.parent_id(pair[1].0), pair[0].0, "{tail}");
        }
    }
}

#[test]
fn long_dotted_names_keep_preorder_and_parent_links() {
    let dots = ".a".repeat(20_000);
    for (source, kind) in [
        (format!("package a{dots};"), "QualifiedName"),
        (format!("import a{dots};"), "QualifiedName"),
        (format!("@a{dots} class A {{}}"), "QualifiedTypeName"),
        (format!("class A {{ a{dots} f; }}"), "QualifiedClassType"),
        (format!("class A {{ Object f = a{dots}; }}"), "FieldAccess"),
    ] {
        let allocator = Allocator::new();
        let parsed = parse(&allocator, &source);
        assert!(parsed.errors.is_empty(), "{kind}: {:?}", parsed.errors);
        let semantic = SemanticBuilder::new().build(&source, &parsed.unit);
        let nodes = semantic.nodes();
        let chain = nodes
            .iter_enumerated()
            .filter(|(_, node)| node.kind().debug_name() == kind)
            .collect::<Vec<_>>();
        assert_eq!(chain.len(), 20_000, "{kind}");
        for pair in chain.windows(2) {
            assert_eq!(nodes.parent_id(pair[1].0), pair[0].0, "{kind}");
        }
    }
}

/// The names in `example`, inside `source`, by the role they have: bindings,
/// references, and names that are never looked up.
fn names_by_role<'a>(source: &'a str, example: &str) -> [Vec<&'a str>; 3] {
    let start = u32::try_from(source.find(example).expect("the example")).unwrap();
    let span = Span::sized(start, u32::try_from(example.len()).unwrap());
    let allocator = Allocator::new();
    let parsed = parse(&allocator, source);
    assert!(parsed.errors.is_empty(), "{source}: {:?}", parsed.errors);
    let semantic = SemanticBuilder::new().build(source, &parsed.unit);
    let mut roles = [Vec::new(), Vec::new(), Vec::new()];
    for node in semantic.nodes().iter() {
        let (role, name) = match node.kind() {
            AstKind::BindingIdentifier(name) => {
                assert!(name.symbol_id.get().is_some(), "{source}: {}", name.name);
                (0, name.span)
            }
            AstKind::IdentifierReference(name) => {
                assert!(name.reference_id.get().is_some(), "{source}: {}", name.name);
                (1, name.span)
            }
            AstKind::IdentifierName(name) => (2, name.span),
            _ => continue,
        };
        if span.contains_inclusive(name) {
            roles[role].push(name.source_text(source));
        }
    }
    roles
}

#[test]
fn names_have_the_role_their_position_gives_them() {
    let none: [&str; 0] = [];
    for (source, example, bindings, references, names) in [
        (
            "class C { void m() { System.out.println(x); } }",
            "System.out.println(x)",
            &none[..],
            &["System", "x"][..],
            &["out", "println"][..],
        ),
        (
            "class C { Object f = String::valueOf; }",
            "String::valueOf",
            &none,
            &["String"],
            &["valueOf"],
        ),
        (
            "class C { void m() { Map.Entry<K, V> e; } }",
            "Map.Entry<K, V> e",
            &["e"],
            &["Map", "K", "V"],
            &["Entry"],
        ),
        ("class C { Object f = o.new Inner(); }", "o.new Inner()", &none, &["o"], &["Inner"]),
        (
            "@SuppressWarnings(value = \"unused\") class C {}",
            "@SuppressWarnings(value = \"unused\")",
            &none,
            &["SuppressWarnings"],
            &["value"],
        ),
        (
            "class Outer { class Inner { Inner(Outer Outer.this) {} } }",
            "Inner(Outer Outer.this) {}",
            &none,
            &["Outer"],
            &["Inner", "Outer"],
        ),
        (
            "import java.util.Map.Entry;",
            "import java.util.Map.Entry;",
            &["Entry"],
            &none,
            &["java", "util", "Map", "Entry"],
        ),
        ("class C { void m() { this.<T>f(); } }", "this.<T>f()", &none, &["T"], &["f"]),
        (
            "class C { java.util.@A Map<K>.Entry<V> f; }",
            "java.util.@A Map<K>.Entry<V> f",
            &["f"],
            &["java", "A", "K", "V"],
            &["util", "Map", "Entry"],
        ),
        (
            "class C { void m() { outer: while (true) break outer; } }",
            "outer: while (true) break outer;",
            &none,
            &none,
            &none,
        ),
        ("class C { C() {} }", "C() {}", &none, &none, &["C"]),
        ("record R(int a) { R { } }", "R { }", &none, &none, &["R"]),
        (
            r"class C { int f = Sys\u0074em.\u006fut; }",
            r"Sys\u0074em.\u006fut",
            &none,
            &[r"Sys\u0074em"],
            &[r"\u006fut"],
        ),
    ] {
        let [found_bindings, found_references, found_names] = names_by_role(source, example);
        assert_eq!(found_bindings, bindings, "{source}");
        assert_eq!(found_references, references, "{source}");
        assert_eq!(found_names, names, "{source}");
    }
}

#[test]
fn only_a_single_type_or_static_import_binds() {
    let source = "package p.q; import a.*; import static a.B.*; import a.List; \
        import a.List; import static a.B.max; class C {}";
    let allocator = Allocator::new();
    let parsed = parse(&allocator, source);
    assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
    let semantic = SemanticBuilder::new().build(source, &parsed.unit);
    let imports = semantic
        .scoping()
        .symbols()
        .iter()
        .filter(|symbol| matches!(symbol.kind(), SymbolKind::Import(_)))
        .map(Symbol::name)
        .collect::<Vec<_>>();
    assert_eq!(imports, ["List", "List", "max"]);
    assert!(semantic.scoping().symbols().iter().all(|symbol| symbol.name() != "q"));
}

#[test]
fn a_member_name_is_not_looked_up() {
    let source = "class C { int out; void println() {} C() {} \
        void m(C c) { c.out = 1; c.println(); this.out++; Object f = c::println; g(); } }";
    let allocator = Allocator::new();
    let parsed = parse(&allocator, source);
    assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
    let semantic = SemanticBuilder::new().build(source, &parsed.unit);
    let references = semantic
        .scoping()
        .references()
        .iter()
        .map(|reference| (reference.name(), reference.kind()))
        .collect::<Vec<_>>();
    assert_eq!(
        references,
        [
            ("C", ReferenceKind::Type),
            ("c", ReferenceKind::Read),
            ("c", ReferenceKind::Read),
            ("Object", ReferenceKind::Type),
            ("c", ReferenceKind::Read),
            ("g", ReferenceKind::Method),
        ]
    );
    let names = semantic.scoping().symbols().iter().map(Symbol::name).collect::<Vec<_>>();
    assert_eq!(names, ["C", "out", "println", "m", "c", "f"]);
}

#[test]
fn a_variable_is_written_only_as_the_operand_of_an_assignment_or_update() {
    use ReferenceKind::{Read, ReadWrite, Write};
    for (statement, kinds) in [
        ("a = b;", &[("a", Write), ("b", Read)][..]),
        ("a += b;", &[("a", ReadWrite), ("b", Read)]),
        ("a = b = c;", &[("a", Write), ("b", Write), ("c", Read)]),
        ("a = (b += c);", &[("a", Write), ("b", ReadWrite), ("c", Read)]),
        ("a++; b--;", &[("a", ReadWrite), ("b", ReadWrite)]),
        ("++a; --b;", &[("a", ReadWrite), ("b", ReadWrite)]),
        ("a = -b + ~c;", &[("a", Write), ("b", Read), ("c", Read)]),
        ("(a)++; --((b));", &[("a", ReadWrite), ("b", ReadWrite)]),
        ("(a) = b; ((a)) += c;", &[("a", Write), ("b", Read), ("a", ReadWrite), ("c", Read)]),
        (
            "(x.f) = a; (y[b]) += c;",
            &[("x", Read), ("a", Read), ("y", Read), ("b", Read), ("c", Read)],
        ),
        ("(a = b).hashCode();", &[("a", Write), ("b", Read)]),
        ("x.f = a; x.f++;", &[("x", Read), ("a", Read), ("x", Read)]),
        ("y[a] = b; y[c]++;", &[("y", Read), ("a", Read), ("b", Read), ("y", Read), ("c", Read)]),
        ("y[a = b] = c;", &[("y", Read), ("a", Write), ("b", Read), ("c", Read)]),
        ("a = a + 1;", &[("a", Write), ("a", Read)]),
    ] {
        let source = format!(
            "class C {{ int f; void m(Integer a, int b, int c, C x, int[] y) {{ {statement} }} }}"
        );
        let allocator = Allocator::new();
        let parsed = parse(&allocator, &source);
        assert!(parsed.errors.is_empty(), "{source}: {:?}", parsed.errors);
        let semantic = SemanticBuilder::new().build(&source, &parsed.unit);
        let found = semantic
            .scoping()
            .references()
            .iter()
            .filter(|reference| reference.kind() != ReferenceKind::Type)
            .filter(|reference| reference.kind() != ReferenceKind::Method)
            .map(|reference| (reference.name(), reference.kind()))
            .collect::<Vec<_>>();
        assert_eq!(found, kinds, "{statement}");
    }
}

#[test]
fn an_annotation_element_value_is_looked_up_as_a_variable() {
    let source = "import a.X; class C { static final int X = 1; @B(X) @D(X.y) int z; }";
    let allocator = Allocator::new();
    let parsed = parse(&allocator, source);
    assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
    let semantic = SemanticBuilder::new().build(source, &parsed.unit);
    let scoping = semantic.scoping();
    let references = scoping
        .references()
        .iter()
        .map(|reference| {
            (
                reference.name(),
                reference.kind(),
                reference.symbol_id().map(|symbol| scoping.symbol(symbol).kind()),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        references,
        [
            ("B", ReferenceKind::Type, None),
            ("X", ReferenceKind::Read, Some(SymbolKind::Field)),
            ("D", ReferenceKind::Type, None),
            ("X", ReferenceKind::Read, Some(SymbolKind::Field)),
        ]
    );
}

#[test]
fn relationship_indexes_keep_declaration_and_source_order() {
    let source = "class A { int value; void use() {} void use(int value) { \
        int copy = value; { int value = copy; use(value); } use(value); } }";
    let allocator = Allocator::new();
    let parsed = parse(&allocator, source);
    assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
    let semantic = SemanticBuilder::new().build(source, &parsed.unit);
    let scoping = semantic.scoping();

    for scope_id in scoping.scopes.indices() {
        let expected: Vec<_> = scoping
            .scopes
            .iter_enumerated()
            .filter_map(|(id, child)| (child.parent_id() == Some(scope_id)).then_some(id))
            .collect();
        assert_eq!(scoping.scope_children(scope_id), expected);
        for (&name, candidates) in &*scoping.cell.borrow_dependent().bindings[scope_id] {
            let expected: Vec<_> = scoping
                .symbols_enumerated()
                .filter_map(|(id, symbol)| {
                    (symbol.scope_id() == scope_id && symbol.ident() == name).then_some(id)
                })
                .collect();
            assert_eq!(&candidates[..], expected);
        }
    }
    for (symbol_id, symbol) in scoping.symbols_enumerated() {
        let expected: Vec<_> = scoping
            .references
            .iter_enumerated()
            .filter_map(|(id, reference)| (reference.symbol_id() == Some(symbol_id)).then_some(id))
            .collect();
        assert_eq!(scoping.resolved_reference_ids(symbol_id), expected);
        let expected: Vec<_> = scoping
            .symbols_enumerated()
            .filter_map(|(id, candidate)| {
                (candidate.declaration_node_id() == symbol.declaration_node_id()).then_some(id)
            })
            .collect();
        assert_eq!(
            scoping
                .symbols_for_declaration(symbol.declaration_node_id())
                .map(|(id, _)| id)
                .collect::<Vec<_>>(),
            expected
        );
    }
    let overloads: Vec<_> =
        scoping.symbols_enumerated().filter(|(_, symbol)| symbol.name() == "use").collect();
    assert_eq!(overloads.len(), 1);
    assert_eq!(scoping.references_for_symbol(overloads[0].0).count(), 2);
    assert_eq!(scoping.symbol_declarations(overloads[0].0).count(), 2);
}

#[test]
fn relationship_storage_survives_owner_moves_and_independent_drops() {
    use std::fmt::Write;

    let mut source = String::from("class A { int shared;");
    for n in 0..32 {
        write!(
            source,
            "int first{n}, second{n}; void use{n}() {{ first{n} = second{n}; shared++; }}"
        )
        .unwrap();
    }
    source.push('}');
    let allocator = Allocator::new();
    let parsed = parse(&allocator, &source);
    assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
    let mut results = vec![
        SemanticBuilder::new().build(&source, &parsed.unit),
        SemanticBuilder::new().build(&source, &parsed.unit),
    ];
    results.swap(0, 1);
    drop(results.pop());
    let result = results.pop().unwrap();
    drop(results);

    let scoping = result.scoping();
    let (shared, _) =
        scoping.symbols_enumerated().find(|(_, symbol)| symbol.name() == "shared").unwrap();
    assert_eq!(scoping.references_for_symbol(shared).count(), 32);
    for (id, symbol) in
        scoping.symbols_enumerated().filter(|(_, symbol)| symbol.name().starts_with("first"))
    {
        let references: Vec<_> = scoping.references_for_symbol(id).collect();
        assert_eq!(references.len(), 1);
        assert_eq!(references[0].symbol_id(), Some(id));
        let declarations: Vec<_> = scoping
            .symbols_for_declaration(symbol.declaration_node_id())
            .map(|(_, symbol)| symbol.name())
            .collect();
        assert_eq!(declarations, [symbol.name()]);
    }
}

#[test]
fn escaped_identifiers_share_their_cooked_symbol_identity() {
    for (declaration, reference, cooked) in [
        (r"f\u006fo", "foo", "foo"),
        ("foo", r"f\u006fo", "foo"),
        (r"\u0066oo", r"f\uuuu006fo", "foo"),
        (r"\uD835\uDC82", "𝒂", "𝒂"),
        ("𝒂", r"\uD835\uDC82", "𝒂"),
        (r"rec\u006frd", "record", "record"),
    ] {
        let source =
            format!("class A {{ int f() {{ int {declaration} = 1; return {reference}; }} }}");
        let allocator = Allocator::new();
        let result = parse(&allocator, &source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        let semantic = SemanticBuilder::new().build(&source, &result.unit);
        let scoping = semantic.scoping();
        let (symbol_id, symbol) = scoping
            .symbols_enumerated()
            .find(|(_, symbol)| symbol.kind() == SymbolKind::LocalVariable)
            .expect("local variable");
        assert_eq!(symbol.name(), cooked, "{source}");
        assert_eq!(symbol.name_span().source_text(&source), declaration);
        assert!(scoping.symbol_is_read(symbol_id), "{source}");
        let references = scoping.references_for_symbol(symbol_id).collect::<std::vec::Vec<_>>();
        assert_eq!(references.len(), 1, "{source}");
        assert_eq!(references[0].name(), cooked);
        assert_eq!(references[0].span().source_text(&source), reference);
        let identifiers = semantic
            .nodes()
            .iter()
            .filter_map(|node| match node.kind() {
                AstKind::BindingIdentifier(binding)
                    if binding.span.source_text(&source) == declaration =>
                {
                    Some(binding.name)
                }
                AstKind::IdentifierReference(name)
                    if name.span.source_text(&source) == reference =>
                {
                    Some(name.name)
                }
                _ => None,
            })
            .collect::<std::vec::Vec<_>>();
        assert_eq!(identifiers, [cooked, cooked]);
    }
}

#[test]
fn identifier_ignorable_spellings_share_names_and_preserve_spans() {
    for ch in [
        '\u{0000}', '\u{0008}', '\u{000e}', '\u{001b}', '\u{007f}', '\u{0085}', '\u{009f}',
        '\u{00ad}', '\u{200c}', '\u{2060}', '\u{feff}',
    ] {
        let escaped = format!("\\u{:04x}", u32::from(ch));
        for spelling in [format!("coun{ch}t"), format!("coun{escaped}t")] {
            for (declaration, reference) in [(&spelling[..], "count"), ("count", &spelling[..])] {
                for (use_site, kind) in [
                    (format!("return {reference};"), ReferenceKind::Read),
                    (format!("{reference} = 1; return 0;"), ReferenceKind::Write),
                ] {
                    let source =
                        format!("class A {{ int f() {{ int {declaration} = 1; {use_site} }} }}");
                    let allocator = Allocator::new();
                    let parsed = parse(&allocator, &source);
                    assert!(parsed.errors.is_empty(), "{source:?}: {:?}", parsed.errors);
                    let semantic = SemanticBuilder::new().build(&source, &parsed.unit);
                    let scoping = semantic.scoping();
                    let (id, symbol) = scoping
                        .symbols_enumerated()
                        .find(|(_, symbol)| symbol.kind() == SymbolKind::LocalVariable)
                        .unwrap();
                    assert_eq!(symbol.name(), "count");
                    assert_eq!(symbol.name_span().source_text(&source), declaration);
                    let references: Vec<_> = scoping.references_for_symbol(id).collect();
                    assert_eq!(references.len(), 1, "{source:?}");
                    assert_eq!(references[0].name(), "count");
                    assert_eq!(references[0].span().source_text(&source), reference);
                    assert_eq!(references[0].kind(), kind);
                    assert_eq!(scoping.symbol_is_read(id), kind.is_read());
                }
            }
        }
    }
}

#[test]
fn supplementary_format_characters_keep_compiler_distinct_bindings() {
    for (literal, escaped) in
        [("coun\u{e0001}t", r"coun\uDB40\uDC01t"), ("coun\u{1d173}t", r"coun\uD834\uDD73t")]
    {
        for (declaration, reference) in [(literal, escaped), (escaped, literal)] {
            let source = format!(
                "class A {{ int f() {{ int {declaration}; int count; \
                 {reference} = 1; count = 2; return {reference} + count; }} }}"
            );
            let allocator = Allocator::new();
            let parsed = parse(&allocator, &source);
            assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
            let semantic = SemanticBuilder::new().build(&source, &parsed.unit);
            let scoping = semantic.scoping();
            let locals: Vec<_> = scoping
                .symbols_enumerated()
                .filter(|(_, symbol)| symbol.kind() == SymbolKind::LocalVariable)
                .collect();
            assert_eq!(locals.len(), 2);
            for ((id, symbol), expected) in locals.into_iter().zip([literal, "count"]) {
                assert_eq!(symbol.name(), expected);
                assert!(scoping.symbol_is_read(id));
                assert_eq!(scoping.references_for_symbol(id).count(), 2);
            }
        }
    }
}

#[test]
fn identifier_identity_preserves_shadowing_and_unicode_distinctions() {
    let source = r"class A { int coun\u200Ct; void f(int count) { count++; } }";
    let allocator = Allocator::new();
    let parsed = parse(&allocator, source);
    assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
    let semantic = SemanticBuilder::new().build(source, &parsed.unit);
    let scoping = semantic.scoping();
    let (field, _) = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.kind() == SymbolKind::Field)
        .unwrap();
    let (parameter, symbol) = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.kind() == SymbolKind::Parameter)
        .unwrap();
    assert_eq!(symbol.shadows(), Some(field));
    assert!(scoping.symbol_is_read(parameter));
    assert!(!scoping.symbol_is_read(field));

    for (declaration, reference) in
        [("Á", "A\u{0301}"), ("a", "α"), ("count", "coun\u{034f}t"), ("count", "coun\u{fe0f}t")]
    {
        let source =
            format!("class A {{ int f() {{ int {declaration} = 1; return {reference}; }} }}");
        let allocator = Allocator::new();
        let parsed = parse(&allocator, &source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let semantic = SemanticBuilder::new().build(&source, &parsed.unit);
        let scoping = semantic.scoping();
        let (id, symbol) = scoping
            .symbols_enumerated()
            .find(|(_, symbol)| symbol.kind() == SymbolKind::LocalVariable)
            .unwrap();
        assert_eq!(symbol.name(), declaration);
        assert!(!scoping.symbol_is_read(id));
        assert!(
            scoping
                .references()
                .iter()
                .find(|r| r.name() == reference)
                .unwrap()
                .symbol_id()
                .is_none()
        );
    }
}

#[test]
fn escaped_imports_and_type_references_share_the_cooked_qualified_name() {
    for source in [
        r"import java.ut\u0069l.L\u0069st; class A { List field; }",
        r"import java.util.List; class A { L\u0069st field; }",
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert_eq!(result.unit.imports()[0].name.dotted(), "java.util.List");
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let scoping = semantic.scoping();
        let (symbol_id, symbol) = scoping
            .symbols_enumerated()
            .find(|(_, symbol)| matches!(symbol.kind(), SymbolKind::Import(_)))
            .expect("import symbol");
        assert_eq!(symbol.name(), "List");
        assert!(scoping.symbol_is_referenced(symbol_id));
    }
}

#[test]
fn semantic_ids_leave_a_niche_for_option() {
    for (id_size, option_size) in [
        (std::mem::size_of::<NodeId>(), std::mem::size_of::<Option<NodeId>>()),
        (std::mem::size_of::<ScopeId>(), std::mem::size_of::<Option<ScopeId>>()),
        (std::mem::size_of::<SymbolId>(), std::mem::size_of::<Option<SymbolId>>()),
        (std::mem::size_of::<ReferenceId>(), std::mem::size_of::<Option<ReferenceId>>()),
    ] {
        assert_eq!(id_size, std::mem::size_of::<u32>());
        assert_eq!(option_size, id_size);
    }
}

#[test]
fn flattens_concrete_nodes_with_parent_links() {
    let allocator = Allocator::new();
    let source = "class Outer { void run() { if (ready) { call(10l, value -> value + 1); } ; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let nodes = semantic.nodes();

    let traversal: std::vec::Vec<_> = nodes
        .iter_enumerated()
        .map(|(id, node)| (node.kind().debug_name(), nodes.parent_id(id).index()))
        .collect();
    assert_eq!(
        traversal,
        [
            ("CompilationUnit", 0),
            // The unit's form is its own node (JLS §19), so every top-level
            // declaration hangs off the ordinary unit rather than the root.
            ("OrdinaryCompilationUnit", 0),
            ("ClassDeclaration", 1),
            ("BindingIdentifier", 2),
            ("ClassBody", 2),
            ("MethodDeclaration", 4),
            ("VoidType", 5),
            ("BindingIdentifier", 5),
            ("FormalParameters", 5),
            ("Block", 5),
            ("IfStatement", 9),
            ("IdentifierReference", 10),
            ("Block", 10),
            ("ExpressionStatement", 12),
            ("MethodCall", 13),
            ("IdentifierReference", 14),
            ("IntLiteral", 14),
            ("LambdaExpression", 14),
            ("FormalParameters", 17),
            ("FormalParameter", 18),
            ("BindingIdentifier", 19),
            ("BinaryExpression", 17),
            ("IdentifierReference", 21),
            ("IntLiteral", 21),
            ("EmptyStatement", 9),
        ]
    );

    assert!(!nodes.is_empty());
    assert!(matches!(nodes.kind(NodeId::ROOT), AstKind::CompilationUnit(_)));
    assert_eq!(nodes.parent_id(NodeId::ROOT), NodeId::ROOT);

    for (id, _) in nodes.iter_enumerated().skip(1) {
        assert!(nodes.parent_id(id).index() < id.index());
    }

    let (literal_id, _) = nodes
        .iter_enumerated()
        .find(
            |(_, node)| matches!(node.kind(), AstKind::IntLiteral(literal) if literal.raw == "10l"),
        )
        .expect("expected the long literal");
    assert!(matches!(nodes.parent_kind(literal_id), AstKind::MethodCall(_)));

    let (lambda_id, _) = nodes
        .iter_enumerated()
        .find(|(_, node)| matches!(node.kind(), AstKind::LambdaExpression(_)))
        .expect("expected the lambda");
    assert!(matches!(nodes.parent_kind(lambda_id), AstKind::MethodCall(_)));

    let ancestor_kinds: std::vec::Vec<_> = nodes.ancestor_kinds(lambda_id).collect();
    assert!(matches!(ancestor_kinds[0], AstKind::MethodCall(_)));
    assert!(matches!(ancestor_kinds.last(), Some(AstKind::CompilationUnit(_))));

    let empty_id = nodes
        .iter_enumerated()
        .find_map(|(id, node)| matches!(node.kind(), AstKind::EmptyStatement(_)).then_some(id))
        .expect("expected the empty statement");
    assert!(matches!(nodes.parent_kind(empty_id), AstKind::Block(_)));
}

#[test]
fn flattens_modular_and_compact_compilation_units() {
    let allocator = Allocator::new();
    let sources = [
        "@Deprecated open module app.core {\
             requires transitive java.base;\
             exports app.api to app.client;\
             opens app.internal;\
             uses app.Service;\
             provides app.Service with app.ServiceImpl;\
         }",
        "import module java.base; void main() { IO.println(1); } class Helper {}",
    ];

    for source in sources {
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let nodes = semantic.nodes();

        assert_eq!(nodes.parent_id(NodeId::ROOT), NodeId::ROOT);
        assert!(
            nodes.iter_enumerated().skip(1).all(|(id, _)| nodes.parent_id(id).index() < id.index())
        );

        if source.starts_with('@') {
            let module_id = nodes
                .iter_enumerated()
                .find_map(|(id, node)| {
                    matches!(node.kind(), AstKind::ModuleDeclaration(_)).then_some(id)
                })
                .expect("module declaration");
            assert!(nodes.iter().any(|node| matches!(node.kind(), AstKind::RequiresDirective(_))));
            let package_directives: std::vec::Vec<_> = nodes
                .iter_enumerated()
                .filter_map(|(id, node)| {
                    matches!(node.kind(), AstKind::PackageDirective(_)).then_some(id)
                })
                .collect();
            assert_eq!(package_directives.len(), 2);
            assert!(
                package_directives.iter().all(|&id| nodes.parent_id(id) == module_id),
                "exports and opens must collapse to sibling PackageDirective nodes"
            );
            assert!(nodes.iter().any(|node| matches!(node.kind(), AstKind::ProvidesDirective(_))));
        } else {
            assert!(
                nodes.iter().any(|node| matches!(node.kind(), AstKind::CompactCompilationUnit(_)))
            );
        }
    }
}

#[test]
fn semantic_builds_scopes_and_resolves_reads() {
    let allocator = Allocator::new();
    let source = "class A { int value; void f(int input) { int copy = input; use(copy); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    assert!(scoping.scopes().len() >= 4);
    let copy = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.name() == "copy")
        .map(|(symbol_id, _)| symbol_id)
        .expect("copy symbol");
    assert!(scoping.symbol_is_read(copy));
}

#[test]
fn receiver_parameters_declare_no_symbol() {
    let allocator = Allocator::new();
    let source = "class A { void f(A this, int value) {} class B { B(A A.this) {} } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let parameters = semantic
        .scoping()
        .symbols_enumerated()
        .filter(|(_, symbol)| symbol.kind() == SymbolKind::Parameter)
        .map(|(_, symbol)| symbol.name())
        .collect::<std::vec::Vec<_>>();
    assert_eq!(parameters, ["value"]);
}

#[test]
fn qualified_super_qualifiers_name_types() {
    let allocator = Allocator::new();
    let source = "interface I { default void m() {} } class A implements I { \
        void f() { int I = 0; Runnable r = I.super::m; I.super.m(); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    let kinds = scoping
        .references()
        .iter()
        .filter(|reference| reference.name() == "I")
        .map(Reference::kind)
        .collect::<std::vec::Vec<_>>();
    // `implements I`, `I.super::m`, and `I.super.m()`.
    assert_eq!(kinds, [ReferenceKind::Type; 3]);
    let local = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.kind() == SymbolKind::LocalVariable)
        .map(|(symbol_id, _)| symbol_id)
        .expect("local symbol");
    assert!(!scoping.symbol_is_referenced(local));
}

#[test]
fn default_labels_are_nodes_in_source_order() {
    let allocator = Allocator::new();
    let source = "class A { void f(int x) { switch (x) { default -> g(); } \
        switch (x) { case 1: default: h(); } } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let nodes = semantic.nodes();

    let children = nodes
        .iter_enumerated()
        .map(|(id, node)| (node.kind().debug_name(), nodes.parent_kind(id).debug_name()))
        .filter(|(_, parent)| matches!(*parent, "SwitchRule" | "SwitchBlockStatementGroup"))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        children,
        [
            ("DefaultLabel", "SwitchRule"),
            ("MethodCall", "SwitchRule"),
            ("CaseLabel", "SwitchBlockStatementGroup"),
            ("DefaultLabel", "SwitchBlockStatementGroup"),
            ("ExpressionStatement", "SwitchBlockStatementGroup"),
        ]
    );
    let labels = nodes
        .iter()
        .filter_map(|node| match node.kind() {
            AstKind::DefaultLabel(label) => Some(label.span.source_text(source)),
            _ => None,
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(labels, ["default", "default"]);
}

#[test]
fn switch_cases_are_children_of_their_switch_in_source_order() {
    let allocator = Allocator::new();
    let source = "class A { int f(int x) { switch (x) { case 1 -> g(); default -> h(); } \
        switch (x) { case 1: int y; default: } \
        int z = switch (x) { case 1 -> 1; default -> 2; }; \
        return switch (x) { case 1: yield 1; default: yield 2; }; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let nodes = semantic.nodes();

    let cases = nodes
        .iter_enumerated()
        .filter(|(_, node)| {
            matches!(node.kind(), AstKind::SwitchRule(_) | AstKind::SwitchBlockStatementGroup(_))
        })
        .map(|(id, node)| {
            (node.kind().span().source_text(source), nodes.parent_kind(id).debug_name())
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        cases,
        [
            ("case 1 -> g();", "SwitchStatement"),
            ("default -> h();", "SwitchStatement"),
            ("case 1: int y;", "SwitchStatement"),
            ("default:", "SwitchStatement"),
            ("case 1 -> 1;", "SwitchExpression"),
            ("default -> 2;", "SwitchExpression"),
            ("case 1: yield 1;", "SwitchExpression"),
            ("default: yield 2;", "SwitchExpression"),
        ]
    );

    let scoping = semantic.scoping();
    let (_, local) = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.name() == "y")
        .expect("the group's local");
    let owner = scoping.scope(local.scope_id()).owner_node_id();
    assert!(matches!(nodes.kind(owner), AstKind::SwitchStatement(_)));
}

#[test]
fn annotation_arguments_are_children_of_their_annotation_in_source_order() {
    let allocator = Allocator::new();
    let source = "@A(x = 1, y = @B({2, @C(3)})) @D(4) @E() @F class G {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let nodes = semantic.nodes();

    let arguments = nodes
        .iter_enumerated()
        .filter(|(id, node)| {
            matches!(nodes.parent_kind(*id), AstKind::Annotation(_))
                && !matches!(
                    node.kind(),
                    AstKind::IdentifierReference(_) | AstKind::QualifiedTypeName(_)
                )
        })
        .map(|(id, node)| {
            let AstKind::Annotation(annotation) = nodes.parent_kind(id) else { unreachable!() };
            (annotation.name.span().source_text(source), node.kind().span().source_text(source))
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        arguments,
        [("A", "x = 1"), ("A", "y = @B({2, @C(3)})"), ("B", "{2, @C(3)}"), ("C", "3"), ("D", "4"),]
    );
}

#[test]
fn for_expression_lists_are_children_of_their_for_in_source_order() {
    let allocator = Allocator::new();
    let source = "class A { void f(int i, int j) { for (i = 0, j = 1; i < j; i++, j--) g(); \
        for (int k = 0; ; j++) break; for (i = 0; ; ) break; for (;;) break; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let nodes = semantic.nodes();

    let lists = nodes
        .iter_enumerated()
        .filter(|(_, node)| matches!(node.kind(), AstKind::StatementExpressionList(_)))
        .map(|(id, node)| {
            (node.kind().span().source_text(source), nodes.parent_kind(id).debug_name())
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        lists,
        [
            ("i = 0, j = 1", "ForStatement"),
            ("i++, j--", "ForStatement"),
            ("j++", "ForStatement"),
            ("i = 0", "ForStatement"),
        ]
    );

    let expressions = nodes
        .iter_enumerated()
        .filter(|(id, _)| matches!(nodes.parent_kind(*id), AstKind::StatementExpressionList(_)))
        .map(|(_, node)| node.kind().span().source_text(source))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(expressions, ["i = 0", "j = 1", "i++", "j--", "j++", "i = 0"]);

    let (first_for, _) = nodes
        .iter_enumerated()
        .find(|(_, node)| matches!(node.kind(), AstKind::ForStatement(_)))
        .expect("the first for");
    let children = nodes
        .iter_enumerated()
        .filter(|(id, _)| *id != NodeId::ROOT && nodes.parent_id(*id) == first_for)
        .map(|(_, node)| node.kind().debug_name())
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        children,
        [
            "StatementExpressionList",
            "BinaryExpression",
            "StatementExpressionList",
            "ExpressionStatement"
        ]
    );
}

#[test]
fn references_for_symbol_yields_only_that_symbols_references_in_source_order() {
    let allocator = Allocator::new();
    let source = "class A { int value; void f(int value) { use(value); value = value + 1; } void g() { use(value); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    let parameter = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.name() == "value" && symbol.kind() == SymbolKind::Parameter)
        .map(|(symbol_id, _)| symbol_id)
        .expect("parameter symbol");
    let references = scoping.references_for_symbol(parameter).collect::<std::vec::Vec<_>>();

    assert_eq!(references.len(), 3);
    assert_eq!(
        references.iter().map(|reference| reference.kind()).collect::<std::vec::Vec<_>>(),
        [ReferenceKind::Read, ReferenceKind::Write, ReferenceKind::Read]
    );
    assert!(references.iter().all(|reference| reference.symbol_id() == Some(parameter)));
    assert!(references.windows(2).all(|pair| pair[0].span().start < pair[1].span().start));
}

#[test]
fn semantic_records_shadowing_and_write_only_references() {
    let allocator = Allocator::new();
    let source = "class A { int value; void f(int value) { int stale; stale = 1; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    assert!(
        scoping
            .symbols()
            .iter()
            .any(|symbol| symbol.kind() == SymbolKind::Parameter && symbol.shadows().is_some())
    );
    let stale = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.name() == "stale")
        .map(|(symbol_id, _)| symbol_id)
        .expect("stale symbol");
    assert!(!scoping.symbol_is_read(stale));
}

#[test]
fn semantic_enhanced_for_binding_starts_at_the_body() {
    let allocator = Allocator::new();
    let source = "class A { void f() { for (String item : item) { use(item); } } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let resolved = semantic
        .scoping()
        .references()
        .iter()
        .filter(|reference| reference.name() == "item" && reference.symbol_id().is_some())
        .count();
    assert_eq!(resolved, 1);
}

#[test]
fn resolve_breaks_priority_ties_toward_the_earliest_declaration() {
    // Two static imports share the scope, name, and `symbol_priority`. The
    // resolver must pick the earliest-declared candidate (first minimum wins).
    let allocator = Allocator::new();
    let source = "import static p.A.f; import static p.B.f; class C { void g() { f(); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    let imports: std::vec::Vec<SymbolId> = scoping
        .symbols_enumerated()
        .filter(|(_, symbol)| symbol.name() == "f")
        .map(|(symbol_id, _)| symbol_id)
        .collect();
    assert_eq!(imports.len(), 2);
    // `symbols_enumerated` yields symbols in source order, so the first entry is
    // the earliest-declared import.
    let earliest = imports[0];

    let call = scoping
        .references()
        .iter()
        .find(|reference| reference.name() == "f")
        .expect("call reference");
    assert_eq!(call.symbol_id(), Some(earliest));
}

#[test]
fn methods_of_one_name_share_a_binding() {
    for source in [
        "void log(String m) {} void log(int n) {} void f() { log(1); }",
        "class A { void log(String m) {} void log(int n) {} void f() { log(1); } }",
        "interface A { void log(String m); void log(int n); default void f() { log(1); } }",
        "enum A { ; void log(String m) {} void log(int n) {} void f() { log(1); } }",
        "enum A { B { void log(String m) {} void log(int n) {} void f() { log(1); } } }",
        "record A() { void log(String m) {} void log(int n) {} void f() { log(1); } }",
        "class A { Object o = new Object() { \
         void log(String m) {} void log(int n) {} void f() { log(1); } }; }",
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let scoping = semantic.scoping();

        let mut symbols = scoping.symbols_enumerated().filter(|(_, symbol)| symbol.name() == "log");
        let (log, symbol) = symbols.next().expect("method symbol");
        assert!(symbols.next().is_none(), "{source}");
        assert_eq!(symbol.kind(), SymbolKind::Method, "{source}");

        let methods: Vec<_> = semantic
            .nodes()
            .iter_enumerated()
            .filter_map(|(id, node)| match node.kind() {
                AstKind::MethodDeclaration(method) if method.name.name == "log" => {
                    Some((id, method))
                }
                _ => None,
            })
            .collect();
        assert_eq!(methods.len(), 2, "{source}");
        assert_eq!(symbol.declaration_node_id(), methods[0].0, "{source}");
        assert_eq!(symbol.name_span(), methods[0].1.name.span, "{source}");
        assert_eq!(
            scoping.symbol_declarations(log).collect::<Vec<_>>(),
            [methods[0].0, methods[1].0],
            "{source}"
        );
        let redeclarations = scoping.symbol_redeclarations(log);
        for (redeclaration, (id, method)) in redeclarations.iter().zip(&methods) {
            assert_eq!(redeclaration.declaration, *id, "{source}");
            assert_eq!(redeclaration.span, method.name.span, "{source}");
            assert_eq!(method.name.symbol_id(), log, "{source}");
            let declared: Vec<_> = scoping.symbols_for_declaration(*id).map(|(id, _)| id).collect();
            assert_eq!(declared, [log], "{source}");
        }

        let reference = scoping.references_for_symbol(log).next().expect("call reference");
        assert_eq!(reference.kind(), ReferenceKind::Method, "{source}");
        assert_eq!(reference.span().source_text(source), "log", "{source}");
        assert_eq!(scoping.references_for_symbol(log).count(), 1, "{source}");
    }
}

#[test]
fn a_single_declaration_has_no_redeclarations() {
    for source in ["class A { void log() {} }", "@interface A { int log(); }"] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let scoping = semantic.scoping();
        let (log, symbol) =
            scoping.symbols_enumerated().find(|(_, symbol)| symbol.name() == "log").unwrap();
        assert_eq!(symbol.kind(), SymbolKind::Method, "{source}");
        assert!(scoping.symbol_redeclarations(log).is_empty(), "{source}");
        assert_eq!(
            scoping.symbol_declarations(log).collect::<Vec<_>>(),
            [symbol.declaration_node_id()],
            "{source}"
        );
    }
}

#[test]
fn a_nested_class_binds_its_own_methods() {
    let source = "class A { void f() {} void f(int x) { f(); } \
        class B { void f(String s) {} void f(long l) { f(s()); } String s() { return null; } } }";
    let allocator = Allocator::new();
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    let methods: Vec<_> =
        scoping.symbols_enumerated().filter(|(_, symbol)| symbol.name() == "f").collect();
    assert_eq!(methods.len(), 2);
    assert_ne!(methods[0].1.scope_id(), methods[1].1.scope_id());
    for (id, _) in methods {
        assert_eq!(scoping.symbol_declarations(id).count(), 2);
        assert_eq!(scoping.references_for_symbol(id).count(), 1);
    }
    let calls: Vec<_> =
        scoping.references().iter().filter(|reference| reference.name() == "f").collect();
    assert_ne!(calls[0].symbol_id(), calls[1].symbol_id());
}

#[test]
fn methods_stay_apart_from_same_name_variables_and_types() {
    let source = "class A { int log; class log {} void log() {} \
        void log(int n) { log(); log = n; new log(); } }";
    let allocator = Allocator::new();
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    let kinds: Vec<_> = scoping
        .symbols()
        .iter()
        .filter(|symbol| symbol.name() == "log")
        .map(Symbol::kind)
        .collect();
    assert_eq!(kinds, [SymbolKind::Field, SymbolKind::Type, SymbolKind::Method]);
    let references: Vec<_> = scoping
        .references()
        .iter()
        .filter(|reference| reference.name() == "log")
        .map(|reference| {
            (reference.kind(), reference.symbol_id().map(|id| scoping.symbol(id).kind()))
        })
        .collect();
    assert_eq!(
        references,
        [
            (ReferenceKind::Method, Some(SymbolKind::Method)),
            (ReferenceKind::Write, Some(SymbolKind::Field)),
            (ReferenceKind::Type, Some(SymbolKind::Type)),
        ]
    );
}

#[test]
fn methods_of_a_class_take_a_call_from_a_static_import_of_their_name() {
    for (source, expected) in [
        (
            "import static p.Q.log; class A { void log() {} void log(int n) { log(); } }",
            SymbolKind::Method,
        ),
        (
            "import static p.Q.log; class A { void f() { log(); } \
             class B { void log() {} void log(int n) {} } }",
            SymbolKind::Import(ImportKind::SingleStatic),
        ),
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let scoping = semantic.scoping();
        let call = scoping.references().iter().find(|reference| reference.name() == "log").unwrap();
        assert_eq!(
            call.symbol_id().map(|id| scoping.symbol(id).kind()),
            Some(expected),
            "{source}"
        );
    }
}

#[test]
fn a_shared_method_binding_is_private_only_while_every_method_is() {
    for (source, expected) in [
        ("class A { void f() {} }", false),
        ("class A { private void f() {} }", true),
        ("class A { private void f() {} private void f(int x) {} }", true),
        ("class A { private void f() {} void f(int x) {} }", false),
        ("class A { void f() {} private void f(int x) {} }", false),
        ("class A { private void f() {} void f(int x) {} private void f(long x) {} }", false),
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let symbol =
            semantic.scoping().symbols().iter().find(|symbol| symbol.name() == "f").unwrap();
        assert_eq!(symbol.is_private(), expected, "{source}");
    }
}

#[test]
fn a_member_type_binding_is_private_when_declared_private() {
    for (source, expected) in [
        ("class A { class B {} }", false),
        ("class A { private class B {} }", true),
        ("class A { private static interface B {} }", true),
        ("class A { protected enum B {} }", false),
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let symbol =
            semantic.scoping().symbols().iter().find(|symbol| symbol.name() == "B").unwrap();
        assert_eq!(symbol.is_private(), expected, "{source}");
    }
}

#[test]
fn resolve_binds_a_reused_pattern_variable_to_its_latest_declaration() {
    // Java lets one block introduce a pattern-variable name again once the
    // earlier binding is not definitely matched. Each use must bind to the
    // declaration introduced most recently before it.
    for source in [
        "class A { void f(Object a, Object b) { \
         if (a instanceof String s) use(s); if (b instanceof String s) use(s); } }",
        "class A { void f(Object a, Object b) { \
         if (a instanceof String s && s.isEmpty()) return; \
         if (b instanceof String s) use(s.length()); } }",
        "class A { Object f(Object a, Object b) { \
         return a instanceof String s ? s : b instanceof String s ? s : \"\"; } }",
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let scoping = semantic.scoping();

        let patterns: std::vec::Vec<(SymbolId, u32)> = scoping
            .symbols_enumerated()
            .filter(|(_, symbol)| symbol.kind() == SymbolKind::PatternVariable)
            .map(|(symbol_id, symbol)| (symbol_id, symbol.name_span().start))
            .collect();
        assert_eq!(patterns.len(), 2, "{source}");

        let uses: std::vec::Vec<_> =
            scoping.references().iter().filter(|reference| reference.name() == "s").collect();
        assert_eq!(uses.len(), 2, "{source}");
        for reference in uses {
            let expected = patterns
                .iter()
                .rev()
                .find(|&&(_, start)| start < reference.span().start)
                .map(|&(symbol_id, _)| symbol_id);
            assert_eq!(reference.symbol_id(), expected, "{source}");
        }
    }
}

#[test]
fn resolve_pattern_variables_only_where_they_are_definitely_matched() {
    // Each read of `s` names what it resolves to: the field, the local, or the
    // nth pattern variable `s` in source order.
    for (body, expected) in [
        ("if (!(o instanceof String s)) return; use(s);", &["pattern 0"][..]),
        (
            "if (o instanceof String s) use(s); if (o instanceof Integer s) use(s);",
            &["pattern 0", "pattern 1"],
        ),
        ("if (o instanceof String s) {} else { use(s); return; } use(s);", &["field", "pattern 0"]),
        ("for (; !(o instanceof String s); ) {} use(s);", &["pattern 0"]),
        ("for (; !(o instanceof String s); ) { if (c) break; } use(s);", &["field"]),
        (
            "for (; o instanceof String s; use(s)) use(s); use(s);",
            &["pattern 0", "pattern 0", "field"],
        ),
        ("while (!(o instanceof String s)) {} use(s);", &["pattern 0"]),
        ("while (!(o instanceof String s)) { if (c) break; } use(s);", &["field"]),
        ("while (o instanceof String s) use(s); use(s);", &["pattern 0", "field"]),
        ("do {} while (!(o instanceof String s)); use(s);", &["pattern 0"]),
        ("l: while (!(o instanceof String s)) { break l; } use(s);", &["field"]),
        ("l: while (!(o instanceof String s)) { for (;;) break; } use(s);", &["pattern 0"]),
        (
            "if (!(o instanceof String s) || s.isEmpty()) return; use(s);",
            &["pattern 0", "pattern 0"],
        ),
        (
            "if (o instanceof String s && s.isEmpty()) use(s); else use(s);",
            &["pattern 0", "pattern 0", "field"],
        ),
        ("use(o instanceof String s ? s : s);", &["pattern 0", "field"]),
        ("use(!(o instanceof String s) ? s : s);", &["field", "pattern 0"]),
        ("if (!(o instanceof String s)) { use(s); } use(s);", &["field", "field"]),
        (
            "if (!(o instanceof String s)) { throw new Error(); } else { use(s); } use(s);",
            &["pattern 0", "pattern 0"],
        ),
        ("if (!(o instanceof String s)) { while (true) {} } use(s);", &["pattern 0"]),
        ("if (!(o instanceof String s)) { while (true) { break; } } use(s);", &["field"]),
        (r"if (!(o instanceof String s)) { while (\u0074rue) {} } use(s);", &["pattern 0"]),
        (r"if (!(o instanceof String s)) { while (\u0066alse) {} } use(s);", &["field"]),
        (
            "final boolean t = true; if (!(o instanceof String s)) { while (t) {} } use(s);",
            &["pattern 0"],
        ),
        ("if (!(o instanceof String s)) { do {} while (false); } use(s);", &["field"]),
        ("if (!(o instanceof String s)) { while (c) {} } use(s);", &["field"]),
        ("if (!(o instanceof String s)) { while (flag) {} } use(s);", &["field"]),
        ("if (!(o instanceof String s)) { while (ON) {} } use(s);", &["pattern 0"]),
        ("if (!(o instanceof String s)) { do {} while (!CHAIN); } use(s);", &["field"]),
        ("final boolean t = c; if (!(o instanceof String s)) { while (t) {} } use(s);", &["field"]),
        (
            "final Boolean t = true; if (!(o instanceof String s)) { while (t) {} } use(s);",
            &["field"],
        ),
        (
            "final var t = true; if (!(o instanceof String s)) { for (; t; ) {} } use(s);",
            &["pattern 0"],
        ),
        (
            "interface K { boolean c = true; } new K() { void g(Object o) { \
             if (!(o instanceof String s)) { while (c) {} } use(s); } };",
            &["pattern 0"],
        ),
        (
            "class L { void g(Object o) { \
             if (!(o instanceof String s)) { while (c) {} } use(s); } }",
            &["field"],
        ),
        (
            "if (!(p instanceof Boolean ON)) return; if (!(o instanceof String s)) { while (ON) {} } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { if (!(p instanceof Boolean ON)) return; while (ON) {} } use(s);",
            &["field"],
        ),
        (
            "if (!(p instanceof Boolean ON)) { while (c) {} } if (!(o instanceof String s)) { while (ON) {} } use(s);",
            &["pattern 0"],
        ),
        (
            "if (!(o instanceof String s)) { if (!(p instanceof Boolean ON)) { while (c) {} } while (ON) {} } use(s);",
            &["pattern 0"],
        ),
        (
            "if (o instanceof String s) { while (c) use(s); } else { return; } use(s);",
            &["pattern 0", "pattern 0"],
        ),
        ("if (!(o instanceof String s)) { try { return; } finally {} } use(s);", &["pattern 0"]),
        ("if (!(o instanceof String s)) { do continue; while (o != null); } use(s);", &["field"]),
        (
            "if (!(o instanceof String s)) { try { return; } catch (Error e) {} } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { while (true) { try { break; } finally { return; } } } use(s);",
            &["pattern 0"],
        ),
        (
            "if (!(o instanceof String s)) { l: do continue l; while (o != null); } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { l: do continue l; while (true); } use(s);",
            &["pattern 0"],
        ),
        (
            "if (!(o instanceof String s)) { l: while (true) { switch (n) { case 1 -> { break l; } } } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { l: while (true) { switch (n) { case 1: break l; } } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { while (true) { switch (n) { case 1 -> { break; } } } } use(s);",
            &["pattern 0"],
        ),
        (
            "if (!(o instanceof String s)) { while (true) { switch (n) { case 1: break; } } } use(s);",
            &["pattern 0"],
        ),
        (
            "if (!(o instanceof String s)) { switch (c ? 1 : 2) { case 1 -> { return; } } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { switch (c ? 1 : 2) { case ONE -> { return; } } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { switch (c ? 1 : 2) { case 1: return; default: throw new Error(); } } use(s);",
            &["pattern 0"],
        ),
        (
            "if (!(o instanceof String s)) { switch (c ? 1 : 2) { case 1: return; default: break; } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { switch (p) { case String t -> { return; } case Object t -> { return; } } } use(s);",
            &["pattern 0"],
        ),
        ("if (!(o instanceof String s)) { switch (c ? 1 : 2) {} } use(s);", &["field"]),
        (
            "if (!(o instanceof String s)) { switch (c ? 1 : 2) { case 1: return; } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { switch (c ? 1 : 2) { case 1: return; default: run(); } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { switch (p) { case String t -> run(); case Object t -> { return; } } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { switch (p) { case String t -> { run(); } case Object t -> { return; } } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { switch (p) { case String t -> { break; } case Object t -> { return; } } } use(s);",
            &["field"],
        ),
        (
            "if (!(o instanceof String s)) { switch (p) { case String t -> throw new Error(); case Object t -> { return; } } } use(s);",
            &["pattern 0"],
        ),
        ("if (!(o instanceof Box(String s))) return; use(s);", &["pattern 0"]),
        ("if (o instanceof String s) use(s); String s = \"\"; use(s);", &["pattern 0", "local"]),
        ("run(() -> o instanceof String s && s.isEmpty()); use(s);", &["pattern 0", "field"]),
        (
            "switch (o) { case String s when s.isEmpty() -> use(s); default -> use(s); }",
            &["pattern 0", "pattern 0", "field"],
        ),
        (
            "switch (o) { case String t when p instanceof Integer s -> use(s); default -> use(s); }",
            &["pattern 0", "field"],
        ),
        (
            "switch (o) { case String t: if (!(p instanceof Integer s)) break; use(s); default: use(s); }",
            &["pattern 0", "field"],
        ),
        (
            "switch (s.length()) { case 1: String s; break; default: s = \"\"; use(s); } use(s);",
            &["field", "local", "local", "field"],
        ),
        (
            "use(switch (c ? 1 : 2) { case 1: String s = \"\"; yield s; default: s = \"\"; yield s; });",
            &["local", "local", "local"],
        ),
    ] {
        let source = format!(
            "class A {{ String s; void f(Object o, Object p, boolean c) {{ {body} }} \
             boolean flag; static final boolean ON = true, CHAIN = ON; static final int ONE = 1; }}"
        );
        let allocator = Allocator::new();
        let result = parse(&allocator, &source);
        assert!(result.errors.is_empty(), "{body}: {:?}", result.errors);
        let semantic = SemanticBuilder::new().build(&source, &result.unit);
        let scoping = semantic.scoping();

        let patterns: std::vec::Vec<SymbolId> = scoping
            .symbols_enumerated()
            .filter(|(_, symbol)| symbol.kind() == SymbolKind::PatternVariable)
            .filter(|(_, symbol)| symbol.name() == "s")
            .map(|(symbol_id, _)| symbol_id)
            .collect();
        let resolved: std::vec::Vec<String> = scoping
            .references()
            .iter()
            .filter(|reference| reference.name() == "s")
            .map(|reference| match reference.symbol_id() {
                Some(id) if scoping.symbol(id).kind() == SymbolKind::Field => "field".to_string(),
                Some(id) if scoping.symbol(id).kind() == SymbolKind::LocalVariable => {
                    "local".to_string()
                }
                Some(id) => {
                    let index = patterns.iter().position(|&pattern| pattern == id).unwrap();
                    format!("pattern {index}")
                }
                None => "none".to_string(),
            })
            .collect();
        assert_eq!(resolved, expected, "{body}");
    }
}

#[test]
fn constant_chains_end_in_code_that_does_not_compile() {
    let allocator = Allocator::new();
    let source = "class A { static final boolean X = Y, Y = X, Z = Z; void f(Object o) { \
         if (!(o instanceof String s)) { while (X || Z) {} } use(s); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();
    let read = scoping.references().iter().rfind(|reference| reference.name() == "s").unwrap();
    assert_eq!(
        read.symbol_id().map(|id| scoping.symbol(id).kind()),
        Some(SymbolKind::PatternVariable)
    );
}

#[test]
fn a_pattern_variable_after_a_basic_for_binds_in_the_enclosing_block() {
    let allocator = Allocator::new();
    let source = "class A { void f(Object o) { for (; !(o instanceof String s); ) {} use(s); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    let (pattern_id, pattern) = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.kind() == SymbolKind::PatternVariable)
        .expect("pattern symbol");
    let nodes = semantic.nodes();
    let (body_id, body) = nodes
        .iter_enumerated()
        .find(|(_, node)| matches!(node.kind(), AstKind::Block(_)))
        .expect("method body");
    let for_statement = nodes
        .iter()
        .find(|node| matches!(node.kind(), AstKind::ForStatement(_)))
        .expect("for statement");
    assert_eq!(scoping.scope(pattern.scope_id()).owner_node_id(), body_id);
    assert_eq!(
        scoping.symbol_parts(pattern_id),
        &[Span::new(for_statement.kind().span().end, body.kind().span().end)]
    );
}

#[test]
fn resolve_local_types_only_from_their_declaration() {
    for (members, expected) in [
        ("void f() { L a = new L(); class L {} L b; }", &["import", "import", "type 0"][..]),
        ("void f() { class L { L next; } }", &["type 0"]),
        ("void f() { interface L { L get(); } }", &["type 0"]),
        ("void f() { enum L { X; L self() { return X; } } }", &["type 0"]),
        ("void f() { record L(L next) {} }", &["type 0"]),
        ("void f() { { class L {} } L a; }", &["import"]),
        (
            "void f() { { class L {} L a; } { L b; class L {} L c; } }",
            &["type 0", "import", "type 1"],
        ),
        (
            "void f(int n) { switch (n) { case 1: L a; class L {} L b; default: L c; } }",
            &["import", "type 0", "import"],
        ),
        ("void f() { L a; } class L {}", &["type 0"]),
        ("class L {} void f() { L a; class L {} L b; }", &["type 0", "type 1"]),
    ] {
        assert_eq!(resolve_l(&format!("class A {{ {members} }}")), expected, "{members}");
    }
}

#[test]
fn resolve_members_only_in_their_body() {
    for (declaration, expected) in [
        ("class A implements L { class L {} L a; }", &["import", "type 0"][..]),
        ("class A<T extends L> extends L { interface L {} }", &["import", "import"]),
        ("@L class A { @interface L {} }", &["import"]),
        ("interface A extends L { interface L {} L a(); }", &["import", "type 0"]),
        (
            "enum A implements L { X { L a; }; interface L {} L b; }",
            &["import", "type 0", "type 0"],
        ),
        (
            "record A(@L L a) implements L { interface L {} L b; }",
            &["type 0", "type 0", "import", "type 0"],
        ),
        ("@L @interface A { @interface L {} L a(); }", &["import", "type 0"]),
        ("class A { class L {} class B { L a; } }", &["type 0"]),
        ("class A { Object a = new Object() { class L {} L b; }; L c; }", &["type 0", "import"]),
    ] {
        assert_eq!(resolve_l(declaration), expected, "{declaration}");
    }
}

/// What each use of `L` in `declaration`, after `import p.L;`, resolves to:
/// the import, or the nth type `L` in source order.
fn resolve_l(declaration: &str) -> std::vec::Vec<String> {
    let source = format!("import p.L; {declaration}");
    let allocator = Allocator::new();
    let result = parse(&allocator, &source);
    assert!(result.errors.is_empty(), "{declaration}: {:?}", result.errors);
    let semantic = SemanticBuilder::new().build(&source, &result.unit);
    let scoping = semantic.scoping();

    let types: std::vec::Vec<SymbolId> = scoping
        .symbols_enumerated()
        .filter(|(_, symbol)| symbol.kind() == SymbolKind::Type && symbol.name() == "L")
        .map(|(symbol_id, _)| symbol_id)
        .collect();
    scoping
        .references()
        .iter()
        .filter(|reference| reference.name() == "L")
        .map(|reference| match reference.symbol_id() {
            Some(id) if matches!(scoping.symbol(id).kind(), SymbolKind::Import(_)) => {
                "import".to_string()
            }
            Some(id) => {
                let index = types.iter().position(|&ty| ty == id).unwrap();
                format!("type {index}")
            }
            None => "none".to_string(),
        })
        .collect()
}

#[test]
fn record_components_bind_in_the_body_scope() {
    let allocator = Allocator::new();
    let source = "record A<T>(T value) implements B<T> { T get() { return value; } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();

    let (body_id, body) = nodes
        .iter_enumerated()
        .find_map(|(id, node)| match node.kind() {
            AstKind::ClassBody(body) => Some((id, body)),
            _ => None,
        })
        .expect("record body");
    let component = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.kind() == SymbolKind::RecordComponent)
        .map(|(symbol_id, symbol)| (symbol_id, symbol.scope_id()))
        .expect("record component");
    assert_eq!(scoping.scope(component.1).owner_node_id(), body_id);
    assert!(scoping.scope(component.1).is_type());
    assert_eq!(body.scope_id(), component.1, "the header and the body share one scope");
    let read = scoping.references().iter().find(|reference| reference.name() == "value");
    assert_eq!(read.and_then(Reference::symbol_id), Some(component.0));

    let parameter = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.kind() == SymbolKind::TypeParameter)
        .map(|(symbol_id, _)| symbol_id)
        .expect("type parameter");
    let uses = scoping.references().iter().filter(|reference| reference.name() == "T");
    assert!(uses.map(Reference::symbol_id).eq([Some(parameter); 3]));
}

/// The owners of the scopes a reference to `name` looks up through, innermost first.
fn scope_owners_of(source: &str, name: &str) -> Vec<&'static str> {
    let allocator = Allocator::new();
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();
    let reference = scoping
        .references()
        .iter()
        .find(|reference| reference.name() == name)
        .unwrap_or_else(|| panic!("{source}: no reference to {name}"));
    std::iter::successors(Some(reference.scope_id()), |&id| scoping.scope(id).parent_id())
        .map(|id| semantic.nodes().kind(scoping.scope(id).owner_node_id()).debug_name())
        .collect()
}

#[test]
fn scope_markers_open_each_scope_at_its_boundary() {
    let unit = "CompilationUnit";
    for (source, name, owners) in [
        // A type's own scope starts at its type parameters (JLS 6.3).
        ("@A(x) class C<T> {}", "x", vec![unit]),
        ("class C<T extends x> {}", "x", vec!["ClassDeclaration", unit]),
        ("class C<T> extends x {}", "x", vec!["ClassDeclaration", unit]),
        ("class C<T> implements x {}", "x", vec!["ClassDeclaration", unit]),
        ("class C { x y; }", "x", vec!["ClassBody", "ClassDeclaration", unit]),
        ("@A(x) interface I<T> {}", "x", vec![unit]),
        ("interface I<T> extends x {}", "x", vec!["InterfaceDeclaration", unit]),
        ("interface I { x y = 1; }", "x", vec!["InterfaceBody", "InterfaceDeclaration", unit]),
        ("@A(x) enum E {}", "x", vec![unit]),
        ("enum E implements x {}", "x", vec![unit]),
        ("enum E { A(x); }", "x", vec!["EnumBody", unit]),
        ("@A(x) @interface I {}", "x", vec![unit]),
        ("@interface I { x y(); }", "x", vec!["AnnotationInterfaceBody", unit]),
        // A record's header is in its body's scope, and its `implements` clause is not.
        ("@A(x) record R<T>() {}", "x", vec![unit]),
        ("record R<T extends x>() {}", "x", vec!["RecordDeclaration", unit]),
        ("record R(x y) {}", "x", vec!["ClassBody", "RecordDeclaration", unit]),
        ("record R() implements x {}", "x", vec!["RecordDeclaration", unit]),
        (
            "record R() { x y() {} }",
            "x",
            vec!["MethodDeclaration", "ClassBody", "RecordDeclaration", unit],
        ),
        // A method's scope is its entire declaration.
        (
            "class C { @A(x) <T> void m() {} }",
            "x",
            vec!["MethodDeclaration", "ClassBody", "ClassDeclaration", unit],
        ),
        (
            "class C { @A(x) C() {} }",
            "x",
            vec!["ConstructorDeclaration", "ClassBody", "ClassDeclaration", unit],
        ),
        ("class C { { x(); } }", "x", vec!["Block", "ClassBody", "ClassDeclaration", unit]),
        // A compact unit's implicit class holds its members, not its imports.
        ("int y = x; void m() {}", "x", vec!["CompactCompilationUnit", unit]),
        // A switch's scope starts at its block.
        (
            "void m() { switch (x) { default: y(); } }",
            "x",
            vec!["Block", "MethodDeclaration", "CompactCompilationUnit", unit],
        ),
        (
            "void m() { switch (x) { default: y(); } }",
            "y",
            vec!["SwitchStatement", "Block", "MethodDeclaration", "CompactCompilationUnit", unit],
        ),
        (
            "int z = switch (x) { default -> y; }; void m() {}",
            "x",
            vec!["CompactCompilationUnit", unit],
        ),
        (
            "int z = switch (x) { default -> y; }; void m() {}",
            "y",
            vec!["SwitchExpression", "CompactCompilationUnit", unit],
        ),
        // A resource's scope ends with the `try` block.
        (
            "void m() { try (var r = x) {} }",
            "x",
            vec!["TryStatement", "Block", "MethodDeclaration", "CompactCompilationUnit", unit],
        ),
        (
            "void m() { try { x(); } catch (E e) {} }",
            "x",
            vec![
                "Block",
                "TryStatement",
                "Block",
                "MethodDeclaration",
                "CompactCompilationUnit",
                unit,
            ],
        ),
        (
            "void m() { try {} catch (E e) { x(); } }",
            "x",
            vec![
                "Block",
                "CatchClause",
                "Block",
                "MethodDeclaration",
                "CompactCompilationUnit",
                unit,
            ],
        ),
        (
            "void m() { try {} finally { x(); } }",
            "x",
            vec!["Block", "Block", "MethodDeclaration", "CompactCompilationUnit", unit],
        ),
        (
            "void m() { for (int i = x; ; ) {} }",
            "x",
            vec!["ForStatement", "Block", "MethodDeclaration", "CompactCompilationUnit", unit],
        ),
        (
            "void m() { for (var i : x) {} }",
            "x",
            vec![
                "EnhancedForStatement",
                "Block",
                "MethodDeclaration",
                "CompactCompilationUnit",
                unit,
            ],
        ),
        (
            "Runnable f = () -> x; void m() {}",
            "x",
            vec!["LambdaExpression", "CompactCompilationUnit", unit],
        ),
    ] {
        assert_eq!(scope_owners_of(source, name), owners, "{source}: {name}");
    }
}

#[test]
fn a_local_after_the_try_block_does_not_shadow_its_resource() {
    for source in [
        "class A { void m() { try (var r = a()) {} catch (E e) { int r = 0; } } }",
        "class A { void m() { try (var r = a()) {} finally { int r = 0; } } }",
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let local = semantic
            .scoping()
            .symbols()
            .iter()
            .find(|symbol| symbol.kind() == SymbolKind::LocalVariable)
            .expect("a local");
        assert_eq!(local.shadows(), None, "{source}");
    }
}

#[test]
fn a_compact_units_imports_stay_in_the_file_scope() {
    let allocator = Allocator::new();
    let source = "import java.util.List; void main() {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let nodes = semantic.nodes();

    let scope_of = |matches: fn(AstKind<'_>) -> bool| {
        nodes.iter().find(|node| matches(node.kind())).expect("a node").scope_id()
    };
    assert_eq!(scope_of(|kind| matches!(kind, AstKind::ImportDeclaration(_))), ScopeId::ROOT);
    assert_eq!(scope_of(|kind| matches!(kind, AstKind::IdentifierName(_))), ScopeId::ROOT);
    let method = scope_of(|kind| matches!(kind, AstKind::MethodDeclaration(_)));
    assert_ne!(method, ScopeId::ROOT);
    assert!(semantic.scoping().scope(method).is_type());
}

#[test]
fn analysis_records_the_nodes_the_generated_walk_visits() {
    #[derive(Default)]
    struct Walked(Vec<&'static str>);

    impl Visit<'_> for Walked {
        fn enter_node(&mut self, kind: AstKind<'_>) {
            self.0.push(kind.debug_name());
        }
    }

    let allocator = Allocator::new();
    let source = r"
        import a.B;
        @A class C<@A T extends D> extends E implements F permits G {
            @A <U> int[] m(C this, @A int p[], @A int @A ... q)[] throws H {
                @A int a[] = { 1 };
                for (@A int i[] : a) {}
                try (@A var r = s) {} catch (@A J | K e) {}
                if (o instanceof @A L l) {}
                Runnable f = (@A int x[], @A int @A ... y) -> {};
            }
        }
        @A interface I<T> extends J permits K {}
        @A enum N implements O { @A P(1) {} }
        @A record R<T>(@A int c, @A int @A ... d) implements S { @A R {} }
        @A @interface V { @A int[] w()[] default {}; }
    ";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);

    let mut walked = Walked::default();
    walked.visit_compilation_unit(&result.unit);
    let recorded = semantic.nodes().iter().map(|node| node.kind().debug_name()).collect::<Vec<_>>();
    assert_eq!(recorded, walked.0);
}

#[test]
fn every_node_that_opens_a_scope_holds_it_after_analysis() {
    #[derive(Default)]
    struct ScopeCells(Vec<ScopeId>);

    impl Visit<'_> for ScopeCells {
        fn enter_scope(&mut self, scope_id: &Cell<Option<ScopeId>>) {
            self.0.push(scope_id.get().expect("analysis sets every scope cell"));
        }
    }

    let allocator = Allocator::new();
    let source = r"
        class C<T> {
            { }
            C() {}
            int m(int p) {
                for (;;) {}
                for (var i : p) {}
                try (var r = q) {} catch (E e) {}
                switch (p) { default: }
                Runnable f = () -> {};
                return switch (p) { default -> 1; };
            }
        }
        interface I<T> {}
        enum E {}
        record R<T>(T c) { R {} }
        @interface A {}
    ";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);

    let mut cells = ScopeCells::default();
    cells.visit_compilation_unit(&result.unit);
    assert_eq!(result.unit.scope_id(), ScopeId::ROOT);
    let mut opened = cells.0;
    opened.sort_unstable();
    let scopes = semantic.scoping().scopes().len();
    let expected = (0..scopes).map(ScopeId::from_usize).collect::<Vec<_>>();
    assert_eq!(opened, expected, "the walk enters each scope once");
}

#[test]
fn compact_constructor_parameters_shadow_record_components() {
    let allocator = Allocator::new();
    let source = r"
        record A<T>(T value, String... names) {
            A {
                use(value, names);
                Runnable f = () -> use(value, names);
                Runnable g = new Runnable() { public void run() { use(value, names); } };
            }
            void method() { use(value, names); }
            static void use(Object... args) {}
        }
    ";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();

    for name in ["value", "names"] {
        let (component_id, component) = scoping
            .symbols_enumerated()
            .find(|(_, symbol)| {
                symbol.name() == name && symbol.kind() == SymbolKind::RecordComponent
            })
            .expect("record component");
        let (parameter_id, parameter) = scoping
            .symbols_enumerated()
            .find(|(_, symbol)| symbol.name() == name && symbol.kind() == SymbolKind::Parameter)
            .expect("implicit parameter");
        assert_eq!(parameter.name_span(), component.name_span());
        assert_eq!(parameter.shadows(), Some(component_id));
        let AstKind::RecordComponent(declaration) = nodes.kind(component.declaration_node_id())
        else {
            panic!("the component symbol belongs to the record component");
        };
        assert_eq!(declaration.name.symbol_id.get(), Some(component_id));
        let AstKind::CompactConstructorDeclaration(constructor) =
            nodes.kind(parameter.declaration_node_id())
        else {
            panic!("implicit parameter belongs to the compact constructor");
        };
        assert_eq!(scoping.symbol_parts(parameter_id), &[constructor.body.span]);
        assert_eq!(
            scoping.scope(parameter.scope_id()).owner_node_id(),
            parameter.declaration_node_id()
        );
        let uses = scoping.references().iter().filter(|reference| reference.name() == name);
        assert!(uses.map(Reference::symbol_id).eq([
            Some(parameter_id),
            Some(parameter_id),
            Some(parameter_id),
            Some(component_id),
        ]));
    }
}

#[test]
fn resolve_prefers_the_nearest_enclosing_scope() {
    // A local named `x` shadows the field `x`; the read must bind to the local.
    let allocator = Allocator::new();
    let source = "class A { int x; void f() { int x = 0; use(x); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    let local = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.name() == "x" && symbol.kind() == SymbolKind::LocalVariable)
        .map(|(symbol_id, _)| symbol_id)
        .expect("local variable symbol");
    let read = scoping
        .references()
        .iter()
        .find(|reference| reference.name() == "x")
        .expect("read reference");
    assert_eq!(read.symbol_id(), Some(local));
}

#[test]
fn resolve_shadowing_links_to_the_ancestor_declaration() {
    // The parameter `value` shadows the field `value`; the shadow edge must
    // point at the field.
    let allocator = Allocator::new();
    let source = "class A { int value; void f(int value) { use(value); } }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    let field = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.name() == "value" && symbol.kind() == SymbolKind::Field)
        .map(|(symbol_id, _)| symbol_id)
        .expect("field symbol");
    let parameter = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.name() == "value" && symbol.kind() == SymbolKind::Parameter)
        .map(|(_, symbol)| symbol)
        .expect("parameter symbol");
    assert_eq!(parameter.shadows(), Some(field));
}

#[test]
fn semantic_type_parameters_shadow_imported_types() {
    let allocator = Allocator::new();
    let source = "package p; import example.T; class A<T> { T value; }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let scoping = semantic.scoping();

    let parameter = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| symbol.kind() == SymbolKind::TypeParameter)
        .map(|(symbol_id, _)| symbol_id)
        .expect("type parameter symbol");
    assert!(scoping.symbol_is_read(parameter));
    let import = scoping
        .symbols_enumerated()
        .find(|(_, symbol)| matches!(symbol.kind(), SymbolKind::Import(_)))
        .map(|(symbol_id, _)| symbol_id)
        .expect("import symbol");
    assert!(!scoping.symbol_is_referenced(import));
}

#[test]
fn the_root_accessor_hands_back_the_units_own_declarations() {
    let allocator = Allocator::new();
    let source = "package p; import java.util.List; class A {} class B {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);
    let nodes = semantic.nodes();

    let unit = nodes.compilation_unit();
    assert!(std::ptr::eq(unit, &raw const result.unit));
    assert!(unit.package().is_some_and(|package| package.name.dotted() == "p"));
    assert_eq!(unit.imports().len(), 1);
    assert_eq!(unit.types().len(), 2);
}

#[test]
fn the_unusual_unit_shapes_still_carry_their_imports_on_the_root() {
    // A modular unit is `{ImportDeclaration} ModuleDeclaration` and a compact
    // unit is `{ImportDeclaration}` plus implicit class members (JLS §7.3), so
    // neither body owns the imports — the compilation unit does.
    for source in [
        "import java.util.List; module m { requires java.base; }",
        "import java.util.List; void main() {}",
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let nodes = semantic.nodes();

        let unit = nodes.compilation_unit();
        assert!(unit.package().is_none());
        assert_eq!(unit.imports().len(), 1);
    }
}

#[test]
fn named_type_declarations_expose_one_common_ast_view() {
    let allocator = Allocator::new();
    let source = "class C {} interface I {} enum E {} record R() {} @interface A {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let semantic = SemanticBuilder::new().build(source, &result.unit);

    let declarations: Vec<_> =
        semantic.nodes().iter().filter_map(|node| node.kind().as_type_declaration_kind()).collect();
    assert_eq!(
        declarations.iter().map(|declaration| declaration.name().name).collect::<Vec<_>>(),
        ["C", "I", "E", "R", "A"]
    );
    assert!(declarations.into_iter().all(|declaration| declaration.modifiers().is_empty()));
}

#[test]
fn flattened_nodes_follow_source_order() {
    // The flattened list is a preorder walk, so a node never starts before the
    // node visited ahead of it. That holds only while every AST field order is
    // the source order, or states its traversal position explicitly. The two
    // families below are the ones whose fields cannot be in source order:
    // `ArrayType` carries the annotations that follow its element type, and
    // `ConstructorInvocation` carries the type arguments that follow its
    // qualifier.
    for source in [
        "class C { char @Nullable [] grid; String @A [] @B [] table; }",
        "@interface A { int value() @A []; }",
        "class C { C(Outer outer) { outer.<String>super(1); } }",
        "class C { C() { <String>this(1); } C(int n) {} }",
        "class C { void run() { new Outer().new Inner<String>(1) {}; } }",
        "class C<T extends @A Object> { <U> U pick(java.util.List<? extends @A U> in) { \
             return in.get(0).<U>cast(); } }",
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert!(result.errors.is_empty(), "{source}: {:?}", result.errors);
        let semantic = SemanticBuilder::new().build(source, &result.unit);
        let nodes = semantic.nodes();

        let mut previous = None::<(AstKind<'_>, Span)>;
        for node in nodes.iter() {
            let span = node.kind().span();
            if let Some((previous_kind, previous_span)) = previous {
                assert!(
                    span.start >= previous_span.start,
                    "{source}: {:?} at {}..{} is visited after {:?} at {}..{}",
                    node.kind(),
                    span.start,
                    span.end,
                    previous_kind,
                    previous_span.start,
                    previous_span.end
                );
            }
            previous = Some((node.kind(), span));
        }
    }
}

#[test]
fn javadocs_are_the_documentation_comments_that_document_a_declaration() {
    let mentioned = |source: &str| {
        let allocator = Allocator::new();
        let parsed = parse(&allocator, source);
        assert!(parsed.errors.is_empty(), "{source}: {:?}", parsed.errors);
        let semantic = SemanticBuilder::new().build(source, &parsed.unit);
        ["List", "Map"]
            .map(|name| semantic.javadocs().structures().any(|structure| structure.mentions(name)))
    };

    assert_eq!(mentioned("/** First List. */\nclass A {}\n/// Second Map.\nclass B {}"), [true; 2]);
    assert_eq!(mentioned("class A { int x; /// [List]\n/// [Map]\nint y; }"), [true; 2]);
    assert_eq!(mentioned("/** List */\n/** Map */\nclass A {}"), [false, true]);
    assert_eq!(mentioned("class A { int f() { /** List */ return 0; } /// Map\n}"), [false; 2]);
    assert_eq!(mentioned("/** List */\npackage p;\nclass A {}"), [false; 2]);
    assert_eq!(mentioned("/** List */\npackage p;\n"), [true, false]);
}

#[test]
fn javadoc_structure_is_parsed_once() {
    let source = "/** First List. */\nclass A {}";
    let allocator = Allocator::new();
    let parsed = parse(&allocator, source);
    let semantic = SemanticBuilder::new().build(source, &parsed.unit);

    let first = semantic.javadocs().structures().next().expect("Javadoc structure");
    let again = semantic.javadocs().structures().next().expect("Javadoc structure");
    assert!(std::ptr::eq(first, again));
}
