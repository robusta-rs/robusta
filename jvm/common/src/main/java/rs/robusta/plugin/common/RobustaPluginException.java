package rs.robusta.plugin.common;

/**
 * A failure raised by the shared plugin mechanics.
 *
 * <p>The shared code cannot throw {@code GradleException} or {@code
 * MojoExecutionException} without depending on one framework, so it throws this
 * instead and each adapter translates at its own boundary. Translation is the
 * adapter's job precisely because the two build tools report failure
 * differently; nothing else about detecting a platform, reading a checksum, or
 * tokenizing an argument string differs between them.
 */
public class RobustaPluginException extends Exception {

    private static final long serialVersionUID = 1L;

    public RobustaPluginException(String message) {
        super(message);
    }

    public RobustaPluginException(String message, Throwable cause) {
        super(message, cause);
    }
}
