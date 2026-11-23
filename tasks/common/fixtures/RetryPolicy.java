package com.example.net;

import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.Random;
import java.util.concurrent.Callable;
import java.util.function.Predicate;

/**
 * An immutable exponential-backoff retry policy with full jitter.
 *
 * <p>Instances are built once and shared; every {@link #execute(Callable)}
 * call tracks its own attempt state, so a single policy can serve any number
 * of concurrent callers.
 */
public final class RetryPolicy {

    private static final Duration DEFAULT_INITIAL_DELAY = Duration.ofMillis(100);
    private static final Duration DEFAULT_MAX_DELAY = Duration.ofSeconds(30);

    private final int maxAttempts;
    private final Duration initialDelay;
    private final Duration maxDelay;
    private final double multiplier;
    private final Predicate<Exception> retryOn;
    private final Random random;

    private RetryPolicy(Builder builder) {
        this.maxAttempts = builder.maxAttempts;
        this.initialDelay = builder.initialDelay;
        this.maxDelay = builder.maxDelay;
        this.multiplier = builder.multiplier;
        this.retryOn = builder.retryOn;
        this.random = new Random(builder.seed);
    }

    public static Builder builder() {
        return new Builder();
    }

    /** Runs {@code task}, retrying per this policy until success or exhaustion. */
    public <T> T execute(Callable<T> task) throws RetriesExhaustedException {
        List<Exception> failures = new ArrayList<>();
        for (int attempt = 1; attempt <= maxAttempts; attempt++) {
            try {
                return task.call();
            } catch (Exception e) {
                failures.add(e);
                if (attempt == maxAttempts || !retryOn.test(e)) {
                    throw new RetriesExhaustedException(attempt, failures);
                }
                sleep(delayFor(attempt));
            }
        }
        throw new IllegalStateException("unreachable: loop either returns or throws");
    }

    /** Computes the jittered delay before the given 1-based attempt retries. */
    Duration delayFor(int attempt) {
        double scale = Math.pow(multiplier, attempt - 1L);
        long cappedMillis = Math.min(
                (long) (initialDelay.toMillis() * scale),
                maxDelay.toMillis());
        long jittered = cappedMillis == 0 ? 0 : random.nextLong(cappedMillis + 1);
        return Duration.ofMillis(jittered);
    }

    private static void sleep(Duration duration) {
        try {
            Thread.sleep(duration.toMillis());
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new IllegalStateException("interrupted while backing off", e);
        }
    }

    /** Thrown when every allowed attempt has failed. */
    public static final class RetriesExhaustedException extends Exception {

        private final int attempts;
        private final List<Exception> failures;

        RetriesExhaustedException(int attempts, List<Exception> failures) {
            super("gave up after " + attempts + " attempts", failures.get(failures.size() - 1));
            this.attempts = attempts;
            this.failures = List.copyOf(failures);
        }

        public int attempts() {
            return attempts;
        }

        public List<Exception> failures() {
            return failures;
        }
    }

    /** Mutable staging area for {@link RetryPolicy} configuration. */
    public static final class Builder {

        private int maxAttempts = 3;
        private Duration initialDelay = DEFAULT_INITIAL_DELAY;
        private Duration maxDelay = DEFAULT_MAX_DELAY;
        private double multiplier = 2.0;
        private Predicate<Exception> retryOn = e -> true;
        private long seed = System.nanoTime();

        private Builder() {}

        public Builder maxAttempts(int maxAttempts) {
            if (maxAttempts < 1) {
                throw new IllegalArgumentException("maxAttempts must be at least 1");
            }
            this.maxAttempts = maxAttempts;
            return this;
        }

        public Builder initialDelay(Duration initialDelay) {
            this.initialDelay = Objects.requireNonNull(initialDelay, "initialDelay");
            return this;
        }

        public Builder maxDelay(Duration maxDelay) {
            this.maxDelay = Objects.requireNonNull(maxDelay, "maxDelay");
            return this;
        }

        public Builder multiplier(double multiplier) {
            if (multiplier < 1.0) {
                throw new IllegalArgumentException("multiplier must be >= 1.0");
            }
            this.multiplier = multiplier;
            return this;
        }

        public Builder retryOn(Predicate<Exception> retryOn) {
            this.retryOn = Objects.requireNonNull(retryOn, "retryOn");
            return this;
        }

        public Builder seed(long seed) {
            this.seed = seed;
            return this;
        }

        public RetryPolicy build() {
            if (initialDelay.compareTo(maxDelay) > 0) {
                throw new IllegalStateException(
                        "initialDelay " + initialDelay + " exceeds maxDelay " + maxDelay);
            }
            return new RetryPolicy(this);
        }
    }
}
