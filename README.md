<div align="center">

[![MIT licensed][license-badge]][license-url]
[![Build Status][ci-badge]][ci-url]

</div>

## ☕ Robusta

A fast, Rust-native Java formatter and linter. Robusta is one native executable with no JVM to start, takes its formatting and lint settings from the nearest `robusta.yaml` above each file, and runs the same way on a developer machine and in CI. Gradle and Maven plugins fetch that executable for the running platform so a build can call it directly.

Robusta formats and lints in one binary. **Formatting** is deterministic and idempotent. **Linting** analyzes each file on its own, so it is deterministic, needs no build information, and is safe to enforce in CI.

Robusta is a DX toolchain, not a correctness analyzer — comprehensive semantic analysis stays with SonarQube, Error Prone, and SpotBugs. Fixes never raise the source language level: no fix makes Java 8 code need a later release.

Robusta follows [Oxc](https://oxc.rs)'s architecture and project structure and ports parts of its Rust implementation. [google-java-format](https://github.com/google/google-java-format) helped shape Robusta's Java layout and Javadoc formatting.

Thanks to the Oxc and google-java-format teams.

For more information, check out our website at [robusta.rs](https://robusta.rs).

<sub>\* Early development. The formatter covers the finalized Java grammar through Java 25, excluding preview features. Interfaces may still change.</sub>

## 🔧 Lint or Format a Codebase

- **Lint**: [Linter](https://robusta.rs/docs/guide/usage/linter) — `robusta lint --fix <path>...`
- **Format**: [Formatter](https://robusta.rs/docs/guide/usage/formatter) — `robusta format --fix <path>...`
- **Both, in CI**: `robusta check <path>...`
- **Gradle and Maven**: [Gradle](https://robusta.rs/docs/guide/usage/gradle) and [Maven](https://robusta.rs/docs/guide/usage/maven) plugins — `./gradlew robustaCheck` or `mvn robusta:check`

## ✍️ Contribute

Check out the [open issues](https://github.com/robusta-rs/robusta/issues), or see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidance.

## 📚 Other Resources

- [Architecture and roadmap](./ARCHITECTURE.md)
- [Maintenance](./MAINTENANCE.md)
- [Releases](https://github.com/robusta-rs/robusta/releases)

## 📖 License

Robusta is free and open-source software licensed under the [MIT License](./LICENSE).

Robusta ports or copies code from other open source projects; their licenses are listed in [**Third-party library licenses**](./THIRD-PARTY-LICENSE).

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/robusta-rs/robusta/blob/main/LICENSE
[ci-badge]: https://github.com/robusta-rs/robusta/actions/workflows/ci.yml/badge.svg?event=push&branch=main
[ci-url]: https://github.com/robusta-rs/robusta/actions/workflows/ci.yml?query=event%3Apush+branch%3Amain
