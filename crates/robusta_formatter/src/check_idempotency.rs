use crate::{Options, format};
use robusta_diagnostics::RobustaDiagnostic;

/// The result of formatting already-formatted source one more time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyReport {
    reformatted: String,
    stable: bool,
}

impl IdempotencyReport {
    fn new(formatted: &str, reformatted: String) -> Self {
        let stable = reformatted == formatted;
        Self { reformatted, stable }
    }

    /// Whether the second pass is byte-identical to the first.
    #[must_use]
    pub const fn is_stable(&self) -> bool {
        self.stable
    }

    /// The second pass, retained so callers can render a useful diff.
    #[must_use]
    pub fn reformatted(&self) -> &str {
        &self.reformatted
    }

    /// The shared user-facing failure text for an unstable second pass.
    #[must_use]
    pub fn failure_message(&self) -> Option<String> {
        (!self.stable).then(|| "formatter is not idempotent".to_string())
    }
}

/// Format already-formatted source once more.
///
/// # Errors
///
/// Returns the first parse diagnostic of `formatted`.
pub fn check_idempotency(
    formatted: &str,
    options: &Options,
) -> Result<IdempotencyReport, RobustaDiagnostic> {
    let reformatted = format(formatted, options)?;
    Ok(IdempotencyReport::new(formatted, reformatted))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idempotency_keeps_a_changed_second_pass() {
        let report = IdempotencyReport::new("class A {}\n", "class B {}\n".to_string());

        assert!(!report.is_stable());
        assert_eq!(report.reformatted(), "class B {}\n");
        assert_eq!(report.failure_message().as_deref(), Some("formatter is not idempotent"));
    }

    #[test]
    fn idempotency_forwards_the_parse_failure() {
        assert!(check_idempotency("class A {\n", &Options::default()).is_err());
    }
}
