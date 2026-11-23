package rs.robusta.gradle;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;
import java.util.Properties;
import java.util.regex.Pattern;

import rs.robusta.plugin.common.RobustaPlatform;
import rs.robusta.plugin.common.RobustaPluginException;

final class BundledPluginClasspath {
    private BundledPluginClasspath() {}

    static Path create(Path directory, boolean validChecksum) throws IOException {
        RobustaPlatform platform = hostPlatform();
        Path resources = Files.createDirectories(directory.resolve("resources"));
        Path binaryDirectory = Files.createDirectories(
                resources.resolve("META-INF/robusta/bin"));
        Path binary = binaryDirectory.resolve(platform.bundledBinaryName());
        if (platform.isWindows()) {
            Files.copy(windowsShell(), binary, StandardCopyOption.REPLACE_EXISTING);
        } else {
            Files.writeString(
                    binary,
                    "#!/bin/sh\necho bundled robusta\n",
                    StandardCharsets.UTF_8);
        }
        String checksum = validChecksum ? sha256(binary) : "0".repeat(64);
        Files.writeString(
                binary.resolveSibling(binary.getFileName() + ".sha256"),
                checksum + "  " + platform.bundledBinaryName() + "\n",
                StandardCharsets.US_ASCII);
        return resources;
    }

    static List<File> pluginClasspath(Path bundledResources) throws IOException {
        Properties metadata = new Properties();
        try (InputStream input = BundledPluginClasspath.class
                .getClassLoader()
                .getResourceAsStream("plugin-under-test-metadata.properties")) {
            if (input == null) {
                throw new IOException("plugin-under-test-metadata.properties is missing");
            }
            metadata.load(input);
        }
        String implementationClasspath = metadata.getProperty("implementation-classpath");
        if (implementationClasspath == null) {
            throw new IOException("implementation-classpath is missing from plugin metadata");
        }
        List<File> classpath = new ArrayList<>();
        classpath.add(bundledResources.toFile());
        for (String entry : implementationClasspath.split(Pattern.quote(File.pathSeparator))) {
            classpath.add(new File(entry));
        }
        return List.copyOf(classpath);
    }

    static List<String> echoArguments() {
        if (hostPlatform().isWindows()) {
            return List.of("--args=/c \"echo bundled robusta\"");
        }
        return List.of("--args=-c \"echo bundled robusta\"");
    }

    private static Path windowsShell() {
        return Path.of(System.getenv().getOrDefault(
                "ComSpec", "C:\\Windows\\System32\\cmd.exe"));
    }

    private static String sha256(Path path) throws IOException {
        MessageDigest digest;
        try {
            digest = MessageDigest.getInstance("SHA-256");
        } catch (NoSuchAlgorithmException error) {
            throw new IllegalStateException(error);
        }
        try (InputStream input = Files.newInputStream(path)) {
            byte[] buffer = new byte[16 * 1024];
            int read;
            while ((read = input.read(buffer)) != -1) {
                digest.update(buffer, 0, read);
            }
        }
        return HexFormat.of().formatHex(digest.digest());
    }

    /**
     * The platform these tests run on. Detection only fails on a host Robusta
     * publishes no binary for, which is not a case this suite can run in.
     */
    static RobustaPlatform hostPlatform() {
        try {
            return RobustaPlatform.detect(System.getProperty("os.name"), System.getProperty("os.arch"));
        } catch (RobustaPluginException failure) {
            throw new IllegalStateException("Unsupported test host", failure);
        }
    }
}
