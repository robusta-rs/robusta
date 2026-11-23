use robusta_allocator::Allocator;

use robusta_allocator::Vec as ArenaVec;

use crate::{
    BestFittingElement, Document, Format, FormatElement, FormatState, Formatted, Formatter,
    LineMode, SimpleFormatContext, Tag,
    builders::{
        best_fitting_variant, group, hard_line_break, indent, soft_line_break_or_space, space, text,
    },
    format_args,
};

type Context = SimpleFormatContext<'static>;

/// Writes one fixed element, standing in for the builder layer.
struct Raw(FormatElement<'static>);

impl<'ast> Format<'ast, Context> for Raw {
    fn fmt(&self, f: &mut Formatter<'_, 'ast, Context>) {
        f.write_element(self.0.clone());
    }
}

fn token(text: &'static str) -> Raw {
    Raw(FormatElement::token(text))
}

fn state(allocator: &Allocator) -> FormatState<'_, Context> {
    FormatState::new(Context::default(), allocator)
}

#[test]
fn simple_context_carries_options_and_source() {
    use crate::FormatContext;

    let context = SimpleFormatContext::default().with_source_code("class A {}");
    assert_eq!(context.source_code(), "class A {}");
}

#[test]
fn write_streams_arguments_in_order() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut f = Formatter::new(&mut state);

    crate::write!(f, [token("class"), Raw(FormatElement::Space), token("A")]);

    let elements = f.into_vec();
    assert_eq!(
        &*elements,
        [FormatElement::token("class"), FormatElement::Space, FormatElement::token("A"),]
    );
}

#[test]
fn interning_collapses_by_element_count() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut formatter = Formatter::new(&mut state);

    assert!(formatter.intern(&()).is_none());
    assert_eq!(formatter.intern(&token("a")), Some(FormatElement::token("a")));
    assert!(matches!(
        formatter.intern(&crate::format_args!(token("a"), token("b"))),
        Some(FormatElement::Interned(_))
    ));
}

#[test]
fn nested_content_leaves_the_parents_elements_in_place() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut f = Formatter::new(&mut state);

    crate::write!(f, [token("before")]);
    let nested = f.format_to_vec(|f| crate::write!(f, [token("inner")]));
    crate::write!(f, [token("after")]);

    assert_eq!(&*nested, [FormatElement::token("inner")]);
    assert_eq!(&*f.into_vec(), [FormatElement::token("before"), FormatElement::token("after")]);
}

#[test]
fn remove_soft_lines_rewrites_soft_breaks_inside_interned_content() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut formatter = Formatter::new(&mut state);

    let interned = formatter
        .intern(&crate::format_args!(token("a"), Raw(FormatElement::Line(LineMode::Soft))))
        .unwrap();
    formatter.remove_soft_lines(|f| {
        f.write_element(interned);
        f.write_element(FormatElement::Line(LineMode::SoftOrSpace));
        f.write_element(FormatElement::Line(LineMode::Hard));
    });

    let elements = formatter.into_vec();
    let FormatElement::Interned(cleaned) = &elements[0] else { panic!("{elements:?}") };
    assert_eq!(&**cleaned, [FormatElement::token("a")]);
    assert_eq!(&elements[1..], [FormatElement::Space, FormatElement::Line(LineMode::Hard)]);
}

#[test]
fn best_fitting_prints_the_first_variant_that_fits() {
    // The default print width is 100 columns.
    for (width, expected) in [(10, None), (120, Some("middle"))] {
        let allocator = &Allocator::new();
        let mut state = state(allocator);
        let mut f = Formatter::new(&mut state);
        let flat = allocator.alloc_str(&"x".repeat(width));

        let most_flat = best_fitting_variant(&mut f, |f| text(flat).fmt(f));
        let middle = best_fitting_variant(&mut f, |f| token("middle").fmt(f));
        let most_expanded = best_fitting_variant(&mut f, |f| token("expanded").fmt(f));
        let variants = ArenaVec::from_iter_in([most_flat, middle, most_expanded], &allocator);
        // SAFETY: There are three variants.
        f.write_element(FormatElement::BestFitting(unsafe {
            BestFittingElement::from_vec_unchecked(variants)
        }));

        let document = Document::new(f.into_vec());
        let printed = Formatted::new(document, Context::default()).print().unwrap();
        assert_eq!(printed.as_code(), expected.unwrap_or(flat));
    }
}

#[test]
fn group_and_indent_write_paired_tags_around_content() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut f = Formatter::new(&mut state);

    crate::write!(f, [group(&indent(&token("body")))]);

    let elements = f.into_vec();
    assert!(matches!(elements[0], FormatElement::Tag(Tag::StartGroup(_))));
    assert!(matches!(elements[1], FormatElement::Tag(Tag::StartIndent)));
    assert_eq!(elements[2], FormatElement::token("body"));
    assert!(matches!(elements[3], FormatElement::Tag(Tag::EndIndent)));
    assert!(matches!(elements[4], FormatElement::Tag(Tag::EndGroup)));
    assert_eq!(elements.len(), 5);
}

#[test]
fn text_builder_measures_display_width() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut f = Formatter::new(&mut state);

    crate::write!(f, [text("café")]);

    let elements = f.into_vec();
    let FormatElement::Text { text, width } = &elements[0] else {
        panic!("expected a text element, got {:?}", elements[0]);
    };
    assert_eq!(text.as_str(), "café");
    assert!(!width.is_multiline());
    assert_eq!(width.value(), 4);
}

#[test]
fn join_with_writes_the_separator_between_entries() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut formatter = Formatter::new(&mut state);

    formatter.join_with(space()).entry(&token("a")).entry(&token("b")).entry(&token("c"));

    assert_eq!(
        &*formatter.into_vec(),
        [
            FormatElement::token("a"),
            FormatElement::Space,
            FormatElement::token("b"),
            FormatElement::Space,
            FormatElement::token("c"),
        ]
    );
}

#[test]
fn fill_wraps_each_entry_and_separator_in_entry_tags() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut formatter = Formatter::new(&mut state);

    formatter
        .fill()
        .entries(&Raw(FormatElement::Line(LineMode::SoftOrSpace)), [token("a"), token("b")])
        .finish();

    let elements = formatter.into_vec();
    assert!(matches!(elements.first(), Some(FormatElement::Tag(Tag::StartFill))));
    assert!(matches!(elements.last(), Some(FormatElement::Tag(Tag::EndFill))));
    let entry_starts =
        elements.iter().filter(|e| matches!(e, FormatElement::Tag(Tag::StartEntry))).count();
    assert_eq!(entry_starts, 3, "two entries plus the separator each get an entry pair");
}

#[test]
fn format_args_nest_as_a_single_argument() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut f = Formatter::new(&mut state);

    crate::write!(f, [format_args!(token("a"), space()), token("b")]);

    assert_eq!(
        &*f.into_vec(),
        [FormatElement::token("a"), FormatElement::Space, FormatElement::token("b"),]
    );
}

#[test]
fn formatted_prints_the_document_through_the_context_options() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut f = Formatter::new(&mut state);

    crate::write!(
        f,
        [group(&format_args!(
            token("class"),
            space(),
            token("A"),
            space(),
            token("{"),
            token("}")
        ))]
    );

    let document = Document::new(f.into_vec());
    let printed = Formatted::new(document, Context::default()).print().unwrap();
    assert_eq!(printed.as_code(), "class A {}");
}

/// `Formatted::print` must finalize the document (propagate group expansion)
/// before printing, not just print the raw IR: a hard line inside a group
/// forces that group's own soft line to break too, even though the printer
/// never performs this propagation itself.
#[test]
fn formatted_print_expands_the_enclosing_group_before_printing() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut f = Formatter::new(&mut state);

    crate::write!(
        f,
        [group(&format_args!(
            token("a"),
            soft_line_break_or_space(),
            token("b"),
            hard_line_break(),
            token("c"),
        ))]
    );

    let document = Document::new(f.into_vec());
    let printed = Formatted::new(document, Context::default()).print().unwrap();
    assert_eq!(printed.as_code(), "a\nb\nc");
}

#[test]
fn replace_end_swaps_the_written_tail() {
    let allocator = &Allocator::new();
    let mut state = state(allocator);
    let mut f = Formatter::new(&mut state);

    crate::write!(f, [token("keep"), token("drop"), token("drop too")]);
    f.replace_end(1, &[FormatElement::token("tail")]);

    assert_eq!(&*f.into_vec(), [FormatElement::token("keep"), FormatElement::token("tail")]);
}
