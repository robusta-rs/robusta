package rs.robusta.maven;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.concurrent.TimeUnit;

import org.apache.maven.plugin.AbstractMojo;
import org.apache.maven.plugin.MojoExecutionException;

/**
 * Maven work unrelated to Robusta, run by {@link FunctionalTestSupport#nativeReleasePlugin}. It
 * waits for a native fake that awaits release, then releases it, recording the active count.
 */
public class NativeReleaseMojo extends AbstractMojo {
    private File activity;

    @Override
    public void execute() throws MojoExecutionException {
        Path directory = activity.toPath();
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(20);
        try {
            while (!Files.exists(directory.resolve("waiting"))) {
                if (System.nanoTime() > deadline) {
                    throw new MojoExecutionException("Native process did not start");
                }
                Thread.sleep(50);
            }
            Files.writeString(directory.resolve("release"), Files.readString(directory.resolve("active")));
        } catch (IOException error) {
            throw new MojoExecutionException("Cannot release the native process", error);
        } catch (InterruptedException error) {
            Thread.currentThread().interrupt();
            throw new MojoExecutionException("Interrupted while waiting for the native process", error);
        }
    }
}
