# AGENTS.md - AI Assistant Guide for Robusta

Robusta is a high-performance Java formatter and linter written in Rust containing:

- Parser (Java with AST), Linter, Formatter, and Gradle and Maven plugins that bundle the native `robusta` executable

## AI Usage Policy for Contributors

**IMPORTANT**: If you are an AI assistant helping a human contributor:

- **Full responsibility** - The human contributor is responsible for every AI-assisted change they submit
- **Quality standards** - AI-assisted changes must be reviewed, tested, and understood before submission

All AI-generated code must meet Robusta's performance and quality standards.

## Repository Structure

Cargo workspace with key directories:

- `crates/` - Core functionality (start here when exploring)
- `apps/` - Application binaries (robusta)
- `tasks/` - Development tools/automation
- `jvm/` - Gradle build for the Gradle and Maven plugins, outside the Cargo workspace

Avoid editing `generated` subdirectories.

### Core Crates

- `robusta_parser` - Java parser
- `robusta_lexer` - Java tokenizer and comment table
- `robusta_ast` - AST definitions
- `robusta_ast_visit` - AST traversal
- `robusta_ast_macros` - AST attribute macro
- `robusta_semantic` - Semantic analysis/symbols/scopes
- `robusta_linter` - Linting engine/rules
- `robusta_macros` - Lint rule declaration macro
- `robusta_formatter` - Java formatting
- `robusta_formatter_core` - Language-agnostic formatting IR and printer
- `robusta_javadoc` - Javadoc structure for the formatter and linter
- `robusta_config` - Configuration discovery and file selection
- `robusta_diagnostics` - Error reporting
- `robusta_syntax` - Operators and semantic index types
- `robusta_span` - Source spans
- `robusta_str` - Identifier string type with a precomputed hash
- `robusta_allocator` - Memory management
- `robusta_data_structures` - Shared low-level data structures

## Development Commands

Prerequisites: Rust (MSRV: 1.97), JDK 17, CMake and a C/C++ toolchain, just, cargo-binstall

**Setup Notes:**

- `just init` installs `watchexec`, `cargo-insta`, `cargo-deny`, and `cargo-shear`
- Use Conventional Commits for commit messages, like `fix(parser): recover after a malformed member`
- Choose verification from the changed code; reserve `just ready` for an optional broad checkpoint as described below

### Essential Commands

```bash
just fmt             # Format code (run after modifications)
just test            # Run unit/integration tests
just conformance     # Run coverage tests (alias: cargo coverage)
just ready           # Optional broad checkpoint (see below)
just ast             # Update generated files (robusta_ast changes)
just config-schema   # Update robusta.schema.json (config or rule changes)
just jvmgen          # Update the JVM traversal policy (robusta_config traversal changes)
just allocs          # Update allocation snapshots (parser, semantic, or formatter changes)

# JVM plugins (not part of just ready)
just gradle-test     # Build the native executable and test fake, then run the Gradle plugin suites
just maven-test      # Build the native executable and test fake, then run the Maven plugin suites

# Useful shortcuts
just watch "command" # Watch files and re-run command
```

More commands can be found in `justfile`.

### Local and stacked development

Before selecting verification checks, read the [verification guidance](.agents/verification.md).

- Select checks from the changed code and its consumers; for a stack, use each PR's diff against its parent.
- Reuse passing results while tested inputs stay unchanged, and keep `just ready` for stable checkpoints that need broad validation.
- Keep the required subsystem checks and per-PR CI; a passing stack tip does not show that the PRs below it pass.

## Manual Testing & Examples

Run the CLI through stdin for quick testing and debugging:

```bash
printf 'class A{long n=10l;}\n' |
  cargo run --locked --bin robusta -- format --fix --stdin-filepath Example.java
cargo run --locked --bin robusta -- lint --rules
```

## Code Navigation

### Key Locations

- AST: Start with `robusta_ast`, use `robusta_ast_visit` for traversal
- Linting rules: `crates/robusta_linter/src/rules/`
- Parser: `crates/robusta_parser/src/lib.rs`, grammar in `src/java/`, lexer in `robusta_lexer`
- CLI: arguments in `apps/robusta/src/command/`, dispatch in `apps/robusta/src/run.rs`
- Tests: Co-located with source, integration in `tests/`, uses `insta` for snapshots

### Conventions

- Use `robusta_allocator` for memory management
- Follow rustfmt config in `.rustfmt.toml`
- Use `robusta_diagnostics` for errors with source locations
- Performance-critical: avoid unnecessary allocations

## Common Tasks

### Parser Changes

1. Research and test grammar changes against the Java Language Specification
2. Update AST definitions in `robusta_ast` if needed, then run `just ast`
3. Ensure existing tests pass
4. Add parser tests, and formatter tests for every new printable AST shape

## Testing

Robusta uses several testing approaches:

- **Unit/Integration tests**: Standard Rust tests, inline and in `tests/` directories
- **Coverage tests**: Parser and formatter checks over misc fixtures and Java corpora
- **Conformance tests**: Built-in fixes compiled with `javac` across Java releases
- **Snapshot tests**: Track expected outputs using `insta`

### Quick Test Commands

```bash
just test                   # Run all Rust tests
cargo test -p <crate_name>  # Test specific crate
cargo coverage              # Parser and formatter misc fixtures, plus any Java corpora given
cargo coverage parser       # Parser coverage only
cargo conformance           # Compile fix output across Java releases (JDK 25+)
```

### JVM Plugin Testing

The Gradle and Maven plugins are tested through the Gradle build in `jvm/`:

```bash
just gradle-test   # Gradle plugin unit, functional, and validation suites
just maven-test    # Maven plugin unit and functional suites
```

- Production sources compile with `--release 8`; tests compile against JDK 17
- Unit tests are in `src/test`, Gradle TestKit and Maven Wrapper builds in `src/functionalTest`
- The human diagnostic format is asserted by both the CLI integration snapshots and the Gradle functional suite

### Where to Add Tests

| Crate                | Location                                                                                |
| -------------------- | --------------------------------------------------------------------------------------- |
| Parser               | `tasks/coverage/misc/pass/` or `fail/`                                                  |
| Linter               | Inline in rule files; fix fixtures in `tasks/conformance/`; see the crate's `AGENTS.md` |
| Formatter            | Inline snapshots, or `tests/fixtures/java/`                                             |
| CLI                  | `apps/robusta/tests/cli/`                                                               |
| Gradle/Maven plugins | `src/test` and `src/functionalTest`                                                     |

## Notes

- Rapidly evolving project - APIs may change
- Performance is critical for all changes
- Maintain Java language compatibility through Java 25
- Fixes never raise the source language level

---

For human contributors see `CONTRIBUTING.md`
