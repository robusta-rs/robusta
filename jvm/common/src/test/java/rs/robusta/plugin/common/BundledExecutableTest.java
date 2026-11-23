package rs.robusta.plugin.common;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.nio.file.Path;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

class BundledExecutableTest {
    private static final BundledExecutable BUNDLED = new BundledExecutable("Test");

    @TempDir Path cacheRoot;

    @Test
    void reportsAMissingBundledResource() throws RobustaPluginException {
        RobustaPluginException failure = assertThrows(
                RobustaPluginException.class,
                () -> BUNDLED.obtain(
                        cacheRoot, System.getProperty("os.name"), System.getProperty("os.arch"), "1.0.0"));

        RobustaPlatform platform =
                RobustaPlatform.detect(System.getProperty("os.name"), System.getProperty("os.arch"));
        assertEquals(
                "The Robusta Test plugin is missing bundled resource /META-INF/robusta/bin/"
                        + platform.bundledBinaryName()
                        + ".sha256.",
                failure.getMessage());
    }

    @Test
    void rejectsAVersionThatIsNotASafeCachePathSegment() {
        RobustaPluginException failure = assertThrows(
                RobustaPluginException.class,
                () -> BUNDLED.obtain(
                        cacheRoot, System.getProperty("os.name"), System.getProperty("os.arch"), "../escape"));

        assertEquals("Invalid Robusta plugin version for cache path: ../escape", failure.getMessage());
    }
}
