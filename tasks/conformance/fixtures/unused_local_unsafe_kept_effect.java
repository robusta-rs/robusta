// robusta-fixture: rule=unused-local-variable
// Unsafe fix: preserve calls and discard updates.
class UnusedLocalUnsafeKeptEffect {
    int compute() {
        return 1;
    }

    void method() {
        int ignored = compute();
        ignored++;
        ignored = ignored + 1;
        int parenthesized = compute();
        ((parenthesized)) += compute();
        int a = compute(), unused = compute(), b = compute();
        System.out.println(a + b);
        for (int extra = compute(); System.nanoTime() < 0; ) {}
        for (int i = 0, count = 0; i < 1; i++, count++) {}
        int extra = 0;
        for (extra = 1, extra = 2, compute(); System.nanoTime() < 0; extra++, extra++, compute()) {}
        Runnable retry = () -> compute();
    }
}
