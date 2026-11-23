class SeparatorCommentPolicy {
  enum Constants {
    FIRST /* after constant */,
    SECOND,
  }

  void enhancedFor(java.util.List<String> values) {
    for (String value /* after variable */ : values) {
      use(value);
    }
  }

  void assertion(boolean ok) {
    assert ok /* after condition */ : "message";
  }

  void use(String value) {}
}
