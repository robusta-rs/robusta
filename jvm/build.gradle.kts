import com.vanniktech.maven.publish.MavenPublishBaseExtension
import org.gradle.api.publish.PublishingExtension
import org.gradle.api.publish.maven.tasks.PublishToMavenRepository
import org.gradle.api.tasks.Delete
import org.gradle.api.tasks.compile.JavaCompile
import org.gradle.api.tasks.testing.logging.TestExceptionFormat

// The plugins and the binary jars publish through Vanniktech, whose Maven
// Central build service is shared across the build. Sibling projects loading the
// plugin separately would each get their own copy of that service class, so it
// is put on the root classpath once.
plugins {
    id("com.vanniktech.maven.publish") version "0.37.0" apply false
}

// Conventions shared by every JVM module. Each plugin project keeps only what is
// genuinely its own: its publishing plugin, its framework dependencies, and the
// compatibility matrices its own consumers impose.

val repositoryRoot: File = rootDir.resolve("..").canonicalFile

// The workspace Cargo manifest is the single version source for every published
// JVM artifact, so the plugins can never drift from the CLI they deliver.
fun workspaceVersion(manifest: File): String {
    var inWorkspacePackage = false
    var version: String? = null
    manifest.forEachLine { line ->
        val value = line.substringBefore('#').trim()
        if (value.startsWith('[')) {
            inWorkspacePackage = value == "[workspace.package]"
        } else if (inWorkspacePackage) {
            Regex("""version\s*=\s*"([^"]+)"""").matchEntire(value)?.let {
                version = it.groupValues[1]
            }
        }
    }
    return requireNotNull(version) { "Missing [workspace.package] version in $manifest" }
}

val robustaVersion = workspaceVersion(repositoryRoot.resolve("Cargo.toml"))

// Between releases the workspace version is the last published one, so every
// build is a snapshot of the next minor release unless the release passes
// `-Prelease=true`. A local plugin build then never claims a published version,
// nor shares the published plugin's binary cache.
val isRelease = providers.gradleProperty("release").orNull.toBoolean()
val pluginVersion = if (isRelease) {
    robustaVersion
} else {
    val (major, minor) = robustaVersion.split('.')
    "$major.${minor.toInt() + 1}.0-SNAPSHOT"
}

val verifyReleaseVersion = tasks.register("verifyReleaseVersion") {
    description = "Requires a release build before publishing to a public repository."
    group = "publishing"
    val release = isRelease
    val refusal = "Pass -Prelease=true to publish $robustaVersion instead of $pluginVersion"
    doLast {
        require(release) { refusal }
    }
}

// The native distribution both plugins run. Declared once here because a new
// platform is one product decision, not one per adapter.
val robustaTriples = listOf(
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
)

val buildJavaVersion = 17

// Plugin bytecode must load on a Java 8 consumer daemon; test sources compile
// against the build toolchain. `common` ships inside both plugin jars, so
// it carries the same floor.
val pluginJvmTarget = 8

extra["repositoryRoot"] = repositoryRoot
extra["robustaVersion"] = robustaVersion
extra["robustaTriples"] = robustaTriples

extra["buildJavaVersion"] = buildJavaVersion
extra["pluginJvmTarget"] = pluginJvmTarget

// Local Maven repositories that the functional tests and the release rehearsal
// resolve from. Every published project writes to them, so a consumer finds the
// binary jars beside the plugins, as it does on Maven Central.
val localRepositories = mapOf(
    "functionalTest" to layout.buildDirectory.dir("repositories/functional-test"),
    "releaseRehearsal" to layout.buildDirectory.dir("repositories/release-rehearsal"),
)
extra["localRepositories"] = localRepositories

// A snapshot is published under a new timestamp each time, so each repository
// is emptied once per build rather than keeping every earlier publication.
val cleanLocalRepositories = localRepositories.mapValues { (name, directory) ->
    tasks.register<Delete>("clean${name.replaceFirstChar(Char::uppercase)}Repository") {
        delete(directory)
    }
}

subprojects {
    group = "rs.robusta"
    version = pluginVersion
    description = "A fast, Rust-native Java formatter and linter."

    // The POM metadata every published artifact shares; each project names and
    // describes its own.
    plugins.withId("com.vanniktech.maven.publish.base") {
        extensions.configure<MavenPublishBaseExtension> {
            pom {
                url.set("https://github.com/robusta-rs/robusta")
                licenses {
                    license {
                        name.set("MIT License")
                        url.set("https://opensource.org/licenses/MIT")
                        distribution.set("repo")
                    }
                }
                developers {
                    developer {
                        id.set("ThomasRopke")
                        name.set("Thomas Ropke")
                        url.set("https://github.com/ThomasRopke")
                    }
                }
                scm {
                    connection.set("scm:git:https://github.com/robusta-rs/robusta.git")
                    developerConnection.set("scm:git:ssh://git@github.com/robusta-rs/robusta.git")
                    url.set("https://github.com/robusta-rs/robusta")
                }
                issueManagement {
                    system.set("GitHub")
                    url.set("https://github.com/robusta-rs/robusta/issues")
                }
            }
        }
    }

    plugins.withId("maven-publish") {
        extensions.configure<PublishingExtension> {
            repositories {
                localRepositories.forEach { (repositoryName, directory) ->
                    maven {
                        name = repositoryName
                        url = uri(directory)
                    }
                }
            }
        }
        localRepositories.keys.forEach { repositoryName ->
            val suffix = "To${repositoryName.replaceFirstChar(Char::uppercase)}Repository"
            tasks.withType<PublishToMavenRepository>()
                .matching { it.name.endsWith(suffix) }
                .configureEach {
                    dependsOn(cleanLocalRepositories.getValue(repositoryName))
                    if (repositoryName == "releaseRehearsal") {
                        dependsOn(verifyReleaseVersion)
                    }
                }
        }
    }
}

val signAllPublications = providers.gradleProperty("signAllPublications")
val signingInMemoryKey = providers.gradleProperty("signingInMemoryKey")
val signingSecretKeyRingFile = providers.gradleProperty("signing.secretKeyRingFile")
val hasSigningCredentials = providers.provider {
    signingInMemoryKey.orNull?.isNotBlank() == true ||
        signingSecretKeyRingFile.orNull?.isNotBlank() == true
}

val validateMavenCentralSigning = tasks.register("validateMavenCentralSigning") {
    description = "Requires the PGP credentials used for Maven Central."
    group = "publishing"
    doLast {
        require(signAllPublications.orNull.toBoolean()) {
            "Set signAllPublications=true before publishing to Maven Central"
        }
        require(hasSigningCredentials.get()) {
            "Configure signingInMemoryKey (recommended for CI) or signing.secretKeyRingFile " +
                "before publishing to Maven Central"
        }
    }
}

// The binary jars hold no classes, so only the code modules are Java projects.
configure(subprojects.filter { it.name != "binary" }) {
    apply(plugin = "java")

    repositories {
        mavenCentral()
    }

    extensions.configure<JavaPluginExtension> {
        toolchain {
            languageVersion = JavaLanguageVersion.of(buildJavaVersion)
        }
        sourceCompatibility = JavaVersion.toVersion(pluginJvmTarget)
        targetCompatibility = JavaVersion.toVersion(pluginJvmTarget)
    }

    // Java 17 tools otherwise read sources in the platform charset, which is
    // the ANSI code page on Windows.
    tasks.withType<JavaCompile>().configureEach {
        options.encoding = "UTF-8"
        options.release = buildJavaVersion
    }

    tasks.withType<Javadoc>().configureEach {
        options.encoding = "UTF-8"
    }

    tasks.named<JavaCompile>("compileJava") {
        options.release = pluginJvmTarget
    }

    tasks.withType<Test>().configureEach {
        useJUnitPlatform()
        testLogging {
            exceptionFormat = TestExceptionFormat.FULL
        }
    }
}

// The two plugin coordinates are a public contract. Restructuring the JVM
// build into subprojects silently renamed both — the jar took the project name
// and the Gradle plugin's Maven coordinate took `rootProject.name` — so the
// names are pinned here rather than derived, and this gate proves it.
val verifyPublishedArtifactNames = tasks.register("verifyPublishedArtifactNames") {
    description = "Verifies the published plugin artifacts keep their contracted names."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    val expected = mapOf(
        "gradle-plugin" to "robusta-gradle-plugin",
        "maven-plugin" to "robusta-maven-plugin",
    )
    val actual = expected.keys.associateWith { path ->
        project(":$path").extensions.getByType<BasePluginExtension>().archivesName
    }
    dependsOn(expected.keys.map { ":$it:jar" })
    doLast {
        expected.forEach { (path, name) ->
            val found = actual.getValue(path).get()
            require(found == name) {
                "Published artifact name for :$path is '$found', must stay '$name'"
            }
        }
    }
}

// The Maven goal prefix is derived from the artifactId, so the same rename that
// broke the jar names emptied `<goalPrefix/>` and would have broken every
// documented `robusta:<goal>` invocation. `GoalSurfaceFunctionalTest` asserts
// the goals; this asserts the prefix that makes them reachable.
val verifyMavenGoalPrefix = tasks.register("verifyMavenGoalPrefix") {
    description = "Verifies the Maven plugin descriptor keeps its contracted goal prefix."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    dependsOn(":maven-plugin:generateMavenPluginDescriptor")
    val descriptor = project(":maven-plugin").layout.buildDirectory
        .file("mavenPlugin/descriptor/META-INF/maven/plugin.xml")
    inputs.file(descriptor)
    doLast {
        val text = descriptor.get().asFile.readText()
        listOf(
            "<artifactId>robusta-maven-plugin</artifactId>",
            "<goalPrefix>robusta</goalPrefix>",
        ).forEach { expected ->
            require(text.contains(expected)) {
                "Maven plugin descriptor is missing $expected"
            }
        }
    }
}

tasks.register("check") {
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    dependsOn(verifyPublishedArtifactNames, verifyMavenGoalPrefix)
}
