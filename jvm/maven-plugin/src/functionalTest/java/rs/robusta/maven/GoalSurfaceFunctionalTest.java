package rs.robusta.maven;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.jar.JarFile;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.junit.jupiter.api.Test;

import rs.robusta.plugin.common.RobustaFixedCommand;

/**
 * Pins the published goal surface against the shared fixed-command table.
 *
 * <p>{@link RobustaFixedCommand} is the one statement of the command surface, and both adapters
 * compile it in. Maven's descriptor cannot be generated from it — a goal name and its description
 * are compile-time annotations and Javadoc on each mojo — so every mojo restates one row. This
 * test is what makes that restatement safe: it reads the descriptor out of the published jar and
 * holds it to the shared table, so a row and its mojo cannot drift apart, and so a command added
 * or renamed on the Gradle side cannot leave the Maven side behind.
 */
class GoalSurfaceFunctionalTest {
    private static final Pattern MOJO = Pattern.compile("<mojo>(.*?)</mojo>", Pattern.DOTALL);
    private static final Pattern GOAL = Pattern.compile("<goal>([^<]+)</goal>");
    private static final Pattern DESCRIPTION = Pattern.compile("<description>(.*?)</description>", Pattern.DOTALL);
    private static final Pattern GOAL_PREFIX = Pattern.compile("<goalPrefix>([^<]+)</goalPrefix>");

    @Test
    void publishesExactlyTheGoalsTheSharedCommandTableNames() throws Exception {
        List<String> expected = new ArrayList<>();
        for (RobustaFixedCommand command : RobustaFixedCommand.values()) {
            expected.add(command.mavenGoalName());
        }
        expected.sort(String::compareTo);

        List<String> actual = new ArrayList<>(publishedMojos().keySet());
        actual.sort(String::compareTo);

        assertEquals(expected, actual);
    }

    @Test
    void theGoalPrefixSuppliesTheNameGradleSpellsPerTask() throws Exception {
        Matcher matcher = GOAL_PREFIX.matcher(pluginDescriptor());
        assertTrue(matcher.find(), "the descriptor declares no goal prefix");
        assertEquals("robusta", matcher.group(1));
    }

    @Test
    void everyGoalPublishesTheDescriptionItsCommandCarries() throws Exception {
        Map<String, String> mojos = publishedMojos();
        for (RobustaFixedCommand command : RobustaFixedCommand.values()) {
            String block = mojos.get(command.mavenGoalName());
            assertTrue(block != null, "missing goal " + command.mavenGoalName());
            Matcher description = DESCRIPTION.matcher(block);
            assertTrue(description.find(), "goal " + command.mavenGoalName() + " publishes no description");
            assertEquals(
                    command.description(),
                    description.group(1).trim(),
                    "goal " + command.mavenGoalName() + " describes itself differently from its command");
        }
    }

    @Test
    void noGoalRequiresDependencyResolution() throws Exception {
        publishedMojos().forEach((goal, block) -> assertFalse(
                block.contains("<requiresDependencyResolution>"),
                "goal " + goal + " requires dependency resolution"));
    }

    @Test
    void everyGoalExposesTheArgsProperty() throws Exception {
        publishedMojos().forEach((goal, block) -> {
            assertTrue(block.contains("<name>args</name>"), goal + " has no args parameter");
            assertTrue(block.contains("${robusta.args}</args>"), goal + " does not bind robusta.args");
        });
    }

    /** Every published {@code <mojo>} block, keyed by its goal name. */
    private static Map<String, String> publishedMojos() throws IOException {
        Map<String, String> mojos = new LinkedHashMap<>();
        Matcher mojo = MOJO.matcher(pluginDescriptor());
        while (mojo.find()) {
            String block = mojo.group(1);
            Matcher goal = GOAL.matcher(block);
            assertTrue(goal.find(), "a <mojo> block declares no goal");
            mojos.put(goal.group(1), block);
        }
        return mojos;
    }

    private static String pluginDescriptor() throws IOException {
        Path jar = FunctionalTestSupport.publishedPlugin("jar");
        assertTrue(Files.isRegularFile(jar), "published plugin jar is missing: " + jar);
        try (JarFile jarFile = new JarFile(jar.toFile());
                InputStream input = jarFile.getInputStream(jarFile.getEntry("META-INF/maven/plugin.xml"))) {
            return new String(input.readAllBytes(), StandardCharsets.UTF_8);
        }
    }
}
