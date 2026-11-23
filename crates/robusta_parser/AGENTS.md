# Coding agent guides for `crates/robusta_parser`

## Testing parser behavior

Parser syntax regressions should usually be tested through coverage fixtures, not Rust unit tests in this crate.

Before adding a `misc` fixture, check whether the case is already covered by an existing fixture.

Use:

```text
tasks/coverage/misc/pass/
tasks/coverage/misc/fail/
```

Put valid syntax that must parse successfully in `pass/`, and invalid syntax or diagnostic regressions in `fail/`. Each `pass/` fixture also runs through the formatter checks.

Choose a short descriptive name for the syntax or diagnostic category:

```text
tasks/coverage/misc/pass/var-as-identifier.java
tasks/coverage/misc/fail/method-without-body.java
```

Run a focused parser coverage case with:

```sh
cargo coverage parser --filter misc/pass/var-as-identifier.java
cargo coverage parser --filter misc/fail/method-without-body.java
```

Run all parser coverage with:

```sh
cargo coverage parser
```

## When to use crate tests

Use Rust tests in `crates/robusta_parser` only for parser API behavior, AST shape, recovery, or focused implementation details that cannot be represented as a source fixture.

Do not add ordinary syntax or diagnostic regressions there when a `tasks/coverage/misc` fixture can cover the behavior.
