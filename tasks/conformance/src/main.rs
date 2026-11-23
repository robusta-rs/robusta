use std::process::ExitCode;

use pico_args::Arguments;
use robusta_conformance::{TestRunner, args, render, render_corpus};
use robusta_diagnostics::Output;

fn main() -> ExitCode {
    let mut output = Output::stdio();
    let args = match args::parse(Arguments::from_env()) {
        Ok(args) => args,
        Err(error) => {
            output.stderr_line(&format!("robusta_conformance: {error}"));
            return output.finish(ExitCode::from(2));
        }
    };

    let corpus = !args.corpus.is_empty();
    let result = TestRunner::new(args).and_then(|runner| {
        if corpus {
            runner.run_corpus().map(|report| (render_corpus(&report), report.passed()))
        } else {
            runner.run().map(|report| (render(&report), report.passed()))
        }
    });
    match result {
        Ok((text, passed)) => {
            output.stdout(&text);
            output.finish(if passed { ExitCode::SUCCESS } else { ExitCode::FAILURE })
        }
        Err(error) => {
            output.stderr_line(&format!("robusta_conformance: {error}"));
            output.finish(ExitCode::from(2))
        }
    }
}
