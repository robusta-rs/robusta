class Counter {
  void bump(int[] cells, int i) {
    cells[i]++;
    --this.total;
    for (int k = 0; k < cells.length; k++) cells[k]++;
    total = - --i + ++i;
    (total)++;
  }
}
