// Adapted from https://github.com/oxc-project/oxc/blob/777d2d1b5cb7845740dd734bf85f420bc5c3e3a5/crates/oxc_parser/examples/visitor.rs
#![expect(clippy::print_stdout)]
//! # AST Visitor Example
//!
//! This example demonstrates how to use the visitor pattern to traverse and analyze
//! an AST. It counts different types of nodes (classes, methods, lambdas).
//!
//! ## Usage
//!
//! Create a `Test.java` file and run:
//! ```bash
//! cargo run -p robusta_parser --example visitor [filename]
//! ```

use std::env;

use robusta_allocator::Allocator;
use robusta_ast::{ClassDeclaration, LambdaExpression, MethodDeclaration};
use robusta_ast_visit::{Visit, walk};
use robusta_parser::parse;

/// Demonstrate AST traversal using the visitor pattern
fn main() -> std::io::Result<()> {
    let name = env::args().nth(1).unwrap_or_else(|| "Test.java".to_string());
    let source_text = std::fs::read_to_string(&name)?;
    let allocator = Allocator::default();
    let ret = parse(&allocator, &source_text);

    // Report any parsing errors
    for error in ret.errors {
        println!("{}", error.render_with_source_code(source_text.as_str()));
    }

    let unit = ret.unit;

    // Use visitor to count different AST node types
    let mut ast_pass = CountASTNodes::default();
    ast_pass.visit_compilation_unit(&unit);
    println!("{ast_pass:?}");

    Ok(())
}

/// A visitor that counts different types of AST nodes
#[derive(Debug, Default)]
struct CountASTNodes {
    classes: usize,
    methods: usize,
    lambdas: usize,
}

impl<'a> Visit<'a> for CountASTNodes {
    fn visit_class_declaration(&mut self, class: &'a ClassDeclaration<'a>) {
        self.classes += 1;
        walk::walk_class_declaration(self, class);
    }

    fn visit_method_declaration(&mut self, method: &'a MethodDeclaration<'a>) {
        self.methods += 1;
        walk::walk_method_declaration(self, method);
    }

    fn visit_lambda_expression(&mut self, lambda: &'a LambdaExpression<'a>) {
        self.lambdas += 1;
        walk::walk_lambda_expression(self, lambda);
    }
}
