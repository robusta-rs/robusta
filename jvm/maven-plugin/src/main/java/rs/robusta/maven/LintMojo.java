package rs.robusta.maven;

import org.apache.maven.plugin.MojoExecutionException;
import org.apache.maven.plugin.MojoFailureException;
import org.apache.maven.plugins.annotations.Mojo;

import rs.robusta.plugin.common.RobustaFixedCommand;

/** Verifies lint findings with Robusta. */
@Mojo(name = "lint", threadSafe = true)
public final class LintMojo extends AbstractRobustaMojo {
    @Override
    public void execute() throws MojoExecutionException, MojoFailureException {
        run(RobustaFixedCommand.LINT);
    }
}
