package rs.robusta.plugin.common;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.List;
import java.util.zip.ZipEntry;
import java.util.zip.ZipOutputStream;

public final class FakeRobusta {
    /**
     * The file the native fake reads its behavior from, in its working directory or the
     * nearest ancestor that has one.
     */
    public static final String BEHAVIOR_FILE = "robusta-fake.properties";

    private FakeRobusta() {}

    /**
     * Makes every native fake run under {@code directory} exit with {@code exitCode} and, unless
     * {@code record} is null, append its working directory and arguments to it.
     */
    public static void writeBehavior(Path directory, Path record, int exitCode) throws IOException {
        String behavior = "exit=" + exitCode + "\n";
        if (record != null) {
            Files.createDirectories(record.toAbsolutePath().getParent());
            behavior += "record=" + record.toAbsolutePath() + "\n";
        }
        Files.writeString(directory.resolve(BEHAVIOR_FILE), behavior, StandardCharsets.UTF_8);
    }

    /**
     * Asserts that {@code processes} fakes logged their activity to {@code directory} one at a
     * time and that none is still running.
     */
    public static void assertSerialActivity(Path directory, int processes) throws IOException {
        List<String> events = Files.readAllLines(directory.resolve("events"));
        if (events.size() != processes * 2) {
            throw new AssertionError("Unexpected process count: " + events);
        }
        for (int index = 0; index < events.size(); index += 2) {
            String[] enter = events.get(index).split(" ");
            String[] exit = events.get(index + 1).split(" ");
            if (!enter[0].equals("enter") || !enter[2].equals("1")
                    || !List.of(exit).equals(List.of("exit", enter[1], "0"))) {
                throw new AssertionError("Native processes overlapped: " + events);
            }
        }
        if (!Files.readString(directory.resolve("active")).equals("0")) {
            throw new AssertionError("Native process still active: " + events);
        }
    }

    /**
     * Writes {@code jar} as {@code platform}'s binary jar, holding {@code binary} and a
     * {@code .sha256} record of {@code checksum}.
     */
    public static void writeBinaryJar(Path jar, RobustaPlatform platform, byte[] binary, String checksum)
            throws IOException {
        Files.createDirectories(jar.toAbsolutePath().getParent());
        try (ZipOutputStream output = new ZipOutputStream(Files.newOutputStream(jar))) {
            output.putNextEntry(new ZipEntry(platform.binaryName()));
            output.write(binary);
            output.putNextEntry(new ZipEntry(platform.binaryName() + ".sha256"));
            output.write((checksum + "  " + platform.binaryName() + "\n").getBytes(StandardCharsets.US_ASCII));
        }
    }

    /** Returns the lowercase hexadecimal SHA-256 digest of {@code bytes}. */
    public static String sha256(byte[] bytes) {
        try {
            return Hex.encode(MessageDigest.getInstance("SHA-256").digest(bytes));
        } catch (NoSuchAlgorithmException error) {
            throw new IllegalStateException(error);
        }
    }

    public static List<Invocation> readInvocations(Path record) throws IOException {
        List<Invocation> invocations = new ArrayList<>();
        Path cwd = null;
        List<String> arguments = new ArrayList<>();
        for (String line : Files.readAllLines(record, StandardCharsets.UTF_8)) {
            if (line.startsWith("cwd=")) {
                if (cwd != null) {
                    invocations.add(new Invocation(cwd, List.copyOf(arguments)));
                    arguments.clear();
                }
                cwd = Path.of(line.substring("cwd=".length()));
            } else if (line.startsWith("arg[")) {
                arguments.add(line.substring(line.indexOf('=') + 1));
            }
        }
        if (cwd != null) {
            invocations.add(new Invocation(cwd, List.copyOf(arguments)));
        }
        return invocations;
    }

    public record Invocation(Path cwd, List<String> arguments) {}
}
