## Goals

In no specific order:

- Better performance
- Lower memory usage
- More test coverage
- Better conformance with existing tools and specifications

## Robusta CLI & Plugins

### Release Robusta

- Set `[workspace.package] version` in `Cargo.toml` and tag the release commit `v<version>`
  - The CLI, the Gradle plugin, and the Maven coordinates all read that version
- Run [Release CLI](https://github.com/robusta-rs/robusta/actions/workflows/release_cli.yml)
  - Also runs automatically on a `v*.*.*` tag push, without publishing the plugins
  - Builds and tests the five native executables and the two plugin jars that bundle them
  - On the tag push, creates the `v<version>` GitHub release with the native executables, their checksums, and the license

### Local Rehearsal

- Download the five native artifacts from that run into one directory and build the functional-test fake with `cargo build --locked --bin robusta_plugin_fake`, then from `jvm/` run

```bash
./gradlew clean check publicationTest validatePlugins assembleBundledPlugin \
  -ProbustaBinariesDirectory=<directory> \
  "-ProbustaFakeExecutable=$PWD/../target/debug/robusta_plugin_fake" \
  -Drobusta.real.bundled=true
```

- Install JDK 8 and 11 and add `-PrequireAllConsumerJvms=true` to rehearse the full consumer JVM matrix

## Publish Artifacts

- Run [Release CLI](https://github.com/robusta-rs/robusta/actions/workflows/release_cli.yml) from the `v<version>` tag with `publish_maven_central`, `publish_gradle_plugin_portal`, or both
  - Nothing is uploaded unless both plugins were assembled and tested
  - Each plugin uploads its own Central deployment, whose final Publish action in the Central Portal is manual and cannot be undone
  - With both selected, the Gradle Plugin Portal upload runs after the Central uploads

## Update Publication Secrets

- `MAVEN_CENTRAL_USERNAME`, `MAVEN_CENTRAL_PASSWORD` — Central Portal upload
- `MAVEN_SIGNING_KEY`, `MAVEN_SIGNING_PASSWORD` — in-memory PGP signing of the Central deployment
- `GRADLE_PUBLISH_KEY`, `GRADLE_PUBLISH_SECRET` — Gradle Plugin Portal upload
