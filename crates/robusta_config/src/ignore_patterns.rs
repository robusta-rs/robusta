use ignore::gitignore::GitignoreBuilder;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

use crate::gitignore;

/// Gitignore patterns relative to the directory containing `robusta.yaml`.
/// A pattern with a `..` segment, or one the gitignore syntax cannot express,
/// is rejected when the file is read.
//
// Patterns are kept as written and compiled once their configuration
// directory is known, which parsing alone does not supply.
#[derive(Clone, Debug, Default, Serialize, JsonSchema, Eq, PartialEq)]
pub struct IgnorePatterns(Vec<String>);

impl<'de> Deserialize<'de> for IgnorePatterns {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let patterns = Vec::<String>::deserialize(deserializer)?;
        for pattern in &patterns {
            validate_ignore_pattern(pattern).map_err(serde::de::Error::custom)?;
        }
        Ok(Self(patterns))
    }
}

impl IgnorePatterns {
    pub fn new<S: Into<String>, I: IntoIterator<Item = S>>(patterns: I) -> Self {
        Self(patterns.into_iter().map(Into::into).collect())
    }

    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

// Adapted from https://github.com/oxc-project/oxc/blob/83378351e43ecd605c5a52cb1454cbed9421ac1a/crates/oxc_config/src/ignore_patterns.rs
/// Reject a pattern that could reach outside the configuration directory or
/// that the gitignore syntax cannot express.
fn validate_ignore_pattern(pattern: &str) -> Result<(), String> {
    let path_part = gitignore::trim_trailing_spaces(pattern);
    let path_part = path_part.strip_prefix('!').unwrap_or(path_part);
    // Gitignore patterns use `/` as the separator on every platform.
    if path_part.split('/').any(|segment| segment == "..") {
        return Err(format!(
            "invalid ignore pattern {pattern:?}: `..` is not supported, patterns are resolved within the configuration directory"
        ));
    }
    gitignore::add_line(&mut GitignoreBuilder::new(""), pattern)
        .map_err(|err| format!("invalid ignore pattern {pattern:?}: {err}"))
}

#[cfg(test)]
mod tests {
    use super::IgnorePatterns;

    fn parse(yaml: &str) -> Result<IgnorePatterns, serde_saphyr::Error> {
        serde_saphyr::from_str(yaml)
    }

    #[test]
    fn valid_patterns_deserialize_and_round_trip() {
        let parsed = parse("- generated/\n- \"!generated/Keep.java\"\n- \"*.gen.java\"\n")
            .expect("patterns should be accepted");

        assert_eq!(
            parsed,
            IgnorePatterns::new(["generated/", "!generated/Keep.java", "*.gen.java"])
        );
        assert_eq!(
            serde_saphyr::to_string(&parsed).expect("patterns should serialize"),
            "- generated/\n- \"!generated/Keep.java\"\n- \"*.gen.java\"\n"
        );
    }

    #[test]
    fn parent_segment_is_rejected_with_its_pattern() {
        for pattern in ["../x", "src/../x", "!../x", "src/..", ".. "] {
            let error = parse(&format!("- {pattern:?}\n")).expect_err("`..` should fail");

            let message = error.to_string();
            assert!(message.contains(&format!("{pattern:?}")), "{message}");
            assert!(message.contains("`..` is not supported"), "{message}");
        }
    }

    #[test]
    fn dots_inside_a_segment_are_accepted() {
        parse("- \"..x/\"\n- \"x..\"\n- \"a/.../b\"\n").expect("only a whole `..` segment fails");
    }

    #[test]
    fn whitespace_git_keeps_is_part_of_the_segment() {
        let patterns = ["..\u{a0}", "!..\u{3000}", "..\t", "..\\ "];
        let yaml = patterns.map(|pattern| format!("- '{pattern}'")).join("\n");

        assert_eq!(parse(&yaml).expect("only spaces are trimmed"), IgnorePatterns::new(patterns));
    }

    #[test]
    fn invalid_pattern_is_rejected_with_its_pattern() {
        for pattern in ["src/*.[java", "end\\", "[[:word:]]"] {
            let error = parse(&format!("- src/\n- {pattern:?}\n"))
                .expect_err("invalid pattern should fail");

            let message = error.to_string();
            assert!(message.contains(&format!("invalid ignore pattern {pattern:?}")), "{message}");
        }
    }

    #[test]
    fn literal_braces_are_accepted() {
        let patterns = ["Open{.java", "}", "{First,Second}.java", "{"];
        let parsed = parse(&format!("{patterns:?}\n")).expect("braces are literal");

        assert_eq!(parsed, IgnorePatterns::new(patterns));
    }
}
