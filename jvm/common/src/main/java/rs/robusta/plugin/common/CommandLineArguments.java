package rs.robusta.plugin.common;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/** Shell-style tokenizer for a user-supplied extra-argument string. */
public final class CommandLineArguments {
    private CommandLineArguments() {}

    /**
     * Splits {@code commandLine} into arguments the way a POSIX shell would.
     *
     * @param commandLine the raw user-supplied string
     * @param optionLabel how the surrounding build tool names this option, used
     *     only to make the failure message actionable ({@code --args} for
     *     Gradle, {@code robusta.args} for Maven)
     * @return the parsed arguments, never null
     * @throws RobustaPluginException when a quote is left unmatched
     */
    public static List<String> parse(String commandLine, String optionLabel)
            throws RobustaPluginException {
        List<String> arguments = new ArrayList<>();
        StringBuilder argument = new StringBuilder();
        char quote = 0;
        boolean started = false;

        for (int index = 0; index < commandLine.length(); index++) {
            char current = commandLine.charAt(index);

            if (quote == 0 && Character.isWhitespace(current)) {
                if (started) {
                    arguments.add(argument.toString());
                    argument.setLength(0);
                    started = false;
                }
                continue;
            }

            if (current == '\'' && quote != '"') {
                quote = quote == '\'' ? 0 : '\'';
                started = true;
                continue;
            }

            if (current == '"' && quote != '\'') {
                quote = quote == '"' ? 0 : '"';
                started = true;
                continue;
            }

            if (current == '\\' && quote != '\'' && index + 1 < commandLine.length()) {
                char next = commandLine.charAt(index + 1);
                if (isEscaped(next, quote)) {
                    argument.append(next);
                    started = true;
                    index++;
                    continue;
                }
            }

            argument.append(current);
            started = true;
        }

        if (quote != 0) {
            throw new RobustaPluginException(
                    "Cannot parse " + optionLabel + ": unmatched " + quote + " quote");
        }
        if (started) {
            arguments.add(argument.toString());
        }
        return Collections.unmodifiableList(arguments);
    }

    private static boolean isEscaped(char character, char quote) {
        if (quote == '"') {
            return character == '"' || character == '\\';
        }
        return Character.isWhitespace(character)
                || character == '\''
                || character == '"'
                || character == '\\';
    }
}
