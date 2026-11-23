// robusta-fixture: rule=uppercase-long-suffix
// Safe fix: the lowercase long-literal suffix `l` becomes `L`.
class UppercaseLongSuffix {
    long value() {
        return 10l;
    }

    long escaped() {
        return 0xFF\uu006c;
    }
}
