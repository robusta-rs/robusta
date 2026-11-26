// Adapted from https://github.com/oxc-project/oxc/blob/777d2d1b5cb7845740dd734bf85f420bc5c3e3a5/crates/oxc_parser/examples/parser.rs
#![expect(clippy::print_stdout)]
//! # Parser Example
//!
//! This example demonstrates how to use the Robusta parser to parse Java files.
//!
//! ## Usage
//!
//! Create a `Test.java` file and run:
//! ```bash
//! cargo run -p robusta_parser --example parser [filename] [--ast] [--comments]
//! ```
//!
//! ## Options
//!
//! - `--ast`: Display the parsed AST structure
//! - `--comments`: Display extracted comments

use std::fs;

use pico_args::Arguments;
use robusta_allocator::Allocator;
use robusta_parser::{ParseStatus, parse};

/// Parse and display information about a Java file
fn main() -> Result<(), String> {
    let mut args = Arguments::from_env();

    // Parse command line arguments
    let show_ast = args.contains("--ast");
    let show_comments = args.contains("--comments");
    let name = args.free_from_str().unwrap_or_else(|_| "Test.java".to_string());

    // Read source file
    let source_text = fs::read_to_string(&name).map_err(|_| format!("Missing '{name}'"))?;

    // Parse the source code
    let allocator = Allocator::default();
    let ret = parse(&allocator, &source_text);
    let unit = ret.unit;

    // Display comments if requested
    if show_comments {
        println!("Comments:");
        for comment in &unit.comments {
            let s = comment.content_span(&source_text).source_text(&source_text);
            println!("{s}");
        }
    }

    // Display AST if requested
    if show_ast {
        println!("AST:");
        println!("{unit:#?}");
    }

    // Report parsing results
    for error in ret.errors {
        println!("{}", error.render_with_source_code(source_text.as_str()));
    }
    match ret.status {
        ParseStatus::Clean => println!("Parsed Successfully."),
        ParseStatus::Recovered => println!("Parsed with Errors."),
        ParseStatus::Fatal => println!("Failed to Parse."),
    }

    Ok(())
}
