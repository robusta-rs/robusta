public record Pair<T>(@Nonnull String left, List<T> right, String @Nullable ... tags)
    implements Named {
  Pair(String left, List<T> right, String... tags) {
    this.left = left;
  }

  @Deprecated
  public Pair {
    if (left == null) {
      throw bad();
    }
  }

  int count() {
    return tags.length;
  }

  record Meta(int code) {}
}

class Uses {
  record value;
}
