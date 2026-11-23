// robusta-fixture: rule=duplicate-import
// Safe fix: an import repeated later in the file keeps its first copy, and
// the repeats are deleted, including on-demand and static imports.
import java.util.List;
import java.util.Map;
import java.util.List;
import java.util.function.*;
import static java.util.Collections.emptyList;
import java.util.function.*;
import static java.util.Collections.emptyList;
import java.util.Map;

class DuplicateImport {
    List<String> all = emptyList();
    Predicate<String> filter;
}
