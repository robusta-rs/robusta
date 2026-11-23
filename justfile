#!/usr/bin/env -S just --justfile

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

_default:
  @just --list -u

# ==================== ALIASES ====================
alias r := ready
alias c := conformance
alias f := fix

# ==================== SETUP & INITIALIZATION ====================

# Install the developer tooling used by the recipes below.
init:
  cargo binstall watchexec-cli cargo-insta typos-cli cargo-deny cargo-shear@1.13.1 -y

# Install git pre-commit hook to format files
install-hook:
  echo -e "#!/bin/sh\njust fmt" > .git/hooks/pre-commit
  chmod +x .git/hooks/pre-commit

# ==================== CORE DEVELOPMENT ====================

# Run the local Rust gate before pushing; CI adds plugin, conformance, allocation, MSRV, unused-dependency, and cross-platform jobs.
ready:
  git diff --exit-code --quiet
  typos
  just fmt
  just check
  just test
  just snapshots-check
  just lint
  just doc
  just ast
  git status

# Type-check the whole workspace (cargo alias `ck`).
check *args:
  cargo ck {{args}}

# Format all Rust sources and drop dependencies nothing uses.
fmt:
  -cargo shear --fix --check-test-targets
  cargo fmt --all

# Verify formatting without writing (CI check).
fmt-check:
  cargo fmt --all -- --check

# Run the workspace test suite.
test *args:
  cargo test --all-features --locked {{args}}

# Reject unreviewed insta snapshot changes.
snapshots-check:
  #!/usr/bin/env bash
  set -euo pipefail
  pending="$(cargo insta pending-snapshots --workspace --as-json)"
  if [[ -n "$pending" ]]; then
    cargo insta pending-snapshots --workspace
    exit 1
  fi
  echo "No pending snapshots."

# Interactively accept or reject insta snapshot changes.
snapshots-review:
  cargo insta review --workspace

# Regenerate the committed sources derived from the AST definitions.
ast:
  cargo run --locked -p robusta_ast_tools

# Regenerate the committed robusta.yaml JSON Schema.
config-schema:
  cargo run --locked -p website_linter schema-json > robusta.schema.json

# Lint the whole workspace, denying warnings (cargo alias `lint`).
[unix]
lint *args:
  CARGO_BUILD_WARNINGS=deny cargo lint --locked {{args}}

[windows]
lint *args:
  $Env:CARGO_BUILD_WARNINGS='deny'; cargo lint --locked {{args}}

# Build all workspace documentation with the same strict flags as CI.
[unix]
doc *args:
  RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --document-private-items --all-features --locked {{args}}

[windows]
doc *args:
  $Env:RUSTDOCFLAGS='-D warnings'; cargo doc --no-deps --document-private-items --all-features --locked {{args}}

# License, advisory, ban, and source checks over the dependency graph.
deny:
  cargo deny check

# Auto-fix formatting, clippy, and typos where possible.
fix:
  cargo clippy --fix --allow-staged --no-deps
  just fmt
  typos -w
  git status

# ==================== DEVELOPMENT TOOLS ====================

# Run a command on every file change.
watch *args='':
  watchexec --no-vcs-ignore {{args}}

# Re-check and re-lint the workspace on every file change.
watch-check:
  just watch "'cargo check; cargo clippy'"

# Run the formatter and linter benchmarks (cargo alias `benchmark`).
benchmark:
  cargo benchmark

# Update memory allocation snapshots (cargo alias `allocs`).
allocs:
  cargo allocs

# Regenerate the website documentation into a local checkout of the site.
website path:
  cargo run --locked -p website_linter rules --rules-json {{path}}/.vitepress/data/rules.json --rule-docs {{path}}/src/docs/guide/usage/linter/rules --git-ref $(git rev-parse HEAD) --rule-count {{path}}/src/docs/guide/usage
  cargo run --locked -p website_linter cli > {{path}}/src/docs/guide/usage/linter/generated-cli.md
  cargo run --locked -p website_linter schema-markdown > {{path}}/src/docs/guide/usage/linter/generated-config.md
  cargo run --locked -p website_formatter cli > {{path}}/src/docs/guide/usage/formatter/generated-cli.md
  cargo run --locked -p website_formatter schema-markdown > {{path}}/src/docs/guide/usage/formatter/generated-config.md

# ==================== TESTING & CONFORMANCE ====================

# Run all conformance tests
coverage:
  cargo coverage
  cargo conformance

# Run the misc fixtures and any Java corpora given
conformance *args='':
  cargo coverage {{args}}

# ==================== GRADLE AND MAVEN PLUGINS ====================

# Regenerate the JVM traversal policy from the native configuration.
jvmgen:
  cargo run --locked -p robusta_jvm_codegen

# Build the debug native executable and the fake the plugin tests bundle.
plugin-binaries:
  cargo build --locked --bin robusta --bin robusta_plugin_fake

# Run the Gradle plugin unit, functional, and validation suites.
gradle-test: plugin-binaries
  #!/usr/bin/env bash
  set -euo pipefail
  cd {{justfile_directory()}}/jvm
  ./gradlew :common:test :gradle-plugin:test :gradle-plugin:functionalTest \
    :gradle-plugin:validatePlugins verifyPublishedArtifactNames verifyMavenGoalPrefix \
    "-ProbustaFakeExecutable={{justfile_directory()}}/target/debug/robusta_plugin_fake" \
    "-ProbustaDebugExecutable={{justfile_directory()}}/target/debug/robusta" \
    -Drobusta.real.bundled=true

# Run the Maven plugin unit and Maven-Wrapper-driven functional suites.
maven-test: plugin-binaries
  #!/usr/bin/env bash
  set -euo pipefail
  cd {{justfile_directory()}}/jvm
  ./gradlew :common:test :maven-plugin:test :maven-plugin:functionalTest \
    verifyPublishedArtifactNames verifyMavenGoalPrefix \
    "-ProbustaFakeExecutable={{justfile_directory()}}/target/debug/robusta_plugin_fake" \
    "-ProbustaDebugExecutable={{justfile_directory()}}/target/debug/robusta" \
    -Drobusta.real.bundled=true
