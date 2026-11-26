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

/** Extracts a bundled native executable into a cache and verifies it against its checksum. */
public final class BundledExecutable {
    private static final String RESOURCE_ROOT = "/META-INF/robusta/bin/";
    private static final int MAXIMUM_CHECKSUM_BYTES = 512;
    private static final Pattern CHECKSUM = Pattern.compile("^([0-9a-fA-F]{64})(?:\\s+\\*?.+)?$");
    private static final Pattern SAFE_VERSION = Pattern.compile("[0-9A-Za-z][0-9A-Za-z._+-]*");
    private static final long LOCK_RETRY_MILLIS = 10;

    private final String buildTool;

    /**
     * Creates an extractor whose failures name the build tool.
     *
     * @param buildTool the build tool name used in failure messages, such as {@code Gradle}
     */
    public BundledExecutable(String buildTool) {
        this.buildTool = buildTool;
    }

    /**
     * Extracts the binary for the running platform under
     * {@code cacheRoot/version/triple}, reusing a checksum-valid binary already there.
     *
     * @param cacheRoot the build tool's Robusta cache directory
     * @param osName the {@code os.name} system property
     * @param osArchitecture the {@code os.arch} system property
     * @param version the plugin version, which must be a safe path segment
     * @return the extracted executable
     * @throws RobustaPluginException when the platform is unsupported or the binary cannot be
     *     extracted and verified
     */
    public Path obtain(Path cacheRoot, String osName, String osArchitecture, String version)
            throws RobustaPluginException {
        RobustaPlatform platform = RobustaPlatform.detect(osName, osArchitecture);
        if (!SAFE_VERSION.matcher(version).matches()) {
            throw new RobustaPluginException("Invalid Robusta plugin version for cache path: " + version);
        }
        return extract(platform, destination(cacheRoot, version, platform));
    }

    /** Returns where {@link #obtain} places the binary for {@code platform}. */
    public static Path destination(Path cacheRoot, String version, RobustaPlatform platform) {
        return cacheRoot.resolve(version).resolve(platform.triple()).resolve(platform.extractedBinaryName());
    }

    /**
     * Verifies the binary at {@code destination}, extracting it first when it is absent.
     *
     * @throws RobustaPluginException when the binary cannot be extracted or does not match its
     *     bundled checksum
     */
    public Path extract(RobustaPlatform platform, Path destination) throws RobustaPluginException {
        String binaryResource = RESOURCE_ROOT + platform.bundledBinaryName();
        String checksumResource = binaryResource + ".sha256";
        String expected = readChecksum(checksumResource);

        try {
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
                    String actual = sha256(destination);
                    if (!actual.equals(expected)) {
                        throw checksumFailure(destination.toString(), expected, actual);
                    }
                    makeExecutable(destination, platform);
                    return destination;
                }
                return extractLocked(binaryResource, destination, platform, expected);
            }
        } catch (IOException error) {
            throw new RobustaPluginException(
                    "Could not extract the bundled Robusta binary to " + destination, error);
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

    private Path extractLocked(
            String binaryResource, Path destination, RobustaPlatform platform, String expected)
            throws IOException, RobustaPluginException {
        Path temporary = Files.createTempFile(destination.getParent(), ".robusta-", ".tmp");
        try {
            String actual;
            try (InputStream input = openResource(binaryResource);
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
                throw checksumFailure(binaryResource, expected, actual);
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
                        "The Robusta cache filesystem cannot atomically install the bundled binary at "
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

    private String readChecksum(String resource) throws RobustaPluginException {
        String contents;
        try (InputStream input = openResource(resource)) {
            contents = new String(readBounded(input), StandardCharsets.US_ASCII).trim();
        } catch (IOException error) {
            throw new RobustaPluginException("Could not read bundled Robusta checksum " + resource, error);
        }
        Matcher matcher = CHECKSUM.matcher(contents);
        if (!matcher.matches()) {
            throw new RobustaPluginException("Invalid bundled Robusta checksum record " + resource);
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

    private InputStream openResource(String resource) throws RobustaPluginException {
        InputStream input = BundledExecutable.class.getResourceAsStream(resource);
        if (input == null) {
            throw new RobustaPluginException(
                    "The Robusta " + buildTool + " plugin is missing bundled resource " + resource + ".");
        }
        return input;
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
                "Bundled Robusta checksum mismatch for "
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
            throw new RobustaPluginException("Could not mark the bundled Robusta binary executable: " + path);
        }
    }
}
