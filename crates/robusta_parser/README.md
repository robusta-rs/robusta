# Robusta Parser

High-performance Java parser with error recovery.

## Overview

This crate provides a hand-written parser for Java that produces a complete Abstract Syntax Tree (AST). It supports finalized Java syntax through Java 25.

## Key Features

- **Complete language support**: Ordinary, compact, and modular compilation units
- **Latest Java**: Finalized Java syntax through Java 25
- **Error recovery**: Recovers from some syntax errors; fatal errors return an empty compilation unit
- **Comprehensive AST**: Typed nodes with accurate source positions

## Architecture

### Parser Design

- **Recursive descent**: Hand-written recursive descent over a pre-tokenized stream
- **Error recovery**: Recovers at declaration, member, statement, and selected-list boundaries, and reports a `Clean`, `Recovered`, or `Fatal` outcome
- **Memory efficient**: Uses arena allocation for optimal performance
- **Source input**: Tokenizes a complete UTF-8 source string before parsing

### Language Support

- **Declarations**: Classes, interfaces, enums, records, and annotation interfaces
- **Modules**: Module declarations and module import declarations
- **Patterns**: Type and record patterns in `instanceof` and `switch`
- **Contextual keywords**: Words such as `var`, `record`, and `yield` are resolved by position and stay usable as identifiers elsewhere

### AST Structure

The parser produces an AST that closely follows the Java Language Specification:

- **Accurate positions**: Every node has precise source location information
- **Comments and source text**: Records comments in source order on the compilation unit; whitespace is available through the original source text
- **Type-safe**: Leverages Rust's type system for correctness
- **Visitor-friendly**: Traversed with the visitor in `robusta_ast_visit`

The parser is the foundation for Robusta's formatter and linter, providing the AST they format and analyze.
