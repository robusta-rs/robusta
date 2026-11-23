use std::borrow::Cow;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use rustc_hash::FxHashSet;
use serde::Serialize;

use robusta_diagnostics::{FileReport, RuleInfo, SourceRange};

use super::{Info, severity_label, to_pretty_document};

const INFORMATION_URI: &str = "https://robusta.rs";
/// The normative OASIS schema for SARIF 2.1.0. Mirrors of this document exist
/// and describe the same format, but validators that pin the canonical URI
/// treat them as different schemas.
const SARIF_SCHEMA: &str =
    "https://docs.oasis-open.org/sarif/sarif/v2.1.0/errata01/os/schemas/sarif-schema-2.1.0.json";

#[derive(Serialize)]
struct SarifDocument<'a> {
    #[serde(rename = "$schema")]
    schema: &'static str,
    version: &'static str,
    runs: Vec<SarifRun<'a>>,
}

#[derive(Serialize)]
struct SarifRun<'a> {
    tool: SarifTool<'a>,
    results: Vec<SarifResult<'a>>,
}

#[derive(Serialize)]
struct SarifTool<'a> {
    driver: SarifDriver<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifDriver<'a> {
    name: &'static str,
    information_uri: &'static str,
    version: &'static str,
    rules: Vec<SarifRule<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifRule<'a> {
    id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_description: Option<SarifText<'a>>,
}

#[derive(Serialize)]
struct SarifText<'a> {
    text: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifResult<'a> {
    rule_id: &'a str,
    level: &'static str,
    message: SarifText<'a>,
    locations: Vec<SarifLocation<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifLocation<'report> {
    physical_location: SarifPhysicalLocation<'report>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifPhysicalLocation<'report> {
    artifact_location: SarifArtifactLocation<'report>,
    /// Absent when the producer could not resolve a start line; SARIF then
    /// locates the result at the artifact rather than at coordinates that
    /// point nowhere.
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<SarifRegion>,
}

#[derive(Serialize)]
struct SarifArtifactLocation<'report> {
    /// SARIF defines this as a URI reference, not a path. Reported paths
    /// already carry `/` on every platform, but the names between those
    /// separators are file data that URI syntax would otherwise read as
    /// structure, so [`artifact_uri`] encodes them.
    uri: Cow<'report, str>,
}

/// A text region. `endColumn` is exclusive, so it addresses the character
/// following the last one in the region. Every optional component is omitted
/// rather than serialized as an out-of-range `0`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifRegion {
    start_line: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_column: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_column: Option<usize>,
}

pub(super) fn render(reports: &[&FileReport]) -> String {
    let mut rule_ids = FxHashSet::default();
    let mut rules: Vec<RuleInfo> = Vec::new();
    let mut results: Vec<SarifResult<'_>> = Vec::new();
    for info in Info::of_all(reports) {
        if rule_ids.insert(info.rule.id) {
            rules.push(info.rule);
        }
        results.push(sarif_result(&info));
    }
    let rules = rules
        .iter()
        .map(|rule| SarifRule {
            id: rule.id,
            short_description: rule.summary.map(|text| SarifText { text }),
        })
        .collect();
    let document = SarifDocument {
        schema: SARIF_SCHEMA,
        version: "2.1.0",
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "robusta",
                    information_uri: INFORMATION_URI,
                    version: env!("CARGO_PKG_VERSION"),
                    rules,
                },
            },
            results,
        }],
    };
    to_pretty_document(&document)
}

fn sarif_result<'report>(info: &Info<'report>) -> SarifResult<'report> {
    let Info { filename, message, rule, .. } = *info;
    SarifResult {
        rule_id: rule.id,
        level: severity_label(info.severity),
        message: SarifText { text: message },
        locations: vec![SarifLocation {
            physical_location: SarifPhysicalLocation {
                artifact_location: SarifArtifactLocation { uri: artifact_uri(filename) },
                region: region(info.range()),
            },
        }],
    }
}

/// Everything a URI path may carry as itself: RFC 3986's unreserved set, plus
/// `/`, which stays a separator rather than becoming name data. Every other
/// byte — a space, `#`, `?`, `%`, a control character, or one byte of a
/// non-ASCII UTF-8 name — is percent-encoded, so the reference decodes back to
/// the reported path exactly.
const PATH_DATA: AsciiSet =
    NON_ALPHANUMERIC.remove(b'-').remove(b'.').remove(b'_').remove(b'~').remove(b'/');

/// A reported display path as a SARIF `artifactLocation.uri`.
///
/// This is a lexical conversion: it reads no filesystem, resolves nothing, and
/// requires no artifact to exist, so a path named only by a diagnostic stays
/// reportable. Encoding `:` is what keeps a path a path — an unencoded first
/// segment such as `C:` or `notes:2026` would parse as a URI scheme — while a
/// leading `/` still marks an absolute path and a leading `//` still gives a
/// UNC server the reference's authority position. An encoded drive letter is
/// the first path segment rather than an authority, so a drive path stays a
/// relative reference instead of becoming a `file:` URI; re-rooting it would
/// name an artifact the run never selected.
fn artifact_uri(path: &str) -> Cow<'_, str> {
    utf8_percent_encode(path, &PATH_DATA).into()
}

/// A region needs a start line to address anything; without one the result
/// carries its artifact location alone.
fn region(range: SourceRange) -> Option<SarifRegion> {
    (range.start.line != 0).then(|| SarifRegion {
        start_line: range.start.line,
        start_column: nonzero(range.start.column),
        end_line: nonzero(range.end.line),
        end_column: nonzero(range.end.column),
    })
}

fn nonzero(value: usize) -> Option<usize> {
    (value != 0).then_some(value)
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{
        DEMO_RULE, UNDESCRIBED_RULE, finding, position, ranged_finding, report_with,
        unpositioned_finding,
    };
    use super::*;
    use cow_utils::CowUtils;
    use robusta_diagnostics::Severity;

    #[test]
    fn collects_rules_with_descriptions_and_results() {
        let report = report_with(
            "src/C.java",
            Some(false),
            vec![
                ranged_finding(
                    DEMO_RULE,
                    Severity::Warning,
                    "msg",
                    position(2, 5),
                    position(3, 12),
                ),
                ranged_finding(
                    UNDESCRIBED_RULE,
                    Severity::Error,
                    "bare",
                    position(4, 9),
                    position(4, 14),
                ),
            ],
        );
        let rendered = render(&[&report]);
        let normalized = rendered.cow_replace(env!("CARGO_PKG_VERSION"), "<version>");
        insta::assert_snapshot!(normalized, @r###"
        {
          "$schema": "https://docs.oasis-open.org/sarif/sarif/v2.1.0/errata01/os/schemas/sarif-schema-2.1.0.json",
          "version": "2.1.0",
          "runs": [
            {
              "tool": {
                "driver": {
                  "name": "robusta",
                  "informationUri": "https://robusta.rs",
                  "version": "<version>",
                  "rules": [
                    {
                      "id": "format",
                      "shortDescription": {
                        "text": "Reports files whose contents differ from the project's configured format."
                      }
                    },
                    {
                      "id": "demo",
                      "shortDescription": {
                        "text": "Reports demonstration findings."
                      }
                    },
                    {
                      "id": "undescribed"
                    }
                  ]
                }
              },
              "results": [
                {
                  "ruleId": "format",
                  "level": "warning",
                  "message": {
                    "text": "file is not formatted; run `robusta format --fix`"
                  },
                  "locations": [
                    {
                      "physicalLocation": {
                        "artifactLocation": {
                          "uri": "src/C.java"
                        },
                        "region": {
                          "startLine": 1,
                          "startColumn": 1,
                          "endLine": 1,
                          "endColumn": 1
                        }
                      }
                    }
                  ]
                },
                {
                  "ruleId": "demo",
                  "level": "warning",
                  "message": {
                    "text": "msg"
                  },
                  "locations": [
                    {
                      "physicalLocation": {
                        "artifactLocation": {
                          "uri": "src/C.java"
                        },
                        "region": {
                          "startLine": 2,
                          "startColumn": 5,
                          "endLine": 3,
                          "endColumn": 12
                        }
                      }
                    }
                  ]
                },
                {
                  "ruleId": "undescribed",
                  "level": "error",
                  "message": {
                    "text": "bare"
                  },
                  "locations": [
                    {
                      "physicalLocation": {
                        "artifactLocation": {
                          "uri": "src/C.java"
                        },
                        "region": {
                          "startLine": 4,
                          "startColumn": 9,
                          "endLine": 4,
                          "endColumn": 14
                        }
                      }
                    }
                  ]
                }
              ]
            }
          ]
        }
        "###);
    }

    /// Every path shape the CLI can report, with the URI reference it must
    /// produce. The separators and the unreserved characters stay literal; a
    /// name's own bytes do not.
    const URI_CASES: &[(&str, &str)] = &[
        // An ordinary path is already a URI reference and stays byte-identical.
        ("src/main/java/C.java", "src/main/java/C.java"),
        ("src/a-b_c.d~e.java", "src/a-b_c.d~e.java"),
        // A space and a fragment marker are name data, not URI syntax.
        ("src/Example #1.java", "src/Example%20%231.java"),
        // A name that only looks pre-encoded is encoded once more, so it stays
        // distinct from the name that really holds a space.
        ("src/Example%20.java", "src/Example%2520.java"),
        ("src/Which?.java", "src/Which%3F.java"),
        ("src/a\u{1}b.java", "src/a%01b.java"),
        ("src/Caf\u{e9}.java", "src/Caf%C3%A9.java"),
        // Outside Windows a backslash is part of a file name.
        (r"src\A.java", "src%5CA.java"),
        // A colon in the first segment would otherwise read as a scheme, so a
        // relative path stays relative and a drive letter stays a directory.
        ("notes:2026/C.java", "notes%3A2026/C.java"),
        ("C:/temp/Example #1.java", "C%3A/temp/Example%20%231.java"),
        // A leading slash still means absolute, and a leading double slash
        // still puts the UNC server in the reference's authority position.
        ("/tmp/a b/C.java", "/tmp/a%20b/C.java"),
        ("//server/share/a b/C.java", "//server/share/a%20b/C.java"),
    ];

    #[test]
    fn artifact_uris_encode_name_data_and_keep_separators() {
        for (path, expected) in URI_CASES {
            assert_eq!(artifact_uri(path), *expected, "{path}");
        }
    }

    /// The conversion is lossless: percent-decoding a reference recovers the
    /// path the run selected, which is what makes the SARIF artifact the same
    /// file the rest of the output names.
    #[test]
    fn artifact_uris_decode_back_to_the_reported_path() {
        for (path, _) in URI_CASES {
            let uri = artifact_uri(path);
            let decoded = percent_encoding::percent_decode_str(&uri)
                .decode_utf8()
                .expect("an encoded UTF-8 path decodes as UTF-8");
            assert_eq!(decoded, *path);
        }
    }

    /// Guards the encoding set itself rather than the individual cases: a URI
    /// path may hold unreserved characters, the `/` separator, and nothing
    /// else that is not a complete percent-escape.
    #[test]
    fn artifact_uris_hold_only_unreserved_characters_separators_and_escapes() {
        for (path, _) in URI_CASES {
            let uri = artifact_uri(path);
            let mut bytes = uri.bytes();
            while let Some(byte) = bytes.next() {
                if byte == b'%' {
                    let escape: Vec<u8> = bytes.by_ref().take(2).collect();
                    assert!(
                        escape.len() == 2 && escape.iter().all(u8::is_ascii_hexdigit),
                        "{uri}: truncated escape"
                    );
                } else {
                    assert!(
                        byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte),
                        "{uri}: unencoded {byte:#04x}"
                    );
                }
            }
        }
    }

    #[test]
    fn omits_the_region_when_no_start_line_resolved() {
        let report = report_with(
            "src/C.java",
            None,
            vec![unpositioned_finding(DEMO_RULE, Severity::Error, "no location")],
        );
        let rendered = render(&[&report]);

        assert!(rendered.contains(r#""uri": "src/C.java""#), "{rendered}");
        assert!(!rendered.contains("region"), "{rendered}");
        assert!(!rendered.contains(": 0"), "{rendered}");
    }

    #[test]
    fn registers_each_rule_once_in_first_occurrence_order() {
        const RESTATED_DEMO: RuleInfo = RuleInfo { id: "demo", summary: Some("Restated.") };
        let first = report_with(
            "src/A.java",
            Some(false),
            vec![
                finding(UNDESCRIBED_RULE, Severity::Error, "one", 1, 1),
                finding(DEMO_RULE, Severity::Warning, "two", 2, 1),
            ],
        );
        let second = report_with(
            "src/B.java",
            Some(false),
            vec![
                finding(RESTATED_DEMO, Severity::Warning, "three", 1, 1),
                finding(UNDESCRIBED_RULE, Severity::Error, "four", 2, 1),
            ],
        );
        let document: serde_json::Value =
            serde_json::from_str(&render(&[&first, &second])).unwrap();
        let driver = &document["runs"][0]["tool"]["driver"];

        insta::assert_snapshot!(serde_json::to_string_pretty(&driver["rules"]).unwrap(), @r#"
        [
          {
            "id": "format",
            "shortDescription": {
              "text": "Reports files whose contents differ from the project's configured format."
            }
          },
          {
            "id": "undescribed"
          },
          {
            "id": "demo",
            "shortDescription": {
              "text": "Reports demonstration findings."
            }
          }
        ]
        "#);
        assert_eq!(document["runs"][0]["results"].as_array().unwrap().len(), 6);
    }

    #[test]
    fn empty_input_registers_no_rules() {
        let document: serde_json::Value = serde_json::from_str(&render(&[])).unwrap();
        let run = &document["runs"][0];

        assert_eq!(run["tool"]["driver"]["rules"], serde_json::json!([]));
        assert_eq!(run["results"], serde_json::json!([]));
    }
}
