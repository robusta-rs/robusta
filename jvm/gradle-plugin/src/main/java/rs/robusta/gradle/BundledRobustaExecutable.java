package rs.robusta.gradle;

import java.io.File;
import java.nio.file.Path;

import org.gradle.api.GradleException;

import rs.robusta.plugin.common.BundledExecutable;
import rs.robusta.plugin.common.RobustaPluginException;

/** Extracts the bundled native executable for the running platform into the Gradle user cache. */
final class BundledRobustaExecutable {
    private static final BundledExecutable BUNDLED = new BundledExecutable("Gradle");

    private BundledRobustaExecutable() {}

    /**
     * Returns the binary matching {@code osName}/{@code osArchitecture} under {@code cacheRoot},
     * extracting it when absent and verifying it against its bundled checksum on every call.
     */
    static File obtain(Path cacheRoot, String osName, String osArchitecture, String version) {
        try {
            return BUNDLED.obtain(cacheRoot, osName, osArchitecture, version).toFile();
        } catch (RobustaPluginException failure) {
            throw new GradleException(failure.getMessage(), failure);
        }
    }
}
