package rs.robusta.plugin.common;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.util.Collections;
import java.util.List;
import org.junit.jupiter.api.Test;

class CommandLineArgumentsTest {
    @Test
    void preservesQuotedBoundariesPathsAndOptionValues() throws RobustaPluginException {
        assertEquals(
                List.of(
                        "lint",
                        "--config",
                        "path with spaces/robusta.yaml",
                        "--option=value with spaces",
                        "C:\\work\\Example.java",
                        ""),
                parse(
                        "lint --config \"path with spaces/robusta.yaml\" "
                                + "'--option=value with spaces' C:\\work\\Example.java \"\""));
    }

    @Test
    void supportsEscapedWhitespaceAndQuotes() throws RobustaPluginException {
        assertEquals(
                List.of("two words", "say \"hello\"", "it's fine"),
                parse(
                        "two\\ words \"say \\\"hello\\\"\" 'it'\\''s fine'"));
    }

    @Test
    void rejectsUnmatchedQuotes() {
        assertThrows(RobustaPluginException.class, () -> parse("lint \"unterminated"));
    }

    /**
     * Carried over from the Maven copy of this suite, which had gained a case
     * the Gradle copy lacked before the two were merged.
     */
    @Test
    void anAbsentArgumentStringYieldsNoArguments() throws RobustaPluginException {
        assertEquals(Collections.emptyList(), parse(""));
    }

    private static List<String> parse(String commandLine) throws RobustaPluginException {
        return CommandLineArguments.parse(commandLine, "--args");
    }
}
