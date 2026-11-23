class Tasks {
  Runnable r = () -> {};

  void wire() {
    inc = x -> x + 1;
    handler = (x) -> x;
    chain = x -> y -> x + y;
    pick = flag ? x -> low : (y) -> high;
    each = (final int a, int b) -> a + b;
    run((Runnable) () -> {
      counter++;
    });
  }

  Predicate<Argument> tables() {
    return argumentSpecification ->
      argumentSpecification instanceof TableArgumentSpecificationValue;
  }
}
