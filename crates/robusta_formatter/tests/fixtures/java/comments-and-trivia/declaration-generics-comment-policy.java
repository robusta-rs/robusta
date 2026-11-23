class DeclarationGenericsCommentPolicy<T> {
  /* constructor type parameters */ <U /* constructor bound */ extends Number>
      DeclarationGenericsCommentPolicy(U value) {}

  static /* method type parameters */ <U /* method bound */ extends Number> U identity(U value) {
    return value;
  }

  <U> /* constructor name */ DeclarationGenericsCommentPolicy(U value, int count) {}

  <U> /* return type */ U first(U value) {
    return value;
  }
}
