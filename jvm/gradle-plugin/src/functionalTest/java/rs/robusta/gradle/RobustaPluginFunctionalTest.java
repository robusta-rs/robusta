package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.createPluginProject;
import static rs.robusta.gradle.FunctionalTestSupport.kotlinString;
import static rs.robusta.gradle.FunctionalTestSupport.realRunner;
import static rs.robusta.gradle.FunctionalTestSupport.runner;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.GradleRunner;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

class RobustaPluginFunctionalTest {
    @TempDir Path temporaryDirectory;

    @Test
    void passesArgumentsWorkingDirectoryAndStreamsOutput() throws Exception {
        Path project = createPluginProject(temporaryDirectory, "arguments");
        Path record = project.resolve("record file.txt");

        BuildResult result =
                runner(
                                project,
                                "robustaRun",
                                "--args=--record=\""
                                        + record
                                        + "\" alpha \"two words\" "
                                        + "--option=\"value with spaces\" "
                                        + "\"path with spaces/Example.java\"")
                        .build();

        assertTrue(result.getOutput().contains("fake robusta stdout"));
        assertTrue(result.getOutput().contains("fake robusta stderr"));
        assertEquals(
                List.of(
                        "cwd=" + project.toRealPath(),
                        "arg[0]=--record=" + record,
                        "arg[1]=alpha",
                        "arg[2]=two words",
                        "arg[3]=--option=value with spaces",
                        "arg[4]=path with spaces/Example.java"),
                Files.readAllLines(record, StandardCharsets.UTF_8));
    }

    @Test
    void acceptsArgumentListConfiguredInBuildScript() throws Exception {
        Path project = createPluginProject(temporaryDirectory, "configured-arguments");
        Path record = project.resolve("configured record.txt");
        Files.writeString(
                project.resolve("build.gradle.kts"),
                """
                plugins { id("rs.robusta") }

                tasks.named<rs.robusta.gradle.RobustaExecTask>("robustaRun") {
                    arguments.set(listOf("--record=%s", "configured value"))
                }
                """
                        .formatted(kotlinString(record)),
                StandardCharsets.UTF_8);

        runner(project, "robustaRun").build();

        assertEquals(
                List.of(
                        "cwd=" + project.toRealPath(),
                        "arg[0]=--record=" + record,
                        "arg[1]=configured value"),
                Files.readAllLines(record, StandardCharsets.UTF_8));
    }

    @Test
    void mapsVerificationAndOperationalExitCodes() throws Exception {
        Path project = createPluginProject(temporaryDirectory, "exit-codes");

        BuildResult verificationFailure =
                runner(
                                project,
                                "robustaRun",
                                "--args=--exit=1")
                        .buildAndFail();
        assertTrue(
                verificationFailure
                        .getOutput()
                        .contains(
                                "Robusta reported problems in the analyzed source (exit code 1)."));

        BuildResult operationalFailure =
                runner(
                                project,
                                "robustaRun",
                                "--args=--exit=2")
                        .buildAndFail();
        assertTrue(
                operationalFailure
                        .getOutput()
                        .contains("Robusta failed with operational exit code 2."));
    }

    @Test
    void reusesConfigurationCache() throws Exception {
        Path project = createPluginProject(temporaryDirectory, "configuration-cache");

        GradleRunner firstRunner =
                runner(
                        project,
                        "robustaRun",
                        "--args=--version",
                        "--configuration-cache");
        BuildResult first = firstRunner.build();
        assertTrue(first.getOutput().contains("Configuration cache entry stored"));

        BuildResult second =
                runner(
                                project,
                                "robustaRun",
                                "--args=--version",
                                "--configuration-cache")
                        .build();
        assertTrue(second.getOutput().contains("Reusing configuration cache"));
        assertTrue(second.getOutput().contains("Configuration cache entry reused"));
    }

    @Test
    void runsTheRealBundledRobustaCliWhenSupplied() throws IOException {
        Path project = createPluginProject(temporaryDirectory, "real-bundled-robusta");

        BuildResult version = realRunner(
                        project,
                        "robustaRun",
                        "--args=--version",
                        "--configuration-cache")
                .build();
        assertTrue(version.getOutput().contains("Version: "));
        assertTrue(version.getOutput().contains("Configuration cache entry stored"));

        BuildResult reusedVersion = realRunner(
                        project,
                        "robustaRun",
                        "--args=--version",
                        "--configuration-cache")
                .build();
        assertTrue(reusedVersion.getOutput().contains("Version: "));
        assertTrue(reusedVersion.getOutput().contains("Configuration cache entry reused"));

        BuildResult rules = realRunner(project, "robustaRun", "--args=lint --rules").build();
        assertTrue(rules.getOutput().contains("modifier-order"));
    }
}
