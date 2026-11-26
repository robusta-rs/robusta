use robusta_syntax::node::NodeId;

use super::*;

#[test]
fn parser_distinguishes_clean_recovered_and_fatal_results() {
    let allocator = Allocator::new();
    let clean = parse(&allocator, "class Clean {}");
    assert_eq!(clean.status, ParseStatus::Clean);

    let allocator = Allocator::new();
    let recovered =
        parse(&allocator, "class A { int ; void f() { int ; call(); int ; } long ; } class B {}");
    assert_eq!(recovered.status, ParseStatus::Recovered);
    assert_eq!(recovered.errors.len(), 4, "{:?}", recovered.errors);

    let allocator = Allocator::new();
    let fatal = parse(&allocator, "class A { void f() { call(");
    assert_eq!(fatal.status, ParseStatus::Fatal);
    assert!(fatal.unit.types().is_empty(), "a fatal result carries an empty unit");
    assert_ne!(fatal.errors.len(), 0);
}

#[test]
fn recovered_diagnostics_are_ordered_and_lexer_errors_suppress_parser_cascades() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "class A { int ; }\nclass # {}\nclass B { long ; }");

    assert_eq!(result.status, ParseStatus::Recovered);
    assert_eq!(result.errors.len(), 3, "{:?}", result.errors);
    assert!(
        result
            .errors
            .windows(2)
            .all(|errors| errors[0].primary_span().start <= errors[1].primary_span().start)
    );
    assert_eq!(
        result
            .errors
            .iter()
            .filter(|error| error
                .primary_span()
                .source_text("class A { int ; }\nclass # {}\nclass B { long ; }")
                == "#")
            .count(),
        1
    );
}

#[test]
fn separated_list_recovery_keeps_valid_siblings() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "interface A extends Good, int, Better {}");
    assert_eq!(result.status, ParseStatus::Recovered);
    assert_eq!(result.errors.len(), 1, "{:?}", result.errors);
    let unit = result.unit;
    let TypeDeclaration::Interface(interface) = &unit.types()[0] else {
        panic!("expected an interface");
    };
    assert_eq!(interface.extends.len(), 2);
}

#[test]
fn missing_member_semicolon_recovers_at_the_closing_brace() {
    let allocator = Allocator::new();
    let result = parse(&allocator, "class A { int x }");
    assert_ne!(result.errors.len(), 0);
    assert_eq!(result.status, ParseStatus::Recovered);
    assert_eq!(result.errors.len(), 1);
    assert!(result.errors[0].message.contains("`;`"));
}

#[test]
fn malformed_truncated_inputs_terminate_at_eof() {
    for source in [
        "class A { A() { bar(",
        "class A { void f() { new Foo(",
        "class A { void f() { @A(",
        "class A { void f() { Runnable r = (x",
        "class A { void f() { List<",
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert_eq!(result.status, ParseStatus::Fatal, "{source} parsed unexpectedly");
        assert!(result.unit.types().is_empty(), "{source} retained a partial unit");
        assert!(!result.errors.is_empty(), "{source} produced no error");
    }
}

#[test]
fn fatal_units_are_empty_ordinary_forms_keeping_comments_in_order() {
    for (source, comments) in [
        (
            "/* keep this */ class Fatal { void f() { call( // trailing",
            ["/* keep this */", "// trailing"],
        ),
        // The lexer reports the unterminated comment and ends the source.
        ("/* keep this */ class Fatal { /* unterminated", ["/* keep this */", "/* unterminated"]),
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert_eq!(result.status, ParseStatus::Fatal, "{source}");
        assert!(!result.errors.is_empty(), "{source}");
        let unit = result.unit;
        let texts = unit.comments.iter().map(|comment| comment.span.source_text(source));
        assert!(texts.eq(comments), "{source}: {:?}", unit.comments);
        assert_eq!(unit.span, Span::empty(0), "{source}");
        let CompilationUnitContent::Ordinary(form) = &unit.content else {
            panic!("{source}: expected an ordinary form");
        };
        assert_eq!(form.span, Span::empty(0), "{source}");
        assert!(form.package.is_none(), "{source}");
        assert!(form.imports.is_empty() && form.types.is_empty(), "{source}");
    }
}

#[test]
fn compact_source_stray_closing_brace_is_reported_and_skipped() {
    for source in [
        "}",
        "x }",
        "public // c\n}",
        "void main() {}\n}",
        "void main() {\nIO.println(1);\n}}",
        "import a.B;\n}",
    ] {
        let allocator = Allocator::new();
        let result = parse(&allocator, source);
        assert_ne!(result.status, ParseStatus::Clean, "{source} parsed cleanly");
        let end = u32::try_from(source.len()).unwrap();
        let stray = Span::new(end - 1, end);
        assert!(
            result.errors.iter().any(|error| error.primary_span() == stray),
            "{source}: {:?}",
            result.errors
        );
    }

    let allocator = Allocator::new();
    let result = parse(&allocator, "void main() {}\n}\nvoid next() {}");
    assert_eq!(result.status, ParseStatus::Recovered);
    assert_eq!(result.errors.len(), 1, "{:?}", result.errors);
    let compact = result.unit.compact().expect("expected a compact unit");
    assert_eq!(compact.members.len(), 2);
}

// Skip on 32-bit systems: a string longer than `isize::MAX` cannot exist.
#[cfg(target_pointer_width = "64")]
#[test]
fn overlong_source_yields_a_fatal_empty_unit() {
    // `vec![0; n]` allocates zeroed pages without writing 4 GiB, and the
    // length guard rejects the source before any byte is read.
    let bytes = vec![0u8; robusta_lexer::MAX_LEN + 1];
    // SAFETY: a slice consisting entirely of zeros is valid UTF-8.
    let source = unsafe { std::str::from_utf8_unchecked(&bytes) };
    let allocator = Allocator::new();
    let result = parse(&allocator, source);
    assert_eq!(result.status, ParseStatus::Fatal);
    assert!(result.unit.types().is_empty());
    assert!(result.unit.comments.is_empty());
    assert_eq!(result.unit.node_id(), NodeId::ROOT);
    assert_ne!(result.unit.content.node_id(), NodeId::ROOT);
    // The placeholder's lists stay empty without taking arena space.
    assert_eq!(allocator.used_bytes(), 0);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].message, "source length exceeds 4 GiB limit");
}
