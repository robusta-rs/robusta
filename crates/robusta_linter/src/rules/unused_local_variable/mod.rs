use robusta_ast::AstKind;
use robusta_diagnostics::RobustaDiagnostic;
use robusta_macros::declare_robusta_lint;
use robusta_semantic::{AstNode, SymbolKind};

use crate::{FixKind, LintContext, Rule, RuleFixMeta};

mod fixers;
mod options;
#[cfg(test)]
mod tests;
mod usage;
mod value;

use options::UnusedLocalVariableConfig;
use usage::Local;

declare_robusta_lint! {
    /// Reports never-read locals and removes their declarations and writes.
    ///
    /// A deleted statement that directly follows a documentation comment
    /// leaves `;`, which keeps the comment off the next declaration, where a
    /// `@deprecated` tag would mark it deprecated.
    UnusedLocalVariable {
        id: UNUSED_LOCAL_VARIABLE_ID = "unused-local-variable",
        rule: UNUSED_LOCAL_VARIABLE,
        summary: "Reports local variables whose value is never read.",
        category: Correctness,
        fix: RuleFixMeta::Fixable(FixKind::ALL),
        node_types: [VariableDeclarator, TypePattern],
        settings: UnusedLocalVariableConfig = UnusedLocalVariableConfig::DEFAULT,
    }
}

impl Rule for UnusedLocalVariable {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        let settings = if value.is_null() {
            UnusedLocalVariableConfig::DEFAULT
        } else {
            serde_json::from_value(value)?
        };
        Ok(Self { settings })
    }

    fn to_configuration(&self) -> Option<serde_json::Value> {
        self.settings.ignore_pattern.as_ref()?;
        Some(serde_json::to_value(&self.settings).expect("rule settings should serialize"))
    }

    fn run(&self, node: &AstNode<'_>, context: &mut LintContext<'_, '_, '_>) {
        let semantic = context.semantic();
        let Some((symbol_id, symbol)) =
            semantic.scoping().symbols_for_declaration(node.id()).next()
        else {
            return;
        };
        if symbol.name() == "_"
            || self
                .settings
                .ignore_pattern
                .as_ref()
                .is_some_and(|pattern| pattern.is_match(symbol.name()))
            || !matches!(symbol.kind(), SymbolKind::LocalVariable | SymbolKind::PatternVariable)
        {
            return;
        }
        if symbol.kind() == SymbolKind::PatternVariable
            && !matches!(context.nodes().parent_kind(node.id()), AstKind::InstanceofExpression(_))
        {
            return;
        }
        let local = Local { semantic, symbol_id, effect_cache: std::cell::Cell::new(None) };
        if local.is_read() {
            return;
        }
        let diagnostic = RobustaDiagnostic::warning(
            format!("local variable `{}` is never read", symbol.name()),
            symbol.name_span(),
        )
        .with_help("Remove the unused binding and its writes.");
        let (kind, fixes) = local.fix(node.kind(), context.source_text());
        context.diagnostic_with_fix_of_kind(diagnostic, kind, |fixer| fixer.noop().extend(fixes));
    }
}
