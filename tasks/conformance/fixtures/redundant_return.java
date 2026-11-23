// robusta-fixture: rule=redundant-return
// Safe fix: falling through from the return ends the body it would leave.
class RedundantReturn {
    void greet(String name) {
        System.out.println(name);
        return;
    }

    void escapedKeyword(String name) {
        System.out.println(name);
        \u0072eturn;
    }

    void escapedSemicolon(String name) {
        System.out.println(name);
        return /* done */ \uu003B
    }

    void branch(boolean dirty) {
        if (dirty) {
            System.out.println("flush");
            return;
        } else if (!dirty)
            return;
    }

    void group(int mode) {
        switch (mode) {
            case 1:
                System.out.println("one");
                break;
            default:
                System.out.println("reload");
                return;
        }
    }

    void attempt(Object lock) {
        try {
            synchronized (lock) {
                System.out.println("flush");
                return;
            }
        } catch (RuntimeException e) {
            System.out.println(e);
            return;
        } finally {
            System.out.println("done");
        }
    }

    void labeled(boolean ready) {
        done:
        if (ready) return;
    }

    Runnable lambda() {
        return () -> {
            System.out.println("run");
            return;
        };
    }
}
