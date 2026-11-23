// robusta-fixture: rule=unused-local-variable
// Unsafe fix: discard a nested call, and replace nested assignments with their values.
class UnusedLocalUnsafeLostEffect {
    int compute() { return 1; }

    String line() { return ""; }

    void method() {
        int ignored = compute() + 2;
    }

    int read(int a, int b) {
        String header;
        if ((header = line()) == null) return 0;
        int sum;
        return (sum = a + b) * 2;
    }
}
