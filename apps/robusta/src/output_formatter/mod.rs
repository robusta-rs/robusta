use std::io::{self, IsTerminal};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use robusta_diagnostics::{
    DiagnosticSummary, FileReport, Finding, Reporter, RobustaDiagnostic, RuleInfo, Severity,
    SourceLocator, SourcePosition, SourceRange,
};
use robusta_linter::{LintDiagnostic, RuleId, RuleTimingRecord, rules};
use rustc_hash::FxHashSet;

use crate::baseline;
use crate::parallel::Sink;

mod agent;
mod azure_devops;
mod checkstyle;
mod default;
mod github;
mod gitlab;
mod json;
mod junit;
mod sarif;
mod stylish;
mod unix;
mod xml_utils;

pub use self::default::DefaultReporter;
pub use self::json::get_all_rules_json;

/// Reserved rule identity for a "file is not formatted" finding in formats
/// that cannot express formatting state except as a rule result.
const FORMAT_RULE: RuleInfo = RuleInfo {
    id: "format",
    summary: Some("Reports files whose contents differ from the project's configured format."),
};
const FORMAT_MESSAGE: &str = "file is not formatted; run `robusta format --fix`";

/// Where a synthesized formatting finding is placed. Formatting state belongs
/// to the whole file, and the first character is the one position every format
/// can address.
const FORMAT_POSITION: SourcePosition = SourcePosition { line: 1, column: 1 };

/// Rule identity for a finding no rule produced: a parse error, or a file the
/// tool could not read, write, or transform. Formats that key on a rule emit
/// the empty identifier, and the human reporter renders no `code:` prefix.
const NO_RULE: RuleInfo = RuleInfo { id: "", summary: None };

/// One reportable item as a format renders it: every field a renderer reads,
/// resolved once, so each format destructures the same struct instead of
/// receiving six positional arguments from its own helper.
///
/// Each item holds its source range, file name, message, severity, rule, and
/// optional help. The file name travels on the item because reporting is per
/// file: a renderer walks files rather than a flat diagnostic stream. The
/// synthetic [`FORMAT_RULE`] item is produced here too, so no format can
/// silently omit it or any field the others emit.
struct Info<'report> {
    start: SourcePosition,
    end: SourcePosition,
    filename: &'report str,
    message: &'report str,
    severity: Severity,
    rule: RuleInfo,
    help: Option<&'report str>,
}

impl<'report> Info<'report> {
    /// Everything one file contributes to a report, in document order: the
    /// formatting finding when the file would be reformatted, then the file's
    /// findings as the run resolved them.
    fn of(report: &'report FileReport) -> impl Iterator<Item = Self> {
        let filename = report.path();
        let formatting = report.would_reformat().then_some(Self {
            start: FORMAT_POSITION,
            end: FORMAT_POSITION,
            filename,
            message: FORMAT_MESSAGE,
            severity: Severity::Warning,
            rule: FORMAT_RULE,
            help: None,
        });
        formatting.into_iter().chain(report.findings().iter().map(move |finding| Self {
            start: finding.range().start,
            end: finding.range().end,
            filename,
            message: finding.message(),
            severity: finding.severity(),
            rule: finding.rule(),
            help: finding.help(),
        }))
    }

    /// Every reportable item in the run, file by file.
    fn of_all(reports: &[&'report FileReport]) -> impl Iterator<Item = Self> {
        reports.iter().flat_map(|report| Self::of(report))
    }

    /// The item's start and exclusive end, for the formats that emit both ends.
    const fn range(&self) -> SourceRange {
        SourceRange { start: self.start, end: self.end }
    }

    /// Whether the producer resolved a source position. Unresolved positions
    /// keep zero coordinates, which the line-oriented formats print as they
    /// stand; only `agent` reads a zero as absence.
    const fn has_position(&self) -> bool {
        self.start.line != 0 && self.start.column != 0
    }
}

/// A report format that buffers the whole run into one document.
///
/// Most of these are machine formats, but the class is not "machine": what its
/// members share is that the run produces exactly one document, written when
/// the run ends. `stylish` is written for a person to read and belongs here all
/// the same.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferedFormat {
    /// Robusta-native JSON with a per-file `formatted` flag and diagnostics.
    Json,
    /// Checkstyle XML, consumable by Checkstyle-aware CI plugins.
    Checkstyle,
    /// GitHub Actions workflow commands, rendered as inline annotations.
    Github,
    /// GitLab Code Quality report JSON.
    Gitlab,
    /// Azure Pipelines logging commands, rendered as source annotations.
    AzureDevOps,
    /// ESLint-compatible, grep-friendly line output with a problem count.
    Unix,
    /// Compact one-line findings for automated coding agents.
    Agent,
    /// SARIF 2.1.0, consumable by code-scanning platforms.
    Sarif,
    /// ESLint's `stylish` layout: an aligned block per file, for a person.
    Stylish,
    /// JUnit XML, consumable by CI test-result readers.
    JUnit,
}

/// Buffering reporter for one selected report format.
pub struct BufferedReporter {
    format: BufferedFormat,
    checks_formatting: bool,
    color: bool,
    reports: Vec<FileReport>,
}

impl BufferedReporter {
    pub const fn new(format: BufferedFormat, checks_formatting: bool, color: bool) -> Self {
        Self { format, checks_formatting, color, reports: Vec::new() }
    }
}

impl Reporter for BufferedReporter {
    fn report(&mut self, report: FileReport) -> Option<String> {
        if report.is_reportable() {
            self.reports.push(report);
        }
        None
    }

    /// Every buffered format carries its own totals inside its document, so the
    /// run summary is not part of the bytes a consumer parses.
    fn finish(&mut self, _summary: DiagnosticSummary) -> Option<String> {
        Some(render(self.format, &self.reports, self.checks_formatting, self.color))
    }
}

/// Render the buffered reports in the selected format.
///
/// `checks_formatting` distinguishes `check` / `format` (which report
/// formatting state) from `lint` (which does not), so summaries and synthetic
/// findings only mention formatting where it was actually inspected. `color`
/// reaches only the formats a person reads; a machine document is never
/// colored.
fn render(
    format: BufferedFormat,
    reports: &[FileReport],
    checks_formatting: bool,
    color: bool,
) -> String {
    let reported: Vec<&FileReport> =
        reports.iter().filter(|report| report.is_reportable()).collect();
    match format {
        BufferedFormat::Json => json::render(&reported, checks_formatting),
        BufferedFormat::Checkstyle => checkstyle::render(&reported),
        BufferedFormat::Github => github::render(&reported),
        BufferedFormat::Gitlab => gitlab::render(&reported),
        BufferedFormat::AzureDevOps => azure_devops::render(&reported),
        BufferedFormat::Unix => unix::render(&reported),
        BufferedFormat::Agent => agent::render(&reported),
        BufferedFormat::Sarif => sarif::render(&reported),
        BufferedFormat::Stylish => stylish::render(&reported, color),
        BufferedFormat::JUnit => junit::render(&reported),
    }
}

/// Pretty-print a serde document with the two-space indentation every machine
/// snapshot pins, plus the trailing newline a complete document ends with.
fn to_pretty_document(document: &impl serde::Serialize) -> String {
    let mut out = serde_json::to_string_pretty(document)
        .expect("report documents contain only serializable plain data");
    out.push('\n');
    out
}

fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Advice => "advice",
    }
}

/// When human-readable output should contain ANSI color escapes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ColorChoice {
    /// Colorize each output stream only when it is a terminal and `NO_COLOR`
    /// is not set to a non-empty value.
    #[default]
    Auto,
    /// Always colorize human-readable output.
    Always,
    /// Never colorize output.
    Never,
}

impl ColorChoice {
    fn enabled(self, is_terminal: bool, no_color: bool) -> bool {
        match self {
            Self::Auto => is_terminal && !no_color,
            Self::Always => true,
            Self::Never => false,
        }
    }
}

impl FromStr for ColorChoice {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            other => {
                Err(format!("unknown color choice `{other}` (expected auto, always, or never)"))
            }
        }
    }
}

/// Resolved color decisions for the two human-readable output streams.
#[derive(Debug, Clone, Copy)]
pub struct ColorPolicy {
    stdout: bool,
    stderr: bool,
}

impl ColorPolicy {
    pub fn resolve(choice: ColorChoice) -> Self {
        let no_color = std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty());
        Self {
            stdout: choice.enabled(io::stdout().is_terminal(), no_color),
            stderr: choice.enabled(io::stderr().is_terminal(), no_color),
        }
    }

    pub const fn stdout(self) -> bool {
        self.stdout
    }

    pub const fn stderr(self) -> bool {
        self.stderr
    }
}

/// The stream a run's report goes to. Stdout carries the product of a run: the
/// report, or the document a preview or a stdin fix produces, in which case the
/// report goes to stderr.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportStream {
    Stdout,
    Stderr,
}

impl ReportStream {
    pub fn write(self, out: &mut impl Sink, text: &str) {
        match self {
            Self::Stdout => out.stdout(text),
            Self::Stderr => out.stderr(text),
        }
    }

    pub const fn color(self, policy: ColorPolicy) -> bool {
        match self {
            Self::Stdout => policy.stdout(),
            Self::Stderr => policy.stderr(),
        }
    }
}

/// Count one file's lint findings into the run's totals.
pub fn summarize(diagnostics: &[LintDiagnostic]) -> DiagnosticSummary {
    count(diagnostics.iter().map(|diagnostic| diagnostic.diagnostic().severity))
}

/// Count the diagnostics behind a file that could not be processed. A parse
/// error joins the totals like a rule result, which is what lets the closing
/// counts describe a run that failed.
pub fn summarize_failure(diagnostics: &[RobustaDiagnostic]) -> DiagnosticSummary {
    count(diagnostics.iter().map(|diagnostic| diagnostic.severity))
}

/// Total diagnostics by severity. Counting happens before output filtering, so
/// `--quiet` and `--silent` never change what the run reports or how it exits.
///
/// Advice joins neither total: the closing line names errors and warnings, and
/// the exit status is decided by those two alone. Advice is still reported.
fn count(severities: impl Iterator<Item = Severity>) -> DiagnosticSummary {
    let mut summary = DiagnosticSummary::default();
    for severity in severities {
        match severity {
            Severity::Error => summary.errors += 1,
            Severity::Warning => summary.warnings += 1,
            Severity::Advice => {}
        }
    }
    summary
}

/// What a finished run did, rendered as the human reporter's closing line.
///
/// The line reports the elapsed time, the file count, the rule count, and the
/// thread count, in that order, with the rules clause dropped when the run
/// resolved no rule set. `rules` is `None` for `format`, which runs none, and
/// for a nested-config run whose files did not all resolve the same rule set:
/// such a run reports no count rather than an arbitrary one.
///
/// `rule_timings` is the one clause that is not a single line. It is `Some`
/// only when `--debug=timings` collected measurements, and — like the closing
/// line itself — reaches only the streaming human reporter: the buffered
/// formats write one machine-readable document, and a timing table is neither
/// part of that document nor a finding it could carry.
#[derive(Debug, Clone)]
pub struct ExecutionSummary {
    pub elapsed: Duration,
    pub files: usize,
    pub rules: Option<usize>,
    pub threads: usize,
    pub rule_timings: Option<Vec<RuleTimingRecord>>,
}

impl ExecutionSummary {
    pub fn render(&self) -> String {
        let Self { elapsed, files, rules, threads, rule_timings } = self;
        let time = format_elapsed(*elapsed);
        let files_plural = plural(*files);
        let mut rendered = match rules {
            Some(rules) => format!(
                "Finished in {time} on {files} file{files_plural} with {rules} rules using {threads} threads."
            ),
            None => {
                format!("Finished in {time} on {files} file{files_plural} using {threads} threads.")
            }
        };
        if let Some(timings) = rule_timings.as_deref().filter(|timings| !timings.is_empty()) {
            rendered.push_str("\n\n");
            rendered.push_str(&render_rule_timings(timings));
        }
        rendered
    }
}

/// The per-rule timing table `--debug=timings` closes a run with.
///
/// Column widths are measured from the rows, so the table stays aligned
/// whatever the longest rule ID is, and the relative share is computed against
/// the measured total rather than the run's wall time: rules are one part of a
/// run, and a share of the whole would read as though they were all of it.
fn render_rule_timings(timings: &[RuleTimingRecord]) -> String {
    use std::fmt::Write;

    const RULE: &str = "Rule";
    const TIME: &str = "Time (ms)";
    const RELATIVE: &str = "Relative";
    const CALLS: &str = "Calls";

    let rule_width =
        timings.iter().map(|record| record.rule.as_str().len()).max().unwrap_or(0).max(RULE.len());
    let calls_width = timings
        .iter()
        .map(|record| record.calls.to_string().len())
        .max()
        .unwrap_or(0)
        .max(CALLS.len());
    let total = timings.iter().map(|record| millis(record.duration)).sum::<f64>();

    let mut table = String::from("Rule timings:\n");
    let time_width = TIME.len();
    let relative_width = RELATIVE.len();
    writeln!(
        table,
        "{RULE:<rule_width$}  {TIME:>time_width$}  {RELATIVE:>relative_width$}  \
         {CALLS:>calls_width$}"
    )
    .expect("writing to a String");
    writeln!(
        table,
        "{:-<rule_width$}  {:-<time_width$}  {:-<relative_width$}  {:-<calls_width$}",
        "", "", "", "",
    )
    .expect("writing to a String");
    for record in timings {
        let elapsed = millis(record.duration);
        let relative = if total > 0.0 { elapsed / total * 100.0 } else { 0.0 };
        writeln!(
            table,
            "{:<rule_width$}  {elapsed:>time_width$.3}  {relative:>7.1}%  {:>calls_width$}",
            record.rule.as_str(),
            record.calls,
        )
        .expect("writing to a String");
    }
    // The caller's line writer supplies the final newline.
    table.pop();
    table
}

/// A duration in milliseconds, the unit the timing table reports.
fn millis(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

/// The suffix English pluralizes a counted noun with for every count but one.
const fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

/// Whole milliseconds below a second, then seconds to one decimal place.
fn format_elapsed(elapsed: Duration) -> String {
    let milliseconds = elapsed.as_millis();
    if milliseconds < 1000 {
        format!("{milliseconds}ms")
    } else {
        format!("{:.1}s", elapsed.as_secs_f64())
    }
}

/// Which lint findings should reach a reporter. Counting happens before this
/// filter is applied.
#[derive(Debug, Clone, Copy)]
pub struct DiagnosticFilter {
    quiet: bool,
    silent: bool,
}

impl DiagnosticFilter {
    pub const fn new(quiet: bool, silent: bool) -> Self {
        Self { quiet, silent }
    }

    fn allows(self, severity: Severity) -> bool {
        !(self.silent || self.quiet && severity == Severity::Warning)
    }
}

/// The report output selected by `--reporter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    /// Stream each diagnostic as its file is processed.
    Default,
    /// Buffer the whole run into one document.
    Buffered(BufferedFormat),
}

/// Every `--reporter` spelling, in the order help text and errors list them.
/// Parsing and the rejection message read the same table, so a format cannot be
/// accepted without being offered.
const REPORT_FORMATS: [(&str, ReportFormat); 11] = [
    ("default", ReportFormat::Default),
    ("json", ReportFormat::Buffered(BufferedFormat::Json)),
    ("checkstyle", ReportFormat::Buffered(BufferedFormat::Checkstyle)),
    ("github", ReportFormat::Buffered(BufferedFormat::Github)),
    ("gitlab", ReportFormat::Buffered(BufferedFormat::Gitlab)),
    ("azure-devops", ReportFormat::Buffered(BufferedFormat::AzureDevOps)),
    ("unix", ReportFormat::Buffered(BufferedFormat::Unix)),
    ("agent", ReportFormat::Buffered(BufferedFormat::Agent)),
    ("sarif", ReportFormat::Buffered(BufferedFormat::Sarif)),
    ("stylish", ReportFormat::Buffered(BufferedFormat::Stylish)),
    ("junit", ReportFormat::Buffered(BufferedFormat::JUnit)),
];

impl ReportFormat {
    pub fn all_rules(self, enabled: &FxHashSet<RuleId>) -> Option<String> {
        match self {
            Self::Default => Some(DefaultReporter::all_rules(enabled)),
            Self::Buffered(BufferedFormat::Json) => Some(json::get_all_rules_json()),
            Self::Buffered(BufferedFormat::Agent) => Some(agent::all_rules(enabled)),
            Self::Buffered(_) => None,
        }
    }

    /// The output of a run that processed no file: no diagnostic totals, only
    /// the execution summary or the `json` document.
    pub fn no_files_report(
        self,
        summary: &ExecutionSummary,
        checks_formatting: bool,
    ) -> Option<String> {
        match self {
            Self::Default => Some(format!("{}\n", summary.render())),
            Self::Buffered(BufferedFormat::Json) => Some(json::render(&[], checks_formatting)),
            Self::Buffered(_) => None,
        }
    }

    /// The buffered format, or `None` for streaming default output.
    pub const fn buffered(self) -> Option<BufferedFormat> {
        match self {
            ReportFormat::Default => None,
            ReportFormat::Buffered(format) => Some(format),
        }
    }

    /// The spelling this format is selected with.
    pub fn name(self) -> &'static str {
        REPORT_FORMATS
            .iter()
            .find_map(|&(name, format)| (format == self).then_some(name))
            .expect("every report format has a spelling")
    }
}

impl FromStr for ReportFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        REPORT_FORMATS
            .iter()
            .find_map(|&(name, format)| (name == value).then_some(format))
            .ok_or_else(|| format!("unknown reporter `{value}` (expected {})", offered_formats()))
    }
}

/// The accepted spellings as a sentence: comma-separated, with `or` before the
/// last one.
fn offered_formats() -> String {
    let mut offered = String::new();
    let last = REPORT_FORMATS.len() - 1;
    for (index, (name, _)) in REPORT_FORMATS.iter().enumerate() {
        if index > 0 {
            offered.push_str(", ");
        }
        if index == last {
            offered.push_str("or ");
        }
        offered.push_str(name);
    }
    offered
}

/// Build a report from a display path, resolved diagnostics, and, for a
/// formatting-bearing scope, whether the file is already formatted.
///
/// A clean file never crosses the reporter boundary. Besides matching the
/// reporter's observable contract, deciding that before constructing the
/// source-backed report avoids scanning and copying source that no renderer
/// can use.
pub fn file_report(
    display_path: &Path,
    source: &str,
    diagnostics: &[LintDiagnostic],
    formatted: Option<bool>,
    filter: DiagnosticFilter,
) -> Option<FileReport> {
    let has_findings =
        diagnostics.iter().any(|diagnostic| filter.allows(diagnostic.diagnostic().severity));
    if !has_findings && formatted != Some(false) {
        return None;
    }

    let findings = if has_findings {
        let locator = SourceLocator::new(source);
        diagnostics
            .iter()
            .filter(|diagnostic| filter.allows(diagnostic.diagnostic().severity))
            .map(|diagnostic| {
                let inner = diagnostic.diagnostic();
                let rule = RuleInfo {
                    id: diagnostic.rule_id().as_str(),
                    summary: rules::find_builtin_rule(diagnostic.rule_id())
                        .map(|rule| rule.meta().summary),
                };
                Finding::from_diagnostic(rule, inner.clone(), locator.range(diagnostic.span()))
            })
            .collect()
    } else {
        Vec::new()
    };
    Some(FileReport::new(display_path, source, formatted, findings))
}

/// Build a report for source the parser rejected.
///
/// A parse error is a finding like any rule result — same reporter, same
/// totals — but it belongs to no rule. It is positioned against the text that
/// was actually parsed, which is the transformed candidate rather than the
/// original once a fix, suppression, or prune has run. Formatting state is
/// unknown: the file never got far enough to be compared.
pub fn parse_failure_report(
    display_path: &Path,
    source: &str,
    errors: &[RobustaDiagnostic],
    filter: DiagnosticFilter,
) -> FileReport {
    let locator = SourceLocator::new(source);
    let findings = errors
        .iter()
        .filter(|error| filter.allows(error.severity))
        .map(|error| {
            Finding::from_diagnostic(NO_RULE, error.clone(), locator.range(error.primary_span()))
        })
        .collect();
    FileReport::new(display_path, source, None, findings)
}

/// Build a report for a file the tool could not read, write, or transform.
///
/// The failure is about the file rather than a place in it, and the source is
/// unavailable, so the finding carries no position and the report no excerpt.
/// The path travels in the message as well as in the report, because the human
/// reporter has no source excerpt to head with it.
pub fn tool_failure_report(
    display_path: &Path,
    message: &str,
    filter: DiagnosticFilter,
) -> FileReport {
    let findings = filter
        .allows(Severity::Error)
        .then(|| Finding::without_position(NO_RULE, Severity::Error, message))
        .into_iter()
        .collect();
    FileReport::new(display_path, "", None, findings)
}

/// Build a report for baseline allowances one file no longer needs.
///
/// An allowance belongs to a file and a rule rather than to a place in the
/// file, so like a tool failure each finding carries no position and names the
/// path in its message.
pub fn stale_allowance_report(
    display_path: &Path,
    stale: &[baseline::Stale],
    filter: DiagnosticFilter,
) -> FileReport {
    let findings = stale
        .iter()
        .filter(|_| filter.allows(Severity::Error))
        .map(|stale| {
            let rule = RuleInfo {
                id: stale.rule,
                summary: rules::find_builtin_rule(RuleId::new(stale.rule))
                    .map(|rule| rule.meta().summary),
            };
            Finding::without_position(rule, Severity::Error, stale.message())
        })
        .collect();
    FileReport::new(display_path, "", None, findings)
}

#[cfg(test)]
mod tests {
    use robusta_linter::RuleId;
    use robusta_span::Span;

    use super::test_support::position;
    use super::*;

    #[test]
    fn report_format_parses_known_values_and_rejects_others() {
        assert_eq!("json".parse(), Ok(ReportFormat::Buffered(BufferedFormat::Json)));
        assert_eq!("checkstyle".parse(), Ok(ReportFormat::Buffered(BufferedFormat::Checkstyle)));
        assert_eq!("github".parse(), Ok(ReportFormat::Buffered(BufferedFormat::Github)));
        assert_eq!("gitlab".parse(), Ok(ReportFormat::Buffered(BufferedFormat::Gitlab)));
        assert_eq!("azure-devops".parse(), Ok(ReportFormat::Buffered(BufferedFormat::AzureDevOps)));
        assert_eq!("unix".parse(), Ok(ReportFormat::Buffered(BufferedFormat::Unix)));
        assert_eq!("agent".parse(), Ok(ReportFormat::Buffered(BufferedFormat::Agent)));
        assert_eq!("sarif".parse(), Ok(ReportFormat::Buffered(BufferedFormat::Sarif)));
        assert_eq!("stylish".parse(), Ok(ReportFormat::Buffered(BufferedFormat::Stylish)));
        assert_eq!("junit".parse(), Ok(ReportFormat::Buffered(BufferedFormat::JUnit)));
        assert_eq!("default".parse(), Ok(ReportFormat::Default));
        assert!("human".parse::<ReportFormat>().is_err());
        assert!("Stylish".parse::<ReportFormat>().is_err());
    }

    /// Every accepted spelling is offered back on rejection, and every format
    /// can name itself for a message that has to identify one.
    #[test]
    fn every_report_format_is_offered_and_can_name_itself() {
        let rejected = "human".parse::<ReportFormat>().expect_err("`human` is not a reporter");
        for (name, format) in REPORT_FORMATS {
            assert!(rejected.contains(name), "`{name}` is missing from `{rejected}`");
            assert_eq!(format.name(), name);
        }
        assert!(rejected.ends_with("or junit)"), "{rejected}");
    }

    #[test]
    fn execution_summary_drops_the_rule_clause_and_pluralizes_the_file_count() {
        let summary = ExecutionSummary {
            elapsed: Duration::from_millis(5),
            files: 1,
            rules: Some(10),
            threads: 12,
            rule_timings: None,
        };
        assert_eq!(summary.render(), "Finished in 5ms on 1 file with 10 rules using 12 threads.");

        let unknown_rules = ExecutionSummary { files: 5, rules: None, ..summary };
        assert_eq!(unknown_rules.render(), "Finished in 5ms on 5 files using 12 threads.");
    }

    fn timing(rule: &'static str, micros: u64, calls: u64) -> RuleTimingRecord {
        RuleTimingRecord { rule: RuleId::new(rule), duration: Duration::from_micros(micros), calls }
    }

    /// The table closes the same line the summary opens, is aligned to its own
    /// longest row, and states each rule's share of the measured total rather
    /// than of the run.
    #[test]
    fn the_timing_table_follows_the_summary_and_sizes_its_columns_to_its_rows() {
        let summary = ExecutionSummary {
            elapsed: Duration::from_millis(5),
            files: 2,
            rules: Some(2),
            threads: 4,
            rule_timings: Some(vec![
                timing("a-very-long-rule-name", 1500, 12_345),
                timing("short", 500, 7),
            ]),
        };

        insta::assert_snapshot!(summary.render(), @r"
        Finished in 5ms on 2 files with 2 rules using 4 threads.

        Rule timings:
        Rule                   Time (ms)  Relative  Calls
        ---------------------  ---------  --------  -----
        a-very-long-rule-name      1.500     75.0%  12345
        short                      0.500     25.0%      7
        ");
    }

    /// A run that measured nothing prints no table: an empty one would say the
    /// rules were free rather than that none ran.
    #[test]
    fn an_empty_timing_table_is_not_printed() {
        let summary = ExecutionSummary {
            elapsed: Duration::from_millis(5),
            files: 1,
            rules: Some(0),
            threads: 1,
            rule_timings: Some(Vec::new()),
        };
        assert_eq!(summary.render(), "Finished in 5ms on 1 file with 0 rules using 1 threads.");
    }

    /// Rules that were dispatched but measured below the clock's resolution
    /// still get a row, and the shares stay at zero rather than dividing by it.
    #[test]
    fn a_zero_duration_table_reports_zero_shares() {
        let summary = ExecutionSummary {
            elapsed: Duration::from_millis(5),
            files: 1,
            rules: Some(1),
            threads: 1,
            rule_timings: Some(vec![timing("idle", 0, 0)]),
        };

        insta::assert_snapshot!(summary.render(), @r"
        Finished in 5ms on 1 file with 1 rules using 1 threads.

        Rule timings:
        Rule  Time (ms)  Relative  Calls
        ----  ---------  --------  -----
        idle      0.000      0.0%      0
        ");
    }

    #[test]
    fn elapsed_time_switches_to_seconds_at_one_second() {
        assert_eq!(format_elapsed(Duration::from_millis(999)), "999ms");
        assert_eq!(format_elapsed(Duration::from_secs(1)), "1.0s");
        assert_eq!(format_elapsed(Duration::from_millis(1250)), "1.2s");
    }

    /// A parse error is positioned and counted like a rule finding, but carries
    /// no rule identity and leaves formatting state unknown.
    #[test]
    fn a_parse_failure_reports_positioned_ruleless_findings() {
        let source = "class Broken {\n    int ;\n}\n";
        let errors = vec![RobustaDiagnostic::error("expected a member name", Span::new(24, 24))];

        let report = parse_failure_report(
            Path::new("Broken.java"),
            source,
            &errors,
            DiagnosticFilter::new(false, false),
        );

        assert_eq!(report.formatted(), None);
        assert!(report.is_reportable());
        let [finding] = report.findings() else { panic!("one error, one finding") };
        assert_eq!(finding.rule_id(), "");
        assert!(finding.has_position());
        assert_eq!(finding.range().start, position(2, 10));
        assert_eq!(summarize_failure(&errors), DiagnosticSummary { warnings: 0, errors: 1 });
    }

    /// Advice is reported but is not a problem the closing line counts, and so
    /// cannot move the exit status either.
    #[test]
    fn advice_joins_neither_run_total() {
        let severities = [Severity::Advice, Severity::Warning, Severity::Advice, Severity::Error];

        assert_eq!(count(severities.into_iter()), DiagnosticSummary { warnings: 1, errors: 1 });
        assert_eq!(severity_label(Severity::Advice), "advice");
    }

    /// A tool-side failure names a file it never opened, so it has no position
    /// and no excerpt to point into.
    #[test]
    fn a_tool_failure_reports_one_positionless_finding() {
        let report = tool_failure_report(
            Path::new("Locked.java"),
            "Locked.java: Permission denied",
            DiagnosticFilter::new(false, false),
        );

        let [finding] = report.findings() else { panic!("a tool failure is one finding") };
        assert_eq!(finding.rule_id(), "");
        assert_eq!(finding.severity(), Severity::Error);
        assert!(!finding.has_position());
        assert!(finding.message().starts_with("Locked.java: "));
    }

    /// `--silent` withholds a failure from the output exactly as it withholds a
    /// rule finding. The totals are counted before this point, so the run still
    /// reports and exits on what it found.
    #[test]
    fn silent_withholds_a_failure_from_every_report() {
        let silent = DiagnosticFilter::new(false, true);
        let errors = vec![RobustaDiagnostic::error("expected a member name", Span::empty(0))];

        let parse = parse_failure_report(Path::new("Broken.java"), "class {}", &errors, silent);
        let tool = tool_failure_report(Path::new("Locked.java"), "denied", silent);

        assert!(parse.findings().is_empty());
        assert!(!parse.is_reportable());
        assert!(tool.findings().is_empty());
        assert!(!tool.is_reportable());
    }

    /// The run loop may submit exceptional reports after filtering removed
    /// their findings. The buffered boundary must not retain their source.
    #[test]
    fn buffered_reporter_discards_unreportable_reports_on_submission() {
        let mut reporter = BufferedReporter::new(BufferedFormat::Json, false, false);
        reporter.report(FileReport::new(Path::new("Clean.java"), "class Clean {}", None, vec![]));

        assert!(reporter.reports.is_empty());
    }

    #[test]
    fn clean_file_does_not_build_a_source_backed_report() {
        let report = file_report(
            Path::new("Clean.java"),
            "class Clean {}",
            &[],
            None,
            DiagnosticFilter::new(false, false),
        );

        assert!(report.is_none());
    }

    #[test]
    fn color_choice_parses_known_values_and_resolves_precedence() {
        assert_eq!("auto".parse(), Ok(ColorChoice::Auto));
        assert_eq!("always".parse(), Ok(ColorChoice::Always));
        assert_eq!("never".parse(), Ok(ColorChoice::Never));
        assert!("sometimes".parse::<ColorChoice>().is_err());

        assert!(ColorChoice::Auto.enabled(true, false));
        assert!(!ColorChoice::Auto.enabled(false, false));
        assert!(!ColorChoice::Auto.enabled(true, true));
        assert!(ColorChoice::Always.enabled(false, true));
        assert!(!ColorChoice::Never.enabled(true, false));
    }
}

#[cfg(test)]
mod test_support {
    use std::path::Path;

    use robusta_diagnostics::{
        FileReport, Finding, RobustaDiagnostic, RuleInfo, Severity, SourcePosition, SourceRange,
    };

    pub(super) const DEMO_RULE: RuleInfo =
        RuleInfo { id: "demo", summary: Some("Reports demonstration findings.") };
    pub(super) const RULELESS: RuleInfo = super::NO_RULE;
    pub(super) const UNDESCRIBED_RULE: RuleInfo = RuleInfo { id: "undescribed", summary: None };

    pub(super) fn report_with(
        path: &str,
        formatted: Option<bool>,
        findings: Vec<Finding>,
    ) -> FileReport {
        FileReport::new(Path::new(path), "", formatted, findings)
    }

    pub(super) fn position(line: usize, column: usize) -> SourcePosition {
        SourcePosition { line, column }
    }

    pub(super) fn finding(
        rule: RuleInfo,
        severity: Severity,
        message: &str,
        line: usize,
        column: usize,
    ) -> Finding {
        ranged_finding(rule, severity, message, position(line, column), position(line, column))
    }

    /// A finding covering a real region, for the formats that emit an end.
    pub(super) fn ranged_finding(
        rule: RuleInfo,
        severity: Severity,
        message: &str,
        start: SourcePosition,
        end: SourcePosition,
    ) -> Finding {
        Finding::new(
            rule,
            severity,
            message,
            robusta_span::Span::empty(0),
            SourceRange { start, end },
        )
    }

    pub(super) fn unpositioned_finding(
        rule: RuleInfo,
        severity: Severity,
        message: &str,
    ) -> Finding {
        Finding::without_position(rule, severity, message)
    }

    pub(super) fn finding_with_help(
        rule: RuleInfo,
        severity: Severity,
        message: &str,
        help: &str,
        line: usize,
        column: usize,
    ) -> Finding {
        let span = robusta_span::Span::empty(0);
        Finding::from_diagnostic(
            rule,
            RobustaDiagnostic::warning(message, span).with_severity(severity).with_help(help),
            SourceRange::point(position(line, column)),
        )
    }
}
