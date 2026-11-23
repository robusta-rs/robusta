class SwitchRules {
  void run(int code) {
    switch (code) {
      case 1, 2 -> handle(code);
      case 3 -> {
        prepare();
        break;
      }
      default -> throw bad();
    }
  }
}
