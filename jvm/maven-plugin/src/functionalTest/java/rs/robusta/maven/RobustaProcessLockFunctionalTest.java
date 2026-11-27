package rs.robusta.maven;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.maven.FunctionalTestSupport.*;
import static rs.robusta.plugin.common.FakeRobusta.assertSerialActivity;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.stream.Stream;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Timeout;
import org.junit.jupiter.api.io.TempDir;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;
import org.junit.jupiter.params.provider.ValueSource;

@Timeout(120)
class RobustaProcessLockFunctionalTest {
    @TempDir Path temporaryDirectory;
    private Path root;
    private Path activity;

    @BeforeEach
    void createReactorRoot() throws Exception {
        root = createProject(temporaryDirectory, "root", minimalPom("root")
                .replace("<packaging>jar</packaging>", "<packaging>pom</packaging>")
                .replace("</project>", "<modules><module>alpha</module><module>beta</module></modules></project>"));
        activity = Files.createDirectory(root.resolve("activity"));
    }

    @ParameterizedTest(name = "{0} and {1}")
    @MethodSource("goalPairs")
    void serializesNativeProcessesAcrossParallelModulesAndGoals(String first, String second) throws Exception {
        writeModules(robusta(first, "--exit=0"), robusta(second, "--exit=0"));
        Result result = run("-T", "3", "validate");
        assertEquals(0, result.exitCode(), result.output());
        assertSerialActivity(activity, 2);
    }

    @Test
    void serializesNativeProcessesAcrossPluginRealms() throws Exception {
        String dependencies = separateRealmDependencies(temporaryDirectory.resolve("local-repo"));
        writeModules(robusta("format-fix", "--exit=0"), robusta("config-show", "--exit=0", dependencies));
        Result result = run("-T", "3", "validate");
        assertEquals(0, result.exitCode(), result.output());
        assertSerialActivity(activity, 2);
    }

    @Test
    void serialControlRemainsSerial() throws Exception {
        writeModules(robusta("format-fix", "--exit=0"), robusta("version", "--exit=0"));
        Result result = run("validate");
        assertEquals(0, result.exitCode(), result.output());
        assertSerialActivity(activity, 2);
    }

    @ParameterizedTest
    @ValueSource(ints = {1, 2})
    void nativeFailureReleasesTheSlotForAnotherModule(int exitCode) throws Exception {
        writeModules(robusta("format-fix", "--exit=" + exitCode), robusta("version", "--exit=0"));
        Result result = run("-T", "3", "--fail-at-end", "validate");
        assertEquals(1, result.exitCode(), result.output());
        assertTrue(result.output().contains("beta") && result.output().contains("SUCCESS"), result.output());
        assertSerialActivity(activity, 2);
    }

    @Test
    void unrelatedMavenWorkCanRunWhileTheNativeProcessWaits() throws Exception {
        String unrelated = nativeReleasePlugin(temporaryDirectory.resolve("local-repo"), activity);
        writeModules(robusta("format-fix", "--exit=0 --await-release"), unrelated);
        Result result = run("-T", "3", "validate");
        assertEquals(0, result.exitCode(), result.output());
        assertEquals("1", Files.readString(activity.resolve("release")));
        assertSerialActivity(activity, 1);
    }

    private void writeModules(String alpha, String beta) throws Exception {
        writeModule("alpha", alpha);
        writeModule("beta", beta);
    }

    private void writeModule(String name, String plugin) throws Exception {
        String build = "<build><plugins>" + plugin + "</plugins></build></project>";
        Path module = Files.createDirectory(root.resolve(name));
        Files.writeString(module.resolve("pom.xml"), minimalPom(name).replace("</project>", build));
    }

    private String robusta(String goal, String args) {
        return robusta(goal, args, "");
    }

    private String robusta(String goal, String args, String dependencies) {
        String activityArg = "--activity=\"" + activity.toString().replace('\\', '/') + "\"";
        return """
                <plugin>
                  <groupId>rs.robusta</groupId><artifactId>robusta-maven-plugin</artifactId><version>%s</version>
                  %s<configuration><args>%s %s</args></configuration>
                  <executions><execution><phase>validate</phase><goals><goal>%s</goal></goals></execution></executions>
                </plugin>
                """.formatted(pluginVersion(), dependencies, activityArg, args, goal);
    }

    private Result run(String... arguments) throws Exception {
        return runFakeMaven(root, temporaryDirectory.resolve("local-repo"), arguments);
    }

    private static Stream<Arguments> goalPairs() {
        return Stream.of(
                Arguments.of("format-fix", "format-fix"),
                Arguments.of("format-fix", "version"),
                Arguments.of("config-show", "lint-fix"),
                Arguments.of("config-init", "version"),
                Arguments.of("version", "version"));
    }
}
