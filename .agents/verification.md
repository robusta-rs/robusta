# Verification

Pick checks from the diff and from the code that consumes what it changes; in a
stack, diff each PR against its parent. [AGENTS.md](../AGENTS.md#essential-commands)
lists the commands that update generated files and snapshots.

- **Rust code/tests:** `just lint -p <crate>` and `just test -p <crate>` without a
  filter where the crate has tests. Filter only while debugging, and treat a run
  that finds no tests as incomplete. Run `cargo coverage` after parser or
  formatter changes and `cargo conformance` after built-in fix changes.
- **Gradle and Maven plugins:** `just gradle-test` and `just maven-test` for each
  plugin a change affects; `jvm/common` and CLI behavior the plugins invoke
  affect both.
- **Cargo dependencies and licenses:** `just deny`.
- **Prose:** no checks.
- **Rustdoc:** `just doc -p <crate>`; add
  `cargo test -p <crate> --all-features --doc` when executable examples change.

A passing result stays valid until the tested code, dependencies, generated
inputs, toolchain, or build configuration change; a commit alone does not
invalidate it. Keep profiles, features, and package selections the same across
runs, and run standalone or feature-specific checks on their own, since a
grouped build can hide errors. Run Cargo commands that share a target directory
one at a time.

When the impact is unclear or workspace-wide, widen to `just test`, `just lint`,
and the subsystem checks above. `just ready` is an optional broad checkpoint in
place of repeating equivalent checks. It fails on unstaged changes, so stage or
commit first, and read its output. In a stack, run it once at the tip; every PR
still needs its own CI.
