use std::io::{self, ErrorKind, Write};
use std::process::ExitCode;

/// Stdout and stderr for a command. Unlike the `print!` family, a failed write does not panic.
pub struct Output {
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
    /// The stream holding text not yet flushed. It is flushed before the other
    /// stream is written, so both keep program order on one terminal or pipe.
    unflushed: Option<Stream>,
    failure: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stream {
    Stdout,
    Stderr,
}

impl Stream {
    fn name(self) -> &'static str {
        match self {
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
        }
    }
}

impl Output {
    pub fn stdio() -> Self {
        Self::new(
            Box::new(io::BufWriter::new(io::stdout())),
            Box::new(io::BufWriter::new(io::stderr())),
        )
    }

    fn new(stdout: Box<dyn Write>, stderr: Box<dyn Write>) -> Self {
        Self { stdout, stderr, unflushed: None, failure: None }
    }

    /// Write to stdout without a trailing newline.
    pub fn stdout(&mut self, text: &str) {
        self.write(Stream::Stdout, text, "");
        self.flush();
    }

    /// Write one line to stdout.
    pub fn stdout_line(&mut self, line: &str) {
        self.write(Stream::Stdout, line, "\n");
        self.flush();
    }

    /// Write one line to stderr.
    pub fn stderr_line(&mut self, line: &str) {
        self.write(Stream::Stderr, line, "\n");
        self.flush();
    }

    /// Write to stderr without a trailing newline.
    pub fn stderr(&mut self, text: &str) {
        self.write(Stream::Stderr, text, "");
        self.flush();
    }

    /// Write one file's report, its stdout before its stderr, leaving it in the
    /// streams' buffers until [`Output::flush`] or the next write that flushes.
    pub fn report(&mut self, stdout: &str, stderr: &str) {
        if !stdout.is_empty() {
            self.write(Stream::Stdout, stdout, "");
        }
        if !stderr.is_empty() {
            self.write(Stream::Stderr, stderr, "");
        }
    }

    /// Flush the text the streams' buffers hold.
    pub fn flush(&mut self) {
        let Some(stream) = self.unflushed.take() else {
            return;
        };
        let result = self.writer(stream).flush();
        self.record(stream, result);
    }

    /// Fold any recorded write failure into the command's exit code.
    pub fn finish(mut self, code: ExitCode) -> ExitCode {
        self.flush();
        let Some(failure) = self.failure.take() else {
            return code;
        };
        // Best effort: the failed stream may be stderr itself.
        self.write(Stream::Stderr, &failure, "\n");
        self.flush();
        ExitCode::from(2)
    }

    fn write(&mut self, stream: Stream, text: &str, ending: &str) {
        if self.unflushed.is_some_and(|unflushed| unflushed != stream) {
            self.flush();
        }
        self.unflushed = Some(stream);
        let writer = self.writer(stream);
        let result =
            writer.write_all(text.as_bytes()).and_then(|()| writer.write_all(ending.as_bytes()));
        self.record(stream, result);
    }

    fn writer(&mut self, stream: Stream) -> &mut Box<dyn Write> {
        match stream {
            Stream::Stdout => &mut self.stdout,
            Stream::Stderr => &mut self.stderr,
        }
    }

    fn record(&mut self, stream: Stream, result: io::Result<()>) {
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
            self.failure = Some(format!("failed to write {}: {error}", stream.name()));
        }
    }
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

    /// Accepts every write and fails at the flush.
    struct FailingFlush(ErrorKind);

    impl Write for FailingFlush {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::from(self.0))
        }
    }

    /// One stream of a shared terminal: its text reaches the terminal only when flushed.
    struct TerminalStream {
        terminal: SharedBuffer,
        buffer: Vec<u8>,
    }

    impl Write for TerminalStream {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.terminal.write_all(&std::mem::take(&mut self.buffer))
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

    #[test]
    fn reports_reach_the_terminal_in_program_order_once_flushed() {
        let terminal = SharedBuffer::default();
        let stream = || TerminalStream { terminal: terminal.clone(), buffer: Vec::new() };
        let mut output = Output::new(Box::new(stream()), Box::new(stream()));

        output.report("A.java\n", "");
        output.report("B.java\n", "B.java: error\n");
        output.report("C.java\n", "");
        output.report("D.java\n", "");
        assert_eq!(terminal.contents(), "A.java\nB.java\nB.java: error\n");

        output.flush();
        assert_eq!(terminal.contents(), "A.java\nB.java\nB.java: error\nC.java\nD.java\n");

        output.stderr_line("summary");
        assert_eq!(output.finish(ExitCode::SUCCESS), ExitCode::SUCCESS);
        assert_eq!(terminal.contents(), "A.java\nB.java\nB.java: error\nC.java\nD.java\nsummary\n");
    }

    #[test]
    fn finish_flushes_a_report_left_in_the_buffer() {
        let terminal = SharedBuffer::default();
        let stream = || TerminalStream { terminal: terminal.clone(), buffer: Vec::new() };
        let mut output = Output::new(Box::new(stream()), Box::new(stream()));

        output.report("", "A.java: error\n");

        assert_eq!(output.finish(ExitCode::FAILURE), ExitCode::FAILURE);
        assert_eq!(terminal.contents(), "A.java: error\n");
    }

    #[test]
    fn broken_pipe_at_the_flush_ends_output_without_changing_the_exit_code() {
        for kind in [ErrorKind::BrokenPipe, ErrorKind::WouldBlock] {
            let stderr = SharedBuffer::default();
            let mut output = Output::new(Box::new(FailingFlush(kind)), Box::new(stderr.clone()));

            output.report("A.java\n", "");
            output.flush();

            assert_eq!(output.finish(ExitCode::FAILURE), ExitCode::FAILURE);
            assert_eq!(stderr.contents(), "");
        }
    }

    #[test]
    fn other_failures_at_the_flush_exit_operationally() {
        let stderr = SharedBuffer::default();
        let mut output =
            Output::new(Box::new(FailingFlush(ErrorKind::StorageFull)), Box::new(stderr.clone()));

        output.report("A.java\n", "");
        output.flush();

        assert_eq!(output.finish(ExitCode::SUCCESS), ExitCode::from(2));
        assert!(stderr.contents().starts_with("failed to write stdout: "), "{}", stderr.contents());
    }
}
