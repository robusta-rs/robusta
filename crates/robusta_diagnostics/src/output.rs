use std::io::{self, ErrorKind, Write};
use std::process::ExitCode;

/// Stdout and stderr for a command. Unlike the `print!` family, a failed write does not panic.
pub struct Output {
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
    failure: Option<String>,
}

impl Output {
    pub fn stdio() -> Self {
        Self::new(
            Box::new(io::BufWriter::new(io::stdout())),
            Box::new(io::BufWriter::new(io::stderr())),
        )
    }

    fn new(stdout: Box<dyn Write>, stderr: Box<dyn Write>) -> Self {
        Self { stdout, stderr, failure: None }
    }

    /// Write to stdout without a trailing newline.
    pub fn stdout(&mut self, text: &str) {
        let result = write_and_flush(&mut self.stdout, text, "");
        self.record("stdout", result);
    }

    /// Write one line to stdout.
    pub fn stdout_line(&mut self, line: &str) {
        let result = write_and_flush(&mut self.stdout, line, "\n");
        self.record("stdout", result);
    }

    /// Write one line to stderr.
    pub fn stderr_line(&mut self, line: &str) {
        let result = write_and_flush(&mut self.stderr, line, "\n");
        self.record("stderr", result);
    }

    /// Write to stderr without a trailing newline.
    pub fn stderr(&mut self, text: &str) {
        let result = write_and_flush(&mut self.stderr, text, "");
        self.record("stderr", result);
    }

    /// Fold any recorded write failure into the command's exit code.
    pub fn finish(mut self, code: ExitCode) -> ExitCode {
        let Some(failure) = self.failure.take() else {
            return code;
        };
        // Best effort: the failed stream may be stderr itself.
        let _ = write_and_flush(&mut self.stderr, &failure, "\n");
        ExitCode::from(2)
    }

    fn record(&mut self, stream: &str, result: io::Result<()>) {
        let Err(error) = result else {
            return;
        };
        // A closed pipe (`robusta ... | head`) ends output without changing the exit code.
        if matches!(
            error.kind(),
            ErrorKind::Interrupted | ErrorKind::BrokenPipe | ErrorKind::WouldBlock
        ) {
            return;
        }
        if self.failure.is_none() {
            self.failure = Some(format!("failed to write {stream}: {error}"));
        }
    }
}

fn write_and_flush(writer: &mut Box<dyn Write>, text: &str, ending: &str) -> io::Result<()> {
    writer.write_all(text.as_bytes())?;
    writer.write_all(ending.as_bytes())?;
    writer.flush()
}

#[cfg(test)]
mod tests {
    use std::io::{self, ErrorKind, Write};
    use std::process::ExitCode;
    use std::sync::{Arc, Mutex};

    use super::Output;

    struct FailingWriter(ErrorKind);

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(self.0))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct SharedBuffer(Arc<Mutex<Vec<u8>>>);

    impl SharedBuffer {
        fn contents(&self) -> String {
            String::from_utf8(self.0.lock().expect("buffer lock").clone())
                .expect("buffer holds UTF-8")
        }
    }

    impl Write for SharedBuffer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.lock().expect("buffer lock").extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn writes_reach_both_streams() {
        let stdout = SharedBuffer::default();
        let stderr = SharedBuffer::default();
        let mut output = Output::new(Box::new(stdout.clone()), Box::new(stderr.clone()));

        output.stdout("raw");
        output.stdout_line(" and line");
        output.stderr_line("diagnostic");

        assert_eq!(output.finish(ExitCode::SUCCESS), ExitCode::SUCCESS);
        assert_eq!(stdout.contents(), "raw and line\n");
        assert_eq!(stderr.contents(), "diagnostic\n");
    }

    // `Interrupted` is untested: an always-interrupted writer would loop in `write_all`.
    #[test]
    fn broken_pipe_ends_output_without_changing_the_exit_code() {
        for kind in [ErrorKind::BrokenPipe, ErrorKind::WouldBlock] {
            let stderr = SharedBuffer::default();
            let mut output = Output::new(Box::new(FailingWriter(kind)), Box::new(stderr.clone()));

            output.stdout_line("dropped by the consumer");

            assert_eq!(output.finish(ExitCode::FAILURE), ExitCode::FAILURE);
            assert_eq!(stderr.contents(), "");
        }
    }

    #[test]
    fn other_stdout_failures_exit_operationally() {
        let stderr = SharedBuffer::default();
        let mut output =
            Output::new(Box::new(FailingWriter(ErrorKind::StorageFull)), Box::new(stderr.clone()));

        output.stdout_line("lost");

        assert_eq!(output.finish(ExitCode::SUCCESS), ExitCode::from(2));
        assert!(stderr.contents().starts_with("failed to write stdout: "), "{}", stderr.contents());
    }

    #[test]
    fn stderr_failures_exit_operationally_without_panicking() {
        let mut output = Output::new(
            Box::new(SharedBuffer::default()),
            Box::new(FailingWriter(ErrorKind::StorageFull)),
        );

        output.stderr_line("lost");

        assert_eq!(output.finish(ExitCode::SUCCESS), ExitCode::from(2));
    }
}
