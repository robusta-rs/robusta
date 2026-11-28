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
  - Builds and tests the five native executables, their install scripts, the Homebrew formula, the five jars that publish them, and the two plugin jars
  - On the tag push, creates the `v<version>` GitHub release with the native executables, their checksums, the `install.sh` and `install.ps1` scripts, and the license
  - Then pushes the formula, filled in with the release's checksums, to [robusta-rs/homebrew-java](https://github.com/robusta-rs/homebrew-java)

### Local Rehearsal

- Download the five native artifacts from that run into one directory and build the functional-test fake with `cargo build --locked --bin robusta_plugin_fake`, then from `jvm/` run

```bash
./gradlew clean check publicationTest validatePlugins verifyBinaryJars \
  -Prelease=true \
  -ProbustaBinariesDirectory=<directory> \
  "-ProbustaFakeExecutable=$PWD/../target/debug/robusta_plugin_fake" \
  -Drobusta.real.bundled=true
```

- Install JDK 8 and 11 and add `-PrequireAllConsumerJvms=true` to rehearse the full consumer JVM matrix

## Publish Artifacts

- Run [Release CLI](https://github.com/robusta-rs/robusta/actions/workflows/release_cli.yml) from the `v<version>` tag with `publish_maven_central`, then with `publish_gradle_plugin_portal`
  - Nothing is uploaded unless both plugins were assembled and tested
  - The binary jars and both plugins upload as one Central deployment, whose final Publish action in the Central Portal is manual and cannot be undone
  - The Gradle plugin fetches its binary jars from Maven Central, so run the Gradle Plugin Portal upload only once that deployment is published

## Update Publication Secrets

- `MAVEN_CENTRAL_USERNAME`, `MAVEN_CENTRAL_PASSWORD` — Central Portal upload
- `MAVEN_SIGNING_KEY`, `MAVEN_SIGNING_PASSWORD` — in-memory PGP signing of the Central deployment
- `GRADLE_PUBLISH_KEY`, `GRADLE_PUBLISH_SECRET` — Gradle Plugin Portal upload
- `HOMEBREW_TAP_TOKEN` — push to the `robusta-rs/homebrew-java` tap
