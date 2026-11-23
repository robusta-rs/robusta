package rs.robusta.gradle;

import java.util.ArrayList;
import java.util.List;

import org.gradle.api.file.DirectoryProperty;
import org.gradle.api.provider.Property;
import org.gradle.api.tasks.Internal;
import org.gradle.api.tasks.TaskAction;
import org.gradle.work.DisableCachingByDefault;

import rs.robusta.plugin.common.RobustaFixedCommand;

/** Executes one fixed cwd-native Robusta command. */
@DisableCachingByDefault(
    because = "Robusta intentionally owns discovery, configuration, and source mutation."
)
public abstract class RobustaFixedCommandTask extends AbstractRobustaTask {
    /** Creates an untracked task for one fixed Robusta command. */
    public RobustaFixedCommandTask() {
        doNotTrackState(
            "Robusta owns cwd-based discovery, configuration, ignores, and source mutation."
        );
        onlyIf(task -> {
            RobustaFixedCommandTask sourceTask = (RobustaFixedCommandTask) task;
            return !sourceTask.getSourceDirectory().isPresent()
                || !sourceTask.getSourceRuns().get().covers(new RobustaSourceRuns.Run(sourceTask));
        });
    }

    /** @return the source scope, absent for configuration tasks */
    @Internal
    public abstract DirectoryProperty getSourceDirectory();

    /** @return successful source runs in this build */
    @Internal
    public abstract Property<RobustaSourceRuns> getSourceRuns();

    /** Executes this task's fixed command followed by its {@link #getArguments() arguments}. */
    @TaskAction
    public void runCommand() {
        List<String> arguments =
            new ArrayList<>(RobustaFixedCommand.forGradleTaskName(getName()).arguments());
        RobustaSourceRuns.Run sourceRun = getSourceDirectory().isPresent()
            ? new RobustaSourceRuns.Run(this) : null;
        if (sourceRun != null) {
            if (!sourceRun.directory.equals(sourceRun.workingDirectory)) {
                arguments.add(sourceRun.directory.getPath());
            } else if (!sourceRun.arguments.isEmpty()) {
                // Keep extra paths from replacing the project's scope.
                arguments.add(".");
            }
        }
        arguments.addAll(getArguments().get());
        failOnNonZeroExit(executeRobusta(arguments));
        if (sourceRun != null) {
            getSourceRuns().get().record(sourceRun);
        }
    }
}
