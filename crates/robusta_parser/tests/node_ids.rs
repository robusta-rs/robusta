use robusta_allocator::Allocator;
use robusta_ast::{AstKind, CompilationUnit, CompilationUnitContent};
use robusta_ast_visit::Visit;
use robusta_parser::{ParseStatus, parse};
use robusta_span::{GetSpan, Span};
use robusta_syntax::node::NodeId;

/// Every node ID in visitor order, starting with the compilation unit's.
fn node_ids<'a>(unit: &'a CompilationUnit<'a>) -> Vec<NodeId> {
    #[derive(Default)]
    struct Ids(Vec<NodeId>);

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
