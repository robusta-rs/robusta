package rs.robusta.plugin.common;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class RobustaPlatformTest {
    @Test
    void normalizesSupportedJvmPlatformSpellings() throws RobustaPluginException {
        assertEquals(
                RobustaPlatform.LINUX_X86_64,
                RobustaPlatform.detect("Linux", "amd64"));
        assertEquals(
                RobustaPlatform.LINUX_X86_64,
                RobustaPlatform.detect("linux", "x86_64"));
        assertEquals(
                RobustaPlatform.LINUX_AARCH64,
                RobustaPlatform.detect("Linux", "arm64"));
        assertEquals(
                RobustaPlatform.MACOS_AARCH64,
                RobustaPlatform.detect("Mac OS X", "aarch64"));
        assertEquals(
                RobustaPlatform.MACOS_X86_64,
                RobustaPlatform.detect("Darwin", "x86_64"));
        assertEquals(
                RobustaPlatform.WINDOWS_X86_64,
                RobustaPlatform.detect("Windows 11", "AMD64"));
        assertEquals("robusta.exe", RobustaPlatform.WINDOWS_X86_64.extractedBinaryName());
    }

    @Test
    void unsupportedPlatformNamesDetectionAndEndsAtTheSupportedSet() {
        RobustaPluginException failure = assertThrows(
                RobustaPluginException.class, () -> RobustaPlatform.detect("Plan 9", "mips64"));

        assertTrue(failure.getMessage().contains("os.name='Plan 9', os.arch='mips64'"));
        assertTrue(failure.getMessage().contains("x86_64-unknown-linux-musl"));
        assertTrue(failure.getMessage().contains("aarch64-apple-darwin"));
        assertTrue(failure.getMessage().endsWith("x86_64-pc-windows-msvc."), failure.getMessage());
    }

    @Test
    void rejectsArchitecturesWithoutReleaseArtifacts() {
        assertThrows(
                RobustaPluginException.class,
                () -> RobustaPlatform.detect("Windows 11", "arm64"));
        assertThrows(
                RobustaPluginException.class,
                () -> RobustaPlatform.detect("Linux", "armv7l"));
    }
}
