import java.security.MessageDigest
import java.util.zip.ZipFile
import org.gradle.api.publish.maven.MavenPublication
import org.gradle.api.publish.maven.tasks.PublishToMavenRepository
import org.gradle.api.tasks.PathSensitivity
import org.gradle.api.tasks.bundling.AbstractArchiveTask
import org.gradle.api.tasks.bundling.Jar

// One jar per Rust target, each holding that platform's binary and its checksum,
// published once for both plugins. A plugin resolves the running platform's jar
// when a task or goal first needs the binary.
plugins {
    base
    id("com.vanniktech.maven.publish.base")
}

@Suppress("UNCHECKED_CAST")
val repositoryRoot = rootProject.extra["repositoryRoot"] as File

@Suppress("UNCHECKED_CAST")
val robustaTriples = rootProject.extra["robustaTriples"] as List<String>

val publishedArtifactId = "robusta-binary"
val projectLicense = repositoryRoot.resolve("LICENSE")

fun binaryName(triple: String): String =
    "robusta-$triple${if (triple.endsWith("windows-msvc")) ".exe" else ""}"

val binaryNames = robustaTriples.associateWith(::binaryName)

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

// The binary name for the machine running the build, which a local debug binary
// is staged under. Lazy, so other hosts can still build.
val hostBinaryName = providers.provider {
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
        tripleOperatingSystem == operatingSystem && triple.substringBefore("-") == architecture
    }
    binaryNames.getValue(
        requireNotNull(triple) { "Unsupported platform for a local Robusta binary: os=$os arch=$arch" },
    )
}

val robustaBinariesDirectory = providers.gradleProperty("robustaBinariesDirectory").map(::file)
val robustaDebugExecutable = providers.gradleProperty("robustaDebugExecutable").map(::file)
val debugBinaries = layout.buildDirectory.dir("debug-binaries")
val stagedBinaries = layout.buildDirectory.dir("binaries")

fun inputFiles(directory: File): Map<String, File> {
    val expectedNames = binaryNames.values.flatMap { listOf(it, "$it.sha256") }.toSet()
    val files = directory.walkTopDown()
        .filter(File::isFile)
        .filter { it.name in expectedNames }
        .groupBy(File::getName)
    val duplicates = files.filterValues { it.size != 1 }.keys
    require(duplicates.isEmpty()) {
        "Duplicate Robusta binary inputs: ${duplicates.sorted().joinToString()}"
    }
    return files.mapValues { it.value.single() }
}

fun verifyInputs(directory: File, requireAll: Boolean) {
    require(directory.isDirectory) {
        "Robusta binaries directory does not exist: $directory"
    }
    val files = inputFiles(directory)
    require(files.isNotEmpty()) {
        "No recognized Robusta binaries found under $directory"
    }
    for (binaryName in binaryNames.values) {
        val binary = files[binaryName]
        val checksum = files["$binaryName.sha256"]
        require((binary == null) == (checksum == null)) {
            "Robusta binary input must contain both $binaryName and $binaryName.sha256"
        }
        if (binary == null) continue
        val expected = checksum!!.readText(Charsets.US_ASCII)
            .trim()
            .split(Regex("\\s+"), limit = 2)
            .first()
            .lowercase()
        require(expected.matches(Regex("[0-9a-f]{64}"))) {
            "Invalid SHA-256 record: $checksum"
        }
        val actual = sha256(binary)
        require(actual == expected) {
            "Checksum mismatch for $binaryName: expected $expected, actual $actual"
        }
    }
    if (requireAll) {
        val missing = binaryNames.values.filterNot(files::containsKey)
        require(missing.isEmpty()) {
            "Missing Robusta release binaries: ${missing.joinToString()}"
        }
    }
}

// -ProbustaDebugExecutable publishes a local debug CLI as the host platform's
// binary, with its checksum, the way a release binary is published.
val prepareDebugBinary = tasks.register("prepareDebugBinary") {
    // Rebuilding the binary at the same path must invalidate the copy.
    inputs.file(robustaDebugExecutable)
        .withPropertyName("robustaDebugExecutable")
        .withPathSensitivity(PathSensitivity.NONE)
        .optional()
    inputs.property("binaryName", hostBinaryName)
    outputs.dir(debugBinaries)
    doLast {
        val source = robustaDebugExecutable.orNull
        require(source != null && source.isFile) {
            "Pass -ProbustaDebugExecutable=<path> pointing at a built binary"
        }
        val directory = debugBinaries.get().asFile
        directory.deleteRecursively()
        directory.mkdirs()
        val binary = directory.resolve(hostBinaryName.get())
        source.copyTo(binary, overwrite = true)
        binary.setExecutable(true)
        directory.resolve("${binary.name}.sha256")
            .writeText(sha256(binary) + "  " + binary.name + "\n", Charsets.US_ASCII)
    }
}

// The release binaries from -ProbustaBinariesDirectory, or the debug binary. A
// build given neither publishes no binary jar.
val stageBinaries = tasks.register<Sync>("stageBinaries") {
    if (robustaDebugExecutable.isPresent) {
        dependsOn(prepareDebugBinary)
        from(debugBinaries)
    } else if (robustaBinariesDirectory.isPresent) {
        from(robustaBinariesDirectory)
    }
    include(binaryNames.values.flatMap { listOf("**/$it", "**/$it.sha256") })
    includeEmptyDirs = false
    eachFile {
        path = name
    }
    into(stagedBinaries)
    doFirst {
        robustaBinariesDirectory.orNull?.let { verifyInputs(it, requireAll = false) }
    }
}

tasks.withType<AbstractArchiveTask>().configureEach {
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}

// Which targets are staged is known only once a property names the inputs, so
// each jar is registered and publishes only when its binary was staged.
val stagedTriples = providers.provider {
    when {
        robustaDebugExecutable.isPresent -> listOf(robustaTriples.single { binaryName(it) == hostBinaryName.get() })
        robustaBinariesDirectory.isPresent -> {
            val names = inputFiles(robustaBinariesDirectory.get()).keys
            robustaTriples.filter { binaryName(it) in names }
        }
        else -> emptyList()
    }
}

val binaryJars = robustaTriples.associateWith { triple ->
    tasks.register<Jar>("${triple.replace(Regex("[-_](.)")) { it.groupValues[1].uppercase() }}Jar") {
        description = "Assembles the jar holding the $triple binary."
        group = LifecycleBasePlugin.BUILD_GROUP
        archiveBaseName = publishedArtifactId
        archiveClassifier = triple
        from(stageBinaries) {
            include(binaryNames.getValue(triple), "${binaryNames.getValue(triple)}.sha256")
        }
        from(projectLicense) {
            into("META-INF")
        }
    }
}

tasks.assemble {
    dependsOn(stagedTriples.map { triples -> triples.map(binaryJars::getValue) })
}

mavenPublishing {
    coordinates(group.toString(), publishedArtifactId, version.toString())
    publishToMavenCentral()
    // The base plugin reads no signing property, which the plugins' publishing does.
    if (providers.gradleProperty("signAllPublications").orNull.toBoolean()) {
        signAllPublications()
    }

    pom {
        name.set("Robusta Binary")
        description.set("The Robusta executable for one platform, which the Gradle and Maven plugins run.")
        // The classified jars are the artifacts; there is no main jar.
        packaging = "pom"
    }
}

publishing {
    publications {
        create<MavenPublication>("binary") {
            stagedTriples.get().forEach { triple -> artifact(binaryJars.getValue(triple)) }
        }
    }
}

val verifyBinaryJars = tasks.register("verifyBinaryJars") {
    description = "Verifies each release binary jar holds its target's binary and checksum."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    dependsOn(binaryJars.values)
    doLast {
        val directory = requireNotNull(robustaBinariesDirectory.orNull) {
            "Set -ProbustaBinariesDirectory=<path> to assemble the release binary jars"
        }
        verifyInputs(directory, requireAll = true)
        val inputs = inputFiles(directory)
        binaryJars.forEach { (triple, jar) ->
            ZipFile(jar.get().archiveFile.get().asFile).use { archive ->
                val binaries = archive.entries().asSequence()
                    .map { it.name }
                    .filter { it.startsWith("robusta-") }
                    .toSet()
                val expectedNames = binaryNames.getValue(triple).let { setOf(it, "$it.sha256") }
                require(binaries == expectedNames) {
                    "Binary jar for $triple holds $binaries instead of $expectedNames"
                }
                expectedNames.forEach { name ->
                    val actual = archive.getInputStream(archive.getEntry(name)).use { it.readBytes() }
                    require(actual.contentEquals(requireNotNull(inputs[name]).readBytes())) {
                        "Binary jar for $triple contains stale $name"
                    }
                }
                requireNotNull(archive.getEntry("META-INF/LICENSE")) {
                    "Binary jar for $triple is missing META-INF/LICENSE"
                }
            }
        }
    }
}

// A release publishes every platform's jar or none.
tasks.withType<PublishToMavenRepository>()
    .matching {
        it.name.endsWith("ToMavenCentralRepository") || it.name.endsWith("ToReleaseRehearsalRepository")
    }
    .configureEach {
        dependsOn(verifyBinaryJars)
    }
tasks.named("prepareMavenCentralPublishing") {
    dependsOn(":verifyReleaseVersion", ":validateMavenCentralSigning", verifyBinaryJars)
}
