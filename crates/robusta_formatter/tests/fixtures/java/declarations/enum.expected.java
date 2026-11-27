package com.example;

public enum Color implements Code, Named {
  @Deprecated RED(1),
  GREEN(),
  BLUE;

  private final int code;

  Color(int code) {
    this.code = code;
  }

  int code() {
    return code;
  }
}

enum Trailing {
  A,
}

enum Empty {}
