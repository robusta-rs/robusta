class GenericConstructors {
  GenericConstructors(Object value) {
    <String>this(value, 1);
  }

  GenericConstructors(String value, int count) {
    <java.util.List<String>, Integer>super(value);
    this.count = count;
  }

  GenericConstructors(Outer owner, Object value) {
    owner.super(value);
  }

  GenericConstructors(Supplier supplier) {
    supplier.get().<String>super(name());
  }
}
