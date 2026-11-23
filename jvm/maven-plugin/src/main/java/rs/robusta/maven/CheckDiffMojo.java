package rs.robusta.maven;

import org.apache.maven.plugin.MojoExecutionException;
import org.apache.maven.plugin.MojoFailureException;
import org.apache.maven.plugins.annotations.Mojo;

import rs.robusta.plugin.common.RobustaFixedCommand;

/** Prints formatting and lint diffs with Robusta. */
@Mojo(name = "check-diff", threadSafe = true)
public final class CheckDiffMojo extends AbstractRobustaMojo {
    @Override
    public void execute() throws MojoExecutionException, MojoFailureException {
        run(RobustaFixedCommand.CHECK_DIFF);
    }
}
