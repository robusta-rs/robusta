package rs.robusta.maven;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static rs.robusta.maven.FunctionalTestSupport.*;
import static rs.robusta.plugin.common.FakeRobusta.readInvocations;
import static rs.robusta.plugin.common.FakeRobusta.writeBehavior;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import rs.robusta.plugin.common.FakeRobusta.Invocation;

class MultiModuleFunctionalTest {
    @TempDir Path temporaryDirectory;

    @Test
    void rootGoalsSkipCoveredModules() throws Exception {
        Path root = reactor();
        Path wrapper = wrapper();
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        for (String goal : List.of("check", "check-diff", "check-fix", "format", "format-diff", "format-fix",
                "lint", "lint-diff", "lint-fix", "config-show", "config-init", "version")) {
            Files.deleteIfExists(record);
            run(root, wrapper, goalCoordinate(goal));
            assertEquals(List.of(root.toRealPath().toString()), workingDirectories(record), goal);
        }
    }

    @Test
    void selectedNestedModulesRunConfigAndVersionGoalsInTheOutermostSelectedModule() throws Exception {
        Path root = reactor();
        Path wrapper = wrapper();
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        run(root, wrapper, "-pl", "app", goalCoordinate("config-show"));
        assertEquals(List.of(new Invocation(root.resolve("app").toRealPath(), List.of("config"))),
                readInvocations(record));
        Files.delete(record);
        run(root, wrapper, "-pl", "app/inner,app", goalCoordinate("config-init"));
        assertEquals(List.of(new Invocation(root.resolve("app").toRealPath(), List.of("init"))),
                readInvocations(record));
        Files.delete(record);
        run(root, wrapper, "-pl", "app/inner,app", goalCoordinate("version"));
        assertEquals(List.of(new Invocation(root.resolve("app").toRealPath(), List.of("--version"))),
                readInvocations(record));
        Files.delete(record);
        run(root, wrapper, "-pl", "app/inner", goalCoordinate("version"));
        assertEquals(List.of(new Invocation(root.resolve("app/inner").toRealPath(), List.of("--version"))),
                readInvocations(record));
    }

    @Test
    void childStartUsesOnlyModulesMavenBuildsFromTheMarkedRoot() throws Exception {
        Path root = reactor();
        project(root.getParent().resolve("outside"));
        pom(root, "app", "sibling", "../outside");
        project(root.resolve("app/inner/leaf"));
        pom(root.resolve("app/inner"), "leaf");
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        run(root.resolve("app/inner"), wrapper(), goalCoordinate("format"));
        assertEquals(List.of(new Invocation(root.toRealPath(),
                List.of("format", Path.of(".").resolve("app/inner").toString()))), readInvocations(record));
    }

    @Test
    void nearestMarkerWinsOverAnOuterAggregator() throws Exception {
        Path root = reactor();
        Files.createDirectories(root.resolve("app/.mvn"));
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        run(root.resolve("app/inner"), wrapper(), goalCoordinate("format"));
        assertEquals(List.of(root.resolve("app").toRealPath().toString()), workingDirectories(record));
    }

    @Test
    void noMarkerKeepsTheStartingModuleDespiteIncludingAncestors() throws Exception {
        Path outer = project(temporaryDirectory.resolve("outer"));
        project(outer.resolve("unrelated"));
        pom(outer, "unrelated");
        Path root = project(outer.resolve("build-root"));
        Path group = project(root.resolve("group"));
        Path leaf = project(group.resolve("leaf"));
        pom(group, "leaf");
        pom(root, "group");
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        run(leaf, wrapper(), goalCoordinate("format"));
        assertEquals(List.of(new Invocation(leaf.toRealPath(), List.of("format"))), readInvocations(record));
    }

    @Test
    void unrelatedAncestorPomDoesNotChangeAStandaloneScope() throws Exception {
        Path outer = project(temporaryDirectory.resolve("unrelated"));
        Path start = project(outer.resolve("standalone"));
        Path record = temporaryDirectory.resolve("calls.txt");
        Path wrapper = wrapper();
        writeBehavior(temporaryDirectory, record, 0);
        run(start, wrapper, goalCoordinate("format"));
        assertEquals(List.of(start.toRealPath().toString()), workingDirectories(record));
        Files.delete(record);
        pom(outer, "missing");
        run(start, wrapper, goalCoordinate("format"));
        assertEquals(List.of(start.toRealPath().toString()), workingDirectories(record));
    }

    @Test
    void anIncludingAncestorWithABrokenModuleDoesNotAffectTheChildStart() throws Exception {
        Path root = project(temporaryDirectory.resolve("root"));
        Path group = project(root.resolve("group"));
        Path leaf = project(group.resolve("leaf"));
        pom(group, "leaf");
        pom(root, "group", "missing");
        Path record = temporaryDirectory.resolve("calls.txt");
        Path wrapper = wrapper();
        writeBehavior(temporaryDirectory, record, 0);
        run(leaf, wrapper, goalCoordinate("format"));
        assertEquals(List.of(new Invocation(leaf.toRealPath(), List.of("format"))), readInvocations(record));
        Files.delete(record);
        run(leaf, wrapper, "-N", goalCoordinate("format"));
        assertEquals(List.of(new Invocation(leaf.toRealPath(), List.of("format"))), readInvocations(record));
    }

    @Test
    void selectedAndNonRecursiveReactorsRetainTheirScopeAndLauncherFileWorks() throws Exception {
        Path root = reactor();
        Path wrapper = wrapper();
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        run(root, wrapper, "-pl", "app", goalCoordinate("format"));
        assertEquals(List.of(new Invocation(root.toRealPath(),
                List.of("format", Path.of(".").resolve("app").toString()))), readInvocations(record));
        Files.delete(record);
        run(root.resolve("app"), wrapper, "-N", goalCoordinate("format"));
        assertEquals(List.of(new Invocation(root.toRealPath(),
                List.of("format", Path.of(".").resolve("app").toString()))), readInvocations(record));
        Files.delete(record);
        run(wrapper, wrapper, "-f", root.resolve("app/pom.xml").toString(), goalCoordinate("format"));
        assertEquals(List.of(root.toRealPath().toString()), workingDirectories(record));
    }

    @Test
    void reverseSelectedModulesUsePhysicalCoverageAndResumeKeepsRemainingModules() throws Exception {
        Path root = reactor();
        Path wrapper = wrapper();
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        run(root, wrapper, "-pl", "app/inner,app", goalCoordinate("format"));
        assertEquals(List.of(new Invocation(root.toRealPath(),
                List.of("format", Path.of(".").resolve("app").toString()))), readInvocations(record));
        Files.delete(record);
        Result resumed = run(root, wrapper, "-pl", "app/inner,app,sibling", "-rf", ":app", goalCoordinate("format"));
        List<Invocation> calls = readInvocations(record);
        assertEquals(2, calls.size(), calls + resumed.output());
        assertTrue(calls.contains(new Invocation(root.toRealPath(),
                List.of("format", Path.of(".").resolve("app").toString()))), calls.toString());
        assertTrue(calls.contains(new Invocation(root.toRealPath(),
                List.of("format", Path.of(".").resolve("sibling").toString()))), calls.toString());
    }

    @Test
    void noMarkerKeepsTheStartingDirectoryDespiteAnActiveAncestorProfile() throws Exception {
        Path root = project(temporaryDirectory.resolve("profile-root"));
        Path leaf = project(root.resolve("leaf with spaces"));
        String text = Files.readString(root.resolve("pom.xml"))
                .replace("<packaging>jar</packaging>", "<packaging>pom</packaging>")
                .replace("</project>", """
                    <profiles><profile><id>include-leaf</id>
                    <activation><activeByDefault>true</activeByDefault></activation>
                    <modules><module>unused/../leaf with spaces</module></modules>
                    </profile></profiles></project>
                    """);
        Files.createDirectory(root.resolve("unused"));
        Files.writeString(root.resolve("pom.xml"), text);
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        run(leaf, wrapper(), goalCoordinate("format"));
        assertEquals(List.of(new Invocation(leaf.toRealPath(), List.of("format"))), readInvocations(record));
    }

    @Test
    void aLifecycleBindingInOnlyOneModuleIsNotCoveredByAnUnboundAncestor() throws Exception {
        Path root = reactor();
        Path leaf = root.resolve("app/inner");
        String binding = """
                <build><plugins><plugin><groupId>rs.robusta</groupId><artifactId>robusta-maven-plugin</artifactId>
                <version>%s</version><executions><execution><phase>validate</phase><goals><goal>format</goal></goals>
                </execution></executions></plugin></plugins></build></project>
                """.formatted(pluginVersion());
        Files.writeString(leaf.resolve("pom.xml"), Files.readString(leaf.resolve("pom.xml")).replace("</project>", binding));
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        run(root, wrapper(), "validate");
        assertEquals(List.of(new Invocation(root.toRealPath(),
                List.of("format", Path.of(".").resolve("app/inner").toString()))), readInvocations(record));
    }

    @Test
    void coveredModuleWithItsOwnPluginRealmStillSkips() throws Exception {
        Path root = reactor();
        Path app = root.resolve("app/pom.xml");
        String plugin = """
                <build><plugins><plugin><groupId>rs.robusta</groupId><artifactId>robusta-maven-plugin</artifactId>
                <version>%s</version>%s</plugin></plugins></build></project>
                """.formatted(pluginVersion(), separateRealmDependencies(temporaryDirectory.resolve("local-repo")));
        Files.writeString(app, Files.readString(app).replace("</project>", plugin));
        Path wrapper = wrapper();
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        for (String goal : List.of("format", "config-show")) {
            Files.deleteIfExists(record);
            run(root, wrapper, goalCoordinate(goal));
            assertEquals(List.of(root.toRealPath().toString()), workingDirectories(record), goal);
        }
    }

    @Test
    void disjointAndPrunedModuleDirectoriesAreNotSkipped() throws Exception {
        Path root = reactor();
        project(root.resolve(".svn/generated"));
        project(root.resolve("nested-repo"));
        Files.createDirectory(root.resolve("nested-repo/.git"));
        Path outside = project(root.getParent().resolve("outside"));
        pom(root, "app", "sibling", ".svn/generated", "nested-repo", "../outside");
        Path record = temporaryDirectory.resolve("calls.txt");
        writeBehavior(temporaryDirectory, record, 0);
        run(root, wrapper(), goalCoordinate("format"));
        List<Invocation> calls = readInvocations(record);
        assertEquals(4, calls.size());
        for (Path path : List.of(root.resolve(".svn/generated"), root.resolve("nested-repo"), outside)) {
            Path scope = path.startsWith(root) ? Path.of(".").resolve(root.relativize(path)) : path.toRealPath();
            assertTrue(calls.contains(new Invocation(root.toRealPath(),
                    List.of("format", scope.toString()))), calls.toString());
        }
    }

    @Test
    void realNativeRunVisitsEachSourceOnceAndUsesTheRootBaselineForSelectedModules() throws Exception {
        Path root = reactor();
        Path wrapper = wrapper();
        for (String path : List.of("Root.java", "app/App.java", "app/inner/Inner.java")) {
            writeJavaSource(root, path, "class " + Path.of(path).getFileName().toString().replace(".java", "") + "{}\n");
        }
        Result format = runReal(root, wrapper, goalCoordinate("format-fix"));
        assertEquals(1, format.output().lines().filter(line -> line.contains("on 3 files using")).count(), format.output());
        assertFalse(format.output().contains("on 1 file"), format.output());
        Files.writeString(root.resolve("robusta.yaml"), "lint:\n  rules:\n    uppercase-long-suffix: deny\n");
        writeJavaSource(root, "app/App.java", "class App { long n = 1l; }\n");
        Result unbaselined = runMaven(root, temporaryDirectory.resolve("local-repo"), wrapper,
                "-pl", "app", goalCoordinate("lint"));
        assertEquals(1, unbaselined.exitCode(), unbaselined.output());
        runReal(root, wrapper, "-N", "-Drobusta.args=--suppress-all", goalCoordinate("lint"));
        assertTrue(Files.readString(root.resolve("robusta-suppressions.json")).contains("app/App.java"));
        writeJavaSource(root, "sibling/Sibling.java", "class Sibling { long n = 1l; }\n");
        Result selected = runReal(root, wrapper, "-pl", "app", goalCoordinate("lint"));
        assertTrue(selected.output().contains("on 2 files with"), selected.output());
        assertFalse(Files.exists(root.resolve("app/robusta-suppressions.json")));
    }

    private Path reactor() throws Exception {
        Path root = project(temporaryDirectory.resolve("root"));
        project(root.resolve("app/inner"));
        project(root.resolve("app"));
        project(root.resolve("sibling"));
        pom(root.resolve("app"), "inner");
        pom(root, "app", "sibling");
        Files.createDirectory(root.resolve(".mvn"));
        return root;
    }

    private Path project(Path directory) throws Exception {
        Files.createDirectories(directory);
        pom(directory);
        return directory;
    }

    private void pom(Path directory, String... modules) throws Exception {
        String id = directory.getFileName().toString().replace(' ', '-');
        String text = minimalPom(id);
        if (modules.length != 0) {
            StringBuilder entries = new StringBuilder("<modules>");
            for (String module : modules) {
                entries.append("<module>").append(module).append("</module>");
            }
            entries.append("</modules>");
            text = text.replace("<packaging>jar</packaging>", "<packaging>pom</packaging>")
                    .replace("</project>", entries + "</project>");
        }
        Files.writeString(directory.resolve("pom.xml"), text);
    }

    private Path wrapper() throws Exception {
        return createProject(temporaryDirectory, "launcher", minimalPom("launcher"));
    }

    private Result run(Path project, Path wrapper, String... arguments) throws Exception {
        Result result = runFakeMaven(project, temporaryDirectory.resolve("local-repo"), wrapper, arguments);
        assertEquals(0, result.exitCode(), result.output());
        return result;
    }

    private Result runReal(Path project, Path wrapper, String... arguments) throws Exception {
        Result result = runMaven(project, temporaryDirectory.resolve("local-repo"), wrapper, arguments);
        assertEquals(0, result.exitCode(), result.output());
        return result;
    }

    private List<String> workingDirectories(Path record) throws Exception {
        List<String> directories = new ArrayList<>();
        for (Invocation invocation : readInvocations(record)) {
            directories.add(invocation.cwd().toRealPath().toString());
        }
        return directories;
    }
}
