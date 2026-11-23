# Fix Conformance Test Runner

## Execution

This test runner guards one invariant of Robusta's built-in fixes:

> Applying any built-in fix to Java-8-valid source must never raise the required
> Java source level.

It applies every built-in rule's fixes to the fixtures in [fixtures](./fixtures/) at each fix ceiling (safe, unsafe), and compiles the original and every distinct fixed output with `javac --release` 8, 11, 17, 21, and 25.

To get started, run

```bash
cargo conformance
```

`javac` is taken from `$JAVA_HOME/bin/javac` if it exists, else `PATH`. A requested release the compiler cannot target is an error, never a skip.

## Options

### --release

To compile at specific releases, repeat `--release`, e.g.

```bash
cargo conformance --release 8 --release 21
```

### --filter

To filter for fixtures whose name contains a text, apply the `--filter` option, e.g.

```bash
cargo conformance --filter unused
```

### --corpus

Real projects guard a second contract: safe fixes keep a compiling project compiling. A manifest lists source directories, each compiled with one `javac` run using its classpath jars and release:

```json
{
  "sources": [
    {
      "directory": "guava/src",
      "classpath": ["/path/to/jspecify-1.0.0.jar", "/path/to/failureaccess-1.0.3.jar"],
      "release": 11
    }
  ]
}
```

Relative paths resolve against the manifest's directory. Annotation processing is off and `module-info.java` is skipped. The project itself is never written to; add `--keep-temp` to keep the compile workspace.

```bash
cargo conformance --corpus /path/to/guava.json
```

A failing safe tree fails the run; a failing unsafe tree is marked for review.

## Adding a Fixture

Every fixable built-in rule needs a fixture, or the run fails. Create `fixtures/<name>.java`, declare the rule in its first non-empty comment, and keep it the smallest Java-8-valid, package-private class the fix applies to:

```java
// robusta-fixture: rule=<rule-id>
```
