package rs.robusta.maven;

import java.nio.file.Path;

import org.apache.maven.plugin.MojoExecutionException;

import rs.robusta.plugin.common.BundledExecutable;
import rs.robusta.plugin.common.RobustaPluginException;

/** Extracts and caches the bundled native executable matching the running platform. */
final class BundledRobustaExecutable {
    private static final BundledExecutable BUNDLED = new BundledExecutable("Maven");

    private BundledRobustaExecutable() {}

    /**
     * Extracts the bundled binary matching {@code osName}/{@code osArchitecture} into
     * {@code cacheRoot}, verifying it against its bundled checksum, and returns its path.
     *
     * <p>Reuses a previously extracted, checksum-valid binary in place. The cache root lies under
     * the local Maven repository, and entries are namespaced by plugin version and target triple so
     * concurrent Maven reactors and plugin versions never collide.
     */
    static Path obtain(Path cacheRoot, String osName, String osArchitecture, String version)
            throws MojoExecutionException {
        try {
            return BUNDLED.obtain(cacheRoot, osName, osArchitecture, version);
        } catch (RobustaPluginException failure) {
            throw new MojoExecutionException(failure.getMessage(), failure);
        }
    }
}
