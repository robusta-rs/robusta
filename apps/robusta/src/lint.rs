use std::fmt;
use std::io::{self, Read};
use std::path::Path;
use std::time::Instant;

use robusta_allocator::{Allocator, AllocatorPool};
use robusta_diagnostics::{
    DiagnosticSummary, FileReport, Output, Reporter, RobustaDiagnostic, Severity,
};
use robusta_formatter::Options;
use robusta_linter::{FixKind, LintDiagnostic, LintOptions, Linter, RuleTimingStore, apply_fixes};
use robusta_semantic::{Semantic, SemanticBuilder};

use crate::baseline::{self, Baseline};
use crate::output_formatter::{DiagnosticFilter, ExecutionSummary, ReportFormat, ReportStream};
use crate::parallel::{Buffer, Sink};
use crate::result::CliRunResult;
use crate::run::{FormatExecution, format_source, verify_formatted_source};
use crate::walk::{SelectedFile, selected_files};
use crate::{command, mode, output_formatter, parallel, walk};

/// Whether a failed file is the source's fault or the tool's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fault {
    Source,
    Tool,
}

impl Fault {
    /// A tool failure outranks a source failure.
    const fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Source, Self::Source) => Self::Source,
            _ => Self::Tool,
        }
    }

    const fn result(self) -> CliRunResult {
        match self {
            Self::Source => CliRunResult::SourceParseFailed,
            Self::Tool => CliRunResult::FileProcessingFailed,
        }
    }
}

/// A failure that ends processing of one file.
pub enum FileFailure {
    /// Java the parser rejected. Once a fix has run, `source` is the fixed text
    /// the errors point into.
    Unparsable { source: String, errors: Vec<RobustaDiagnostic> },
    /// A read, write, or transformation the tool could not carry out.
    Tool { message: String },
}

impl FileFailure {
    pub(super) fn unparsable(source: &str, errors: Vec<RobustaDiagnostic>) -> Self {
        Self::Unparsable { source: source.to_owned(), errors }
    }

    fn tool(path: &Path, error: &impl fmt::Display) -> Self {
        Self::Tool { message: format!("{}: {error}", path.display()) }
    }

    const fn fault(&self) -> Fault {
        match self {
            Self::Unparsable { .. } => Fault::Source,
            Self::Tool { .. } => Fault::Tool,
        }
    }

    fn summary(&self) -> DiagnosticSummary {
        match self {
            Self::Unparsable { errors, .. } => output_formatter::summarize_failure(errors),
            Self::Tool { .. } => DiagnosticSummary { warnings: 0, errors: 1 },
        }
    }

    /// The failure's report and the totals it adds to the run.
    fn report(
        &self,
        display_path: &Path,
        filter: DiagnosticFilter,
    ) -> (FileReport, DiagnosticSummary) {
        let report = match self {
            Self::Unparsable { source, errors } => {
                output_formatter::parse_failure_report(display_path, source, errors, filter)
            }
            Self::Tool { message } => {
                output_formatter::tool_failure_report(display_path, message, filter)
            }
        };
        (report, self.summary())
    }
}

/// What a run does with source and the baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Execution {
    Check(FixExecution, baseline::Mode),
    Format(FormatExecution),
    Lint(FixExecution, baseline::Mode),
}

impl Execution {
    const fn checks_formatting(self) -> bool {
        matches!(self, Self::Check(..) | Self::Format(_))
    }

    const fn selection_features(self) -> walk::SelectionFeatures {
        match self {
            Self::Check(..) => walk::SelectionFeatures::CHECK,
            Self::Format(_) => walk::SelectionFeatures::FORMAT,
            Self::Lint(..) => walk::SelectionFeatures::LINT,
        }
    }

    /// Whether the run leaves source alone and shows no preview.
    const fn is_verify(self) -> bool {
        matches!(
            self,
            Self::Check(FixExecution::Verify, _)
                | Self::Format(FormatExecution::Verify)
                | Self::Lint(FixExecution::Verify, _)
        )
    }

    const fn is_write(self) -> bool {
        matches!(
            self,
            Self::Check(FixExecution::Write { .. }, _)
                | Self::Format(FormatExecution::Write)
                | Self::Lint(FixExecution::Write { .. }, _)
        )
    }

    const fn preview_output(self) -> Option<PreviewOutput> {
        match self {
            Self::Check(FixExecution::Preview { output, .. }, _)
            | Self::Format(FormatExecution::Preview(output))
            | Self::Lint(FixExecution::Preview { output, .. }, _) => Some(output),
            Self::Check(FixExecution::Verify | FixExecution::Write { .. }, _)
            | Self::Format(FormatExecution::Verify | FormatExecution::Write)
            | Self::Lint(FixExecution::Verify | FixExecution::Write { .. }, _) => None,
        }
    }

    /// The baseline mode, for a run that lints.
    const fn baseline(self) -> Option<baseline::Mode> {
        match self {
            Self::Check(_, mode) | Self::Lint(_, mode) => Some(mode),
            Self::Format(_) => None,
        }
    }
}

/// What `check` and `lint` do with source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixExecution {
    Verify,
    Preview { output: PreviewOutput, fix_kind: FixKind },
    Write { fix_kind: FixKind },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewOutput {
    Diff,
    ListDifferent,
}

impl PreviewOutput {
    pub(super) const fn from_change_output(output: command::ChangeOutput) -> Option<Self> {
        match output {
            command::ChangeOutput::Default => None,
            command::ChangeOutput::Diff => Some(Self::Diff),
            command::ChangeOutput::ListDifferent => Some(Self::ListDifferent),
        }
    }

    const fn flag(self) -> &'static str {
        match self {
            Self::Diff => "--diff",
            Self::ListDifferent => "--list-different",
        }
    }
}

/// Resolve the fix, preview, and baseline flags of `check` and `lint`.
pub fn rule_execution(
    out: &mut Output,
    fix_options: command::FixOptions,
    output: command::ChangeOutput,
    suppress_all: bool,
    prune_suppressions: bool,
) -> Result<(FixExecution, baseline::Mode), CliRunResult> {
    let preview = PreviewOutput::from_change_output(output);
    let fix_kind = fix_options.kind();
    let fix = match preview {
        Some(output) if fix_options.fix => {
            return Err(redundant_safe_preview(out, output));
        }
        Some(output) => FixExecution::Preview { output, fix_kind },
        None if fix_options.fix => FixExecution::Write { fix_kind },
        None if fix_options.unsafe_fixes => {
            out.stderr_line(
                "Error: `--unsafe-fixes` requires `--fix`, `--diff`, or `--list-different`",
            );
            return Err(CliRunResult::InvalidOptionUnsafeFixesWithoutFix);
        }
        None => FixExecution::Verify,
    };
    let (mode, flag) = match (suppress_all, prune_suppressions) {
        (false, false) => return Ok((fix, baseline::Mode::Enforce)),
        (true, true) => {
            out.stderr_line(
                "Error: `--suppress-all` cannot be combined with `--prune-suppressions`",
            );
            return Err(CliRunResult::InvalidOptionSuppressAllWithPrune);
        }
        (true, false) => (baseline::Mode::SuppressAll, "--suppress-all"),
        (false, true) => (baseline::Mode::Prune, "--prune-suppressions"),
    };
    // A preview promises to write nothing, and both flags write the baseline.
    if let Some(output) = preview {
        let output_flag = output.flag();
        out.stderr_line(&format!("Error: `{flag}` cannot be combined with `{output_flag}`"));
        return Err(CliRunResult::InvalidOptionBaselineUpdateWithPreview);
    }
    Ok((fix, mode))
}

pub fn redundant_safe_preview(out: &mut Output, output: PreviewOutput) -> CliRunResult {
    let output_flag = output.flag();
    out.stderr_line(&format!("Error: `--fix` cannot be used at the same time as `{output_flag}`"));
    CliRunResult::InvalidOptionFixWithPreview
}

/// The first option that cannot be combined with `--stdin-filepath`. A stdin run has
/// no file tree to select from, writes no files, and already uses stdout for its result.
fn stdin_conflict(
    input: &command::Input,
    ignore_options: &command::IgnoreOptions,
    runtime_options: &command::RuntimeOptions,
    execution: Execution,
) -> Option<String> {
    if let Some(path) = input.paths.first() {
        return Some(path.display().to_string());
    }
    let flag = if !ignore_options.ignore_path.is_empty() {
        "--ignore-path"
    } else if !ignore_options.ignore_pattern.is_empty() {
        "--ignore-pattern"
    } else if runtime_options.threads.is_some() {
        "--threads"
    } else if let Some(output) = execution.preview_output() {
        output.flag()
    } else {
        match execution.baseline() {
            Some(baseline::Mode::SuppressAll) => "--suppress-all",
            Some(baseline::Mode::Prune) => "--prune-suppressions",
            Some(baseline::Mode::Enforce) | None => return None,
        }
    };
    Some(flag.to_owned())
}

fn diagnostic_filter(options: &command::WarningOptions) -> DiagnosticFilter {
    DiagnosticFilter::new(options.quiet, options.silent)
}

/// The outcome of completed lint totals, whose failures all exit with `1`.
fn finish_diagnostics(
    out: &mut Output,
    summary: DiagnosticSummary,
    options: &command::WarningOptions,
    other_work_remains: bool,
) -> CliRunResult {
    if options.max_warnings.is_some_and(|maximum| summary.warnings > maximum) {
        if !options.silent {
            out.stderr_line(&format!(
                "Exceeded maximum number of warnings. Found {}.",
                summary.warnings
            ));
        }
        CliRunResult::MaxWarningsExceeded
    } else if summary.errors > 0 {
        CliRunResult::FindingsReported
    } else if other_work_remains {
        CliRunResult::UnappliedWorkRemains
    } else if options.deny_warnings && summary.warnings > 0 {
        CliRunResult::WarningsDenied
    } else {
        CliRunResult::RunSucceeded
    }
}

struct FileOutcome {
    report: Option<FileReport>,
    diagnostics: DiagnosticSummary,
    work_remains: bool,
    /// A processing failure, already reported and counted in `diagnostics`.
    fault: Option<Fault>,
    /// What an analyzed file contributes to the verdict on the baseline.
    baseline: Option<baseline::FileRecord>,
}

/// What every file in one run is processed and reported with.
struct FileRun<'run> {
    execution: Execution,
    report_unused_suppressions: Option<Severity>,
    rule_timings: Option<&'run RuleTimingStore>,
    baseline: Option<&'run Baseline>,
    /// The document read under `--stdin-filepath`, in place of the file's contents.
    stdin: Option<&'run str>,
    reporter: ReportFormat,
    report_stream: ReportStream,
    warning_options: &'run command::WarningOptions,
    color: output_formatter::ColorPolicy,
    allocator_pool: AllocatorPool,
}

impl FileRun<'_> {
    fn filter(&self) -> DiagnosticFilter {
        diagnostic_filter(self.warning_options)
    }

    /// Render the report now, or return it for the buffered reporter.
    fn route(
        &self,
        out: &mut impl Sink,
        report: FileReport,
        stream: ReportStream,
    ) -> Option<FileReport> {
        if self.reporter != ReportFormat::Default {
            return Some(report);
        }
        let color = stream.color(self.color);
        if let Some(rendered) = output_formatter::DefaultReporter::new(color).report(report) {
            stream.write(out, &rendered);
        }
        None
    }
}

/// The run's totals as file outcomes arrive.
struct RunReport {
    diagnostics: DiagnosticSummary,
    work_remains: bool,
    fault: Option<Fault>,
    buffered: Option<output_formatter::BufferedReporter>,
    baseline_records: Vec<baseline::FileRecord>,
}

impl RunReport {
    fn absorb(&mut self, out: &mut Output, buffer: &Buffer, outcome: FileOutcome) {
        buffer.flush_to(out);
        self.diagnostics.merge(outcome.diagnostics);
        self.work_remains |= outcome.work_remains;
        if let (Some(reporter), Some(report)) = (&mut self.buffered, outcome.report) {
            reporter.report(report);
        }
        if let Some(file_fault) = outcome.fault {
            self.fault = Some(self.fault.map_or(file_fault, |first| first.merge(file_fault)));
        }
        self.baseline_records.extend(outcome.baseline);
    }
}

/// The options of `check`, `format`, or `lint`, with their resolved execution.
pub struct SourceCommand {
    pub input: command::Input,
    pub config_options: command::ConfigOptions,
    pub ignore_options: command::IgnoreOptions,
    pub filters: Vec<robusta_linter::LintFilter>,
    pub execution: Execution,
    /// `lint --rules`: list the rules instead of running them.
    pub list_rules: bool,
    pub report_unused_suppressions: command::ReportUnusedSuppressions,
    pub reporter: ReportFormat,
    pub debug: command::DebugOptions,
    pub warning_options: command::WarningOptions,
    pub runtime_options: command::RuntimeOptions,
    pub color: output_formatter::ColorPolicy,
}

pub struct CliRunner {
    options: SourceCommand,
}

impl CliRunner {
    pub const fn new(options: SourceCommand) -> Self {
        Self { options }
    }

    pub fn run(self, out: &mut Output) -> CliRunResult {
        let SourceCommand {
            input,
            config_options,
            ignore_options,
            filters,
            execution,
            list_rules,
            report_unused_suppressions,
            reporter,
            debug,
            warning_options,
            runtime_options,
            color,
        } = self.options;

        let started = Instant::now();
        if input.stdin_filepath.is_some()
            && let Some(conflict) =
                stdin_conflict(&input, &ignore_options, &runtime_options, execution)
        {
            out.stderr_line(&format!(
                "Error: `--stdin-filepath` cannot be combined with `{conflict}`"
            ));
            return CliRunResult::InvalidOptionWithStdin;
        }
        if list_rules {
            return mode::run_rules(out, &config_options, &filters, reporter);
        }
        let execution = match execution {
            Execution::Check(fix @ FixExecution::Write { .. }, baseline::Mode::Enforce)
                if input.stdin_filepath.is_none() =>
            {
                Execution::Check(fix, baseline::Mode::Prune)
            }
            Execution::Lint(fix @ FixExecution::Write { .. }, baseline::Mode::Enforce)
                if input.stdin_filepath.is_none() =>
            {
                Execution::Lint(fix, baseline::Mode::Prune)
            }
            _ => execution,
        };
        parallel::initialize(runtime_options.threads);
        let thread_count = parallel::thread_count();
        let Ok(ignore_matcher) = walk::ignore_matcher(out, &ignore_options) else {
            return CliRunResult::FileSelectionFailed;
        };
        let features = execution.selection_features();
        let files = match &input.stdin_filepath {
            Some(path) => walk::stdin_file(out, path, &config_options, features, &filters),
            None => selected_files(
                out,
                &input.paths,
                &config_options,
                &ignore_matcher,
                runtime_options.no_error_on_unmatched_pattern,
                features,
                &filters,
                thread_count,
            ),
        };
        let Ok(walk::Selection { files, run_options, mut scope }) = files else {
            return CliRunResult::FileSelectionFailed;
        };
        // `--deny-warnings` adds to the main configuration, and `--max-warnings` replaces it.
        let warning_options = &command::WarningOptions {
            deny_warnings: warning_options.deny_warnings || run_options.deny_warnings,
            max_warnings: warning_options.max_warnings.or(run_options.max_warnings),
            ..warning_options
        };
        let stdin = match input.stdin_filepath.as_ref().map(|_| read_stdin(out)).transpose() {
            Ok(stdin) => stdin,
            Err(result) => return result,
        };
        // A preview or a stdin fix writes to stdout, so the report goes to stderr.
        let writes_stdin = stdin.is_some() && execution.is_write();
        let report_stream = if writes_stdin || execution.preview_output().is_some() {
            ReportStream::Stderr
        } else {
            ReportStream::Stdout
        };
        let report_no_files = |out: &mut Output| {
            let summary = ExecutionSummary {
                elapsed: started.elapsed(),
                files: 0,
                rules: None,
                threads: thread_count,
                rule_timings: None,
            };
            if !warning_options.silent
                && let Some(report) =
                    reporter.no_files_report(&summary, execution.checks_formatting())
            {
                report_stream.write(out, &report);
            }
        };
        if files.is_empty() {
            // An excluded document passes through unchanged.
            if writes_stdin && let Some(source) = &stdin {
                out.stdout(source);
            }
            if stdin.is_some() || execution.baseline() != Some(baseline::Mode::Prune) {
                report_no_files(out);
                return CliRunResult::NoFilesSelected;
            }
        }
        // Validate the baseline before any source is written.
        let baseline = match execution.baseline().map(Baseline::load).transpose() {
            Ok(baseline) => baseline.flatten(),
            Err(message) => {
                out.stderr_line(&message);
                return CliRunResult::InvalidSuppressionBaseline;
            }
        };

        // Without a store, rule dispatch reads no clock.
        let rule_timings = debug.contains(command::DebugOption::Timings).then(RuleTimingStore::new);
        let run = FileRun {
            execution,
            report_unused_suppressions: report_unused_suppressions
                .severity(run_options.report_unused_suppressions),
            rule_timings: rule_timings.as_ref(),
            baseline: baseline.as_ref(),
            stdin: stdin.as_deref(),
            reporter,
            report_stream,
            warning_options,
            color,
            allocator_pool: AllocatorPool::new(thread_count),
        };
        let results = parallel::map_files(&files, |file| {
            let mut buffer = Buffer::default();
            let outcome = process_one(&mut buffer, file, &run);
            (buffer, outcome)
        });

        let mut totals = RunReport {
            diagnostics: DiagnosticSummary::default(),
            work_remains: false,
            fault: None,
            buffered: reporter.buffered().map(|format| {
                let color = report_stream.color(color);
                output_formatter::BufferedReporter::new(
                    format,
                    execution.checks_formatting(),
                    color,
                )
            }),
            baseline_records: Vec::new(),
        };
        for (buffer, outcome) in results {
            totals.absorb(out, &buffer, outcome);
        }
        if let Some(baseline) = &baseline {
            settle_baseline(out, baseline, &mut scope, &run, &mut totals);
        }
        if files.is_empty() {
            report_no_files(out);
            return completed_result(
                out,
                totals.diagnostics,
                warning_options,
                totals.work_remains,
                totals.fault,
            );
        }
        let RunReport { diagnostics, work_remains, fault, buffered: mut buffered_reporter, .. } =
            totals;
        if !warning_options.silent
            && let Some(reporter) = &mut buffered_reporter
            && let Some(document) = reporter.finish(diagnostics)
        {
            report_stream.write(out, &document);
        }
        if reporter != ReportFormat::Default || warning_options.silent {
            return completed_result(out, diagnostics, warning_options, work_remains, fault);
        }

        // Totals first, then any `--max-warnings` line, then the execution summary.
        if let Some(totals) =
            output_formatter::DefaultReporter::new(report_stream.color(color)).finish(diagnostics)
        {
            report_stream.write(out, &totals);
        }
        let result = completed_result(out, diagnostics, warning_options, work_remains, fault);
        let summary = ExecutionSummary {
            elapsed: started.elapsed(),
            files: files.len(),
            rules: resolved_rule_count(&files),
            threads: thread_count,
            rule_timings: rule_timings.as_ref().map(RuleTimingStore::collect),
        };
        report_stream.write(out, &format!("{}\n", summary.render()));
        result
    }
}

/// Report obsolete allowances as findings and, after a complete run, write the
/// updated baseline.
fn settle_baseline(
    out: &mut Output,
    baseline: &Baseline,
    scope: &mut walk::SelectionScope,
    run: &FileRun<'_>,
    totals: &mut RunReport,
) {
    let complete = totals.fault.is_none();
    let records = std::mem::take(&mut totals.baseline_records);
    let Ok(settlement) = baseline.settle(records, |path| scope.lints(out, path), complete) else {
        totals.fault = Some(Fault::Tool);
        return;
    };

    for stale in settlement.stale.chunk_by(|left, right| left.key == right.key) {
        let mut buffer = Buffer::default();
        let summary = DiagnosticSummary { warnings: 0, errors: stale.len() };
        let report = (!run.warning_options.silent)
            .then(|| {
                output_formatter::stale_allowance_report(
                    Path::new(&stale[0].key),
                    stale,
                    run.filter(),
                )
            })
            .and_then(|report| run.route(&mut buffer, report, run.report_stream));
        let outcome = FileOutcome {
            report,
            diagnostics: summary,
            work_remains: false,
            fault: None,
            baseline: None,
        };
        totals.absorb(out, &buffer, outcome);
    }
    let silent = run.warning_options.silent;
    if settlement.exceeded > 0 && !silent {
        let (allowances, their) =
            if settlement.exceeded == 1 { ("allowance", "its") } else { ("allowances", "their") };
        out.stderr_line(&format!(
            "{}: {} {allowances} exceeded, so all of {their} findings are reported",
            baseline::FILE_NAME,
            settlement.exceeded
        ));
    }
    if let Some(allowances) = settlement.update {
        match baseline.write(&allowances) {
            Ok(action) if !silent => out.stderr_line(&format!("{}: {action}", baseline::FILE_NAME)),
            Ok(_) => {}
            Err(message) => {
                out.stderr_line(&message);
                totals.fault = Some(Fault::Tool);
            }
        }
    } else if !complete && run.execution.baseline() != Some(baseline::Mode::Enforce) {
        out.stderr_line(&format!(
            "{}: not updated, because the run did not complete",
            baseline::FILE_NAME
        ));
    }
}

/// The run's outcome: a processing fault outranks the totals, which still print
/// any `--max-warnings` line first.
fn completed_result(
    out: &mut Output,
    diagnostics: DiagnosticSummary,
    options: &command::WarningOptions,
    other_work_remains: bool,
    fault: Option<Fault>,
) -> CliRunResult {
    let completed = finish_diagnostics(out, diagnostics, options, other_work_remains);
    fault.map_or(completed, Fault::result)
}

/// The rule count, unless the run lints no file or nested configs disagree.
fn resolved_rule_count(files: &[SelectedFile]) -> Option<usize> {
    let mut counts = files.iter().filter_map(SelectedFile::linter).map(Linter::rule_count);
    let first = counts.next()?;
    counts.all(|count| count == first).then_some(first)
}

#[derive(Debug, Clone, Copy)]
enum DiagnosticSource {
    Original,
    Candidate,
}

struct ProcessedFile {
    original: String,
    candidate: String,
    diagnostics: Vec<LintDiagnostic>,
    diagnostic_source: DiagnosticSource,
}

impl ProcessedFile {
    fn diagnostic_source(&self) -> &str {
        match self.diagnostic_source {
            DiagnosticSource::Original => &self.original,
            DiagnosticSource::Candidate => &self.candidate,
        }
    }
}

fn process_one(out: &mut impl Sink, file: &SelectedFile, run: &FileRun<'_>) -> FileOutcome {
    match processed_outcome(out, file, run) {
        Ok(outcome) => outcome,
        Err(failure) => failed_outcome(out, file, &failure, run),
    }
}

/// Report a processing failure, keeping tool explanations on stderr.
fn failed_outcome(
    out: &mut impl Sink,
    file: &SelectedFile,
    failure: &FileFailure,
    run: &FileRun<'_>,
) -> FileOutcome {
    let fault = Some(failure.fault());
    if run.warning_options.silent {
        let diagnostics = failure.summary();
        return FileOutcome {
            report: None,
            diagnostics,
            work_remains: false,
            fault,
            baseline: None,
        };
    }
    let (report, diagnostics) = failure.report(&file.display_path, run.filter());
    let stream = if let FileFailure::Tool { message } = failure {
        if run.reporter != ReportFormat::Default && run.report_stream == ReportStream::Stdout {
            out.stderr(&format!("Error: {message}\n"));
        }
        ReportStream::Stderr
    } else {
        run.report_stream
    };
    let report = run.route(out, report, stream);
    FileOutcome { report, diagnostics, work_remains: false, fault, baseline: None }
}

fn processed_outcome(
    out: &mut impl Sink,
    file: &SelectedFile,
    run: &FileRun<'_>,
) -> Result<FileOutcome, FileFailure> {
    let execution = run.execution;
    let mut processed = process_file(file, run)?;
    let changed = processed.candidate != processed.original;
    if execution.is_write() && run.stdin.is_some() {
        // A stdin document goes to stdout even when unchanged.
        out.stdout(&processed.candidate);
    } else if execution.is_write() && changed {
        std::fs::write(&file.path, &processed.candidate)
            .map_err(|error| FileFailure::tool(&file.display_path, &error))?;
    }
    // Filter after fixing, so an allowance never blocks a fix. A file the run
    // does not lint says nothing about its allowances.
    let baseline = run
        .baseline
        .filter(|_| file.linter().is_some())
        .and_then(|baseline| baseline.apply(&file.path, &mut processed.diagnostics));

    let diagnostics = output_formatter::summarize(&processed.diagnostics);
    if run.reporter == ReportFormat::Default
        && let Some(report) = output_formatter::file_report(
            &file.display_path,
            processed.diagnostic_source(),
            &processed.diagnostics,
            None,
            run.filter(),
        )
    {
        run.route(out, report, run.report_stream);
    }
    report_source_change(out, file, run, &processed, changed);

    let formats = file.format_options().is_some();
    let report = run.reporter.buffered().and_then(|_| {
        let formatted = formats.then_some(execution.is_write() || !changed);
        output_formatter::file_report(
            &file.display_path,
            processed.diagnostic_source(),
            &processed.diagnostics,
            formatted,
            run.filter(),
        )
    });
    let work_remains =
        changed && !execution.is_write() && (formats || execution.preview_output().is_some());
    Ok(FileOutcome { report, diagnostics, work_remains, fault: None, baseline })
}

fn process_file(file: &SelectedFile, run: &FileRun<'_>) -> Result<ProcessedFile, FileFailure> {
    let original = match run.stdin {
        Some(source) => source.to_owned(),
        None => read_source(&file.path, &file.display_path)?,
    };
    // Every file under `format`, and a file `check` formats but does not lint.
    let (Execution::Check(fix, _) | Execution::Lint(fix, _), Some(linter)) =
        (run.execution, file.linter())
    else {
        let options = file.format_options().expect("selection drops a file no tool processes");
        let candidate = format_source(&run.allocator_pool, &original, options)?;
        return Ok(ProcessedFile {
            original,
            candidate,
            diagnostics: Vec::new(),
            diagnostic_source: DiagnosticSource::Candidate,
        });
    };
    let pass = LintPass {
        linter,
        report_unused_suppressions: run.report_unused_suppressions,
        timings: run.rule_timings,
        allocator_pool: &run.allocator_pool,
    };
    let options = file.format_options();
    let fix_kind = match (fix, options) {
        (FixExecution::Verify, Some(options)) => {
            let analysis = analyze_source(&original, options, pass)?;
            return Ok(ProcessedFile {
                original,
                candidate: analysis.formatted,
                diagnostics: analysis.diagnostics,
                diagnostic_source: DiagnosticSource::Original,
            });
        }
        (FixExecution::Verify, None) => {
            let diagnostics = lint_source(&original, pass)?;
            return Ok(ProcessedFile {
                candidate: original.clone(),
                original,
                diagnostics,
                diagnostic_source: DiagnosticSource::Original,
            });
        }
        (FixExecution::Preview { fix_kind, .. } | FixExecution::Write { fix_kind }, _) => fix_kind,
    };
    let (candidate, diagnostics) = match options {
        Some(options) => {
            transform_check_source(&file.display_path, &original, options, pass, fix_kind)?
        }
        None => transform_lint_source(&file.display_path, &original, pass, fix_kind)?,
    };
    Ok(ProcessedFile {
        original,
        candidate,
        diagnostics,
        diagnostic_source: DiagnosticSource::Candidate,
    })
}

fn report_source_change(
    out: &mut impl Sink,
    file: &SelectedFile,
    run: &FileRun<'_>,
    processed: &ProcessedFile,
    changed: bool,
) {
    if !changed {
        return;
    }
    // A buffered report carries the formatting state itself.
    if run.execution.is_verify()
        && run.execution.checks_formatting()
        && run.reporter == ReportFormat::Default
        && !run.warning_options.silent
    {
        output_formatter::DefaultReporter::print_would_reformat(out, &file.display_path);
    }
    match run.execution.preview_output() {
        Some(PreviewOutput::Diff) => {
            output_formatter::DefaultReporter::print_unified_diff(
                out,
                &file.display_path,
                &processed.original,
                &processed.candidate,
                run.color.stdout(),
            );
        }
        Some(PreviewOutput::ListDifferent) => {
            out.stdout_line(&file.display_path.display().to_string());
        }
        None => {}
    }
}

fn transform_lint_source(
    path: &Path,
    source: &str,
    pass: LintPass<'_>,
    fix_kind: FixKind,
) -> Result<(String, Vec<LintDiagnostic>), FileFailure> {
    let diagnostics = lint_source(source, pass)?;
    let fixed = apply_fixes(source, &diagnostics, fix_kind)
        .map_err(|error| FileFailure::tool(path, &error))?;
    let candidate = fixed.code.into_owned();
    if candidate == source {
        Ok((candidate, diagnostics))
    } else {
        let remaining = lint_source(&candidate, pass)?;
        Ok((candidate, remaining))
    }
}

fn transform_check_source(
    path: &Path,
    source: &str,
    options: &Options,
    pass: LintPass<'_>,
    fix_kind: FixKind,
) -> Result<(String, Vec<LintDiagnostic>), FileFailure> {
    let diagnostics = lint_source(source, pass)?;
    let fixed = apply_fixes(source, &diagnostics, fix_kind)
        .map_err(|error| FileFailure::tool(path, &error))?;
    // Formatting reparses, so it validates the fixed code before any write.
    let candidate = format_source(pass.allocator_pool, fixed.code.as_ref(), options)?;
    let remaining = lint_source(&candidate, pass)?;
    Ok((candidate, remaining))
}

struct Analysis {
    formatted: String,
    diagnostics: Vec<LintDiagnostic>,
}

fn analyze_source(
    source: &str,
    options: &Options,
    pass: LintPass<'_>,
) -> Result<Analysis, FileFailure> {
    with_parsed_source(pass.allocator_pool, source, |allocator, result| {
        let unit = &result.unit;
        let diagnostics = if pass.linter.is_empty() && pass.report_unused_suppressions.is_none() {
            Vec::new()
        } else {
            pass.run(&SemanticBuilder::new().build(unit))
        };
        let formatted = robusta_formatter::format_compilation_unit(allocator, unit, options);
        verify_formatted_source(source, &formatted, options);
        Ok(Analysis { formatted, diagnostics })
    })?
}

/// Run `operation` over a cleanly parsed tree, in an arena leased from
/// `allocator_pool` that holds the tree and returns to the pool afterwards.
///
/// A recovered tree fails too: formatting and linting need source the parser
/// accepted whole.
pub fn with_parsed_source<T>(
    allocator_pool: &AllocatorPool,
    source: &str,
    operation: impl FnOnce(&Allocator, &robusta_parser::ParseResult<'_>) -> T,
) -> Result<T, FileFailure> {
    let allocator = allocator_pool.get();
    let result = robusta_parser::parse(&allocator, source);
    if result.status != robusta_parser::ParseStatus::Clean {
        return Err(FileFailure::unparsable(source, result.errors));
    }
    Ok(operation(&allocator, &result))
}

fn read_stdin(out: &mut Output) -> Result<String, CliRunResult> {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).map_err(|error| {
        out.stderr_line(&format!("failed to read stdin: {error}"));
        CliRunResult::StdinReadFailed
    })?;
    Ok(source)
}

// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/apps/oxfmt/src/core/utils.rs
fn read_source(path: &Path, display_path: &Path) -> Result<String, FileFailure> {
    let bytes = std::fs::read(path).map_err(|error| FileFailure::tool(display_path, &error))?;
    if simdutf8::basic::from_utf8(&bytes).is_err() {
        return Err(FileFailure::tool(
            display_path,
            &io::Error::new(io::ErrorKind::InvalidData, "stream did not contain valid UTF-8"),
        ));
    }
    // SAFETY: simdutf8 validated these exact bytes, which have not been modified.
    Ok(unsafe { String::from_utf8_unchecked(bytes) })
}

/// What every lint pass over one file shares.
#[derive(Clone, Copy)]
struct LintPass<'run> {
    linter: &'run Linter,
    report_unused_suppressions: Option<Severity>,
    timings: Option<&'run RuleTimingStore>,
    allocator_pool: &'run AllocatorPool,
}

impl LintPass<'_> {
    fn run(self, semantic: &Semantic<'_>) -> Vec<LintDiagnostic> {
        self.linter.lint_with_options(
            semantic,
            LintOptions {
                report_unused_suppressions: self.report_unused_suppressions,
                timings: self.timings,
            },
        )
    }
}

fn lint_source(source: &str, pass: LintPass<'_>) -> Result<Vec<LintDiagnostic>, FileFailure> {
    with_parsed_source(pass.allocator_pool, source, |_, result| {
        pass.run(&SemanticBuilder::new().build(&result.unit))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_parse_returns_an_empty_arena_for_the_next_file() {
        let pool = AllocatorPool::new(1);

        let used = with_parsed_source(&pool, "class A {}\n", |allocator, _| allocator.used_bytes());
        assert!(used.is_ok_and(|used| used > 0));
        let retained = {
            let arena = pool.get();
            assert_eq!(arena.used_bytes(), 0);
            arena.capacity()
        };
        assert!(retained > 0);

        let failed = with_parsed_source(&pool, "class {\n", |_, _| ());
        assert!(matches!(failed, Err(FileFailure::Unparsable { .. })));
        let arena = pool.get();
        assert_eq!(arena.used_bytes(), 0);
        assert_eq!(arena.capacity(), retained);
    }

    fn resolve(
        fix_options: command::FixOptions,
        output: command::ChangeOutput,
        suppress_all: bool,
        prune: bool,
    ) -> Result<(FixExecution, baseline::Mode), CliRunResult> {
        rule_execution(&mut Output::stdio(), fix_options, output, suppress_all, prune)
    }

    #[test]
    fn rules_resolve_verification_and_each_fix_ceiling() {
        let enforce = baseline::Mode::Enforce;
        for (fix, unsafe_fixes) in [(false, false), (true, false), (false, true), (true, true)] {
            let options = command::FixOptions { fix, unsafe_fixes };
            let fix_kind = options.kind();
            let write = match (fix, unsafe_fixes) {
                (true, _) => Ok((FixExecution::Write { fix_kind }, enforce)),
                (false, true) => Err(CliRunResult::InvalidOptionUnsafeFixesWithoutFix),
                (false, false) => Ok((FixExecution::Verify, enforce)),
            };
            assert_eq!(resolve(options, command::ChangeOutput::Default, false, false), write);
            for (flag, output) in [
                (command::ChangeOutput::Diff, PreviewOutput::Diff),
                (command::ChangeOutput::ListDifferent, PreviewOutput::ListDifferent),
            ] {
                let expected = if fix {
                    Err(CliRunResult::InvalidOptionFixWithPreview)
                } else {
                    Ok((FixExecution::Preview { output, fix_kind }, enforce))
                };
                assert_eq!(resolve(options, flag, false, false), expected);
            }
        }
    }

    /// Baseline updates combine with both fix ceilings, but not with each other or a preview.
    #[test]
    fn rules_resolve_baseline_updates() {
        for (fix, unsafe_fixes) in [(false, false), (true, false), (true, true)] {
            let options = command::FixOptions { fix, unsafe_fixes };
            let fix = if fix {
                FixExecution::Write { fix_kind: options.kind() }
            } else {
                FixExecution::Verify
            };
            assert_eq!(
                resolve(options, command::ChangeOutput::Default, true, false),
                Ok((fix, baseline::Mode::SuppressAll)),
            );
            assert_eq!(
                resolve(options, command::ChangeOutput::Default, false, true),
                Ok((fix, baseline::Mode::Prune)),
            );
            assert_eq!(
                resolve(options, command::ChangeOutput::Default, true, true),
                Err(CliRunResult::InvalidOptionSuppressAllWithPrune),
            );
        }
        for output in [command::ChangeOutput::Diff, command::ChangeOutput::ListDifferent] {
            for (suppress_all, prune) in [(true, false), (false, true)] {
                assert_eq!(
                    resolve(
                        command::FixOptions { fix: false, unsafe_fixes: false },
                        output,
                        suppress_all,
                        prune,
                    ),
                    Err(CliRunResult::InvalidOptionBaselineUpdateWithPreview),
                );
            }
        }
    }
}
