use lazy_regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub(super) struct UnusedLocalVariableConfig {
    /// Regular expression matching names the rule should ignore.
    #[serde(with = "ignore_pattern", skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "ignore_pattern::schema")]
    pub ignore_pattern: Option<Regex>,
}

impl UnusedLocalVariableConfig {
    pub const DEFAULT: Self = Self { ignore_pattern: None };
}

impl Default for UnusedLocalVariableConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

mod ignore_pattern {
    use lazy_regex::Regex;
    use schemars::{
        SchemaGenerator,
        schema::{InstanceType, Schema, SchemaObject},
    };
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Regex>, D::Error> {
        Option::<String>::deserialize(deserializer)?
            .map(|pattern| Regex::new(&pattern).map_err(D::Error::custom))
            .transpose()
    }

    #[expect(clippy::ref_option, reason = "Serde passes a reference to the field")]
    pub fn serialize<S: Serializer>(
        pattern: &Option<Regex>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        pattern.as_ref().map(Regex::as_str).serialize(serializer)
    }

    pub fn schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(vec![InstanceType::String, InstanceType::Null].into()),
            ..SchemaObject::default()
        }
        .into()
    }
}
