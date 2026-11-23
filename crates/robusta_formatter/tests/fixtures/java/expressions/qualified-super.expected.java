class Child {
  void run() {
    Outer.super.reset();
    value = Outer.super.value;
    result = a.b.Outer.super.<String>adapt(input).size();
  }
}
