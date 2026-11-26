package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.assertTaskHelp;
import static rs.robusta.gradle.FunctionalTestSupport.copyFixture;
import static rs.robusta.gradle.FunctionalTestSupport.occurrences;
import static rs.robusta.gradle.FunctionalTestSupport.realExecutable;
import static rs.robusta.gradle.FunctionalTestSupport.realRunner;
import static rs.robusta.gradle.FunctionalTestSupport.runner;
import static rs.robusta.plugin.common.FakeRobusta.readInvocations;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.TaskOutcome;
import org.junit.jupiter.api.Test;

import rs.robusta.plugin.common.FakeRobusta;
import rs.robusta.plugin.common.FakeRobusta.Invocation;

class RobustaWorkflowFunctionalTest {
    private static final String PLUGIN_BUILD = "plugins { id(\"rs.robusta\") }\n";
    private static final String JAVA_PLUGIN_BUILD = """
        plugins {
            java
            id("rs.robusta")
        }
        """;
    // No repository is declared, so resolving the compile classpath would fail the build.
    private static final String JAVA_PLUGIN_WITH_UNRESOLVABLE_DEPENDENCY_BUILD = """
        plugins {
            java
            id("rs.robusta")
        }
        dependencies {
            implementation("org.example:absent:1.0")
        }
        """;
    private static final List<WorkflowCase> WORKFLOWS = List.of(
        new WorkflowCase(
            "robustaCheck",
            "Verifies formatting and lint findings with Robusta.",
            List.of("check")
        ),
        new WorkflowCase(
            "robustaCheckDiff",
            "Prints formatting and lint diffs with Robusta.",
            List.of("check", "--diff")
        ),
        new WorkflowCase(
            "robustaCheckFix",
            "Applies formatting and lint fixes with Robusta.",
            List.of("check", "--fix")
        ),
        new WorkflowCase(
            "robustaFormat",
            "Verifies formatting with Robusta.",
            List.of("format")
        ),
        new WorkflowCase(
            "robustaFormatDiff",
            "Prints formatting diffs with Robusta.",
            List.of("format", "--diff")
        ),
        new WorkflowCase(
            "robustaFormatFix",
            "Applies formatting fixes with Robusta.",
            List.of("format", "--fix")
        ),
        new WorkflowCase(
            "robustaLint",
            "Verifies lint findings with Robusta.",
            List.of("lint")
        ),
        new WorkflowCase(
            "robustaLintDiff",
            "Prints lint-fix diffs with Robusta.",
            List.of("lint", "--diff")
        ),
        new WorkflowCase(
            "robustaLintFix",
            "Applies lint fixes with Robusta.",
            List.of("lint", "--fix")
        )
    );
    private static final List<WorkflowScope> SCOPES = List.of(
        new WorkflowScope("Check", "check", "fixable"),
        new WorkflowScope("Format", "format", "dirty"),
        new WorkflowScope("Lint", "lint", "fixable")
    );

    @TestKitTempDir Path temporaryDirectory;

    @Test
    void fixedTasksPassExactArgumentsFromTheOwningProjectAndForwardOutput() throws Exception {
        Path project = createProject("fixed-matrix", null, JAVA_PLUGIN_BUILD);
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);
        String[] taskNames = WORKFLOWS.stream().map(WorkflowCase::taskName).toArray(String[]::new);

        BuildResult result = runner(project, taskNames).build();

        for (WorkflowCase workflow : WORKFLOWS) {
            assertEquals(TaskOutcome.SUCCESS, result.task(":" + workflow.taskName()).getOutcome());
        }
        assertEquals(WORKFLOWS.size(), occurrences(result.getOutput(), "fake robusta stdout"));
        assertEquals(WORKFLOWS.size(), occurrences(result.getOutput(), "fake robusta stderr"));

        List<Invocation> invocations = readInvocations(record);
        assertEquals(WORKFLOWS.size(), invocations.size());
        Path projectCwd = project.toRealPath();
        assertTrue(
            invocations.stream().allMatch(invocation -> invocation.cwd().equals(projectCwd))
        );
        assertEquals(
            argumentCounts(WORKFLOWS.stream().map(WorkflowCase::arguments).toList()),
            argumentCounts(invocations.stream().map(Invocation::arguments).toList())
        );
    }

    @Test
    void fixedTasksAppendTheirArgsAfterTheFixedArguments() throws Exception {
        Path project = createProject("fixed-args", null, PLUGIN_BUILD);
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);
        List<String> gradleArguments = new ArrayList<>();
        for (WorkflowCase workflow : WORKFLOWS) {
            gradleArguments.add(workflow.taskName());
            gradleArguments.add(
                "--args=" + workflow.taskName()
                    + " \"two words\" 'single quoted' --option=\"value with spaces\""
            );
        }

        BuildResult result = runner(
            project,
            gradleArguments.toArray(String[]::new)
        ).build();

        for (WorkflowCase workflow : WORKFLOWS) {
            assertEquals(TaskOutcome.SUCCESS, result.task(":" + workflow.taskName()).getOutcome());
        }
        List<List<String>> expected = WORKFLOWS.stream()
            .map(
                workflow -> Stream.concat(
                    workflow.arguments().stream(),
                    Stream.of(
                        ".",
                        workflow.taskName(),
                        "two words",
                        "single quoted",
                        "--option=value with spaces"
                    )
                ).toList()
            )
            .toList();
        assertEquals(
            argumentCounts(expected),
            argumentCounts(readInvocations(record).stream().map(Invocation::arguments).toList())
        );
    }

    @Test
    void unmatchedQuoteInArgsFailsBeforeLaunchingRobusta() throws Exception {
        Path project = createProject("invalid-args", null, PLUGIN_BUILD);
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        BuildResult result = runner(
            project,
            "robustaLintFix",
            "--args=--unsafe-fixes 'unterminated"
        ).buildAndFail();

        assertTrue(
            result.getOutput().contains("Cannot parse --args: unmatched ' quote"),
            result.getOutput()
        );
        assertFalse(Files.exists(record));
    }

    @Test
    void checkAndLintTasksResolveNoDependencies() throws Exception {
        Path project = createProject(
            "unresolvable-dependency",
            null,
            JAVA_PLUGIN_WITH_UNRESOLVABLE_DEPENDENCY_BUILD
        );
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        BuildResult result = runner(
            project,
            "robustaCheck",
            "robustaLint"
        ).build();

        assertEquals(TaskOutcome.SUCCESS, result.task(":robustaCheck").getOutcome());
        assertEquals(TaskOutcome.SUCCESS, result.task(":robustaLint").getOutcome());
        assertEquals(
            argumentCounts(List.of(List.of("check"), List.of("lint"))),
            argumentCounts(readInvocations(record).stream().map(Invocation::arguments).toList())
        );
    }

    @Test
    void fixedTasksMapVerificationAndOperationalExitCodes() throws Exception {
        Path verificationProject = createProject("verification-exit", null, PLUGIN_BUILD);
        FakeRobusta.writeBehavior(verificationProject, null, 1);
        BuildResult verificationFailure = runner(
            verificationProject,
            "robustaCheck"
        ).buildAndFail();
        assertTrue(
            verificationFailure.getOutput()
                .contains("Robusta reported problems in the analyzed source (exit code 1).")
        );

        Path operationalProject = createProject("operational-exit", null, PLUGIN_BUILD);
        FakeRobusta.writeBehavior(operationalProject, null, 2);
        BuildResult operationalFailure = runner(
            operationalProject,
            "robustaLintFix"
        ).buildAndFail();
        assertTrue(
            operationalFailure.getOutput()
                .contains("Robusta failed with operational exit code 2.")
        );
    }

    @Test
    void fixedTasksExecuteEveryTimeAndReuseTheConfigurationCache() throws Exception {
        Path project = createProject("untracked", null, PLUGIN_BUILD);
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        BuildResult first = runner(
            project,
            "robustaCheck",
            "--configuration-cache"
        ).build();
        BuildResult second = runner(
            project,
            "robustaCheck",
            "--configuration-cache"
        ).build();

        assertEquals(TaskOutcome.SUCCESS, first.task(":robustaCheck").getOutcome());
        assertEquals(TaskOutcome.SUCCESS, second.task(":robustaCheck").getOutcome());
        assertTrue(first.getOutput().contains("Configuration cache entry stored"));
        assertTrue(second.getOutput().contains("Reusing configuration cache"));
        assertTrue(second.getOutput().contains("Configuration cache entry reused"));
        assertFalse(second.getOutput().contains(":robustaCheck UP-TO-DATE"));
        assertEquals(2, readInvocations(record).size());
    }

    @Test
    void fixedTaskArgsReuseTheConfigurationCacheOnlyWhenUnchanged() throws Exception {
        Path project = createProject("args-configuration-cache", null, PLUGIN_BUILD);
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        BuildResult first = runner(
            project,
            "robustaLint",
            "--args=--deny-warnings",
            "--configuration-cache"
        ).build();
        BuildResult second = runner(
            project,
            "robustaLint",
            "--args=--deny-warnings",
            "--configuration-cache"
        ).build();
        BuildResult changed = runner(
            project,
            "robustaLint",
            "--args=--max-warnings=3",
            "--configuration-cache"
        ).build();

        assertTrue(first.getOutput().contains("Configuration cache entry stored"));
        assertTrue(second.getOutput().contains("Configuration cache entry reused"));
        assertTrue(changed.getOutput().contains("Configuration cache entry stored"));
        assertEquals(
            List.of(
                List.of("lint", ".", "--deny-warnings"),
                List.of("lint", ".", "--deny-warnings"),
                List.of("lint", ".", "--max-warnings=3")
            ),
            readInvocations(record).stream().map(Invocation::arguments).toList()
        );
    }

    @Test
    void taskListingAndHelpExposeTheWorkflowMatrixAndVersionTask() throws Exception {
        Path project = createProject("task-ux", null, PLUGIN_BUILD);

        BuildResult tasks = runner(project, "tasks", "--group", "robusta").build();
        for (WorkflowCase workflow : WORKFLOWS) {
            assertTrue(
                tasks.getOutput().contains(workflow.taskName() + " - " + workflow.description())
            );
            BuildResult help = assertTaskHelp(
                project,
                workflow.taskName(),
                workflow.description()
            );
            assertTrue(help.getOutput().contains("--args"));
            assertFalse(help.getOutput().contains("sourceRoots"));
            assertFalse(help.getOutput().contains("reportFile"));
            assertFalse(help.getOutput().contains("reporter"));
        }
        BuildResult versionHelp = assertTaskHelp(
            project,
            "robustaVersion",
            "Prints the Robusta CLI version."
        );
        assertTrue(versionHelp.getOutput().contains("--args"));

        BuildResult allTasks = runner(project, "tasks", "--all").build();
        for (WorkflowCase workflow : WORKFLOWS) {
            assertEquals(1, occurrences(allTasks.getOutput(), workflow.taskName() + " -"));
        }
        assertEquals(1, occurrences(allTasks.getOutput(), "robustaVersion -"));
        assertFalse(allTasks.getOutput().contains("robustaRun"));
        assertFalse(allTasks.getOutput().contains("robustaApply"));
        assertFalse(allTasks.getOutput().contains("robustaRoot"));
        assertFalse(allTasks.getOutput().contains("NO-SOURCE"));
    }

    @Test
    void applyingBesideJavaAndBaseDoesNotWireTheirLifecycles() throws Exception {
        Path project = createProject("lifecycle-isolation", null, JAVA_PLUGIN_BUILD);

        BuildResult check = runner(project, "check", "--dry-run").build();
        BuildResult build = runner(project, "build", "--dry-run").build();

        assertFalse(check.getOutput().contains("robusta"));
        assertFalse(build.getOutput().contains("robusta"));
    }

    @Test
    void realEngineRendersSourceControlsVisibly() throws Exception {
        Path project = createProject("visible-controls", null, PLUGIN_BUILD);
        Files.writeString(
            project.resolve("robusta.yaml"),
            "lint:\n  rules:\n    unused-import: deny\n",
            StandardCharsets.UTF_8
        );
        // A legacy daemon charset, as on Windows before Java 18, must not garble
        // the engine's UTF-8 output.
        Files.writeString(
            project.resolve("gradle.properties"),
            "org.gradle.jvmargs=-Dfile.encoding=windows-1252\n",
            StandardCharsets.UTF_8
        );
        String source = "// \033[2J\033]0;title\007\b\000\177\u0085\n"
            + "import java.util.Li\033cst;\nclass Example {}\n";
        Path sourcePath = project.resolve("Example.java");
        Files.writeString(sourcePath, source, StandardCharsets.UTF_8);

        BuildResult result = realRunner(
            project,
            "robustaLint"
        ).buildAndFail();
        String output = result.getOutput();
        assertEquals(TaskOutcome.FAILED, result.task(":robustaLint").getOutcome());
        assertTrue(output.contains("␛[2J␛]0;title␇␈␀␡<U+0085>"), output);
        assertTrue(output.contains("imported name `Licst`"), output);
        assertTrue(output.contains("import java.util.Li␛cst;"), output);
        assertTrue(output.contains("^^^^^^"), output);
        assertTrue(output.contains(" 3 | class Example {}"), output);
        assertFalse(output.contains("\033[2J"));
        assertFalse(output.contains("\033]"));
        assertEquals(source, Files.readString(sourcePath, StandardCharsets.UTF_8));
    }

    @Test
    void realEngineCoversCleanDirtyDiffAndFixForEveryScope() throws Exception {
        Path executable = realExecutable();

        for (WorkflowScope scope : SCOPES) {
            Path clean = createProject(scope.command() + "-clean", "clean", PLUGIN_BUILD);
            Path cleanSource = onlyJavaSource(clean);
            String cleanBytes = Files.readString(cleanSource, StandardCharsets.UTF_8);
            BuildResult cleanResult = realRunner(
                clean,
                scope.taskName()
            ).build();
            assertEquals(TaskOutcome.SUCCESS, cleanResult.task(":" + scope.taskName()).getOutcome());
            assertEquals(cleanBytes, Files.readString(cleanSource, StandardCharsets.UTF_8));

            Path readOnly = createProject(
                scope.command() + "-read-only",
                scope.dirtyFixture(),
                PLUGIN_BUILD
            );
            Path readOnlySource = onlyJavaSource(readOnly);
            String original = Files.readString(readOnlySource, StandardCharsets.UTF_8);
            BuildResult verification = realRunner(
                readOnly,
                scope.taskName()
            ).buildAndFail();
            assertEquals(
                TaskOutcome.FAILED,
                verification.task(":" + scope.taskName()).getOutcome()
            );
            assertEquals(original, Files.readString(readOnlySource, StandardCharsets.UTF_8));

            BuildResult diff = realRunner(
                readOnly,
                scope.diffTaskName()
            ).buildAndFail();
            assertEquals(TaskOutcome.FAILED, diff.task(":" + scope.diffTaskName()).getOutcome());
            assertTrue(diff.getOutput().contains("@@"), diff.getOutput());
            assertEquals(original, Files.readString(readOnlySource, StandardCharsets.UTF_8));

            Path pluginFix = createProject(
                scope.command() + "-plugin-fix",
                scope.dirtyFixture(),
                PLUGIN_BUILD
            );
            Path directFix = createProject(
                scope.command() + "-direct-fix",
                scope.dirtyFixture(),
                ""
            );
            Path pluginSource = onlyJavaSource(pluginFix);
            Path directSource = onlyJavaSource(directFix);
            String beforeFix = Files.readString(pluginSource, StandardCharsets.UTF_8);

            BuildResult fixed = realRunner(
                pluginFix,
                scope.fixTaskName()
            ).build();
            assertEquals(TaskOutcome.SUCCESS, fixed.task(":" + scope.fixTaskName()).getOutcome());
            runDirect(executable, directFix, scope.command(), "--fix");

            String pluginCandidate = Files.readString(pluginSource, StandardCharsets.UTF_8);
            assertNotEquals(beforeFix, pluginCandidate);
            assertEquals(
                Files.readString(directSource, StandardCharsets.UTF_8),
                pluginCandidate
            );
            BuildResult verified = realRunner(
                pluginFix,
                scope.taskName()
            ).build();
            assertEquals(TaskOutcome.SUCCESS, verified.task(":" + scope.taskName()).getOutcome());
        }
    }

    @Test
    void realEngineLintFixAppliesUnsafeFixesAndPrunesTheBaselineThroughArgs() throws Exception {
        Path project = createProject("unsafe-fixes-and-pruning", null, PLUGIN_BUILD);
        Files.writeString(
            project.resolve("robusta.yaml"),
            "lint:\n  rules:\n    unused-import: deny\n    unused-local-variable: deny\n",
            StandardCharsets.UTF_8
        );
        Path source = project.resolve("Example.java");
        Files.writeString(
            source,
            "import java.util.List;\nclass Example {\n    int compute() {\n        return 1;\n"
                + "    }\n\n    void run() {\n        int value = compute();\n"
                + "        int stale = compute() + 1;\n    }\n}\n",
            StandardCharsets.UTF_8
        );
        Path executable = realExecutable();
        runDirect(executable, project, "lint", "--suppress-all");
        Path baseline = project.resolve("robusta-suppressions.json");
        assertTrue(Files.readString(baseline, StandardCharsets.UTF_8).contains("unused-import"));

        BuildResult result = realRunner(
            project,
            "robustaLintFix",
            "--args=--unsafe-fixes --prune-suppressions"
        ).build();

        assertEquals(TaskOutcome.SUCCESS, result.task(":robustaLintFix").getOutcome());
        assertEquals(
            "class Example {\n    int compute() {\n        return 1;\n    }\n\n"
                + "    void run() {\n        compute();\n        \n    }\n}\n",
            Files.readString(source, StandardCharsets.UTF_8)
        );
        assertFalse(Files.readString(baseline, StandardCharsets.UTF_8).contains("unused-"));
    }

    private Path createProject(String name, String fixture, String buildScript) throws IOException {
        Path project = FunctionalTestSupport.createProject(temporaryDirectory, name, buildScript);
        if (fixture != null) {
            copyFixture(RobustaWorkflowFunctionalTest.class, fixture, project);
        }
        return project;
    }

    private static Path onlyJavaSource(Path project) throws IOException {
        try (Stream<Path> paths = Files.walk(project)) {
            return paths
                .filter(path -> path.toString().endsWith(".java"))
                .findFirst()
                .orElseThrow(() -> new IOException("Fixture has no Java source"));
        }
    }

    private static void runDirect(Path executable, Path project, String... arguments)
            throws IOException, InterruptedException {
        List<String> command = new ArrayList<>();
        command.add(executable.toString());
        command.addAll(Arrays.asList(arguments));
        Process process = new ProcessBuilder(command)
            .directory(project.toFile())
            .redirectErrorStream(true)
            .start();
        String output = new String(process.getInputStream().readAllBytes(), StandardCharsets.UTF_8);
        int exitCode = process.waitFor();
        assertEquals(0, exitCode, output);
    }

    private static Map<List<String>, Long> argumentCounts(List<List<String>> arguments) {
        return arguments.stream()
            .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));
    }

    private record WorkflowCase(String taskName, String description, List<String> arguments) {}

    private record WorkflowScope(String suffix, String command, String dirtyFixture) {
        String taskName() {
            return "robusta" + suffix;
        }

        String diffTaskName() {
            return taskName() + "Diff";
        }

        String fixTaskName() {
            return taskName() + "Fix";
        }
    }
}
