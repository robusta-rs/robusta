package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.occurrences;
import static rs.robusta.gradle.FunctionalTestSupport.realExecutable;
import static rs.robusta.gradle.FunctionalTestSupport.realRunner;
import static rs.robusta.gradle.FunctionalTestSupport.runner;
import static rs.robusta.plugin.common.FakeRobusta.readInvocations;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Comparator;
import java.util.List;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.TaskOutcome;
import org.junit.jupiter.api.Test;

import rs.robusta.plugin.common.FakeRobusta;
import rs.robusta.plugin.common.FakeRobusta.Invocation;

class RobustaMultiProjectFunctionalTest {
    private static final String CONFIGURATION_CACHE = "--configuration-cache";

    @TestKitTempDir Path temporaryDirectory;

    @Test
    void settingsRegistersEverySourceTaskAndUnqualifiedRequestsRunOnce() throws Exception {
        Path project = project("settings", "");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);
        List<String> names = List.of(
            "robustaCheck", "robustaCheckDiff", "robustaCheckFix",
            "robustaFormat", "robustaFormatDiff", "robustaFormatFix",
            "robustaLint", "robustaLintDiff", "robustaLintFix"
        );

        BuildResult result = runner(project, names.toArray(String[]::new))
            .build();

        assertEquals(9, readInvocations(record).size());
        for (String name : names) {
            assertEquals(TaskOutcome.SUCCESS, result.task(":" + name).getOutcome());
            for (String child : List.of("app", "app:nested", "lib")) {
                assertEquals(TaskOutcome.SKIPPED, result.task(":" + child + ":" + name).getOutcome());
            }
        }
        assertTrue(readInvocations(record).stream().allMatch(run -> run.cwd().equals(real(project))));
    }

    @Test
    void startingInAChildScopesTheRootProcessAndReusesTheConfigurationCache() throws Exception {
        Path project = project("child", "");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);
        String[] arguments = {"-p", "app", "robustaCheck", "--configuration-cache"};

        BuildResult first = runner(project, arguments).build();
        BuildResult second = runner(project, arguments).build();

        assertTrue(first.getOutput().contains("Configuration cache entry stored"));
        assertTrue(second.getOutput().contains("Configuration cache entry reused"));
        assertEquals(TaskOutcome.SKIPPED, second.task(":app:nested:robustaCheck").getOutcome());
        assertEquals(List.of(
            new Invocation(real(project), List.of("check", real(project.resolve("app")).toString())),
            new Invocation(real(project), List.of("check", real(project.resolve("app")).toString()))
        ), readInvocations(record));
    }

    @Test
    void reverseExplicitRequestsRunTheEnclosingTaskFirst() throws Exception {
        Path project = project("reverse", "");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);
        String[] arguments = {
            ":app:nested:robustaLint", ":app:robustaLint", ":robustaLint",
            "--configuration-cache", "--parallel"
        };

        runner(project, arguments).build();
        BuildResult reused = runner(project, arguments).build();

        assertTrue(reused.getOutput().contains("Configuration cache entry reused"));
        assertEquals(TaskOutcome.SKIPPED, reused.task(":app:robustaLint").getOutcome());
        assertEquals(TaskOutcome.SKIPPED, reused.task(":app:nested:robustaLint").getOutcome());
        assertTrue(reused.getOutput().indexOf("> Task :robustaLint")
            < reused.getOutput().indexOf("> Task :app:robustaLint"));
        assertEquals(List.of(
            new Invocation(real(project), List.of("lint")),
            new Invocation(real(project), List.of("lint"))
        ), readInvocations(record));
    }

    @Test
    void siblingRequestsEachKeepTheirOwnScope() throws Exception {
        Path project = project("siblings", "");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        runner(project, ":lib:robustaFormat", ":app:robustaFormat").build();

        assertEquals(List.of(
            new Invocation(real(project), List.of("format", real(project.resolve("lib")).toString())),
            new Invocation(real(project), List.of("format", real(project.resolve("app")).toString()))
        ), readInvocations(record));
    }

    @Test
    void remappedDirectoriesUsePhysicalContainmentInsteadOfProjectPaths() throws Exception {
        Path project = project("remapped", """
            include("external", "logical")
            project(":external").projectDir = file("../outside")
            project(":logical").projectDir = file("app/physical")
            """);
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        BuildResult result = runner(
            project, ":logical:robustaLint", ":robustaLint", ":external:robustaLint"
        ).build();

        assertEquals(TaskOutcome.SKIPPED, result.task(":logical:robustaLint").getOutcome());
        assertEquals(List.of(
            new Invocation(real(project), List.of("lint")),
            new Invocation(real(project), List.of("lint", real(temporaryDirectory.resolve("outside")).toString()))
        ), readInvocations(record));

        Files.delete(record);
        String[] nestedSiblings = {
            ":logical:robustaLint", ":app:robustaLint", "--parallel", "--configuration-cache"
        };
        runner(project, nestedSiblings).build();
        BuildResult reused = runner(project, nestedSiblings).build();

        assertTrue(reused.getOutput().contains("Configuration cache entry reused"));
        assertEquals(TaskOutcome.SKIPPED, reused.task(":logical:robustaLint").getOutcome());
        assertEquals(List.of(
            new Invocation(real(project), List.of("lint", real(project.resolve("app")).toString())),
            new Invocation(real(project), List.of("lint", real(project.resolve("app")).toString()))
        ), readInvocations(record));
    }

    @Test
    void configurationAndVersionTasksRunOnlyInTheOutermostRequestedProject() throws Exception {
        Path project = project("config", "");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        runner(
            project, ":app:robustaConfigShow", ":app:robustaConfigInit", ":app:robustaVersion"
        ).build();

        List<Invocation> inApp = List.of(
            new Invocation(real(project.resolve("app")), List.of("--version")),
            new Invocation(real(project.resolve("app")), List.of("config")),
            new Invocation(real(project.resolve("app")), List.of("init"))
        );
        assertEquals(inApp, invocationsByArguments(record));

        Files.delete(record);
        String[] arguments = {
            ":app:nested:robustaConfigInit", ":app:nested:robustaVersion", "robustaConfigShow",
            "robustaConfigInit", "robustaVersion", CONFIGURATION_CACHE
        };
        List<Invocation> inRoot = List.of(
            new Invocation(real(project), List.of("--version")),
            new Invocation(real(project), List.of("config")),
            new Invocation(real(project), List.of("init"))
        );
        runner(project, arguments).build();
        assertEquals(inRoot, invocationsByArguments(record));

        Files.delete(record);
        BuildResult reused = runner(project, arguments).build();

        assertTrue(reused.getOutput().contains("Configuration cache entry reused"));
        for (String child : List.of("app", "app:nested", "lib")) {
            assertEquals(TaskOutcome.SKIPPED, reused.task(":" + child + ":robustaConfigShow").getOutcome());
            assertEquals(TaskOutcome.SKIPPED, reused.task(":" + child + ":robustaConfigInit").getOutcome());
            assertEquals(TaskOutcome.SKIPPED, reused.task(":" + child + ":robustaVersion").getOutcome());
        }
        assertEquals(inRoot, invocationsByArguments(record));
    }

    @Test
    void disabledOrFailedOuterTasksDoNotCoverInnerTasks() throws Exception {
        Path disabled = project("disabled", "");
        Files.writeString(disabled.resolve("build.gradle.kts"),
            "tasks.named(\"robustaLint\") { onlyIf { false } }\n");
        Path record = disabled.resolve("invocations.txt");
        FakeRobusta.writeBehavior(disabled, record, 0);

        BuildResult result = runner(
            disabled, ":app:robustaLint", ":robustaLint", "--configuration-cache"
        ).build();
        assertEquals(TaskOutcome.SKIPPED, result.task(":robustaLint").getOutcome());
        assertEquals(TaskOutcome.SUCCESS, result.task(":app:robustaLint").getOutcome());
        assertEquals(1, readInvocations(record).size());

        Path failed = project("failed", "");
        Path failures = failed.resolve("invocations.txt");
        FakeRobusta.writeBehavior(failed, failures, 1);
        BuildResult failure = runner(
            failed, ":app:robustaLint", ":robustaLint", "--continue"
        ).buildAndFail();
        assertEquals(TaskOutcome.FAILED, failure.task(":robustaLint").getOutcome());
        assertEquals(TaskOutcome.FAILED, failure.task(":app:robustaLint").getOutcome());
        assertEquals(2, readInvocations(failures).size());
    }

    @Test
    void differingOptionsAreNotCoveredByAnEnclosingRun() throws Exception {
        Path project = project("different-args", "");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        runner(project,
            ":app:robustaLint", "--args=--deny-warnings", ":robustaLint").build();

        assertEquals(2, readInvocations(record).size());
        assertEquals(List.of("lint", real(project.resolve("app")).toString(), "--deny-warnings"),
            readInvocations(record).get(1).arguments());
    }

    @Test
    void identicalExtraArgumentsKeepTheFullScopeAndSkipNestedRuns() throws Exception {
        Path project = project("same-args", "");
        Files.writeString(project.resolve("build.gradle.kts"), "plugins { id(\"rs.robusta\") }\n");
        Files.writeString(project.resolve("app/build.gradle.kts"), "plugins { id(\"rs.robusta\") }\n");
        Files.writeString(project.resolve("extra.java"), "class Extra {}\n");
        Path record = project.resolve("invocations.txt");
        FakeRobusta.writeBehavior(project, record, 0);

        BuildResult result = runner(project,
            "robustaLint", "--args=extra.java --deny-warnings", "--configuration-cache").build();

        assertEquals(TaskOutcome.SKIPPED, result.task(":app:robustaLint").getOutcome());
        assertEquals(TaskOutcome.SKIPPED, result.task(":lib:robustaLint").getOutcome());
        assertEquals(List.of(new Invocation(real(project),
            List.of("lint", ".", "extra.java", "--deny-warnings"))), readInvocations(record));
    }

    @Test
    void realEngineUsesTheRootBaselineAndFormatsOnlyTheSelectedDirectory() throws Exception {
        Path project = project("native", "");
        Files.writeString(project.resolve("robusta.yaml"), "lint:\n  rules:\n    unused-import: deny\n");
        String lintSource = "import java.util.List;\nclass Example {}\n";
        Files.writeString(project.resolve("Example.java"), lintSource);
        Files.writeString(project.resolve("app/Example.java"), lintSource);
        Path executable = realExecutable();
        List<String> command = List.of(executable.toString(), "lint", "app", "--suppress-all");
        Process process = new ProcessBuilder(command).directory(project.toFile()).redirectErrorStream(true).start();
        String output = new String(process.getInputStream().readAllBytes(), java.nio.charset.StandardCharsets.UTF_8);
        assertEquals(0, process.waitFor(), output);
        assertTrue(Files.readString(project.resolve("robusta-suppressions.json")).contains("app/Example.java"));

        realRunner(project, ":app:robustaLint").build();
        realRunner(project, ":robustaLint").buildAndFail();

        String dirty = "class Dirty{}\n";
        for (String file : List.of("Dirty.java", "app/Dirty.java", "app/nested/Dirty.java", "lib/Dirty.java")) {
            Files.writeString(project.resolve(file), dirty);
        }
        BuildResult formatted = realRunner(project, ":app:robustaFormatFix").build();
        assertEquals(TaskOutcome.SUCCESS, formatted.task(":app:robustaFormatFix").getOutcome());
        assertEquals(dirty, Files.readString(project.resolve("Dirty.java")));
        assertEquals(dirty, Files.readString(project.resolve("lib/Dirty.java")));
        assertEquals("class Dirty {}\n", Files.readString(project.resolve("app/Dirty.java")));
        assertEquals("class Dirty {}\n", Files.readString(project.resolve("app/nested/Dirty.java")));
        assertFalse(Files.exists(project.resolve("app/robusta-suppressions.json")));
    }

    @Test
    void realEngineProcessesExplicitProjectInsideAPrunedDirectory() throws Exception {
        Path project = project("pruned", """
            include("generated")
            project(":generated").projectDir = file(".svn/generated")
            """);
        Files.writeString(project.resolve("Example.java"), "class Example {}\n");
        Path generated = Files.createDirectories(project.resolve(".svn/generated"));
        Path source = generated.resolve("Generated.java");
        Files.writeString(source, "class Generated{}\n");

        BuildResult result = realRunner(project,
            ":generated:robustaFormatFix", ":robustaFormatFix", "--configuration-cache").build();

        assertEquals(TaskOutcome.SUCCESS, result.task(":robustaFormatFix").getOutcome());
        assertEquals(TaskOutcome.SUCCESS, result.task(":generated:robustaFormatFix").getOutcome());
        assertEquals("class Generated {}\n", Files.readString(source));
    }

    @Test
    void realEngineProcessesNestedRepositoriesWithTheirOwnIgnoreBoundary() throws Exception {
        Path project = project("nested-repository", "");
        Files.createDirectory(project.resolve(".git"));
        Files.writeString(project.resolve(".gitignore"), "app/\n");
        Files.createDirectory(project.resolve("app/.git"));
        Files.writeString(project.resolve("Example.java"), "class Example {}\n");
        Path source = project.resolve("app/Nested.java");
        Files.writeString(source, "class Nested{}\n");

        BuildResult result = realRunner(project,
            ":app:robustaFormatFix", ":robustaFormatFix", "--configuration-cache").build();

        assertEquals(TaskOutcome.SUCCESS, result.task(":robustaFormatFix").getOutcome());
        assertEquals(TaskOutcome.SUCCESS, result.task(":app:robustaFormatFix").getOutcome());
        assertEquals("class Nested {}\n", Files.readString(source));
    }

    @Test
    void realEnginePreservesQualifiedAndUnqualifiedGradleSelection() throws Exception {
        Path rootSelection = copyFixture("root-selection");
        SourceSnapshot rootSources = snapshotSources(rootSelection);
        BuildResult rootFailure = realRunner(
            rootSelection,
            ":robustaCheck",
            CONFIGURATION_CACHE
        ).buildAndFail();
        assertEquals(TaskOutcome.FAILED, rootFailure.task(":robustaCheck").getOutcome());
        assertNull(rootFailure.task(":alpha:robustaCheck"));
        assertNull(rootFailure.task(":beta:robustaCheck"));
        assertEquals(1, occurrences(rootFailure.getOutput(), "uppercase-long-suffix"));
        assertEquals(1, occurrences(rootFailure.getOutput(), "modifier-order"));
        assertTrue(rootFailure.getOutput().contains("Configuration cache entry stored"));
        rootSources.assertUnchanged();

        BuildResult reusedRootFailure = realRunner(
            rootSelection,
            ":robustaCheck",
            CONFIGURATION_CACHE
        ).buildAndFail();
        assertEquals(TaskOutcome.FAILED, reusedRootFailure.task(":robustaCheck").getOutcome());
        assertTrue(reusedRootFailure.getOutput().contains("Reusing configuration cache"));
        assertTrue(reusedRootFailure.getOutput().contains("Configuration cache entry reused"));
        rootSources.assertUnchanged();

        Path projectSelection = copyFixture("project-selection");
        SourceSnapshot projectSources = snapshotSources(projectSelection);
        BuildResult alphaFailure = realRunner(
            projectSelection,
            ":alpha:robustaCheck"
        ).buildAndFail();
        assertNull(alphaFailure.task(":robustaCheck"));
        assertEquals(TaskOutcome.FAILED, alphaFailure.task(":alpha:robustaCheck").getOutcome());
        assertNull(alphaFailure.task(":beta:robustaCheck"));
        assertEquals(1, occurrences(alphaFailure.getOutput(), "uppercase-long-suffix"));
        assertFalse(alphaFailure.getOutput().contains("modifier-order"));
        assertFalse(alphaFailure.getOutput().contains("Beta.java"));
        projectSources.assertUnchanged();

        Path fanOut = copyFixture("fan-out");
        SourceSnapshot fanOutSources = snapshotSources(fanOut);
        BuildResult fanOutFailure = realRunner(
            fanOut,
            "robustaCheck",
            "--continue"
        ).buildAndFail();
        assertEquals(TaskOutcome.FAILED, fanOutFailure.task(":robustaCheck").getOutcome());
        assertEquals(TaskOutcome.FAILED, fanOutFailure.task(":alpha:robustaCheck").getOutcome());
        assertEquals(TaskOutcome.SUCCESS, fanOutFailure.task(":beta:robustaCheck").getOutcome());
        assertEquals(2, occurrences(fanOutFailure.getOutput(), "uppercase-long-suffix"));
        assertEquals(2, occurrences(fanOutFailure.getOutput(), "modifier-order"));
        fanOutSources.assertUnchanged();
    }

    private Path copyFixture(String destinationName) throws IOException {
        Path project = Files.createDirectories(temporaryDirectory.resolve(destinationName));
        FunctionalTestSupport.copyFixture(
            RobustaMultiProjectFunctionalTest.class,
            "multi-project",
            project
        );
        return project;
    }

    private static SourceSnapshot snapshotSources(Path project) throws IOException {
        Path alpha = project.resolve("alpha/src/main/java/alpha/Alpha.java");
        Path beta = project.resolve("beta/java/main/beta/Beta.java");
        return new SourceSnapshot(
            alpha,
            Files.readString(alpha, StandardCharsets.UTF_8),
            beta,
            Files.readString(beta, StandardCharsets.UTF_8)
        );
    }

    private record SourceSnapshot(
        Path alpha,
        String alphaBytes,
        Path beta,
        String betaBytes
    ) {
        void assertUnchanged() throws IOException {
            assertEquals(alphaBytes, Files.readString(alpha, StandardCharsets.UTF_8));
            assertEquals(betaBytes, Files.readString(beta, StandardCharsets.UTF_8));
        }
    }

    private Path project(String name, String extraSettings) throws Exception {
        Path project = Files.createDirectories(temporaryDirectory.resolve(name));
        for (String directory : List.of("app/nested", "app/physical", "lib")) {
            Files.createDirectories(project.resolve(directory));
        }
        Files.createDirectories(temporaryDirectory.resolve("outside"));
        Files.writeString(project.resolve("settings.gradle.kts"), """
            plugins { id("rs.robusta") }
            rootProject.name = "fixture"
            include("app", "app:nested", "lib")
            """ + extraSettings);
        Files.writeString(project.resolve("build.gradle.kts"), "");
        return project;
    }

    /** Gradle may run independent tasks in either order, so compare their runs sorted by arguments. */
    private static List<Invocation> invocationsByArguments(Path record) throws IOException {
        return readInvocations(record).stream()
            .sorted(Comparator.comparing(run -> String.join(" ", run.arguments())))
            .toList();
    }

    private static Path real(Path path) {
        try {
            return path.toRealPath();
        } catch (IOException failure) {
            throw new UncheckedIOException(failure);
        }
    }
}
