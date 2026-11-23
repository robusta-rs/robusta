package rs.robusta.gradle;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

import org.gradle.api.GradleException;
import org.gradle.api.services.BuildService;
import org.gradle.api.services.BuildServiceParameters;

import rs.robusta.plugin.common.SourceDirectories;

/** Serializes Robusta tasks and tracks successfully processed source directories. */
public abstract class RobustaSourceRuns implements BuildService<BuildServiceParameters.None> {
    private final List<Run> completed = new ArrayList<>();

    synchronized boolean covers(Run run) {
        for (Run outer : completed) {
            if (run.command.equals(outer.command)
                    && run.arguments.equals(outer.arguments)
                    && run.workingDirectory.equals(outer.workingDirectory)
                    && SourceDirectories.covers(outer.directory, run.directory)) {
                return true;
            }
        }
        return false;
    }

    synchronized void record(Run run) {
        completed.add(run);
    }

    static File directory(File directory) {
        try {
            return directory.getCanonicalFile();
        } catch (IOException failure) {
            throw new GradleException("Cannot resolve Robusta directory: " + directory, failure);
        }
    }

    static final class Run {
        final File directory;
        final String command;
        final List<String> arguments;
        final File workingDirectory;

        Run(RobustaFixedCommandTask task) {
            directory = directory(task.getSourceDirectory().get().getAsFile());
            command = task.getName();
            arguments = task.getArguments().get();
            workingDirectory = directory(task.getWorkingDirectory().get().getAsFile());
        }
    }
}
