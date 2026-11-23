package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.createProject;
import static rs.robusta.gradle.FunctionalTestSupport.newTestKitDirectory;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
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

class PublishedPluginFunctionalTest {
    private static final String REPOSITORY_PROPERTY = "robusta.published.repository";
    private static final String VERSION_PROPERTY = "robusta.published.version";

    @TempDir Path temporaryDirectory;

    @Test
    void resolvesFinalMarkerAndExecutesBundledBinary() throws Exception {
        String repositoryValue = requiredProperty(REPOSITORY_PROPERTY);
        String version = requiredProperty(VERSION_PROPERTY);
        Path repository = Path.of(repositoryValue);
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
            assertFalse(entries.stream().anyMatch(name -> name.startsWith("META-INF/robusta/bin/")));
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
        return GradleRunner.create()
                .withProjectDir(project.toFile())
                .withTestKitDir(testKitDirectory.toFile())
                .withArguments(
                        "robustaRun",
                        "--args=--version",
                        "--configuration-cache",
                        "--stacktrace");
    }

    private static String requiredProperty(String name) {
        String value = System.getProperty(name);
        Assumptions.assumeTrue(
                value != null && !value.isBlank(),
                "The published-plugin rehearsal requires -D" + name);
        return value;
    }
}
