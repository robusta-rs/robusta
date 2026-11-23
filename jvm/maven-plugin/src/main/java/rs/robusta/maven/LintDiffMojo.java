package rs.robusta.maven;

import org.apache.maven.plugin.MojoExecutionException;
import org.apache.maven.plugin.MojoFailureException;
import org.apache.maven.plugins.annotations.Mojo;

import rs.robusta.plugin.common.RobustaFixedCommand;

/** Prints lint-fix diffs with Robusta. */
@Mojo(name = "lint-diff", threadSafe = true)
public final class LintDiffMojo extends AbstractRobustaMojo {
    @Override
    public void execute() throws MojoExecutionException, MojoFailureException {
        run(RobustaFixedCommand.LINT_DIFF);
    }
}
