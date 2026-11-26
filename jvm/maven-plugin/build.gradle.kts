import com.vanniktech.maven.publish.JavaLibrary
import com.vanniktech.maven.publish.JavadocJar
import com.vanniktech.maven.publish.SourcesJar
import java.util.zip.ZipFile
import org.gradle.api.publish.maven.tasks.PublishToMavenRepository
import org.gradle.api.tasks.bundling.AbstractArchiveTask
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
val robustaVersion = rootProject.extra["robustaVersion"] as String

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

// See the Gradle adapter's note: `common` is packaged into this jar
// rather than depended on, so Robusta keeps exactly two plugin coordinates.
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
    // The resolver resolves the binary jar, and its session holds the effective local
    // repository, which honours -Dmaven.repo.local. compileOnly is not transitive, so it is
    // named here.
    compileOnly("org.apache.maven.resolver:maven-resolver-api:1.9.27")
    compileOnly("javax.inject:javax.inject:1")
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

val robustaFakeExecutable = providers.gradleProperty("robustaFakeExecutable").map(::file)
val projectLicense = repositoryRoot.resolve("LICENSE")

@Suppress("UNCHECKED_CAST")
val functionalTestRepository =
    (rootProject.extra["localRepositories"] as Map<String, Provider<Directory>>).getValue("functionalTest")

tasks.processResources {
    from(projectLicense) {
        into("META-INF")
    }
}

mavenPublishing {
    configure(JavaLibrary(javadocJar = JavadocJar.Javadoc(), sourcesJar = SourcesJar.Sources()))
    coordinates(group.toString(), publishedArtifactId, version.toString())
    publishToMavenCentral()

    pom {
        name.set("Robusta Maven Plugin")
        description.set(project.description)
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

// The tests run the native fake from `tasks/plugin_fake`, directly or as the
// running platform's binary jar. -ProbustaFakeExecutable selects the built fake.
fun Test.useFakeRobusta() {
    inputs.file(robustaFakeExecutable)
        .withPropertyName("robustaFakeExecutable")
        .withPathSensitivity(PathSensitivity.NONE)
        .optional()
    robustaFakeExecutable.orNull?.let { systemProperty("robusta.fake.executable", it.absolutePath) }
}

tasks.test {
    useFakeRobusta()
}

val functionalTest = tasks.register<Test>("functionalTest") {
    description = "Runs Maven Wrapper-driven functional tests against a locally published plugin."
    group = LifecycleBasePlugin.VERIFICATION_GROUP
    // The plugin, and the real binary `:binary` publishes from
    // -ProbustaDebugExecutable or -ProbustaBinariesDirectory.
    dependsOn(
        "publishMavenPublicationToFunctionalTestRepository",
        ":binary:publishAllPublicationsToFunctionalTestRepository",
    )
    testClassesDirs = functionalTestSourceSet.output.classesDirs
    classpath = functionalTestSourceSet.runtimeClasspath
    shouldRunAfter(tasks.test)

    // These tests read the published artifact, not the runtime classpath, so the classpath
    // Gradle infers is not the whole input. The generated plugin descriptor is derived from
    // annotations and Javadoc, which produce identical bytecode: without this the goal-surface
    // gate stays up to date across a descriptor change and never runs.
    inputs.dir(functionalTestRepository)
        .withPropertyName("publishedPlugin")
        .withPathSensitivity(PathSensitivity.RELATIVE)

    systemProperty(
        "robusta.functionalTest.repository",
        functionalTestRepository.get().asFile.absolutePath,
    )
    systemProperty("robusta.functionalTest.pluginVersion", project.version.toString())
    systemProperty("robusta.functionalTest.cliVersion", robustaVersion)

    useFakeRobusta()
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

tasks.named("prepareMavenCentralPublishing") {
    dependsOn(
        ":verifyReleaseVersion",
        ":validateMavenCentralSigning",
        verifyBundledCommonClasses,
    )
}

tasks.withType<PublishToMavenRepository>()
    .matching { it.name.endsWith("ToMavenCentralRepository") }
    .configureEach {
        dependsOn(verifyBundledCommonClasses)
    }

tasks.check {
    dependsOn(functionalTest, verifyBundledCommonClasses)
}
