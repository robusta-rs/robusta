// Adapted from https://github.com/oxc-project/oxc/blob/050d1328112e3546635fe7352b726995fa2524ef/tasks/coverage/src/main.rs

use std::{num::NonZeroUsize, process::ExitCode};

use pico_args::Arguments;
use rayon::ThreadPoolBuilder;
use robusta_coverage::{
    TestData,
    args::{self, Tool},
};
use robusta_diagnostics::Output;

fn main() -> ExitCode {
    let mut output = Output::stdio();
    let (tool, app_args) = match args::parse(Arguments::from_env()) {
        Ok(args) => args,
        Err(error) => {
            output.stderr_line(&format!("robusta_coverage: {error}"));
            return output.finish(ExitCode::from(2));
        }
    };
    let thread_count = if app_args.debug {
        1
    } else {
        std::thread::available_parallelism().map_or(1, NonZeroUsize::get)
    };
    if let Err(error) = ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .stack_size(8 * 1024 * 1024)
        .build_global()
    {
        output.stderr_line(&format!("robusta_coverage: failed to initialize worker pool: {error}"));
        return output.finish(ExitCode::from(2));
    }

    let data = match TestData::load(app_args.filter.as_deref(), &app_args.corpora) {
        Ok(data) => data,
        Err(error) => {
            output.stderr_line(&format!("robusta_coverage: {error}"));
            return output.finish(ExitCode::from(2));
        }
    };
    let passed = match tool {
        Some(Tool::Parser) => app_args.run_parser(&data, &mut output),
        Some(Tool::Formatter) => app_args.run_formatter(&data, &mut output),
        None => app_args.run_all(&data, &mut output),
    };
    output.finish(if passed { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}
