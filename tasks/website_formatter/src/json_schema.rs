use robusta_cli::ConfigFile;
use website_common::{Renderer, root_schema};

#[expect(clippy::print_stdout)]
pub fn print_schema_markdown() {
    println!("{}", generate_schema_markdown());
}

/// Render the `robusta.yaml` options the formatter reads as website-ready markdown.
fn generate_schema_markdown() -> String {
    let mut root_schema = root_schema::<ConfigFile>();
    if let Some(object) = root_schema.schema.object.as_mut() {
        object.properties.remove("lint");
    }
    let mut renderer = Renderer::new(root_schema);
    // `format.overrides[n].options` takes the `format` settings, so expanding
    // it would repeat every one of them.
    renderer.with_property_filters(vec!["format.overrides[n].options"]);
    renderer.render()
}

#[cfg(test)]
mod tests {
    use super::generate_schema_markdown;

    #[test]
    fn schema_reference_matches_the_snapshot() {
        insta::with_settings!({ prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(generate_schema_markdown());
        });
    }
}
