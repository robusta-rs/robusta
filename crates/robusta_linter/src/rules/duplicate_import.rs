use robusta_ast::{AstKind, ImportDeclaration, ImportKind};
use robusta_diagnostics::{RobustaDiagnostic, primary_labeled_span, unlabeled_span};
use robusta_macros::declare_robusta_lint;
use robusta_semantic::AstNode;
use robusta_span::ContentEq;
use rustc_hash::FxHashMap;

use crate::utils::{delete_import, normalize_deletions};
use crate::{FixKind, LintContext, Rule, RuleFixMeta};

declare_robusta_lint! {
    /// Reports an import repeated later in the file, on its first copy, and
    /// offers a safe fix that deletes the repeats.
    ///
    /// JLS §7.5 ignores a repeated import whatever the class path, so on-demand
    /// and module imports are reported too. Two imports from one package are
    /// not repeats, because Java cannot combine them into one.
    DuplicateImport {
        id: DUPLICATE_IMPORT_ID,
        rule: DUPLICATE_IMPORT,
        summary: "Reports imports repeated later in the file.",
        category: Correctness,
        fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
        node_types: [CompilationUnit],
    }
}

impl Rule for DuplicateImport {
    fn run(&self, node: &AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let AstKind::CompilationUnit(unit) = node.kind() else {
            return;
        };
        // Equal imports end in the same simple name, so only imports that share
        // one are compared.
        let mut by_last_name = FxHashMap::<&str, Vec<(&ImportDeclaration<'_>, Vec<_>)>>::default();
        for import in unit.imports() {
            let firsts = by_last_name.entry(import.name.last().name).or_default();
            if let Some((_, repeats)) =
                firsts.iter_mut().find(|(first, _)| first.content_eq(import))
            {
                repeats.push(import);
            } else {
                firsts.push((import, Vec::new()));
            }
        }
        let mut repeated = by_last_name
            .into_values()
            .flatten()
            .filter(|(_, repeats)| !repeats.is_empty())
            .collect::<Vec<_>>();
        repeated.sort_unstable_by_key(|(first, _)| first.span.start);
        for (first, repeats) in repeated {
            let diagnostic = RobustaDiagnostic::warning(
                format!("`{}` is imported more than once", display_name(first)),
                first.span,
            )
            .with_labels(
                std::iter::once(primary_labeled_span(first.span, "first imported here"))
                    .chain(repeats.iter().map(|repeat| unlabeled_span(repeat.span))),
            )
            .with_help("Remove the repeated imports.");
            let mut fixes = repeats
                .iter()
                .map(|repeat| delete_import(context, repeat.span))
                .collect::<Option<Vec<_>>>()
                .unwrap_or_default();
            // Repeats on one line delete the whitespace between them twice.
            normalize_deletions(context.source_text(), &mut fixes);
            context.diagnostic_with_fix(diagnostic, |_| fixes);
        }
    }
}

/// The import as Java writes it after `import`, with escapes decoded.
fn display_name(import: &ImportDeclaration<'_>) -> String {
    let name = import.name.dotted();
    match import.kind {
        ImportKind::SingleType => name,
        ImportKind::TypeOnDemand => format!("{name}.*"),
        ImportKind::SingleStatic => format!("static {name}"),
        ImportKind::StaticOnDemand => format!("static {name}.*"),
        ImportKind::SingleModule => format!("module {name}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::UNUSED_IMPORT;
    use crate::tester::{Tester, lint_together};
    use crate::{LintDiagnostic, apply_safe_fixes};

    #[test]
    fn deletes_repeats_on_the_first_copy() {
        let report = Tester::new(&DUPLICATE_IMPORT).fixes(
            "import java.util.List;\nimport java.util.List;\nimport java.util.function.*;\n\
             import java.util.function.*;\n\n\
             class Names {\n    List<String> all;\n    Predicate<String> filter;\n}\n",
            "import java.util.List;\nimport java.util.function.*;\n\n\
             class Names {\n    List<String> all;\n    Predicate<String> filter;\n}\n",
        );
        insta::assert_snapshot!(report, @r#"
        1:1: warning[duplicate-import]: `java.util.List` is imported more than once ("import java.util.List;")
        3:1: warning[duplicate-import]: `java.util.function.*` is imported more than once ("import java.util.function.*;")
        "#);
    }

    #[test]
    fn reports_each_import_kind_once() {
        for import in [
            "import module java.base;",
            "import static java.lang.Math.max;",
            "import static java.lang.Math.*;",
            "import java.util.List;",
            "import java.util.*;",
        ] {
            let source = format!("{import}\n{import}\n{import}\nclass A {{}}\n");
            let report =
                Tester::new(&DUPLICATE_IMPORT).fixes(&source, &format!("{import}\nclass A {{}}\n"));
            assert_eq!(report.lines().count(), 1, "{report}");
        }
    }

    #[test]
    fn treats_escaped_spellings_as_the_same_import() {
        Tester::new(&DUPLICATE_IMPORT).fixes(
            "import java.util.List;\nimport java.util.L\\u0069st;\nclass A {}\n",
            "import java.util.List;\nclass A {}\n",
        );
    }

    #[test]
    fn accepts_different_imports() {
        Tester::new(&DUPLICATE_IMPORT).passes(
            "import java.util.List;\nimport java.util.Map;\nimport java.util.*;\n\
             import static java.util.List.of;\nimport module java.base;\nclass A {}\n",
        );
    }

    #[test]
    fn reports_one_simple_name_from_each_package_apart() {
        let report = Tester::new(&DUPLICATE_IMPORT).fixes(
            "import b.Map;\nimport a.List;\nimport b.List;\nimport b.Map;\n\
             import b.List;\nimport a.List;\nclass A {}\n",
            "import b.Map;\nimport a.List;\nimport b.List;\nclass A {}\n",
        );
        insta::assert_snapshot!(report, @r#"
        1:1: warning[duplicate-import]: `b.Map` is imported more than once ("import b.Map;")
        2:1: warning[duplicate-import]: `a.List` is imported more than once ("import a.List;")
        3:1: warning[duplicate-import]: `b.List` is imported more than once ("import b.List;")
        "#);
    }

    #[test]
    fn keeps_one_name_imported_by_different_kinds_apart() {
        for source in [
            "import java.util.List;\nimport static java.util.List;\nclass A {}\n",
            "import java.util.*;\nimport static java.util.*;\nclass A {}\n",
            "import java.base;\nimport module java.base;\nclass A {}\n",
            "import java.util.Map;\nimport java.util.Map.*;\nclass A {}\n",
        ] {
            Tester::new(&DUPLICATE_IMPORT).passes(source);
        }
    }

    #[test]
    fn deletes_only_a_repeat_that_shares_its_line() {
        let tester = Tester::new(&DUPLICATE_IMPORT);
        tester.fixes(
            "import java.util.List;\nimport java.util.List; // keep\nclass A {}\n",
            "import java.util.List;\n// keep\nclass A {}\n",
        );
        tester.fixes(
            "import java.util.List;\n  import java.util.List;  import java.util.List;\nclass A {}\n",
            "import java.util.List;\nclass A {}\n",
        );
    }

    #[test]
    fn reports_without_a_fix_when_a_repeat_follows_a_documentation_comment() {
        // Deleting the repeats would give the comment to the class.
        for repeats in ["", "import java.util.List;\n"] {
            Tester::new(&DUPLICATE_IMPORT).reports_without_fix(&format!(
                "import java.util.List;\n/** @deprecated Old */\nimport java.util.List;\n\
                 {repeats}class A {{ List<String> all; }}\n"
            ));
        }
    }

    /// Lint `source` with this rule and `unused-import`.
    fn lint_with_unused_import(source: &str) -> Vec<LintDiagnostic> {
        lint_together(&[&DUPLICATE_IMPORT, &UNUSED_IMPORT], source)
    }

    #[test]
    fn leaves_no_copy_of_an_unused_repeated_import_with_unused_import() {
        let source = "import java.util.List;\nimport java.util.Map;\nimport java.util.List;\n\
                      import java.util.Map;\nclass A { List<String> all; }\n";
        let diagnostics = lint_with_unused_import(source);
        let count = |rule| diagnostics.iter().filter(|d| d.rule_id().as_str() == rule).count();
        assert_eq!((count("duplicate-import"), count("unused-import")), (2, 2));
        let fixed = apply_safe_fixes(source, &diagnostics).unwrap();
        assert_eq!(fixed.code, "import java.util.List;\nclass A { List<String> all; }\n");
    }

    #[test]
    fn keeps_a_documentation_comment_off_the_class_with_unused_import() {
        // Each deletion alone would leave the comment before an import.
        let source = "import java.util.List;\n/** @deprecated Old */\nimport java.util.Map;\n\
                      import java.util.List;\nclass A { List<String> all; }\n";
        let fixed = apply_safe_fixes(source, &lint_with_unused_import(source)).unwrap();
        assert_eq!(
            fixed.code,
            "import java.util.List;\n/** @deprecated Old */\nimport java.util.Map;\n\
             class A { List<String> all; }\n"
        );
    }
}
