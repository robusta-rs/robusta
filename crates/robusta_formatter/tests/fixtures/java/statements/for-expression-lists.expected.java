class Sweeper {
  void sweep(int lo, int hi) {
    int i, j;
    for (i = lo, j = hi; i < j; i++, j--) {
      swap(i, j);
    }
  }

  void sweepAll(int[] values) {
    for (
      leftCursorPosition = firstIndexOf(values),
        rightCursorPosition = lastIndexOf(values),
        passes = 0;
      leftCursorPosition < rightCursorPosition;
      leftCursorPosition++, rightCursorPosition--
    ) {
      swap(leftCursorPosition, rightCursorPosition);
    }
    for (
      cursor = 0;
      cursor < values.length;
      cursor++,
        comparisonsSoFar += values[cursor],
        swapsSoFar++,
        elapsed = System.nanoTime() - start
    ) {
      record(cursor);
    }
  }
}
