class Outer {
    class Inner {
        Outer enclosing() {
            return Outer . this ;
        }

        int value() {
            return Outer . this . value + this . local ;
        }

        Runnable task() {
            return a.b.Outer . this :: run ;
        }
    }
}
