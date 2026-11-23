// robusta-fixture: rule=prefer-expression-lambda
// Unsafe fix: a lambda block that only returns a statement expression takes
// the expression form. Each lambda's only target is its method's return type,
// so the wider set of compatible targets changes nothing here.
import java.util.function.IntUnaryOperator;
import java.util.function.Supplier;

class PreferExpressionLambdaUnsafe {
    IntUnaryOperator absolute() {
        return value -> { return Math.abs(value); };
    }

    Supplier<StringBuilder> builder() {
        return () -> { return new StringBuilder(); };
    }
}
