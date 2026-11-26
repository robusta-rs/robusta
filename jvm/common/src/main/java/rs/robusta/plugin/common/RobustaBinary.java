package rs.robusta.plugin.common;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.channels.FileChannel;
import java.nio.channels.FileLock;
import java.nio.channels.FileLockInterruptionException;
import java.nio.channels.OverlappingFileLockException;
import java.nio.charset.StandardCharsets;
import java.nio.file.AtomicMoveNotSupportedException;
import java.nio.file.Files;
import java.nio.file.LinkOption;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.nio.file.StandardOpenOption;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Arrays;
import java.util.Locale;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.zip.ZipEntry;
import java.util.zip.ZipFile;

/**
 * The jar published for each platform, holding its binary and {@code .sha256} checksum, and
 * extraction of that binary into a cache.
 */
public final class RobustaBinary {
    /** The group of the published binary jars. */
    public static final String GROUP = "rs.robusta";

    /** The artifact of the published binary jars, whose classifier is the Rust target. */
    public static final String ARTIFACT = "robusta-binary";

    private static final int MAXIMUM_CHECKSUM_BYTES = 512;
    private static final Pattern CHECKSUM = Pattern.compile("^([0-9a-fA-F]{64})(?:\\s+\\*?.+)?$");
    private static final Pattern SAFE_VERSION = Pattern.compile("[0-9A-Za-z][0-9A-Za-z._+-]*");
    private static final long LOCK_RETRY_MILLIS = 10;

    private RobustaBinary() {}

    /**
     * Returns the Gradle coordinates of the jar holding {@code platform}'s binary.
     *
     * @param version the plugin version
     * @param platform the platform whose Rust target is the classifier
     * @return {@code group:artifact:version:classifier}
     */
    public static String coordinates(String version, RobustaPlatform platform) {
        return GROUP + ":" + ARTIFACT + ":" + version + ":" + platform.triple();
    }

    /**
     * Extracts {@code platform}'s binary from {@code jar} under
     * {@code cacheRoot/version/triple}, reusing a checksum-valid binary already there.
     *
     * @param jar the resolved jar holding the binary
     * @param platform the running platform
     * @param cacheRoot the build tool's Robusta cache directory
     * @param version the plugin version, which must be a safe path segment
     * @return the extracted executable
     * @throws RobustaPluginException when the binary cannot be extracted and verified
     */
    public static Path obtain(Path jar, RobustaPlatform platform, Path cacheRoot, String version)
            throws RobustaPluginException {
        if (!SAFE_VERSION.matcher(version).matches()) {
            throw new RobustaPluginException("Invalid Robusta plugin version for cache path: " + version);
        }
        return extract(jar, platform, destination(cacheRoot, version, platform));
    }

    /** Returns where {@link #obtain} places the binary for {@code platform}. */
    public static Path destination(Path cacheRoot, String version, RobustaPlatform platform) {
        return cacheRoot.resolve(version).resolve(platform.triple()).resolve(platform.extractedBinaryName());
    }

    /**
     * Returns the binary at {@code destination}, extracting it from {@code jar} when it is missing
     * or does not match the checksum in the jar.
     *
     * @throws RobustaPluginException when the jar's binary does not match its checksum or
     *     cannot be installed at {@code destination}
     */
    public static Path extract(Path jar, RobustaPlatform platform, Path destination)
            throws RobustaPluginException {
        String binaryEntry = platform.binaryName();
        try (ZipFile archive = new ZipFile(jar.toFile())) {
            String expected = readChecksum(archive, jar, binaryEntry + ".sha256");
            Files.createDirectories(destination.getParent());
            Path lockPath = destination.resolveSibling(destination.getFileName() + ".lock");
            try (FileChannel channel = FileChannel.open(
                            lockPath, StandardOpenOption.CREATE, StandardOpenOption.WRITE);
                    FileLock ignored = lock(channel)) {
                if (Files.exists(destination, LinkOption.NOFOLLOW_LINKS)) {
                    if (!Files.isRegularFile(destination, LinkOption.NOFOLLOW_LINKS)) {
                        throw new RobustaPluginException(
                                "Robusta cache path is not a regular file: " + destination);
                    }
                    if (sha256(destination).equals(expected)) {
                        makeExecutable(destination, platform);
                        return destination;
                    }
                }
                return extractLocked(archive, jar, binaryEntry, destination, platform, expected);
            }
        } catch (IOException error) {
            throw new RobustaPluginException(
                    "Could not extract the Robusta binary from " + jar + " to " + destination, error);
        }
    }

    // A file lock belongs to the whole JVM, while each build-tool class loader has its own copy of
    // this class, so a thread waits out a lock this JVM already holds instead of sharing a monitor.
    private static FileLock lock(FileChannel channel) throws IOException {
        while (true) {
            try {
                return channel.lock();
            } catch (OverlappingFileLockException held) {
                try {
                    Thread.sleep(LOCK_RETRY_MILLIS);
                } catch (InterruptedException interrupted) {
                    Thread.currentThread().interrupt();
                    throw new FileLockInterruptionException();
                }
            }
        }
    }

    private static Path extractLocked(
            ZipFile archive,
            Path jar,
            String binaryEntry,
            Path destination,
            RobustaPlatform platform,
            String expected)
            throws IOException, RobustaPluginException {
        Path temporary = Files.createTempFile(destination.getParent(), ".robusta-", ".tmp");
        try {
            String actual;
            try (InputStream input = openEntry(archive, jar, binaryEntry);
                    OutputStream output =
                            Files.newOutputStream(temporary, StandardOpenOption.TRUNCATE_EXISTING)) {
                MessageDigest digest = sha256();
                byte[] buffer = new byte[16 * 1024];
                int read;
                while ((read = input.read(buffer)) != -1) {
                    output.write(buffer, 0, read);
                    digest.update(buffer, 0, read);
                }
                actual = Hex.encode(digest.digest());
            }

            if (!actual.equals(expected)) {
                throw checksumFailure(jar + "!/" + binaryEntry, expected, actual);
            }
            makeExecutable(temporary, platform);
            try {
                Files.move(
                        temporary,
                        destination,
                        StandardCopyOption.ATOMIC_MOVE,
                        StandardCopyOption.REPLACE_EXISTING);
            } catch (AtomicMoveNotSupportedException error) {
                throw new RobustaPluginException(
                        "The Robusta cache filesystem cannot atomically install the binary at "
                                + destination,
                        error);
            }

            String installed = sha256(destination);
            if (!installed.equals(expected)) {
                throw checksumFailure(destination.toString(), expected, installed);
            }
            return destination;
        } finally {
            Files.deleteIfExists(temporary);
        }
    }

    private static String readChecksum(ZipFile archive, Path jar, String entry)
            throws IOException, RobustaPluginException {
        String contents;
        try (InputStream input = openEntry(archive, jar, entry)) {
            contents = new String(readBounded(input), StandardCharsets.US_ASCII).trim();
        }
        Matcher matcher = CHECKSUM.matcher(contents);
        if (!matcher.matches()) {
            throw new RobustaPluginException("Invalid Robusta checksum record " + jar + "!/" + entry);
        }
        return matcher.group(1).toLowerCase(Locale.ROOT);
    }

    // A checksum record is one short line. Reading one byte past the accepted size keeps a
    // corrupt or oversized resource from being loaded whole; the record pattern rejects it.
    private static byte[] readBounded(InputStream input) throws IOException {
        byte[] buffer = new byte[MAXIMUM_CHECKSUM_BYTES + 1];
        int total = 0;
        while (total < buffer.length) {
            int read = input.read(buffer, total, buffer.length - total);
            if (read == -1) {
                break;
            }
            total += read;
        }
        return Arrays.copyOf(buffer, total);
    }

    private static InputStream openEntry(ZipFile archive, Path jar, String entry)
            throws IOException, RobustaPluginException {
        ZipEntry found = archive.getEntry(entry);
        if (found == null) {
            throw new RobustaPluginException("The Robusta binary jar " + jar + " is missing " + entry + ".");
        }
        return archive.getInputStream(found);
    }

    private static String sha256(Path path) throws IOException {
        MessageDigest digest = sha256();
        try (InputStream input = Files.newInputStream(path)) {
            byte[] buffer = new byte[16 * 1024];
            int read;
            while ((read = input.read(buffer)) != -1) {
                digest.update(buffer, 0, read);
            }
        }
        return Hex.encode(digest.digest());
    }

    private static MessageDigest sha256() {
        try {
            return MessageDigest.getInstance("SHA-256");
        } catch (NoSuchAlgorithmException error) {
            throw new IllegalStateException("Every Java runtime must provide SHA-256", error);
        }
    }

    private static RobustaPluginException checksumFailure(String binary, String expected, String actual) {
        return new RobustaPluginException(
                "Robusta checksum mismatch for "
                        + binary
                        + ": expected "
                        + expected
                        + ", actual "
                        + actual
                        + ".");
    }

    private static void makeExecutable(Path path, RobustaPlatform platform) throws RobustaPluginException {
        if (platform.isWindows()) {
            return;
        }
        if (!path.toFile().setExecutable(true, true) || !Files.isExecutable(path)) {
            throw new RobustaPluginException("Could not mark the Robusta binary executable: " + path);
        }
    }
}
