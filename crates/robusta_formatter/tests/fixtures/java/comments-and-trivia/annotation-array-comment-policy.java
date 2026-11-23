class AnnotationArrayCommentPolicy {
  @SuppressWarnings("single") // annotation comment
  public <T> T annotated(T value) {
    return value;
  }

  @SuppressWarnings({
    "One", // first comment
    "Two", // second comment
  })
  public <T> T identity(T value) {
    return value;
  }

  @SuppressWarnings({
    "StaticOne", // static first comment
    "StaticTwo", // static second comment
  })
  public static <T> T staticIdentity(T value) {
    return value;
  }
}
