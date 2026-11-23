package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.assertTaskHelp;
import static rs.robusta.gradle.FunctionalTestSupport.createProject;
import static rs.robusta.gradle.FunctionalTestSupport.occurrences;
import static rs.robusta.gradle.FunctionalTestSupport.realRunner;
import static rs.robusta.gradle.FunctionalTestSupport.runner;
import static rs.robusta.plugin.common.FakeRobusta.readInvocations;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.TaskOutcome;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import rs.robusta.plugin.common.FakeRobusta;
import rs.robusta.plugin.common.FakeRobusta.Invocation;

class RobustaConfigFunctionalTest {
    private static final String CONFIGURATION_CACHE = "--configuration-cache";
    private static final String PLUGIN_BUILD = "plugins { id(\"rs.robusta\") }\n";
    private static final String SHOW_DESCRIPTION =
        "Prints the Robusta configuration resolved from this project directory.";
    private static final String INIT_DESCRIPTION =
        "Creates a starter robusta.yaml in this project directory.";

    @TempDir Path temporaryDirectory;

    @Test
    void fixedConfigTasksPassExactArgumentsFromTheOwningProjectAndForwardOutput()
            throws Exception {
        Path project = createSingleProject("fake-arguments");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        BuildResult result = runner(
            project,
            "robustaConfigShow",
            "robustaConfigInit"
        ).build();

        assertEquals(TaskOutcome.SUCCESS, result.task(":robustaConfigShow").getOutcome());
        assertEquals(TaskOutcome.SUCCESS, result.task(":robustaConfigInit").getOutcome());
        assertEquals(2, occurrences(result.getOutput(), "fake robusta stdout"));
        assertEquals(2, occurrences(result.getOutput(), "fake robusta stderr"));
        Path cwd = project.toRealPath();
        assertEquals(
            List.of(
                new Invocation(cwd, List.of("config")),
                new Invocation(cwd, List.of("init"))
            ),
            readInvocations(record)
        );
    }

    @Test
    void fixedConfigTasksAppendTheirArgsAfterTheFixedArguments() throws Exception {
        Path project = createSingleProject("fixed-args");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        runner(
            project,
            "robustaConfigShow",
            "--args=--config \"alternate config.yaml\" --no-nested-config",
            "robustaConfigInit",
            "--args=--color=never"
        ).build();

        Path cwd = project.toRealPath();
        assertEquals(
            List.of(
                new Invocation(
                    cwd,
                    List.of("config", "--config", "alternate config.yaml", "--no-nested-config")
                ),
                new Invocation(cwd, List.of("init", "--color=never"))
            ),
            readInvocations(record)
        );
    }

    @Test
    void fixedConfigTasksPreserveSharedExitCodeMapping() throws Exception {
        Path verificationProject = createSingleProject("verification-exit");
        FakeRobusta.writeBehavior(verificationProject, null, 1);
        BuildResult verificationFailure = runner(
            verificationProject,
            "robustaConfigShow"
        ).buildAndFail();
        assertTrue(
            verificationFailure.getOutput()
                .contains("Robusta reported problems in the analyzed source (exit code 1).")
        );

        Path operationalProject = createSingleProject("operational-exit");
        FakeRobusta.writeBehavior(operationalProject, null, 2);
        BuildResult operationalFailure = runner(
            operationalProject,
            "robustaConfigInit"
        ).buildAndFail();
        assertTrue(
            operationalFailure.getOutput()
                .contains("Robusta failed with operational exit code 2.")
        );
    }

    @Test
    void fixedConfigTasksExecuteEveryTimeAndReuseTheConfigurationCache() throws Exception {
        Path project = createSingleProject("untracked");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        assertAlwaysRunsWithConfigurationCache(project, "robustaConfigShow");
        assertAlwaysRunsWithConfigurationCache(project, "robustaConfigInit");

        assertEquals(
            List.of(
                List.of("config"),
                List.of("config"),
                List.of("init"),
                List.of("init")
            ),
            readInvocations(record).stream().map(Invocation::arguments).toList()
        );
    }

    @Test
    void taskListingAndHelpExposeConfigConveniencesWithoutAConfigDsl() throws Exception {
        Path project = createSingleProject("task-ux");

        BuildResult tasks = runner(project, "tasks", "--group", "robusta").build();
        assertTrue(
            tasks.getOutput().contains("robustaConfigShow - " + SHOW_DESCRIPTION)
        );
        assertTrue(
            tasks.getOutput().contains("robustaConfigInit - " + INIT_DESCRIPTION)
        );

        BuildResult showHelp = assertTaskHelp(
            project,
            "robustaConfigShow",
            SHOW_DESCRIPTION
        );
        BuildResult initHelp = assertTaskHelp(
            project,
            "robustaConfigInit",
            INIT_DESCRIPTION
        );
        for (BuildResult help : List.of(showHelp, initHelp)) {
            assertTrue(help.getOutput().contains("--args"));
            assertFalse(help.getOutput().contains("--config"));
            assertFalse(help.getOutput().contains("--no-nested-config"));
            assertFalse(help.getOutput().contains("--force"));
            assertFalse(help.getOutput().contains("sourceRoots"));
        }

        BuildResult allTasks = runner(project, "tasks", "--all").build();
        assertEquals(1, occurrences(allTasks.getOutput(), "robustaConfigShow -"));
        assertEquals(1, occurrences(allTasks.getOutput(), "robustaConfigInit -"));
    }

    @Test
    void realEngineShowsTheNearestConfigForEachProjectCwdWithoutAnalyzingSources()
            throws Exception {
        Path project = createMultiProject("real-show");
        Files.writeString(
            project.resolve("robusta.yaml"),
            "format:\n  line-width: 120\n",
            StandardCharsets.UTF_8
        );
        Files.writeString(
            project.resolve("app/robusta.yaml"),
            "format:\n  line-width: 88\n",
            StandardCharsets.UTF_8
        );
        Files.writeString(
            project.resolve("Broken.java"),
            "not java\n",
            StandardCharsets.UTF_8
        );
        Files.writeString(
            project.resolve("app/Broken.java"),
            "not java\n",
            StandardCharsets.UTF_8
        );

        BuildResult root = realRunner(
            project,
            ":robustaConfigShow"
        ).build();
        assertEquals(TaskOutcome.SUCCESS, root.task(":robustaConfigShow").getOutcome());
        assertNull(root.task(":app:robustaConfigShow"));
        assertTrue(root.getOutput().contains("line-width: 120"));
        assertFalse(root.getOutput().contains("line-width: 88"));

        BuildResult app = realRunner(
            project,
            ":app:robustaConfigShow",
            CONFIGURATION_CACHE
        ).build();
        assertNull(app.task(":robustaConfigShow"));
        assertEquals(TaskOutcome.SUCCESS, app.task(":app:robustaConfigShow").getOutcome());
        assertTrue(app.getOutput().contains("line-width: 88"));
        assertTrue(app.getOutput().contains("Configuration cache entry stored"));

        BuildResult reusedApp = realRunner(
            project,
            ":app:robustaConfigShow",
            CONFIGURATION_CACHE
        ).build();
        assertEquals(TaskOutcome.SUCCESS, reusedApp.task(":app:robustaConfigShow").getOutcome());
        assertTrue(reusedApp.getOutput().contains("Reusing configuration cache"));
        assertTrue(reusedApp.getOutput().contains("Configuration cache entry reused"));
        assertFalse(reusedApp.getOutput().contains(":app:robustaConfigShow UP-TO-DATE"));
    }

    @Test
    void realEngineInitCreatesOnlyTheQualifiedProjectConfigAndRefusesOverwrite()
            throws Exception {
        Path project = createMultiProject("qualified-init");

        BuildResult root = realRunner(
            project,
            ":robustaConfigInit",
            CONFIGURATION_CACHE
        ).build();
        Path rootConfig = project.resolve("robusta.yaml");
        Path appConfig = project.resolve("app/robusta.yaml");
        Path plainConfig = project.resolve("plain/robusta.yaml");
        assertEquals(TaskOutcome.SUCCESS, root.task(":robustaConfigInit").getOutcome());
        assertNull(root.task(":app:robustaConfigInit"));
        assertTrue(root.getOutput().contains("Created robusta.yaml"));
        assertTrue(Files.isRegularFile(rootConfig));
        assertFalse(Files.exists(appConfig));
        assertFalse(Files.exists(plainConfig));
        String rootBytes = Files.readString(rootConfig, StandardCharsets.UTF_8);

        BuildResult refusal = realRunner(
            project,
            ":robustaConfigInit",
            CONFIGURATION_CACHE
        ).buildAndFail();
        assertEquals(TaskOutcome.FAILED, refusal.task(":robustaConfigInit").getOutcome());
        assertTrue(refusal.getOutput().contains("robusta.yaml already exists"));
        assertFalse(refusal.getOutput().contains("--force"));
        assertTrue(refusal.getOutput().contains("Robusta failed with operational exit code 2."));
        assertTrue(refusal.getOutput().contains("Reusing configuration cache"));
        assertTrue(refusal.getOutput().contains("Configuration cache entry reused"));
        assertFalse(refusal.getOutput().contains(":robustaConfigInit UP-TO-DATE"));
        assertEquals(rootBytes, Files.readString(rootConfig, StandardCharsets.UTF_8));
        assertFalse(Files.exists(appConfig));
        assertFalse(Files.exists(plainConfig));

        BuildResult app = realRunner(
            project,
            ":app:robustaConfigInit"
        ).build();
        assertNull(app.task(":robustaConfigInit"));
        assertEquals(TaskOutcome.SUCCESS, app.task(":app:robustaConfigInit").getOutcome());
        assertEquals(rootBytes, Files.readString(rootConfig, StandardCharsets.UTF_8));
        assertTrue(Files.isRegularFile(appConfig));
        assertFalse(Files.exists(plainConfig));
    }

    @Test
    void realEngineUnqualifiedInitCreatesOneConfigurationInTheOutermostProject() throws Exception {
        Path project = createMultiProject("fan-out-init");

        BuildResult result = realRunner(
            project,
            "robustaConfigInit"
        ).build();

        assertEquals(TaskOutcome.SUCCESS, result.task(":robustaConfigInit").getOutcome());
        assertEquals(TaskOutcome.SKIPPED, result.task(":app:robustaConfigInit").getOutcome());
        assertNull(result.task(":plain:robustaConfigInit"));
        assertEquals(1, occurrences(result.getOutput(), "Created robusta.yaml"));
        assertTrue(Files.isRegularFile(project.resolve("robusta.yaml")));
        assertFalse(Files.exists(project.resolve("app/robusta.yaml")));
        assertFalse(Files.exists(project.resolve("plain/robusta.yaml")));
    }

    private void assertAlwaysRunsWithConfigurationCache(Path project, String taskName) {
        BuildResult first = runner(
            project,
            taskName,
            CONFIGURATION_CACHE
        ).build();
        BuildResult second = runner(
            project,
            taskName,
            CONFIGURATION_CACHE
        ).build();

        assertEquals(TaskOutcome.SUCCESS, first.task(":" + taskName).getOutcome());
        assertEquals(TaskOutcome.SUCCESS, second.task(":" + taskName).getOutcome());
        assertTrue(first.getOutput().contains("Configuration cache entry stored"));
        assertTrue(second.getOutput().contains("Reusing configuration cache"));
        assertTrue(second.getOutput().contains("Configuration cache entry reused"));
        assertFalse(second.getOutput().contains(":" + taskName + " UP-TO-DATE"));
    }

    private Path createSingleProject(String name) throws IOException {
        return createProject(temporaryDirectory, name, PLUGIN_BUILD);
    }

    private Path createMultiProject(String name) throws IOException {
        Path project = createProject(temporaryDirectory, name, PLUGIN_BUILD);
        Files.createDirectories(project.resolve("app"));
        Files.createDirectories(project.resolve("plain"));
        Files.writeString(
            project.resolve("settings.gradle.kts"),
            "rootProject.name = \"fixture\"\ninclude(\"app\", \"plain\")\n",
            StandardCharsets.UTF_8
        );
        Files.writeString(
            project.resolve("app/build.gradle.kts"),
            PLUGIN_BUILD,
            StandardCharsets.UTF_8
        );
        Files.writeString(
            project.resolve("plain/build.gradle.kts"),
            "",
            StandardCharsets.UTF_8
        );
        return project;
    }
}
