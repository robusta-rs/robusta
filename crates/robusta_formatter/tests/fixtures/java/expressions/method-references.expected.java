class MethodRefs {
  void wire() {
    names.forEach(System.out::println);
    mapper = Foo::bar;
    constructor = java.util.ArrayList<String>::new;
    arrays = int[]::new;
    primary = this::reset;
    parent = Outer.super::<String>adapt;
    converted = list.stream().map(Foo::<String>convert).toList();
  }
}
