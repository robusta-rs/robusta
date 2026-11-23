class GenericCalls {
  void wire() {
    sink = java.util.Collections.<String>emptyList();
    this.<Integer>accept(1);
    result = source.<java.util.Map<String, ? extends Number>>convert(value).size();
  }
}
