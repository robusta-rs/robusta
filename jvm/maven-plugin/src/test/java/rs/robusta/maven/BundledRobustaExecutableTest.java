package rs.robusta.maven;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.FileTime;

import org.apache.maven.plugin.MojoExecutionException;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import rs.robusta.plugin.common.RobustaPlatform;

class BundledRobustaExecutableTest {
    @TempDir Path cacheRoot;

    @Test
    void extractsAndReusesTheVerifiedBinary() throws Exception {
        Path first = BundledRobustaExecutable.obtain(
                cacheRoot, System.getProperty("os.name"), System.getProperty("os.arch"), "1.2.3");
        assertTrue(Files.isRegularFile(first));
        RobustaPlatform platform =
                RobustaPlatform.detect(System.getProperty("os.name"), System.getProperty("os.arch"));
        assertEquals(
                cacheRoot.resolve("1.2.3").resolve(platform.triple()).resolve(platform.extractedBinaryName()),
                first);

        FileTime sentinel = FileTime.fromMillis(1_700_000_000_000L);
        Files.setLastModifiedTime(first, sentinel);

        Path second = BundledRobustaExecutable.obtain(
                cacheRoot, System.getProperty("os.name"), System.getProperty("os.arch"), "1.2.3");
        assertEquals(first, second);
        assertEquals(sentinel, Files.getLastModifiedTime(second), "a verified cache entry is not re-extracted");
    }

    @Test
    void rejectsAndDoesNotSilentlyReplaceACorruptedCacheEntry() throws Exception {
        Path extracted = BundledRobustaExecutable.obtain(
                cacheRoot, System.getProperty("os.name"), System.getProperty("os.arch"), "1.0.0");
        Files.writeString(extracted, "corrupted", StandardCharsets.UTF_8);

        MojoExecutionException failure = assertThrows(
                MojoExecutionException.class,
                () -> BundledRobustaExecutable.obtain(
                        cacheRoot, System.getProperty("os.name"), System.getProperty("os.arch"), "1.0.0"));
        assertTrue(failure.getMessage().contains("checksum mismatch"), failure.getMessage());
        assertEquals("corrupted", Files.readString(extracted, StandardCharsets.UTF_8));
    }

    @Test
    void rejectsAnUnsupportedPlatformWithTheSupportedSetAndNoOverride() {
        MojoExecutionException failure = assertThrows(
                MojoExecutionException.class,
                () -> BundledRobustaExecutable.obtain(cacheRoot, "Plan 9", "mips64", "1.0.0"));

        assertTrue(failure.getMessage().contains("os.name='Plan 9', os.arch='mips64'"));
        assertTrue(failure.getMessage().contains("x86_64-unknown-linux-musl"));
        assertTrue(failure.getMessage().contains("aarch64-apple-darwin"));
        assertFalse(failure.getMessage().contains("executable"), failure.getMessage());
    }

    @Test
    void rejectsAVersionThatIsNotASafeCachePathSegment() {
        assertThrows(
                MojoExecutionException.class,
                () -> BundledRobustaExecutable.obtain(
                        cacheRoot,
                        System.getProperty("os.name"),
                        System.getProperty("os.arch"),
                        "../escape"));
    }
}
