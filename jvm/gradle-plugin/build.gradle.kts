import com.vanniktech.maven.publish.GradlePublishPlugin
import java.util.zip.ZipFile
import org.gradle.api.attributes.plugin.GradlePluginApiVersion
import org.gradle.api.publish.maven.tasks.PublishToMavenLocal
import org.gradle.api.publish.maven.tasks.PublishToMavenRepository
import org.gradle.api.tasks.PathSensitivity
import org.gradle.api.tasks.bundling.AbstractArchiveTask
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

val robustaFakeExecutable = providers.gradleProperty("robustaFakeExecutable").map(::file)
val projectLicense = repositoryRoot.resolve("LICENSE")

@Suppress("UNCHECKED_CAST")
val localRepositories = rootProject.extra["localRepositories"] as Map<String, Provider<Directory>>

tasks.processResources {
    from(projectLicense) {
        into("META-INF")
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

// TestKit injects the built jar rather than the class directories, so the plugin
// under test reads its version from the manifest, as a published one does, and
// carries the classes `jar` packages from `common`.
tasks.pluginUnderTestMetadata {
    pluginClasspath.setFrom(tasks.jar)
}

mavenPublishing {
    configure(GradlePublishPlugin())
    coordinates(group.toString(), publishedArtifactId, version.toString())
    publishToMavenCentral()

    pom {
        name.set("Robusta Gradle Plugin")
        description.set(project.description)
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
    dependsOn(verifyDistributionLicenses)
}

tasks.named("prepareMavenCentralPublishing") {
    dependsOn(
        ":verifyReleaseVersion",
        ":validateMavenCentralSigning",
        verifyDistributionLicenses,
        "verifyPluginMetadata",
    )
}

// The plugin runs only once the release's binary jars resolve beside it.
tasks.named("publishPlugins") {
    dependsOn(
        ":verifyReleaseVersion",
        ":binary:verifyBinaryJars",
        verifyDistributionLicenses,
        "verifyPluginMetadata",
    )
}

tasks.withType<PublishToMavenRepository>().configureEach {
    dependsOn(verifyDistributionLicenses)
}

tasks.withType<Test>().configureEach {
    useJUnitPlatform()
    // TestKit daemons outlive individual JUnit tests. Keep their working state in the
    // task-owned temporary directory so each test project can be deleted on Windows.
    systemProperty("robusta.testkit.root", temporaryDir)
    // TestKit captures build output in the test JVM's default charset, which
    // Java 17 takes from the Windows ANSI code page.
    jvmArgs("-Dfile.encoding=UTF-8")
    testLogging {
        exceptionFormat = TestExceptionFormat.FULL
    }
}

// The functional tests publish the native fake from `tasks/plugin_fake` as the
// running platform's binary jar, for the plugin to resolve and run.
// -ProbustaFakeExecutable selects the built fake.
fun Test.useFakeRobusta() {
    inputs.file(robustaFakeExecutable)
        .withPropertyName("robustaFakeExecutable")
        .withPathSensitivity(PathSensitivity.NONE)
        .optional()
    robustaFakeExecutable.orNull?.let { systemProperty("robusta.fake.executable", it.absolutePath) }
    systemProperty("robusta.plugin.version", version.toString())
}

val functionalTest = tasks.register<Test>("functionalTest") {
    description = "Runs Gradle TestKit functional tests."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    testClassesDirs = functionalTestSourceSet.output.classesDirs
    classpath = functionalTestSourceSet.runtimeClasspath
    shouldRunAfter(tasks.test)
    useFakeRobusta()

    // The real binary, published by `:binary` from -ProbustaDebugExecutable or
    // -ProbustaBinariesDirectory.
    val repository = localRepositories.getValue("functionalTest")
    dependsOn(":binary:publishAllPublicationsToFunctionalTestRepository")
    inputs.dir(repository)
        .withPropertyName("binaryRepository")
        .withPathSensitivity(PathSensitivity.RELATIVE)
    systemProperty("robusta.binary.repository", repository.get().asFile.absolutePath)
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
    useFakeRobusta()
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
    useFakeRobusta()
    systemProperty(
        "robusta.test.jvmTargets",
        consumerJvmTargets(project.extensions.getByType<JavaToolchainService>()),
    )
    shouldRunAfter(compatibilityTest)
}

val publicationTest = tasks.register<Test>("publicationTest") {
    description = "Runs the final plugin and binary jars from the rehearsal Maven repository."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    val releaseRehearsalRepository = localRepositories.getValue("releaseRehearsal")
    dependsOn(
        "publishAllPublicationsToReleaseRehearsalRepository",
        ":binary:publishAllPublicationsToReleaseRehearsalRepository",
    )
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
                "rs/robusta/plugin/common/RobustaBinary.class",
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
