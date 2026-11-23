// robusta-fixture: rule=redundant-lambda-parameter-parentheses
// Safe fix: one concise lambda parameter does not require parentheses.
import java.util.function.IntUnaryOperator;

class RedundantLambdaParameterParentheses {
    IntUnaryOperator identity() {
        return (value) -> value;
    }

    IntUnaryOperator compact() {
        return(value)->value + 1;
    }

    IntUnaryOperator compactEscaped() {
        return(\u0076alue)->value + 1;
    }

    IntUnaryOperator compactEscapedOpening() {
        return\u0028value)->value + 1;
    }

    IntUnaryOperator escapedOpening() {
        return \u0028value) -> value;
    }

    IntUnaryOperator escapedClosing() {
        return (value\u0029 -> value;
    }

    IntUnaryOperator escapedBoth() {
        return \uu0028 /* before */ value /* after */ \uuu0029 -> value;
    }
}
