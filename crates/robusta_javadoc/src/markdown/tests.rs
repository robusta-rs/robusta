use markdown::mdast::Node;

use super::{parse, span};

#[test]
fn reparsed_paragraphs_keep_their_source_positions() {
    let content = "- a\n\n`x`\t`y`\n    z\n- b\n\n`w`\n    v";
    let Some(Node::Root(root)) = parse(content) else { panic!("{content:?} parses") };
    assert_eq!(
        root.children.iter().filter_map(span).map(|range| &content[range]).collect::<Vec<_>>(),
        ["- a\n", "`x`\t`y`\n    z", "- b\n", "`w`\n    v"]
    );
    let paragraph = &root.children[1];
    let points = std::iter::once(paragraph)
        .chain(paragraph.children().into_iter().flatten())
        .map(|node| node.position().map(|position| (position.start.offset, position.start.column)))
        .collect::<Vec<_>>();

    assert_eq!(points, [Some((5, 1)), Some((5, 1)), Some((8, 4)), Some((9, 5)), Some((12, 8))]);
}

#[test]
fn html_block_in_a_new_container_keeps_its_source_position() {
    let content = "a\n> b\n>\n> c\n> > <d>\n> > *e*\n\nf";
    let Some(Node::Root(root)) = parse(content) else { panic!("{content:?} parses") };
    let Node::Blockquote(quote) = &root.children[1] else { panic!("a quote follows") };
    let Node::Blockquote(inner) = &quote.children[2] else { panic!("a quote follows c") };
    let html = &inner.children[0];
    assert!(matches!(html, Node::Html(_)));
    let Some(position) = html.position() else { panic!("HTML has a position") };
    assert_eq!(span(html).map(|range| &content[range]), Some("<d>\n> > *e*"));
    assert_eq!((position.start.line, position.start.column), (5, 5));
    assert_eq!((position.end.line, position.end.column), (6, 8));
    let Some(last) = root.children[2].position() else { panic!("f has a position") };
    assert_eq!((last.start.line, last.start.column), (8, 1));
}

#[test]
fn only_a_list_that_can_interrupt_a_paragraph_after_a_closed_item_does() {
    fn blocks(content: &str) -> Vec<&'static str> {
        let Some(Node::Root(root)) = parse(content) else { panic!("{content:?} parses") };
        root.children
            .iter()
            .map(|node| match node {
                Node::List(_) => "list",
                Node::Paragraph(_) => "paragraph",
                Node::Heading(_) => "heading",
                _ => "other",
            })
            .collect()
    }

    assert_eq!(blocks("- m\n\na\n10. b\n11. c"), ["list", "paragraph"]);
    assert_eq!(blocks("1. m\n\na\n10) b"), ["list", "paragraph"]);
    assert_eq!(blocks("1. m\n\na\n2.\n    b"), ["list", "paragraph"]);
    assert_eq!(blocks("- m\n\na\n2. </b>"), ["list", "paragraph"]);
    assert_eq!(blocks("- m\n\na\n-\nb"), ["list", "heading", "paragraph"]);
    assert_eq!(blocks("- m\n\na\n1. b"), ["list", "paragraph", "list"]);
}

fn shape(node: &Node) -> String {
    let name = match node {
        Node::Root(_) => "root",
        Node::Blockquote(_) => "quote",
        Node::List(list) if list.spread => "loose list",
        Node::List(_) => "list",
        Node::ListItem(item) if item.spread => "loose item",
        Node::ListItem(_) => "item",
        Node::Paragraph(_) => return "paragraph".to_string(),
        _ => return "other".to_string(),
    };
    let children = node.children().into_iter().flatten().map(shape).collect::<Vec<_>>();
    format!("{name}[{}]", children.join(", "))
}

/// The shape of `content`, which must parse without any of a repair's text in the tree.
fn read(content: &str) -> String {
    let root = parse(content).unwrap_or_else(|| panic!("{content:?} parses"));
    assert!(!format!("{root:?}").contains("***"), "{content:?} keeps a repair: {root:?}");
    shape(&root)
}

#[test]
fn a_list_at_a_zero_padded_one_interrupts_a_paragraph() {
    assert_eq!(read("a\n01. b"), "root[paragraph, list[item[paragraph]]]");
    assert_eq!(
        read("a\n001) b\n002) c"),
        "root[paragraph, list[item[paragraph], item[paragraph]]]"
    );
    assert_eq!(read("- a\n  01. b"), "root[list[item[paragraph, list[item[paragraph]]]]]");
    assert_eq!(read("- a\n     01. b"), "root[list[item[paragraph, list[item[paragraph]]]]]");
    assert_eq!(read("> a\n>\t01. b"), "root[quote[paragraph, list[item[paragraph]]]]");
    assert_eq!(read("a\n00. b"), "root[paragraph]");
    assert_eq!(read("a\n01."), "root[paragraph]");
    assert_eq!(read("- a\n      01. b"), "root[list[item[paragraph]]]");
    assert_eq!(read(">001) a\n>     01. b"), "root[quote[list[item[paragraph]]]]");
    assert_eq!(read("a\n> <b>\n> 01. c"), "root[paragraph, quote[other]]");
    assert_eq!(
        read("-\ta\n   01. b\n     01. c"),
        "root[list[item[paragraph]], list[item[paragraph]]]"
    );
    assert_eq!(read("> 2. [a]: b\na\n     01. c"), "root[quote[list[item[other, paragraph]]]]");
    // The guard before the paragraph comes first, so the list it starts takes in the item.
    assert_eq!(read("[a]: b\n2. c\n01. d"), "root[other, list[item[paragraph], item[paragraph]]]");
    // A guard taken for the paragraph before the break would loosen the `2) -` item.
    assert_eq!(
        read("- *\na\n2) b\n01. c\n2) -\n> 1. <b>"),
        "root[list[item[list[item[]]]], paragraph, list[item[paragraph]], list[item[list[item[]]]], quote[list[item[other]]]]"
    );
}

#[test]
fn a_lazy_line_continues_the_paragraph_its_containers_hold() {
    let read = |content: &str| shape(&parse(content).unwrap_or_else(|| panic!("{content:?}")));

    assert_eq!(read("-   $i\n  <a href=\"x\">"), "root[list[item[paragraph]]]");
    assert_eq!(read("-   {@link A} z\n  `"), "root[list[item[paragraph]]]");
    assert_eq!(read("- a\n<x"), "root[list[item[paragraph]]]");
    assert_eq!(read("-   [c]: d\n   <a href=\"x\">"), "root[list[item[other, paragraph]]]");
    assert_eq!(
        read("-   each c\n   <b>\n- > b c"),
        "root[list[item[paragraph], item[quote[paragraph]]]]"
    );
    assert_eq!(read("> - a\n>  <x y=\"z\">"), "root[quote[list[item[paragraph]]]]");
    assert_eq!(read(">- a\n<b>"), "root[quote[list[item[paragraph]]]]");
    assert_eq!(read("- > a\n   ~"), "root[list[item[quote[paragraph]]]]");
    assert_eq!(read(">\t- a\n<b>\n<c>"), "root[quote[list[item[paragraph]]]]");
    // The paragraph starts with its definition, on the line that holds every container.
    assert_eq!(read("> - [a]: b\n><b> t\n_"), "root[quote[list[item[other, paragraph]]]]");
    // HTML that can interrupt a paragraph, and a line after a blank one, close the item.
    assert_eq!(read("- a\n<div>"), "root[list[item[paragraph]], other]");
    assert_eq!(read("- a\n  b\n\n<b>"), "root[list[item[paragraph]], other]");
    // A paragraph that javadoc reads as indented code takes no lazy line.
    assert_eq!(read("`\n>     e\tq\n`"), "root[paragraph, quote[other], paragraph]");

    // Inserted quote markers move no position, even before a tab.
    for (content, text) in [("> > a\n<b> `c`", "a\n<b> `c`"), ("> >a\n>\t<b>", "a\n>\t<b>")] {
        let Some(Node::Root(root)) = parse(content) else { panic!("{content:?} parses") };
        let mut paragraph = &root.children[0];
        while !matches!(paragraph, Node::Paragraph(_)) {
            paragraph = &paragraph.children().unwrap()[0];
        }
        let last = paragraph.children().and_then(|children| children.last()).unwrap();
        assert_eq!(span(paragraph).map(|range| &content[range]), Some(text));
        assert_eq!(
            last.position().map(|position| (position.start.line, position.start.column)),
            Some((2, 5)),
            "{content:?}"
        );
    }
}

#[test]
fn a_line_after_a_setext_heading_leaves_the_containers_it_lacks() {
    assert_eq!(read("- a\n  ==\nb"), "root[list[item[other]], paragraph]");
    assert_eq!(read("- a\n  ---\nb"), "root[list[item[other]], paragraph]");
    assert_eq!(read("> a\n> ==\nb"), "root[quote[other], paragraph]");
    assert_eq!(read("> > a\n> > ==\n> b"), "root[quote[quote[other], paragraph]]");
    assert_eq!(read("- > a\n  > ==\n  b"), "root[list[item[quote[other], paragraph]]]");
    assert_eq!(read("- > a\n  > ==\n b"), "root[list[item[quote[other]]], paragraph]");
    assert_eq!(read("> a\n> ---\n[c]: d"), "root[quote[other], other]");
    // The list stays tight, and the next item starts a list of its own.
    assert_eq!(
        read("- a\n  ==\nb\n- c"),
        "root[list[item[other]], paragraph, list[item[paragraph]]]"
    );
    // A line that keeps its containers stays in them.
    assert_eq!(read("- a\n  ==\n  b"), "root[list[item[other, paragraph]]]");
    assert_eq!(read("> a\n> ==\n> b\nc"), "root[quote[other, paragraph]]");

    let content = "> a\n> ==\nb";
    let Some(Node::Root(root)) = parse(content) else { panic!("{content:?} parses") };
    let position = root.children[1].position().unwrap();
    assert_eq!((position.start.offset, position.start.line, position.start.column), (9, 3, 1));
}

#[test]
fn a_last_line_of_underline_marks_that_lacks_its_containers_is_paragraph_text() {
    assert_eq!(read("> a\n--"), "root[quote[paragraph]]");
    assert_eq!(read("- a\n--"), "root[list[item[paragraph]]]");
    assert_eq!(read("> - a\n> --"), "root[quote[list[item[paragraph]]]]");
    assert_eq!(read("> 2. a\n>--"), "root[quote[list[item[paragraph]]]]");
    assert_eq!(read("- > a\n  -- "), "root[list[item[quote[paragraph]]]]");
    assert_eq!(read("> a\n--\n--"), "root[quote[paragraph]]");
    // A line that keeps its containers still underlines the paragraph.
    assert_eq!(read("> a\n> --"), "root[quote[other]]");
    assert_eq!(read("- a\n  --"), "root[list[item[other]]]");

    // The paragraph ends where the content does.
    let content = "> a\n--";
    let Some(Node::Root(root)) = parse(content) else { panic!("{content:?} parses") };
    let paragraph = &root.children[0].children().unwrap()[0];
    assert_eq!(span(paragraph).map(|range| &content[range]), Some("a\n--"));
    let end = &paragraph.position().unwrap().end;
    assert_eq!((end.offset, end.line, end.column), (6, 2, 3));
}

#[test]
fn a_dash_line_after_a_setext_heading_is_a_thematic_break() {
    // The dash line reads as a thematic break, an `other`, and the list stays tight.
    assert_eq!(read("a\n==\n---"), "root[other, other]");
    assert_eq!(read("a\n---\n---\n---"), "root[other, other, other]");
    assert_eq!(read("a\n==\n   ----  \nb"), "root[other, other, paragraph]");
    assert_eq!(read("a\n==\n---\n==="), "root[other, other, paragraph]");
    assert_eq!(read("- a\n  ==\n  ---\n- b"), "root[list[item[other, other], item[paragraph]]]");
    assert_eq!(read("> a\n>  ==\n  e q\n==\n---"), "root[quote[other], other, other]");
    // Too few dashes, or indented code, is not a break.
    assert_eq!(read("a\n==\n--"), "root[other, paragraph]");
    assert_eq!(read("a\n==\n    ---"), "root[other, other]");

    // The break spans its line, indentation included, as after a blank line.
    let content = "a\n==\n   ---\nb";
    let Some(Node::Root(root)) = parse(content) else { panic!("{content:?} parses") };
    assert!(matches!(root.children[1], Node::ThematicBreak(_)));
    assert_eq!(span(&root.children[1]).map(|range| &content[range]), Some("   ---"));
}

#[test]
fn a_line_of_underline_marks_after_no_paragraph_starts_a_block() {
    // The headings and the definition read as an `other` each, and nothing panics.
    assert_eq!(read("a\n==\n==\nb\n=="), "root[other, other]");
    assert_eq!(read("a\n==\n---\nb\n=="), "root[other, other, other]");
    assert_eq!(read("a\n---\n---\nb\n---"), "root[other, other, other]");
    assert_eq!(read("a\n==\n--\nb\n=="), "root[other, other]");
    assert_eq!(read("[l]: x\n-\n1\n-"), "root[other, list[item[]], other]");
    assert_eq!(read("> a\n> ==\n> ==\n> b\n> =="), "root[quote[other, other]]");
    assert_eq!(read("[a]: b\n---\nc"), "root[other, other, paragraph]");
    // A line after a paragraph line still underlines it.
    assert_eq!(read("a\n==\n==\n==\nb"), "root[other, other, paragraph]");
    assert_eq!(read("# a\n==\n==\n==\nb\n=="), "root[other, other, other]");
}

#[test]
fn an_indented_first_line_of_a_container_after_a_paragraph_is_code() {
    // The code line is an `other`, so a dash line after it is a thematic break.
    assert_eq!(read("x\n>     a\n> ---"), "root[paragraph, quote[other, other]]");
    assert_eq!(read("x\n-     a\n  ---"), "root[paragraph, list[item[other, other]]]");
    assert_eq!(read("x\n>      ---\n> ---"), "root[paragraph, quote[other, other]]");
    assert_eq!(read("[a]: b\n>     c\n> d"), "root[other, quote[other, paragraph]]");
    assert_eq!(read("x\n>     - a\n> ---"), "root[paragraph, quote[other, other]]");
    // The item stays tight, as it does for HTML and a list the quote opens with.
    assert_eq!(
        read("- x\n  > >     a\n  > > ---"),
        "root[list[item[paragraph, quote[quote[other, other]]]]]"
    );
    assert_eq!(read("- x\n  > <b>"), "root[list[item[paragraph, quote[other]]]]");
    assert_eq!(read("- x\n  > 2. c"), "root[list[item[paragraph, quote[list[item[paragraph]]]]]]");
    // Three columns in, the line is paragraph text, which the dash line underlines.
    assert_eq!(read("x\n>    a\n> ---"), "root[paragraph, quote[other]]");
}

#[test]
fn tabs_in_list_and_fence_indentation_take_their_columns() {
    fn code(content: &str) -> (String, usize) {
        let mut node = parse(content).unwrap_or_else(|| panic!("{content:?} parses"));
        while !matches!(node, Node::Code(_)) {
            node = node.children().and_then(|children| children.last()).cloned().unwrap();
        }
        let Node::Code(code) = node else { unreachable!() };
        (code.value, code.position.map_or(0, |position| position.start.column))
    }

    assert_eq!(code(">\t~~~\n>    a\n>\t~~~"), (" a".to_string(), 5));
    assert_eq!(code(">\t-\t\t\tfoo"), ("  \tfoo".to_string(), 7));
    // The second item is gone once the first one is measured, and so is its padding.
    assert_eq!(code("-\ta\n\n  ```\n-\tb"), ("-\tb".to_string(), 3));
}

#[test]
fn a_fence_its_container_closes_keeps_its_blank_lines() {
    fn first_code(node: &Node) -> Option<&str> {
        match node {
            Node::Code(code) => Some(&code.value),
            _ => node.children()?.iter().find_map(first_code),
        }
    }
    let value = |content: &str| {
        let root = parse(content).unwrap_or_else(|| panic!("{content:?} parses"));
        first_code(&root).unwrap_or_else(|| panic!("{content:?} holds code")).to_string()
    };

    assert_eq!(value("- ```\n  a\n\n\nb"), "a\n\n");
    assert_eq!(value("- ```\n  a\n\nb"), "a\n");
    assert_eq!(value("-   ```\n\n\n  b"), "\n");
    assert_eq!(value("> ```\n> a\n>\n>\nb"), "a\n\n");
    assert_eq!(value("- - ```\n    a\n\n\nb"), "a\n\n");
    // markdown-rs already reads one blank line, a block ending on code, and a block the next
    // item closes.
    assert_eq!(value("- ```\n\nb"), "");
    assert_eq!(value("- ```\n  a\nb"), "a");
    assert_eq!(value("1. ~~~\n   a\n\n\n2. b"), "a\n\n");
}

#[test]
fn an_open_fence_holds_every_line_to_its_containers_end() {
    fn code(content: &str) -> (String, String) {
        let mut node = parse(content).unwrap_or_else(|| panic!("{content:?} parses"));
        while !matches!(node, Node::Code(_)) {
            node = node.children().and_then(|children| children.last()).cloned().unwrap();
        }
        let range = span(&node).unwrap();
        let Node::Code(code) = node else { unreachable!() };
        (code.value, content[range].to_string())
    }
    let read = |content: &str, value: &str, text: &str| {
        assert_eq!(code(content), (value.to_string(), text.to_string()), "{content:?}");
    };

    read("> ```text\n> b\n>", "b\n", "```text\n> b\n>");
    read("- ```\n  b\n  ", "b\n", "```\n  b\n  ");
    read("- > ```\n  > b\n  >", "b\n", "```\n  > b\n  >");
    read("> - ```\n>   b\n>", "b\n", "```\n>   b\n>");
    // markdown-rs already reads a line with code and a line ending.
    read("> ```\n> b\n> \t", "b\n\t", "```\n> b\n> \t");
    read("> ```text\n> b\n>\n", "b\n", "```text\n> b\n>\n");
    // A block holding only the blank line already has its value, but not its span.
    read("> ```\n>", "", "```\n>");
    read("- ```\n  ", "", "```\n  ");
    // A line in the span that is blank after the fence's indentation.
    read(" ```text\nb\n ", "b\n", "```text\nb\n ");
    read(" ```\nb\n\n ", "b\n\n", "```\nb\n\n ");
    // A line outside the inner quote is outside the block.
    read("> > ```\n> > b\n>", "b", "```\n> > b");
}
