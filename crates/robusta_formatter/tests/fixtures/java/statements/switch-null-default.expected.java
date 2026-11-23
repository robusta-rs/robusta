class SwitchNullDefault {
  Object run(Object value) {
    return switch (value) {
      case null, default -> missing();
      case String name -> name;
    };
  }

  void legacy(Object value) {
    switch (value) {
      case null, default:
        handle();
    }
  }
}
