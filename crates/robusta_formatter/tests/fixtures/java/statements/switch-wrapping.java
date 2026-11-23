class SwitchWrapping {
int rules(Object value) {
return switch (value) {
case FIRST_EXTREMELY_LONG_CONSTANT, SECOND_EXTREMELY_LONG_CONSTANT, THIRD_EXTREMELY_LONG_CONSTANT -> calculateResult(firstArgument, secondArgument);
case SHORT, TINY -> 1;
case String text -> { yield text.length(); }
default -> throw new UnsupportedOperationException("unsupported value");
};
}
void groups(Object value) {
switch (value) {
case FIRST_EXTREMELY_LONG_CONSTANT, SECOND_EXTREMELY_LONG_CONSTANT, THIRD_EXTREMELY_LONG_CONSTANT:
break;
default: break;
}
}
int comments(Object value) {
return switch (value) {
case FIRST_EXTREMELY_LONG_CONSTANT /* before */, // after
SECOND_EXTREMELY_LONG_CONSTANT,
/* leading */ THIRD_EXTREMELY_LONG_CONSTANT -> 1;
default -> 0;
};
}
}
