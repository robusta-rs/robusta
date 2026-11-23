package rs.robusta.gradle;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.Test;

class Utf8LineOutputStreamTest {
    @Test
    void decodesCharactersSplitAcrossWrites() {
        List<String> lines = new ArrayList<>();
        // U+241B SYMBOL FOR ESCAPE, then U+706B.
        byte[] bytes = {'L', 'i', (byte) 0xe2, (byte) 0x90, (byte) 0x9b, (byte) 0xe7, (byte) 0x81,
            (byte) 0xab, '\n'};

        try (Utf8LineOutputStream stream = new Utf8LineOutputStream(lines::add)) {
            stream.write(bytes, 0, 3);
            stream.write(bytes[3]);
            stream.write(bytes, 4, bytes.length - 4);
        }

        assertEquals(List.of("Li\u241b\u706b"), lines);
    }

    @Test
    void splitsLinesAndKeepsBlankOnes() {
        List<String> lines = new ArrayList<>();
        byte[] bytes = "first\r\n\nsecond\nlast".getBytes(StandardCharsets.UTF_8);

        try (Utf8LineOutputStream stream = new Utf8LineOutputStream(lines::add)) {
            stream.write(bytes, 0, bytes.length);
            assertEquals(List.of("first", "", "second"), lines);
        }

        assertEquals(List.of("first", "", "second", "last"), lines);
    }

    @Test
    void closingAfterACompleteLineEmitsNothingMore() {
        List<String> lines = new ArrayList<>();
        byte[] bytes = "only\n".getBytes(StandardCharsets.UTF_8);

        Utf8LineOutputStream stream = new Utf8LineOutputStream(lines::add);
        stream.write(bytes, 0, bytes.length);
        stream.close();
        stream.close();

        assertEquals(List.of("only"), lines);
    }
}
