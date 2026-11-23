package rs.robusta.gradle;

import org.gradle.api.tasks.TaskAction;
import org.gradle.work.DisableCachingByDefault;

/** Executes the Robusta CLI without interpreting its command or configuration. */
@DisableCachingByDefault(because = "This escape hatch has intentionally opaque inputs and outputs.")
public abstract class RobustaExecTask extends AbstractRobustaTask {
    /** Creates an untracked pass-through task. */
    public RobustaExecTask() {
        doNotTrackState(
            "Robusta owns configuration discovery and source selection for this escape-hatch task."
        );
    }

    /** Executes the configured Robusta command line. */
    @TaskAction
    public void runRobusta() {
        failOnNonZeroExit(executeRobusta(getArguments().get()));
    }
}
