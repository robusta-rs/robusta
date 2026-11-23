// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/crates/oxc_config/src/glob_set.rs

use schemars::{JsonSchema, SchemaGenerator, schema::Schema};
use serde::{Deserialize, Deserializer, Serialize};

/// Glob patterns, kept as written for printing.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct GlobSet {
    patterns: Vec<String>,
    /// `patterns` as matched: without a leading `./`, and under `**/` when they have no `/`.
    #[serde(skip)]
    globs: Vec<String>,
}

// Written by hand because a derive on a transparent struct drops the description.
impl JsonSchema for GlobSet {
    fn schema_name() -> String {
        "GlobSet".to_owned()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut schema = generator.subschema_for::<Vec<String>>().into_object();
        schema.metadata().description = Some(
            concat!(
                "Glob patterns relative to the directory containing `robusta.yaml`. ",
                "`*` does not cross `/`, `**` does, and a pattern without `/` matches ",
                "a name at any depth.",
            )
            .to_owned(),
        );
        schema.into()
    }
}

impl<'de> Deserialize<'de> for GlobSet {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_new(Vec::<String>::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl GlobSet {
    /// # Errors
    ///
    /// Returns the message for the first pattern that is not a valid glob.
    pub fn try_new(patterns: Vec<String>) -> Result<Self, String> {
        for pattern in &patterns {
            fast_glob::validate(pattern)
                .map_err(|err| format!("invalid glob pattern {pattern:?}: {err}"))?;
        }
        let globs = patterns
            .iter()
            .map(|pattern| match pattern.strip_prefix("./") {
                Some(pattern) => pattern.to_owned(),
                None if pattern.contains('/') => pattern.clone(),
                None => format!("**/{pattern}"),
            })
            .collect();
        Ok(Self { patterns, globs })
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    pub fn is_match(&self, path: &str) -> bool {
        self.globs.iter().any(|glob| fast_glob::glob_match(glob, path))
    }
}

#[cfg(test)]
mod tests {
    use super::GlobSet;

    fn glob_set(patterns: &[&str]) -> GlobSet {
        GlobSet::try_new(patterns.iter().map(ToString::to_string).collect()).unwrap()
    }

    #[test]
    fn matches_relative_paths() {
        let names = glob_set(&["*.java"]);
        assert!(names.is_match("Foo.java"));
        assert!(names.is_match("src/main/Foo.java"));
        assert!(!names.is_match("Foo.kt"));

        let top_level = glob_set(&["src/*.java"]);
        assert!(top_level.is_match("src/Foo.java"));
        assert!(!top_level.is_match("src/main/Foo.java"));

        let nested = glob_set(&["src/**/*.java"]);
        assert!(nested.is_match("src/Foo.java"));
        assert!(nested.is_match("src/main/Foo.java"));

        let dot_slash = glob_set(&["./Foo.java"]);
        assert!(dot_slash.is_match("Foo.java"));
        assert!(!dot_slash.is_match("src/Foo.java"));
    }

    #[test]
    fn serializes_patterns_as_written() {
        let globs = glob_set(&["./Foo.java", "Bar.java"]);
        assert_eq!(
            serde_json::to_value(&globs).unwrap(),
            serde_json::json!(["./Foo.java", "Bar.java"])
        );
    }

    #[test]
    fn rejects_invalid_patterns() {
        let err = GlobSet::try_new(vec!["*.java".into(), "src/**/*.{java,kt".into()]).unwrap_err();
        assert!(err.starts_with("invalid glob pattern \"src/**/*.{java,kt\": "), "{err}");
    }
}
