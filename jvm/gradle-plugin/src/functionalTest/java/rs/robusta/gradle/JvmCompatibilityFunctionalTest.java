package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.createProject;
import static rs.robusta.gradle.FunctionalTestSupport.runner;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Stream;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.TaskOutcome;
import org.gradle.util.GradleVersion;
import org.junit.jupiter.api.io.TempDir;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import rs.robusta.plugin.common.FakeRobusta;

/**
 * Runs the published plugin classes on each supported consumer JVM.
 *
 * <p>The build supplies {@code <java version>|<gradle version>|<java home>} entries for every
 * installed target, so a machine without an old JDK verifies fewer JVMs rather than failing.
 * Without those entries the test covers the JVM and Gradle version it is already running on.
 */
class JvmCompatibilityFunctionalTest {
    private static final String TARGETS_PROPERTY = "robusta.test.jvmTargets";
    // The consumer build reports the JVM that loaded the plugin, so a daemon that ignored the
    // requested Java home fails the case instead of passing it on the JVM running the tests.
    private static final String REPORTED_JVM = "robusta-consumer-jvm=";
    private static final String BUILD_SCRIPT = "plugins { id(\"rs.robusta\") }\n"
            + "println(\"" + REPORTED_JVM + "\" + System.getProperty(\"java.specification.version\"))\n";

    @TempDir Path temporaryDirectory;

    @ParameterizedTest(name = "Java {0} on Gradle {1}")
    @MethodSource("targets")
    void loadsAndExecutesOnConsumerJvm(String javaVersion, String gradleVersion, String javaHome)
            throws Exception {
        Path project = createProject(temporaryDirectory, "jvm-" + javaVersion, BUILD_SCRIPT);
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);
        writeDaemonJavaHome(project, javaHome);

        BuildResult result = runner(project, "robustaCheck")
            .withGradleVersion(gradleVersion)
            .build();

        assertEquals(TaskOutcome.SUCCESS, result.task(":robustaCheck").getOutcome());
        assertEquals(List.of("arg[0]=check"), recordedArguments(record));
        assertTrue(
            result.getOutput().contains(REPORTED_JVM + specificationVersion(javaVersion)),
            result.getOutput()
        );
    }

    private static String specificationVersion(String javaVersion) {
        return "8".equals(javaVersion) ? "1.8" : javaVersion;
    }

    private static void writeDaemonJavaHome(Path project, String javaHome) throws IOException {
        Files.writeString(
            project.resolve("gradle.properties"),
            "org.gradle.java.home=" + javaHome.replace("\\", "\\\\") + "\n",
            StandardCharsets.UTF_8
        );
    }

    private static Stream<Arguments> targets() {
        List<Arguments> configured =
            Arrays.stream(System.getProperty(TARGETS_PROPERTY, "").split(","))
                .map(String::trim)
                .filter(target -> !target.isEmpty())
                .map(target -> target.split("\\|", 3))
                .map(fields -> Arguments.of(fields[0], fields[1], fields[2]))
                .toList();
        if (!configured.isEmpty()) {
            return configured.stream();
        }
        return Stream.of(Arguments.of(
            System.getProperty("java.specification.version"),
            GradleVersion.current().getVersion(),
            System.getProperty("java.home")
        ));
    }

    private static List<String> recordedArguments(Path record) throws IOException {
        return Files.readAllLines(record, StandardCharsets.UTF_8).stream()
            .filter(line -> line.startsWith("arg["))
            .toList();
    }
}
