package rs.robusta.plugin.common;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.nio.channels.FileChannel;
import java.nio.channels.FileLock;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.nio.file.attribute.FileTime;
import java.util.concurrent.FutureTask;
import java.util.concurrent.TimeUnit;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

class RobustaBinaryTest {
    private static final RobustaPlatform PLATFORM = RobustaPlatform.LINUX_X86_64;
    private static final byte[] BINARY = "#!/bin/sh\necho robusta\n".getBytes(StandardCharsets.US_ASCII);

    @TempDir Path directory;

    @Test
    void extractsAndReusesTheVerifiedBinary() throws Exception {
        Path jar = writeJar(FakeRobusta.sha256(BINARY));
        Path cacheRoot = directory.resolve("cache");

        Path first = RobustaBinary.obtain(jar, PLATFORM, cacheRoot, "1.2.3");
        assertEquals(cacheRoot.resolve("1.2.3/x86_64-unknown-linux-musl/robusta"), first);
        assertArrayEquals(BINARY, Files.readAllBytes(first));
        assertTrue(Files.isExecutable(first));

        FileTime sentinel = FileTime.fromMillis(1_700_000_000_000L);
        Files.setLastModifiedTime(first, sentinel);
        Path second = RobustaBinary.obtain(jar, PLATFORM, cacheRoot, "1.2.3");
        assertEquals(first, second);
        assertEquals(sentinel, Files.getLastModifiedTime(second), "a verified cache entry is not re-extracted");
    }

    @Test
    void replacesACacheEntryThatDoesNotMatchItsChecksum() throws Exception {
        Path jar = writeJar(FakeRobusta.sha256(BINARY));
        Path extracted = RobustaBinary.obtain(jar, PLATFORM, directory.resolve("cache"), "1.0.0");
        Files.writeString(extracted, "corrupted", StandardCharsets.UTF_8);

        Path replaced = RobustaBinary.obtain(jar, PLATFORM, directory.resolve("cache"), "1.0.0");
        assertEquals(extracted, replaced);
        assertArrayEquals(BINARY, Files.readAllBytes(replaced));
    }

    @Test
    void refusesABinaryThatDoesNotMatchTheJarsChecksum() throws Exception {
        Path jar = writeJar("0".repeat(64));
        Path cacheRoot = directory.resolve("cache");

        RobustaPluginException failure = assertThrows(
                RobustaPluginException.class, () -> RobustaBinary.obtain(jar, PLATFORM, cacheRoot, "1.0.0"));

        assertTrue(failure.getMessage().startsWith("Robusta checksum mismatch for " + jar), failure.getMessage());
        assertTrue(failure.getMessage().contains("expected " + "0".repeat(64)), failure.getMessage());
        assertFalse(Files.exists(RobustaBinary.destination(cacheRoot, "1.0.0", PLATFORM)));
    }

    @Test
    void reportsAJarWithoutThePlatformsBinary() throws Exception {
        Path jar = writeJar(FakeRobusta.sha256(BINARY));

        RobustaPluginException failure = assertThrows(
                RobustaPluginException.class,
                () -> RobustaBinary.obtain(jar, RobustaPlatform.MACOS_AARCH64, directory.resolve("cache"), "1.0.0"));

        assertEquals(
                "The Robusta binary jar " + jar + " is missing robusta-aarch64-apple-darwin.sha256.",
                failure.getMessage());
    }

    @Test
    void waitsOutAnExtractionLockThisJvmAlreadyHolds() throws Exception {
        Path jar = writeJar(FakeRobusta.sha256(BINARY));
        Path cacheRoot = directory.resolve("cache");
        Path destination = RobustaBinary.destination(cacheRoot, "1.0.0", PLATFORM);
        Files.createDirectories(destination.getParent());
        Path lock = destination.resolveSibling(destination.getFileName() + ".lock");
        FutureTask<Path> extraction =
                new FutureTask<>(() -> RobustaBinary.obtain(jar, PLATFORM, cacheRoot, "1.0.0"));
        try (FileChannel channel = FileChannel.open(lock, StandardOpenOption.CREATE, StandardOpenOption.WRITE);
                FileLock ignored = channel.lock()) {
            new Thread(extraction).start();
            Thread.sleep(200);
            assertFalse(extraction.isDone());
        }
        assertEquals(destination, extraction.get(10, TimeUnit.SECONDS));
    }

    @Test
    void rejectsAVersionThatIsNotASafeCachePathSegment() throws Exception {
        Path jar = writeJar(FakeRobusta.sha256(BINARY));

        RobustaPluginException failure = assertThrows(
                RobustaPluginException.class,
                () -> RobustaBinary.obtain(jar, PLATFORM, directory.resolve("cache"), "../escape"));

        assertEquals("Invalid Robusta plugin version for cache path: ../escape", failure.getMessage());
    }

    @Test
    void namesThePlatformsJarByItsRustTarget() {
        assertEquals(
                "rs.robusta:robusta-binary:1.2.3:x86_64-pc-windows-msvc",
                RobustaBinary.coordinates("1.2.3", RobustaPlatform.WINDOWS_X86_64));
    }

    private Path writeJar(String checksum) throws IOException {
        Path jar = directory.resolve("robusta-binary.jar");
        FakeRobusta.writeBinaryJar(jar, PLATFORM, BINARY, checksum);
        return jar;
    }
}
