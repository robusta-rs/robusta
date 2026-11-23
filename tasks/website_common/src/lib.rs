use cow_utils::CowUtils;

mod schema_json;
mod schema_markdown;

pub use schema_json::{generate_schema_json, root_schema};
pub use schema_markdown::{Renderer, Section};

// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/tasks/website_common/src/lib.rs
/// Turn bpaf's raw markdown into website-ready markdown.
#[must_use]
pub fn generate_cli_docs(raw_markdown: &str) -> String {
    let markdown = raw_markdown
        .trim_start()
        .trim_start_matches("# Command summary\n")
        .trim_start_matches('\n');
    let markdown = format!("---\nsearch: false\n---\n\n{markdown}");
    let markdown = markdown.cow_replacen("**Usage**:", "## Usage\n", 1);

    markdown
        .split('\n')
        .flat_map(|line| {
            // bpaf can emit a heading marker inline rather than at line start.
            if line.contains("###") {
                line.split("###").map(str::trim).chain(["\n"]).collect::<Vec<_>>()
            } else {
                vec![line]
            }
        })
        .map(|line| {
            if let Some(line) = line.strip_prefix("**")
                && let Some(line) = line.strip_suffix("**")
            {
                format!("## {line}")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
