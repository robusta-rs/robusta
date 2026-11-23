// robusta-fixture: rule=unused-import
// Safe fix: a single-type import whose simple name is referenced nowhere in the
// compilation unit — neither in code nor in Javadoc — is deleted. A type
// header's use counts even where a member type has the same name.
import java.io.Serializable;
import java.util.List;
import static java.nio.ByteBuffer.wrap;
import static java.nio.CharBuffer.wrap;

class UnusedImport {
    int value() {
        return wrap(new byte[] {1}).get() + wrap(new char[] {'a'}).get();
    }
}

class HeaderUse implements Serializable {
    static class Serializable {}
}
