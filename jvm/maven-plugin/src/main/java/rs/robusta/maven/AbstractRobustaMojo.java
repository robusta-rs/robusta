package rs.robusta.maven;

import java.io.File;
import java.io.IOException;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.locks.ReentrantLock;

import org.apache.maven.execution.MavenSession;
import org.apache.maven.plugin.AbstractMojo;
import org.apache.maven.plugin.MojoExecution;
import org.apache.maven.plugin.MojoExecutionException;
import org.apache.maven.plugin.MojoFailureException;
import org.apache.maven.plugin.descriptor.PluginDescriptor;
import org.apache.maven.plugins.annotations.Parameter;
import org.apache.maven.project.MavenProject;

import rs.robusta.plugin.common.CommandLineArguments;
import rs.robusta.plugin.common.RobustaFixedCommand;
import rs.robusta.plugin.common.RobustaPluginException;

/** Shared external-process boundary for Robusta goals. */
abstract class AbstractRobustaMojo extends AbstractMojo {
    @Parameter(defaultValue = "${project}", readonly = true, required = true)
    private MavenProject project;

    @Parameter(defaultValue = "${session}", readonly = true, required = true)
    private MavenSession session;

    @Parameter(defaultValue = "${plugin}", readonly = true, required = true)
    private PluginDescriptor pluginDescriptor;

    @Parameter(defaultValue = "${mojoExecution}", readonly = true, required = true)
    private MojoExecution mojoExecution;

    /** Arguments appended to the goal's own. */
    @Parameter(property = "robusta.args", defaultValue = "")
    private String args;

    /**
     * Runs one fixed Robusta command in its source or configuration scope.
     *
     * <p>The command line comes from the shared table, so a goal does not restate it.
     */
    protected final void run(RobustaFixedCommand command)
            throws MojoExecutionException, MojoFailureException {
        boolean source = command.readsSources();
        boolean commandLine = mojoExecution.getSource() == MojoExecution.Source.CLI;
        if (!RobustaReactor.get(session).skips(project, commandLine, source)) {
            run(command.arguments(), source);
        }
    }

    private void run(List<String> arguments, boolean source) throws MojoExecutionException, MojoFailureException {
        List<String> extraArguments;
        try {
            extraArguments = CommandLineArguments.parse(args == null ? "" : args, "robusta.args");
        } catch (RobustaPluginException failure) {
            throw new MojoExecutionException(failure.getMessage(), failure);
        }
        List<String> scopedArguments = new ArrayList<>(arguments);
        File workingDirectory = project.getBasedir();
        if (source) {
            workingDirectory = RobustaReactor.get(session).root();
            File directory = RobustaReactor.directory(project.getBasedir());
            if (!directory.equals(workingDirectory)) {
                Path path = directory.toPath();
                Path root = workingDirectory.toPath();
                // Relative scopes share the native cwd's spelling, including Windows short names.
                scopedArguments.add(path.startsWith(root)
                        ? "." + File.separator + root.relativize(path)
                        : path.toString());
            } else if (!extraArguments.isEmpty()) {
                scopedArguments.add(".");
            }
        }
        scopedArguments.addAll(extraArguments);
        execute(scopedArguments, workingDirectory);
    }

    private void execute(List<String> arguments, File workingDirectory)
            throws MojoExecutionException, MojoFailureException {
        Path executable = BundledRobustaExecutable.obtain(
                cacheRoot(),
                System.getProperty("os.name"),
                System.getProperty("os.arch"),
                pluginDescriptor.getVersion());
        List<String> command = new ArrayList<>(arguments.size() + 1);
        command.add(executable.toAbsolutePath().toString());
        command.addAll(arguments);

        int exitCode;
        try {
            exitCode = RobustaProcess.run(new ProcessBuilder(command)
                    .directory(workingDirectory)
                    .inheritIO(), processLock());
        } catch (IOException error) {
            throw new MojoExecutionException("Could not run the Robusta executable: " + executable, error);
        } catch (InterruptedException error) {
            Thread.currentThread().interrupt();
            throw new MojoExecutionException("Interrupted while running the Robusta executable", error);
        }
        failOnNonZeroExit(exitCode);
    }

    private ReentrantLock processLock() {
        MavenProject owner = session.getTopLevelProject();
        synchronized (owner) {
            String key = AbstractRobustaMojo.class.getName() + ".processLock";
            ReentrantLock lock = (ReentrantLock) owner.getContextValue(key);
            if (lock == null) {
                lock = new ReentrantLock();
                owner.setContextValue(key, lock);
            }
            return lock;
        }
    }

    // Extracted binaries live beside the artifacts Maven already caches, under the effective
    // local repository rather than a fixed path in the user's home. A build that redirects its
    // local repository — which CI does routinely to cache it — takes the extracted binary with
    // it, and a per-project target/ directory would instead re-extract once per reactor module.
    // The .cache/<tool> layout keeps non-artifact data out of the group/artifact namespace.
    private Path cacheRoot() {
        return session.getRepositorySession()
                .getLocalRepository()
                .getBasedir()
                .toPath()
                .resolve(".cache")
                .resolve("robusta");
    }

    // Robusta's exit codes distinguish a source-at-fault result (1) from a tool-at-fault one
    // (2 and anything else). Maven draws the same line between its two Mojo exceptions, and
    // reports each against its own documentation, so the pairing carries through to the build
    // log: MojoFailureException is the expected verdict of a working tool, MojoExecutionException
    // means the tool itself could not do its job.
    private static void failOnNonZeroExit(int exitCode) throws MojoExecutionException, MojoFailureException {
        if (exitCode == 0) {
            return;
        }
        if (exitCode == 1) {
            throw new MojoFailureException("Robusta reported problems in the analyzed source (exit code 1).");
        }
        throw new MojoExecutionException("Robusta failed with operational exit code " + exitCode + ".");
    }
}
