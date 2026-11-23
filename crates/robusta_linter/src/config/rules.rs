use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt,
    ops::{Deref, DerefMut},
};

use schemars::{
    JsonSchema, SchemaGenerator,
    schema::{ArrayValidation, InstanceType, ObjectValidation, Schema, SchemaObject},
};
use serde::de::{IgnoredAny, IntoDeserializer, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{RuleSeverityConfig, rules::builtin_rules};

/// Built-in rule IDs mapped to their values.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(transparent)]
pub struct LintRules(BTreeMap<String, RuleConfig>);

impl LintRules {
    pub fn new(rules: BTreeMap<String, RuleConfig>) -> Self {
        Self(rules)
    }
}

impl Deref for LintRules {
    type Target = BTreeMap<String, RuleConfig>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LintRules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl JsonSchema for LintRules {
    fn schema_name() -> String {
        "LintRules".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("LintRules")
    }

    fn json_schema(r#gen: &mut SchemaGenerator) -> Schema {
        let severity = r#gen.subschema_for::<RuleSeverityConfig>();
        let properties = builtin_rules()
            .iter()
            .map(|rule| {
                let mut value = rule.settings_schema(r#gen).map_or_else(
                    || severity.clone().into_object(),
                    |settings| {
                        let pair = SchemaObject {
                            instance_type: Some(InstanceType::Array.into()),
                            array: Some(Box::new(ArrayValidation {
                                items: Some(vec![severity.clone(), settings].into()),
                                min_items: Some(2),
                                max_items: Some(2),
                                ..ArrayValidation::default()
                            })),
                            ..SchemaObject::default()
                        };
                        let mut value = SchemaObject::default();
                        value.subschemas().any_of = Some(vec![severity.clone(), pair.into()]);
                        value
                    },
                );
                value.metadata().description = Some(rule.meta().summary.to_string());
                (rule.id().as_str().to_string(), value.into())
            })
            .collect();
        SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                properties,
                additional_properties: Some(Box::new(false.into())),
                ..ObjectValidation::default()
            })),
            ..SchemaObject::default()
        }
        .into()
    }
}

/// A rule's severity, or `[severity, settings]`, where each named setting
/// replaces its default.
#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum RuleConfig {
    Severity(RuleSeverityConfig),
    WithSettings(RuleSeverityConfig, serde_json::Map<String, serde_json::Value>),
}

impl RuleConfig {
    pub const fn severity(&self) -> RuleSeverityConfig {
        match self {
            Self::Severity(severity) | Self::WithSettings(severity, _) => *severity,
        }
    }
}

impl<'de> Deserialize<'de> for RuleConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RuleConfigVisitor;

        impl<'de> Visitor<'de> for RuleConfigVisitor {
            type Value = RuleConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`allow`, `warn`, `deny`, or `[severity, settings]`")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                RuleSeverityConfig::deserialize(value.into_deserializer()).map(RuleConfig::Severity)
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let invalid_length = |length| serde::de::Error::invalid_length(length, &self);
                let severity = seq.next_element()?.ok_or_else(|| invalid_length(0))?;
                let settings = seq.next_element()?.ok_or_else(|| invalid_length(1))?;
                if seq.next_element::<IgnoredAny>()?.is_some() {
                    return Err(invalid_length(3));
                }
                Ok(RuleConfig::WithSettings(severity, settings))
            }
        }

        deserializer.deserialize_any(RuleConfigVisitor)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use serde_json::Value;

    use super::LintRules;
    use crate::rules::builtin_rules;

    fn schema() -> Value {
        serde_json::to_value(schemars::schema_for!(LintRules)).expect("schema should serialize")
    }

    #[test]
    fn rules_are_closed_over_the_live_registry() {
        let schema = schema();
        assert_eq!(schema["additionalProperties"], false);
        let actual = schema["properties"]
            .as_object()
            .expect("rules schema should have named properties")
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let expected = builtin_rules().iter().map(|rule| rule.id().as_str()).collect();
        assert_eq!(actual, expected);
        for rule in builtin_rules() {
            assert_eq!(
                schema["properties"][rule.id().as_str()]["description"],
                rule.meta().summary
            );
        }
    }

    #[test]
    fn rules_with_settings_also_take_a_severity_and_settings_pair() {
        let schema = schema();
        let severity = "#/definitions/RuleSeverityConfig";
        assert_eq!(schema["properties"]["unused-import"]["allOf"][0]["$ref"], severity);
        let forms = &schema["properties"]["unused-local-variable"]["anyOf"];
        assert_eq!(forms[0]["$ref"], severity);
        let pair = &forms[1];
        assert_eq!(pair["minItems"], 2);
        assert_eq!(pair["maxItems"], 2);
        assert_eq!(pair["items"][0]["$ref"], severity);
        assert_eq!(pair["items"][1]["$ref"], "#/definitions/UnusedLocalVariableConfig");
        let settings = &schema["definitions"]["UnusedLocalVariableConfig"];
        assert_eq!(settings["additionalProperties"], false);
        let properties = settings["properties"].as_object().unwrap();
        assert_eq!(properties.len(), 1);
        let pattern = &properties["ignore-pattern"];
        assert_eq!(pattern["type"], serde_json::json!(["string", "null"]));
        assert!(pattern["description"].as_str().is_some_and(|description| !description.is_empty()));
    }
}
