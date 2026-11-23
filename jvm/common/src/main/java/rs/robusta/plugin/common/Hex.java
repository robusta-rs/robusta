package rs.robusta.plugin.common;

/** Lowercase hexadecimal encoding for checksum digests. */
public final class Hex {
    private static final char[] DIGITS = "0123456789abcdef".toCharArray();

    private Hex() {}

    /**
     * Encodes bytes as lowercase hexadecimal.
     *
     * @param bytes digest bytes
     * @return two lowercase hexadecimal characters per byte
     */
    public static String encode(byte[] bytes) {
        char[] encoded = new char[bytes.length * 2];
        for (int index = 0; index < bytes.length; index++) {
            int value = bytes[index] & 0xff;
            encoded[index * 2] = DIGITS[value >>> 4];
            encoded[index * 2 + 1] = DIGITS[value & 0x0f];
        }
        return new String(encoded);
    }
}
