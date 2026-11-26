package rs.robusta.gradle;

import java.io.File;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.Callable;

import org.gradle.api.GradleException;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.gradle.api.artifacts.Configuration;
import org.gradle.api.initialization.Settings;
import org.gradle.api.initialization.dsl.ScriptHandler;
import org.gradle.api.plugins.ExtraPropertiesExtension;
import org.gradle.api.provider.Provider;

import rs.robusta.plugin.common.RobustaBinary;
import rs.robusta.plugin.common.RobustaFixedCommand;
import rs.robusta.plugin.common.RobustaPlatform;
import rs.robusta.plugin.common.RobustaPluginException;

/** Thin Gradle adapter for the Robusta CLI executable. */
public final class RobustaPlugin implements Plugin<Object> {
    private static final String SETTINGS_SCRIPT = RobustaPlugin.class.getName() + ".settingsScript";

    @Override
    public void apply(Object target) {
        if (target instanceof Settings) {
            ScriptHandler settingsScript = ((Settings) target).getBuildscript();
            ((Settings) target).getGradle().beforeProject(project -> {
                // A project's build script lists none of the settings script's plugin repositories.
                project.getExtensions().getExtraProperties().set(SETTINGS_SCRIPT, settingsScript);
                project.getPluginManager().apply(RobustaPlugin.class);
            });
        } else {
            applyToProject((Project) target);
        }
    }

    private static void applyToProject(Project project) {
        File cacheRoot = new File(
            project.getGradle().getGradleUserHomeDir(),
            "caches/robusta"
        );
        String pluginVersion = pluginVersion();
        Configuration binaryJar = binaryJar(project, pluginVersion);
        Provider<RobustaSourceRuns> sourceRuns = project.getGradle().getSharedServices()
            .registerIfAbsent("robustaSourceRuns", RobustaSourceRuns.class,
                spec -> spec.getMaxParallelUsages().set(1));
        project.getTasks().withType(AbstractRobustaTask.class).configureEach(task -> {
            task.usesService(sourceRuns);
            task.getBinaryJar().from(binaryJar);
            task.getCacheRoot().fileValue(cacheRoot);
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

    /**
     * The running platform's binary jar, resolved from the plugin repositories, which served the
     * plugin. Nothing is resolved until a task needs the binary, so an unsupported platform fails
     * only the Robusta tasks.
     */
    private static Configuration binaryJar(Project project, String pluginVersion) {
        ScriptHandler script = pluginScript(project);
        Configuration configuration = script.getConfigurations().detachedConfiguration();
        configuration.setDescription("The Robusta binary for the running platform.");
        configuration.setTransitive(false);
        configuration.getDependencies().addLater(project.getProviders().provider(() -> {
            RobustaPlatform platform;
            try {
                platform = RobustaPlatform.detect(System.getProperty("os.name"), System.getProperty("os.arch"));
            } catch (RobustaPluginException failure) {
                throw new GradleException(failure.getMessage(), failure);
            }
            return script.getDependencies().create(RobustaBinary.coordinates(pluginVersion, platform));
        }));
        return configuration;
    }

    /**
     * The script whose plugin repositories served the plugin: the settings script that applied
     * it, or else the nearest build script, from the project's up to the root's, that declares a
     * repository. A project that applies a plugin its parent loaded declares none of its own.
     */
    private static ScriptHandler pluginScript(Project project) {
        ExtraPropertiesExtension extra = project.getExtensions().getExtraProperties();
        if (extra.has(SETTINGS_SCRIPT)) {
            return (ScriptHandler) extra.get(SETTINGS_SCRIPT);
        }
        for (Project scope = project; scope != null; scope = scope.getParent()) {
            if (!scope.getBuildscript().getRepositories().isEmpty()) {
                return scope.getBuildscript();
            }
        }
        return project.getBuildscript();
    }

    private static String pluginVersion() {
        String version = RobustaPlugin.class.getPackage().getImplementationVersion();
        return version == null || version.trim().isEmpty() ? "development" : version;
    }
}
