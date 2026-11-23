// Directory adapted from https://github.com/oxc-project/oxc/tree/0e6498f7e218420153910074d5a55bbef7c1c1d4/tasks/website_linter/src/rules
mod doc_page;

use std::{fs, path::PathBuf};

use pico_args::Arguments;
use robusta_cli::get_all_rules_json;
use robusta_linter::table::RuleTable;
use schemars::{SchemaGenerator, schema::RootSchema};

use website_common::Renderer;

const HELP: &str = "usage: website_linter rules [options]

Options:
    --rules-json <path>  Write the rule metadata JSON file.
    --rule-docs <path>   Write rule pages in this directory.
    --rule-count <path>  Write rule-count.data.js in this directory.
    --git-ref <ref>      Link rule sources at this commit, branch, or tag (default: main).
    -h, --help           Show this help message.";

/// Write the selected rule references.
///
/// # Errors
/// Returns an error for invalid arguments or an output that cannot be written.
#[expect(clippy::print_stdout)]
pub fn print_rules(mut args: Arguments) -> Result<(), Box<dyn std::error::Error>> {
    if args.contains(["-h", "--help"]) {
        println!("{HELP}");
        return Ok(());
    }
    let rules_json: Option<PathBuf> = args.opt_value_from_str("--rules-json")?;
    let rule_docs: Option<PathBuf> = args.opt_value_from_str("--rule-docs")?;
    let rule_count: Option<PathBuf> = args.opt_value_from_str("--rule-count")?;
    let git_ref: Option<String> = args.opt_value_from_str("--git-ref")?;
    if !args.finish().is_empty() {
        return Err("unexpected rule reference arguments".into());
    }
    if rules_json.is_none() && rule_docs.is_none() && rule_count.is_none() {
        return Err(HELP.into());
    }

    if let Some(path) = rules_json {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, get_all_rules_json())?;
    }
    if let Some(directory) = rule_docs {
        fs::create_dir_all(&directory)?;
        for (name, page) in render_rule_doc_pages() {
            fs::write(directory.join(format!("{name}.md")), page)?;
        }
        fs::write(directory.join("version.data.js"), version_data(git_ref.as_deref()))?;
    }
    if let Some(directory) = rule_count {
        fs::create_dir_all(&directory)?;
        fs::write(directory.join("rule-count.data.js"), rule_count_data())?;
    }
    Ok(())
}

fn render_rule_doc_pages() -> impl Iterator<Item = (&'static str, String)> {
    let mut generator = SchemaGenerator::default();
    let table = RuleTable::new(Some(&mut generator));
    let renderer = Renderer::new(RootSchema {
        definitions: generator.take_definitions(),
        ..RootSchema::default()
    });
    table
        .into_rows()
        .map(move |rule| (rule.name, doc_page::render_rule_docs_page(&rule, &renderer)))
}

fn rule_count_data() -> String {
    format!("export default {{ load() {{ return {} }} }}\n", RuleTable::default().total)
}

fn version_data(git_ref: Option<&str>) -> String {
    format!("export default {{ load() {{ return \"{}\" }} }}\n", git_ref.unwrap_or("main"))
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    #[test]
    fn rule_references_match_the_snapshot() {
        let mut snapshot = get_all_rules_json();
        snapshot.push_str(&rule_count_data());
        snapshot.push_str(&version_data(None));
        for (name, page) in render_rule_doc_pages() {
            writeln!(snapshot, "\n--- {name}.md ---\n{page}").unwrap();
        }
        insta::with_settings!({ prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(snapshot);
        });
    }
}
