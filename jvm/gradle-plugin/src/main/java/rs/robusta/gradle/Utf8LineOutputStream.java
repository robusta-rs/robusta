package rs.robusta.gradle;

import java.io.ByteArrayOutputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.function.Consumer;

/**
 * Decodes Robusta's UTF-8 process output into lines.
 *
 * <p>Gradle decodes forwarded process output in the daemon's default charset,
 * which is the ANSI code page on Windows before Java 18.
 */
final class Utf8LineOutputStream extends OutputStream {
    private final Consumer<String> lines;
    private final ByteArrayOutputStream line = new ByteArrayOutputStream();

    Utf8LineOutputStream(Consumer<String> lines) {
        this.lines = lines;
    }

    @Override
    public synchronized void write(int b) {
        if ((byte) b == '\n') {
            emitLine();
        } else {
            line.write(b);
        }
    }

    @Override
    public synchronized void write(byte[] bytes, int offset, int length) {
        int start = offset;
        int end = offset + length;
        for (int i = offset; i < end; i++) {
            if (bytes[i] == '\n') {
                line.write(bytes, start, i - start);
                emitLine();
                start = i + 1;
            }
        }
        line.write(bytes, start, end - start);
    }

    /** Emits a final line that has no terminator. */
    @Override
    public synchronized void close() {
        if (line.size() > 0) {
            emitLine();
        }
    }

    private void emitLine() {
        byte[] bytes = line.toByteArray();
        line.reset();
        int length = bytes.length;
        if (length > 0 && bytes[length - 1] == '\r') {
            length--;
        }
        lines.accept(new String(bytes, 0, length, StandardCharsets.UTF_8));
    }
}
