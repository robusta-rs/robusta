package rs.robusta.maven;

import java.io.IOException;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.StringJoiner;

import org.junit.jupiter.api.Assumptions;

final class FunctionalTestSupport {
    private static final String REAL_BUNDLED_PROPERTY = "robusta.real.bundled";

    private FunctionalTestSupport() {}

    static String pluginVersion() {
        return System.getProperty("robusta.functionalTest.pluginVersion");
    }

    static String pluginRepositoryUrl() {
        Path repository = Path.of(System.getProperty("robusta.functionalTest.repository"));
        return repository.toUri().toString();
    }

    /** Creates a consumer Maven project with a copy of this module's Maven Wrapper. */
    static Path createProject(Path parent, String name, String pomXml) throws IOException {
        Path project = Files.createDirectories(parent.resolve(name));
        Files.writeString(project.resolve("pom.xml"), pomXml, StandardCharsets.UTF_8);
        copyWrapper(project);
        return project;
    }

    static String minimalPom(String artifactId) {
        return """
                <?xml version="1.0" encoding="UTF-8"?>
                <project xmlns="http://maven.apache.org/POM/4.0.0">
                  <modelVersion>4.0.0</modelVersion>
                  <groupId>rs.robusta.functionaltest</groupId>
                  <artifactId>%s</artifactId>
                  <version>1.0.0</version>
                  <packaging>jar</packaging>
                  <properties>
                    <maven.compiler.source>17</maven.compiler.source>
                    <maven.compiler.target>17</maven.compiler.target>
                    <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
                  </properties>
                  <pluginRepositories>
                    <pluginRepository>
                      <id>robusta-functional-test</id>
                      <url>%s</url>
                    </pluginRepository>
                  </pluginRepositories>
                </project>
                """
                .formatted(artifactId, pluginRepositoryUrl());
    }

    static void writeJavaSource(Path project, String relativePath, String content) throws IOException {
        Path file = project.resolve(relativePath);
        Files.createDirectories(file.getParent());
        Files.writeString(file, content, StandardCharsets.UTF_8);
    }

    record Result(int exitCode, String output) {}

    static String goalCoordinate(String goal) {
        return "rs.robusta:robusta-maven-plugin:" + pluginVersion() + ":" + goal;
    }

    /**
     * Runs the project's Maven Wrapper with the given arguments, e.g. {@code goalCoordinate("format")},
     * against the published plugin and the real CLI it bundles.
     */
    static Result runMaven(Path project, Path localRepository, String... arguments) throws IOException, InterruptedException {
        return runMaven(project, localRepository, project, arguments);
    }

    static Result runMaven(Path project, Path localRepository, Path wrapperProject, String... arguments)
            throws IOException, InterruptedException {
        Assumptions.assumeTrue(
                Boolean.getBoolean(REAL_BUNDLED_PROPERTY),
                "Pass -Drobusta.real.bundled=true with bundled resources to run acceptance checks");
        return run(project, localRepository, wrapperProject, arguments);
    }

    /**
     * Runs the project's Maven Wrapper against a plugin whose bundled binary is the native fake.
     * The fake replaces the published plugin in {@code localRepository}, so real runs need another.
     */
    static Result runFakeMaven(Path project, Path localRepository, String... arguments)
            throws IOException, InterruptedException {
        return runFakeMaven(project, localRepository, project, arguments);
    }

    static Result runFakeMaven(Path project, Path localRepository, Path wrapperProject, String... arguments)
            throws IOException, InterruptedException {
        installFakePlugin(localRepository);
        return run(project, localRepository, wrapperProject, arguments);
    }

    // Maven resolves a local artifact with no recorded origin as installed, ahead of the published jar.
    private static void installFakePlugin(Path localRepository) throws IOException {
        String version = pluginVersion();
        String name = "robusta-maven-plugin-" + version;
        Path coordinate = Path.of("rs/robusta/robusta-maven-plugin", version);
        Path directory = Files.createDirectories(localRepository.resolve(coordinate));
        Path published = Path.of(System.getProperty("robusta.functionalTest.repository")).resolve(coordinate);
        Files.copy(published.resolve(name + ".pom"), directory.resolve(name + ".pom"), StandardCopyOption.REPLACE_EXISTING);
        Files.copy(
                Path.of(System.getProperty("robusta.functionalTest.fakePlugin")),
                directory.resolve(name + ".jar"),
                StandardCopyOption.REPLACE_EXISTING);
    }

    private static Result run(Path project, Path localRepository, Path wrapperProject, String... arguments)
            throws IOException, InterruptedException {
        List<String> command = new ArrayList<>();
        command.add(wrapperProject.resolve(isWindows() ? "mvnw.cmd" : "mvnw").toAbsolutePath().toString());
        command.add("-e");
        command.add("-B");
        command.add("-Dmaven.repo.local=" + localRepository.toAbsolutePath());
        command.addAll(List.of(arguments));
        if (isWindows()) {
            command = windowsShellCommand(command);
        }
        ProcessBuilder builder = new ProcessBuilder(command)
                .directory(project.toRealPath().toFile())
                .redirectErrorStream(true);
        builder.environment().put("MAVEN_BASEDIR", wrapperProject.toAbsolutePath().toString());
        Process process = builder.start();
        String output = new String(process.getInputStream().readAllBytes(), StandardCharsets.UTF_8);
        int exitCode = process.waitFor();
        return new Result(exitCode, output);
    }

    /**
     * Runs the Windows wrapper through {@code cmd}, which hands its arguments to {@code java.exe}
     * as raw text. {@link ProcessBuilder} neither escapes embedded quotes nor requotes an argument
     * that is already quoted, so the line is quoted here by the rules {@code java.exe} splits it
     * with, and {@code /s} strips only the outer quotes.
     */
    private static List<String> windowsShellCommand(List<String> command) {
        StringJoiner line = new StringJoiner(" ", "\"", "\"");
        for (String argument : command) {
            line.add(windowsArgument(argument));
        }
        return List.of("cmd", "/s", "/c", line.toString());
    }

    private static String windowsArgument(String argument) {
        StringBuilder quoted = new StringBuilder("\"");
        int backslashes = 0;
        for (int index = 0; index < argument.length(); index++) {
            char character = argument.charAt(index);
            if (character == '\\') {
                backslashes++;
                continue;
            }
            // Backslashes are literal unless a quote follows them.
            quoted.append("\\".repeat(character == '"' ? 2 * backslashes + 1 : backslashes)).append(character);
            backslashes = 0;
        }
        return quoted.append("\\".repeat(2 * backslashes)).append('"').toString();
    }

    private static void copyWrapper(Path project) throws IOException {
        Map<String, Boolean> wrapperFiles = new LinkedHashMap<>();
        wrapperFiles.put("wrapper/mvnw", true);
        wrapperFiles.put("wrapper/mvnw.cmd", false);
        wrapperFiles.put("wrapper/.mvn/wrapper/maven-wrapper.jar", false);
        wrapperFiles.put("wrapper/.mvn/wrapper/maven-wrapper.properties", false);
        for (Map.Entry<String, Boolean> entry : wrapperFiles.entrySet()) {
            String resource = entry.getKey();
            String relativePath = resource.substring("wrapper/".length());
            Path destination = project.resolve(relativePath);
            Files.createDirectories(destination.getParent());
            try (var input = resourceStream(resource)) {
                Files.copy(input, destination);
            }
            if (entry.getValue()) {
                destination.toFile().setExecutable(true);
            }
        }
    }

    private static java.io.InputStream resourceStream(String resource) throws IOException {
        URL url = FunctionalTestSupport.class.getClassLoader().getResource(resource);
        if (url == null) {
            throw new IOException("Missing functional-test resource " + resource);
        }
        return url.openStream();
    }

    private static boolean isWindows() {
        return System.getProperty("os.name").toLowerCase(Locale.ROOT).contains("windows");
    }
}
