package rs.robusta.plugin.common;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

/**
 * The fixed Robusta commands every build-tool adapter exposes.
 *
 * <p>One product surface, so one table. Each row carries everything an adapter needs to present
 * the command: the name it takes in each build tool, the sentence describing it, and the native
 * command line it runs. An adapter contributes only the framework mechanics around a row —
 * registering a Gradle task, declaring a Maven goal — never the row's contents.
 *
 * <p>Both published names are spelled out rather than derived from one another. The projection
 * between them is mechanical, but a published name that follows a computed value changes
 * silently when the computation does, so each is stated where it can be read and reviewed.
 */
public enum RobustaFixedCommand {
    /** Verify formatting and lint findings. */
    CHECK(
        "robustaCheck",
        "check",
        "Verifies formatting and lint findings with Robusta.",
        "check"
    ),
    /** Preview aggregate formatting and safe lint fixes. */
    CHECK_DIFF(
        "robustaCheckDiff",
        "check-diff",
        "Prints formatting and lint diffs with Robusta.",
        "check",
        "--diff"
    ),
    /** Apply formatting and safe lint fixes. */
    CHECK_FIX(
        "robustaCheckFix",
        "check-fix",
        "Applies formatting and lint fixes with Robusta.",
        "check",
        "--fix"
    ),
    /** Verify formatting. */
    FORMAT(
        "robustaFormat",
        "format",
        "Verifies formatting with Robusta.",
        "format"
    ),
    /** Preview formatting changes. */
    FORMAT_DIFF(
        "robustaFormatDiff",
        "format-diff",
        "Prints formatting diffs with Robusta.",
        "format",
        "--diff"
    ),
    /** Apply formatting changes. */
    FORMAT_FIX(
        "robustaFormatFix",
        "format-fix",
        "Applies formatting fixes with Robusta.",
        "format",
        "--fix"
    ),
    /** Report lint findings. */
    LINT(
        "robustaLint",
        "lint",
        "Verifies lint findings with Robusta.",
        "lint"
    ),
    /** Preview safe lint fixes. */
    LINT_DIFF(
        "robustaLintDiff",
        "lint-diff",
        "Prints lint-fix diffs with Robusta.",
        "lint",
        "--diff"
    ),
    /** Apply safe lint fixes. */
    LINT_FIX(
        "robustaLintFix",
        "lint-fix",
        "Applies lint fixes with Robusta.",
        "lint",
        "--fix"
    ),
    /** Print the resolved configuration. */
    CONFIG_SHOW(
        "robustaConfigShow",
        "config-show",
        "Prints the Robusta configuration resolved from this project directory.",
        "config"
    ),
    /** Create a starter {@code robusta.yaml}. */
    CONFIG_INIT(
        "robustaConfigInit",
        "config-init",
        "Creates a starter robusta.yaml in this project directory.",
        "init"
    ),
    /** Print the CLI version. */
    VERSION(
        "robustaVersion",
        "version",
        "Prints the Robusta CLI version.",
        "--version"
    );

    private final String gradleTaskName;
    private final String mavenGoalName;
    private final String description;
    private final List<String> arguments;

    RobustaFixedCommand(
        String gradleTaskName,
        String mavenGoalName,
        String description,
        String... arguments
    ) {
        this.gradleTaskName = gradleTaskName;
        this.mavenGoalName = mavenGoalName;
        this.description = description;
        this.arguments = Collections.unmodifiableList(new ArrayList<String>(Arrays.asList(arguments)));
    }

    /**
     * Returns the name this command takes as a Gradle task.
     *
     * @return the registered Gradle task name
     */
    public String gradleTaskName() {
        return gradleTaskName;
    }

    /**
     * Returns the name this command takes as a Maven goal, without the plugin's goal prefix.
     *
     * @return the declared Maven goal name
     */
    public String mavenGoalName() {
        return mavenGoalName;
    }

    /**
     * Returns the sentence describing this command to a build-tool user.
     *
     * @return the command description
     */
    public String description() {
        return description;
    }

    /**
     * Returns the native Robusta command line this command runs.
     *
     * @return the fixed arguments, never null
     */
    public List<String> arguments() {
        return arguments;
    }

    /**
     * Returns whether this command reads sources, so that it runs from the root directory limited
     * to a project's directory rather than in that directory.
     *
     * @return true for the check, format, and lint commands
     */
    public boolean readsSources() {
        String verb = arguments.get(0);
        return verb.equals("check") || verb.equals("format") || verb.equals("lint");
    }

    /**
     * Returns the command registered under a Gradle task name.
     *
     * @param taskName the registered Gradle task name
     * @return the matching command
     * @throws IllegalArgumentException when no fixed command carries that task name
     */
    public static RobustaFixedCommand forGradleTaskName(String taskName) {
        for (RobustaFixedCommand command : values()) {
            if (command.gradleTaskName.equals(taskName)) {
                return command;
            }
        }
        throw new IllegalArgumentException("Unknown fixed Robusta command: " + taskName);
    }
}
