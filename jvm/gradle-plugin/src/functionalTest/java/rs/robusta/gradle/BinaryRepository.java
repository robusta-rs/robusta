package rs.robusta.gradle;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

import rs.robusta.plugin.common.FakeRobusta;
import rs.robusta.plugin.common.RobustaBinary;
import rs.robusta.plugin.common.RobustaPlatform;
import rs.robusta.plugin.common.RobustaPluginException;

/** Local Maven repositories that serve the plugin under test its binary jar. */
final class BinaryRepository {
    private BinaryRepository() {}

    /**
     * Writes a repository whose only binary jar holds the native fake for the running platform,
     * recorded with its checksum or, unless {@code validChecksum}, with a wrong one.
     */
    static Path createFake(Path directory, boolean validChecksum) throws IOException {
        RobustaPlatform platform = hostPlatform();
        String version = FunctionalTestSupport.pluginVersion();
        Path module = Files.createDirectories(
                directory.resolve(RobustaBinary.GROUP.replace('.', '/'))
                        .resolve(RobustaBinary.ARTIFACT)
                        .resolve(version));
        String baseName = RobustaBinary.ARTIFACT + "-" + version;
        Files.writeString(module.resolve(baseName + ".pom"), """
                <project xmlns="http://maven.apache.org/POM/4.0.0">
                  <modelVersion>4.0.0</modelVersion>
                  <groupId>%s</groupId>
                  <artifactId>%s</artifactId>
                  <version>%s</version>
                  <packaging>pom</packaging>
                </project>
                """.formatted(RobustaBinary.GROUP, RobustaBinary.ARTIFACT, version),
                StandardCharsets.UTF_8);
        byte[] binary = Files.readAllBytes(FunctionalTestSupport.fakeExecutable());
        FakeRobusta.writeBinaryJar(
                module.resolve(baseName + "-" + platform.triple() + ".jar"),
                platform,
                binary,
                validChecksum ? FakeRobusta.sha256(binary) : "0".repeat(64));
        return directory;
    }

    /**
     * Writes an init script that adds {@code repository} to the settings script's and every build
     * script's repositories, where the plugin resolves its binary jar from. TestKit injects the
     * plugin under test without resolving it from a repository.
     */
    static Path initScript(Path directory, Path repository) throws IOException {
        Files.createDirectories(directory);
        String url = repository.toUri().toString();
        return Files.writeString(directory.resolve("robusta-binary-repository.gradle"), """
                beforeSettings { settings ->
                    settings.buildscript.repositories.maven { url = '%1$s' }
                }
                allprojects {
                    buildscript.repositories.maven { url = '%1$s' }
                }
                """.formatted(url), StandardCharsets.UTF_8);
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
