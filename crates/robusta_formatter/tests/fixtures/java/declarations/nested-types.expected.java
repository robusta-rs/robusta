package com.example;

public class Outer {
  private final int id;

  // Builder for Outer.
  public static final class Builder implements Cloneable {
    private int id;

    Outer build() {
      return new Outer();
    }

    static class Defaults {
      int id = 1;
    }
  }

  public interface Listener {
    void onChange(Outer outer);

    enum Priority {
      LOW,
      HIGH,
    }
  }

  private enum State implements Marker {
    NEW(1),
    DONE(2);

    private final int code;

    State(int code) {
      this.code = code;
    }

    interface Marker {}
  }

  void touch() {}
}
