package rs.robusta.plugin.common;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.EnumSet;
import java.util.HashSet;
import java.util.Set;

import org.junit.jupiter.api.Test;

class RobustaFixedCommandTest {
    @Test
    void eachGoalNameIsTheDocumentedProjectionOfItsTaskName() {
        // Both published names are stated on the row rather than derived, so that neither can
        // change silently. This holds the two spellings to the one documented projection:
        // robusta<Verb><Flag> drops the prefix and spells the remainder in Maven's own style.
        for (RobustaFixedCommand command : RobustaFixedCommand.values()) {
            String taskName = command.gradleTaskName();
            assertTrue(taskName.startsWith("robusta"), taskName + " does not carry the task prefix");
            StringBuilder projected = new StringBuilder();
            for (char character : taskName.substring("robusta".length()).toCharArray()) {
                if (Character.isUpperCase(character) && projected.length() > 0) {
                    projected.append('-');
                }
                projected.append(Character.toLowerCase(character));
            }
            assertEquals(projected.toString(), command.mavenGoalName(), "projection of " + taskName);
        }
    }

    @Test
    void everyPublishedNameIsUniqueAndEveryRowRunsACommand() {
        Set<String> taskNames = new HashSet<>();
        Set<String> goalNames = new HashSet<>();
        for (RobustaFixedCommand command : RobustaFixedCommand.values()) {
            assertTrue(taskNames.add(command.gradleTaskName()), "duplicate task " + command);
            assertTrue(goalNames.add(command.mavenGoalName()), "duplicate goal " + command);
            assertFalse(command.arguments().isEmpty(), command + " runs no command");
            assertFalse(command.description().isEmpty(), command + " has no description");
        }
    }

    @Test
    void argumentsCannotBeMutatedThroughTheSharedTable() {
        assertThrows(
            UnsupportedOperationException.class,
            () -> RobustaFixedCommand.CHECK.arguments().add("--fix"));
    }

    @Test
    void onlyTheConfigAndVersionCommandsReadNoSources() {
        Set<RobustaFixedCommand> sourceless = EnumSet.of(
            RobustaFixedCommand.CONFIG_SHOW,
            RobustaFixedCommand.CONFIG_INIT,
            RobustaFixedCommand.VERSION);
        for (RobustaFixedCommand command : RobustaFixedCommand.values()) {
            assertEquals(!sourceless.contains(command), command.readsSources(), command.name());
        }
    }

    @Test
    void aTaskNameResolvesToItsCommandAndAnUnknownOneFails() {
        assertEquals(
            RobustaFixedCommand.CONFIG_INIT,
            RobustaFixedCommand.forGradleTaskName("robustaConfigInit"));
        assertThrows(
            IllegalArgumentException.class,
            () -> RobustaFixedCommand.forGradleTaskName("robustaConfigInitialise"));
    }
}
