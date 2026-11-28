//! Compilation units, packages, modules, and imports.

use cow_utils::CowUtils;
use insta::assert_snapshot;
use robusta_formatter::{JavadocOptions, LineEnding, Options};

use crate::fixtures::{
    check_contract_at_widths_and_modes, format_source as fmt,
    format_source_with_options as fmt_with_options, format_stable, format_stable_with_options,
};

fixture_tests! {
    golden_full_file => "compilation-units/full-file";
    golden_module_imports_file => "compilation-units/module-imports";
    golden_import_order_file => "compilation-units/import-order";
    golden_compact_source_file => "compilation-units/compact-source";
    golden_compact_source_module_words_file => "compilation-units/compact-source-module-words";
    golden_package_annotations_file => "compilation-units/package-annotations";
    golden_module_declaration_file => "compilation-units/module-declaration";
}

#[test]
fn leading_utf8_bom_is_preserved_exactly_once() {
    const UTF8_BOM: &str = "\u{feff}";

    let body = format_stable("class A{}");
    let formatted = format_stable("\u{feff}class A{}");

    assert_eq!(formatted, format!("{UTF8_BOM}{body}"));
    assert_eq!(formatted.strip_prefix(UTF8_BOM), Some(body.as_str()));
    assert!(!body.starts_with(UTF8_BOM));
}

#[test]
fn ignored_final_sub_is_omitted() {
    for sub in ["\x1a", r"\u001a", r"\uuu001A"] {
        for source in ["", "class Z {}", "\u{feff}class Z {}", "// note\n", "/* note */\n"] {
            for line_ending in [LineEnding::Lf, LineEnding::Cr, LineEnding::Crlf] {
                for insert_final_newline in [false, true] {
                    let options =
                        Options { line_ending, insert_final_newline, ..Options::default() };
                    assert_eq!(
                        format_stable_with_options(&format!("{source}{sub}"), &options),
                        format_stable_with_options(source, &options),
                        "{source:?} {sub:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn sub_in_comments_identifiers_and_literals_is_preserved() {
    for sub in ["\x1a", r"\u001a"] {
        for source in [
            format!("// note{sub}"),
            format!("/* note{sub} */"),
            format!("class A{sub} {{ String s = \"{sub}\"; char c = '{sub}'; }}"),
        ] {
            let output = format_stable(&source);
            assert!(output.contains(sub), "{source:?} -> {output:?}");
        }
    }
}

#[test]
fn block_file_header_keeps_blank_line_before_package() {
    assert_snapshot!(fmt("/* header */\n\npackage p;"), @"
    /* header */

    package p;
    ");
}

#[test]
fn line_file_header_keeps_blank_line_before_package() {
    assert_snapshot!(fmt("// header\n\npackage p;"), @"
    // header

    package p;
    ");
}

#[test]
fn file_header_collapses_multiple_blank_lines_before_package() {
    assert_snapshot!(fmt("/* header */\n\n\npackage p;"), @"
    /* header */

    package p;
    ");
}

#[test]
fn file_header_keeps_blank_line_before_first_import() {
    assert_snapshot!(fmt("/* header */\n\nimport a.B;"), @"
    /* header */

    import a.B;
    ");
}

#[test]
fn first_import_comments_keep_their_attachment_or_header_slot() {
    for header in ["", "// header\n\n", "/* header */\n\n", "/* first */\n\n// second\n\n"] {
        for attached in ["", "// attached\n", "/* attached */\n", "/// attached\n"] {
            let imports =
                format!("{header}{attached}import java.util.Map;\nimport java.util.List;\n");
            let sorted =
                format!("{header}import java.util.List;\n{attached}import java.util.Map;\n");
            for newline in ["\n", "\r\n", "\\u000a", "\\u000d\\u000a"] {
                for following in ["", "class X {}\n", "void main() {}\n", "module m {}\n"] {
                    let separator = if following.is_empty() { "" } else { "\n" };
                    assert_eq!(
                        fmt(&format!("{imports}{following}").cow_replace('\n', newline)),
                        format!("{sorted}{separator}{following}")
                    );
                }
                assert_eq!(
                    fmt(&format!("package p;\n{imports}").cow_replace('\n', newline)),
                    format!("package p;\n\n{sorted}")
                );
            }
        }
    }
}

#[test]
fn detached_import_comments_keep_their_slot() {
    for orphan in ["// orphan\n\n", "/* orphan */\n\n", "/* first */\n\n// second\n\n"] {
        for attached in ["", "// attached\n", "/* attached */\n"] {
            let source = format!("import z.Z;\n{orphan}{attached}import a.A;\nimport b.B;\n");
            let expected = format!("{attached}import a.A;\n{orphan}import b.B;\nimport z.Z;\n");
            for newline in ["\n", "\r\n", "\\u000a", "\\u000d\\u000a"] {
                for following in ["", "class X {}\n", "void main() {}\n", "module m {}\n"] {
                    let separator = if following.is_empty() { "" } else { "\n" };
                    assert_eq!(
                        fmt(&format!("{source}{following}").cow_replace('\n', newline)),
                        format!("{expected}{separator}{following}")
                    );
                }
                assert_eq!(
                    fmt(&format!("package p;\n{source}").cow_replace('\n', newline)),
                    format!("package p;\n\n{expected}")
                );
            }
        }
    }
}

#[test]
fn detached_multiline_import_comments_keep_their_slot() {
    let source = "import z.Z;\n/* orphan\n * text */\n\n/* attached\n * text */\n\
                  import a.A;\nimport b.B;\n";
    let expected = "/* attached\n * text */\nimport a.A;\n/* orphan\n * text */\n\n\
                    import b.B;\nimport z.Z;\n";
    for newline in ["\n", "\r\n"] {
        assert_eq!(fmt(&source.cow_replace('\n', newline)), expected);
    }
}

#[test]
fn detached_import_comments_keep_their_slot_beside_an_empty_javadoc() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    for (gap, expected) in [
        ("/** */\n\n", "import a.A;\n// orphan\n/** */\n\nimport b.B;\nimport z.Z;\n"),
        ("/**\n */\n\n", "import a.A;\n// orphan\n/**\n */\n\nimport b.B;\nimport z.Z;\n"),
        ("\n/** */\n", "/** */\nimport a.A;\n// orphan\n\nimport b.B;\nimport z.Z;\n"),
        ("\n/**\n */\n", "/**\n */\nimport a.A;\n// orphan\n\nimport b.B;\nimport z.Z;\n"),
    ] {
        let source = format!("import z.Z;\n// orphan\n{gap}import a.A;\nimport b.B;\n");
        assert_eq!(fmt_with_options(&source, &options), expected);
    }
}

#[test]
fn detached_import_comments_keep_their_slot_beside_terminator_comments() {
    for (declaration, expected) in [
        (
            "import z.Z\n/* below */\n;",
            "import a.A;\n/* below */\n// orphan\n\nimport b.B;\nimport z.Z;\n",
        ),
        (
            "import z.Z // first\n; /* second */",
            "import a.A;\n// orphan\n\nimport b.B;\nimport z.Z // first\n; /* second */\n",
        ),
    ] {
        let source = format!("{declaration}\n// orphan\n\nimport a.A;\nimport b.B;\n");
        assert_eq!(fmt(&source), expected);
    }
}

#[test]
fn detached_import_comments_cross_group_and_section_boundaries() {
    use robusta_formatter::{ImportGroup, ImportMatcher, ImportOrder, ImportSection};

    let options = Options {
        import_order: ImportOrder {
            sections: vec![ImportSection::Static, ImportSection::Type, ImportSection::Module],
            groups: vec![
                ImportGroup { matchers: vec![ImportMatcher::All] },
                ImportGroup { matchers: vec![ImportMatcher::Prefix("java".into())] },
            ],
        },
        ..Options::default()
    };
    let source = "import java.util.Map;\n// one\n\nimport module java.base;\n/* two */\n\n\
                  import a.A;\nimport static a.A.value;\n";
    let expected = "import static a.A.value;\n\n// one\n\nimport a.A;\n\n/* two */\n\n\
                    import java.util.Map;\n\nimport module java.base;\n";
    assert_eq!(fmt_with_options(source, &options), expected);
}

#[test]
fn file_header_keeps_blank_line_before_first_type() {
    assert_snapshot!(fmt("/* header */\n\nclass A {}"), @"
    /* header */

    class A {}
    ");
}

#[test]
fn file_header_keeps_blank_line_before_module() {
    assert_snapshot!(fmt("/* header */\n\nmodule m {}"), @"
    /* header */

    module m {}
    ");
}

#[test]
fn file_header_keeps_blank_line_before_compact_source() {
    assert_snapshot!(fmt("/* header */\n\nvoid main() {}"), @"
    /* header */

    void main() {}
    ");
}

#[test]
fn adjacent_file_header_and_package_stay_adjacent() {
    assert_snapshot!(fmt("/* header */\npackage p;"), @"
    /* header */
    package p;
    ");
}

#[test]
fn package_and_import_keep_their_existing_blank_line() {
    assert_snapshot!(fmt("package p;\n\nimport a.B;"), @"
    package p;

    import a.B;
    ");
}

#[test]
fn line_comment_after_package_annotation_stays_on_its_line() {
    assert_snapshot!(fmt("/** Package docs. */\n@Deprecated // kept for old callers\npackage p;"), @"
    /** Package docs. */
    @Deprecated // kept for old callers
    package p;
    ");
}

#[test]
fn comments_between_package_annotations_keep_their_lines() {
    assert_snapshot!(fmt("@A /* first */ @B\n// before the package\npackage p;"), @"
    @A
    /* first */ @B
    // before the package
    package p;
    ");
}

#[test]
fn comments_after_package_and_import_keywords_lead_the_name() {
    let source = "package /* p */ p;\n\
                  import /* a */ static /* b */ a.B.c;\n\
                  import /* c */ a.B;\n\
                  import // d\n    c.D;\n\
                  import module /* e */ java.base;";
    assert_snapshot!(fmt(source), @"
    package /* p */ p;

    import module /* e */ java.base;

    import /* c */ a.B;
    import // d
    c.D;

    import static /* a */ /* b */ a.B.c;
    ");
}

#[test]
fn comments_before_an_import_modifier_lead_the_name() {
    for (source, expected) in [
        ("import /* c */ static a.B;", "import static /* c */ a.B;\n"),
        ("import // c\nstatic a.B;", "import static // c\na.B;\n"),
        ("import\n// c\nstatic a.B;", "import static\n// c\na.B;\n"),
        ("import\n/* c */\nstatic a.B;", "import static\n/* c */\na.B;\n"),
        ("import\n/* c */ static a.B;", "import static\n/* c */ a.B;\n"),
        ("import /* c */\nstatic a.*;", "import static /* c */\na.*;\n"),
        ("import /* c */ module a;", "import module /* c */ a;\n"),
        ("import // c\nmodule a;", "import module // c\na;\n"),
        ("import\n\n// c\nmodule a;", "import module\n// c\na;\n"),
        ("import /* c */ module.A;", "import /* c */ module.A;\n"),
    ] {
        assert_eq!(fmt(source), expected, "{source}");
        check_contract_at_widths_and_modes(source);
    }
}

#[test]
fn comments_after_module_keywords_stay_in_place() {
    let source = "@A open module /* b */ m { requires /* c */ transitive q; \
                  exports /* d */ p; uses /* e */ S; provides /* f */ S with T; }";
    assert_snapshot!(fmt(source), @"
    @A
    open module /* b */ m {
      requires /* c */ transitive q;
      exports /* d */ p;
      uses /* e */ S;
      provides /* f */ S with T;
    }
    ");
}

#[test]
fn comments_after_open_lead_the_module_name() {
    for (source, expected) in [
        ("open /* a */ module /* b */ m {}", "open module /* a */ /* b */ m {}\n"),
        ("open // a\nmodule m {}", "open module // a\nm {}\n"),
        ("open\n// a\nmodule m {}", "open module\n// a\nm {}\n"),
        ("open\n/// a\n\nmodule m {}", "open module\n/// a\n\nm {}\n"),
    ] {
        assert_eq!(fmt(source), expected);
        check_contract_at_widths_and_modes(source);
    }
}

#[test]
fn comments_around_requires_modifiers_stay_in_place() {
    for (source, expected) in [
        (
            "requires /* a */ transitive /* b */ static /* c */ q;",
            "requires /* a */ transitive /* b */ static /* c */ q;",
        ),
        (
            "requires // a\ntransitive // b\nstatic // c\nq;",
            "requires // a\n  transitive // b\n  static // c\n  q;",
        ),
        (
            "requires\n// a\ntransitive\n// b\nstatic\n// c\nq;",
            "requires\n  // a\n  transitive\n  // b\n  static\n  // c\n  q;",
        ),
        ("requires\n// a\n\nstatic q;", "requires\n  // a\n\n  static q;"),
    ] {
        let source = format!("module m {{ {source} }}");
        assert_eq!(fmt(&source), format!("module m {{\n  {expected}\n}}\n"));
        check_contract_at_widths_and_modes(&source);
    }
}

#[test]
fn comments_after_a_module_name_that_end_its_line_follow_the_brace() {
    for head in ["module m", "open module m"] {
        for (run, empty, directive) in [
            (" /* c */ ", " /* c */ {}", " /* c */ {\n  requires a;\n}"),
            (" /* a */ /* c */ ", " /* a */ /* c */ {}", " /* a */ /* c */ {\n  requires a;\n}"),
            (" // c\n", " { // c\n}", " { // c\n  requires a;\n}"),
            (" /* a */ // c\n", " /* a */ { // c\n}", " /* a */ { // c\n  requires a;\n}"),
            ("\n// c\n", " {\n  // c\n}", " {\n  // c\n  requires a;\n}"),
        ] {
            for (body, formatted) in [("{}", empty), ("{ requires a; }", directive)] {
                let source = format!("{head}{run}{body}");
                assert_eq!(fmt(&source), format!("{head}{formatted}\n"));
                check_contract_at_widths_and_modes(&source);
            }
        }
    }
}

#[test]
fn repeated_import_comments_stay_with_their_copy() {
    let cases = [
        ("import a.B;\n// x\nimport a.B;\n", "import a.B;\n// x\nimport a.B;\n"),
        ("import a.B; // t\nimport a.B; // u\n", "import a.B; // t\nimport a.B; // u\n"),
        ("import a.B;\nimport /* b */ a.B;\n", "import a.B;\nimport /* b */ a.B;\n"),
        (
            "import a.B;\n// x\nimport a.B;\nclass X {}\n",
            "import a.B;\n// x\nimport a.B;\n\nclass X {}\n",
        ),
        (
            "import a.B; // t\nimport a.B; // u\nclass X {}\n",
            "import a.B; // t\nimport a.B; // u\n\nclass X {}\n",
        ),
        (
            "import a.B;\n// x\nimport a.B;\nmodule m {}\n",
            "import a.B;\n// x\nimport a.B;\n\nmodule m {}\n",
        ),
        (
            "import a.B;\n// x\nimport a.B;\nvoid main() {}\n",
            "import a.B;\n// x\nimport a.B;\n\nvoid main() {}\n",
        ),
        (
            "import java.util.List;\n// x\nimport java.util.List;\nimport a.B;\n",
            "import java.util.List;\n// x\nimport java.util.List;\n\nimport a.B;\n",
        ),
        (
            "import a.B; // t\nimport a.B; // u\nimport static a.C.d;\n",
            "import a.B; // t\nimport a.B; // u\n\nimport static a.C.d;\n",
        ),
        ("import a.B;\nimport a. /* c */ B;\n", "import a.B;\nimport a./* c */ B;\n"),
        ("import a.b.*;\nimport a.b. /* c */ *;\n", "import a.b.*;\nimport a.b./* c */ *;\n"),
        (
            "import a. /* x */ B;\nimport a. /* y */ B;\n",
            "import a./* x */ B;\nimport a./* y */ B;\n",
        ),
    ];
    for (source, expected) in cases {
        assert_eq!(fmt(source), expected);
    }
}

#[test]
fn repeated_imports_keep_every_kind_in_source_order() {
    let source = "import java.util.List; // first\n\
                  import java.util.*;\n\
                  import java.util.List; // second\n\
                  import java.util.*;\n\
                  import module java.base; // first\n\
                  import module java.base; // second\n\
                  import static java.util.Collections.emptyList; // first\n\
                  import static java.util.Collections.*;\n\
                  import static java.util.Collections.emptyList; // second\n\
                  import static java.util.Collections.*;\n";
    assert_snapshot!(fmt(source), @"
    import module java.base; // first
    import module java.base; // second

    import java.util.*;
    import java.util.*;
    import java.util.List; // first
    import java.util.List; // second

    import static java.util.Collections.*;
    import static java.util.Collections.*;
    import static java.util.Collections.emptyList; // first
    import static java.util.Collections.emptyList; // second
    ");
}

#[test]
fn comments_inside_import_names_move_with_their_import() {
    let cases = [
        ("import b.C;\nimport a. /* c */ B;\n", "import a./* c */ B;\nimport b.C;\n"),
        ("import b.C;\nimport a /* c */ .B;\n", "import a /* c */.B;\nimport b.C;\n"),
        ("import b.C;\nimport a.b. /* c */ *;\n", "import a.b./* c */ *;\nimport b.C;\n"),
        ("import b.C;\nimport a.b /* c */ .*;\n", "import a.b /* c */.*;\nimport b.C;\n"),
        (
            "import static b.C.d;\nimport static a. /* c */ B.e;\n",
            "import static a./* c */ B.e;\nimport static b.C.d;\n",
        ),
        (
            "import static b.C.d;\nimport static a.B. /* c */ *;\n",
            "import static a.B./* c */ *;\nimport static b.C.d;\n",
        ),
        (
            "import b.C;\nimport a. /* c */ B;\nclass X {}\n",
            "import a./* c */ B;\nimport b.C;\n\nclass X {}\n",
        ),
        ("import a. /* c */ B;\nimport b.C;\n", "import a./* c */ B;\nimport b.C;\n"),
        (
            "import b.C;\nimport a. /* c */ B /* t */;\n",
            "import a./* c */ B; /* t */\nimport b.C;\n",
        ),
        (
            "import b.C;\nimport a.b. // c\n    * /* t */;\n",
            "import a.b // c\n.*; /* t */\nimport b.C;\n",
        ),
    ];
    for (source, expected) in cases {
        assert_eq!(fmt(source), expected);
    }
}

fn assert_formats_with_each_newline(cases: &[(&str, &str)]) {
    for (source, expected) in cases {
        for newline in ["\n", "\r\n", "\\u000a", "\\u000d\\u000a"] {
            assert_eq!(fmt(&source.cow_replace('\n', newline)), *expected);
        }
    }
}

#[test]
fn comments_around_an_import_terminator_move_with_their_import() {
    let cases = [
        ("import b.C;\nimport a.B /* t */; // u\n", "import a.B; /* t */ // u\nimport b.C;\n"),
        ("import b.C;\nimport a.B; /* t */ // u\n", "import a.B; /* t */ // u\nimport b.C;\n"),
        ("import a.B; /* t */ // u\nimport b.C;\n", "import a.B; /* t */ // u\nimport b.C;\n"),
        ("import a.B; /* t */ // u\n", "import a.B; /* t */ // u\n"),
        ("import a.B; /* t */ // u\nclass X {}\n", "import a.B; /* t */ // u\n\nclass X {}\n"),
        (
            "import b.C;\nimport a.B\n/* c */;\nimport c.D;\n",
            "import a.B;\nimport b.C;\n/* c */\nimport c.D;\n",
        ),
        ("import a.B\n/* c */;\n", "import a.B;\n/* c */\n"),
        ("import b.C;\nimport a.B // c\n; /* d */\n", "import a.B // c\n; /* d */\nimport b.C;\n"),
        ("import b.B; /* c */\nimport a.A;\n", "import a.A;\nimport b.B; /* c */\n"),
        (
            "import b.C;\nimport a.B;\n// c\nimport c.D;\n",
            "import a.B;\nimport b.C;\n// c\nimport c.D;\n",
        ),
        ("import a.B;\nimport a.B\n// d\n; /* e */\n", "import a.B;\nimport a.B;\n// d\n/* e */\n"),
    ];
    assert_formats_with_each_newline(&cases);
}

#[test]
fn a_comment_after_an_import_with_code_after_it_on_its_line_leads_that_code() {
    let cases = [
        ("import b.B; /* c */ import a.A;\n", "/* c */ import a.A;\nimport b.B;\n"),
        ("import z.Y; /* c */ class Z {}\n", "import z.Y;\n\n/* c */ class Z {}\n"),
    ];
    assert_formats_with_each_newline(&cases);
}

#[test]
fn import_comments_move_with_their_import_unless_they_document() {
    let source = "import d.D; /** stray */
import c.C; // with C
// above B
import b.B; /* next */ import a.A;

class Q {}
";
    assert_snapshot!(format_stable(source), @"
    /* next */ import a.A;
    // above B
    import b.B;
    /** stray */
    import c.C; // with C
    import d.D;

    class Q {}
    ");
}

#[test]
fn a_javadoc_comment_after_the_last_import_leads_the_declaration() {
    let javadoc = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    for declaration in ["class Q {}", "void main() {}", "module m {}"] {
        // Each case: the comments, what still trails `import a.A;`, and what
        // leads the declaration.
        let cases = [
            ("/** doc */ ", "", "/** doc */ "),
            ("/** doc */\n", "", "/** doc */\n"),
            ("/** */\n", "", "/** */\n"),
            ("/* c */ /** doc */ ", "", "/* c */ /** doc */ "),
            ("/* c */ /** doc */\n", "", "/* c */ /** doc */\n"),
            ("/* c */ ", "", "/* c */ "),
            ("/* c */\n", " /* c */", ""),
        ];
        for (comments, trailing, leading) in cases {
            let source = format!("import b.B;\nimport a.A; {comments}{declaration}\n");
            let expected =
                format!("import a.A;{trailing}\nimport b.B;\n\n{leading}{declaration}\n");
            assert_eq!(fmt(&source), expected, "{source:?}");
            assert_eq!(fmt_with_options(&source, &javadoc), expected, "{source:?}");
        }
    }
    let source = "import b.B;\nimport a.A; /// doc\nclass Q {}\n";
    let expected = "import a.A;\nimport b.B;\n\n/// doc\nclass Q {}\n";
    assert_eq!(fmt(source), expected);
    assert_eq!(fmt_with_options(source, &javadoc), expected);
    // A line break inside a later comment does not end the line.
    assert_eq!(
        fmt("import b.B;\nimport a.A; /** doc */ /* c\n */ class Q {}\n"),
        "import a.A;\nimport b.B;\n\n/** doc */ /* c\n            */ class Q {}\n"
    );
}

#[test]
fn a_javadoc_comment_after_an_import_leads_what_follows_it() {
    let cases = [
        (
            "import b.B; /** doc */\nimport a.A;\nclass Q {}\n",
            "/** doc */\nimport a.A;\nimport b.B;\n\nclass Q {}\n",
        ),
        (
            "import b.B; /** */\nimport a.A;\nclass Q {}\n",
            "/** */\nimport a.A;\nimport b.B;\n\nclass Q {}\n",
        ),
        (
            "import b.B; /// doc\nimport a.A;\nclass Q {}\n",
            "/// doc\nimport a.A;\nimport b.B;\n\nclass Q {}\n",
        ),
        (
            "import z.Z; /** doc */ import a.A; class Q {}\n",
            "/** doc */ import a.A;\nimport z.Z;\n\nclass Q {}\n",
        ),
        (
            "import b.B; /* c */ /** doc */ /* e */\nimport a.A;\n",
            "/* c */ /** doc */ /* e */\nimport a.A;\nimport b.B;\n",
        ),
        (
            "import b.B; /** one */ /** two */\nimport a.A;\n",
            "/** one */ /** two */\nimport a.A;\nimport b.B;\n",
        ),
        (
            "// lead\nimport b.B; /** doc */\nimport a.A;\n",
            "/** doc */\nimport a.A;\n// lead\nimport b.B;\n",
        ),
        // Nothing follows the last import, so the comment stays after the imports.
        ("import b.B;\nimport a.A; /** doc */\n", "import a.A;\nimport b.B; /** doc */\n"),
        ("import b.B; /** doc */ import a.A; // t\n", "/** doc */ import a.A; // t\nimport b.B;\n"),
        (
            "import z.Z // c\n; /// doc\nimport b.B;\nclass Q {}\n",
            "/// doc\nimport b.B;\nimport z.Z; // c\n\nclass Q {}\n",
        ),
        (
            "import y.Y /* note */; /** doc */ // tail\nimport b.B;\nclass Q {}\n",
            "/** doc */ // tail\nimport b.B;\nimport y.Y; /* note */\n\nclass Q {}\n",
        ),
    ];
    assert_formats_with_each_newline(&cases);
    let javadoc = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    for (source, expected) in cases {
        assert_eq!(fmt_with_options(source, &javadoc), expected, "{source:?}");
    }
}

#[test]
fn import_trailing_comments_continue_past_a_multiline_block() {
    assert_eq!(
        fmt("import z.Z; /* multi\n text */ // tail\n\nimport a.A;\n"),
        "import a.A;\nimport z.Z; /* multi\n text */ // tail\n"
    );
    for newline in ["\n", "\r\n", "\\u000a", "\\u000d\\u000a"] {
        for tail in ["// tail", "/* tail */"] {
            let source = format!(
                "import z.Z; /* multi\n text */ {tail}\n// orphan\n\n// attached\nimport a.A;\nimport b.B;\n"
            );
            crate::fixtures::check_contract_at_widths_and_modes(&source.cow_replace('\n', newline));
        }
    }
}

#[test]
fn a_semicolon_line_after_a_deferred_comment_is_not_a_blank_line() {
    let cases = [
        ("import a.A\n// c\n;\n", "import a.A;\n// c\n"),
        ("import a.A\n// c\n;\n\n", "import a.A;\n// c\n"),
        ("import a.A\n// c\n\n;\n", "import a.A;\n// c\n"),
        ("import a.A\n// c\n;\nimport b.B;\n", "import a.A;\n// c\nimport b.B;\n"),
        ("package p\n// c\n;\nclass X {}\n", "package p;\n\n// c\nclass X {}\n"),
        ("package p\n// c\n;\n", "package p;\n// c\n"),
        ("package p\n// c\n\n;\n", "package p;\n// c\n"),
    ];
    assert_formats_with_each_newline(&cases);
}

#[test]
fn own_line_import_terminator_comments_lead_the_next_source_import() {
    let imports = "import z.Z\n/* c */;\nimport b.C;\n";
    let sorted = "/* c */\nimport b.C;\nimport z.Z;\n";
    for following in ["", "class X {}\n", "void main() {}\n", "module m {}\n"] {
        let separator = if following.is_empty() { "" } else { "\n" };
        assert_eq!(
            fmt(&format!("{imports}{following}")),
            format!("{sorted}{separator}{following}")
        );
    }
}

#[test]
fn javadoc_between_imports_stays_as_written() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    assert_snapshot!(fmt_with_options("import a.B;\n/// d\nimport a.B;\nclass X {}\n", &options), @"
    import a.B;
    /// d
    import a.B;

    class X {}
    ");
    assert_snapshot!(fmt_with_options("import a.B;\nimport a.B; /**u*/\n", &options), @"
    import a.B;
    import a.B; /**u*/
    ");
    assert_snapshot!(fmt_with_options("import a.A // p\n; /**q*/\nimport a.B;\n", &options), @"
    import a.A; // p
    /**q*/
    import a.B;
    ");
}

#[test]
fn import_comment_runs_keep_markdown_hard_break_spaces() {
    let source = "import a.A;\n/// first  \n/// second\nimport a.B;\n";
    for javadoc in [None, Some(JavadocOptions::default())] {
        assert_eq!(fmt_with_options(source, &Options { javadoc, ..Options::default() }), source);
    }
}

#[test]
fn escaped_package_directive_keywords_keep_their_kind() {
    let source = r"module m { \u0065xports p; \u006fpens q; }";
    assert_eq!(fmt(source), "module m {\n  exports p;\n  opens q;\n}\n");
}

#[test]
fn module_directive_comments_keep_their_boundaries() {
    let options = Options { javadoc: Some(JavadocOptions::default()), ..Options::default() };
    for source in [
        "module m { // first\nrequires /* modifier */ transitive /* name */ a; // end\nuses S; // tail\n}",
        "module m { requires\n// modifier\nstatic\n// name\na;\n// next\nexports p to q, r; }",
        "module m { requires static // modifier\ntransitive // name\na; }",
        "module m { exports p /* to */ to /* target */ q, /* next */ r; opens s to t; }",
        "module m { provides S /* with */ with /* provider */ T, /* next */ U; }",
        "module m { /** doc */ requires a; /** next */ uses S; /** dangling */ }",
    ] {
        format_stable(source);
        format_stable_with_options(source, &options);
    }
}

#[test]
fn comments_before_a_module_directive_keyword_keep_their_side() {
    for (gap, expected) in [
        (" /* c */ ", " /* c */ to b;"),
        (" /* c */\n", " /* c */ to b;"),
        (" // c\n", " // c\n  to b;"),
        ("\n// c\n", "\n  // c\n  to b;"),
        ("\n/* c */\n", "\n  /* c */\n  to b;"),
        ("\n/* c */ ", "\n  /* c */ to b;"),
        ("\n/* a\n * b */\n", "\n  /* a\n   * b */\n  to b;"),
        (" // a\n/* b */\n", " // a\n  /* b */\n  to b;"),
        ("\n\n// c\n", "\n  // c\n  to b;"),
    ] {
        let source = format!("module m {{ exports a{gap}to b; }}");
        assert_eq!(fmt(&source), format!("module m {{\n  exports a{expected}\n}}\n"), "{source}");
        check_contract_at_widths_and_modes(&source);
        let source = format!("module m {{ opens a{gap}to b, d; provides a.B{gap}with c.D; }}");
        check_contract_at_widths_and_modes(&source);
    }
}

#[test]
fn file_tail_comments_use_the_compilation_unit_envelope() {
    for insert_final_newline in [true, false] {
        for javadoc in [None, Some(JavadocOptions::default())] {
            let options = Options { insert_final_newline, javadoc, ..Options::default() };
            for source in [
                "@A /* annotation */ package /* name */ p.q; // tail\n",
                "/** header */\n\nvoid main() {} // trailing\n",
                "import b.B;\nimport a.A; // tail\n",
                "import b.B;\nimport a.A;\n// tail\n",
                "module m {} // tail\n",
            ] {
                for newline in ["\n", "\r\n"] {
                    format_stable_with_options(&source.cow_replace('\n', newline), &options);
                }
            }
        }
    }
}
