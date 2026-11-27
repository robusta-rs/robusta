use cow_utils::CowUtils;

use crate::fixtures::{
    format_source_with_options, format_stable_with_options, indent_width, line_width,
};
use robusta_formatter::{IndentStyle, JavadocOptions, LineEnding, Options};

fn options() -> Options {
    Options { javadoc: Some(JavadocOptions::default()), ..Options::default() }
}

#[test]
fn traditional_examples_format_each_java_context() {
    for (code, expected) in [
        ("int   x=1;", "int x = 1;"),
        ("public void f(){run();}", "public void f() {\n *   run();\n * }"),
        ("class B{int x=1;}", "class B {\n *   int x = 1;\n * }"),
        ("if(ok){run();}", "if (ok) {\n *   run();\n * }"),
    ] {
        for (open, separator, close) in
            [("<pre>{@code", " ", "}</pre>"), ("{@snippet :", "\n * ", "}")]
        {
            let source = format!("/** {open}{separator}{code}{close} */\nclass A {{}}\n");
            let actual = format_stable_with_options(&source, &options());
            assert!(actual.contains(&format!("{open}\n * {expected}\n * {close}")), "{actual}");
        }
    }
}

#[test]
fn examples_keep_invalid_code_inline_code_and_snippet_markup() {
    for text in [
        "{@code int   x=1;}",
        "<pre>int   x=1;</pre>",
        "<pre>{@code int   x=;}</pre>",
        "{@snippet : int   x=1;}",
        "{@snippet :}",
        "{@snippet : // note\nint   x=1;}",
        "{@snippet :\nint   x=;}",
        "{@snippet :\x0c\nint   x=;}",
        "{@snippet :\x0c\nint   x=1;}",
        "{@snippet file=\"Example.java\" :\nint   x=1;}",
        "{@snippet class=Example :\nint   x=1;}",
        "{@snippet lang=python :\nint   x=1;}",
        "{@snippet :\nint   x=1; // @highlight substring=\"x\"\n}",
        "{@snippet :\nint   x=1; // @replace substring=\"x\" replacement=\"y\"\n}",
        "{@snippet :\nint   x=1; // @start region=\"a\"\n}",
    ] {
        let source = format!("/**\n * {}\n */\nclass A {{}}\n", text.cow_replace('\n', "\n * "));
        let actual = format_stable_with_options(&source, &options());
        assert!(actual.contains(text.cow_replace('\n', "\n * ").as_ref()), "{actual}");
    }
}

#[test]
fn snippets_drop_spaces_and_tabs_after_the_colon() {
    let source = "/**\n * {@snippet :  \t\n * int   x=1;\n * }\n */\nclass A {}\n";
    let actual = format_stable_with_options(source, &options());
    assert!(actual.contains("{@snippet :\n * int x = 1;\n * }"), "{actual}");
}

#[test]
fn markdown_snippets_keep_a_form_feed_after_the_colon() {
    let source = "/// {@snippet :\x0c\n/// int   x=1;\n/// }\nclass A {}\n";
    assert_eq!(format_stable_with_options(source, &options()), source);
}

#[test]
fn snippets_accept_quoted_attributes_and_plain_java_comments() {
    let source =
        "/** {@snippet id=\"a:b\" lang='java' :\n * int   x=1; // value\n * } */\nclass A {}";
    let actual = format_stable_with_options(source, &options());
    assert!(
        actual.contains("{@snippet id=\"a:b\" lang='java' :\n * int x = 1; // value\n * }"),
        "{actual}"
    );
}

#[test]
fn markdown_formats_java_and_untagged_blocks_inside_containers() {
    for prefix in ["", "> ", "- "] {
        let continuation = if prefix == "- " { "  " } else { prefix };
        for language in ["java", "", "JAVA"] {
            let source = format!(
                "/// {prefix}```{language}\n/// {continuation}int   x=1;\n/// {continuation}```\nclass A {{}}\n"
            );
            let actual = format_stable_with_options(&source, &options());
            assert!(actual.contains(&format!("/// {continuation}int x = 1;")), "{actual}");
        }
    }
    let indented = "/// Example:\n///\n///     int   x=1;\nclass A {}\n";
    assert!(format_stable_with_options(indented, &options()).contains("/// int x = 1;"));
}

#[test]
fn markdown_blank_code_lines_inside_containers_end_at_the_last_marker() {
    for (prefix, continuation, blank) in [("- ", "  ", "///"), ("> ", "> ", "/// >")] {
        let source = format!(
            "/// {prefix}```java\n/// {continuation}int  x;\n{blank}\n/// {continuation}int  y;\n/// {continuation}```\nclass A {{}}\n"
        );
        let expected = source.cow_replace("int  ", "int ");
        assert_eq!(format_stable_with_options(&source, &options()), expected);
    }
}

#[test]
fn empty_examples_gain_no_code_line() {
    for (source, expected) in [
        ("/// ```java\nclass A {}\n", "/// ```java\n/// ```\nclass A {}\n"),
        ("/// ~~~\nclass A {}\n", "/// ```\n/// ```\nclass A {}\n"),
        ("/// - ~~~\nclass A {}\n", "/// - ```\n///   ```\nclass A {}\n"),
        ("/// > ```\nclass A {}\n", "/// > ```\n/// > ```\nclass A {}\n"),
        (
            "/// ```java\n/// int  x;\nclass A {}\n",
            "/// ```java\n/// int x;\n/// ```\nclass A {}\n",
        ),
    ] {
        assert_eq!(format_stable_with_options(source, &options()), expected);
    }
    for source in [
        "/// ```java\n/// ```\nclass A {}\n",
        "/**\n * {@snippet :\n * }\n */\nclass A {}\n",
        "/**\n * <pre>{@code\n * }</pre>\n */\nclass A {}\n",
    ] {
        assert_eq!(format_stable_with_options(source, &options()), source);
    }
}

#[test]
fn markdown_java_fences_their_container_closes_keep_the_next_line_outside() {
    // The list item, definition, and tilde fence would change the Markdown reading.
    let source =
        "/// 1. ```\n///    int  x;\n/// 10. [a]: b\n///    ~~~\n/// b c\n/// - *z*\nclass A {}\n";
    assert_eq!(
        format_stable_with_options(source, &options()),
        "/// 1. ```\n///    int x;\n///    ```\n/// 10. [a]: b\n///    ~~~\n/// b c\n/// - *z*\nclass A {}\n"
    );
}

/// A fence its container closes after blank lines holds those lines as javadoc reads them, so
/// closing the fence after them keeps the section's meaning and the first pass normalizes it.
#[test]
fn markdown_java_fences_their_container_closes_after_blank_lines_normalize_on_the_first_pass() {
    let source = "/// - ```\n///   int  x;\n///\n///\n/// *z*\nclass A {}\n";
    assert_eq!(
        format_stable_with_options(source, &options()),
        "/// - ```\n///   int x;\n///   ```\n/// _z_\nclass A {}\n"
    );
}

/// A Java fence left open at the end of a quote holds the quote's last line, so the formatted
/// snippet closes after it.
#[test]
fn markdown_java_fences_open_at_the_end_of_a_quote_close_after_its_last_line() {
    let source = "/// > ```java\n/// > int  x;\n/// >\nclass A {}\n";
    assert_eq!(
        format_stable_with_options(source, &options()),
        "/// > ```java\n/// > int x;\n/// > ```\nclass A {}\n"
    );
}

/// The formatted snippet's lines take the opening line's indentation, so the comment drops the
/// indentation all its lines then share, as javadoc does when it reads them.
#[test]
fn markdown_java_fences_after_an_indented_quote_marker_keep_the_comment_margin() {
    let source = "///   > ```java\n/// > int  x;\nclass A {}\n";
    assert_eq!(
        format_stable_with_options(source, &options()),
        "/// > ```java\n/// > int x;\n/// > ```\nclass A {}\n"
    );
}

#[test]
fn markdown_fence_text_continuing_a_paragraph_after_a_closed_list_stays_text() {
    let source = "/// - m\n///\n/// a\n/// 10. ```java\n///     int  x;\n///\n/// c\nclass A {}\n";
    assert_eq!(
        format_stable_with_options(source, &options()),
        "/// - m\n///\n/// a 10. ```java int x;\n///\n/// c\nclass A {}\n"
    );
}

#[test]
fn markdown_keeps_other_languages_invalid_examples_and_inline_code() {
    for (language, code) in [("python", "int   x=1;"), ("java", "int   x=;")] {
        let source = format!("/// ```{language}\n/// {code}\n/// ```\nclass A {{}}\n");
        assert_eq!(format_source_with_options(&source, &options()), source);
    }
    let inline = "/// Use `int   x=1;` here.\nclass A {}\n";
    assert_eq!(format_source_with_options(inline, &options()), inline);
}

#[test]
fn embedded_documentation_and_import_order_are_preserved() {
    let source = "/// ```java\n/// import z.Z;\n/// import static z.Z.m;\n/// import a.A;\n/// /**   a comment. */\n/// ///   a Markdown comment.\n/// class B{int   x=1;}\n/// ```\nclass A {}\n";
    let actual = format_stable_with_options(source, &options());
    assert!(
        actual.contains("/// import z.Z;\n/// import static z.Z.m;\n/// import a.A;"),
        "{actual}"
    );
    assert!(actual.contains("/// /**   a comment. */\n/// ///   a Markdown comment."), "{actual}");
    assert!(actual.contains("/// class B {\n///   int x = 1;\n/// }"), "{actual}");
}

#[test]
fn embedded_documentation_does_not_recurse() {
    let source = "/// ```java\n/// /** <pre>{@code int   y=1;}</pre> */\n/// class B{}\n/// ```\nclass A {}\n";
    let actual = format_stable_with_options(source, &options());
    assert!(actual.contains("/// /** <pre>{@code int   y=1;}</pre> */"), "{actual}");
}

#[test]
fn examples_cannot_escape_the_fragment_wrapper() {
    for code in ["int x; } class Lost {", "run(); } void lost() {", "int x; } // tail"] {
        let source = format!("/// ```java\n/// {code}\n/// ```\nclass A {{}}\n");
        assert_eq!(format_source_with_options(&source, &options()), source);
    }
}

#[test]
fn example_layout_inherits_options_and_reserves_the_comment_margin() {
    let source = "/// ```java\n/// if(ok){call(firstArgument,secondArgument,thirdArgument);}\n/// ```\nclass A {}\n";
    for indent_style in [IndentStyle::Space, IndentStyle::Tab] {
        for width in [40, 60, 100] {
            for indent in [2, 4] {
                let options = Options {
                    line_width: line_width(width),
                    indent_width: indent_width(indent),
                    indent_style,
                    line_ending: LineEnding::Crlf,
                    ..options()
                };
                let actual = format_source_with_options(source, &options);
                assert!(!actual.cow_replace("\r\n", "").contains('\n'));
                for line in actual.lines() {
                    let columns = line
                        .chars()
                        .map(|ch| if ch == '\t' { usize::from(indent) } else { 1 })
                        .sum::<usize>();
                    assert!(columns <= usize::from(width), "{width}: {actual}");
                }
            }
        }
    }
}

#[test]
fn disabled_javadoc_keeps_embedded_examples() {
    let source = "/** <pre>{@code int   x=1;}</pre> */\nclass A {}\n";
    assert_eq!(format_source_with_options(source, &Options::default()), source);
}

#[test]
fn nested_examples_use_the_emitted_indent_even_when_source_margins_are_ragged() {
    for body in [
        "/** <pre>{@code call(firstArgument,secondArgument);}</pre> */",
        "/** {@snippet :\n * call(firstArgument,secondArgument);} */",
        "/// ```java\n/// call(firstArgument,secondArgument);\n/// ```",
        "/// <pre>{@code call(firstArgument,secondArgument);}</pre>",
        "/// > ```java\n/// > call(firstArgument,secondArgument);\n/// > ```",
        "/// > - ```java\n/// >   call(firstArgument,secondArgument);\n/// >   ```",
        "/// >\t```java\n/// >\tcall(firstArgument,secondArgument);\n/// >\t```",
    ] {
        let source = format!("class A {{\nclass B {{\n{body}\nint x;\n}}\n}}\n");
        for indent_style in [IndentStyle::Space, IndentStyle::Tab] {
            for indent in [2, 4] {
                let options = Options {
                    line_width: line_width(40),
                    indent_width: indent_width(indent),
                    indent_style,
                    ..options()
                };
                let actual = format_source_with_options(&source, &options);
                for line in actual.lines() {
                    let columns = line
                        .chars()
                        .map(|ch| if ch == '\t' { usize::from(indent) } else { 1 })
                        .sum::<usize>();
                    assert!(columns <= 40, "{columns}: {actual}");
                }
            }
        }
    }
}

#[test]
fn examples_keep_surrounding_text_empty_bodies_and_trailing_comments() {
    for code in ["", "// tail", "run();\n// tail", "int x;\n// tail"] {
        for (open, separator, close) in
            [("<pre>{@code", " ", "}</pre>"), ("{@snippet :", "\n", "}")]
        {
            let source = format!(
                "/**\n * Before. {open}{separator}{code}\n{close} After.\n * @return the value\n */\nclass A {{}}\n"
            );
            let actual = format_stable_with_options(&source, &options());
            assert!(actual.contains("Before."), "{actual}");
            assert!(actual.contains("After."), "{actual}");
            assert!(actual.contains("@return the value"), "{actual}");
            if code.contains("tail") {
                assert!(actual.contains("tail"), "{actual}");
            }
        }
    }
}

#[test]
fn wrapped_examples_keep_the_comments_after_their_last_member_or_statement() {
    for (code, expected) in [
        ("int   x;\n// tail", "int x;\n// tail"),
        ("run( );\n\n// tail\n// end", "run();\n\n// tail\n// end"),
    ] {
        for (open, margin, close) in
            [("/**\n * <pre>{@code", " *", " * }</pre>\n */"), ("/// ```java", "///", "/// ```")]
        {
            let wrap = |code: &str| {
                let lines: Vec<_> = code
                    .lines()
                    .map(|line| {
                        if line.is_empty() {
                            margin.to_string()
                        } else {
                            format!("{margin} {line}")
                        }
                    })
                    .collect();
                format!("{open}\n{}\n{close}\nclass A {{}}\n", lines.join("\n"))
            };
            assert_eq!(format_stable_with_options(&wrap(code), &options()), wrap(expected));
        }
    }
}

#[test]
fn unattached_markdown_comments_keep_their_code() {
    let source = "class A {\n  void f() {\n    /// ```java\n    /// call(  argument );\n    /// ```\n    run();\n  }\n}\n";
    assert_eq!(format_source_with_options(source, &options()), source);
}

#[test]
fn rewritten_examples_keep_cr_line_endings() {
    let source = "class A {\n/// ```java\n/// call(  argument );\n/// ```\nint x;\n}\n";
    let actual =
        format_source_with_options(source, &Options { line_ending: LineEnding::Cr, ..options() });
    assert!(!actual.contains('\n'), "{actual:?}");
    assert!(actual.contains("\r  /// call(argument);\r"), "{actual:?}");
}

#[test]
fn multiline_embedded_documentation_keeps_its_payload_and_ragged_stars() {
    let source = "/// ```java\n/// class B{\n/// /**\n///     *   ragged   text\n///  ** second\n/// no star\n/// */\n/// ///   first  text\n///   /// second\n/// int x=1;\n/// }\n/// ```\nclass A {}\n";
    let actual = format_stable_with_options(source, &options());
    assert!(
        actual
            .contains("///   /**\n///     *   ragged   text\n///  ** second\n/// no star\n/// */"),
        "{actual}"
    );
    assert!(actual.contains("///   ///   first  text\n///   /// second"), "{actual}");
    assert!(actual.contains("///   int x = 1;"), "{actual}");
}

#[test]
fn traditional_code_handles_quoted_html_attributes_and_whitespace() {
    for open in ["<pre title=\">\">", "<PRE>", "<pRe\n * title=\">\">"] {
        let source = format!("/** {open}{{@code\u{c}int   x=1;}}</pRe > */\nclass A {{}}\n");
        let actual = format_stable_with_options(&source, &options());
        assert!(actual.contains(open), "{actual}");
        assert!(actual.contains(" * int x = 1;\n * }</pRe >"), "{actual}");
    }
}

#[test]
fn preceding_comments_do_not_become_embedded_code_margins() {
    for preceding in ["/* before */", "/** before */"] {
        let source = format!("{preceding} /** <pre>{{@code int x=1;}}</pre> */\nclass A{{}}\n");
        let actual = format_stable_with_options(&source, &options());
        assert_eq!(actual.matches("before").count(), 1, "{actual}");
        let star = preceding.len() + 2;
        assert!(actual.contains(&format!("\n{:star$}* int x = 1;\n", "")), "{actual}");
    }
}

#[test]
fn embedded_code_margins_line_up_under_a_comment_after_code() {
    let source = "class A {\nint n; /** <pre>{@code int x=1;}</pre>\n * @see B */\nint m;\n}\n";
    assert_eq!(
        format_stable_with_options(source, &options()),
        "class A {\n  int n; /**\n          * <pre>{@code\n          * int x = 1;\n          * }</pre>\n          *\n          * @see B\n          */\n  int m;\n}\n"
    );
}
