package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.nio.file.Path;

import org.gradle.api.GradleException;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

class BundledRobustaExecutableTest {
    @TempDir Path cacheRoot;

    @Test
    void rejectsAnUnsupportedPlatformWithTheSupportedSetAndNoOverride() {
        GradleException failure = assertThrows(
                GradleException.class,
                () -> BundledRobustaExecutable.obtain(cacheRoot, "Plan 9", "mips64", "test"));

        assertTrue(failure.getMessage().contains("os.name='Plan 9', os.arch='mips64'"));
        assertTrue(failure.getMessage().contains("x86_64-unknown-linux-musl"));
        assertTrue(failure.getMessage().contains("aarch64-apple-darwin"));
        assertFalse(failure.getMessage().contains("executable"), failure.getMessage());
    }
}
