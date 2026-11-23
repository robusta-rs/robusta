package rs.robusta.maven;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.function.BiPredicate;

import org.apache.maven.execution.MavenSession;
import org.apache.maven.plugin.MojoExecutionException;
import org.apache.maven.project.MavenProject;

import rs.robusta.plugin.common.SourceDirectories;

/** Root discovery and goal skipping for one Maven reactor. */
final class RobustaReactor {
    private final File root;
    private final List<File> sourceDirectories;
    private final List<File> configDirectories;

    private RobustaReactor(MavenSession session) throws MojoExecutionException {
        File start = directory(new File(session.getExecutionRootDirectory()));
        File discovered = start;
        for (File ancestor = start; ancestor != null; ancestor = ancestor.getParentFile()) {
            if (new File(ancestor, ".mvn").isDirectory()) {
                discovered = ancestor;
                break;
            }
        }
        root = discovered;
        List<File> selected = new ArrayList<>();
        for (MavenProject project : session.getProjects()) {
            File candidate = directory(project.getBasedir());
            if (!selected.contains(candidate)) {
                selected.add(candidate);
            }
        }
        sourceDirectories = outermost(selected, SourceDirectories::covers);
        // The enclosing module's configuration file also applies inside it.
        configDirectories = outermost(selected, (outer, inner) -> inner.toPath().startsWith(outer.toPath()));
    }

    private static List<File> outermost(List<File> selected, BiPredicate<File, File> covers) {
        List<File> outermost = new ArrayList<>();
        for (File candidate : selected) {
            boolean covered = false;
            for (File outer : selected) {
                if (!outer.equals(candidate) && covers.test(outer, candidate)) {
                    covered = true;
                    break;
                }
            }
            if (!covered) {
                outermost.add(candidate);
            }
        }
        return outermost;
    }

    static RobustaReactor get(MavenSession session) throws MojoExecutionException {
        MavenProject owner = session.getTopLevelProject();
        synchronized (owner) {
            String key = RobustaReactor.class.getName();
            RobustaReactor reactor = (RobustaReactor) owner.getContextValue(key);
            if (reactor == null) {
                reactor = new RobustaReactor(session);
                owner.setContextValue(key, reactor);
            }
            return reactor;
        }
    }

    File root() {
        return root;
    }

    boolean skips(MavenProject project, boolean commandLine, boolean source) throws MojoExecutionException {
        List<File> outermost = source ? sourceDirectories : configDirectories;
        return commandLine && !outermost.contains(directory(project.getBasedir()));
    }

    static File directory(File file) throws MojoExecutionException {
        try {
            return file.getCanonicalFile();
        } catch (IOException failure) {
            throw new MojoExecutionException("Cannot resolve Robusta directory: " + file, failure);
        }
    }
}
