package rs.robusta.plugin.common;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

import org.junit.jupiter.api.Test;

class HexTest {
    @Test
    void encodesEveryByteAsTwoLowercaseDigits() {
        byte[] bytes = new byte[256];
        for (int value = 0; value < bytes.length; value++) {
            bytes[value] = (byte) value;
        }

        String encoded = Hex.encode(bytes);

        assertEquals(512, encoded.length());
        assertEquals("000102030405060708090a0b0c0d0e0f", encoded.substring(0, 32));
        assertEquals("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff", encoded.substring(480));
    }

    @Test
    void encodesAnEmptyDigestAsAnEmptyString() {
        assertEquals("", Hex.encode(new byte[0]));
    }

    @Test
    void matchesTheKnownSha256OfAnEmptyInput() throws NoSuchAlgorithmException {
        MessageDigest digest = MessageDigest.getInstance("SHA-256");

        assertEquals(
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                Hex.encode(digest.digest(new byte[0])));
        assertEquals(
                "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae",
                Hex.encode(digest.digest("foo".getBytes(StandardCharsets.UTF_8))));
    }
}
