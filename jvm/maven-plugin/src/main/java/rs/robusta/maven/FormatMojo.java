package rs.robusta.maven;

import org.apache.maven.plugin.MojoExecutionException;
import org.apache.maven.plugin.MojoFailureException;
import org.apache.maven.plugins.annotations.Mojo;

import rs.robusta.plugin.common.RobustaFixedCommand;

/** Verifies formatting with Robusta. */
@Mojo(name = "format", threadSafe = true)
public final class FormatMojo extends AbstractRobustaMojo {
    @Override
    public void execute() throws MojoExecutionException, MojoFailureException {
        run(RobustaFixedCommand.FORMAT);
    }
}
