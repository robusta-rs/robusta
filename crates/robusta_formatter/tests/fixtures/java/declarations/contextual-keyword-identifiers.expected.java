class ContextualKeywordIdentifiers {
  int exports;
  int module;
  int non;
  int open;
  int opens;
  int permits;
  int provides;
  int record;
  int requires;
  int sealed;
  int to;
  int transitive;
  int uses;
  int var;
  int when;
  int with;
  int yield;

  void locals() {
    int exports = 0,
      module = 0,
      non = 0,
      open = 0,
      opens = 0;
    int permits = 0,
      provides = 0,
      record = 0,
      requires = 0,
      sealed = 0;
    int to = 0,
      transitive = 0,
      uses = 0,
      var = 0,
      when = 0,
      with = 0,
      yield = 0;
  }

  void exports() {}

  void module() {}

  void record() {}

  void sealed() {}

  void var() {}

  void yield() {}

  void params(int record, int var, int yield, int sealed, int permits) {}
}

sealed class Shape permits Circle {}

non-sealed class FreeShape extends Shape {}

record Point(int x) {}

class Subtraction {
  int difference() {
    int non = 10,
      sealed = 3;
    return non - sealed;
  }

  void inference() {
    var inferred = 0;
  }
}
