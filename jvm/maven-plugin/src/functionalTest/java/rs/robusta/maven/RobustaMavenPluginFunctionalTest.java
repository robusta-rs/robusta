package rs.robusta.maven;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.maven.FunctionalTestSupport.Result;
import static rs.robusta.maven.FunctionalTestSupport.createProject;
import static rs.robusta.maven.FunctionalTestSupport.goalCoordinate;
import static rs.robusta.maven.FunctionalTestSupport.minimalPom;
import static rs.robusta.maven.FunctionalTestSupport.runFakeMaven;
import static rs.robusta.maven.FunctionalTestSupport.runMaven;
import static rs.robusta.maven.FunctionalTestSupport.writeJavaSource;
import static rs.robusta.plugin.common.FakeRobusta.readInvocations;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import rs.robusta.plugin.common.FakeRobusta;
import rs.robusta.plugin.common.FakeRobusta.Invocation;
import rs.robusta.plugin.common.RobustaFixedCommand;
import rs.robusta.plugin.common.RobustaPlatform;

/**
 * Drives real {@code ./mvnw} builds against the plugin published by the {@code functionalTest}
 * Gradle task, proving the bundled binary is actually delivered to and invoked from a Maven
 * build, not just from unit-tested Java classes.
 */
class RobustaMavenPluginFunctionalTest {
    @TempDir Path temporaryDirectory;

    @Test
    void bundledBinaryFormatsACleanFile() throws Exception {
        Path project = createProject(temporaryDirectory, "clean", minimalPom("clean"));
        writeJavaSource(
                project,
                "src/main/java/example/Clean.java",
                """
                package example;

                final class Clean {
                  long value() {
                    return 1L;
                  }
                }
                """);

        Result result = runMaven(
                project, temporaryDirectory.resolve("local-repo"), goalCoordinate("format"));

        assertEquals(0, result.exitCode(), result.output());
    }

    @Test
    void bundledBinaryFailsOnMisformattedSource() throws Exception {
        Path project = createProject(temporaryDirectory, "dirty", minimalPom("dirty"));
        writeJavaSource(
                project,
                "src/main/java/example/Dirty.java",
                "package example;\nfinal class Dirty{long value(){return 1L;}}\n");

        Result result = runMaven(
                project, temporaryDirectory.resolve("local-repo"), goalCoordinate("format"));

        assertEquals(1, result.exitCode(), result.output());
        assertTrue(result.output().contains("BUILD FAILURE"), result.output());
    }

    @Test
    void extractsTheBundledBinaryIntoTheEffectiveLocalRepository() throws Exception {
        Path project = createProject(temporaryDirectory, "cache", minimalPom("cache"));
        Path localRepository = temporaryDirectory.resolve("redirected-repo");

        Result result = runMaven(project, localRepository, goalCoordinate("config-show"));

        assertEquals(0, result.exitCode(), result.output());
        RobustaPlatform platform =
                RobustaPlatform.detect(System.getProperty("os.name"), System.getProperty("os.arch"));
        Path extracted = localRepository
                .resolve(".cache/robusta")
                .resolve(System.getProperty("robusta.functionalTest.pluginVersion"))
                .resolve(platform.triple())
                .resolve(platform.extractedBinaryName());
        assertTrue(
                Files.isRegularFile(extracted),
                "a redirected -Dmaven.repo.local should carry the extracted binary with it; "
                        + "expected " + extracted);
    }

    @Test
    void theLintAndCheckGoalsRunTheirOwnCommands() throws Exception {
        Path project = createProject(temporaryDirectory, "verbs", minimalPom("verbs"));
        writeJavaSource(
                project,
                "src/main/java/example/Clean.java",
                """
                package example;

                final class Clean {
                  long value() {
                    return 1L;
                  }
                }
                """);
        Path localRepository = temporaryDirectory.resolve("local-repo");

        assertEquals(
                0,
                runMaven(project, localRepository, goalCoordinate("lint")).exitCode(),
                "lint on clean source");
        assertEquals(
                0,
                runMaven(project, localRepository, goalCoordinate("check")).exitCode(),
                "check on clean source");
        assertEquals(
                0,
                runMaven(project, localRepository, goalCoordinate("format-diff")).exitCode(),
                "format-diff on clean source");
    }

    /** The goals read only sources, so a dependency that cannot be resolved does not stop them. */
    @Test
    void theLintAndCheckGoalsResolveNoProjectDependencies() throws Exception {
        String pom = minimalPom("unresolvable").replace(
                "</project>",
                """
                  <dependencies>
                    <dependency>
                      <groupId>org.example</groupId>
                      <artifactId>absent</artifactId>
                      <version>1.0</version>
                    </dependency>
                  </dependencies>
                </project>""");
        Path project = createProject(temporaryDirectory, "unresolvable", pom);
        writeJavaSource(
                project,
                "src/main/java/example/Clean.java",
                """
                package example;

                final class Clean {}
                """);
        Path localRepository = temporaryDirectory.resolve("local-repo");

        for (String goal : new String[] {"lint", "check"}) {
            Result result = runMaven(project, localRepository, goalCoordinate(goal));
            assertEquals(0, result.exitCode(), goal + ":\n" + result.output());
        }
    }

    @Test
    void theRunGoalPassesItsArgumentsThrough() throws Exception {
        Path project = createProject(temporaryDirectory, "passthrough", minimalPom("passthrough"));

        Result result = runMaven(
                project,
                temporaryDirectory.resolve("local-repo"),
                "-Drobusta.args=lint --rules",
                goalCoordinate("run"));

        assertEquals(0, result.exitCode(), result.output());
        assertTrue(
                result.output().contains("modifier-order"),
                "the pass-through goal should reach `robusta lint --rules`:\n" + result.output());
    }

    @Test
    void everyFixedGoalAppendsQuotedArgumentsInOrder() throws Exception {
        Path project = createProject(temporaryDirectory, "fixed-args", minimalPom("fixed-args"));
        Path record = temporaryDirectory.resolve("arguments.txt");
        FakeRobusta.writeBehavior(temporaryDirectory, record, 0);
        List<String> extras = List.of("two words", "single quoted", "--option=value with spaces", "");
        for (RobustaFixedCommand command : RobustaFixedCommand.values()) {
            Files.deleteIfExists(record);
            Result result = runFakeMaven(
                    project,
                    temporaryDirectory.resolve("local-repo"),
                    "-Drobusta.args=\"two words\" 'single quoted' --option=\"value with spaces\" \"\"",
                    goalCoordinate(command.mavenGoalName()));

            assertEquals(0, result.exitCode(), result.output());
            List<String> expected = new ArrayList<>(command.arguments());
            if (!command.arguments().get(0).equals("config") && !command.arguments().get(0).equals("init")) {
                expected.add(".");
            }
            expected.addAll(extras);
            assertEquals(
                    List.of(new Invocation(project.toRealPath(), expected)),
                    readInvocations(record),
                    command.mavenGoalName());
        }

        Files.delete(record);
        Result result = runFakeMaven(
                project,
                temporaryDirectory.resolve("local-repo"),
                "-Drobusta.args=lint --config 'path with spaces/robusta.yaml'",
                goalCoordinate("run"));
        assertEquals(0, result.exitCode(), result.output());
        assertEquals(List.of(new Invocation(project.toRealPath(),
                List.of("lint", "--config", "path with spaces/robusta.yaml"))), readInvocations(record));
    }

    @Test
    void malformedArgsFailBeforeLaunchingAnyGoal() throws Exception {
        Path project = createProject(temporaryDirectory, "invalid-args", minimalPom("invalid-args"));
        Path record = temporaryDirectory.resolve("arguments.txt");
        FakeRobusta.writeBehavior(temporaryDirectory, record, 0);
        List<String> goals = new ArrayList<>();
        for (RobustaFixedCommand command : RobustaFixedCommand.values()) {
            goals.add(command.mavenGoalName());
        }
        goals.add("run");
        for (String goal : goals) {
            Result result = runFakeMaven(
                    project,
                    temporaryDirectory.resolve("local-repo"),
                    "-Drobusta.args=--config 'unterminated",
                    goalCoordinate(goal));

            assertEquals(1, result.exitCode(), result.output());
            assertTrue(result.output().contains("Cannot parse robusta.args: unmatched ' quote"), result.output());
            assertFalse(Files.exists(record), goal + " launched the executable");
        }
    }

    @Test
    void lintFixAppliesUnsafeFixesAndPrunesTheBaseline() throws Exception {
        Path project = createProject(temporaryDirectory, "unsafe-fixes", minimalPom("unsafe-fixes"));
        Files.writeString(project.resolve("robusta.yaml"),
                "lint:\n  rules:\n    unused-local-variable: deny\n    uppercase-long-suffix: deny\n");
        Path source = project.resolve("Example.java");
        Files.writeString(source,
                "class Example { void f() { int unused = send(); } int send() { return 1; } }\n");
        Path baseline = project.resolve("robusta-suppressions.json");
        Files.writeString(baseline, "{\"Example.java\":{\"uppercase-long-suffix\":{\"count\":1}}}\n");

        Result result = runMaven(
                project,
                temporaryDirectory.resolve("local-repo"),
                "-Drobusta.args=--unsafe-fixes --prune-suppressions",
                goalCoordinate("lint-fix"));

        assertEquals(0, result.exitCode(), result.output());
        assertEquals("class Example { void f() { send(); } int send() { return 1; } }\n", Files.readString(source));
        assertEquals("{}", Files.readString(baseline).trim());
    }

    @Test
    void configGoalsForwardTheirOptionsToTheNativeCli() throws Exception {
        Path project = createProject(temporaryDirectory, "config-args", minimalPom("config-args"));
        writeJavaSource(project, "source dir/Example.java", "class Example {}\n");
        Path localRepository = temporaryDirectory.resolve("local-repo");

        Result show = runMaven(
                project, localRepository, "-Drobusta.args=--files 'source dir'", goalCoordinate("config-show"));
        assertEquals(0, show.exitCode(), show.output());
        assertTrue(show.output().replace('\\', '/').contains("source dir/Example.java"), show.output());

        Result init = runMaven(project, localRepository, "-Drobusta.args=--help", goalCoordinate("config-init"));
        assertEquals(0, init.exitCode(), init.output());
        // Usage names the program by its file name, which is robusta.exe on Windows.
        String program = RobustaPlatform.detect(System.getProperty("os.name"), System.getProperty("os.arch"))
                .extractedBinaryName();
        assertTrue(init.output().contains("Usage: " + program + " init"), init.output());
        assertFalse(Files.exists(project.resolve("robusta.yaml")));
    }

    /** A standalone project's source and pass-through goals share its baseline. */
    @Test
    void theLintGoalEnforcesABaselineThePassThroughGoalRecords() throws Exception {
        Path project = createProject(temporaryDirectory, "baseline", minimalPom("baseline"));
        Files.writeString(
                project.resolve("robusta.yaml"),
                "lint:\n  rules:\n    uppercase-long-suffix: deny\n",
                StandardCharsets.UTF_8);
        writeJavaSource(
                project,
                "src/main/java/example/Legacy.java",
                """
                package example;

                final class Legacy {
                    long value() {
                        return 1l;
                    }
                }
                """);
        Path localRepository = temporaryDirectory.resolve("local-repo");

        Result unbaselined = runMaven(project, localRepository, goalCoordinate("lint"));
        assertEquals(1, unbaselined.exitCode(), unbaselined.output());

        Result suppress = runMaven(
                project, localRepository, "-Drobusta.args=lint --suppress-all", goalCoordinate("run"));
        assertEquals(0, suppress.exitCode(), suppress.output());
        assertEquals(
                """
                {
                  "src/main/java/example/Legacy.java": {
                    "uppercase-long-suffix": {
                      "count": 1
                    }
                  }
                }
                """,
                Files.readString(project.resolve("robusta-suppressions.json"), StandardCharsets.UTF_8));

        Result enforced = runMaven(project, localRepository, goalCoordinate("lint"));
        assertEquals(0, enforced.exitCode(), enforced.output());
    }
}
