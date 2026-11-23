class YieldIdentifiers {
  AtomicBoolean yield;

  void run() {
    yield.set(true);
    yield = other;
    yield[0] = true;
    yield++;
    yield >>= 1;
  }
}
