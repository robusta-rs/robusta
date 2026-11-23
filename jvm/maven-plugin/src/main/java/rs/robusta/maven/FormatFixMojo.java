package rs.robusta.maven;

import org.apache.maven.plugin.MojoExecutionException;
import org.apache.maven.plugin.MojoFailureException;
import org.apache.maven.plugins.annotations.Mojo;

import rs.robusta.plugin.common.RobustaFixedCommand;

/** Applies formatting fixes with Robusta. */
@Mojo(name = "format-fix", threadSafe = true)
public final class FormatFixMojo extends AbstractRobustaMojo {
    @Override
    public void execute() throws MojoExecutionException, MojoFailureException {
        run(RobustaFixedCommand.FORMAT_FIX);
    }
}
