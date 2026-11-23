class SwitchPatterns {
  void run(Object value) {
    switch (value) {
      case @Nonnull String name when name.length() > 0 -> handle(name);
      case final var item -> handle(item);
      default -> throw bad();
    }
  }

  void legacy(Object value) {
    switch (value) {
      case String name, Integer count:
        handle(name);
      default:
    }
  }
}
