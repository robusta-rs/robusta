mod broken_null_check;
mod duplicate_import;
mod modifier_order;
mod prefer_expression_lambda;
mod redundant_lambda_parameter_parentheses;
mod redundant_return;
mod redundant_semicolons;
mod redundant_super_call;
mod unused_import;
mod unused_local_variable;
mod uppercase_long_suffix;

pub use broken_null_check::{BROKEN_NULL_CHECK, BROKEN_NULL_CHECK_ID, BrokenNullCheck};
pub use duplicate_import::{DUPLICATE_IMPORT, DUPLICATE_IMPORT_ID, DuplicateImport};
pub use modifier_order::{MODIFIER_ORDER, MODIFIER_ORDER_ID, ModifierOrder};
pub use prefer_expression_lambda::{
    PREFER_EXPRESSION_LAMBDA, PREFER_EXPRESSION_LAMBDA_ID, PreferExpressionLambda,
};
pub use redundant_lambda_parameter_parentheses::{
    REDUNDANT_LAMBDA_PARAMETER_PARENTHESES, REDUNDANT_LAMBDA_PARAMETER_PARENTHESES_ID,
    RedundantLambdaParameterParentheses,
};
pub use redundant_return::{REDUNDANT_RETURN, REDUNDANT_RETURN_ID, RedundantReturn};
pub use redundant_semicolons::{
    REDUNDANT_SEMICOLONS, REDUNDANT_SEMICOLONS_ID, RedundantSemicolons,
};
pub use redundant_super_call::{REDUNDANT_SUPER_CALL, REDUNDANT_SUPER_CALL_ID, RedundantSuperCall};
pub use unused_import::{UNUSED_IMPORT, UNUSED_IMPORT_ID, UnusedImport};
pub use unused_local_variable::{
    UNUSED_LOCAL_VARIABLE, UNUSED_LOCAL_VARIABLE_ID, UnusedLocalVariable,
};
pub use uppercase_long_suffix::{
    UPPERCASE_LONG_SUFFIX, UPPERCASE_LONG_SUFFIX_ID, UppercaseLongSuffix,
};

use crate::Rule;

/// All built-in rules in stable registry order.
pub fn builtin_rules() -> &'static [&'static dyn Rule] {
    static RULES: [&dyn Rule; 11] = [
        &REDUNDANT_SEMICOLONS,
        &UPPERCASE_LONG_SUFFIX,
        &MODIFIER_ORDER,
        &UNUSED_IMPORT,
        &UNUSED_LOCAL_VARIABLE,
        &PREFER_EXPRESSION_LAMBDA,
        &REDUNDANT_LAMBDA_PARAMETER_PARENTHESES,
        &REDUNDANT_RETURN,
        &REDUNDANT_SUPER_CALL,
        &BROKEN_NULL_CHECK,
        &DUPLICATE_IMPORT,
    ];
    &RULES
}

pub fn find_builtin_rule(id: crate::RuleId) -> Option<&'static dyn Rule> {
    builtin_rules().iter().copied().find(|rule| rule.id() == id)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn builtin_rules_are_addressable_by_stable_id() {
        assert_eq!(builtin_rules().len(), 11);
        assert_eq!(
            find_builtin_rule(REDUNDANT_SEMICOLONS_ID).map(Rule::id),
            Some(REDUNDANT_SEMICOLONS_ID)
        );
        assert_eq!(
            find_builtin_rule(UPPERCASE_LONG_SUFFIX_ID).map(Rule::id),
            Some(UPPERCASE_LONG_SUFFIX_ID)
        );
        assert_eq!(find_builtin_rule(MODIFIER_ORDER_ID).map(Rule::id), Some(MODIFIER_ORDER_ID));
        assert_eq!(find_builtin_rule(UNUSED_IMPORT_ID).map(Rule::id), Some(UNUSED_IMPORT_ID));
        assert_eq!(
            find_builtin_rule(UNUSED_LOCAL_VARIABLE_ID).map(Rule::id),
            Some(UNUSED_LOCAL_VARIABLE_ID)
        );
        assert_eq!(
            find_builtin_rule(PREFER_EXPRESSION_LAMBDA_ID).map(Rule::id),
            Some(PREFER_EXPRESSION_LAMBDA_ID)
        );
        assert_eq!(
            find_builtin_rule(REDUNDANT_LAMBDA_PARAMETER_PARENTHESES_ID).map(Rule::id),
            Some(REDUNDANT_LAMBDA_PARAMETER_PARENTHESES_ID)
        );
        assert_eq!(find_builtin_rule(REDUNDANT_RETURN_ID).map(Rule::id), Some(REDUNDANT_RETURN_ID));
        assert_eq!(
            find_builtin_rule(REDUNDANT_SUPER_CALL_ID).map(Rule::id),
            Some(REDUNDANT_SUPER_CALL_ID)
        );
        assert_eq!(
            find_builtin_rule(BROKEN_NULL_CHECK_ID).map(Rule::id),
            Some(BROKEN_NULL_CHECK_ID)
        );
        assert_eq!(find_builtin_rule(DUPLICATE_IMPORT_ID).map(Rule::id), Some(DUPLICATE_IMPORT_ID));
        assert!(find_builtin_rule(crate::RuleId::new("unknown")).is_none());
    }

    /// `RuleCategory::ALL` is the vocabulary a frontend offers for category
    /// selection, so a category a rule declares but the list omits would be a
    /// group no user could name. This is the guard that the list stays honest.
    #[test]
    fn every_declared_category_is_listed_and_addressable_by_name() {
        for rule in builtin_rules() {
            let category = rule.meta().category;
            assert!(
                crate::RuleCategory::ALL.contains(&category),
                "{} declares a category missing from RuleCategory::ALL",
                rule.id()
            );
        }
        for category in crate::RuleCategory::ALL {
            assert_eq!(crate::RuleCategory::from_name(category.as_str()), Some(category));
        }
        assert_eq!(crate::RuleCategory::from_name("all"), None);
        assert_eq!(crate::RuleCategory::from_name(""), None);
    }

    #[test]
    fn every_rule_module_registers_one_rule_named_after_its_file() {
        let mut modules = std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/rules"))
            .expect("rules directory should be readable")
            .map(|entry| {
                entry
                    .expect("rules directory entry")
                    .file_name()
                    .into_string()
                    .expect("rule file names are UTF-8")
            })
            .filter(|name| name != "mod.rs")
            .map(|name| name.trim_end_matches(".rs").to_string())
            .collect::<Vec<_>>();
        modules.sort_unstable();

        #[expect(clippy::disallowed_methods, reason = "the names are collected and sorted")]
        let mut registered = builtin_rules()
            .iter()
            .map(|rule| rule.id().as_str().replace('-', "_"))
            .collect::<Vec<_>>();
        registered.sort_unstable();

        assert_eq!(
            modules, registered,
            "every src/rules module must register exactly one built-in rule \
             whose ID is the file name with dashes"
        );
    }
}
