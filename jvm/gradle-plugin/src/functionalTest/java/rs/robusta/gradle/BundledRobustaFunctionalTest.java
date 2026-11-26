package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.createPluginProject;
import static rs.robusta.gradle.FunctionalTestSupport.newTestKitDirectory;

import java.io.IOException;
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
import org.junit.jupiter.api.io.TempDir;

import rs.robusta.plugin.common.RobustaPlatform;

class BundledRobustaFunctionalTest {
    @TempDir Path temporaryDirectory;

    @Test
    void executesBundledBinaryWithNoConfigurationAndReusesOnlyVerifiedExtraction()
            throws Exception {
        Path project = createPluginProject(temporaryDirectory, "bundled");
        Path testKitDirectory = newTestKitDirectory("bundled");
        Path resources = BundledPluginClasspath.create(testKitDirectory.resolve("bundle"), true);

        BuildResult first =
                runner(project, resources, testKitDirectory, bundledArguments(true)).build();
        assertTrue(first.getOutput().contains("fake robusta stdout"));
        assertTrue(first.getOutput().contains("Configuration cache entry stored"));

        RobustaPlatform platform = BundledPluginClasspath.hostPlatform();
        Path extracted = testKitDirectory
                .resolve("caches/robusta/development")
                .resolve(platform.triple())
                .resolve(platform.extractedBinaryName());
        assertTrue(Files.isRegularFile(extracted));
        FileTime sentinel = FileTime.fromMillis(1_700_000_000_000L);
        Files.setLastModifiedTime(extracted, sentinel);

        BuildResult second =
                runner(project, resources, testKitDirectory, bundledArguments(true)).build();
        assertTrue(second.getOutput().contains("fake robusta stdout"));
        assertTrue(second.getOutput().contains("Reusing configuration cache"));
        assertTrue(second.getOutput().contains("Configuration cache entry reused"));
        assertEquals(sentinel, Files.getLastModifiedTime(extracted));

        byte[] verified = Files.readAllBytes(extracted);
        Files.writeString(extracted, "corrupted cache", StandardCharsets.UTF_8);
        BuildResult corruptedCache =
                runner(project, resources, testKitDirectory, bundledArguments(true)).build();
        assertTrue(corruptedCache.getOutput().contains("fake robusta stdout"));
        assertTrue(corruptedCache.getOutput().contains("Configuration cache entry reused"));
        assertArrayEquals(verified, Files.readAllBytes(extracted));
    }

    @Test
    void rejectsCorruptedBundledBinaryBeforeExecution() throws Exception {
        Path project = createPluginProject(temporaryDirectory, "corrupt");
        Path testKitDirectory = newTestKitDirectory("corrupt");
        Path resources = BundledPluginClasspath.create(testKitDirectory.resolve("bundle"), false);

        BuildResult failure = runner(
                        project, resources, testKitDirectory, bundledArguments(false))
                .buildAndFail();

        assertTrue(failure.getOutput().contains("Bundled Robusta checksum mismatch"));
        assertTrue(failure.getOutput().contains("expected " + "0".repeat(64)));
        assertTrue(failure.getOutput().contains("actual "));
        RobustaPlatform platform = BundledPluginClasspath.hostPlatform();
        assertTrue(Files.notExists(testKitDirectory
                .resolve("caches/robusta/development")
                .resolve(platform.triple())
                .resolve(platform.extractedBinaryName())));
    }

    @Test
    void concurrentBuildsShareOneAtomicExtraction() throws Exception {
        Path firstProject = createPluginProject(temporaryDirectory, "concurrent-a");
        Path secondProject = createPluginProject(temporaryDirectory, "concurrent-b");
        Path testKitDirectory = newTestKitDirectory("concurrent");
        Path resources = BundledPluginClasspath.create(testKitDirectory.resolve("bundle"), true);
        ExecutorService executor = Executors.newFixedThreadPool(2);
        try {
            Future<BuildResult> first = executor.submit(() -> runner(
                    firstProject, resources, testKitDirectory, bundledArguments(false)).build());
            Future<BuildResult> second = executor.submit(() -> runner(
                    secondProject, resources, testKitDirectory, bundledArguments(false)).build());

            assertTrue(first.get(2, TimeUnit.MINUTES).getOutput().contains("fake robusta stdout"));
            assertTrue(second.get(2, TimeUnit.MINUTES).getOutput().contains("fake robusta stdout"));
        } finally {
            executor.shutdownNow();
        }

        RobustaPlatform platform = BundledPluginClasspath.hostPlatform();
        Path extracted = testKitDirectory
                .resolve("caches/robusta/development")
                .resolve(platform.triple())
                .resolve(platform.extractedBinaryName());
        assertTrue(Files.isRegularFile(extracted));
        assertTrue(Files.size(extracted) > 0);
    }

    private GradleRunner runner(
            Path project, Path bundledResources, Path testKitDirectory, String... arguments)
            throws IOException {
        return FunctionalTestSupport.runner(
                project,
                BundledPluginClasspath.pluginClasspath(bundledResources),
                testKitDirectory,
                arguments);
    }

    private static String[] bundledArguments(boolean configurationCache) {
        List<String> arguments = new ArrayList<>();
        arguments.add("robustaVersion");
        if (configurationCache) {
            arguments.add("--configuration-cache");
        }
        return arguments.toArray(String[]::new);
    }
}
