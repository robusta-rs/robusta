// robusta-fixture: rule=redundant-super-call
// Safe fix: deleting a constructor's first-statement, no-argument super call
// makes Java insert the same implicit constructor invocation.
class RedundantSuperCallParent {
    RedundantSuperCallParent() {}
}

class RedundantSuperCall extends RedundantSuperCallParent {
    RedundantSuperCall() {
        super();
        initialize();
    }

    void initialize() {}
}
