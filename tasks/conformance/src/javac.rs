use std::{
    ffi::OsString,
    io,
    path::{Path, PathBuf},
    process::Command,
};

use cow_utils::CowUtils;

/// A resolved `javac` executable.
#[derive(Debug, Clone)]
pub struct Javac {
    program: PathBuf,
}

/// The result of one compilation attempt.
#[derive(Debug)]
pub struct CompileResult {
    pub success: bool,
    /// Combined stdout and stderr, trimmed, for failure reporting.
    pub diagnostics: String,
}

impl Javac {
    /// Resolve the `javac` to use: `$JAVA_HOME/bin/javac` if that file
    /// exists, else `javac` from `PATH`.
    pub fn resolve() -> Self {
        if let Some(java_home) = std::env::var_os("JAVA_HOME") {
            let candidate = Path::new(&java_home).join("bin").join(javac_file_name());
            if candidate.is_file() {
                return Self { program: candidate };
            }
        }
        Self { program: PathBuf::from("javac") }
    }

    /// Report the compiler's version string, proving the executable runs.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when `javac` cannot be executed.
    pub fn version(&self) -> io::Result<String> {
        let output = Command::new(&self.program).arg("--version").output()?;
        let text = String::from_utf8_lossy(&output.stdout);
        let text = if text.trim().is_empty() {
            String::from_utf8_lossy(&output.stderr).trim().to_owned()
        } else {
            text.trim().to_owned()
        };
        Ok(text)
    }

    /// Whether this compiler accepts `--release release` at all, distinguishing
    /// an unsupported source level from a real fixture failure.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when `javac` cannot be executed.
    pub fn supports_release(&self, release: u16, dir: &Path) -> io::Result<bool> {
        let result = self.compile("class RobustaReleaseProbe {}\n", release, dir)?;
        Ok(result.success)
    }

    /// Compile `source` at `--release release`, writing class files under `dir`.
    ///
    /// The fixture is written to a package-private `.java` file, so the file
    /// name need not match the class name.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the source cannot be written or `javac` cannot
    /// be executed. A non-zero compiler exit is reported in the result, not as
    /// an error.
    pub fn compile(&self, source: &str, release: u16, dir: &Path) -> io::Result<CompileResult> {
        let source_path = dir.join("Fixture.java");
        std::fs::write(&source_path, source)?;
        let mut command = self.command(release, dir)?;
        command.arg(&source_path);
        run(&mut command)
    }

    /// Compile the UTF-8 `files` under `dir/src`, given relative to it, against
    /// `classpath`. Diagnostics name the files as given.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the argument file cannot be written or `javac`
    /// cannot be executed.
    pub fn compile_tree(
        &self,
        dir: &Path,
        files: &[&str],
        release: u16,
        classpath: &[PathBuf],
    ) -> io::Result<CompileResult> {
        // An argument file keeps thousands of paths under the command-line limit.
        let mut sources = String::new();
        for file in files {
            sources.push('"');
            sources.push_str(&file.cow_replace('\\', "\\\\"));
            sources.push_str("\"\n");
        }
        let argument_file = dir.join("sources.txt");
        std::fs::write(&argument_file, sources)?;
        let mut argument = OsString::from("@");
        argument.push(argument_file);

        let mut command = self.command(release, dir)?;
        // `group_errors` reads English diagnostics.
        command
            .current_dir(dir.join("src"))
            .args(["-J-Duser.language=en", "-encoding", "UTF-8", "-proc:none", "-nowarn"])
            .arg(argument)
            .arg("-classpath")
            .arg(std::env::join_paths(classpath).map_err(io::Error::other)?);
        run(&mut command)
    }

    /// A `javac --release release` command writing class files under `dir/classes`.
    fn command(&self, release: u16, dir: &Path) -> io::Result<Command> {
        let classes = dir.join("classes");
        std::fs::create_dir_all(&classes)?;
        let mut command = Command::new(&self.program);
        command.arg("--release").arg(release.to_string()).arg("-d").arg(classes);
        Ok(command)
    }
}

fn run(command: &mut Command) -> io::Result<CompileResult> {
    let output = command.output()?;
    let mut diagnostics = String::from_utf8_lossy(&output.stdout).into_owned();
    diagnostics.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(CompileResult {
        success: output.status.success(),
        diagnostics: diagnostics.trim().to_owned(),
    })
}

fn javac_file_name() -> &'static str {
    if cfg!(windows) { "javac.exe" } else { "javac" }
}

/// Split `javac` output into the errors of each named file, in report order.
/// Output naming no file stays whole under an empty name.
pub fn group_errors(diagnostics: &str) -> Vec<(String, String)> {
    let mut groups: Vec<(String, String)> = Vec::new();
    let mut current = None;
    for line in diagnostics.lines() {
        if let Some(file) = error_file(line) {
            let index = groups.iter().position(|(name, _)| name == file).unwrap_or_else(|| {
                groups.push((file.to_owned(), String::new()));
                groups.len() - 1
            });
            current = Some(index);
        } else if ends_error(line) {
            current = None;
        }
        if let Some(index) = current {
            let errors = &mut groups[index].1;
            if !errors.is_empty() {
                errors.push('\n');
            }
            errors.push_str(line);
        }
    }
    if groups.is_empty() {
        groups.push((String::new(), diagnostics.to_owned()));
    }
    groups
}

/// The file of a `File.java:12: error: message` line.
fn error_file(line: &str) -> Option<&str> {
    let (location, _) = line.split_once(": error: ")?;
    let (file, number) = location.rsplit_once(':')?;
    (!number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())).then_some(file)
}

/// Whether `line` starts output that belongs to no error: a warning, a note, or
/// the final `N errors` count.
fn ends_error(line: &str) -> bool {
    line.contains(": warning: ")
        || line.starts_with("warning: ")
        || line.starts_with("Note: ")
        || line.split_once(' ').is_some_and(|(count, word)| {
            count.bytes().all(|byte| byte.is_ascii_digit()) && matches!(word, "error" | "errors")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_group_per_file_without_warnings_or_counts() {
        let output = "\
p/A.java:3: error: cannot find symbol
    List<String> names;
    ^
  symbol:   class List
p/B.java:9: warning: [removal] Thread.stop() has been deprecated
p/A.java:5: error: missing return statement
    }
    ^
2 errors";
        let groups = group_errors(output);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].0, "p/A.java");
        assert_eq!(
            groups[0].1,
            "p/A.java:3: error: cannot find symbol\n    List<String> names;\n    ^\n  \
             symbol:   class List\np/A.java:5: error: missing return statement\n    }\n    ^"
        );
    }

    #[test]
    fn output_naming_no_file_stays_whole() {
        let groups = group_errors("error: invalid flag: --bogus");
        assert_eq!(groups, [(String::new(), "error: invalid flag: --bogus".to_owned())]);
    }
}
