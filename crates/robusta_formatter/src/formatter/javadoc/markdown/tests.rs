use super::*;

/// Normalize `content` and check that normalizing again changes nothing.
fn normalized_with(content: &str, capitalize: bool) -> String {
    let normalize =
        |content: &str| normalize(content, &Structure::from_markdown(content), capitalize);
    let normalized = normalize(content).unwrap_or_else(|| content.to_string());
    assert_eq!(normalize(&normalized), None, "not idempotent: {normalized:?}");
    normalized
}

fn normalized(content: &str) -> String {
    normalized_with(content, false)
}

/// Check that `content` is kept by its own rule: the guard would also
/// undo the emphasis edit after it.
fn assert_kept(content: &str) {
    assert_eq!(normalized(&format!("{content}\n\n*z*")), format!("{content}\n\n_z_"));
}

#[test]
fn setext_headings_become_atx_headings() {
    assert_eq!(normalized("Title\nline two\n===\n\n  Sub\n  ---"), "# Title line two\n\n## Sub");
    assert_eq!(normalized("- Item\n  Heading\n  ==="), "- # Item Heading");
    for kept in ["# Already", "Uses C#\n===", "Hard\\\nbreak\n==="] {
        assert_kept(kept);
    }
}

#[test]
fn emphasis_uses_underscores_and_strong_uses_asterisks() {
    assert_eq!(normalized("*a* and __b__ in (*c*)."), "_a_ and **b** in (_c_).");
    assert_eq!(normalized("__a *b* c__ and *a __b__ c*"), "**a _b_ c** and _a **b** c_");
    assert_eq!(normalized("*see `a_b`* now"), "_see `a_b`_ now");
    assert_eq!(normalized("*a *b* c*"), "_a *b* c_");
}

#[test]
fn emphasis_that_could_pair_differently_keeps_its_delimiters() {
    for kept in [
        "a*b*c inside a word",
        "*foo _bar* with an underscore inside",
        "***x*** and __*y*__",
        "*x*_y_ side by side",
        "\\*not emphasis\\* and *é*é",
    ] {
        assert_kept(kept);
    }
}

#[test]
fn thematic_breaks_are_kept_as_written() {
    assert_kept("Text\n\n***\n\n* * *\n\n___\n\n---");
}

#[test]
fn bullets_become_dashes_at_every_level() {
    assert_eq!(normalized("* a\n* b\n  + nested\n    more"), "- a\n- b\n  - nested\n    more");
    assert_eq!(normalized("Text\n\n  * a\n  * b"), "Text\n\n  - a\n  - b");
    assert_eq!(normalized("- a\n\t+ b"), "- a\n\t- b");
}

#[test]
fn adjacent_lists_alternate_their_markers() {
    for (source, expected) in [
        ("* a\n+ b", "- a\n\n* b"),
        ("* a\n\n- b", "- a\n\n* b"),
        ("1. a\n1) b", "1. a\n\n1) b"),
        ("+ a\n- b\n+ c", "- a\n\n* b\n\n- c"),
        ("* a\n1) b\n+ c", "- a\n1. b\n- c"),
        // Only top-level lists get a blank line, which cannot make an item loose.
        ("- item\n  * a\n  + b", "- item\n  - a\n  * b"),
        ("- item\n\t* a\n  + b", "- item\n\t- a\n  * b"),
        ("> * a\n> + b", "> - a\n> * b"),
    ] {
        assert_eq!(normalized(source), expected, "{source}");
    }
}

#[test]
fn ordered_lists_use_periods_and_count_from_their_first_number() {
    assert_eq!(normalized("1) a\n2) b"), "1. a\n2. b");
    assert_eq!(normalized("3. a\n7. b\n10. c"), "3. a\n4. b\n5. c");
    assert_eq!(normalized("8) a\n8) b\n8) c"), "8. a\n9. b\n10. c");
    assert_eq!(normalized("Text\n\n  3) a\n  3) b"), "Text\n\n  3. a\n  4. b");
    assert_eq!(normalized("8) a\n8) b\n8) c\n   - d"), "8. a\n9. b\n10. c\n    - d");
    assert_eq!(normalized("- a\n\t1) b\n\t1) c"), "- a\n\t1. b\n\t2. c");
}

#[test]
fn a_list_that_interrupts_a_paragraph_starts_after_a_blank_line() {
    assert_eq!(normalized("Text\n- a"), "Text\n\n- a");
    assert_eq!(normalized("Text\n1. a"), "Text\n\n1. a");
    assert_eq!(normalized("Text\n01. a"), "Text\n\n1. a");
    assert_eq!(normalized("- Text\n  001) a"), "- Text\n  1. a");
    assert_eq!(normalized("- a\n  - b"), "- a\n  - b");
    // After a list, markdown-rs reads these lines as lists, and javadoc does not.
    for kept in [
        "- a\n\nText\n2)  b\n    1) c",
        "- x\n\n  - a\n\n  Text\n  2) b",
        "- a\n\nText\n2. b\n3) c",
        "- a\n\nText\n2. b\n5) c\n9) d",
        "1. a\n\nText\n10) b\n11) c",
        "> - a\n>\n> Text\n> *",
    ] {
        assert_kept(kept);
    }
    assert_eq!(normalized("- a\n\nText\n-"), "- a\n\n## Text");
    assert_eq!(normalized("- a\n\nText\n2) b\n1) c"), "- a\n\nText\n2) b\n\n1. c");
}

#[test]
fn capitalization_reaches_each_list_item() {
    assert_eq!(normalized_with("- one\n- two", true), "- One\n- Two");
    assert_eq!(normalized_with("1) one", true), "1. One");
    let kept = "- `code`\n- http://example.com\n- {@code x}";
    assert_eq!(normalized_with(kept, true), kept);
    assert_eq!(normalized_with("- one", false), "- one");
    assert_eq!(normalized_with("- \u{a0}one", true), "- \u{a0}one");
}

#[test]
fn tables_are_padded_to_their_column_widths() {
    let source = "|a|`x\\|y`|right|\n|:-|:-:|--:|\n|long cell|*b*|\n";
    assert_eq!(
        normalized(source),
        concat!(
            "| a         | `x\\|y` | right |\n",
            "| :-------- | :----: | ----: |\n",
            "| long cell | _b_    |       |\n",
        )
    );
    assert_eq!(normalized("a | b\n--|--\n1 | 2"), "| a   | b   |\n| --- | --- |\n| 1   | 2   |");
    assert_eq!(normalized("- | a |\n  |---|\n  |bb|"), "- | a   |\n  | --- |\n  | bb  |");
}

#[test]
fn tables_javadoc_reads_differently_are_kept_as_written() {
    for kept in ["| a |\n|---|\n| 1 | 2 |", "| {@code a|b} |\n|---|---|", "Text\n| a |\n|---|"] {
        assert_kept(kept);
    }
}

#[test]
fn code_blocks_are_fenced_with_more_backticks_than_any_run_inside() {
    assert_eq!(normalized("Text\n\n    code\n\n      more"), "Text\n\n```\ncode\n\n  more\n```");
    assert_eq!(normalized("    has ``` inside"), "````\nhas ``` inside\n````");
    assert_eq!(normalized("    a\n      \n    b"), "```\na\n  \nb\n```");
    assert_eq!(normalized("# Title\n    code"), "# Title\n```\ncode\n```");
    for indent in ["    ", "\t", "  \t"] {
        assert_eq!(normalized(&format!("a\n\n{indent}code")), "a\n\n```\ncode\n```");
    }
    assert_eq!(normalized("a\n\t\n\tcode"), "a\n\t\n```\ncode\n```");
    assert_eq!(normalized(">\t\tcode"), "> ```\n>   code\n> ```");
    assert_eq!(normalized("~~~ java  title\nint x;\n~~~~"), "``` java  title\nint x;\n```");
    assert_eq!(normalized("````\nx\n````"), "```\nx\n```");
    assert_eq!(normalized("```java\nunclosed"), "```java\nunclosed\n```");
    assert_eq!(normalized("* ```\n- b"), "- ```\n  ```\n\n* b");
    assert_eq!(normalized("- ```\n  x\n1. y"), "- ```\n  x\n  ```\n1. y");
    assert_eq!(normalized("~~~\nx\n~~~\n    raw"), "```\nx\n```\n```\nraw\n```");
    assert_eq!(normalized("    raw\n~~~\nx\n~~~"), "```\nraw\n```\n```\nx\n```");
    assert_eq!(
        normalized("- item\n\n      code\n      more\n- next"),
        "- item\n\n  ```\n  code\n  more\n  ```\n- next"
    );
    assert_eq!(normalized("    {@code *raw*}"), "```\n{@code *raw*}\n```");
    assert_eq!(
        normalized("-     {@code *raw*}\n\n      continuation"),
        "- ```\n  {@code *raw*}\n\n  continuation\n  ```"
    );
    assert_eq!(normalized(">     {@link A}"), "> ```\n> {@link A}\n> ```");
    assert_eq!(
        normalized("- Outer\n  -     {@link A}"),
        "- Outer\n  - ```\n    {@link A}\n    ```"
    );
}

#[test]
fn code_blocks_javadoc_reads_differently_are_kept_as_written() {
    assert_kept("~~~ a`b\nx\n~~~");
}

#[test]
fn indented_lines_after_a_definition_continue_its_paragraph() {
    assert_kept("[a]: b\n    text");
    assert_kept("[a]: b\n    @return {@link A}");
}

#[test]
fn a_list_after_a_definition_starts_at_any_number() {
    assert_eq!(normalized("[a]: b\n2. x\n1) c"), "[a]: b\n2. x\n\n1) c");
    assert_eq!(normalized("[a]: b\n1) x\n1. c"), "[a]: b\n1. x\n\n1) c");
    assert_eq!(normalized("> [a]: b\n> 3) x\n> 1. c"), "> [a]: b\n> 3. x\n> 1) c");
    assert_eq!(normalized("[a]: b\n*\n  x"), "[a]: b\n-\n  x");
    assert_kept("[a]: b\n2. x\n---");
    assert_kept("[a]: b\n    2) x");
    assert_eq!(normalized("[a]: b\n    2. x\n---"), "[a]: b\n## 2. x");
}

#[test]
fn lines_after_a_closed_list_continue_a_paragraph() {
    for start in ["`x`", "{@link A}", "each", "_x_", "#1"] {
        assert_kept(&format!("- a\n\n{start} b\n    continued"));
    }
    assert_kept("> - a\n>\n> `x` b\n>     continued");
    assert_kept(">- a\n>\n>`x` b\n>     continued");
    assert_kept("> - a\n>\n> `x` b\n    lazy");
    assert_eq!(normalized("- a\n\n`x` b\n\n    code"), "- a\n\n`x` b\n\n```\ncode\n```");
}

#[test]
fn html_blocks_in_containers_opened_after_a_paragraph_stay_html() {
    assert_kept("a\n> <b>\n> *z*");
    assert_kept("a\n> </b>\n> *z*");
    assert_kept("a\n>    <b>\n> *z*");
    assert_kept("a\n> > <b>\n> > *z*");
    assert_kept("> a\n> > <b>\n> > *z*");
    assert_kept("- a\n  > <b>\n  > *z*");
    assert_kept("- a\n  - <b>\n    *z*");
    assert_kept("[a]: b\n> <c>\n> *z*");
    assert_kept("- a\n\n~j\n> <b>\n> *z*");
    assert_kept("a\n> <b>\n> > <c>\n> *z*");
    assert_kept("a\n> <b>\n> ---");
    assert_eq!(normalized("a\n- <b>\n  *z*"), "a\n\n- <b>\n  *z*");
    assert_eq!(normalized("a\n1. <b>\n   *z*"), "a\n\n1. <b>\n   *z*");
    assert_eq!(normalized("a\n- <b>\n  ==="), "a\n\n- <b>\n  ===");
}

#[test]
fn lists_in_containers_opened_after_a_paragraph_start_at_any_number() {
    assert_kept("a\n> 2. <b>\n>    *z*");
    assert_kept("a\n> -\n>   <b>\n>   *z*");
    assert_kept("[a]: b\n> 3. <b>\n>    *z*");
    assert_kept("[a]: b\n> -\n>   <b>\n>   *z*");
    assert_kept("- a\n  > > 2. <b>\n  > >    *z*");
    assert_kept("a\n> 2. x\n> 1) c");
    assert_kept("a\n> 2. x\n> ---");
    assert_eq!(normalized("a\n- 2. <b>\n     *z*"), "a\n\n- 2. <b>\n     *z*");
    assert_eq!(normalized("a\n> b\n> 2. <c>\n> *z*"), "a\n> b\n> 2. <c>\n> _z_");
}

#[test]
fn tabs_in_list_indentation_take_their_columns() {
    assert_kept(">\t2. <b>\n>\t   *z*");
    assert_kept("a\n>\t2. <b>\n>\t   *z*");
    assert_kept("1.\ta\n\n   2) b");
    assert_eq!(normalized(">\t- a\n>\n>\t  *z*"), ">\t- a\n>\n>\t  _z_");
}

#[test]
fn inline_html_continues_a_paragraph() {
    assert_eq!(normalized("a\n<b>\n*z*"), "a\n<b>\n_z_");
    assert_eq!(normalized("[a]: b\n<c>\n*z*"), "[a]: b\n<c>\n_z_");
    assert_eq!(normalized("a\n> <b> y\n> *z*"), "a\n> <b> y\n> _z_");
    assert_eq!(normalized("> a\n> <b>\n> *z*"), "> a\n> <b>\n> _z_");
    assert_eq!(normalized("a\n\n> b\n> <c>\n> *z*"), "a\n\n> b\n> <c>\n> _z_");
}

#[test]
fn links_escapes_and_inline_tags_are_kept_as_written() {
    let kept = concat!(
        "See [java.util.List], [List][java.util.List], [Map#get(Object)], and [ref].\n",
        "\\*not emphasis\\* &amp; {@code *x*} {@link A *b*}\n",
        "\n",
        "[ref]: https://example.com/*x*",
    );
    assert_kept(kept);
    assert_eq!(normalized("*y* {@code *x*}"), "_y_ {@code *x*}");
    // Neither indented fence line opens code that would hide the tag.
    assert_kept("Text\n    ~~~\n{@literal *a*}");
    assert_kept("1. ```\n   x\n   ```\n\n@param x {@literal *a*}");
}

#[test]
fn the_description_and_each_block_tag_are_separate_markdown() {
    assert_eq!(
        normalized("Intro\n1) a\n@param x text\n1) b\n2) c"),
        "Intro\n\n1. a\n@param x text\n\n1. b\n2. c"
    );
    assert_eq!(
        normalized_with("- one\n@return a list:\n- two", true),
        "- One\n@return a list:\n\n- two"
    );
    assert_kept("@return the value\n---");
    assert_kept("Text\n\n  @return the value\n  ---");
    assert_eq!(normalized("```\n@Override\n```\n* a"), "```\n@Override\n```\n- a");
    assert_eq!(normalized("- ```\n  x\n\n@param y z"), "- ```\n  x\n  ```\n\n@param y z");
}

#[test]
fn edits_that_would_change_the_rendering_are_dropped() {
    // `- - -` would be a thematic break.
    assert_eq!(normalized("* - -\n\n*a*"), "* - -\n\n*a*");
}

#[test]
fn the_normalized_structure_still_finds_tags_and_names() {
    let structure = normalized_structure(
        "See {@link List}.\n* item [Map]\n\n@param value the *input*\n@see Set",
        false,
    );

    assert!(["List", "Map", "value", "Set"].into_iter().all(|name| structure.mentions(name)));
    assert_eq!(
        structure.tags().iter().map(robusta_javadoc::Tag::name).collect::<Vec<_>>(),
        ["link", "param", "see"]
    );
}

#[test]
fn quoted_blocks_are_normalized() {
    for (source, expected) in [
        ("> Title\n> ===", "> # Title"),
        ("> > Title\n> > second line\n> > ===", "> > # Title second line"),
        ("> Title\ncontinued\n> ===", "> # Title continued"),
        ("> |a|bb|\n> |-|--|\n> |x|long|", "> | a   | bb   |\n> | --- | ---- |\n> | x   | long |"),
        ("> ~~~ text\n> raw\n> ~~~", "> ``` text\n> raw\n> ```"),
        (
            "> > ~~~ text\n> > has ``` inside\n> > ~~~",
            "> > ```` text\n> > has ``` inside\n> > ````",
        ),
        ("> ~~~ text\n> raw", "> ``` text\n> raw\n> ```"),
    ] {
        assert_eq!(normalized(source), expected, "{source}");
    }
}

#[test]
fn tables_after_closed_blocks_are_padded() {
    for prefix in ["# Heading\n", "```text\nx\n```\n"] {
        assert_eq!(
            normalized(&format!("{prefix}|a|bb|\n|-|--|\n|x|long|")),
            format!("{prefix}| a   | bb   |\n| --- | ---- |\n| x   | long |")
        );
    }
}

#[test]
fn code_on_a_list_marker_line_is_fenced_in_the_item() {
    assert_eq!(normalized("-     int   x=1;"), "- ```\n  int   x=1;\n  ```");
    assert_eq!(normalized("1.     raw"), "1. ```\n   raw\n   ```");
    assert_eq!(normalized("1.     raw\n       more"), "1. ```\n   raw\n   more\n   ```");
}

#[test]
fn indented_code_keeps_blank_lines_within_one_block() {
    for gap in ["\n", "\n\n", "\n   \n", "\n       \n", "\n\n\n"] {
        for (prefix, continuation) in [("-     ", "      "), ("1.     ", "       ")] {
            let source = format!("{prefix}raw{gap}{continuation}continuation");
            let inner_gap = gap
                .split('\n')
                .map(|line| line.get(prefix.len()..).unwrap_or(""))
                .collect::<Vec<_>>()
                .join("\n");
            let indent = " ".repeat(prefix.len() - 4);
            let value = format!("raw{inner_gap}continuation")
                .split('\n')
                .map(|line| if line.is_empty() { String::new() } else { format!("{indent}{line}") })
                .collect::<Vec<_>>()
                .join("\n");
            let marker = prefix.trim_end();
            assert_eq!(normalized(&source), format!("{marker} ```\n{value}\n{indent}```"));
        }
    }
    assert_eq!(
        normalized("8) a\n8) b\n8)     raw\n\n       continuation"),
        "8. a\n9. b\n10. ```\n    raw\n\n    continuation\n    ```"
    );
    assert_eq!(normalized("    raw\n\n    continuation"), "```\nraw\n\ncontinuation\n```");
    assert_eq!(
        normalized("- item\n\n      raw\n\n      continuation"),
        "- item\n\n  ```\n  raw\n\n  continuation\n  ```"
    );
}

#[test]
fn nested_indented_code_is_not_list_item_prose() {
    for marker in ["-     ", "1.     ", "-\t\t"] {
        for gap in ["\n", "\n\n", "\n  \n", "\n  \t\n", "\n\n\n"] {
            for literal in
                ["raw", "*raw*", "__raw__", "`raw`", r"\*raw\*", "[ref]: url", "[^note]: *raw*"]
            {
                let source = format!("- Outer\n  {marker}{literal}{gap}        continuation");
                let output = normalized(&source);
                assert!(output.contains(literal), "{output}");
                assert_eq!(normalized_with(&source, true), output);
            }
        }
    }
    assert_eq!(normalized_with("- outer\n  -    prose", true), "- Outer\n  -    Prose");
    assert_eq!(
        normalized_with("- Outer\n  -     raw\n\n    *prose*\n\n    ~~~\n    more\n    ~~~", true),
        "- Outer\n  - ```\n    raw\n    ```\n\n    _prose_\n\n    ```\n    more\n    ```"
    );
}

#[test]
fn quote_prefixes_and_literal_content_survive_normalization() {
    assert_eq!(
        normalized(">> Title\n> > next\n>> ===\n> >\n>> ~~~ text\n> > *raw* &amp; ```\n>> ~~~"),
        ">> # Title next\n> >\n>> ```` text\n> > *raw* &amp; ```\n>> ````"
    );
    assert_eq!(
        normalized("> > |a|`x\\|y`|\n>> |-|--|\n> > |[List][java.util.List]|&amp;|"),
        "> > | a                      | `x\\|y` |\n>> | ---------------------- | ------ |\n> > | [List][java.util.List] | &amp;  |"
    );
    assert_kept("> Text\n> |a|\n> |-|");
    assert_kept("> ~~~ a`b\n> x\n> ~~~");
    assert_kept("> {@code Title\n> ===}");
}

#[test]
fn quote_markers_in_code_do_not_close_a_fence() {
    for (source, expected) in [
        ("~~~\nraw\n> ~~~", "```\nraw\n> ~~~\n```"),
        ("> ~~~\n> raw\n> > ~~~", "> ```\n> raw\n> > ~~~\n> ```"),
        ("> > ~~~\n> > raw\n> > > ~~~", "> > ```\n> > raw\n> > > ~~~\n> > ```"),
        ("> ~~~\n> > ~~~\n> ~~~", "> ```\n> > ~~~\n> ```"),
    ] {
        assert_eq!(normalized(source), expected, "{source}");
    }
}

#[test]
fn a_fence_open_at_a_section_end_closes_after_its_last_non_empty_line() {
    for (source, expected) in [
        ("> ```text\n> b\n>", "> ```text\n> b\n>\n> ```"),
        ("```text\nb\n\n\n", "```text\nb\n```\n\n\n"),
        ("- ```\n  b\n  ", "- ```\n  b\n  \n  ```"),
        ("- ```\n  b\n  \n\n@param y z", "- ```\n  b\n  \n  ```\n\n@param y z"),
        // javadoc keeps whitespace at the end of the code.
        ("```text\nb  ", "```text\nb  \n```"),
        ("```text\nb\n> ", "```text\nb\n> \n```"),
        ("```text\nb\u{a0}", "```text\nb\u{a0}\n```"),
        ("> ```\n>", "> ```\n>\n> ```"),
        ("- ```\n  ", "- ```\n  \n  ```"),
    ] {
        assert_eq!(normalized(source), expected, "{source:?}");
    }
}

#[test]
fn ordered_marker_width_moves_continuation_blocks() {
    for (source, expected) in [
        (
            "8) a\n8) b\n8) c\n\n   paragraph\n\n   - d",
            "8. a\n9. b\n10. c\n\n    paragraph\n\n    - d",
        ),
        (
            "1) a\n100) b\n     continuation\n\n     - nested",
            "1. a\n2. b\n   continuation\n\n   - nested",
        ),
        ("> 8) a\n> 8) b\n> 8) c\n>    - d", "> 8. a\n> 9. b\n> 10. c\n>     - d"),
        (
            "8) a\n8) b\n8) 8) c\n   8) d\n   8) e\n      continuation",
            "8. a\n9. b\n10. 8. c\n    9. d\n    10. e\n        continuation",
        ),
        (
            "8) a\n8) b\n8) c\n   ~~~ text\n   raw\n   ~~~",
            "8. a\n9. b\n10. c\n    ``` text\n    raw\n    ```",
        ),
        ("8) a\n8) b\n8) c\n\n       raw", "8. a\n9. b\n10. c\n\n    ```\n    raw\n    ```"),
        (
            "8) a\n8) b\n8) c\n   hard  \n   break\n\n   [List][java.util.List] &amp; {@code raw}",
            "8. a\n9. b\n10. c\n    hard  \n    break\n\n    [List][java.util.List] &amp; {@code raw}",
        ),
        ("8) a\n8) b\n8) c\nlazy continuation", "8. a\n9. b\n10. c\nlazy continuation"),
        ("8) a\n8) b\n8) c\n\t- d", "8. a\n9. b\n10. c\n     - d"),
        ("1) a\n100) b\n\t - nested", "1. a\n2. b\n   - nested"),
    ] {
        assert_eq!(normalized(source), expected, "{source}");
    }
}

#[test]
fn ordered_numbers_stop_at_the_largest_marker() {
    assert_eq!(
        normalized("999999998) a\n0) b\n0) c\n   - nested"),
        "999999998. a\n999999999. b\n999999999. c\n           - nested"
    );
    assert_eq!(normalized("8) a\n08) b\n    continuation"), "8. a\n9. b\n   continuation");
}

#[test]
fn complete_container_code_is_fenced_as_one_block() {
    for (source, expected) in [
        ("-     raw\n\n      continuation", "- ```\n  raw\n\n  continuation\n  ```"),
        (
            "- Outer\n  -     *raw*\n  \n        continuation",
            "- Outer\n  - ```\n    *raw*\n\n    continuation\n    ```",
        ),
        (">     raw", "> ```\n> raw\n> ```"),
        (">     raw\n>     more", "> ```\n> raw\n> more\n> ```"),
        (
            "- >     *raw*\n  >\n  >     continuation",
            "- > ```\n  > *raw*\n  >\n  > continuation\n  > ```",
        ),
        ("> -\t\traw\n> \t\tcontinuation", "> -\t```\n>  \traw\n>  \tcontinuation\n>  \t```"),
    ] {
        assert_eq!(normalized_with(source, true), expected, "{source}");
    }
}

#[test]
fn indented_code_preserves_section_ends_and_list_tightness() {
    assert_eq!(normalized("a\n\n    code  "), "a\n\n```\ncode  \n```");
    assert_eq!(
        normalized("-     raw\n      continuation\t"),
        "- ```\n  raw\n  continuation\t\n  ```"
    );
    assert_eq!(
        normalized("-     raw\n      continuation\n      \t"),
        "- ```\n  raw\n  continuation\n  ```"
    );
    assert_eq!(
        normalized("-     raw\n      continuation \n\n  prose"),
        "- ```\n  raw\n  continuation \n  ```\n  prose"
    );
    assert_eq!(
        normalized("-     raw\n      continuation\n      \t\n\n  prose"),
        "- ```\n  raw\n  continuation\n  ```\n  prose"
    );
}
