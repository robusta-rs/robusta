use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use robusta_config::CONFIG_FILE_NAME;
use robusta_diagnostics::Output;

use crate::result::CliRunResult;

const SCAFFOLD: &str = concat!(
    "# yaml-language-server: $schema=https://raw.githubusercontent.com/robusta-rs/robusta/v",
    env!("CARGO_PKG_VERSION"),
    "/robusta.schema.json\n",
    r#"# Robusta configuration.
format:
  # Target maximum line width, measured in Unicode display columns (1..=320).
  line-width: 100
  # Structural indentation: space or tab.
  indent-style: space
  # Display columns per indentation level and per tab in tab mode (0..=24).
  indent-width: 2
  # Placement of else, catch, and finally: same-line or new-line.
  after-closing-brace: same-line
  # Import-kind sections followed by package-prefix groups.
  import-order:
    sections: [module, type, static]
    groups:
      - ["java", "javax"]
      - "*"
  # Traditional (/** ... */) and Markdown (///) Javadoc: false, true, or a mapping of settings.
  javadoc: false
  # Output line endings: lf, crlf, or cr.
  line-ending: lf
  # End non-empty formatted files with one line terminator.
  insert-final-newline: true

lint:
  # Correctness rules warn by default; other categories are allowed.
  categories:
    correctness: warn
  # Per-rule severities override categories.
  rules: {}
  # Example:
  # rules:
  #   redundant-semicolons: warn
  #   uppercase-long-suffix: deny
"#
);

pub fn run_init(out: &mut Output) -> CliRunResult {
    let path = Path::new(CONFIG_FILE_NAME);
    match write_scaffold(path) {
        Ok(()) => {
            out.stdout_line(&format!("Created {CONFIG_FILE_NAME}"));
            CliRunResult::ConfigFileCreated
        }
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            out.stderr_line(&format!("{CONFIG_FILE_NAME} already exists"));
            CliRunResult::ConfigFileExists
        }
        Err(error) => {
            out.stderr_line(&format!("failed to create {CONFIG_FILE_NAME}: {error}"));
            CliRunResult::ConfigFileCreationFailed
        }
    }
}

fn write_scaffold(path: &Path) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    if let Err(error) = file.write_all(SCAFFOLD.as_bytes()) {
        drop(file);
        let _ = fs::remove_file(path);
        return Err(error);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::SCAFFOLD;
    use crate::config_loader::{Config, parse_config};

    #[test]
    fn scaffold_parses_to_the_typed_defaults() {
        let scaffold = parse_config(SCAFFOLD).expect("scaffold should be valid robusta.yaml");

        assert_eq!(scaffold, Config::default());
    }
}
