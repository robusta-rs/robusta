package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.gradle.FunctionalTestSupport.runner;
import static rs.robusta.plugin.common.FakeRobusta.assertSerialActivity;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.Stream;

import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.TaskOutcome;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Timeout;
import org.junit.jupiter.api.io.TempDir;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;
import org.junit.jupiter.params.provider.ValueSource;

@Timeout(120)
class RobustaProcessServiceFunctionalTest {
    @TempDir Path temporaryDirectory;

    @ParameterizedTest(name = "{0} and {1}")
    @MethodSource("taskPairs")
    void serializesNativeProcessesAndReusesTheConfigurationCache(String first, String second)
            throws Exception {
        Path project = project();
        String[] arguments = {
            ":alpha:" + first, ":beta:" + second, "--parallel", "--max-workers=3",
            "--configuration-cache"
        };

        BuildResult stored = runner(project, arguments).build();
        assertTrue(stored.getOutput().contains("Configuration cache entry stored"));
        assertSerialActivity(project.resolve("activity"), 2);
        Files.delete(project.resolve("activity/events"));

        BuildResult reused = runner(project, arguments).build();
        assertTrue(reused.getOutput().contains("Configuration cache entry reused"));
        assertEquals(TaskOutcome.SUCCESS, reused.task(":alpha:" + first).getOutcome());
        assertEquals(TaskOutcome.SUCCESS, reused.task(":beta:" + second).getOutcome());
        assertSerialActivity(project.resolve("activity"), 2);
    }

    @ParameterizedTest
    @ValueSource(ints = {1, 2})
    void releasesCapacityAfterNativeFailure(int exitCode) throws Exception {
        Path project = project();
        Files.writeString(project.resolve("alpha/build.gradle"), """
            tasks.named("robustaFormatFix") { arguments.add("--exit=%d") }
            """.formatted(exitCode), java.nio.file.StandardOpenOption.APPEND);
        Files.writeString(project.resolve("beta/build.gradle"), """
            tasks.named("robustaRun") { mustRunAfter(":alpha:robustaFormatFix") }
            """, java.nio.file.StandardOpenOption.APPEND);
        String[] arguments = {
            ":alpha:robustaFormatFix", ":beta:robustaRun", "--parallel", "--max-workers=3",
            "--continue", "--configuration-cache"
        };

        BuildResult failed = runner(project, arguments).buildAndFail();
        assertEquals(TaskOutcome.FAILED, failed.task(":alpha:robustaFormatFix").getOutcome());
        assertEquals(TaskOutcome.SUCCESS, failed.task(":beta:robustaRun").getOutcome());
        assertSerialActivity(project.resolve("activity"), 2);
        Files.delete(project.resolve("activity/events"));

        BuildResult reused = runner(project, arguments).buildAndFail();
        assertTrue(reused.getOutput().contains("Configuration cache entry reused"));
        assertEquals(TaskOutcome.SUCCESS, reused.task(":beta:robustaRun").getOutcome());
        assertSerialActivity(project.resolve("activity"), 2);
    }

    @Test
    void unrelatedGradleWorkCanRunWhileANativeProcessWaits() throws Exception {
        Path project = project();
        Files.writeString(project.resolve("alpha/build.gradle"), """
            tasks.named("robustaFormatFix") { arguments.add("--await-release") }
            """, java.nio.file.StandardOpenOption.APPEND);
        Files.writeString(project.resolve("other/build.gradle"), """
            abstract class ReleaseNative extends DefaultTask {
                @Internal abstract DirectoryProperty getActivityDirectory()

                @TaskAction void release() {
                    def directory = activityDirectory.get().asFile.toPath()
                    def deadline = System.nanoTime() + java.util.concurrent.TimeUnit.SECONDS.toNanos(20)
                    while (!java.nio.file.Files.exists(directory.resolve("waiting"))) {
                        if (System.nanoTime() > deadline) {
                            throw new GradleException("Native process did not start")
                        }
                        Thread.sleep(10)
                    }
                    def active = java.nio.file.Files.readString(directory.resolve("active"))
                    java.nio.file.Files.writeString(directory.resolve("release"), active)
                }
            }
            tasks.register("releaseNative", ReleaseNative) {
                activityDirectory.set(rootProject.layout.projectDirectory.dir("activity"))
            }
            """);
        String[] arguments = {
            ":alpha:robustaFormatFix", ":beta:robustaConfigShow", ":other:releaseNative",
            "--parallel", "--max-workers=3", "--configuration-cache"
        };

        runner(project, arguments).build();
        assertEquals("1", Files.readString(project.resolve("activity/release")));
        assertSerialActivity(project.resolve("activity"), 2);
        for (String name : List.of("events", "waiting", "release")) {
            Files.delete(project.resolve("activity/" + name));
        }

        BuildResult reused = runner(project, arguments).build();
        assertTrue(reused.getOutput().contains("Configuration cache entry reused"));
        assertEquals("1", Files.readString(project.resolve("activity/release")));
        assertSerialActivity(project.resolve("activity"), 2);
    }

    private Path project() throws Exception {
        Path project = Files.createDirectories(temporaryDirectory.resolve("project"));
        Files.writeString(project.resolve("settings.gradle.kts"), """
            plugins { id("rs.robusta") }
            rootProject.name = "process-service"
            include("alpha", "beta", "other")
            """);
        Files.createDirectory(project.resolve("activity"));
        for (String name : List.of("alpha", "beta", "other")) {
            Path child = Files.createDirectory(project.resolve(name));
            Files.writeString(child.resolve("build.gradle"), """
                tasks.withType(rs.robusta.gradle.AbstractRobustaTask).configureEach {
                    arguments.add("--activity=" + rootProject.file("activity").absolutePath)
                }
                tasks.named("robustaRun") { arguments.addAll("lint", "--fix") }
                """);
        }
        return project;
    }

    private static Stream<Arguments> taskPairs() {
        return Stream.of(
            Arguments.of("robustaFormatFix", "robustaFormatFix"),
            Arguments.of("robustaFormatFix", "robustaRun"),
            Arguments.of("robustaConfigShow", "robustaLintFix"),
            Arguments.of("robustaConfigInit", "robustaRun"),
            Arguments.of("robustaRun", "robustaRun")
        );
    }
}
