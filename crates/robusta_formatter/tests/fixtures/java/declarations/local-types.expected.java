class LocalTypes {
  void build() {
    @Marker
    final class Local {
      int value;

      int read() {
        return value;
      }
    }
    interface Helper {
      void help();
    }
    enum Kind {
      ONE,
      TWO,
    }
    int after = 1;
  }
}
