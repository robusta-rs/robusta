use std::process::{ExitCode, Termination};

/// One run's terminal outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliRunResult {
    /// Usage or version text was requested and printed.
    HelpPrinted,
    /// `lint --rules` listed the built-in registry.
    RulesListed,
    /// Selection completed and matched no file to process.
    NoFilesSelected,
    /// Every selected file was processed with nothing left outstanding.
    RunSucceeded,
    /// `config` printed the resolved configuration.
    ConfigPrinted,
    /// `config --files` listed the files `check` would process.
    FilesListed,
    /// `init` wrote the scaffold.
    ConfigFileCreated,

    /// Rule violations of error severity were reported.
    FindingsReported,
    /// Formatting or fixes remain unapplied after a verification or preview.
    UnappliedWorkRemains,
    /// `--deny-warnings` was set and warnings were reported.
    WarningsDenied,
    /// Reported warnings exceeded the `--max-warnings` budget.
    MaxWarningsExceeded,
    /// A file under analysis could not be parsed.
    SourceParseFailed,

    /// The command line could not be parsed.
    InvalidArguments,
    /// `--fix` was combined with `--diff` or `--list-different`.
    InvalidOptionFixWithPreview,
    /// `--unsafe-fixes` was passed without `--fix`, `--diff`, or `--list-different`.
    InvalidOptionUnsafeFixesWithoutFix,
    /// `--suppress-all` was combined with `--prune-suppressions`.
    InvalidOptionSuppressAllWithPrune,
    /// A baseline update was combined with `--diff` or `--list-different`.
    InvalidOptionBaselineUpdateWithPreview,
    /// `--stdin-filepath` was combined with file selection, a preview, or a
    /// baseline update.
    InvalidOptionWithStdin,
    /// Configuration or path resolution failed while selecting files.
    FileSelectionFailed,
    /// `robusta-suppressions.json` could not be read or is malformed.
    InvalidSuppressionBaseline,
    /// Standard input could not be read.
    StdinReadFailed,
    /// A selected file could not be read, transformed, or written, or the
    /// suppression baseline could not be written.
    FileProcessingFailed,
    /// Configuration resolution failed for `config`.
    ConfigResolutionFailed,
    /// The resolved configuration could not be serialized.
    ConfigSerializationFailed,
    /// `init` found an existing file.
    ConfigFileExists,
    /// `init` could not write the scaffold.
    ConfigFileCreationFailed,
}

impl Termination for CliRunResult {
    // Source failures exit 1. Invocation, configuration, and I/O failures exit 2.
    fn report(self) -> ExitCode {
        match self {
            Self::HelpPrinted
            | Self::RulesListed
            | Self::NoFilesSelected
            | Self::RunSucceeded
            | Self::ConfigPrinted
            | Self::FilesListed
            | Self::ConfigFileCreated => ExitCode::SUCCESS,
            Self::FindingsReported
            | Self::UnappliedWorkRemains
            | Self::WarningsDenied
            | Self::MaxWarningsExceeded
            | Self::SourceParseFailed => ExitCode::FAILURE,
            Self::InvalidArguments
            | Self::InvalidOptionFixWithPreview
            | Self::InvalidOptionUnsafeFixesWithoutFix
            | Self::InvalidOptionSuppressAllWithPrune
            | Self::InvalidOptionBaselineUpdateWithPreview
            | Self::InvalidOptionWithStdin
            | Self::FileSelectionFailed
            | Self::InvalidSuppressionBaseline
            | Self::StdinReadFailed
            | Self::FileProcessingFailed
            | Self::ConfigResolutionFailed
            | Self::ConfigSerializationFailed
            | Self::ConfigFileExists
            | Self::ConfigFileCreationFailed => ExitCode::from(2),
        }
    }
}
