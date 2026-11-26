package rs.robusta.gradle;

import java.io.File;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.Callable;

import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.gradle.api.initialization.Settings;
import org.gradle.api.provider.Provider;

import rs.robusta.plugin.common.RobustaFixedCommand;

/** Thin Gradle adapter for the bundled Robusta CLI executable. */
public final class RobustaPlugin implements Plugin<Object> {
    @Override
    public void apply(Object target) {
        if (target instanceof Settings) {
            ((Settings) target).getGradle().beforeProject(
                project -> project.getPluginManager().apply(RobustaPlugin.class)
            );
        } else {
            applyToProject((Project) target);
        }
    }

    private static void applyToProject(Project project) {
        File bundledCacheRoot = new File(
            project.getGradle().getGradleUserHomeDir(),
            "caches/robusta"
        );
        String pluginVersion = pluginVersion();
        Provider<RobustaSourceRuns> sourceRuns = project.getGradle().getSharedServices()
            .registerIfAbsent("robustaSourceRuns", RobustaSourceRuns.class,
                spec -> spec.getMaxParallelUsages().set(1));
        project.getTasks().withType(AbstractRobustaTask.class).configureEach(task -> {
            task.usesService(sourceRuns);
            task.getBundledCacheRoot().fileValue(bundledCacheRoot);
            task.getPluginVersion().set(pluginVersion);
        });

        List<RobustaFixedCommandTask> directoryTasks = new ArrayList<>();
        for (RobustaFixedCommand command : RobustaFixedCommand.values()) {
            project.getTasks()
                .register(
                    command.gradleTaskName(),
                    RobustaFixedCommandTask.class,
                    task -> {
                        task.setGroup("robusta");
                        task.setDescription(command.description());
                        task.getWorkingDirectory()
                            .convention(project.getLayout().getProjectDirectory());
                        if (command.readsSources()) {
                            task.getWorkingDirectory().convention(
                                project.getRootProject().getLayout().getProjectDirectory()
                            );
                            task.getSourceDirectory().set(project.getLayout().getProjectDirectory());
                            task.getSourceRuns().set(sourceRuns);
                            task.mustRunAfter((Callable<List<String>>) () -> enclosingTasks(project, task));
                        } else {
                            directoryTasks.add(task);
                        }
                    }
                );
        }
        // An enclosing project's run serves this one: its configuration file applies here too, and
        // it reports the same CLI version.
        project.getGradle().getTaskGraph().whenReady(graph -> {
            for (RobustaFixedCommandTask task : directoryTasks) {
                if (graph.hasTask(task) && enclosingTasks(project, task).stream().anyMatch(graph::hasTask)) {
                    task.setEnabled(false);
                }
            }
        });
    }

    private static List<String> enclosingTasks(Project project, RobustaFixedCommandTask task) {
        List<String> paths = new ArrayList<>();
        File directory = RobustaSourceRuns.directory(project.getProjectDir());
        for (Project other : project.getRootProject().getAllprojects()) {
            if (other == project || !other.getPlugins().hasPlugin(RobustaPlugin.class)) {
                continue;
            }
            File outer = RobustaSourceRuns.directory(other.getProjectDir());
            if (directory.toPath().startsWith(outer.toPath())
                    && (!directory.equals(outer) || other.getPath().compareTo(project.getPath()) < 0)) {
                paths.add(other.absoluteProjectPath(task.getName()));
            }
        }
        return paths;
    }

    private static String pluginVersion() {
        String version = RobustaPlugin.class.getPackage().getImplementationVersion();
        return version == null || version.trim().isEmpty() ? "development" : version;
    }
}
