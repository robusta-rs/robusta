package terminators /* package */;

import java.util.List /* import */;

@interface Marker {
    int weight() default 1 /* annotation member */;
}

abstract class TerminatorCommentPolicy {
    int field /* field */;

    int moved // line comment
    ; /* block comment */

    int deferred
    /* own line, leads the next field */;

    abstract void bodyless(List<String> values) /* body-less method */;

    enum Constants {
        ONLY /* before the enum terminator */;

        int member;
    }

    int statements() {
        int local = 1 /* local declaration */;
        use(local) /* expression statement */;
        use(local) // line comment
        ; /* block comment */
        for (int index = 0 /* for initializer */; index < 2; index++) {
            use(index);
        }
        return local /* return */;
    }

    void use(int value) {}
}
