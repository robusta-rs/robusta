package rs.robusta.plugin.common;

import java.util.Arrays;
import java.util.Locale;
import java.util.stream.Collectors;

/** The five native targets Robusta publishes a binary for, and detection of the running one. */
public enum RobustaPlatform {
    LINUX_X86_64("linux", "x86_64", "x86_64-unknown-linux-musl", false),
    LINUX_AARCH64("linux", "aarch64", "aarch64-unknown-linux-musl", false),
    MACOS_X86_64("macos", "x86_64", "x86_64-apple-darwin", false),
    MACOS_AARCH64("macos", "aarch64", "aarch64-apple-darwin", false),
    WINDOWS_X86_64("windows", "x86_64", "x86_64-pc-windows-msvc", true);

    private static final String SUPPORTED_TRIPLES = Arrays.stream(values())
            .map(RobustaPlatform::triple)
            .collect(Collectors.joining(", "));

    private final String operatingSystem;
    private final String architecture;
    private final String triple;
    private final boolean windows;

    RobustaPlatform(
            String operatingSystem,
            String architecture,
            String triple,
            boolean windows) {
        this.operatingSystem = operatingSystem;
        this.architecture = architecture;
        this.triple = triple;
        this.windows = windows;
    }

    /**
     * Resolves the published platform for a running JVM.
     *
     * @param osName the {@code os.name} system property
     * @param osArchitecture the {@code os.arch} system property
     * @return the matching platform
     * @throws RobustaPluginException when no published binary matches
     */
    public static RobustaPlatform detect(String osName, String osArchitecture)
            throws RobustaPluginException {
        String operatingSystem = normalizeOperatingSystem(osName);
        String architecture = normalizeArchitecture(osArchitecture);
        for (RobustaPlatform platform : values()) {
            if (platform.operatingSystem.equals(operatingSystem)
                    && platform.architecture.equals(architecture)) {
                return platform;
            }
        }
        throw new RobustaPluginException(
                "Robusta publishes no binary for detected platform os.name='"
                        + osName
                        + "', os.arch='"
                        + osArchitecture
                        + "'. Supported platforms: "
                        + SUPPORTED_TRIPLES
                        + ".");
    }

    public String triple() {
        return triple;
    }

    public String binaryName() {
        return "robusta-" + triple + (windows ? ".exe" : "");
    }

    public String extractedBinaryName() {
        return windows ? "robusta.exe" : "robusta";
    }

    public boolean isWindows() {
        return windows;
    }

    private static String normalizeOperatingSystem(String value) {
        String normalized = value.toLowerCase(Locale.ROOT);
        if (normalized.contains("windows")) {
            return "windows";
        }
        if (normalized.contains("mac") || normalized.contains("darwin")) {
            return "macos";
        }
        if (normalized.contains("linux")) {
            return "linux";
        }
        return normalized;
    }

    private static String normalizeArchitecture(String value) {
        String normalized = value.toLowerCase(Locale.ROOT);
        switch (normalized) {
            case "amd64":
            case "x86_64":
                return "x86_64";
            case "aarch64":
            case "arm64":
                return "aarch64";
            default:
                return normalized;
        }
    }
}
