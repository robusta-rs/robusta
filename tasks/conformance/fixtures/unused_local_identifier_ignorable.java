// robusta-fixture: rule=unused-local-variable
class UnusedLocalIdentifierIgnorable {
    void method() {
        int coun\u200Ct;
        count = 1;
        int stale;
    }
    int supplementaryIdentity() {
        int coun\uDB40\uDC01t;
        int count;
        coun\uDB40\uDC01t = 1;
        count = 2;
        return coun\uDB40\uDC01t + count;
    }
}
