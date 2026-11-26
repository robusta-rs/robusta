---
# Adapted from https://github.com/oxc-project/oxc/blob/777d2d1b5cb7845740dd734bf85f420bc5c3e3a5/.agents/skills/insta-snapshots/SKILL.md
name: insta-snapshots
description: Review and update Robusta Insta snapshots non-interactively after expected output changes or snapshot test failures.
---

# Insta snapshots

Run the affected test to record pending snapshots, then list them. For each one, compare the old and new output and decide whether the change is the one the task intended:

```bash
cargo insta pending-snapshots
cargo insta pending-snapshots --as-json
```

Most Robusta snapshots are inline: the expected output sits in the test source, and `--as-json` prints the old and new text side by side. A file snapshot keeps its old output in a `.snap` file and its new output in the `.snap.new` beside it.

Accept each reviewed snapshot by name rather than through the interactive `cargo insta review` UI or by editing the expected output by hand:

```bash
cargo insta accept --snapshot <path>:<line>
cargo insta accept --snapshot <name>.snap
```

Accept only the snapshots this task changed, and leave any unrelated pending snapshot alone instead of accepting or rejecting everything. Accepting an inline snapshot can move later snapshots in the same file, so list them again before accepting the next one. When the new output is wrong, fix the implementation instead of accepting it.

After accepting, inspect the diff and rerun the affected test. It should pass with no pending snapshots left from the task. Updating snapshots does not itself call for a commit.

Some expected output is not an Insta snapshot, so `cargo insta` neither lists nor accepts it. Formatter fixtures in `crates/robusta_formatter/tests/fixtures/java/` pair each input with a `.expected.java` file that the test compares directly, and `cargo coverage` rewrites the reports in `tasks/coverage/snapshots/`. Inspect the harness that owns such output instead.
