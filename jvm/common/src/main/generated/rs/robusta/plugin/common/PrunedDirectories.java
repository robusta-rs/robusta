// Auto-generated code, DO NOT EDIT DIRECTLY!
// To regenerate: `just jvmgen`

package rs.robusta.plugin.common;

final class PrunedDirectories {
    private PrunedDirectories() {}

    static boolean contains(String name) {
        switch (name) {
            case ".git":
            case ".hg":
            case ".jj":
            case ".sl":
            case ".svn":
                return true;
            default:
                return false;
        }
    }
}
