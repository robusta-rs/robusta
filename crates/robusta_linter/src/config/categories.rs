use std::{borrow::Cow, collections::BTreeMap, ops::Deref};

use schemars::{JsonSchema, r#gen::SchemaGenerator, schema::Schema};
use serde::{Deserialize, Serialize};

use crate::{RuleCategory, RuleSeverityConfig};

/// Category names mapped to the severity of their rules.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LintCategories(BTreeMap<String, RuleSeverityConfig>);

impl Deref for LintCategories {
    type Target = BTreeMap<String, RuleSeverityConfig>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JsonSchema for LintCategories {
    fn schema_name() -> String {
        "LintCategories".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("LintCategories")
    }

    fn json_schema(r#gen: &mut SchemaGenerator) -> Schema {
        let properties = RuleCategory::ALL
            .into_iter()
            .map(|category| {
                let mut severity = r#gen.subschema_for::<RuleSeverityConfig>().into_object();
                severity.metadata().description = Some(category.description().to_string());
                (category.as_str().to_string(), severity.into())
            })
            .collect();
        let mut schema =
            r#gen.subschema_for::<BTreeMap<String, RuleSeverityConfig>>().into_object();
        let object = schema.object();
        object.properties = properties;
        object.additional_properties = Some(Box::new(false.into()));
        schema.into()
    }
}

#[cfg(test)]
mod tests {
    use super::LintCategories;
    use crate::RuleCategory;

    #[test]
    fn categories_are_closed_over_the_live_vocabulary() {
        let schema = serde_json::to_value(schemars::schema_for!(LintCategories))
            .expect("schema should serialize");
        assert_eq!(schema["additionalProperties"], false);
        let properties = schema["properties"].as_object().expect("category properties");
        assert_eq!(properties.len(), RuleCategory::ALL.len());
        for category in RuleCategory::ALL {
            assert_eq!(properties[category.as_str()]["description"], category.description());
            assert_eq!(
                properties[category.as_str()]["allOf"][0]["$ref"],
                "#/definitions/RuleSeverityConfig"
            );
        }
    }
}
