class Varargs {
  Varargs(String... names) {}

  void log(final String prefix, int[]... counts) {
    sink = (String... values) -> values.length;
  }
}
