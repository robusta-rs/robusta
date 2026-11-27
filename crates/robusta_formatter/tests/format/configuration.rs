//! Formatter option behavior.

use cow_utils::CowUtils;
use insta::assert_snapshot;
use robusta_formatter::{
    AfterClosingBrace, ImportGroup, ImportMatcher, ImportOrder, ImportSection, IndentStyle,
    JavadocOptions, LineEnding, Options,
};

use crate::fixtures::{
    format_source_with_options as fmt_with_options, format_stable_with_options, indent_width,
    line_width,
};

fn import_group_options(groups: &[&[&str]]) -> Options {
    Options {
        import_order: ImportOrder {
            groups: groups
                .iter()
                .map(|matchers| ImportGroup {
                    matchers: matchers
                        .iter()
                        .map(|matcher| {
                            if *matcher == "*" {
                                ImportMatcher::All
                            } else {
                                ImportMatcher::Prefix((*matcher).to_string())
                            }
                        })
                        .collect(),
                })
                .collect(),
            ..ImportOrder::default()
        },
        ..Options::default()
    }
}

#[test]
fn configured_import_groups_are_sorted_and_spaced() {
    let source = r"import static z.Tools.z;
import static org.Tools.o;
import static java.util.Collections.emptyList;
import module java.base;
import com.other.Z;
import org.beta.B;
import javax.inject.Inject;
import java.util.List;
import org.alpha.A;

class Uses {}
";
    let options = import_group_options(&[&["java", "javax"], &["org"], &["*"]]);

    assert_snapshot!(fmt_with_options(source, &options), @"
    import module java.base;

    import java.util.List;
    import javax.inject.Inject;

    import org.alpha.A;
    import org.beta.B;

    import com.other.Z;

    import static java.util.Collections.emptyList;

    import static org.Tools.o;

    import static z.Tools.z;

    class Uses {}
    ");
}

#[test]
fn import_groups_use_the_longest_matching_entry() {
    let source = "import com.example.A; import org.slf4j.Logger; import com.google.C; \
        import com.examples.D; import static com.example.Tools.a; \
        import static org.Tools.z; import static com.google.Tools.c; class Uses {}";
    for (groups, expected) in [
        (
            vec![&["*"][..], &["com.example"]],
            "import com.examples.D;\nimport com.google.C;\nimport org.slf4j.Logger;\n\n\
             import com.example.A;\n\n\
             import static com.google.Tools.c;\nimport static org.Tools.z;\n\n\
             import static com.example.Tools.a;\n\nclass Uses {}\n",
        ),
        (
            vec![&["com"][..], &["com.example"]],
            "import com.examples.D;\nimport com.google.C;\n\n\
             import com.example.A;\n\nimport org.slf4j.Logger;\n\n\
             import static com.google.Tools.c;\n\nimport static com.example.Tools.a;\n\n\
             import static org.Tools.z;\n\nclass Uses {}\n",
        ),
        (
            vec![&["com.example"][..], &["*"], &["com"]],
            "import com.example.A;\n\nimport org.slf4j.Logger;\n\n\
             import com.examples.D;\nimport com.google.C;\n\n\
             import static com.example.Tools.a;\n\nimport static org.Tools.z;\n\n\
             import static com.google.Tools.c;\n\nclass Uses {}\n",
        ),
        (
            vec![&["*", "com"][..], &["com.example"]],
            "import org.slf4j.Logger;\nimport com.examples.D;\nimport com.google.C;\n\n\
             import com.example.A;\n\n\
             import static org.Tools.z;\nimport static com.google.Tools.c;\n\n\
             import static com.example.Tools.a;\n\nclass Uses {}\n",
        ),
        (
            vec![],
            "import com.example.A;\nimport com.examples.D;\nimport com.google.C;\n\
             import org.slf4j.Logger;\n\n\
             import static com.example.Tools.a;\nimport static com.google.Tools.c;\n\
             import static org.Tools.z;\n\nclass Uses {}\n",
        ),
    ] {
        let options = import_group_options(&groups);
        assert_eq!(fmt_with_options(source, &options), expected, "{groups:?}");
    }
}

#[test]
fn nested_import_groups_keep_their_listed_order() {
    let source = "import com.example.billing.Invoice; import org.slf4j.Logger; \
        import javax.inject.Inject; import com.example.platform.Clock; \
        import java.util.List; import com.google.common.collect.ImmutableList;";
    let options = import_group_options(&[
        &["java", "javax"],
        &["*"],
        &["com.example.platform", "com.example.billing"],
    ]);
    assert_snapshot!(fmt_with_options(source, &options), @"
    import java.util.List;
    import javax.inject.Inject;

    import com.google.common.collect.ImmutableList;
    import org.slf4j.Logger;

    import com.example.platform.Clock;
    import com.example.billing.Invoice;
    ");
}

#[test]
fn nested_import_entries_sort_names_within_each_section() {
    let source = "import java.util.Set; import javax.inject.Inject; import java.util.List; \
        import static java.util.Collections.emptySet; import static javax.Tools.z; \
        import static java.util.Collections.emptyList; import static javax.Tools.a; \
        import module javax.foo; import module java.base;";
    let options = import_group_options(&[&["javax", "java"]]);
    assert_snapshot!(fmt_with_options(source, &options), @"
    import module java.base;
    import module javax.foo;

    import javax.inject.Inject;
    import java.util.List;
    import java.util.Set;

    import static javax.Tools.a;
    import static javax.Tools.z;
    import static java.util.Collections.emptyList;
    import static java.util.Collections.emptySet;
    ");
}

#[test]
fn new_line_after_closing_brace_moves_else_catch_and_finally() {
    let source = "class Cache { Value get(Key key) { try { if (entries.containsKey(key)) { \
                  return entries.get(key); } else if (key.isTransient()) { return compute(key); \
                  } else { return load(key); } } catch (IOException e) { \
                  throw new UncheckedIOException(e); } finally { lock.unlock(); } } }";
    let options = Options {
        indent_width: indent_width(2),
        after_closing_brace: AfterClosingBrace::NewLine,
        ..Options::default()
    };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class Cache {
      Value get(Key key) {
        try {
          if (entries.containsKey(key)) {
            return entries.get(key);
          }
          else if (key.isTransient()) {
            return compute(key);
          }
          else {
            return load(key);
          }
        }
        catch (IOException e) {
          throw new UncheckedIOException(e);
        }
        finally {
          lock.unlock();
        }
      }
    }
    ");
}

#[test]
fn default_options_snapshot_pins_every_formatter_policy() {
    assert_eq!(
        Options::default(),
        Options {
            line_width: line_width(100),
            indent_style: IndentStyle::Space,
            indent_width: indent_width(2),
            after_closing_brace: AfterClosingBrace::SameLine,
            import_order: ImportOrder::default(),
            javadoc: None,
            line_ending: LineEnding::Lf,
            insert_final_newline: true,
        }
    );
}

#[test]
fn tab_style_indents_wrapped_lines_with_tabs() {
    let options = Options {
        line_width: line_width(44),
        indent_style: IndentStyle::Tab,
        indent_width: indent_width(4),
        ..Options::default()
    };
    let formatted = format_stable_with_options(
        "class A { void f() { extremelyLongTargetName = call(alphaAlphaAlpha, betaBetaBeta); \
         anotherLongTargetName = extremelyLongValueName; } }",
        &options,
    );

    assert!(formatted.contains("\n\tvoid f() {\n"), "{formatted:?}");
    // A value that breaks internally keeps the operator on its line, and its
    // entries take one more level.
    assert!(formatted.contains("\n\t\textremelyLongTargetName = call(\n"), "{formatted:?}");
    assert!(formatted.contains("\n\t\t\talphaAlphaAlpha,\n"), "{formatted:?}");
    // A value with nowhere to break moves to a line of its own, one level deeper.
    assert!(formatted.contains("\n\t\tanotherLongTargetName =\n"), "{formatted:?}");
    assert!(formatted.contains("\n\t\t\textremelyLongValueName;\n"), "{formatted:?}");
    assert!(formatted.lines().all(|line| tabs_then_spaces(line).1 == 0), "{formatted:?}");
}

#[test]
fn tab_style_nests_blocks_one_tab_deeper_than_wrapped_lines() {
    let anonymous_class = "class A { void m() { Runnable r = condition ? new Runnable() { \
         public void run() { doSomethingLong(); } } : fallbackRunnable; } }";
    let lambda_block = "class A { void m() { boolean ok = items.stream().filter(x -> x.isValid() \
         && x.hasPermission(p -> { check(p); return true; })).findFirst().isPresent(); } }";

    for indent in [4, 2] {
        let options = Options {
            line_width: line_width(40),
            indent_style: IndentStyle::Tab,
            indent_width: indent_width(indent),
            ..Options::default()
        };

        assert_eq!(
            fmt_with_options(anonymous_class, &options),
            "class A {\n\tvoid m() {\n\t\tRunnable r = condition\n\
             \t\t\t? new Runnable() {\n\
             \t\t\t\t\tpublic void run() {\n\
             \t\t\t\t\t\tdoSomethingLong();\n\
             \t\t\t\t\t}\n\
             \t\t\t\t}\n\
             \t\t\t: fallbackRunnable;\n\t}\n}\n",
            "indent {indent}"
        );

        let formatted = format_stable_with_options(lambda_block, &options);
        let lines: Vec<(usize, usize)> = formatted.lines().map(tabs_then_spaces).collect();
        let opener = formatted.lines().position(|line| line.ends_with("p -> {")).unwrap();
        let closer = opener
            + formatted
                .lines()
                .skip(opener)
                .position(|line| line.trim_start().starts_with('}'))
                .unwrap();
        let (tabs, spaces) = lines[opener];
        assert_eq!(spaces, 0, "{formatted:?}");
        assert_eq!(lines[opener + 1], (tabs + 1, 0), "{formatted:?}");
        assert_eq!(lines[closer], (tabs, 0), "{formatted:?}");
    }
}

#[test]
fn zero_indent_width_formats_flush_left_and_counts_tabs_as_zero_columns() {
    let source = "class A { void m() { if (ready) { call(alpha, beta); } } }";
    let spaces =
        Options { line_width: line_width(20), indent_width: indent_width(0), ..Options::default() };

    assert_eq!(
        fmt_with_options(source, &spaces),
        "class A {\nvoid m() {\nif (ready) {\ncall(alpha, beta);\n}\n}\n}\n"
    );

    let tabs = Options { indent_style: IndentStyle::Tab, ..spaces };
    // Three tabs and the 18-column call fit in 20 columns only because each
    // tab counts as 0.
    assert_eq!(
        fmt_with_options(source, &tabs),
        "class A {\n\tvoid m() {\n\t\tif (ready) {\n\t\t\tcall(alpha, beta);\n\t\t}\n\t}\n}\n"
    );
}

/// Counts a line's leading tabs and the spaces after them, and fails if any
/// other whitespace follows.
fn tabs_then_spaces(line: &str) -> (usize, usize) {
    let after_tabs = line.trim_start_matches('\t');
    let content = after_tabs.trim_start_matches(' ');
    assert!(!content.starts_with(char::is_whitespace), "{line:?}");
    (line.len() - after_tabs.len(), after_tabs.len() - content.len())
}

#[test]
fn final_newline_can_be_omitted_without_losing_idempotency() {
    let options = Options { insert_final_newline: false, ..Options::default() };

    assert_eq!(fmt_with_options("class A {}\n", &options), "class A {}");
    assert_eq!(fmt_with_options("class A {} // trailing\n", &options), "class A {} // trailing");
}

#[test]
fn final_newline_policy_applies_to_file_end_comments() {
    let cases = [
        ("// only", "// only\n"),
        ("/* only */", "/* only */\n"),
        ("/* first */ /* last */", "/* first */ /* last */\n"),
        ("// first\n\n// last", "// first\n\n// last\n"),
        ("\u{feff}// only", "\u{feff}// only\n"),
        ("package a;\n// end", "package a;\n// end\n"),
        ("import a.B;\n// end", "import a.B;\n// end\n"),
        ("import a.B;\n/* end */", "import a.B;\n/* end */\n"),
        ("import z.Z\n/* c */;\nimport b.C;", "/* c */\nimport b.C;\nimport z.Z;\n"),
        ("import a.B; // trailing", "import a.B; // trailing\n"),
        ("class A {}\n// end", "class A {}\n// end\n"),
        ("module a {}\n// end", "module a {}\n// end\n"),
        ("void main() {}\n// end", "void main() {}\n// end\n"),
    ];
    assert_snapshot!(fmt_with_options(cases[0].0, &Options::default()), @"// only");
    assert_snapshot!(fmt_with_options(cases[6].0, &Options::default()), @"
    import a.B;
    // end
    ");
    assert_snapshot!(fmt_with_options(cases[8].0, &Options::default()), @"
    /* c */
    import b.C;
    import z.Z;
    ");

    for (line_ending, ending) in
        [(LineEnding::Lf, "\n"), (LineEnding::Crlf, "\r\n"), (LineEnding::Cr, "\r")]
    {
        for insert_final_newline in [true, false] {
            for javadoc in [None, Some(JavadocOptions::default())] {
                let options =
                    Options { line_ending, insert_final_newline, javadoc, ..Options::default() };
                for (source, expected) in cases {
                    let expected = if insert_final_newline {
                        expected
                    } else {
                        expected.strip_suffix('\n').unwrap()
                    };
                    assert_eq!(
                        fmt_with_options(source, &options),
                        expected.cow_replace('\n', ending)
                    );
                }
            }
        }
    }
}

#[test]
fn final_empty_javadoc_keeps_the_final_newline_policy() {
    for insert_final_newline in [true, false] {
        let options = Options {
            insert_final_newline,
            javadoc: Some(JavadocOptions::default()),
            ..Options::default()
        };
        let expected = if insert_final_newline { "// only\n/** */\n" } else { "// only\n/** */" };
        assert_eq!(fmt_with_options("// only\n/** */", &options), expected);
    }
}

#[test]
fn top_level_blank_lines_ignore_empty_declarations_as_boundaries() {
    let source = "class A {} class B {}\n\nclass C {} ;\n// leading D\nclass D {} class E {}";
    assert_snapshot!(fmt_with_options(source, &Options::default()), @"
    class A {}

    class B {}

    class C {}
    ;
    // leading D
    class D {}

    class E {}
    ");
}

#[test]
fn consecutive_fields_preserve_source_blank_lines() {
    let source = "class C { int first;\n\n\n\nint second; int third; }";
    assert_snapshot!(fmt_with_options(source, &Options::default()), @"
    class C {
      int first;

      int second;
      int third;
    }
    ");
}

#[test]
fn consecutive_annotated_fields_preserve_source_spacing() {
    let source = r"class C { int first; @A(1) int second; int third; }";
    let options = Options::default();

    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      int first;
      @A(1) int second;
      int third;
    }
    ");
}

#[test]
fn member_blank_lines_cover_every_class_like_body_and_keep_comment_owners() {
    let source = r"class Empty {}
class C {
    @Deprecated int field;
    ; // empty member
    // nested owner
    class Nested {
        int first;
        int second; // trailing owner
        // dangling owner
    }
}
interface I { int FIRST = 1; int SECOND = 2; }
enum E { A, B; int first; int second; }
record R() { static int first; static int second; }
@interface Ann { int first(); int second(); }
";
    let options = Options::default();

    assert_snapshot!(fmt_with_options(source, &options), @"
    class Empty {}

    class C {
      @Deprecated int field;

      ; // empty member

      // nested owner
      class Nested {
        int first;
        int second; // trailing owner
        // dangling owner
      }
    }

    interface I {
      int FIRST = 1;
      int SECOND = 2;
    }

    enum E {
      A,
      B;

      int first;
      int second;
    }

    record R() {
      static int first;
      static int second;
    }

    @interface Ann {
      int first();

      int second();
    }
    ");
}

#[test]
fn member_blank_lines_preserve_dangling_comment_spacing() {
    let source = "class C { int first; // trailing\n\n// leading\nint second;\n\n// dangling\n}";
    let options = Options::default();

    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      int first; // trailing

      // leading
      int second;

      // dangling
    }
    ");
}

#[test]
fn anonymous_class_members_use_the_shared_spacing() {
    let source = "class C { Object value = new Object() { int first; int second; }; }";
    let options = Options::default();

    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      Object value = new Object() {
        int first;
        int second;
      };
    }
    ");
}

#[test]
fn member_blank_lines_preserve_statement_block_spacing() {
    let source = "class C { void run() { first();\n\nsecond(); }\n\nint field; }";
    let options = Options::default();

    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void run() {
        first();

        second();
      }

      int field;
    }
    ");
}

#[test]
fn broken_argument_lists_put_one_entry_per_line() {
    let source = "class A { void f() { call(alphaAlphaAlpha, betaBetaBeta, gammaGammaGamma, \
                  deltaDeltaDelta); } }";
    let options =
        Options { line_width: line_width(52), indent_width: indent_width(2), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        call(
          alphaAlphaAlpha,
          betaBetaBeta,
          gammaGammaGamma,
          deltaDeltaDelta
        );
      }
    }
    ");
}

#[test]
fn list_entries_after_line_comments_keep_the_list_indent() {
    let source = "class A { void f() { call(firstArgument, // group\n \
                  secondArgument, thirdArgument, lastArgument); } }";
    let options = Options { line_width: line_width(40), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        call(
          firstArgument, // group
          secondArgument,
          thirdArgument,
          lastArgument
        );
      }
    }
    ");
}

#[test]
fn list_comment_lines_preserve_configured_contracts() {
    let sources = [
        "class A { void f() { call(firstArgument, // group\n secondArgument, thirdArgument, lastArgument); } }",
        "class A { void f() { call(firstArgument, /* group\n next */ secondArgument, thirdArgument, lastArgument); } }",
        "class A { void f() { call(firstArgument, /* group \n next */ secondArgument, thirdArgument, lastArgument); } }",
        "class A { void f() { call(firstArgument, /* group\n\n next */ secondArgument, thirdArgument, lastArgument); } }",
        "class A { void f() { call(firstArgument, // group  \n // next\n secondArgument, thirdArgument, lastArgument); } }",
        "class A { void f() { call(firstArgument, // group\n b, c, d); } }",
        "class A { void f() { call(firstArgument, secondArgument, thirdArgument, lastArgument); } }",
        "class A { void f() { call(nested(firstArgument, // group\n secondArgument, thirdArgument, lastArgument), end); } }",
        "class A { int[] values = {firstArgument, // group\n secondArgument, thirdArgument, lastArgument}; }",
    ];
    for source in sources {
        for width in [40, 52, 80] {
            for indent in [2, 4] {
                for ending in [LineEnding::Lf, LineEnding::Crlf] {
                    let source = match ending {
                        LineEnding::Crlf => source.cow_replace('\n', "\r\n"),
                        _ => source.into(),
                    };
                    let options = Options {
                        line_width: line_width(width),
                        indent_width: indent_width(indent),
                        line_ending: ending,
                        ..Options::default()
                    };
                    format_stable_with_options(&source, &options);
                }
            }
        }
    }
}

#[test]
fn generic_field_headers_break_before_the_declarator() {
    let source =
        "class A { class B { static final Map<String, Section> NAME_TO_SECTION = make(); } }";
    let options = Options { line_width: line_width(40), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      class B {
        static final Map<String, Section>
          NAME_TO_SECTION = make();
      }
    }
    ");
}

#[test]
fn long_for_headers_break_between_clauses() {
    let source = "class A { void f() { for (int index = firstIndex; index < items.size(); \
                  index += stride) { process(items.get(index)); } } }";
    let options =
        Options { line_width: line_width(50), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        for (
          int index = firstIndex;
          index < items.size();
          index += stride
        ) {
          process(items.get(index));
        }
      }
    }
    ");
}

#[test]
fn fitting_for_headers_keep_their_clauses_inline() {
    let source =
        "class A { void f() { for (int i = 0; i < n; i++) {} for (;;) {} for (i = 0;;) {} } }";
    let options =
        Options { line_width: line_width(50), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        for (int i = 0; i < n; i++) {}
        for (;;) {}
        for (i = 0; ;) {}
      }
    }
    ");
}

#[test]
fn call_initializers_keep_the_empty_for_tail() {
    let source = "class A { void f() { for (Map<String, Section> /* one */\n \
                  /* two */ NAME_TO_SECTION = make(firstArgument, secondArgument); ; ) {} } }";
    let mut options = Options { line_width: line_width(40), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        for (
          Map<String, Section> /* one */
            /* two */ NAME_TO_SECTION =
              make(
                firstArgument,
                secondArgument
              );
          ;
        ) {}
      }
    }
    ");
    for ending in [LineEnding::Lf, LineEnding::Crlf] {
        let source = match ending {
            LineEnding::Crlf => source.cow_replace('\n', "\r\n"),
            _ => source.into(),
        };
        options.line_ending = ending;
        for javadoc in [None, Some(JavadocOptions::default())] {
            options.javadoc = javadoc;
            for insert_final_newline in [false, true] {
                options.insert_final_newline = insert_final_newline;
                format_stable_with_options(&source, &options);
            }
        }
    }
}

/// The block comment ends the first `;`'s line, so it trails that `;`, and it
/// prints against the empty slot's `;`, where a second pass reads it.
#[test]
fn block_comment_ending_an_empty_for_slot_line_prints_against_its_semicolon() {
    let source = "class A { void f() { for (i = 0; /* b */\n; u()) b(); } }";
    assert_snapshot!(fmt_with_options(source, &Options::default()), @"
    class A {
      void f() {
        for (i = 0; /* b */; u()) b();
      }
    }
    ");
}

/// The init's line comment rides past the `;`, so the block comment trailing
/// the `;` starts the next line, where it leads the empty slot's `;`.
#[test]
fn block_comment_after_a_riding_for_init_line_comment_leads_the_empty_slot() {
    let source = "class A { void f() { for (i = 0 // l\n; /* b */\n; u()) b(); } }";
    assert_snapshot!(fmt_with_options(source, &Options::default()), @"
    class A {
      void f() {
        for (
          i = 0; // l
          /* b */;
          u()
        )
          b();
      }
    }
    ");
}

#[test]
fn comments_before_empty_for_slots_format_in_one_pass() {
    for separator in [";", "\\u003b"] {
        for comment in ["/* b */", "/** b */", "// b", "/// b"] {
            for head in [
                format!("i = 0; {comment}\n{separator} u()"),
                format!("i = 0; {comment}\n{separator}"),
                format!("; {comment}\n{separator} u()"),
                format!("i = 0 // l\n{separator} {comment}\n{separator} u()"),
                format!("i = 0 // l\n{separator} /* a */ {comment}\n{separator}"),
                format!("i = 0; c{separator} {comment}\n"),
            ] {
                let source = format!("class A {{ void f() {{ for ({head}) b(); }} }}");
                for width in [40, 100] {
                    for javadoc in [None, Some(JavadocOptions::default())] {
                        let options = Options {
                            line_width: line_width(width),
                            javadoc,
                            ..Options::default()
                        };
                        format_stable_with_options(&source, &options);
                    }
                }
            }
        }
    }
}

#[test]
fn line_comments_after_for_semicolons_keep_their_slots() {
    let source = "class A { void f() { \
        for (i = 0; // b\n; u()) b(); \
        for (; /* a */ // b\n; u()) b(); \
        for (i = 0; c; // b\n) b(); \
        } }";
    assert_snapshot!(fmt_with_options(source, &Options::default()), @"
    class A {
      void f() {
        for (
          i = 0; // b
          ;
          u()
        )
          b();
        for (
          ; /* a */ // b
          ;
          u()
        )
          b();
        for (
          i = 0;
          c; // b
        )
          b();
      }
    }
    ");
}

#[test]
fn comments_inside_parenthesized_for_clauses_stay_inside() {
    for (open, close) in [("(", ")"), ("\\u0028", "\\u0029")] {
        let source = format!(
            "class A {{ void f() {{ \
            for (i = 0; /* before condition */\n\
            {open}/* inside condition */ c{close}; /* before update */\n\
            {open}/* inside update */ o{close}.u()) b(); \
            }} }}"
        );
        insta::allow_duplicates! { assert_snapshot!(fmt_with_options(&source, &Options::default()), @"
        class A {
          void f() {
            for (
              i = 0; /* before condition */
              (/* inside condition */ c); /* before update */
              (/* inside update */ o).u()
            )
              b();
          }
        }
        "); }
    }
}

#[test]
fn empty_for_tail_breaks_preserve_other_header_layouts() {
    let sources = [
        "class A { void f() { for (Map<String, Section> NAME_TO_SECTION = make(firstArgument, secondArgument); ; ) {} } }",
        "class A { void f() { for (value = make(firstArgument, secondArgument); ; ) {} } }",
        "class A { void f() { for (i = 0; /* c */ ; /* u */ ) {} } }",
        "class A { void f() { for (i = 0; /* c */ x; i++) {} } }",
        "class A { void f() { for (i = 0; // condition\n ; // update\n ) {} } }",
        "class A { void f() { for (;test(firstArgument, secondArgument);step(firstArgument, secondArgument)) {} } }",
        "class A { void f() { for (int i = 0; /** */ i < n; i++) {} } }",
        "class A { void f() { for (;;) {} } }",
    ];
    let mut options = Options::default();
    for source in sources {
        for width in [40, 80] {
            options.line_width = line_width(width);
            for indent in [2, 4] {
                options.indent_width = indent_width(indent);
                for javadoc in [None, Some(JavadocOptions::default())] {
                    options.javadoc = javadoc;
                    format_stable_with_options(source, &options);
                }
            }
        }
    }
}

#[test]
fn erased_for_tail_comments_do_not_change_the_break() {
    let sources = [
        "class A { void f() { for (value = make(firstArgument, secondArgument);\n/** */\n;) {} } }",
        "class A { void f() { for (value = make(firstArgument, secondArgument);;\n/** */\n) {} } }",
    ];
    let mut options = Options {
        line_width: line_width(40),
        javadoc: Some(JavadocOptions::default()),
        ..Options::default()
    };
    for source in sources {
        for ending in [LineEnding::Lf, LineEnding::Crlf] {
            options.line_ending = ending;
            let source = match ending {
                LineEnding::Crlf => source.cow_replace('\n', "\r\n"),
                _ => source.into(),
            };
            for insert_final_newline in [false, true] {
                options.insert_final_newline = insert_final_newline;
                format_stable_with_options(&source, &options);
            }
        }
    }
}

#[test]
fn broken_lists_put_the_closer_on_its_own_line() {
    let source = "class A { void f() { call(firstArgument, secondArgument, thirdArgument, \
                  lastArgument); } }";
    let options =
        Options { line_width: line_width(37), indent_width: indent_width(2), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        call(
          firstArgument,
          secondArgument,
          thirdArgument,
          lastArgument
        );
      }
    }
    ");
}

#[test]
fn switch_labels_put_one_entry_per_line() {
    let source = "class C { void f(int value) { switch (value) { \
                  case FIRST_CONSTANT, SECOND_CONSTANT, THIRD_CONSTANT, FOURTH_CONSTANT -> {} \
                  default -> {} } } }";
    let options =
        Options { line_width: line_width(54), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class C {
      void f(int value) {
        switch (value) {
          case FIRST_CONSTANT,
            SECOND_CONSTANT,
            THIRD_CONSTANT,
            FOURTH_CONSTANT -> {}
          default -> {}
        }
      }
    }
    ");
}

#[test]
fn assignment_rhs_breaks_only_when_the_assignment_overflows() {
    let source = "class A { void f() { shortName = shortValue; \
                  extremelyLongTargetName = extremelyLongValueNameThatDoesNotFit; } }";
    let options =
        Options { line_width: line_width(54), indent_width: indent_width(2), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        shortName = shortValue;
        extremelyLongTargetName =
          extremelyLongValueNameThatDoesNotFit;
      }
    }
    ");
}

#[test]
fn wrapped_lines_take_one_level_or_two_inside_a_control_flow_header() {
    let declarations = "class Example { \
        Invoice createInvoice(Customer customer, List<Item> items) { return invoiceFor(customer, items); } \
        Invoice readInvoice(Path path, Charset charset) throws IOException { return parse(path, charset); } \
        public boolean equals(Object object) { return object instanceof Order other && id.equals(other.id) && total.equals(other.total); } \
        class OrderService extends BaseService implements Closeable, Flushable { private final Repository repository; } \
        record OrderLine(Item item, int quantity) implements Comparable<OrderLine> { \
        public int compareTo(OrderLine other) { return item.compareTo(other.item); } } }";
    let statements = "class Example { void run() { \
        while (queue.hasPending() && !shutdown.isRequested()) { process(queue.next()); } \
        if (isEligible(order.customer(), order.region())) { ship(order); } \
        for (int index = firstIndex; index < items.size(); index += stride) { process(items.get(index)); } \
        try (InputStream input = open(source); OutputStream output = create(target)) { input.transferTo(output); } \
        do { poll(); } while (queue.hasPending() && !shutdown.isRequested()); \
        synchronized (registry.lockFor(order).forRegion(region)) { ship(order); } } }";
    // The class and method add 2 and 4 columns to the design's 50-column examples.
    let at = |width| Options {
        line_width: line_width(width),
        indent_width: indent_width(2),
        ..Options::default()
    };

    assert_snapshot!(fmt_with_options(declarations, &at(52)), @"
    class Example {
      Invoice createInvoice(
        Customer customer,
        List<Item> items
      ) {
        return invoiceFor(customer, items);
      }

      Invoice readInvoice(Path path, Charset charset)
          throws IOException {
        return parse(path, charset);
      }

      public boolean equals(Object object) {
        return object instanceof Order other
          && id.equals(other.id)
          && total.equals(other.total);
      }

      class OrderService extends BaseService
          implements Closeable, Flushable {
        private final Repository repository;
      }

      record OrderLine(Item item, int quantity)
          implements Comparable<OrderLine> {
        public int compareTo(OrderLine other) {
          return item.compareTo(other.item);
        }
      }
    }
    ");
    assert_snapshot!(fmt_with_options(statements, &at(54)), @"
    class Example {
      void run() {
        while (queue.hasPending()
            && !shutdown.isRequested()) {
          process(queue.next());
        }
        if (isEligible(
          order.customer(),
          order.region()
        )) {
          ship(order);
        }
        for (
          int index = firstIndex;
          index < items.size();
          index += stride
        ) {
          process(items.get(index));
        }
        try (
          InputStream input = open(source);
          OutputStream output = create(target)
        ) {
          input.transferTo(output);
        }
        do {
          poll();
        } while (queue.hasPending()
            && !shutdown.isRequested());
        synchronized (registry
            .lockFor(order)
            .forRegion(region)) {
          ship(order);
        }
      }
    }
    ");
}

#[test]
fn header_conditions_under_a_prefix_or_cast_wrap_two_levels_deeper() {
    let source = "class A { void f() { \
        if (!(firstConditionFlag && secondConditionFlag)) { a(); } \
        while (!order.customer().region().isEligible()) { a(); } \
        if ((Boolean) context.attributes().get(key)) { a(); } } }";
    let options =
        Options { line_width: line_width(40), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        if (!(firstConditionFlag
            && secondConditionFlag)) {
          a();
        }
        while (!order
            .customer()
            .region()
            .isEligible()) {
          a();
        }
        if ((Boolean) context
            .attributes()
            .get(key)) {
          a();
        }
      }
    }
    ");
}

#[test]
fn header_conditions_that_cannot_break_break_inside_their_parentheses() {
    let source = "class A { void f() { \
        if (aVeryLongConditionVariableNameThatOverflowsXYZ) { a(); } \
        while (anotherVeryLongConditionVariableNameHereXYZ) { a(); } \
        do { a(); } while (anotherVeryLongConditionVariableNameHereXYZ); \
        switch (aVeryLongSelectorVariableNameThatOverflows) { default -> {} } \
        int code = switch (aVeryLongSelectorVariableName) { default -> 1; }; \
        synchronized (aVeryLongLockObjectVariableNameThatOverflow) { a(); } \
        if (/* checked */ aVeryLongConditionVariableNameXY) a(); \
        if (anyMatch(candidates, candidate -> candidate.isEligible())) { a(); } \
        try { a(); } catch (AVeryLongExceptionTypeNameThatOverflows exception) {} } }";
    let options =
        Options { line_width: line_width(52), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        if (
          aVeryLongConditionVariableNameThatOverflowsXYZ
        ) {
          a();
        }
        while (
          anotherVeryLongConditionVariableNameHereXYZ
        ) {
          a();
        }
        do {
          a();
        } while (
          anotherVeryLongConditionVariableNameHereXYZ
        );
        switch (
          aVeryLongSelectorVariableNameThatOverflows
        ) {
          default -> {}
        }
        int code = switch (
          aVeryLongSelectorVariableName
        ) {
          default -> 1;
        };
        synchronized (
          aVeryLongLockObjectVariableNameThatOverflow
        ) {
          a();
        }
        if (
          /* checked */ aVeryLongConditionVariableNameXY
        )
          a();
        if (anyMatch(candidates, candidate ->
          candidate.isEligible()
        )) {
          a();
        }
        try {
          a();
        } catch (AVeryLongExceptionTypeNameThatOverflows
            exception) {}
      }
    }
    ");
}

#[test]
fn enhanced_for_iterable_wraps_two_levels_deeper() {
    let source =
        "class A { void f() { for (String currentValue : allAvailableValuesFromTheRequest) {} } }";
    let options =
        Options { line_width: line_width(54), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        for (String currentValue :
            allAvailableValuesFromTheRequest) {}
      }
    }
    ");
}

#[test]
fn multi_catch_keeps_the_parameter_name_with_the_last_type() {
    let source = "class A { void f() { try {} catch (FirstException | SecondException | ThirdException | LastException exception) {} } }";
    let options =
        Options { line_width: line_width(56), indent_width: indent_width(2), ..Options::default() };
    assert_snapshot!(fmt_with_options(source, &options), @"
    class A {
      void f() {
        try {} catch (FirstException
            | SecondException
            | ThirdException
            | LastException exception) {}
      }
    }
    ");
}

#[test]
fn declaration_clauses_cover_every_declaration_owner() {
    let source = r"@Deprecated sealed class Service<T> extends Base<T> implements pkg./* owner */FirstContract, SecondContract permits ServiceImpl, BackupImpl {
  Service() throws FirstFailure, SecondFailure {}
  <E extends Exception> void run(E value) throws FirstFailure, SecondFailure {}
  class Nested implements FirstContract, SecondContract {}
  void local() { class Local implements FirstContract, SecondContract {} }
}
interface Contract extends FirstContract, SecondContract permits ContractImpl, OtherImpl {}
enum Kind implements FirstContract, SecondContract { A }
record Pair(String left, String right) implements FirstContract, SecondContract {}
";
    let options =
        Options { line_width: line_width(58), indent_width: indent_width(2), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    @Deprecated
    sealed class Service<T> extends Base<T>
        implements pkg./* owner */ FirstContract,
            SecondContract
        permits ServiceImpl, BackupImpl {
      Service() throws FirstFailure, SecondFailure {}

      <E extends Exception> void run(E value)
          throws FirstFailure, SecondFailure {}

      class Nested implements FirstContract, SecondContract {}

      void local() {
        class Local
            implements FirstContract, SecondContract {}
      }
    }

    interface Contract extends FirstContract, SecondContract
        permits ContractImpl, OtherImpl {}

    enum Kind implements FirstContract, SecondContract {
      A,
    }

    record Pair(String left, String right)
        implements FirstContract, SecondContract {}
    ");
}

#[test]
fn wrapped_declaration_clauses_keep_their_entries_comments() {
    // A line comment written after the last clause entry follows the brace, so
    // it does not count toward the header's width, and a block comment between
    // entries stays with them.
    let source = "class Outer { \
                  static class Wide<T extends Comparable<T>> extends Predicate<T> implements Serializable, Cloneable, AutoCloseable // required by the caller\n\
                  { int a; } \
                  static class Narrow extends Base implements Ser // trailing\n\
                  { int b; } \
                  static class Between implements Alpha /* why */, Beta { int c; } }";
    let options = Options::default();

    assert_snapshot!(fmt_with_options(source, &options), @"
    class Outer {
      static class Wide<T extends Comparable<T>> extends Predicate<T>
          implements Serializable, Cloneable, AutoCloseable { // required by the caller
        int a;
      }

      static class Narrow extends Base implements Ser { // trailing
        int b;
      }

      static class Between implements Alpha /* why */, Beta {
        int c;
      }
    }
    ");
}

#[test]
fn declaration_clause_fit_includes_braces_semicolons_and_indivisible_types() {
    let at_boundary = Options { line_width: line_width(25), ..Options::default() };
    let over_boundary = Options { line_width: line_width(24), ..at_boundary.clone() };
    let method_at_boundary = Options { line_width: line_width(22), ..at_boundary.clone() };
    let method_over_boundary = Options { line_width: line_width(21), ..at_boundary.clone() };
    let indivisible = Options { line_width: line_width(20), ..at_boundary.clone() };

    assert_eq!(
        fmt_with_options("class Fit extends Base {}", &at_boundary),
        "class Fit extends Base {}\n"
    );
    assert_eq!(
        fmt_with_options("class Fit extends Base {}", &over_boundary),
        "class Fit\n    extends Base {}\n"
    );
    assert_eq!(
        fmt_with_options("interface I { void fit() throws E; }", &method_at_boundary),
        "interface I {\n  void fit() throws E;\n}\n"
    );
    assert_eq!(
        fmt_with_options("interface I { void fit() throws E; }", &method_over_boundary),
        "interface I {\n  void fit()\n      throws E;\n}\n"
    );
    assert_eq!(
        fmt_with_options(
            "class C extends extremely.qualified.QualifiedTypeNameThatCannotFit {}",
            &indivisible,
        ),
        "class C\n    extends\n        extremely\n          .qualified\n          .QualifiedTypeNameThatCannotFit {}\n"
    );
}

#[test]
fn import_sections_follow_configured_kind_order() {
    let source = "import z.Normal; import module z.module; import static z.Tools.value; class A {}";
    let options = Options {
        import_order: ImportOrder {
            sections: vec![ImportSection::Static, ImportSection::Module, ImportSection::Type],
            groups: vec![ImportGroup { matchers: vec![ImportMatcher::All] }],
        },
        ..Options::default()
    };

    assert_snapshot!(fmt_with_options(source, &options), @"
    import static z.Tools.value;

    import module z.module;

    import z.Normal;

    class A {}
    ");
}

#[test]
fn import_sections_keep_the_default_groups_inside_each_section() {
    let source = "import module java.base; import org.slf4j.Logger; import java.util.List; \
        import static org.junit.jupiter.api.Assertions.assertEquals; \
        import javax.inject.Inject; import com.google.common.collect.ImmutableList; \
        import static java.util.Objects.requireNonNull; import java.util.concurrent.Future;";
    let options = Options {
        import_order: ImportOrder {
            sections: vec![ImportSection::Static, ImportSection::Type, ImportSection::Module],
            ..ImportOrder::default()
        },
        ..Options::default()
    };

    assert_snapshot!(fmt_with_options(source, &options), @"
    import static java.util.Objects.requireNonNull;

    import static org.junit.jupiter.api.Assertions.assertEquals;

    import java.util.List;
    import java.util.concurrent.Future;
    import javax.inject.Inject;

    import com.google.common.collect.ImmutableList;
    import org.slf4j.Logger;

    import module java.base;
    ");
}

/// Format without the shared LF-shaped contract checks, for asserting exact
/// bytes under non-default line endings.
fn format_raw(source: &str, options: &Options) -> String {
    robusta_formatter::format(source, options).unwrap()
}

#[test]
fn the_default_line_ending_is_lf_whatever_the_source_uses() {
    let lf_source = "class A {\n  int x;\n}\n";
    for source in [lf_source, "class A {\r\n  int x;\r\n}\r\n", "class A {\r  int x;\r}\r"] {
        assert_eq!(format_raw(source, &Options::default()), lf_source);
    }
    assert_eq!(format_raw("class A {}", &Options::default()), "class A {}\n");
}

#[test]
fn crlf_line_ending_is_idempotent_on_a_uniform_crlf_file() {
    let source = "class A {\r\nint x;\r\n}\r\n";
    let options = Options { line_ending: LineEnding::Crlf, ..Options::default() };

    let formatted = format_raw(source, &options);
    assert_eq!(formatted, "class A {\r\n  int x;\r\n}\r\n");
    assert_eq!(format_raw(&formatted, &options), formatted);
}

#[test]
fn line_ending_unifies_a_mixed_file() {
    let crlf = Options { line_ending: LineEnding::Crlf, ..Options::default() };
    for source in ["class A {\r\n    int x;\n}\n", "class A {\n    int x;\r\n}\r\n"] {
        assert_eq!(format_raw(source, &Options::default()), "class A {\n  int x;\n}\n");
        assert_eq!(format_raw(source, &crlf), "class A {\r\n  int x;\r\n}\r\n");
    }
}

#[test]
fn a_text_block_interior_takes_the_configured_line_ending() {
    // A text block is the one Java token whose own text spans physical lines,
    // so its interior terminators travel through the IR rather than being
    // re-derived at each break. They must normalize on the way in and be
    // re-emitted as the configured ending, exactly like structural breaks —
    // JLS §3.10.6 normalizes a text block's terminators before escape
    // interpretation, so this never changes the compiled string value.
    let crlf_source = "class A {\r\n  String s = \"\"\"\r\n    hi\r\n    \"\"\";\r\n}\r\n";
    let cr_source = "class A {\r    String s = \"\"\"\r        hi\r        \"\"\";\r}\r";
    let lf_source = "class A {\n  String s = \"\"\"\n    hi\n    \"\"\";\n}\n";

    let lf = Options { line_ending: LineEnding::Lf, ..Options::default() };
    let crlf = Options { line_ending: LineEnding::Crlf, ..Options::default() };

    // Every source ending collapses to LF interior-and-all when LF is forced.
    assert_eq!(format_raw(crlf_source, &lf), lf_source);
    assert_eq!(format_raw(cr_source, &lf), lf_source);
    assert_eq!(format_raw(lf_source, &lf), lf_source);

    // And back out again, so the interior is not merely being left alone.
    assert_eq!(format_raw(lf_source, &crlf), crlf_source);
    assert_eq!(format_raw(cr_source, &crlf), crlf_source);

    // A uniform CRLF file stays byte-identical through the literal.
    assert_eq!(format_raw(crlf_source, &crlf), crlf_source);
}

#[test]
fn forced_line_endings_apply_regardless_of_source_bytes() {
    let lf_source = "class A {\n  int x;\n}\n";
    let crlf_source = "class A {\r\n  int x;\r\n}\r\n";

    let crlf = Options { line_ending: LineEnding::Crlf, ..Options::default() };
    let cr = Options { line_ending: LineEnding::Cr, ..Options::default() };
    let lf = Options { line_ending: LineEnding::Lf, ..Options::default() };

    assert_eq!(format_raw(lf_source, &crlf), crlf_source);
    assert_eq!(format_raw(crlf_source, &lf), lf_source);
    assert_eq!(format_raw(lf_source, &cr), "class A {\r  int x;\r}\r");
}

#[test]
fn text_block_interiors_follow_the_configured_line_ending() {
    // Defect regression: a CRLF source's text-block interior used to keep its
    // source bytes while structural breaks were normalized, so output was
    // neither LF nor CRLF.
    let crlf_source =
        "class T {\r\n  String s = \"\"\"\r\n    alpha\r\n    beta\r\n    \"\"\";\r\n}\r\n";

    let lf_out =
        format_raw(crlf_source, &Options { line_ending: LineEnding::Lf, ..Options::default() });
    assert_eq!(lf_out.bytes().filter(|&b| b == b'\r').count(), 0);

    // Under `crlf` the same file stays byte-identical, interior included.
    let crlf_out =
        format_raw(crlf_source, &Options { line_ending: LineEnding::Crlf, ..Options::default() });
    assert_eq!(crlf_out, crlf_source);
}

#[test]
fn line_ending_choice_changes_terminator_bytes_only() {
    // The JLS (§3.10.6) normalizes text-block terminators to LF before escape
    // interpretation, so terminator-only divergence pins that formatting under
    // `crlf` cannot change any compiled value relative to `lf`.
    let source = "class T {\n    String s = \"\"\"\n        alpha\n        beta\n        \"\"\";\n\n    void f() {\n        g();\n    }\n}\n";

    let lf_out = format_raw(source, &Options { line_ending: LineEnding::Lf, ..Options::default() });
    let crlf_out =
        format_raw(source, &Options { line_ending: LineEnding::Crlf, ..Options::default() });
    let cr_out = format_raw(source, &Options { line_ending: LineEnding::Cr, ..Options::default() });

    assert!(!lf_out.contains('\r'));
    assert_eq!(crlf_out.cow_replace("\r\n", "\n"), lf_out);
    assert_eq!(cr_out.cow_replace('\r', "\n"), lf_out);
}

#[test]
fn fixed_class_member_blank_lines() {
    let source = "class C { int first; int second; C() {} {} static {} void one() {} void two() {} class Nested {} }";
    assert_snapshot!(fmt_with_options(source, &Options::default()), @"
    class C {
      int first;
      int second;

      C() {}

      {}

      static {}

      void one() {}

      void two() {}

      class Nested {}
    }
    ");
}

#[test]
fn fixed_interface_member_blank_lines() {
    assert_snapshot!(fmt_with_options("interface I { void one(); void two(); }", &Options::default()), @"
    interface I {
      void one();

      void two();
    }
    ");
}

#[test]
fn fixed_enum_member_blank_lines() {
    assert_snapshot!(fmt_with_options("enum E { A, B; int first; int second; void one() {} void two() {} }", &Options::default()), @"
    enum E {
      A,
      B;

      int first;
      int second;

      void one() {}

      void two() {}
    }
    ");
}

#[test]
fn fixed_record_member_blank_lines() {
    assert_snapshot!(fmt_with_options("record R(int n) { R {} void one() {} void two() {} }", &Options::default()), @"
    record R(int n) {
      R {}

      void one() {}

      void two() {}
    }
    ");
}

#[test]
fn compact_members_keep_comments_and_final_newline_policy() {
    for insert_final_newline in [true, false] {
        let options = Options { insert_final_newline, ..Options::default() };
        let expected = "void one() {}\n\n// main\nvoid main() {}\n\n// last";
        let expected =
            if insert_final_newline { format!("{expected}\n") } else { expected.to_string() };
        assert_eq!(
            fmt_with_options("void one() {}\n// main\nvoid main() {}\n\n// last\n", &options),
            expected
        );
    }
}

#[test]
fn multi_entry_clauses_keep_the_first_entry_on_the_keyword_line() {
    let options = Options { line_width: line_width(56), ..Options::default() };
    for (head, keyword) in [
        ("final class IncomingRequestHandler", "implements"),
        ("interface IncomingRequestHandler", "extends"),
        ("sealed class IncomingRequestHandler", "permits"),
    ] {
        let two = format!("{head} {keyword} AlphaContract, BetaContract {{}}");
        assert_eq!(
            fmt_with_options(&two, &options),
            format!("{head}\n    {keyword} AlphaContract, BetaContract {{}}\n")
        );
        let three = format!("{head} {keyword} AlphaContract, BetaContract, GammaContract {{}}");
        assert_eq!(
            fmt_with_options(&three, &options),
            format!(
                "{head}\n    {keyword} AlphaContract,\n        BetaContract,\n        GammaContract {{}}\n"
            )
        );
    }
    let source = "class A { void incomingRequestHandler() throws AlphaContract, BetaContract {} }";
    assert_eq!(
        fmt_with_options(source, &options),
        "class A {\n  void incomingRequestHandler()\n      throws AlphaContract, BetaContract {}\n}\n"
    );
    let source = "class A { void incomingRequestHandler() throws AlphaContract, BetaContract, GammaContract {} }";
    assert_eq!(
        fmt_with_options(source, &options),
        "class A {\n  void incomingRequestHandler()\n      throws AlphaContract,\n          BetaContract,\n          GammaContract {}\n}\n"
    );
}

#[test]
fn single_entry_declaration_clauses_keep_the_entry_beside_the_keyword() {
    let options = Options { line_width: line_width(60), ..Options::default() };
    for (head, keyword) in [
        ("final class IncomingRequestHandler", "implements"),
        ("interface IncomingRequestHandler", "extends"),
        ("sealed class IncomingRequestHandler", "permits"),
    ] {
        let source = format!("{head} {keyword} AlphaContractThatCannotFitOnTheHeader {{}}");
        assert_eq!(
            fmt_with_options(&source, &options),
            format!("{head}\n    {keyword} AlphaContractThatCannotFitOnTheHeader {{}}\n")
        );
    }
    let source =
        "class A { void incomingRequestHandler() throws AlphaContractThatCannotFitOnTheHeader {} }";
    assert_eq!(
        fmt_with_options(source, &options),
        "class A {\n  void incomingRequestHandler()\n      throws AlphaContractThatCannotFitOnTheHeader {}\n}\n"
    );
}

#[test]
fn type_declaration_clauses_stay_on_the_line_where_they_fit() {
    let source = "class OrderService extends AbstractOrderServiceSupport implements Closeable {} \
                  class OrderService extends BaseService implements Closeable, Flushable, AutoCloseable, Serializable {} \
                  class OrderService extends AbstractOrderServiceSupport implements Closeable, Flushable, AutoCloseable {} \
                  class OrderService extends Base<RequestContext, ResponseBody> implements Closeable {} \
                  class OrderService<RequestContext, ResponseBody> extends Base implements Closeable {} \
                  class OrderService<RequestContextParameter, ResponseBodyParameter> extends Base implements Closeable {} \
                  sealed class OrderService extends BaseService permits FirstService, SecondService {}";
    let options =
        Options { line_width: line_width(50), indent_width: indent_width(2), ..Options::default() };

    assert_snapshot!(fmt_with_options(source, &options), @"
    class OrderService
        extends AbstractOrderServiceSupport
        implements Closeable {}

    class OrderService extends BaseService
        implements Closeable,
            Flushable,
            AutoCloseable,
            Serializable {}

    class OrderService
        extends AbstractOrderServiceSupport
        implements Closeable,
            Flushable,
            AutoCloseable {}

    class OrderService
        extends Base<RequestContext, ResponseBody>
        implements Closeable {}

    class OrderService<RequestContext, ResponseBody>
        extends Base implements Closeable {}

    class OrderService<
            RequestContextParameter,
            ResponseBodyParameter>
        extends Base implements Closeable {}

    sealed class OrderService extends BaseService
        permits FirstService, SecondService {}
    ");
}
