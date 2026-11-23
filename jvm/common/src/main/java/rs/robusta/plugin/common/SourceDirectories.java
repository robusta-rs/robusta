package rs.robusta.plugin.common;

import java.io.File;

/** Physical source containment under the native directory traversal policy. */
public final class SourceDirectories {
    private SourceDirectories() {}

    /**
     * Tests whether an enclosing source selection also visits an inner directory.
     *
     * @param outer canonical enclosing directory
     * @param inner canonical candidate directory
     * @return whether no native pruning or repository boundary separates them
     */
    public static boolean covers(File outer, File inner) {
        if (!inner.toPath().startsWith(outer.toPath())) {
            return false;
        }
        for (File directory = inner; !directory.equals(outer); directory = directory.getParentFile()) {
            // Explicit roots bypass traversal pruning and restart repository ignore lookup.
            if (PrunedDirectories.contains(directory.getName())
                    || new File(directory, ".git").exists()
                    || new File(directory, ".jj").exists()) {
                return false;
            }
        }
        return true;
    }
}
