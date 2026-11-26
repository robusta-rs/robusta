package rs.robusta.gradle;

import java.io.File;
import java.util.Collections;
import java.util.List;

import org.gradle.api.DefaultTask;
import org.gradle.api.GradleException;
import org.gradle.api.InvalidUserDataException;
import org.gradle.api.file.ConfigurableFileCollection;
import org.gradle.api.file.DirectoryProperty;
import org.gradle.api.logging.LogLevel;
import org.gradle.api.logging.Logger;
import org.gradle.api.provider.ListProperty;
import org.gradle.api.provider.Property;
import org.gradle.api.tasks.Input;
import org.gradle.api.tasks.InputFiles;
import org.gradle.api.tasks.Internal;
import org.gradle.api.tasks.PathSensitive;
import org.gradle.api.tasks.PathSensitivity;
import org.gradle.api.tasks.options.Option;
import org.gradle.process.ExecOperations;
import org.gradle.process.ExecResult;
import org.gradle.work.DisableCachingByDefault;

import rs.robusta.plugin.common.CommandLineArguments;
import rs.robusta.plugin.common.RobustaBinary;
import rs.robusta.plugin.common.RobustaPlatform;
import rs.robusta.plugin.common.RobustaPluginException;

/** Shared external-process boundary for Robusta tasks. */
@DisableCachingByDefault(
        because = "Robusta owns source discovery and mutation outside Gradle's task state model.")
public abstract class AbstractRobustaTask extends DefaultTask {
    /** Creates a task with an empty argument list. */
    protected AbstractRobustaTask() {
        getArguments().convention(Collections.<String>emptyList());
    }

    /**
     * Returns the directory in which Robusta executes.
     *
     * @return the working directory
     */
    @Internal
    public abstract DirectoryProperty getWorkingDirectory();

    /**
     * Returns the running platform's binary jar.
     *
     * @return the resolved {@code rs.robusta:robusta-binary} jar
     */
    @InputFiles
    @PathSensitive(PathSensitivity.NONE)
    public abstract ConfigurableFileCollection getBinaryJar();

    /**
     * Returns the cache root the binary is extracted under.
     *
     * @return the binary cache root
     */
    @Internal
    public abstract DirectoryProperty getCacheRoot();

    /**
     * Returns the plugin version that namespaces cache entries.
     *
     * @return the plugin version
     */
    @Internal
    public abstract Property<String> getPluginVersion();

    /**
     * Returns arguments passed without Robusta-specific interpretation, after any the task
     * supplies itself.
     *
     * @return native CLI arguments
     */
    @Input
    public abstract ListProperty<String> getArguments();

    /**
     * Sets the arguments from one command line while respecting ordinary single and double quotes.
     *
     * @param arguments command line supplied through Gradle's {@code --args} option
     */
    @Option(option = "args", description = "Arguments to pass to the Robusta CLI.")
    public void setCommandLineArguments(String arguments) {
        try {
            getArguments().set(CommandLineArguments.parse(arguments, "--args"));
        } catch (RobustaPluginException failure) {
            throw new InvalidUserDataException(failure.getMessage(), failure);
        }
    }

    /**
     * Returns Gradle's configuration-cache-compatible process execution service.
     *
     * @return the process execution service
     */
    @javax.inject.Inject
    protected abstract ExecOperations getExecOperations();

    /**
     * Executes the Robusta binary with the supplied arguments.
     *
     * @param arguments native CLI arguments
     * @return the native process exit code
     */
    protected final int executeRobusta(List<String> arguments) {
        File executable;
        try {
            RobustaPlatform platform =
                    RobustaPlatform.detect(System.getProperty("os.name"), System.getProperty("os.arch"));
            executable = RobustaBinary.obtain(
                    getBinaryJar().getSingleFile().toPath(),
                    platform,
                    getCacheRoot().get().getAsFile().toPath(),
                    getPluginVersion().get())
                .toFile();
        } catch (RobustaPluginException failure) {
            throw new GradleException(failure.getMessage(), failure);
        }
        Logger logger = getLogger();
        // Gradle logs captured standard output at QUIET and standard error at ERROR.
        try (Utf8LineOutputStream output =
                        new Utf8LineOutputStream(line -> logger.log(LogLevel.QUIET, line));
                Utf8LineOutputStream error =
                        new Utf8LineOutputStream(line -> logger.log(LogLevel.ERROR, line))) {
            ExecResult result = getExecOperations().exec(
                spec -> {
                    spec.setExecutable(executable.getAbsolutePath());
                    spec.args(arguments);
                    spec.setWorkingDir(getWorkingDirectory().get().getAsFile());
                    spec.setIgnoreExitValue(true);
                    spec.setStandardOutput(output);
                    spec.setErrorOutput(error);
                }
            );
            return result.getExitValue();
        }
    }

    /**
     * Converts a nonzero Robusta exit code into the corresponding Gradle failure.
     *
     * @param exitCode native process exit code
     */
    protected static void failOnNonZeroExit(int exitCode) {
        if (exitCode == 0) {
            return;
        }
        if (exitCode == 1) {
            throw new GradleException(
                "Robusta reported problems in the analyzed source (exit code 1)."
            );
        }
        throw new GradleException("Robusta failed with operational exit code " + exitCode + ".");
    }
}
