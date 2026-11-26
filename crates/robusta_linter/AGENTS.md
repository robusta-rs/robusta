# AGENTS.md

## Adding Linting Rule

1. Add `crates/robusta_linter/src/rules/<rule_name>.rs` starting with `declare_robusta_lint!`
2. Implement `Rule::run` for the node types the declaration lists
3. Add tests in same module with `Tester`
4. Register in `rules/mod.rs`; a fixable rule also needs a fixture in `tasks/conformance/fixtures/`
