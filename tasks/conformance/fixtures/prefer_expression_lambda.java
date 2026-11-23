// robusta-fixture: rule=prefer-expression-lambda
// Safe fix: a lambda block that only returns a value takes the expression form.
// The returned expression is not a statement expression, so the rewritten
// lambda stays value-only and keeps its target type.
import java.util.function.IntUnaryOperator;

class PreferExpressionLambda {
    IntUnaryOperator increment() {
        return value -> { return value + 1; };
    }
}
