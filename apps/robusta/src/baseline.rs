use std::collections::BTreeMap;
use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::path::{Component, Path, PathBuf};

use robusta_diagnostics::Severity;
use robusta_linter::{LintDiagnostic, UNUSED_SUPPRESSION_ID, rules};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};

use crate::walk;

pub const FILE_NAME: &str = "robusta-suppressions.json";

/// What a run does with the baseline besides enforcing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Hide the findings each allowance covers and report the rest.
    Enforce,
    /// `--suppress-all`: record the errors that remain in each analyzed file.
    SuppressAll,
    /// `--prune-suppressions`: lower each settled allowance to the errors left.
    Prune,
}

/// Allowed error counts per file key, then per rule ID.
/// Counts do not distinguish old findings from new ones.
type Allowances = BTreeMap<String, BTreeMap<&'static str, usize>>;

/// The baseline one run reads.
pub struct Baseline {
    mode: Mode,
    /// The current directory, normalized like selected file paths.
    root: PathBuf,
    file_exists: bool,
    allowances: Allowances,
}

impl Baseline {
    /// Read and validate the baseline. `None` means nothing to enforce or write.
    ///
    /// # Errors
    ///
    /// Returns a message when the file cannot be read or is malformed.
    pub fn load(mode: Mode) -> Result<Option<Self>, String> {
        let root = walk::absolute_path(Path::new("."))
            .map_err(|error| format!("failed to read the working directory: {error}"))?;
        let contents = match std::fs::read(root.join(FILE_NAME)) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == io::ErrorKind::NotFound => None,
            Err(error) => return Err(format!("{FILE_NAME}: {error}")),
        };
        let file_exists = contents.is_some();
        let allowances = match &contents {
            Some(bytes) => parse(bytes).map_err(|error| format!("{FILE_NAME}: {error}"))?,
            None if mode == Mode::SuppressAll => Allowances::new(),
            None => return Ok(None),
        };
        Ok(Some(Self { mode, root, file_exists, allowances }))
    }

    /// Hide the errors the file's allowances cover and record its counts.
    ///
    /// A file outside the current directory is left alone and records nothing.
    pub fn apply(&self, path: &Path, diagnostics: &mut Vec<LintDiagnostic>) -> Option<FileRecord> {
        let key = self.key(path)?;
        let allowed = self.allowances.get(&key);
        if allowed.is_none() && self.mode != Mode::SuppressAll {
            return None;
        }
        let mut found = BTreeMap::<&'static str, usize>::new();
        for diagnostic in diagnostics.iter().filter(|diagnostic| is_baselined(diagnostic)) {
            *found.entry(diagnostic.rule_id().as_str()).or_default() += 1;
        }
        let allowance = |rule| allowed.and_then(|allowed| allowed.get(rule)).copied();
        let covered =
            |rule| self.mode == Mode::SuppressAll || found[rule] <= allowance(rule).unwrap_or(0);
        diagnostics.retain(|diagnostic| {
            !is_baselined(diagnostic) || !covered(diagnostic.rule_id().as_str())
        });
        let exceeded =
            found.keys().filter(|rule| allowance(rule).is_some() && !covered(rule)).count();
        Some(FileRecord { key, found, exceeded })
    }

    /// Find obsolete allowances and the contents to write, once every file has
    /// reported.
    ///
    /// Deleted entries are obsolete only within the run's scope. Incomplete
    /// runs report obsolete allowances without writing.
    pub fn settle<E>(
        &self,
        records: Vec<FileRecord>,
        mut in_scope: impl FnMut(&Path) -> Result<bool, E>,
        complete: bool,
    ) -> Result<Settlement, E> {
        let records: BTreeMap<_, _> =
            records.into_iter().map(|record| (record.key.clone(), record)).collect();
        let mut stale = Vec::new();
        for (key, allowed) in &self.allowances {
            if let Some(record) = records.get(key) {
                for (&rule, &allowed) in allowed {
                    let found = record.found.get(rule).copied().unwrap_or(0);
                    if found < allowed {
                        stale.push(Stale { key: key.clone(), rule, allowed, found: Some(found) });
                    }
                }
            } else if self.is_deleted(key) && in_scope(&self.root.join(key))? {
                stale.extend(allowed.iter().map(|(&rule, &allowed)| Stale {
                    key: key.clone(),
                    rule,
                    allowed,
                    found: None,
                }));
            }
        }
        let exceeded = records.values().map(|record| record.exceeded).sum();

        let mut next = self.allowances.clone();
        match self.mode {
            Mode::SuppressAll if complete => {
                for record in records.values() {
                    if record.found.is_empty() {
                        next.remove(&record.key);
                    } else {
                        next.insert(record.key.clone(), record.found.clone());
                    }
                }
                stale.retain(|obsolete| obsolete.found.is_none());
            }
            Mode::Prune if complete => {
                for obsolete in std::mem::take(&mut stale) {
                    let Some(allowed) = next.get_mut(&obsolete.key) else { continue };
                    match obsolete.found {
                        Some(found) if found > 0 => {
                            allowed.insert(obsolete.rule, found);
                        }
                        _ => {
                            allowed.remove(obsolete.rule);
                        }
                    }
                    if allowed.is_empty() {
                        next.remove(&obsolete.key);
                    }
                }
            }
            Mode::Enforce | Mode::Prune | Mode::SuppressAll => {}
        }
        let changed = if self.file_exists { next != self.allowances } else { !next.is_empty() };
        let update = (complete && changed).then_some(next);
        Ok(Settlement { stale, exceeded, update })
    }

    /// Replace the file with `allowances`.
    ///
    /// # Errors
    ///
    /// Returns a message when the file could not be written.
    pub fn write(&self, allowances: &Allowances) -> Result<&'static str, String> {
        std::fs::write(self.root.join(FILE_NAME), serialize(allowances))
            .map_err(|error| format!("failed to write {FILE_NAME}: {error}"))?;
        Ok(if self.file_exists { "updated" } else { "created" })
    }

    /// The path below the current directory with `/` separators, if a key can
    /// spell it.
    fn key(&self, path: &Path) -> Option<String> {
        let relative = path.strip_prefix(&self.root).ok()?;
        let mut segments = Vec::new();
        for component in relative.components() {
            let Component::Normal(name) = component else { return None };
            segments.push(name.to_str().filter(|name| is_key_segment(name))?);
        }
        (!segments.is_empty()).then(|| segments.join("/"))
    }

    /// Whether `key` is provably gone. A permission error is not proof.
    fn is_deleted(&self, key: &str) -> bool {
        std::fs::symlink_metadata(self.root.join(key))
            .is_err_and(|error| error.kind() == io::ErrorKind::NotFound)
    }
}

/// What one analyzed file contributes to the run's verdict on the baseline.
pub struct FileRecord {
    key: String,
    /// Error findings per rule after inline suppressions, before the baseline.
    found: BTreeMap<&'static str, usize>,
    /// Rules with an allowance this file exceeded.
    exceeded: usize,
}

/// The run's verdict on the baseline.
pub struct Settlement {
    /// Obsolete allowances the run did not prune, ordered by file.
    pub stale: Vec<Stale>,
    /// File and rule pairs reported in full because they exceed an allowance.
    pub exceeded: usize,
    /// The allowances to write, when the run changed them.
    pub update: Option<Allowances>,
}

/// One allowance larger than the errors that remain.
pub struct Stale {
    pub key: String,
    pub rule: &'static str,
    allowed: usize,
    /// The errors that remain, or `None` when the file no longer exists.
    found: Option<usize>,
}

impl Stale {
    pub fn message(&self) -> String {
        let Self { key, allowed, .. } = self;
        let errors = if *allowed == 1 { "error" } else { "errors" };
        let remaining = match self.found {
            None => "the file no longer exists".to_string(),
            Some(0) => "none remain".to_string(),
            Some(1) => "only 1 remains".to_string(),
            Some(found) => format!("only {found} remain"),
        };
        format!(
            "{key}: {FILE_NAME} allows {allowed} {errors}, but {remaining}; lower the allowance \
             with `--prune-suppressions`"
        )
    }
}

/// Whether the baseline governs a finding: an error that is not suppression hygiene.
fn is_baselined(diagnostic: &LintDiagnostic) -> bool {
    diagnostic.diagnostic().severity == Severity::Error
        && diagnostic.rule_id() != UNUSED_SUPPRESSION_ID
}

/// Whether `segment` can be one `/`-separated part of a key on this host.
///
/// `\` and `:` are part of a file name except on Windows, where they would
/// separate components or name a drive or stream.
fn is_key_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment != "."
        && segment != ".."
        && !(cfg!(windows) && segment.contains(['\\', ':']))
}

/// The serialized form of one allowance.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Count {
    count: usize,
}

/// A JSON object with unique keys. `serde_json` otherwise keeps the last duplicate.
struct UniqueMap<V>(BTreeMap<String, V>);

impl<'de, V: Deserialize<'de>> Deserialize<'de> for UniqueMap<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct UniqueVisitor<V>(PhantomData<V>);

        impl<'de, V: Deserialize<'de>> Visitor<'de> for UniqueVisitor<V> {
            type Value = UniqueMap<V>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut entries = BTreeMap::new();
                while let Some((key, value)) = map.next_entry::<String, V>()? {
                    if entries.contains_key(&key) {
                        return Err(de::Error::custom(format!("duplicate key `{key}`")));
                    }
                    entries.insert(key, value);
                }
                Ok(UniqueMap(entries))
            }
        }

        deserializer.deserialize_map(UniqueVisitor(PhantomData))
    }
}

fn parse(bytes: &[u8]) -> Result<Allowances, String> {
    let files: UniqueMap<UniqueMap<Count>> =
        serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let mut allowances = Allowances::new();
    for (key, entries) in files.0 {
        if !key.split('/').all(is_key_segment) {
            return Err(format!(
                "`{key}` is not a relative path below the current directory with `/` separators"
            ));
        }
        if entries.0.is_empty() {
            return Err(format!("`{key}` lists no rules"));
        }
        let mut allowed = BTreeMap::new();
        for (rule, Count { count }) in entries.0 {
            let Some(rule) = rules::builtin_rules()
                .iter()
                .map(|builtin| builtin.id().as_str())
                .find(|id| *id == rule)
            else {
                return Err(format!("`{key}`: `{rule}` is not a rule ID"));
            };
            if count == 0 {
                return Err(format!("`{key}`: the count for `{rule}` must be at least 1"));
            }
            allowed.insert(rule, count);
        }
        allowances.insert(key, allowed);
    }
    Ok(allowances)
}

fn serialize(allowances: &Allowances) -> String {
    let files: BTreeMap<_, BTreeMap<_, _>> = allowances
        .iter()
        .map(|(key, allowed)| {
            (key, allowed.iter().map(|(rule, &count)| (*rule, Count { count })).collect())
        })
        .collect();
    let mut contents =
        serde_json::to_string_pretty(&files).expect("string keys and integers always serialize");
    contents.push('\n');
    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_the_serialized_form_and_serialization_is_stable() {
        let text = "{\n  \"src/A.java\": {\n    \"modifier-order\": {\n      \"count\": 1\n    },\n    \"unused-import\": {\n      \"count\": 2\n    }\n  }\n}\n";
        let allowances = parse(text.as_bytes()).expect("well-formed baseline");
        assert_eq!(allowances["src/A.java"]["unused-import"], 2);
        assert_eq!(serialize(&allowances), text);
        assert_eq!(serialize(&Allowances::new()), "{}\n");
    }

    #[test]
    fn parse_rejects_malformed_baselines() {
        for (text, expected) in [
            ("[]", "invalid type"),
            ("{\"A.java\": {\"unused-import\": {\"count\": -1}}}", "invalid value"),
            ("{\"A.java\": {\"unused-import\": {\"count\": 1, \"extra\": 1}}}", "unknown field"),
            ("{\"A.java\": {\"unused-import\": {\"count\": 0}}}", "at least 1"),
            ("{\"A.java\": {}}", "lists no rules"),
            ("{\"A.java\": {\"no-such-rule\": {\"count\": 1}}}", "not a rule ID"),
            ("{\"A.java\": {\"unused-suppression\": {\"count\": 1}}}", "not a rule ID"),
            ("{\"A.java\": {\"unused-import\": {\"count\": 1}}, \"A.java\": {}}", "duplicate key"),
            (
                "{\"A.java\": {\"unused-import\": {\"count\": 1}, \"unused-import\": {\"count\": 2}}}",
                "duplicate key",
            ),
        ] {
            let error = parse(text.as_bytes()).expect_err(text);
            assert!(error.contains(expected), "{text}: {error}");
        }
        for key in ["", "/A.java", "../A.java", "src/../A.java", "./A.java", "src//A.java", "src/"]
        {
            let text = format!("{{\"{key}\": {{\"unused-import\": {{\"count\": 1}}}}}}");
            let error = parse(text.as_bytes()).expect_err(&text);
            assert!(error.contains("not a relative path"), "{key}: {error}");
        }
    }

    #[test]
    fn keys_follow_the_host_path_rules() {
        for key in ["Back\\Slash.java", "Colon:Name.java", "C:/A.java", "src\\..\\A.java"] {
            let allowances =
                Allowances::from([(key.to_string(), BTreeMap::from([("unused-import", 1)]))]);
            let text = serialize(&allowances);
            if cfg!(windows) {
                let error = parse(text.as_bytes()).expect_err(&text);
                assert!(error.contains("not a relative path"), "{key}: {error}");
            } else {
                assert_eq!(parse(text.as_bytes()).as_ref(), Ok(&allowances), "{key}");
            }
        }

        let root = PathBuf::from(if cfg!(windows) { "C:\\project" } else { "/project" });
        let baseline = Baseline {
            mode: Mode::Enforce,
            root: root.clone(),
            file_exists: true,
            allowances: Allowances::new(),
        };
        let expected = if cfg!(windows) { "Back/Slash.java" } else { "Back\\Slash.java" };
        assert_eq!(baseline.key(&root.join("Back\\Slash.java")).as_deref(), Some(expected));
    }
}
