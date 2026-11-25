class Texts {
  String body = """
    hello
      world
    """;

  String escaped() {
    return """
      \"""
      \
      """;
  }
}
