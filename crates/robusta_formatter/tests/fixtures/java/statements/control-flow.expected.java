class Walker {
  int classify(int a) {
    if (a < 0) {
      return -1;
    } else if (a == 0) {
      return 0;
    } else {
      return 1;
    }
  }

  void drain(java.util.List<Task> tasks) {
    outer:
    for (int i = 0, n = tasks.size(); i < n; i += 1) {
      for (final Task task : tasks) {
        if (task.isDone()) continue;
        if (task.isPoison()) break outer;
        task.run();
      }
    }
    while (pending > 0) poll();
    do {
      tick();
    } while (running);
    for (;;) {
      spin();
    }
    {
      int scoped = 1;
      use(scoped);
    }
  }
}
