use robusta_ast::AstKind;
use robusta_ast_visit::Visit;

use super::*;

/// Every node ID in visitor order, starting with the compilation unit's.
fn node_ids<'a>(unit: &'a CompilationUnit<'a>) -> std::vec::Vec<NodeId> {
    #[derive(Default)]
    struct Ids(std::vec::Vec<NodeId>);

    impl<'a> Visit<'a> for Ids {
        fn enter_node(&mut self, kind: AstKind<'a>) {
            self.0.push(kind.node_id());
        }
    }

    let mut ids = Ids::default();
    ids.visit_compilation_unit(unit);
    ids.0
}

#[test]
fn parsing_alone_gives_every_node_a_unique_id_and_the_root_zero() {
    for source in [
        "package a; import java.util.List; @Deprecated class C<T> { int f = (1); void m() { g(); } }",
        "import java.util.List; int f = 1; void main() { var x = new int[] {1}; }",
        "import java.util.List; @Deprecated open module m { requires transitive java.base; }",
        "class C { void m() { x = (a) -> a; y = (b) + 1; List<String> z; } }",
        // The compact parse rebuilds `A`, whose nodes the failed ordinary parse discards.
        "class A { int x; } void main() {}",
        "class { int x; }",
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        let mut ids = node_ids(&result.unit);
        assert_eq!(ids[0], NodeId::ROOT, "{source}");
        assert!(ids.len() > 1, "{source}");
        ids.sort_unstable();
        let count = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), count, "{source}: an ID is shared");
    }
}

#[test]
fn every_unit_form_has_its_root_id_and_its_spans() {
    for (source, is_form, content_span) in [
        (
            "package a; class C {}",
            (|content| matches!(content, CompilationUnitContent::Ordinary(_)))
                as fn(&CompilationUnitContent<'_>) -> bool,
            Span::new(0, 21),
        ),
        (
            "import a.B; void main() {}",
            |content| matches!(content, CompilationUnitContent::Compact(_)),
            Span::new(0, 26),
        ),
        (
            "import a.B; module m {}",
            |content| matches!(content, CompilationUnitContent::Modular(_)),
            Span::new(0, 23),
        ),
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert_eq!(result.status, ParseStatus::Clean, "{source}: {:?}", result.errors);
        let unit = result.unit;
        assert!(is_form(&unit.content), "{source}: {:?}", unit.content);
        assert_eq!(unit.node_id(), NodeId::ROOT, "{source}");
        assert_ne!(unit.content.node_id(), NodeId::ROOT, "{source}");
        assert_eq!(unit.span, Span::new(0, u32::try_from(source.len()).unwrap()), "{source}");
        assert_eq!(unit.content.span(), content_span, "{source}");
    }
}

#[test]
fn a_fatal_empty_unit_has_ids_too() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "import ;\n}");
    assert_eq!(result.status, ParseStatus::Fatal, "{:?}", result.errors);
    assert_eq!(node_ids(&result.unit), [NodeId::ROOT, NodeId::new(1)]);
    assert_eq!(result.unit.content.node_id(), NodeId::new(1));
    assert_eq!(result.unit.span, Span::new(0, 0));
    assert_eq!(result.unit.content.span(), Span::empty(0));
    assert!(matches!(result.unit.content, CompilationUnitContent::Ordinary(_)));
}

#[test]
fn parses_package_imports_and_class_with_fields() {
    let allocator = Allocator::new();
    let source = "package a.b;\nimport java.util.List;\nimport static x.Y.z;\nimport a.*;\n\
                      public final class Foo { private int x = 5; String s = \"hi\", t; }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;

    let package = unit.package().unwrap();
    assert_eq!(name_parts(&package.name), ["a", "b"]);

    assert_eq!(unit.imports().len(), 3);
    assert_eq!(unit.imports()[0].kind, ImportKind::SingleType);
    assert_eq!(unit.imports()[1].kind, ImportKind::SingleStatic);
    assert_eq!(unit.imports()[2].kind, ImportKind::TypeOnDemand);

    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(class.name.name, "Foo");
    assert_eq!(class.modifiers.len(), 2);
    assert!(class.type_params.is_none());
    assert!(class.superclass.is_none());
    assert!(class.interfaces.is_empty());
    assert_eq!(class.body.members.len(), 2);
    let ClassMember::Field(field) = &class.body.members[1] else {
        panic!("expected a field");
    };
    assert_eq!(field.declarators.len(), 2);
    assert!(field.declarators[0].init.is_some());
    assert!(field.declarators[1].init.is_none());
}

#[test]
fn parses_module_import_declarations() {
    let allocator = Allocator::new();
    let source = "import module java.base;\n\
                  import module com.example.feature;\n\
                  import module.foo.Bar;\n\
                  import static module.foo.Bar.baz;";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;

    assert_eq!(unit.imports().len(), 4);
    assert_eq!(unit.imports()[0].kind, ImportKind::SingleModule);
    assert_eq!(name_parts(&unit.imports()[0].name), ["java", "base"]);
    assert_eq!(unit.imports()[1].kind, ImportKind::SingleModule);

    assert_eq!(unit.imports()[2].kind, ImportKind::SingleType);
    assert_eq!(name_parts(&unit.imports()[2].name), ["module", "foo", "Bar"]);
    assert_eq!(unit.imports()[3].kind, ImportKind::SingleStatic);
}

#[test]
fn parses_compact_source_files() {
    let allocator = Allocator::new();
    let source = "import module java.base;\
                  final int answer = 42;\
                  void main() { IO.println(answer); }\
                  class Helper {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;

    assert!(unit.package().is_none());
    assert!(unit.module().is_none());
    assert!(unit.types().is_empty());
    assert_eq!(unit.imports().len(), 1);

    let compact = unit.compact().expect("expected a compact unit");
    assert_eq!(compact.members.len(), 3);
    assert!(matches!(&compact.members[0], ClassMember::Field(_)));
    let ClassMember::Method(main) = &compact.members[1] else {
        panic!("expected the compact source method");
    };
    assert_eq!(main.name.name, "main");
    assert!(main.body.is_some());
    let ClassMember::Class(helper) = &compact.members[2] else {
        panic!("expected a compact source helper class");
    };
    assert_eq!(helper.name.name, "Helper");
}

#[test]
fn compact_source_files_can_start_with_helper_types() {
    let allocator = Allocator::new();
    let source = "class Helper {} void main() {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;

    assert!(unit.types().is_empty());
    let compact = unit.compact().expect("expected a compact unit");
    assert_eq!(compact.members.len(), 2);
    assert!(matches!(&compact.members[0], ClassMember::Class(_)));
    assert!(matches!(&compact.members[1], ClassMember::Method(_)));
}

#[test]
fn compact_source_files_can_use_module_words_as_field_declarations() {
    let allocator = Allocator::new();
    let source = "@A module value; open module; void main() {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;

    assert!(unit.module().is_none());
    assert!(unit.types().is_empty());
    let compact = unit.compact().expect("expected a compact unit");
    assert_eq!(compact.members.len(), 3);

    let ClassMember::Field(module_field) = &compact.members[0] else {
        panic!("expected the `module value` field");
    };
    assert_eq!(module_field.modifiers.len(), 1);
    let Some(module_ty) = module_field.ty.as_class_type() else {
        panic!("expected a class `module` type");
    };
    assert_eq!(class_type_names(module_ty), ["module"]);
    assert_eq!(module_field.declarators[0].name.name, "value");

    let ClassMember::Field(open_field) = &compact.members[1] else {
        panic!("expected the `open module` field");
    };
    let Some(open_ty) = open_field.ty.as_class_type() else {
        panic!("expected a class `open` type");
    };
    assert_eq!(class_type_names(open_ty), ["open"]);
    assert_eq!(open_field.declarators[0].name.name, "module");
}

#[test]
fn ordinary_unit_diagnostic_wins_when_compact_makes_no_more_progress() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "class A {} @");

    assert_ne!(result.errors.len(), 0);
    assert_eq!(result.errors.len(), 1);
    assert!(result.errors[0].message.contains("expected a class, interface, enum, record"));
}

#[test]
fn compact_unit_diagnostic_wins_after_helper_type_when_it_gets_farther() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "class Helper {} void main() {} Script() {}");

    assert_ne!(result.errors.len(), 0);
    assert_eq!(result.errors.len(), 1);
    assert!(
        result.errors[0].message.contains("constructors are not valid in compact source files")
    );
}

#[test]
fn parses_annotated_package_declaration() {
    let allocator = Allocator::new();
    // JLS §7.4.1: a `package-info.java` package declaration may carry
    // annotations; the leading run belongs to the package, not a type.
    let source = "@CheckReturnValue\n@NullMarked\npackage com.google.common.net;";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;

    let package = unit.package().unwrap();
    assert_eq!(package.annotations.len(), 2);
    assert_eq!(type_name_parts(&package.annotations[0].name), ["CheckReturnValue"]);
    assert_eq!(type_name_parts(&package.annotations[1].name), ["NullMarked"]);
    assert_eq!(name_parts(&package.name), ["com", "google", "common", "net"]);
    assert!(unit.types().is_empty());
}

#[test]
fn leading_annotations_without_package_belong_to_the_type() {
    let allocator = Allocator::new();
    // The same leading-annotation run, but a type follows rather than
    // `package`: the annotations must flow into the type's modifiers.
    let source = "@Deprecated\npublic class Foo {}";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    assert!(unit.package().is_none());
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(class.modifiers.len(), 2);
}

#[test]
fn parses_module_declaration_directives() {
    let allocator = Allocator::new();
    let source = "import java.sql.Driver;\n\
                      @Deprecated open module com.example.app {\
                          requires transitive java.compiler;\
                          requires static java.sql;\
                          requires transitive;\
                          exports com.example.api to other.module, another.module;\
                          opens com.example.internal;\
                          uses com.example.Service;\
                          provides com.example.Service with com.example.impl.ServiceImpl, com.example.impl.OtherService;\
                      }";
    let result = parse(&allocator, source);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    assert!(unit.package().is_none());
    assert_eq!(unit.imports().len(), 1);
    assert!(unit.types().is_empty());

    let module = unit.module().unwrap();
    assert!(module.is_open);
    assert_eq!(module.annotations.len(), 1);
    assert_eq!(name_parts(&module.name), ["com", "example", "app"]);
    assert_eq!(module.directives.len(), 7);

    let ModuleDirective::Requires(requires) = &module.directives[0] else {
        panic!("expected a requires directive");
    };
    assert_eq!(requires.modifiers.len(), 1);
    assert_eq!(requires.modifiers[0].kind, RequiresModifierKind::Transitive);
    assert_eq!(name_parts(&requires.name), ["java", "compiler"]);

    let ModuleDirective::Requires(static_requires) = &module.directives[1] else {
        panic!("expected a requires directive");
    };
    assert_eq!(static_requires.modifiers.len(), 1);
    assert_eq!(static_requires.modifiers[0].kind, RequiresModifierKind::Static);

    let ModuleDirective::Requires(transitive_module) = &module.directives[2] else {
        panic!("expected a requires directive");
    };
    assert!(transitive_module.modifiers.is_empty());
    assert_eq!(name_parts(&transitive_module.name), ["transitive"]);

    let ModuleDirective::Package(exports) = &module.directives[3] else {
        panic!("expected an exports directive");
    };
    assert_eq!(exports.kind, PackageDirectiveKind::Exports);
    assert_eq!(exports.targets.len(), 2);

    let ModuleDirective::Package(opens) = &module.directives[4] else {
        panic!("expected an opens directive");
    };
    assert_eq!(opens.kind, PackageDirectiveKind::Opens);
    assert!(opens.targets.is_empty());

    assert!(matches!(&module.directives[5], ModuleDirective::Uses(_)));
    let ModuleDirective::Provides(provides) = &module.directives[6] else {
        panic!("expected a provides directive");
    };
    assert_eq!(provides.with.len(), 2);
}

#[test]
fn module_declaration_is_the_only_top_level_declaration() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "module example.app {} class Extra {}");
    assert_ne!(result.errors.len(), 0);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn module_words_remain_identifiers_outside_module_declarations() {
    let allocator = Allocator::new();
    let result =
        parse(&allocator, "class module { int requires; void open() { int exports = 1; } }");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let unit = result.unit;
    assert!(unit.module().is_none());
    let TypeDeclaration::Class(class) = &unit.types()[0] else {
        panic!("expected a class");
    };
    assert_eq!(class.name.name, "module");
}

#[test]
fn comments_are_forwarded() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "// header\nclass A {} /* tail */");
    assert_eq!(result.unit.comments.len(), 2);
}
