// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/tasks/website_common/src/schema_json.rs

use schemars::JsonSchema;
use schemars::r#gen::SchemaSettings;
use schemars::schema::RootSchema;
use serde_json::Value;

/// Derive the schema of a configuration type exactly as its JSON Schema derives it.
///
/// Both the JSON Schema and the generated configuration reference read this,
/// so the two cannot disagree about shape, defaults, or ranges.
#[must_use]
pub fn root_schema<T: JsonSchema>() -> RootSchema {
    SchemaSettings::draft07()
        .with(|settings| {
            // Optional fields are already absent from `required`, so a null
            // type adds nothing.
            settings.option_add_null_type = false;
        })
        .into_generator()
        .into_root_schema_for::<T>()
}

/// Generate the JSON Schema of a configuration type for editors.
///
/// # Panics
///
/// Panics if the derived schema cannot be serialized.
pub fn generate_schema_json<T: JsonSchema>() -> String {
    let mut schema = serde_json::to_value(root_schema::<T>()).expect("schema should serialize");

    inject_markdown_descriptions(&mut schema);

    let mut output = serde_json::to_string_pretty(&schema).expect("schema should serialize");
    output.push('\n');
    output
}

// Standard descriptions remain the source of truth; markdown-aware editors
// receive the same text.
fn inject_markdown_descriptions(value: &mut Value) {
    match value {
        Value::Object(object) => {
            if let Some(Value::String(description)) = object.get("description") {
                object
                    .insert("markdownDescription".to_string(), Value::String(description.clone()));
            }
            for value in object.values_mut() {
                inject_markdown_descriptions(value);
            }
        }
        Value::Array(items) => {
            for item in items {
                inject_markdown_descriptions(item);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}
