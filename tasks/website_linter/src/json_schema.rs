use robusta_cli::ConfigFile;
use website_common::{Renderer, generate_schema_json, root_schema};

#[expect(clippy::print_stdout)]
pub fn print_schema_json() {
    print!("{}", generate_schema_json::<ConfigFile>());
}

#[expect(clippy::print_stdout)]
pub fn print_schema_markdown() {
    println!("{}", generate_schema_markdown());
}

/// Render the `robusta.yaml` options the linter reads as website-ready markdown.
fn generate_schema_markdown() -> String {
    let mut root_schema = root_schema::<ConfigFile>();
    if let Some(object) = root_schema.schema.object.as_mut() {
        object.properties.remove("format");
    }
    let mut renderer = Renderer::new(root_schema);
    // Rule maps would list every rule.
    renderer.with_property_filters(vec!["lint.rules", "lint.overrides[n].rules"]);
    renderer.render()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use cow_utils::CowUtils;
    use robusta_formatter::{IndentWidth, LineWidth};
    use serde_json::Value;

    use robusta_cli::ConfigFile;

    use super::{generate_schema_json, generate_schema_markdown};

    const SCHEMA_FILE_NAME: &str = "robusta.schema.json";

    fn generated_schema() -> Value {
        serde_json::from_str(&generate_schema_json::<ConfigFile>())
            .expect("generated schema should be JSON")
    }

    #[test]
    fn committed_schema_matches_generated_output() {
        let root = project_root::get_project_root().expect("project root should exist");
        let committed = fs::read_to_string(root.join(SCHEMA_FILE_NAME))
            .expect("committed schema should be readable");
        assert_eq!(
            committed.cow_replace("\r\n", "\n"),
            generate_schema_json::<ConfigFile>().cow_replace("\r\n", "\n"),
            "{SCHEMA_FILE_NAME} is stale; run `just config-schema`"
        );
    }

    #[test]
    fn numeric_ranges_match_runtime_validation() {
        let schema = generated_schema();
        let format = schema
            .pointer("/definitions/FormatConfig/properties")
            .and_then(Value::as_object)
            .expect("format properties should exist");
        for (name, minimum, maximum) in [
            ("line-width", f64::from(LineWidth::MIN), f64::from(LineWidth::MAX)),
            ("indent-width", f64::from(IndentWidth::MIN), f64::from(IndentWidth::MAX)),
        ] {
            let property = format.get(name).expect("width property should exist");
            assert_eq!(property.get("minimum").and_then(Value::as_f64), Some(minimum));
            assert_eq!(property.get("maximum").and_then(Value::as_f64), Some(maximum));
        }
    }

    #[test]
    fn enum_values_match_the_configuration_contract() {
        let schema = generated_schema();
        for (name, expected) in [
            ("ImportSectionConfig", &["static", "module", "type"][..]),
            ("AfterClosingBraceConfig", &["same-line", "new-line"][..]),
            ("IndentStyleConfig", &["space", "tab"][..]),
            ("DescriptionWithDotConfig", &["never", "summary", "always"][..]),
            ("CommentLineStrategyConfig", &["single-line", "multiline", "keep"][..]),
            ("LineWrappingStyleConfig", &["greedy", "keep"][..]),
            ("SeparateTagGroupsConfig", &["never", "before-return", "always"][..]),
            ("LineEndingConfig", &["lf", "crlf", "cr"][..]),
            ("RuleSeverityConfig", &["allow", "warn", "deny"][..]),
        ] {
            let values = schema
                .pointer(&format!("/definitions/{name}/enum"))
                .and_then(Value::as_array)
                .expect("enum schema should contain values")
                .iter()
                .map(|value| value.as_str().expect("enum values should be strings"))
                .collect::<Vec<_>>();
            assert_eq!(values, expected, "schema enum changed for {name}");
        }
    }

    #[test]
    fn javadoc_is_a_boolean_or_a_closed_settings_mapping() {
        let schema = generated_schema();
        let variants = schema
            .pointer("/definitions/JavadocConfig/anyOf")
            .and_then(Value::as_array)
            .expect("javadoc schema should list its forms");
        assert_eq!(variants[0].get("type").and_then(Value::as_str), Some("boolean"));
        let settings = schema
            .pointer("/definitions/JavadocSettingsConfig")
            .expect("javadoc settings schema should exist");
        assert_eq!(settings.get("additionalProperties"), Some(&Value::Bool(false)));
    }

    #[test]
    fn schema_reference_matches_the_snapshot() {
        insta::with_settings!({ prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(generate_schema_markdown());
        });
    }
}
