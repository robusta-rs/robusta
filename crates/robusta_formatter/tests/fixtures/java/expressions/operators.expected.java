class Bits {
  int mix(int value, long seed) {
    int hash = (int) (seed >>> 32) ^ value;
    hash = hash << 5 | hash >>> 27;
    hash <<= 1;
    hash >>= 2;
    hash >>>= 3;
    return hash;
  }

  boolean check(Object o) {
    if (o instanceof CharSequence && ((CharSequence) o).length() > 0) return true;
    return o instanceof int[];
  }

  long widen(byte b) {
    return (long) -b * (long) b;
  }

  Object read(java.util.Map<String, String> raw) {
    return (java.util.Map<String, String>) raw;
  }
}
