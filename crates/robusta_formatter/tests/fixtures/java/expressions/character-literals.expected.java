class Chars {
  char letter() {
    return 'a';
  }

  boolean matches(char c) {
    return c == '\n' || c == '\'' || c == '\123' || c == '\u0061';
  }
}
