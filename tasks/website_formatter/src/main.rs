#![expect(clippy::print_stderr)]

use std::process::ExitCode;

use pico_args::Arguments;

mod cli;
mod json_schema;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();
    let print: fn() = match args.subcommand()?.as_deref() {
        Some("cli") => cli::print_cli,
        Some("schema-markdown") => json_schema::print_schema_markdown,
        _ => return Err("usage: website_formatter <cli|schema-markdown>".into()),
    };
    if !args.finish().is_empty() {
        return Err("unexpected arguments".into());
    }
    print();
    Ok(())
}
