package rs.robusta.maven;

import org.apache.maven.plugin.MojoExecutionException;
import org.apache.maven.plugin.MojoFailureException;
import org.apache.maven.plugins.annotations.Mojo;

/** Runs the Robusta CLI with the supplied arguments, without interpreting them. */
@Mojo(name = "run", threadSafe = true)
public final class RunMojo extends AbstractRobustaMojo {
    @Override
    public void execute() throws MojoExecutionException, MojoFailureException {
        run();
    }
}
