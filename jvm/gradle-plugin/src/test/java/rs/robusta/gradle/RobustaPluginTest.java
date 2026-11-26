package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.File;
import java.lang.reflect.Method;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;

import org.gradle.api.InvalidUserDataException;
import org.gradle.api.Project;
import org.gradle.api.Task;
import org.gradle.api.tasks.TaskProvider;
import org.gradle.testfixtures.ProjectBuilder;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import rs.robusta.plugin.common.RobustaFixedCommand;

class RobustaPluginTest {
    private static final Map<String, List<String>> EXPECTED_FIXED_COMMANDS = Map.ofEntries(
        Map.entry("robustaCheck", List.of("check")),
        Map.entry("robustaCheckDiff", List.of("check", "--diff")),
        Map.entry("robustaCheckFix", List.of("check", "--fix")),
        Map.entry("robustaFormat", List.of("format")),
        Map.entry("robustaFormatDiff", List.of("format", "--diff")),
        Map.entry("robustaFormatFix", List.of("format", "--fix")),
        Map.entry("robustaLint", List.of("lint")),
        Map.entry("robustaLintDiff", List.of("lint", "--diff")),
        Map.entry("robustaLintFix", List.of("lint", "--fix")),
        Map.entry("robustaConfigShow", List.of("config")),
        Map.entry("robustaConfigInit", List.of("init")),
        Map.entry("robustaVersion", List.of("--version"))
    );

    @TempDir File projectDirectory;

    @Test
    void registersNoExtensionAndOnlyCwdNativeTasks() throws Exception {
        Project project = ProjectBuilder.builder().withProjectDir(projectDirectory).build();

        project.getPluginManager().apply(RobustaPlugin.class);

        assertNull(project.getExtensions().findByName("robusta"));
        assertFalse(project.getPluginManager().hasPlugin("java"));
        assertFalse(project.getPluginManager().hasPlugin("base"));
        assertFalse(Arrays.stream(AbstractRobustaTask.class.getMethods())
            .anyMatch(method -> method.getName().equals("getExecutable")));

        Set<String> robustaTasks = project.getTasks()
            .getNames()
            .stream()
            .filter(name -> name.startsWith("robusta"))
            .collect(Collectors.toSet());
        assertEquals(EXPECTED_FIXED_COMMANDS.keySet(), robustaTasks);
        assertFalse(robustaTasks.contains("robustaApply"));
        assertFalse(robustaTasks.stream().anyMatch(name -> name.startsWith("robustaRoot")));

        for (String taskName : EXPECTED_FIXED_COMMANDS.keySet()) {
            RobustaFixedCommandTask task = (RobustaFixedCommandTask) project.getTasks()
                .getByName(taskName);
            assertTrue(task.getArguments().get().isEmpty());
            assertEquals(
                projectDirectory.getCanonicalFile(),
                task.getWorkingDirectory().get().getAsFile().getCanonicalFile()
            );
        }
    }

    @Test
    void everyTaskParsesQuotedArgsAndRejectsAnUnmatchedQuote() {
        Project project = ProjectBuilder.builder().withProjectDir(projectDirectory).build();
        project.getPluginManager().apply(RobustaPlugin.class);
        List<String> parsed = List.of("--option=two words", "single quoted");

        for (String taskName : EXPECTED_FIXED_COMMANDS.keySet()) {
            AbstractRobustaTask task = (AbstractRobustaTask) project.getTasks()
                .getByName(taskName);
            task.setCommandLineArguments("--option=\"two words\" 'single quoted'");
            assertEquals(parsed, task.getArguments().get(), taskName);

            InvalidUserDataException failure = assertThrows(
                InvalidUserDataException.class,
                () -> task.setCommandLineArguments("--unsafe-fixes 'unterminated")
            );
            assertEquals("Cannot parse --args: unmatched ' quote", failure.getMessage());
            assertEquals(parsed, task.getArguments().get(), taskName);
        }
    }

    @Test
    void fixedCommandNamesMapToExactCliArguments() {
        assertEquals(EXPECTED_FIXED_COMMANDS.size(), RobustaFixedCommand.values().length);
        EXPECTED_FIXED_COMMANDS.forEach(
            (taskName, arguments) -> assertEquals(
                arguments,
                RobustaFixedCommand.forGradleTaskName(taskName).arguments()
            )
        );
    }

    @Test
    void fixedCommandsDependOnNoOtherTaskBesideJava() {
        Project project = ProjectBuilder.builder().withProjectDir(projectDirectory).build();
        project.getPluginManager().apply("java");
        // A compile classpath entry that another task builds.
        TaskProvider<Task> library = project.getTasks().register("library");
        project.getDependencies()
            .add("implementation", project.files("library.jar").builtBy(library));

        project.getPluginManager().apply(RobustaPlugin.class);

        for (String taskName : EXPECTED_FIXED_COMMANDS.keySet()) {
            Task task = project.getTasks().getByName(taskName);
            assertEquals(Set.of(), dependencyNames(task), taskName);
        }
    }

    @Test
    void applyingBesideJavaDoesNotWireBuildLifecyclesOrExposeSourceAndReportModels() {
        Project project = ProjectBuilder.builder().withProjectDir(projectDirectory).build();
        project.getPluginManager().apply("java");
        project.getPluginManager().apply(RobustaPlugin.class);

        Set<String> checkDependencies = dependencyNames(project.getTasks().getByName("check"));
        Set<String> buildDependencies = dependencyNames(project.getTasks().getByName("build"));
        assertFalse(checkDependencies.stream().anyMatch(name -> name.startsWith("robusta")));
        assertFalse(buildDependencies.stream().anyMatch(name -> name.startsWith("robusta")));

        Set<String> workflowMethods = Arrays.stream(RobustaFixedCommandTask.class.getMethods())
            .map(Method::getName)
            .collect(Collectors.toSet());
        assertFalse(workflowMethods.contains("getSourceRoots"));
        assertFalse(workflowMethods.contains("getJavaSources"));
        assertFalse(workflowMethods.contains("getReporter"));
        assertFalse(workflowMethods.contains("getReportFile"));
        assertFalse(workflowMethods.contains("getConfig"));
        assertFalse(workflowMethods.contains("getConfigFile"));
        assertFalse(workflowMethods.contains("getForce"));
    }

    private static Set<String> dependencyNames(Task task) {
        return task.getTaskDependencies()
            .getDependencies(task)
            .stream()
            .map(Task::getName)
            .collect(Collectors.toSet());
    }
}
