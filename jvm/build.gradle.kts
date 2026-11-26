import java.security.MessageDigest
import org.gradle.api.tasks.PathSensitivity
import org.gradle.api.tasks.compile.JavaCompile
import org.gradle.api.tasks.testing.logging.TestExceptionFormat

// Both plugin modules publish through Vanniktech, whose Maven Central build service
// is shared across the build. Sibling projects loading the plugin separately would
// each get their own copy of that service class, so it is put on the root classpath once.
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

// The native distribution both plugins bundle. Declared once here because a new
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
val robustaBinaryNames = robustaTriples.associateWith { triple ->
    "robusta-$triple${if (triple.endsWith("windows-msvc")) ".exe" else ""}"
}
extra["robustaBinaryNames"] = robustaBinaryNames

// The bundled binary name for the machine running the build, which the functional
// tests stage their own binary under. Lazy, so other hosts can still build.
val hostRobustaBinaryName = providers.provider {
    val os = System.getProperty("os.name").lowercase()
    val arch = System.getProperty("os.arch").lowercase()
    val operatingSystem = when {
        "windows" in os -> "windows"
        "mac" in os || "darwin" in os -> "macos"
        "linux" in os -> "linux"
        else -> os
    }
    val architecture = when (arch) {
        "amd64", "x86_64" -> "x86_64"
        "aarch64", "arm64" -> "aarch64"
        else -> arch
    }
    val triple = robustaTriples.firstOrNull { triple ->
        val tripleOperatingSystem = when {
            triple.contains("linux") -> "linux"
            triple.contains("apple-darwin") -> "macos"
            triple.contains("windows") -> "windows"
            else -> ""
        }
        val tripleArchitecture = triple.substringBefore("-")
        tripleOperatingSystem == operatingSystem && tripleArchitecture == architecture
    }
    robustaBinaryNames.getValue(
        requireNotNull(triple) { "Unsupported platform for functional-test fixture: os=$os arch=$arch" },
    )
}

fun sha256(file: File): String {
    val digest = MessageDigest.getInstance("SHA-256")
    file.inputStream().buffered().use { input ->
        val buffer = ByteArray(16 * 1024)
        while (true) {
            val read = input.read(buffer)
            if (read == -1) break
            digest.update(buffer, 0, read)
        }
    }
    return digest.digest().joinToString("") { byte -> "%02x".format(byte) }
}

extra["sha256"] = ::sha256

// Copies one built binary into `destination` under the host platform's bundled
// name, with its checksum, the way a release binary is bundled.
extra["registerHostBundle"] = fun Project.(
    name: String,
    executable: Provider<File>,
    property: String,
    destination: Provider<Directory>,
): TaskProvider<Task> = tasks.register(name) {
    // Rebuilding the binary at the same path must invalidate the copy.
    inputs.file(executable)
        .withPropertyName(property)
        .withPathSensitivity(PathSensitivity.NONE)
        .optional()
    inputs.property("binaryName", hostRobustaBinaryName)
    outputs.dir(destination)
    doLast {
        val source = executable.orNull
        require(source != null && source.isFile) {
            "Pass -P$property=<path> pointing at a built binary"
        }
        val directory = destination.get().asFile
        directory.mkdirs()
        val binary = directory.resolve(hostRobustaBinaryName.get())
        source.copyTo(binary, overwrite = true)
        binary.setExecutable(true)
        directory.resolve("${binary.name}.sha256")
            .writeText(sha256(binary) + "  " + binary.name + "\n", Charsets.US_ASCII)
    }
}

extra["buildJavaVersion"] = buildJavaVersion
extra["pluginJvmTarget"] = pluginJvmTarget

subprojects {
    apply(plugin = "java")

    group = "rs.robusta"
    version = pluginVersion
    description = "A fast, Rust-native Java formatter and linter."

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

// The two published coordinates are a public contract. Restructuring the JVM
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
