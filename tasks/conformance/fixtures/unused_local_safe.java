// robusta-fixture: rule=unused-local-variable
// Safe fix: remove the declaration and literal assignments.
class UnusedLocalSafe {
    void method() {
        int ignored = 0;
        if (System.nanoTime() < 0) ignored = 1;
        else ignored = 2;
        for (int kept = 0, extra = 1; kept < 1; kept++) {}
        for (int separator = 0\u003b System.nanoTime() < 0; ) {}
        int parenthesized;
        (parenthesized) = 1;
        int a = 1, unused = 2, b = 3;
        System.out.println(a + b);
    }
}
