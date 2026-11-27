use robusta_ast::AstKind;
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;
use robusta_span::Span;

use crate::utils::strip_suffix;
use crate::{FixKind, LintContext, Rule, RuleFixMeta};

declare_robusta_lint! {
    /// Replaces the visually ambiguous lowercase `l` suffix on integer literals.
    UppercaseLongSuffix {
        id: UPPERCASE_LONG_SUFFIX_ID,
        rule: UPPERCASE_LONG_SUFFIX,
        summary: "Reports integer literals with the visually ambiguous lowercase `l` long suffix.",
        category: Style,
        fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
        node_types: [IntLiteral],
    }
}

impl Rule for UppercaseLongSuffix {
    fn run(&self, node: &robusta_semantic::AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let AstKind::IntLiteral(literal) = node.kind() else {
            return;
        };
        let Some(digits) = strip_suffix(literal.raw, "l") else {
            return;
        };

        #[expect(clippy::cast_possible_truncation, reason = "parsed source spans fit u32")]
        let suffix = Span::new(literal.span.start + digits.len() as u32, literal.span.end);
        context.diagnostic_with_fix(
            RobustaDiagnostic::warning("use uppercase `L` for the long literal suffix", suffix)
                .with_help("Replace the lowercase suffix with `L`."),
            |fixer| fixer.replace(suffix, "L"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tester::Tester;

    #[test]
    fn reports_and_fixes_lowercase_suffixes_in_every_integer_base() {
        let source = "class Numbers {\n    long[] values = {10, 10l, 0xffl, 0b10l, 077l};\n}\n";

        let report = Tester::new(&UPPERCASE_LONG_SUFFIX).fixes(
            source,
            "class Numbers {\n    long[] values = {10, 10L, 0xffL, 0b10L, 077L};\n}\n",
        );

        insta::assert_snapshot!(report, @r#"
        2:28: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("l")
        2:35: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("l")
        2:42: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("l")
        2:48: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("l")
        "#);
    }

    #[test]
    fn reports_and_fixes_unicode_escaped_suffixes() {
        let source = r"class A { long[] values = { 10\u006c, 0xFF\uuuu006C, 0b10\u006c, 077\u006c, 1\u0030l, 1\u0030\u006c }; }";

        let report = Tester::new(&UPPERCASE_LONG_SUFFIX).fixes(
            source,
            r"class A { long[] values = { 10L, 0xFFL, 0b10L, 077L, 1\u0030L, 1\u0030L }; }",
        );

        insta::assert_snapshot!(report, @r#"
        1:31: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("\\u006c")
        1:43: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("\\uuuu006C")
        1:58: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("\\u006c")
        1:69: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("\\u006c")
        1:84: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("l")
        1:94: warning[uppercase-long-suffix]: use uppercase `L` for the long literal suffix ("\\u006c")
        "#);
    }

    #[test]
    fn ignores_uppercase_and_non_integer_literal_suffixes() {
        Tester::new(&UPPERCASE_LONG_SUFFIX).passes(
            r#"class A {
            long value = 10L;
            float low = 10f;
            long escaped = 10\u004c;
            int hex = 0x\u0061;
            String text = "10l";
            String escapedText = "10\u006c";
            char letter = 'l';
            char escapedLetter = '\u006c';
        }"#,
        );
    }
}
