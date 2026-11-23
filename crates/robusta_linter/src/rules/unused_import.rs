use robusta_ast::{AstKind, ImportDeclaration, ImportKind};
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;
use robusta_semantic::{AstNode, ReferenceKind, Symbol, SymbolKind};
use robusta_span::ContentEq;

use crate::utils::delete_import_line;
use crate::{FixKind, LintContext, Rule, RuleFixMeta};

declare_robusta_lint! {
    /// Reports a single type or single static import whose imported simple name
    /// is referenced nowhere in the compilation unit and offers a safe deletion.
    ///
    /// A name used only by a Javadoc reference (`{@link C}`, `@see`, `@throws`,
    /// …) counts as used, so its import is neither reported nor removed. On-demand
    /// and module imports need class-path or module-graph resolution to decide,
    /// so this file-local rule deliberately leaves them alone.
    UnusedImport {
        id: UNUSED_IMPORT_ID = "unused-import",
        rule: UNUSED_IMPORT,
        summary: "Reports single imports whose imported name is never referenced.",
        category: Correctness,
        fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
        node_types: [ImportDeclaration],
    }
}

impl Rule for UnusedImport {
    fn run(&self, node: &AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        // The import symbol is declared on its own `ImportDeclaration` node, so a
        // rule visit reasons about exactly this one import.
        let AstKind::ImportDeclaration(import) = node.kind() else {
            return;
        };
        // On-demand and module imports depend on class-path/module resolution.
        if !matches!(import.kind, ImportKind::SingleType | ImportKind::SingleStatic) {
            return;
        }

        let scoping = context.scoping();
        // The name borrows AST storage (`'ast`), not the scoping borrow, so
        // collect it and the source span before reporting mutably.
        let unused = scoping
            .symbols_for_declaration(node.id())
            .find(|(_, symbol)| matches!(symbol.kind(), SymbolKind::Import(_)))
            .filter(|(symbol_id, _)| !scoping.symbol_is_referenced(*symbol_id))
            .filter(|(_, symbol)| !shares_import_reference(context, import, symbol))
            .map(|(_, symbol)| (symbol.name(), symbol.name_span()));

        let Some((name, name_span)) = unused else {
            return;
        };

        // A Javadoc reference is a real use; deleting the import would break the
        // documentation reference, so the deletion would not be meaning-preserving.
        if context.javadocs().structures().any(|structure| structure.mentions(name)) {
            return;
        }

        let message = format!("imported name `{name}` is never used");
        let diagnostic =
            RobustaDiagnostic::warning(message, name_span).with_help("Remove the unused import.");
        let fix = delete_import_line(context, import.span);
        context.diagnostic_with_fix(diagnostic, |_| fix);
    }
}

fn shares_import_reference(
    context: &LintContext<'_, '_, '_>,
    import: &ImportDeclaration<'_>,
    symbol: &Symbol<'_>,
) -> bool {
    let scoping = context.scoping();
    let mut imports = scoping.symbols_for_name(symbol.scope_id(), symbol.ident()).filter_map(
        |(symbol_id, symbol)| match context.nodes().kind(symbol.declaration_node_id()) {
            AstKind::ImportDeclaration(other) => Some((symbol_id, other)),
            _ => None,
        },
    );
    // A repeat of a used import is a `duplicate-import` finding. Static imports
    // can supply overloads, fields, or types (JLS §7.5.3), so without a class
    // path, retain every distinct import that could supply a use.
    imports.any(|(symbol_id, other)| {
        if import.content_eq(other) {
            scoping.symbol_is_referenced(symbol_id)
        } else {
            scoping.references_for_symbol(symbol_id).any(|reference| {
                import.kind == ImportKind::SingleStatic || reference.kind() != ReferenceKind::Method
            })
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tester::Tester;

    #[test]
    fn escaped_imports_and_uses_share_the_same_name() {
        for source in [
            r"import java.ut\u0069l.L\u0069st; class A { List field; }",
            r"import java.util.List; class A { L\u0069st field; }",
            r"import static java.lang.Math.m\u0061x; class A { int f() { return max(1, 2); } }",
        ] {
            Tester::new(&UNUSED_IMPORT).passes(source);
        }
    }

    #[test]
    fn reports_unused_type_and_static_imports() {
        let report = Tester::new(&UNUSED_IMPORT).reports(
            "import java.util.List; import static java.util.Collections.emptyList; class A {}",
        );
        insta::assert_snapshot!(report, @r#"
        1:18: warning[unused-import]: imported name `List` is never used ("List")
        1:60: warning[unused-import]: imported name `emptyList` is never used ("emptyList")
        "#);
    }

    #[test]
    fn deletes_an_unused_single_type_import() {
        Tester::new(&UNUSED_IMPORT).fixes(
            "import java.util.List;
import java.util.Map;

class Names {
    List<String> all;
}
",
            "import java.util.List;

class Names {
    List<String> all;
}
",
        );
    }

    #[test]
    fn accepts_referenced_imports_and_conservative_import_forms() {
        Tester::new(&UNUSED_IMPORT).passes(
            "import java.util.List; import static java.util.Collections.emptyList; \
             import java.time.*; import static java.lang.Math.*; \
             class A { List<String> values = emptyList(); }",
        );
    }

    #[test]
    fn keeps_static_imports_that_share_a_used_name() {
        for (first, second) in [("ByteBuffer", "CharBuffer"), ("CharBuffer", "ByteBuffer")] {
            let source = format!(
                "import static java.nio.{first}.wrap;\n\
                 import static java.nio.{second}.wrap;\n\
                 class A {{ int f() {{ return wrap(new byte[] {{1}}).get() \
                 + wrap(new char[] {{'a'}}).get(); }} }}\n"
            );
            Tester::new(&UNUSED_IMPORT).passes(&source);
        }
    }

    #[test]
    fn keeps_type_and_static_imports_in_their_possible_namespaces() {
        for imports in [
            "import java.util.List;\nimport static example.Names.List;\n",
            "import static example.Names.List;\nimport java.util.List;\n",
        ] {
            for body in [
                "class A { List<String> values; Object f() { return List(); } }\n",
                "class A { List<String> values; }\n",
            ] {
                Tester::new(&UNUSED_IMPORT).passes(&format!("{imports}{body}"));
            }
            Tester::new(&UNUSED_IMPORT).fixes(
                &format!("{imports}class A {{ Object f() {{ return List(); }} }}\n"),
                "import static example.Names.List;\nclass A { Object f() { return List(); } }\n",
            );
        }
    }

    #[test]
    fn removes_shared_static_names_without_an_import_use() {
        for body in [
            "class A {}\n",
            "class A { int wrap; int f() { return wrap; } }\n",
            "class A { void wrap() {} void f() { wrap(); } }\n",
            "class A { Object f() { return java.nio.ByteBuffer.wrap(new byte[] {}); } }\n",
        ] {
            let source = format!(
                "import static java.nio.ByteBuffer.wrap;\n\
                 import static java.nio.CharBuffer.wrap;\n{body}"
            );
            let tester = Tester::new(&UNUSED_IMPORT);
            assert_eq!(tester.reports(&source).matches("is never used").count(), 2);
            tester.fixes(&source, body);
        }
    }

    #[test]
    fn leaves_repeats_of_used_imports_to_duplicate_import() {
        let body = "class A { Object f() { return wrap(new byte[] {}); } }\n";
        for second in ["", "import static java.nio.CharBuffer.wrap;\n"] {
            Tester::new(&UNUSED_IMPORT).passes(&format!(
                "import static java.nio.ByteBuffer.wrap;\n{second}\
                 import static java.nio.ByteBuffer.wrap;\n{body}"
            ));
        }
        Tester::new(&UNUSED_IMPORT)
            .passes("import java.util.List;\nimport java.util.List;\nclass A { List<?> l; }\n");
    }

    #[test]
    fn reports_every_copy_of_an_unused_repeated_import() {
        Tester::new(&UNUSED_IMPORT)
            .fixes("import java.util.Map;\nimport java.util.Map;\nclass A {}\n", "class A {}\n");
    }

    #[test]
    fn receiver_parameter_types_and_annotations_count_as_import_uses() {
        Tester::new(&UNUSED_IMPORT).passes(
            "import example.Owner; import example.Receiver; \
             class A { class Inner { Inner(@Receiver Owner Owner.this) {} } }",
        );
    }

    #[test]
    fn uses_before_a_local_type_of_the_same_name_count_as_import_uses() {
        Tester::new(&UNUSED_IMPORT).passes(
            "import java.util.ArrayList;\n\
             class A { void m() { ArrayList<String> a = new ArrayList<>(); class ArrayList {} } }\n",
        );
        Tester::new(&UNUSED_IMPORT).fixes(
            "import java.util.ArrayList;\nclass A { void m() { class ArrayList {} new ArrayList(); } }\n",
            "class A { void m() { class ArrayList {} new ArrayList(); } }\n",
        );
    }

    #[test]
    fn header_uses_beside_a_member_type_of_the_same_name_count_as_import_uses() {
        for source in [
            "import java.util.List;\nabstract class A implements List<String> { class List {} }\n",
            "import java.io.Serializable;\ninterface A extends Serializable { class Serializable {} }\n",
            "import java.io.Serializable;\nrecord A() implements Serializable { class Serializable {} }\n",
            "import java.io.Serializable;\nenum A implements Serializable { X; class Serializable {} }\n",
        ] {
            Tester::new(&UNUSED_IMPORT).passes(source);
        }
        // A record's members are in scope in its header.
        Tester::new(&UNUSED_IMPORT).fixes(
            "import java.util.List;\nrecord A(List value) { class List {} }\n",
            "record A(List value) { class List {} }\n",
        );
    }

    #[test]
    fn a_pattern_variable_out_of_scope_does_not_hide_an_import_use() {
        Tester::new(&UNUSED_IMPORT).passes(
            "import static p.Q.s; class G { void f(Object o, boolean c) { \
             if (!(o instanceof String s)) { while (c) {} } System.out.println(s); } }",
        );
    }

    #[test]
    fn type_parameters_do_not_count_as_import_uses() {
        let report =
            Tester::new(&UNUSED_IMPORT).reports("import example.T; class A<T> { T value; }");
        assert!(report.contains("imported name `T` is never used"));
    }

    #[test]
    fn honors_line_suppressions() {
        Tester::new(&UNUSED_IMPORT)
            .passes("import java.util.List; // robusta-disable-line\nclass A {}");
    }

    // --- Javadoc references keep the import ------------------------------

    #[test]
    fn javadoc_reference_keeps_the_import() {
        for source in [
            "import java.util.List;\n/** A {@link List} of things. */\nclass A {}",
            "import java.util.List;\n/** A List in prose. */\nclass A {}",
            "import java.util.List;\n/** @see List */\nclass A {}",
            "import java.io.IOException;\n/** @throws IOException on failure */\nclass A {}",
        ] {
            Tester::new(&UNUSED_IMPORT).passes(source);
        }
    }

    #[test]
    fn javadoc_preserves_written_and_cooked_ignorable_import_names() {
        for spelling in ["Li\u{200c}st", r"Li\u200Cst"] {
            for reference in [spelling, "List", "Li\u{200c}st", "Lis\u{200c}t"] {
                let source = format!(
                    "import java.util.{spelling};\n/** {{@link {reference}}} */\nclass A {{}}"
                );
                Tester::new(&UNUSED_IMPORT).passes(&source);
            }
        }
    }

    #[test]
    fn escaped_javadoc_reference_keeps_the_import() {
        for documentation in [
            r"/** {@link M\u0061p} */",
            r"/** {@link M\uuu0061p} */",
            r"/** @see M\u0061p */",
            r"/// [M\u0061p]",
            r"/// @see M\u0061p",
            r"\u002f** {@link Map} */",
            r"/** {@link Map} *\u002f",
            r"\u002f// [Map]",
            "/// See\n\\u002f// [Map]",
        ] {
            Tester::new(&UNUSED_IMPORT)
                .passes(&format!("import java.util.Map;\n{documentation}\nclass A {{}}"));
        }
    }

    #[test]
    fn escaped_javadoc_text_naming_another_name_is_not_a_use() {
        // `Maps` is another name, and `\\u` begins no escape.
        for documentation in
            [r"/** {@link M\u0061ps} */", r"/** {@link \\u004dap} */", r"\u002f** {@link Maps} */"]
        {
            Tester::new(&UNUSED_IMPORT).fixes(
                &format!("import java.util.Map;\n{documentation}\nclass A {{}}\n"),
                &format!("{documentation}\nclass A {{}}\n"),
            );
        }
    }

    #[test]
    fn markdown_javadoc_reference_keeps_the_import() {
        for source in [
            "import java.util.List;\n/// A [List] of things.\nclass A {}",
            "import java.util.List;\n/// A document split across lines.\n/// @see List\nclass A {}",
            "import java.io.IOException;\n/// @throws IOException on failure\nclass A {}",
            "import java.util.List;\n//// A [List] of things.\nclass A {}",
        ] {
            Tester::new(&UNUSED_IMPORT).passes(source);
        }
    }

    #[test]
    fn separate_javadocs_keep_multiple_imports() {
        Tester::new(&UNUSED_IMPORT).passes(
            "import java.util.List;\nimport java.util.Map;\n\
             /** A List value. */\nclass A {}\n/** @see Map */\nclass B {}",
        );
    }

    #[test]
    fn javadoc_substring_does_not_count_as_a_use() {
        // `Lists` must not mask an unused `List` import.
        let report = Tester::new(&UNUSED_IMPORT)
            .reports("import java.util.List;\n/** See Lists elsewhere. */\nclass A {}");
        assert!(report.contains("imported name `List` is never used"));
    }

    #[test]
    fn non_javadoc_block_comment_does_not_count_as_a_use() {
        // A plain `/* ... */` comment is not a documentation comment.
        let report = Tester::new(&UNUSED_IMPORT)
            .reports("import java.util.List;\n/* uses List */\nclass A {}");
        assert!(report.contains("imported name `List` is never used"));
    }

    #[test]
    fn line_comment_does_not_count_as_a_use() {
        let report =
            Tester::new(&UNUSED_IMPORT).reports("import java.util.List;\n// uses List\nclass A {}");
        assert!(report.contains("imported name `List` is never used"));
    }

    // --- Safe deletion --------------------------------------------------

    #[test]
    fn safe_deletes_a_single_unused_import_line() {
        Tester::new(&UNUSED_IMPORT).fixes(
            "import java.util.List;\nimport java.util.Map;\nclass A { Map<?, ?> m; }\n",
            "import java.util.Map;\nclass A { Map<?, ?> m; }\n",
        );
    }

    #[test]
    fn safe_deletes_an_unused_static_import_line() {
        Tester::new(&UNUSED_IMPORT)
            .fixes("import static java.util.Collections.emptyList;\nclass A {}\n", "class A {}\n");
    }

    #[test]
    fn safe_deletes_multiple_unused_imports_independently() {
        Tester::new(&UNUSED_IMPORT).fixes(
            "import java.util.List;\nimport java.util.Map;\nimport java.util.Set;\n\
             class A { Map<?, ?> m; }\n",
            "import java.util.Map;\nclass A { Map<?, ?> m; }\n",
        );
    }

    #[test]
    fn safe_preserves_indentation_of_surrounding_lines() {
        Tester::new(&UNUSED_IMPORT).fixes(
            "import java.util.List;\n    import java.util.Map;\nclass A { Map<?, ?> m; }\n",
            "    import java.util.Map;\nclass A { Map<?, ?> m; }\n",
        );
    }

    /// A form feed is Java horizontal whitespace (JLS §3.6), so it indents an
    /// import exactly as a space or tab does and must not cost the import its
    /// fix.
    #[test]
    fn safe_deletes_an_import_indented_with_a_form_feed() {
        Tester::new(&UNUSED_IMPORT).fixes(
            "import java.util.List;\n\u{000c}import java.util.Map;\nclass A { List<String> l; }\n",
            "import java.util.List;\nclass A { List<String> l; }\n",
        );
    }

    #[test]
    fn safe_preserves_crlf_line_endings() {
        Tester::new(&UNUSED_IMPORT).fixes(
            "import java.util.List;\r\nimport java.util.Map;\r\nclass A { Map<?, ?> m; }\r\n",
            "import java.util.Map;\r\nclass A { Map<?, ?> m; }\r\n",
        );
    }

    #[test]
    fn safe_deletes_a_final_import_without_a_trailing_newline() {
        Tester::new(&UNUSED_IMPORT).fixes("import java.util.List;", "");
    }

    // --- No fix: shapes without one obvious single edit -----------------

    #[test]
    fn import_sharing_a_line_reports_without_a_fix() {
        // Two imports on one physical line: deleting one cleanly is not obvious.
        Tester::new(&UNUSED_IMPORT)
            .reports_without_fix("import java.util.List; import java.util.Map; class A { Map m; }");
    }

    #[test]
    fn import_with_a_trailing_comment_reports_without_a_fix() {
        Tester::new(&UNUSED_IMPORT)
            .reports_without_fix("import java.util.List; // keep\nclass A {}\n");
    }

    #[test]
    fn import_carrying_an_interior_comment_reports_without_a_fix() {
        // The whitespace guards say nothing about a comment written inside the
        // declaration; the comment table does.
        Tester::new(&UNUSED_IMPORT)
            .reports_without_fix("import /* keep */ java.util.List;\nclass A {}\n");
    }

    #[test]
    fn import_after_a_documentation_comment_reports_without_a_fix() {
        // Deleting it would give the comment to the next declaration.
        let tester = Tester::new(&UNUSED_IMPORT);
        tester.reports_without_fix("/** @deprecated Old */\nimport java.util.Map;\nclass A {}\n");
        tester.reports_without_fix("/// Old.\n// Note.\nimport java.util.Map;\nclass A {}\n");
        tester.fixes_only(
            FixKind::SAFE_FIX,
            "import java.util.List;\n/** @deprecated Old */\nimport java.util.Map;\n\
             import java.util.Set;\nclass A {}\n",
            "/** @deprecated Old */\nimport java.util.Map;\nclass A {}\n",
        );
        tester.fixes(
            "/* Note. */\nimport java.util.Map;\n/** Documentation. */\nclass A {}\n",
            "/* Note. */\n/** Documentation. */\nclass A {}\n",
        );
    }

    #[test]
    fn on_demand_and_module_imports_are_left_alone() {
        Tester::new(&UNUSED_IMPORT).passes(
            "import java.util.*;\nimport static java.lang.Math.*;\nimport module java.base;\n\
             class A {}\n",
        );
    }

    #[test]
    fn safe_fix_is_not_applied_below_the_safe_tier() {
        // Nothing lower than safe exists, but the deletion must never be gated up.
        Tester::new(&UNUSED_IMPORT).fixes_to_fixpoint(
            FixKind::SAFE_FIX,
            "import java.util.List;\nimport java.util.Map;\nclass A { Map<?, ?> m; }\n",
            "import java.util.Map;\nclass A { Map<?, ?> m; }\n",
        );
    }
}
