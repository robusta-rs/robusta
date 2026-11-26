import com.vanniktech.maven.publish.GradlePublishPlugin
import java.util.zip.ZipFile
import org.gradle.api.attributes.plugin.GradlePluginApiVersion
import org.gradle.api.publish.maven.tasks.PublishToMavenLocal
import org.gradle.api.publish.maven.tasks.PublishToMavenRepository
import org.gradle.api.tasks.Delete
import org.gradle.api.tasks.PathSensitivity
import org.gradle.api.tasks.bundling.AbstractArchiveTask
import org.gradle.api.tasks.bundling.Jar
import org.gradle.api.tasks.compile.JavaCompile
import org.gradle.api.tasks.testing.logging.TestExceptionFormat
import org.gradle.plugin.compatibility.compatibility

plugins {
    id("com.gradle.plugin-publish") version "2.1.1"
    id("com.vanniktech.maven.publish")
}

@Suppress("UNCHECKED_CAST")
val repositoryRoot = rootProject.extra["repositoryRoot"] as File
val buildJavaVersion = rootProject.extra["buildJavaVersion"] as Int
val pluginJvmTarget = rootProject.extra["pluginJvmTarget"] as Int
// The published artifact name. Pinned rather than derived from the project
// name: this module became `:gradle-plugin` inside a multi-project build, and
// the coordinate a consumer already depends on must not follow it.
val publishedArtifactId = "robusta-gradle-plugin"
val minimumGradleVersion = "7.3"
val latestGradle8Version = "8.14.5"
val compatibilityGradleVersions = listOf(
    minimumGradleVersion,
    "8.1",
    latestGradle8Version,
    gradle.gradleVersion,
).distinct()
val consumerJvmVersions = listOf(pluginJvmTarget, 11, buildJavaVersion).distinct()

base {
    archivesName = publishedArtifactId
}

tasks.withType<AbstractArchiveTask>().configureEach {
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}

// Plugin Publish sources the jar from SourceSet.allSource, which includes generated resources.
tasks.withType<Jar>().matching { it.name == "sourcesJar" }.configureEach {
    exclude("META-INF/robusta/bin/**")
    includeEmptyDirs = false
}

listOf("apiElements", "runtimeElements").forEach { configurationName ->
    configurations.named(configurationName) {
        attributes {
            attribute(
                GradlePluginApiVersion.GRADLE_PLUGIN_API_VERSION_ATTRIBUTE,
                objects.named(minimumGradleVersion),
            )
        }
    }
}

@Suppress("UNCHECKED_CAST")
val robustaTriples = rootProject.extra["robustaTriples"] as List<String>

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

// The functional tests run the plugin against the native fake from
// `tasks/plugin_fake`. -ProbustaFakeExecutable selects the built fake.
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

val functionalTestSourceSet = sourceSets.create("functionalTest")

configurations[functionalTestSourceSet.implementationConfigurationName]
    .extendsFrom(configurations.testImplementation.get())
configurations[functionalTestSourceSet.runtimeOnlyConfigurationName]
    .extendsFrom(configurations.testRuntimeOnly.get())

// `common` is never published on its own, so it is compiled into this
// jar rather than declared as a dependency: Robusta ships two plugin
// coordinates, and a consumer must not have to resolve a third. `jar` below
// packages its classes; `verifyBundledCommonClasses` proves they arrived.
val pluginCommon = project(":common")

dependencies {
    compileOnly(pluginCommon)
    testImplementation(pluginCommon)
    "functionalTestImplementation"(pluginCommon)
    "functionalTestImplementation"(testFixtures(pluginCommon))
    "functionalTestImplementation"(sourceSets.main.get().output)
    testImplementation(gradleApi())
    testImplementation(gradleTestKit())
    testImplementation("org.junit.jupiter:junit-jupiter:5.13.4")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

gradlePlugin {
    website = "https://github.com/robusta-rs/robusta"
    vcsUrl = "https://github.com/robusta-rs/robusta"
    plugins {
        create("robusta") {
            id = "rs.robusta"
            implementationClass = "rs.robusta.gradle.RobustaPlugin"
            displayName = "Robusta"
            description = project.description
            tags.set(listOf("java", "formatter", "linter", "code-quality"))
            compatibility {
                features {
                    configurationCache = true
                }
            }
        }
    }
    testSourceSets(functionalTestSourceSet)
}

// TestKit injects the plugin under test as a *classpath*, not as the built jar,
// so the classes `jar` packages from `common` have to be named here too.
// Without this the functional suite fails at NoClassDefFoundError inside the
// consumer build — which is exactly what a published jar missing them would do.
tasks.pluginUnderTestMetadata {
    pluginClasspath.from(pluginCommon.sourceSets.main.get().output)
}

val releaseRehearsalRepository = layout.buildDirectory.dir("repositories/release-rehearsal")

val signAllPublications = providers.gradleProperty("signAllPublications")
val signingInMemoryKey = providers.gradleProperty("signingInMemoryKey")
val signingSecretKeyRingFile = providers.gradleProperty("signing.secretKeyRingFile")
val hasSigningCredentials = providers.provider {
    signingInMemoryKey.orNull?.isNotBlank() == true ||
        signingSecretKeyRingFile.orNull?.isNotBlank() == true
}

mavenPublishing {
    configure(GradlePublishPlugin())
    coordinates(group.toString(), publishedArtifactId, version.toString())
    publishToMavenCentral()

    pom {
        name.set("Robusta Gradle Plugin")
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
            name = "releaseRehearsal"
            url = uri(releaseRehearsalRepository)
        }
    }
}

val verifyDistributionLicenses = tasks.register("verifyDistributionLicenses") {
    description = "Verifies the license files shipped in the plugin jar."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    dependsOn(tasks.jar)
    inputs.files(projectLicense)
    inputs.file(tasks.jar.flatMap { it.archiveFile })
    doLast {
        ZipFile(tasks.jar.get().archiveFile.get().asFile).use { archive ->
            mapOf(
                "META-INF/LICENSE" to projectLicense,
            ).forEach { (path, expected) ->
                val entry = requireNotNull(archive.getEntry(path)) {
                    "Plugin jar is missing $path"
                }
                val actual = archive.getInputStream(entry).use { it.readBytes() }
                require(actual.contentEquals(expected.readBytes())) {
                    "Plugin jar contains stale $path"
                }
            }
        }
    }
}

tasks.withType<PublishToMavenLocal>().configureEach {
    dependsOn(verifyRobustaBinaryBundle, verifyDistributionLicenses)
}

val cleanReleaseRehearsalRepository = tasks.register<Delete>("cleanReleaseRehearsalRepository") {
    delete(releaseRehearsalRepository)
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
    dependsOn(
        ":verifyReleaseVersion",
        validateMavenCentralSigning,
        verifyRobustaBinaryBundle,
        verifyDistributionLicenses,
        "verifyPluginMetadata",
    )
}

tasks.named("publishPlugins") {
    dependsOn(
        ":verifyReleaseVersion",
        verifyRobustaBinaryBundle,
        verifyDistributionLicenses,
        "verifyPluginMetadata",
    )
}

tasks.withType<PublishToMavenRepository>().configureEach {
    dependsOn(verifyRobustaBinaryBundle, verifyDistributionLicenses)
    if (name.endsWith("ToReleaseRehearsalRepository")) {
        dependsOn(":verifyReleaseVersion", cleanReleaseRehearsalRepository)
    }
}

tasks.withType<Test>().configureEach {
    useJUnitPlatform()
    // TestKit daemons outlive individual JUnit tests. Keep their working state in the
    // task-owned temporary directory so JUnit can safely delete each test project on Windows.
    systemProperty("robusta.testkit.root", temporaryDir)
    // TestKit captures build output in the test JVM's default charset, which
    // Java 17 takes from the Windows ANSI code page.
    jvmArgs("-Dfile.encoding=UTF-8")
    testLogging {
        exceptionFormat = TestExceptionFormat.FULL
    }
}

// The functional-test runner puts this bundle ahead of the plugin's own
// resources, so the plugin extracts and runs the fake.
fun Test.bundleFakeRobusta() {
    inputs.files(prepareFakeRobustaBundle)
        .withPropertyName("fakeRobustaBundle")
        .withPathSensitivity(PathSensitivity.RELATIVE)
    systemProperty("robusta.fake.bundle", fakeRobustaBundle.get().asFile.absolutePath)
}

val functionalTest = tasks.register<Test>("functionalTest") {
    description = "Runs Gradle TestKit functional tests."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    testClassesDirs = functionalTestSourceSet.output.classesDirs
    classpath = functionalTestSourceSet.runtimeClasspath
    shouldRunAfter(tasks.test)
    bundleFakeRobusta()

    providers.systemProperty("robusta.real.bundled").orNull?.let {
        systemProperty("robusta.real.bundled", it)
    }
}

val compatibilityTest = tasks.register<Test>("compatibilityTest") {
    description = "Runs focused TestKit checks across supported Gradle versions."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    testClassesDirs = functionalTestSourceSet.output.classesDirs
    classpath = functionalTestSourceSet.runtimeClasspath
    filter {
        includeTestsMatching("rs.robusta.gradle.GradleCompatibilityFunctionalTest")
    }
    bundleFakeRobusta()
    systemProperty(
        "robusta.test.gradleVersions",
        compatibilityGradleVersions.joinToString(","),
    )
    shouldRunAfter(functionalTest)
}

// Gradle 9 requires a Java 17 daemon, so an older consumer JVM is paired with the last
// Gradle 8 release. The Gradle axis itself is covered by compatibilityTest.
fun consumerGradleVersion(javaVersion: Int): String =
    if (javaVersion >= 17) gradle.gradleVersion else latestGradle8Version

// A machine without an old JDK verifies fewer JVMs; CI sets requireAllConsumerJvms so a
// missing installation fails the gate instead of quietly shrinking it.
val requireAllConsumerJvms = providers.gradleProperty("requireAllConsumerJvms")

fun consumerJvmTargets(toolchains: JavaToolchainService): String =
    consumerJvmVersions.mapNotNull { javaVersion ->
        val installation = try {
            toolchains.launcherFor {
                languageVersion = JavaLanguageVersion.of(javaVersion)
            }.get().metadata.installationPath.asFile.absolutePath
        } catch (error: GradleException) {
            require(!requireAllConsumerJvms.orNull.toBoolean()) {
                "No Java $javaVersion installation found for the consumer JVM check"
            }
            logger.lifecycle("No Java $javaVersion installation found; skipping its consumer check")
            null
        }
        installation?.let { "$javaVersion|${consumerGradleVersion(javaVersion)}|$it" }
    }.joinToString(",")

val jvmCompatibilityTest = tasks.register<Test>("jvmCompatibilityTest") {
    description = "Runs the plugin on every supported consumer JVM that is installed."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    testClassesDirs = functionalTestSourceSet.output.classesDirs
    classpath = functionalTestSourceSet.runtimeClasspath
    filter {
        includeTestsMatching("rs.robusta.gradle.JvmCompatibilityFunctionalTest")
    }
    bundleFakeRobusta()
    systemProperty(
        "robusta.test.jvmTargets",
        consumerJvmTargets(project.extensions.getByType<JavaToolchainService>()),
    )
    shouldRunAfter(compatibilityTest)
}

val publicationTest = tasks.register<Test>("publicationTest") {
    description = "Resolves the final plugin marker from the rehearsal Maven repository."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    dependsOn("publishAllPublicationsToReleaseRehearsalRepository")
    inputs.dir(releaseRehearsalRepository)
    testClassesDirs = functionalTestSourceSet.output.classesDirs
    classpath = functionalTestSourceSet.runtimeClasspath
    filter {
        includeTestsMatching("rs.robusta.gradle.PublishedPluginFunctionalTest")
    }
    systemProperty(
        "robusta.published.repository",
        releaseRehearsalRepository.get().asFile.absolutePath,
    )
    systemProperty("robusta.published.version", version.toString())
    shouldRunAfter(functionalTest)
}

val pluginModuleMetadata = layout.buildDirectory.file("publications/pluginMaven/module.json")
val verifyPluginMetadata = tasks.register("verifyPluginMetadata") {
    description = "Verifies the published Gradle and Java compatibility attributes."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    dependsOn("generateMetadataFileForPluginMavenPublication")
    inputs.file(pluginModuleMetadata)
    doLast {
        val metadata = pluginModuleMetadata.get().asFile.readText()
        require(Regex("\\\"org\\.gradle\\.jvm\\.version\\\"\\s*:\\s*$pluginJvmTarget").containsMatchIn(metadata)) {
            "Plugin module metadata does not declare Java $pluginJvmTarget compatibility"
        }
        require(Regex("\\\"org\\.gradle\\.plugin\\.api-version\\\"\\s*:\\s*\\\"$minimumGradleVersion\\\"").containsMatchIn(metadata)) {
            "Plugin module metadata does not declare Gradle $minimumGradleVersion compatibility"
        }
    }
}

tasks.jar {
    manifest {
        attributes["Implementation-Version"] = project.version
    }
    // Package the shared mechanics rather than depending on them; see the
    // `pluginCommon` note above the dependencies block.
    from(pluginCommon.sourceSets.main.get().output)
}

val verifyBundledCommonClasses = tasks.register("verifyBundledCommonClasses") {
    description = "Verifies the shared plugin classes are packaged into the jar."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    dependsOn(tasks.jar)
    inputs.file(tasks.jar.flatMap { it.archiveFile })
    doLast {
        // Without this the plugin still compiles and its unit tests still pass,
        // because both see `common` on a classpath the published jar does
        // not have. A consumer would only find out at NoClassDefFoundError.
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

tasks.register("assembleBundledPlugin") {
    description = "Assembles a plugin jar containing every supported Robusta binary."
    group = LifecycleBasePlugin.BUILD_GROUP
    dependsOn(verifyRobustaBinaryBundle, tasks.jar)
}

tasks.check {
    dependsOn(
        functionalTest,
        compatibilityTest,
        jvmCompatibilityTest,
        verifyDistributionLicenses,
        verifyBundledCommonClasses,
        verifyPluginMetadata,
    )
}
