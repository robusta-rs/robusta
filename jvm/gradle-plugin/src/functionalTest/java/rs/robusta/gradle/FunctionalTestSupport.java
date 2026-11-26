package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertTrue;

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

import rs.robusta.plugin.common.RobustaBinary;
import rs.robusta.plugin.common.RobustaPlatform;
import rs.robusta.plugin.common.RobustaPluginException;

final class FunctionalTestSupport {
    private static final String FAKE_EXECUTABLE_PROPERTY = "robusta.fake.executable";
    private static final String PLUGIN_VERSION_PROPERTY = "robusta.plugin.version";
    private static final String BINARY_REPOSITORY_PROPERTY = "robusta.binary.repository";
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

    /** Runs the plugin with the native fake as its binary. */
    static GradleRunner runner(Path project, String... arguments) {
        return runner(project, FakeRepository.INIT_SCRIPT, SharedTestKitDirectories.FAKE, arguments);
    }

    /**
     * Runs the plugin with the real CLI the build published. Both binaries extract to the same
     * cache path, so each keeps its own TestKit directory.
     */
    static GradleRunner realRunner(Path project, String... arguments) {
        assumeRealBundle();
        return runner(project, RealRepository.INIT_SCRIPT, SharedTestKitDirectories.REAL, arguments);
    }

    /**
     * Runs the plugin with the binary jar from the repository that {@code initScript} adds to the
     * plugin repositories.
     */
    static GradleRunner runner(
            Path project, Path initScript, Path testKitDirectory, String... arguments) {
        List<String> allArguments = mutableArguments(arguments);
        allArguments.add("--init-script");
        allArguments.add(initScript.toString());
        return runner(project, allArguments)
                .withPluginClasspath()
                .withTestKitDir(testKitDirectory.toFile());
    }

    /**
     * Extracts the real CLI the build published, for tests that also run it directly, to where
     * {@link #realRunner} has the plugin extract it.
     */
    static Path realExecutable() {
        assumeRealBundle();
        RobustaPlatform platform = BinaryRepository.hostPlatform();
        Path module = Path.of(System.getProperty(BINARY_REPOSITORY_PROPERTY))
                .resolve(RobustaBinary.GROUP.replace('.', '/'))
                .resolve(RobustaBinary.ARTIFACT)
                .resolve(pluginVersion());
        try (Stream<Path> files = Files.list(module)) {
            Path jar = files
                    .filter(file -> file.getFileName().toString().endsWith("-" + platform.triple() + ".jar"))
                    .findFirst()
                    .orElseThrow(() -> new IllegalStateException("No published binary jar in " + module));
            return RobustaBinary.obtain(
                    jar, platform, SharedTestKitDirectories.REAL.resolve("caches/robusta"), pluginVersion());
        } catch (IOException error) {
            throw new UncheckedIOException(error);
        } catch (RobustaPluginException failure) {
            throw new IllegalStateException(failure.getMessage(), failure);
        }
    }

    /** Returns the native fake the suite's Gradle task names. */
    static Path fakeExecutable() {
        return Path.of(requiredProperty(FAKE_EXECUTABLE_PROPERTY));
    }

    /** Returns the version of the plugin under test, which its binary jar shares. */
    static String pluginVersion() {
        return requiredProperty(PLUGIN_VERSION_PROPERTY);
    }

    private static String requiredProperty(String name) {
        String value = System.getProperty(name);
        if (value == null) {
            throw new IllegalStateException(
                    "Run the suite through its Gradle task, which sets " + name);
        }
        return value;
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

    private static final class FakeRepository {
        private static final Path INIT_SCRIPT = create();

        private FakeRepository() {}

        private static Path create() {
            Path directory = SharedTestKitDirectories.FAKE.resolve("binary");
            try {
                return BinaryRepository.initScript(
                        directory, BinaryRepository.createFake(directory.resolve("repository"), true));
            } catch (IOException error) {
                throw new UncheckedIOException("Failed to create the fake binary repository", error);
            }
        }
    }

    private static final class RealRepository {
        private static final Path INIT_SCRIPT = create();

        private RealRepository() {}

        private static Path create() {
            try {
                return BinaryRepository.initScript(
                        SharedTestKitDirectories.REAL.resolve("binary"),
                        Path.of(requiredProperty(BINARY_REPOSITORY_PROPERTY)));
            } catch (IOException error) {
                throw new UncheckedIOException("Failed to write the binary repository init script", error);
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
