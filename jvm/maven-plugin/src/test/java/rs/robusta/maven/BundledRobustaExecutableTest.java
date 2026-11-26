package rs.robusta.maven;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.nio.channels.FileChannel;
import java.nio.channels.FileLock;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.nio.file.attribute.FileTime;
import java.util.concurrent.FutureTask;
import java.util.concurrent.TimeUnit;

import org.apache.maven.plugin.MojoExecutionException;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import rs.robusta.plugin.common.BundledExecutable;
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
    void waitsOutAnExtractionLockThisJvmAlreadyHolds() throws Exception {
        RobustaPlatform platform =
                RobustaPlatform.detect(System.getProperty("os.name"), System.getProperty("os.arch"));
        Path destination = BundledExecutable.destination(cacheRoot, "1.0.0", platform);
        Files.createDirectories(destination.getParent());
        Path lock = destination.resolveSibling(destination.getFileName() + ".lock");
        FutureTask<Path> extraction = new FutureTask<>(() -> BundledRobustaExecutable.obtain(
                cacheRoot, System.getProperty("os.name"), System.getProperty("os.arch"), "1.0.0"));
        try (FileChannel channel = FileChannel.open(lock, StandardOpenOption.CREATE, StandardOpenOption.WRITE);
                FileLock ignored = channel.lock()) {
            new Thread(extraction).start();
            Thread.sleep(200);
            assertFalse(extraction.isDone());
        }
        assertEquals(destination, extraction.get(10, TimeUnit.SECONDS));
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
