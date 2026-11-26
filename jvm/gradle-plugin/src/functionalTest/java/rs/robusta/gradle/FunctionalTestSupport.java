package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.File;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Locale;
import java.util.stream.Stream;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.GradleRunner;
import org.junit.jupiter.api.Assumptions;

import rs.robusta.plugin.common.BundledExecutable;
import rs.robusta.plugin.common.RobustaPluginException;

final class FunctionalTestSupport {
    private static final String FAKE_BUNDLE_PROPERTY = "robusta.fake.bundle";
    private static final String REAL_BUNDLED_PROPERTY = "robusta.real.bundled";
    private static final String TEST_KIT_ROOT_PROPERTY = "robusta.testkit.root";
    private static final String PLUGIN_BUILD = "plugins { id(\"rs.robusta\") }\n";

    private FunctionalTestSupport() {}

    static Path createPluginProject(Path parent, String name) throws IOException {
        return createProject(parent, name, PLUGIN_BUILD);
    }

    static Path createProject(Path parent, String name, String buildScript) throws IOException {
        Path project = Files.createDirectories(parent.resolve(name));
        Files.writeString(
                project.resolve("settings.gradle.kts"),
                "rootProject.name = \"fixture\"\n",
                StandardCharsets.UTF_8);
        Files.writeString(
                project.resolve("build.gradle.kts"), buildScript, StandardCharsets.UTF_8);
        return project;
    }

    static void copyFixture(Class<?> resourceOwner, String fixture, Path project)
            throws IOException {
        URL resource = resourceOwner.getResource("/fixtures/" + fixture);
        if (resource == null) {
            throw new IOException("Missing functional-test fixture " + fixture);
        }

        Path source;
        try {
            source = Path.of(resource.toURI());
        } catch (URISyntaxException error) {
            throw new IOException("Invalid fixture URI for " + fixture, error);
        }

        try (Stream<Path> paths = Files.walk(source)) {
            for (Path path : paths.toList()) {
                Path destination = project.resolve(source.relativize(path));
                if (Files.isDirectory(path)) {
                    Files.createDirectories(destination);
                } else {
                    Files.copy(path, destination);
                }
            }
        }
    }

    /** Runs the plugin with the native fake as its bundled binary. */
    static GradleRunner runner(Path project, String... arguments) {
        return runner(project, FakeBundle.CLASSPATH, SharedTestKitDirectories.FAKE, arguments);
    }

    /**
     * Runs the plugin with the real CLI the build bundled. Both bundles extract to the same
     * cache path, so each keeps its own TestKit directory.
     */
    static GradleRunner realRunner(Path project, String... arguments) {
        assumeRealBundle();
        return runner(project, mutableArguments(arguments))
                .withPluginClasspath()
                .withTestKitDir(SharedTestKitDirectories.REAL.toFile());
    }

    static GradleRunner runner(
            Path project,
            Iterable<? extends File> pluginClasspath,
            Path testKitDirectory,
            String... arguments) {
        return runner(project, mutableArguments(arguments))
                .withPluginClasspath(pluginClasspath)
                .withTestKitDir(testKitDirectory.toFile());
    }

    /**
     * Extracts the real CLI the build bundled, for tests that also run it directly, to where
     * {@link #realRunner} has the plugin extract it.
     */
    static Path realExecutable() {
        assumeRealBundle();
        try {
            return new BundledExecutable("Gradle").obtain(
                    SharedTestKitDirectories.REAL.resolve("caches/robusta"),
                    System.getProperty("os.name"),
                    System.getProperty("os.arch"),
                    "development");
        } catch (RobustaPluginException failure) {
            throw new IllegalStateException(failure.getMessage(), failure);
        }
    }

    /** Returns the directory the suite's Gradle task staged the native fake's bundle in. */
    static Path fakeBundle() {
        String bundle = System.getProperty(FAKE_BUNDLE_PROPERTY);
        if (bundle == null) {
            throw new IllegalStateException(
                    "Run the suite through its Gradle task, which stages the fake bundle");
        }
        return Path.of(bundle);
    }

    static Path newTestKitDirectory(String name) throws IOException {
        String configuredRoot = System.getProperty(TEST_KIT_ROOT_PROPERTY);
        Path root = configuredRoot == null
                ? Path.of(System.getProperty("java.io.tmpdir"))
                : Path.of(configuredRoot);
        Files.createDirectories(root);
        return Files.createTempDirectory(root, "robusta-" + name + "-");
    }

    static void assumeRealBundle() {
        Assumptions.assumeTrue(
                Boolean.getBoolean(REAL_BUNDLED_PROPERTY),
                "Pass -Drobusta.real.bundled=true with bundled resources to run acceptance checks");
    }

    static BuildResult assertTaskHelp(Path project, String task, String description) {
        BuildResult result = runner(project, "help", "--task", task).build();
        // Gradle renders help with the platform line separator, so the task-path
        // block arrives CRLF-delimited on Windows.
        String output = result.getOutput().replace("\r\n", "\n");
        assertTrue(output.contains("Path\n     :" + task), result.getOutput());
        assertTrue(output.contains(description), result.getOutput());
        return result;
    }

    static int occurrences(String value, String needle) {
        return (value.length() - value.replace(needle, "").length()) / needle.length();
    }

    static boolean isWindows() {
        return System.getProperty("os.name").toLowerCase(Locale.ROOT).contains("windows");
    }

    static String kotlinString(Path value) {
        return value.toString().replace("\\", "\\\\").replace("\"", "\\\"");
    }

    private static GradleRunner runner(Path project, List<String> arguments) {
        return GradleRunner.create()
                .withProjectDir(project.toFile())
                .withArguments(arguments);
    }

    private static List<String> mutableArguments(String... arguments) {
        return new ArrayList<>(Arrays.asList(arguments));
    }

    private static final class FakeBundle {
        private static final List<File> CLASSPATH = load();

        private FakeBundle() {}

        private static List<File> load() {
            try {
                return BundledPluginClasspath.pluginClasspath(fakeBundle());
            } catch (IOException error) {
                throw new UncheckedIOException("Failed to read the plugin classpath", error);
            }
        }
    }

    private static final class SharedTestKitDirectories {
        private static final Path FAKE = create("shared");
        private static final Path REAL = create("real");

        private SharedTestKitDirectories() {}

        private static Path create(String name) {
            try {
                return newTestKitDirectory(name);
            } catch (IOException error) {
                throw new UncheckedIOException(
                        "Failed to create the shared TestKit directory", error);
            }
        }
    }
}
