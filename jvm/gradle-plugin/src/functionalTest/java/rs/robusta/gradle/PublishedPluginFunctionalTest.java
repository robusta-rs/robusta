package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.createProject;
import static rs.robusta.gradle.FunctionalTestSupport.newTestKitDirectory;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.zip.ZipFile;

import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.xpath.XPathFactory;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.GradleRunner;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.w3c.dom.Document;

import rs.robusta.plugin.common.RobustaPlatform;

class PublishedPluginFunctionalTest {
    private static final String REPOSITORY_PROPERTY = "robusta.published.repository";
    private static final String VERSION_PROPERTY = "robusta.published.version";

    @TempDir Path temporaryDirectory;

    // A fresh TestKit directory has empty caches, so each first run resolves the plugin and its
    // binary jar from the rehearsal repository alone.
    @Test
    void resolvesFinalMarkerAndRunsTheBinaryFromThePluginRepository() throws Exception {
        Path repository = Path.of(requiredProperty(REPOSITORY_PROPERTY));
        String version = requiredProperty(VERSION_PROPERTY);
        verifyPublication(repository, version);
        Path project = createProject(temporaryDirectory, "published", "");
        Files.writeString(
                project.resolve("settings.gradle.kts"),
                """
                pluginManagement {
                    repositories {
                        maven { url = uri("%s") }
                    }
                }

                plugins {
                    id("rs.robusta") version "%s"
                }

                dependencyResolutionManagement {
                    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
                }

                rootProject.name = "published-consumer"
                """.formatted(repository.toUri(), version),
                StandardCharsets.UTF_8);

        Path testKitDirectory = newTestKitDirectory("published");
        BuildResult first = runner(project, testKitDirectory).build();
        assertTrue(first.getOutput().contains("Version: " + version), first.getOutput());
        assertTrue(first.getOutput().contains("Configuration cache entry stored"), first.getOutput());

        BuildResult second = runner(project, testKitDirectory).build();
        assertTrue(second.getOutput().contains("Version: " + version), second.getOutput());
        assertTrue(second.getOutput().contains("Configuration cache entry reused"), second.getOutput());
    }

    @Test
    void runsTheBinaryFromALegacyBuildscriptClasspath() throws Exception {
        Path repository = Path.of(requiredProperty(REPOSITORY_PROPERTY));
        String version = requiredProperty(VERSION_PROPERTY);
        Path project = createProject(temporaryDirectory, "legacy", """
                buildscript {
                    repositories {
                        maven { url = uri("%s") }
                    }
                    dependencies {
                        classpath("rs.robusta:robusta-gradle-plugin:%s")
                    }
                }

                subprojects {
                    apply(plugin = "rs.robusta")
                }
                """.formatted(repository.toUri(), version));
        Files.writeString(
                project.resolve("settings.gradle.kts"),
                "rootProject.name = \"legacy-consumer\"\ninclude(\"app\")\n",
                StandardCharsets.UTF_8);
        Files.createDirectories(project.resolve("app"));

        BuildResult result = runner(project, newTestKitDirectory("legacy"), ":app:robustaVersion").build();
        assertTrue(result.getOutput().contains("Version: " + version), result.getOutput());
    }

    @Test
    void runsTheBinaryFromAProjectPluginsBlock() throws Exception {
        Path repository = Path.of(requiredProperty(REPOSITORY_PROPERTY));
        String version = requiredProperty(VERSION_PROPERTY);
        Path project = createProject(temporaryDirectory, "project-plugin", """
                plugins {
                    id("rs.robusta") version "%s" apply false
                }
                """.formatted(version));
        Files.writeString(project.resolve("settings.gradle.kts"), """
                pluginManagement {
                    repositories {
                        maven { url = uri("%s") }
                    }
                }

                rootProject.name = "project-plugin-consumer"
                include("app")
                """.formatted(repository.toUri()), StandardCharsets.UTF_8);
        Files.createDirectories(project.resolve("app"));
        Files.writeString(
                project.resolve("app/build.gradle.kts"),
                "plugins {\n    id(\"rs.robusta\")\n}\n",
                StandardCharsets.UTF_8);

        BuildResult result =
                runner(project, newTestKitDirectory("project-plugin"), ":app:robustaVersion").build();
        assertTrue(result.getOutput().contains("Version: " + version), result.getOutput());
    }

    private static void verifyPublication(Path repository, String version) throws Exception {
        Path implementation = repository.resolve("rs/robusta/robusta-gradle-plugin").resolve(version);
        String implementationName = "robusta-gradle-plugin-" + version;
        for (String extension : List.of(".jar", "-sources.jar", "-javadoc.jar", ".module", ".pom")) {
            assertTrue(Files.isRegularFile(implementation.resolve(implementationName + extension)));
        }

        Document pom = readPom(implementation.resolve(implementationName + ".pom"));
        assertPomValue(pom, "/project/groupId", "rs.robusta");
        assertPomValue(pom, "/project/artifactId", "robusta-gradle-plugin");
        assertPomValue(pom, "/project/version", version);
        assertPomValue(pom, "/project/name", "Robusta Gradle Plugin");
        assertPomValue(pom, "/project/licenses/license/name", "MIT License");
        assertPomValue(pom, "/project/developers/developer/id", "ThomasRopke");
        assertPomValue(pom, "/project/scm/url", "https://github.com/robusta-rs/robusta");

        Path marker = repository.resolve("rs/robusta/rs.robusta.gradle.plugin")
                .resolve(version)
                .resolve("rs.robusta.gradle.plugin-" + version + ".pom");
        Document markerPom = readPom(marker);
        assertPomValue(markerPom, "/project/groupId", "rs.robusta");
        assertPomValue(markerPom, "/project/dependencies/dependency/groupId", "rs.robusta");
        assertPomValue(
                markerPom,
                "/project/dependencies/dependency/artifactId",
                "robusta-gradle-plugin");
        assertPomValue(markerPom, "/project/dependencies/dependency/version", version);

        try (ZipFile sources = new ZipFile(implementation.resolve(implementationName + "-sources.jar").toFile())) {
            List<String> entries = sources.stream().map(entry -> entry.getName()).toList();
            assertTrue(entries.contains("rs/robusta/gradle/RobustaPlugin.java"));
        }
        try (ZipFile plugin = new ZipFile(implementation.resolve(implementationName + ".jar").toFile())) {
            assertFalse(plugin.stream()
                    .map(entry -> entry.getName().substring(entry.getName().lastIndexOf('/') + 1))
                    .anyMatch(name -> name.startsWith("robusta-")));
        }

        Path binary = repository.resolve("rs/robusta/robusta-binary").resolve(version);
        String binaryName = "robusta-binary-" + version;
        Document binaryPom = readPom(binary.resolve(binaryName + ".pom"));
        assertPomValue(binaryPom, "/project/artifactId", "robusta-binary");
        assertPomValue(binaryPom, "/project/packaging", "pom");
        for (RobustaPlatform platform : RobustaPlatform.values()) {
            try (ZipFile jar = new ZipFile(binary.resolve(binaryName + "-" + platform.triple() + ".jar").toFile())) {
                List<String> binaries = jar.stream()
                        .map(entry -> entry.getName())
                        .filter(name -> name.startsWith("robusta-"))
                        .sorted()
                        .toList();
                assertEquals(
                        List.of(platform.binaryName(), platform.binaryName() + ".sha256"),
                        binaries);
            }
        }
    }

    private static Document readPom(Path path) throws Exception {
        assertTrue(Files.isRegularFile(path));
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        factory.setFeature("http://apache.org/xml/features/disallow-doctype-decl", true);
        return factory.newDocumentBuilder().parse(path.toFile());
    }

    private static void assertPomValue(Document pom, String expression, String expected)
            throws Exception {
        String actual = XPathFactory.newInstance().newXPath().evaluate(expression, pom);
        assertEquals(expected, actual, expression);
    }

    private static GradleRunner runner(Path project, Path testKitDirectory) {
        return runner(project, testKitDirectory, "robustaVersion", "--configuration-cache");
    }

    private static GradleRunner runner(Path project, Path testKitDirectory, String... arguments) {
        List<String> allArguments = new ArrayList<>(List.of(arguments));
        allArguments.add("--stacktrace");
        return GradleRunner.create()
                .withProjectDir(project.toFile())
                .withTestKitDir(testKitDirectory.toFile())
                .withArguments(allArguments);
    }

    private static String requiredProperty(String name) {
        String value = System.getProperty(name);
        Assumptions.assumeTrue(
                value != null && !value.isBlank(),
                "The published-plugin rehearsal requires -D" + name);
        return value;
    }
}
