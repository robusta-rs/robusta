package rs.robusta.maven;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.maven.FunctionalTestSupport.*;
import static rs.robusta.plugin.common.FakeRobusta.assertSerialActivity;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.Stream;

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

    @ParameterizedTest(name = "{0} and {1}")
    @MethodSource("goalPairs")
    void serializesNativeProcessesAcrossParallelModulesAndGoals(String first, String second) throws Exception {
        Path root = reactor(first, second, 0);
        Result result = run(root, "-T", "3", "validate");
        assertEquals(0, result.exitCode(), result.output());
        assertSerialActivity(root.resolve("activity"), 2);
    }

    @Test
    void serializesNativeProcessesAcrossPluginRealms() throws Exception {
        Path root = reactor("format-fix", "config-show", 0);
        Path beta = root.resolve("beta/pom.xml");
        String dependencies = separateRealmDependencies(temporaryDirectory.resolve("local-repo"));
        Files.writeString(beta, Files.readString(beta).replace("<configuration>", dependencies + "<configuration>"));
        Result result = run(root, "-T", "3", "validate");
        assertEquals(0, result.exitCode(), result.output());
        assertSerialActivity(root.resolve("activity"), 2);
    }

    @Test
    void serialControlRemainsSerial() throws Exception {
        Path root = reactor("format-fix", "run", 0);
        Result result = run(root, "validate");
        assertEquals(0, result.exitCode(), result.output());
        assertSerialActivity(root.resolve("activity"), 2);
    }

    @ParameterizedTest
    @ValueSource(ints = {1, 2})
    void nativeFailureReleasesTheSlotForAnotherModule(int exitCode) throws Exception {
        Path root = reactor("format-fix", "run", exitCode);
        Result result = run(root, "-T", "3", "--fail-at-end", "validate");
        assertEquals(1, result.exitCode(), result.output());
        assertTrue(result.output().contains("beta") && result.output().contains("SUCCESS"), result.output());
        assertSerialActivity(root.resolve("activity"), 2);
    }

    @Test
    void unrelatedMavenWorkCanRunWhileTheNativeProcessWaits() throws Exception {
        Path root = reactor("format-fix", "run", 0);
        Path alpha = root.resolve("alpha/pom.xml");
        Files.writeString(alpha, Files.readString(alpha).replace("--exit=0", "--exit=0 --await-release"));
        Path beta = root.resolve("beta/pom.xml");
        String unrelated = """
            <build><plugins><plugin>
              <groupId>org.apache.maven.plugins</groupId><artifactId>maven-antrun-plugin</artifactId><version>3.1.0</version>
              <executions><execution><phase>validate</phase><goals><goal>run</goal></goals><configuration><target>
                <waitfor maxwait="20" maxwaitunit="second" timeoutproperty="native.timeout">
                  <available file="%s/waiting"/>
                </waitfor>
                <fail if="native.timeout" message="Native process did not start"/>
                <loadfile property="native.active" srcFile="%s/active"/>
                <echo file="%s/release" message="${native.active}"/>
              </target></configuration></execution></executions>
            </plugin></plugins></build></project>
            """.formatted(root.resolve("activity"), root.resolve("activity"), root.resolve("activity"));
        Files.writeString(beta, minimalPom("beta").replace("</project>", unrelated));
        Result result = run(root, "-T", "3", "validate");
        assertEquals(0, result.exitCode(), result.output());
        assertEquals("1", Files.readString(root.resolve("activity/release")));
        assertSerialActivity(root.resolve("activity"), 1);
    }

    private Path reactor(String first, String second, int exitCode) throws Exception {
        Path root = createProject(temporaryDirectory, "root", minimalPom("root")
                .replace("<packaging>jar</packaging>", "<packaging>pom</packaging>")
                .replace("</project>", "<modules><module>alpha</module><module>beta</module></modules></project>"));
        Files.createDirectory(root.resolve("activity"));
        for (String name : List.of("alpha", "beta")) {
            String goal = name.equals("alpha") ? first : second;
            String activity = root.resolve("activity").toString().replace('\\', '/');
            String args = (goal.equals("run") ? "lint --fix " : "") + "--activity=\"" + activity + "\""
                    + " --exit=" + (name.equals("alpha") ? exitCode : 0);
            String binding = """
                <build><plugins><plugin>
                  <groupId>rs.robusta</groupId><artifactId>robusta-maven-plugin</artifactId><version>%s</version>
                  <configuration><args>%s</args></configuration>
                  <executions><execution><phase>validate</phase><goals><goal>%s</goal></goals></execution></executions>
                </plugin></plugins></build></project>
                """.formatted(pluginVersion(), args, goal);
            createProject(root, name, minimalPom(name).replace("</project>", binding));
        }
        return root;
    }

    private Result run(Path root, String... arguments) throws Exception {
        return runFakeMaven(root, temporaryDirectory.resolve("local-repo"), arguments);
    }

    private static Stream<Arguments> goalPairs() {
        return Stream.of(
                Arguments.of("format-fix", "format-fix"),
                Arguments.of("format-fix", "run"),
                Arguments.of("config-show", "lint-fix"),
                Arguments.of("config-init", "run"),
                Arguments.of("run", "run"));
    }
}
