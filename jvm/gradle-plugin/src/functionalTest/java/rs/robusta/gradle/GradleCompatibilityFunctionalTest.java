package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.createPluginProject;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Stream;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.GradleRunner;
import org.gradle.testkit.runner.TaskOutcome;
import org.gradle.util.GradleVersion;
import org.junit.jupiter.api.io.TempDir;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.MethodSource;

import rs.robusta.plugin.common.FakeRobusta;

class GradleCompatibilityFunctionalTest {
    private static final String VERSIONS_PROPERTY = "robusta.test.gradleVersions";

    @TempDir Path temporaryDirectory;

    @ParameterizedTest(name = "Gradle {0}")
    @MethodSource("gradleVersions")
    void runsReadOnlyMutatingAndConfigurationCacheWorkflows(String gradleVersion)
            throws Exception {
        Path project = createPluginProject(temporaryDirectory, "gradle-" + gradleVersion);
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        BuildResult readOnly = runner(project, gradleVersion, "robustaCheck").build();
        assertEquals(TaskOutcome.SUCCESS, readOnly.task(":robustaCheck").getOutcome());
        assertEquals(List.of("arg[0]=check"), recordedArguments(record));

        truncate(record);
        BuildResult mutating = runner(
            project,
            gradleVersion,
            "robustaFormatFix"
        ).build();
        assertEquals(TaskOutcome.SUCCESS, mutating.task(":robustaFormatFix").getOutcome());
        assertEquals(List.of("arg[0]=format", "arg[1]=--fix"), recordedArguments(record));

        truncate(record);
        BuildResult stored = runner(
            project,
            gradleVersion,
            "robustaCheck",
            "--configuration-cache"
        ).build();
        BuildResult reused = runner(
            project,
            gradleVersion,
            "robustaCheck",
            "--configuration-cache"
        ).build();

        assertTrue(stored.getOutput().contains("Configuration cache entry stored"));
        assertTrue(reused.getOutput().contains("Reusing configuration cache"));
        assertTrue(reused.getOutput().contains("Configuration cache entry reused"));
        assertEquals(
            List.of("arg[0]=check", "arg[0]=check"),
            recordedArguments(record)
        );
    }

    private static GradleRunner runner(Path project, String gradleVersion, String... arguments) {
        return FunctionalTestSupport.runner(project, arguments).withGradleVersion(gradleVersion);
    }

    private static Stream<String> gradleVersions() {
        String current = GradleVersion.current().getVersion();
        return Arrays.stream(System.getProperty(VERSIONS_PROPERTY, current).split(","))
            .map(String::trim)
            .filter(version -> !version.isEmpty())
            .distinct();
    }

    private static List<String> recordedArguments(Path record) throws IOException {
        return Files.readAllLines(record, StandardCharsets.UTF_8).stream()
            .filter(line -> line.startsWith("arg["))
            .toList();
    }

    private static void truncate(Path record) throws IOException {
        Files.writeString(
            record,
            "",
            StandardCharsets.UTF_8,
            StandardOpenOption.TRUNCATE_EXISTING
        );
    }
}
