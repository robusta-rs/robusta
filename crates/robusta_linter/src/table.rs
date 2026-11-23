// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_linter/src/table.rs
use std::fmt::Write;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::{RuleCategory, RuleFixMeta, RuleId, rules::builtin_rules};

pub struct RuleTable {
    pub sections: Vec<RuleTableSection>,
    pub total: usize,
    pub turned_on_by_default_count: usize,
    pub rules_with_fixes: usize,
}

pub struct RuleTableSection {
    pub rows: Vec<RuleTableRow>,
    pub category: RuleCategory,
    pub rule_column_width: usize,
}

pub struct RuleTableRow {
    pub name: &'static str,
    pub category: RuleCategory,
    #[cfg(feature = "ruledocs")]
    pub documentation: Option<&'static str>,
    #[cfg(feature = "ruledocs")]
    pub settings_schema: Option<schemars::schema::Schema>,
    pub turned_on_by_default: bool,
    pub autofix: RuleFixMeta,
}

impl Default for RuleTable {
    fn default() -> Self {
        Self::new(None)
    }
}

impl RuleTable {
    #[cfg_attr(not(feature = "ruledocs"), expect(unused_variables))]
    pub fn new(generator: Option<&mut schemars::SchemaGenerator>) -> Self {
        #[cfg(feature = "ruledocs")]
        let mut generator = generator;
        let mut rows = builtin_rules()
            .iter()
            .map(|rule| {
                let meta = rule.meta();
                RuleTableRow {
                    name: meta.id.as_str(),
                    category: meta.category,
                    #[cfg(feature = "ruledocs")]
                    documentation: rule.documentation(),
                    #[cfg(feature = "ruledocs")]
                    settings_schema: generator.as_deref_mut().and_then(|g| rule.settings_schema(g)),
                    turned_on_by_default: meta.category == RuleCategory::Correctness,
                    autofix: meta.fix,
                }
            })
            .collect::<Vec<_>>();
        rows.sort_unstable_by_key(|row| row.name);
        let total = rows.len();
        let turned_on_by_default_count = rows.iter().filter(|r| r.turned_on_by_default).count();
        let rules_with_fixes = rows.iter().filter(|r| r.autofix.has_fix()).count();
        let mut rows_by_category = rows.into_iter().fold(
            FxHashMap::default(),
            |mut map: FxHashMap<RuleCategory, Vec<RuleTableRow>>, row| {
                map.entry(row.category).or_default().push(row);
                map
            },
        );
        let sections = [RuleCategory::Correctness, RuleCategory::Suspicious, RuleCategory::Style]
            .into_iter()
            .filter_map(|category| {
                let rows = rows_by_category.remove(&category)?;
                let rule_column_width = rows.iter().map(|r| r.name.len()).max()?.max(9);
                Some(RuleTableSection { rows, category, rule_column_width })
            })
            .collect();
        Self { sections, total, turned_on_by_default_count, rules_with_fixes }
    }

    /// Iterate over all rows in section order.
    pub fn into_rows(self) -> impl Iterator<Item = RuleTableRow> {
        self.sections.into_iter().flat_map(|section| section.rows)
    }
}

impl RuleTableSection {
    fn render_markdown_table_inner(&self, enabled: Option<&FxHashSet<RuleId>>) -> String {
        const FIX: usize = 8;
        const DEFAULT: usize = 7;
        const ENABLED: usize = 8;

        let mut s = String::new();
        let category = self.category;
        let rule_width = self.rule_column_width;
        writeln!(s, "## {} ({})", category, self.rows.len()).unwrap();
        writeln!(s, "{}", category.description()).unwrap();
        let x = "";
        if enabled.is_some() {
            writeln!(s, "| {:<rule_width$} | Default | Enabled? | Fixable? |", "Rule name")
                .unwrap();
            writeln!(s, "| {x:-<rule_width$} | {x:-<7} | {x:-<8} | {x:-<8} |").unwrap();
        } else {
            writeln!(s, "| {:<rule_width$} | Default | Fixable? |", "Rule name").unwrap();
            writeln!(s, "| {x:-<rule_width$} | {x:-<7} | {x:-<8} |").unwrap();
        }
        for row in &self.rows {
            let name = row.name;
            let (default, default_width) =
                if row.turned_on_by_default { ("✅", DEFAULT - 1) } else { ("", DEFAULT) };
            let (fix_emoji, fix_width) = Self::calculate_fix_emoji_width(row.autofix.emoji(), FIX);
            if let Some(enabled) = enabled {
                let (mark, enabled_width) = if enabled.contains(&RuleId::new(name)) {
                    ("✅", ENABLED - 1)
                } else {
                    ("", ENABLED)
                };
                writeln!(
                    s,
                    "| {name:<rule_width$} | {default:<default_width$} | {mark:<enabled_width$} | {fix_emoji:<fix_width$} |"
                )
                .unwrap();
            } else {
                writeln!(
                    s,
                    "| {name:<rule_width$} | {default:<default_width$} | {fix_emoji:<fix_width$} |"
                )
                .unwrap();
            }
        }
        s
    }

    fn calculate_fix_emoji_width(emoji: Option<&str>, fix_col_width: usize) -> (&str, usize) {
        emoji.map_or(("", fix_col_width), |emoji| {
            let width = match emoji {
                "🛠️⚠️🛠️️" => fix_col_width + 4,
                "⚠️🛠️️" => fix_col_width + 3,
                "🛠️" => fix_col_width + 1,
                "" => fix_col_width,
                _ => fix_col_width.saturating_sub(2),
            };
            (emoji, width)
        })
    }

    pub fn render_markdown_table(&self) -> String {
        self.render_markdown_table_inner(None)
    }

    pub fn render_markdown_table_cli(&self, enabled: &FxHashSet<RuleId>) -> String {
        self.render_markdown_table_inner(Some(enabled))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documentation_tables_omit_only_the_enabled_column() {
        let table = RuleTable::default();
        let mut names = FxHashSet::default();
        for section in &table.sections {
            let enabled = section.rows.iter().map(|row| RuleId::new(row.name)).collect();
            let docs = section.render_markdown_table();
            let cli = section.render_markdown_table_cli(&enabled);
            assert!(!docs.contains("Enabled?"));
            assert!(cli.contains("Enabled?"));
            assert_eq!(docs.lines().count(), 4 + section.rows.len());
            for row in &section.rows {
                assert!(names.insert(row.name));
                assert!(docs.contains(row.name));
                assert_eq!(row.category, section.category);
            }
        }
        assert_eq!(names.len(), builtin_rules().len());
        assert_eq!(table.total, names.len());
    }
}
