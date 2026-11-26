package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.createPluginProject;
import static rs.robusta.gradle.FunctionalTestSupport.createProject;
import static rs.robusta.gradle.FunctionalTestSupport.kotlinString;
import static rs.robusta.gradle.FunctionalTestSupport.newTestKitDirectory;
import static rs.robusta.gradle.FunctionalTestSupport.pluginVersion;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.FileTime;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.GradleRunner;
import org.junit.jupiter.api.Test;

import rs.robusta.plugin.common.RobustaPlatform;

// Each repository here holds only the running platform's jar, so a task needs no other.
class RobustaBinaryFunctionalTest {
    @TestKitTempDir Path temporaryDirectory;

    @Test
    void executesTheResolvedBinaryAndReusesOnlyAVerifiedExtraction() throws Exception {
        Path project = createPluginProject(temporaryDirectory, "binary");
        Path testKitDirectory = newTestKitDirectory("binary");
        Path initScript = fakeRepository(testKitDirectory, true);

        BuildResult first = runner(project, initScript, testKitDirectory, arguments(true)).build();
        assertTrue(first.getOutput().contains("fake robusta stdout"));
        assertTrue(first.getOutput().contains("Configuration cache entry stored"));

        Path extracted = extracted(testKitDirectory);
        assertTrue(Files.isRegularFile(extracted));
        FileTime sentinel = FileTime.fromMillis(1_700_000_000_000L);
        Files.setLastModifiedTime(extracted, sentinel);

        BuildResult second = runner(project, initScript, testKitDirectory, arguments(true)).build();
        assertTrue(second.getOutput().contains("fake robusta stdout"));
        assertTrue(second.getOutput().contains("Configuration cache entry reused"));
        assertEquals(sentinel, Files.getLastModifiedTime(extracted));

        byte[] verified = Files.readAllBytes(extracted);
        Files.writeString(extracted, "corrupted cache", StandardCharsets.UTF_8);
        BuildResult corruptedCache =
                runner(project, initScript, testKitDirectory, arguments(true)).build();
        assertTrue(corruptedCache.getOutput().contains("fake robusta stdout"));
        assertTrue(corruptedCache.getOutput().contains("Configuration cache entry reused"));
        assertArrayEquals(verified, Files.readAllBytes(extracted));
    }

    @Test
    void refusesABinaryThatFailsItsChecksumBeforeExecution() throws Exception {
        Path project = createPluginProject(temporaryDirectory, "corrupt");
        Path testKitDirectory = newTestKitDirectory("corrupt");
        Path initScript = fakeRepository(testKitDirectory, false);

        BuildResult failure =
                runner(project, initScript, testKitDirectory, arguments(false)).buildAndFail();

        assertTrue(failure.getOutput().contains("Robusta checksum mismatch"), failure.getOutput());
        assertTrue(failure.getOutput().contains("expected " + "0".repeat(64)));
        assertTrue(Files.notExists(extracted(testKitDirectory)));
    }

    @Test
    void failsWhenThePluginRepositoriesLackTheRunningPlatformsJar() throws Exception {
        Path project = createPluginProject(temporaryDirectory, "missing");
        Path testKitDirectory = newTestKitDirectory("missing");
        Path initScript = BinaryRepository.initScript(
                testKitDirectory.resolve("binary"),
                Files.createDirectories(testKitDirectory.resolve("binary/empty")));

        BuildResult failure =
                runner(project, initScript, testKitDirectory, arguments(false)).buildAndFail();

        assertTrue(
                failure.getOutput().contains("Could not find rs.robusta:robusta-binary:" + pluginVersion()),
                failure.getOutput());
        assertTrue(Files.notExists(extracted(testKitDirectory)));
    }

    @Test
    void resolvesFromTheSettingsPluginRepositoriesWhenProjectRepositoriesAreRefused()
            throws Exception {
        Path testKitDirectory = newTestKitDirectory("settings");
        Path repository = BinaryRepository.createFake(testKitDirectory.resolve("repository"), true);
        Path project = createProject(temporaryDirectory, "settings", "");
        Files.createDirectories(project.resolve("app"));
        Files.writeString(project.resolve("settings.gradle.kts"), """
                buildscript {
                    repositories {
                        maven { url = uri("%s") }
                    }
                }

                plugins {
                    id("rs.robusta")
                }

                dependencyResolutionManagement {
                    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
                }

                rootProject.name = "settings"
                include("app")
                """.formatted(kotlinString(repository)), StandardCharsets.UTF_8);

        BuildResult stored = settingsRunner(project, testKitDirectory).build();
        assertTrue(stored.getOutput().contains("fake robusta stdout"), stored.getOutput());
        assertTrue(stored.getOutput().contains("Configuration cache entry stored"));

        BuildResult reused = settingsRunner(project, testKitDirectory).build();
        assertTrue(reused.getOutput().contains("fake robusta stdout"), reused.getOutput());
        assertTrue(reused.getOutput().contains("Configuration cache entry reused"));
    }

    @Test
    void concurrentBuildsShareOneAtomicExtraction() throws Exception {
        Path firstProject = createPluginProject(temporaryDirectory, "concurrent-a");
        Path secondProject = createPluginProject(temporaryDirectory, "concurrent-b");
        Path testKitDirectory = newTestKitDirectory("concurrent");
        Path initScript = fakeRepository(testKitDirectory, true);
        ExecutorService executor = Executors.newFixedThreadPool(2);
        try {
            Future<BuildResult> first = executor.submit(() -> runner(
                    firstProject, initScript, testKitDirectory, arguments(false)).build());
            Future<BuildResult> second = executor.submit(() -> runner(
                    secondProject, initScript, testKitDirectory, arguments(false)).build());

            assertTrue(first.get(2, TimeUnit.MINUTES).getOutput().contains("fake robusta stdout"));
            assertTrue(second.get(2, TimeUnit.MINUTES).getOutput().contains("fake robusta stdout"));
        } finally {
            executor.shutdownNow();
        }

        Path extracted = extracted(testKitDirectory);
        assertTrue(Files.isRegularFile(extracted));
        assertTrue(Files.size(extracted) > 0);
    }

    private static Path fakeRepository(Path testKitDirectory, boolean validChecksum) throws Exception {
        return BinaryRepository.initScript(
                testKitDirectory.resolve("binary"),
                BinaryRepository.createFake(testKitDirectory.resolve("binary/repository"), validChecksum));
    }

    private static Path extracted(Path testKitDirectory) {
        RobustaPlatform platform = BinaryRepository.hostPlatform();
        return testKitDirectory
                .resolve("caches/robusta")
                .resolve(pluginVersion())
                .resolve(platform.triple())
                .resolve(platform.extractedBinaryName());
    }

    private static GradleRunner runner(
            Path project, Path initScript, Path testKitDirectory, String... arguments) {
        return FunctionalTestSupport.runner(project, initScript, testKitDirectory, arguments);
    }

    private static GradleRunner settingsRunner(Path project, Path testKitDirectory) {
        return GradleRunner.create()
                .withProjectDir(project.toFile())
                .withPluginClasspath()
                .withTestKitDir(testKitDirectory.toFile())
                .withArguments(":app:robustaVersion", "--configuration-cache");
    }

    private static String[] arguments(boolean configurationCache) {
        List<String> arguments = new ArrayList<>();
        arguments.add("robustaVersion");
        if (configurationCache) {
            arguments.add("--configuration-cache");
        }
        return arguments.toArray(String[]::new);
    }
}
