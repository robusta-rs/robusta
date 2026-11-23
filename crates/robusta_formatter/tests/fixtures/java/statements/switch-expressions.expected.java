class SwitchExpressions {
  int run(int code) {
    return switch (code) {
      case 1, 2 -> 10;
      case 3 -> {
        yield compute(code);
      }
      default -> throw bad();
    };
  }

  int legacy(int code) {
    return switch (code) {
      case 1:
        yield 10;
      default:
        yield 0;
    };
  }
}
