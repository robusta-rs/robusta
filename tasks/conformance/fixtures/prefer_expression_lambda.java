// robusta-fixture: rule=prefer-expression-lambda
// Safe fix: a lambda block that only returns a value takes the expression form.
// The returned expression is not a statement expression, so the rewritten
// lambda stays value-only and keeps its target type. Comments around the
// deleted tokens stay, and a line comment keeps the line break that ends it.
import java.util.function.IntUnaryOperator;

class PreferExpressionLambda {
    IntUnaryOperator increment() {
        return value -> { return value + 1; };
    }

    IntUnaryOperator scale() {
        return value -> { // doubled
            return /* twice */ value * 2; // exact
        };
    }
}
