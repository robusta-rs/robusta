class Recovery {
  void recover() {
    try {
      load();
    } catch (InstantiationException
        | IllegalAccessException
        | InvocationTargetException exception) {
      report(exception);
    }
    try {
      load();
    } catch (FirstException | SecondException exception) {
      report(exception);
    }
  }
}
