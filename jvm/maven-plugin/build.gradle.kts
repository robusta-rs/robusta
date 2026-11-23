import com.vanniktech.maven.publish.JavaLibrary
import com.vanniktech.maven.publish.JavadocJar
import com.vanniktech.maven.publish.SourcesJar
import java.util.zip.ZipFile
import org.gradle.api.publish.maven.tasks.PublishToMavenRepository
import org.gradle.api.tasks.bundling.AbstractArchiveTask
import org.gradle.api.tasks.bundling.Jar
import org.gradle.api.tasks.PathSensitivity
import org.gradle.api.tasks.compile.JavaCompile
import org.gradle.api.tasks.testing.logging.TestExceptionFormat

plugins {
    id("org.gradlex.maven-plugin-development") version "1.0.3"
    id("com.vanniktech.maven.publish")
}

@Suppress("UNCHECKED_CAST")
val repositoryRoot = rootProject.extra["repositoryRoot"] as File
val buildJavaVersion = rootProject.extra["buildJavaVersion"] as Int
val pluginJvmTarget = rootProject.extra["pluginJvmTarget"] as Int

// See the Gradle adapter: the published name is pinned, not derived from the
// project name this module has inside the multi-project build.
val publishedArtifactId = "robusta-maven-plugin"

base {
    archivesName = publishedArtifactId
}

// The release workflow tests this jar in one job and rebuilds it in the job
// that uploads it, so the two builds must agree byte for byte. Current Gradle
// already normalizes both, but the published bytes should not depend on a
// default. See the Gradle adapter, which pins the same pair.
tasks.withType<AbstractArchiveTask>().configureEach {
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}

// The sources jar is sourced from SourceSet.allSource, which includes generated resources.
tasks.withType<Jar>().matching { it.name == "sourcesJar" }.configureEach {
    exclude("META-INF/robusta/bin/**")
    includeEmptyDirs = false
}

// See the Gradle adapter's note: `common` is packaged into this jar
// rather than depended on, so Robusta keeps exactly two published coordinates.
val pluginCommon = project(":common")

dependencies {
    compileOnly(pluginCommon)
    testImplementation(pluginCommon)
    testImplementation(testFixtures(pluginCommon))
    // Maven's own launcher classloader supplies these to every plugin realm; bundling them
    // would duplicate classes the runtime already provides, so they never reach the published
    // POM's dependencies (mirrors <scope>provided</scope>).
    compileOnly("org.apache.maven:maven-plugin-api:3.9.16")
    compileOnly("org.apache.maven:maven-core:3.9.16")
    // The effective local repository, which honours -Dmaven.repo.local, is only reachable
    // through the resolver session. compileOnly is not transitive, so it is named here.
    compileOnly("org.apache.maven.resolver:maven-resolver-api:1.9.27")
    compileOnly("org.apache.maven.plugin-tools:maven-plugin-annotations:3.15.2")

    testImplementation("org.apache.maven:maven-plugin-api:3.9.16")
    testImplementation("org.junit.jupiter:junit-jupiter:5.13.4")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

mavenPlugin {
    name = "Robusta Maven Plugin"
    description = project.description
    // Maven derives the goal prefix from the artifactId's `<prefix>-maven-plugin`
    // shape, so the descriptor must carry the published name rather than this
    // module's project name. Without it `<goalPrefix/>` is empty and every
    // documented `robusta:<goal>` invocation stops resolving.
    artifactId = publishedArtifactId
    goalPrefix = "robusta"
}

@Suppress("UNCHECKED_CAST")
val robustaBinaryNames = rootProject.extra["robustaBinaryNames"] as Map<String, String>

@Suppress("UNCHECKED_CAST")
val sha256 = rootProject.extra["sha256"] as (File) -> String

@Suppress("UNCHECKED_CAST")
val registerHostBundle = rootProject.extra["registerHostBundle"]
    as Project.(String, Provider<File>, String, Provider<Directory>) -> TaskProvider<Task>

val robustaBinariesDirectory = providers.gradleProperty("robustaBinariesDirectory")
val robustaDebugExecutable = providers.gradleProperty("robustaDebugExecutable").map(::file)
val robustaFakeExecutable = providers.gradleProperty("robustaFakeExecutable").map(::file)
val debugRobustaBundle = layout.buildDirectory.dir("functionalTestFixture/debug-robusta")
val fakeRobustaBundle = layout.buildDirectory.dir("functionalTestFixture/fake-robusta")
val generatedRobustaResources = layout.buildDirectory.dir("generated/robusta-resources")
val projectLicense = repositoryRoot.resolve("LICENSE")

fun bundledInputFiles(directory: File): Map<String, File> {
    val expectedNames = robustaBinaryNames.values.flatMap { listOf(it, "$it.sha256") }.toSet()
    val files = directory.walkTopDown()
        .filter(File::isFile)
        .filter { it.name in expectedNames }
        .groupBy(File::getName)
    val duplicates = files.filterValues { it.size != 1 }.keys
    require(duplicates.isEmpty()) {
        "Duplicate Robusta bundle inputs: ${duplicates.sorted().joinToString()}"
    }
    return files.mapValues { it.value.single() }
}

fun verifyBundledInputs(directory: File, requireAll: Boolean) {
    require(directory.isDirectory) {
        "Robusta binaries directory does not exist: $directory"
    }
    val files = bundledInputFiles(directory)
    require(files.isNotEmpty()) {
        "No recognized Robusta binaries found under $directory"
    }
    for (binaryName in robustaBinaryNames.values) {
        val binary = files[binaryName]
        val checksum = files["$binaryName.sha256"]
        require((binary == null) == (checksum == null)) {
            "Robusta bundle input must contain both $binaryName and $binaryName.sha256"
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
        val missing = robustaBinaryNames.values.filterNot(files::containsKey)
        require(missing.isEmpty()) {
            "Missing Robusta release binaries: ${missing.joinToString()}"
        }
    }
}

// -ProbustaDebugExecutable bundles a local debug CLI in place of release binaries.
val prepareDebugRobustaBundle = registerHostBundle(
    "prepareDebugRobustaBundle",
    robustaDebugExecutable,
    "robustaDebugExecutable",
    debugRobustaBundle,
)

// The tests run the native fake from `tasks/plugin_fake`, directly or as the
// bundled binary of a second plugin jar. -ProbustaFakeExecutable selects the
// built fake.
val prepareFakeRobustaBundle = registerHostBundle(
    "prepareFakeRobustaBundle",
    robustaFakeExecutable,
    "robustaFakeExecutable",
    fakeRobustaBundle.map { it.dir("META-INF/robusta/bin") },
)

val prepareRobustaBinaries = tasks.register<Sync>("prepareRobustaBinaries") {
    val input = robustaDebugExecutable.map { debugRobustaBundle.get().asFile }
        .orElse(robustaBinariesDirectory.map(::file))
        .orElse(file("src/main/robusta-binaries"))
    if (robustaDebugExecutable.isPresent) {
        dependsOn(prepareDebugRobustaBundle)
    }
    from(input) {
        include(robustaBinaryNames.values.flatMap { listOf("**/$it", "**/$it.sha256") })
        includeEmptyDirs = false
        eachFile {
            path = "META-INF/robusta/bin/$name"
        }
    }
    into(generatedRobustaResources)
    doFirst {
        if (robustaBinariesDirectory.isPresent) {
            verifyBundledInputs(file(robustaBinariesDirectory.get()), requireAll = false)
        }
    }
}

sourceSets.main {
    resources.srcDir(prepareRobustaBinaries)
}

tasks.processResources {
    from(projectLicense) {
        into("META-INF")
    }
}

val verifyRobustaBinaryBundle = tasks.register("verifyRobustaBinaryBundle") {
    dependsOn(tasks.jar)
    doLast {
        require(robustaBinariesDirectory.isPresent) {
            "Set -ProbustaBinariesDirectory=<path> to assemble the self-contained plugin jar"
        }
        val directory = file(robustaBinariesDirectory.get())
        verifyBundledInputs(directory, requireAll = true)
        val inputs = bundledInputFiles(directory)
        ZipFile(tasks.jar.get().archiveFile.get().asFile).use { archive ->
            robustaBinaryNames.values
                .flatMap { listOf(it, "$it.sha256") }
                .forEach { name ->
                    val path = "META-INF/robusta/bin/$name"
                    val entry = requireNotNull(archive.getEntry(path)) {
                        "Plugin jar is missing $path"
                    }
                    val actual = archive.getInputStream(entry).use { it.readBytes() }
                    val expected = requireNotNull(inputs[name]).readBytes()
                    require(actual.contentEquals(expected)) {
                        "Plugin jar contains stale $path"
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

mavenPublishing {
    configure(JavaLibrary(javadocJar = JavadocJar.Javadoc(), sourcesJar = SourcesJar.Sources()))
    coordinates(group.toString(), publishedArtifactId, version.toString())
    publishToMavenCentral()

    pom {
        name.set("Robusta Maven Plugin")
        description.set(project.description)
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

publishing {
    repositories {
        maven {
            name = "functionalTest"
            url = uri(layout.buildDirectory.dir("repositories/functional-test"))
        }
    }
}

// The plugin jar with the fake in place of its bundled binaries. A functional
// test installs it into its own local repository, where Maven resolves it
// instead of the published jar.
val fakeBundledPluginJar = tasks.register<Jar>("fakeBundledPluginJar") {
    destinationDirectory = layout.buildDirectory.dir("functionalTestFixture/fake-plugin")
    from(zipTree(tasks.jar.flatMap { it.archiveFile })) {
        exclude("META-INF/robusta/bin/**")
    }
    from(prepareFakeRobustaBundle) {
        into("META-INF/robusta/bin")
    }
}

val functionalTestSourceSet = sourceSets.create("functionalTest")

configurations[functionalTestSourceSet.implementationConfigurationName]
    .extendsFrom(configurations.testImplementation.get())
configurations[functionalTestSourceSet.runtimeOnlyConfigurationName]
    .extendsFrom(configurations.testRuntimeOnly.get())

dependencies {
    "functionalTestImplementation"(pluginCommon)
    "functionalTestImplementation"(testFixtures(pluginCommon))
    "functionalTestImplementation"(sourceSets.main.get().output)
}

tasks.withType<Test>().configureEach {
    useJUnitPlatform()
    testLogging {
        exceptionFormat = TestExceptionFormat.FULL
    }
}

tasks.test {
    inputs.files(prepareFakeRobustaBundle)
        .withPropertyName("fakeRobustaBundle")
        .withPathSensitivity(PathSensitivity.RELATIVE)
    systemProperty("robusta.fake.bundle", fakeRobustaBundle.get().asFile.absolutePath)
}

val functionalTest = tasks.register<Test>("functionalTest") {
    description = "Runs Maven Wrapper-driven functional tests against a locally published plugin."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    dependsOn("publishMavenPublicationToFunctionalTestRepository")
    testClassesDirs = functionalTestSourceSet.output.classesDirs
    classpath = functionalTestSourceSet.runtimeClasspath
    shouldRunAfter(tasks.test)

    // These tests read the published artifact, not the runtime classpath, so the classpath
    // Gradle infers is not the whole input. The generated plugin descriptor is derived from
    // annotations and Javadoc, which produce identical bytecode: without this the goal-surface
    // gate stays up to date across a descriptor change and never runs.
    inputs.dir(layout.buildDirectory.dir("repositories/functional-test"))
        .withPropertyName("publishedPlugin")
        .withPathSensitivity(PathSensitivity.RELATIVE)

    systemProperty(
        "robusta.functionalTest.repository",
        layout.buildDirectory.dir("repositories/functional-test").get().asFile.absolutePath,
    )
    systemProperty("robusta.functionalTest.pluginVersion", project.version.toString())

    inputs.files(fakeBundledPluginJar)
        .withPropertyName("fakeBundledPlugin")
        .withPathSensitivity(PathSensitivity.NONE)
    systemProperty(
        "robusta.functionalTest.fakePlugin",
        fakeBundledPluginJar.get().archiveFile.get().asFile.absolutePath,
    )
    providers.systemProperty("robusta.real.bundled").orNull?.let {
        systemProperty("robusta.real.bundled", it)
    }
}

tasks.jar {
    // Packaged, not depended on; see the `pluginCommon` note above.
    from(pluginCommon.sourceSets.main.get().output)
}

val verifyBundledCommonClasses = tasks.register("verifyBundledCommonClasses") {
    description = "Verifies the shared plugin classes are packaged into the jar."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    dependsOn(tasks.jar)
    inputs.file(tasks.jar.flatMap { it.archiveFile })
    doLast {
        // Maven resolves a plugin realm from the published POM, which does not
        // name `common`. If these classes were missing the mojos would
        // fail at NoClassDefFoundError inside a consumer's build.
        ZipFile(tasks.jar.get().archiveFile.get().asFile).use { archive ->
            listOf(
                "rs/robusta/plugin/common/BundledExecutable.class",
                "rs/robusta/plugin/common/RobustaPlatform.class",
                "rs/robusta/plugin/common/CommandLineArguments.class",
                "rs/robusta/plugin/common/Hex.class",
                "rs/robusta/plugin/common/RobustaFixedCommand.class",
                "rs/robusta/plugin/common/RobustaPluginException.class",
            ).forEach { entry ->
                requireNotNull(archive.getEntry(entry)) {
                    "Plugin jar is missing shared class $entry"
                }
            }
        }
    }
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

tasks.named("prepareMavenCentralPublishing") {
    dependsOn(validateMavenCentralSigning, verifyRobustaBinaryBundle, verifyBundledCommonClasses)
}

// The functional suite publishes a jar bundling only the current platform's binary to
// its in-tree repository, so the complete-bundle gate guards Maven Central alone.
tasks.withType<PublishToMavenRepository>()
    .matching { it.name.endsWith("ToMavenCentralRepository") }
    .configureEach {
        dependsOn(verifyRobustaBinaryBundle, verifyBundledCommonClasses)
    }

tasks.register("assembleBundledPlugin") {
    description = "Assembles a plugin jar containing every supported Robusta binary."
    group = LifecycleBasePlugin.BUILD_GROUP
    dependsOn(verifyRobustaBinaryBundle, tasks.jar)
}

tasks.check {
    dependsOn(functionalTest, verifyBundledCommonClasses)
}
