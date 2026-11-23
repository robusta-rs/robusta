package com.example;

public interface Cache<K, V> extends Store<K, V>, AutoCloseable {
  int LIMIT = 64;

  V get(K key);

  default V getOrNull(K key) {
    return get(key);
  }

  static <T> Cache<T, T> identity() {
    return null;
  }

  private void evict() {}

  ;

  class Support {}

  @interface Tag {
    String value();
  }
}

interface Id {}
