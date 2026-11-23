class FlexibleConstructors {
  FlexibleConstructors(String name) {
    String normalized = name.trim();
    this(normalized, 0);
    audit(normalized);
  }

  FlexibleConstructors(Outer owner) {
    Object key = owner.key();
    owner.super(key);
    init();
  }
}
