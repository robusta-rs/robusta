package com.example.smoke;

import java.util.function.Function;

sealed interface Shape permits Circle, Square {
    int area();
}

record Circle(int radius) implements Shape {
    @Override
    public int area() {
        return radius * radius;
    }
}

record Square(int side) implements Shape {
    @Override
    public int area() {
        return side * side;
    }
}

@Deprecated
final class Smoke<T extends Number> {
    private final java.util.List<T> values = new java.util.ArrayList<>();
    private final String header = """
        shape
        """;

    Smoke() {
        values.add((T) Integer.valueOf(1));
    }

    Function<T, String> describe() {
        return value -> value.toString().trim();
    }

    String render(Shape shape) {
        return switch (shape) {
            case Circle(int radius) when radius > 0 -> header.trim() + radius;
            case Square(_) -> header.trim();
            case null, default -> "missing";
        };
    }

    int total() {
        int _ = 0;
        int result = 0;
        for (T value : values) {
            result += value.intValue();
        }
        return result;
    }
}
