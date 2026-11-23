// Adapted from https://github.com/oxc-project/oxc/blob/f9ed53b1e82cb35a1118de673be637791d60676d/crates/oxc_macros/src/lib.rs

//! Macros for declaring lint rules.
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod declare_robusta_lint;

/// Declares one lint rule: its stable-ID const, its doc-commented struct, its
/// registry static, and its `DeclaredRule` metadata impl.
///
/// The hand-written half is only the `impl Rule` beneath it. The rule ID must
/// match the rule's file name (with `-` for `_`); the linter's registry test
/// enforces that.
///
/// `node_types` lists the `AstKind` variants the rule's `run` matches on, or
/// `any` for a rule that inspects every node. The optional `settings` names
/// the settings type and its default registry value.
///
/// # Example
///
/// ```rust,ignore
/// declare_robusta_lint! {
///     /// Replaces the visually ambiguous lowercase `l` suffix on integer literals.
///     UppercaseLongSuffix {
///         id: UPPERCASE_LONG_SUFFIX_ID = "uppercase-long-suffix",
///         rule: UPPERCASE_LONG_SUFFIX,
///         summary: "Reports integer literals with the visually ambiguous lowercase `l` long suffix.",
///         category: Style,
///         fix: RuleFixMeta::Fixable(FixKind::SAFE_FIX),
///         node_types: [IntLiteral],
///     }
/// }
/// ```
#[proc_macro]
pub fn declare_robusta_lint(input: TokenStream) -> TokenStream {
    let metadata = parse_macro_input!(input as declare_robusta_lint::LintRuleMeta);
    declare_robusta_lint::declare_robusta_lint(metadata)
}
