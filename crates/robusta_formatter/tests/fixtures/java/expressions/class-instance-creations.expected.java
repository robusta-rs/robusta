class Factory {
  private final java.util.List<String> names = new java.util.ArrayList<>();

  Widget build(int size) {
    Widget widget = new Widget(size, new Config<String>("fast"));
    Widget typed = new <Widget> Widget(size);
    new audit.Logger(widget).record();
    return widget;
  }
}
