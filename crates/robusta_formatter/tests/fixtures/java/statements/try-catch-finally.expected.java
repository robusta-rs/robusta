class Files {
  void read(Path path) {
    try {
      open(path);
    } catch (final java.io.IOException | IllegalStateException ex) {
      throw ex;
    } catch (RuntimeException ex) {
      recover(ex);
    } finally {
      close();
    }
    try {
      cleanup();
    } finally {
      close();
    }
  }
}
