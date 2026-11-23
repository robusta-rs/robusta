package com.example.inventory;

import java.math.BigDecimal;
import java.math.RoundingMode;
import java.time.Clock;
import java.time.Duration;
import java.time.Instant;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.Comparator;
import java.util.Deque;
import java.util.EnumMap;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.NavigableMap;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;
import java.util.TreeMap;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;
import java.util.function.BiFunction;
import java.util.function.Function;
import java.util.function.Predicate;
import java.util.function.Supplier;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * A double-entry ledger for warehouse inventory movements.
 *
 * <p>Every movement is recorded twice: once against the source location and
 * once against the destination. The ledger never mutates a posted entry;
 * corrections are posted as compensating movements, which keeps the full
 * history reconstructable at any point in time.
 *
 * <p>The class is thread-safe. Posting is serialized per SKU while reads run
 * against immutable snapshots, so reporting queries never block writers.
 */
public final class InventoryLedger {

    /** The number of decimal places carried by monetary amounts. */
    public static final int MONEY_SCALE = 4;

    private static final BigDecimal ZERO = BigDecimal.ZERO.setScale(MONEY_SCALE, RoundingMode.UNNECESSARY);
    private static final Duration STALE_RESERVATION_AGE = Duration.ofHours(6);
    private static final int MAX_BATCH = 10_000;

    private final Clock clock;
    private final Map<String, Deque<Entry>> entriesBySku = new ConcurrentHashMap<>();
    private final Map<Location, AtomicLong> sequenceByLocation = new EnumMap<>(Location.class);
    private final NavigableMap<Instant, String> auditTrail = new TreeMap<>();
    private final List<LedgerListener> listeners = new ArrayList<>();

    public InventoryLedger(Clock clock) {
        this.clock = Objects.requireNonNull(clock, "clock");
        for (Location location : Location.values()) {
            sequenceByLocation.put(location, new AtomicLong());
        }
    }

    /** Physical or virtual places stock can live in. */
    public enum Location {
        RECEIVING("REC", true),
        PICK_FACE("PCK", true),
        BULK_RESERVE("BLK", true),
        QUALITY_HOLD("QAH", false),
        IN_TRANSIT("TRN", false),
        CUSTOMER("CUS", false);

        private final String code;
        private final boolean sellable;

        Location(String code, boolean sellable) {
            this.code = code;
            this.sellable = sellable;
        }

        public String code() {
            return code;
        }

        public boolean isSellable() {
            return sellable;
        }

        /** Parses the three-letter warehouse code used by the scanner fleet. */
        public static Location fromCode(String raw) {
            String normalized = raw == null ? "" : raw.trim().toUpperCase(Locale.ROOT);
            for (Location candidate : values()) {
                if (candidate.code.equals(normalized)) {
                    return candidate;
                }
            }
            throw new IllegalArgumentException("unknown location code: " + raw);
        }
    }

    /** The reason a movement happened, used for shrinkage reporting. */
    public enum MovementReason {
        PURCHASE_RECEIPT,
        SALE,
        TRANSFER,
        CYCLE_COUNT_ADJUSTMENT,
        DAMAGE_WRITE_OFF,
        RETURN_TO_VENDOR
    }

    /** One immutable ledger line. */
    public record Entry(
            long sequence,
            String sku,
            Location from,
            Location to,
            int quantity,
            BigDecimal unitCost,
            MovementReason reason,
            Instant postedAt,
            Optional<String> reference) {

        public Entry {
            Objects.requireNonNull(sku, "sku");
            Objects.requireNonNull(from, "from");
            Objects.requireNonNull(to, "to");
            Objects.requireNonNull(reason, "reason");
            Objects.requireNonNull(postedAt, "postedAt");
            Objects.requireNonNull(reference, "reference");
            if (quantity <= 0) {
                throw new IllegalArgumentException("quantity must be positive, got " + quantity);
            }
            if (from == to) {
                throw new IllegalArgumentException("movement must change location: " + from);
            }
            unitCost = unitCost.setScale(MONEY_SCALE, RoundingMode.HALF_EVEN);
        }

        public BigDecimal extendedCost() {
            return unitCost.multiply(BigDecimal.valueOf(quantity));
        }

        public boolean isCorrection() {
            return reason == MovementReason.CYCLE_COUNT_ADJUSTMENT
                    || reason == MovementReason.DAMAGE_WRITE_OFF;
        }
    }

    /** A point-in-time quantity for one SKU at one location. */
    public record StockLevel(String sku, Location location, long onHand, Instant asOf) {

        public boolean isNegative() {
            return onHand < 0;
        }
    }

    /** Callback interface for downstream systems that mirror the ledger. */
    @FunctionalInterface
    public interface LedgerListener {
        void onPosted(Entry entry);

        default LedgerListener filtered(Predicate<Entry> predicate) {
            return entry -> {
                if (predicate.test(entry)) {
                    onPosted(entry);
                }
            };
        }
    }

    /** Signals a posting that would drive sellable stock below zero. */
    public static class InsufficientStockException extends RuntimeException {

        private final String sku;
        private final long available;
        private final long requested;

        public InsufficientStockException(String sku, long available, long requested) {
            super("sku %s has %d on hand, requested %d".formatted(sku, available, requested));
            this.sku = sku;
            this.available = available;
            this.requested = requested;
        }

        public String sku() {
            return sku;
        }

        public long shortfall() {
            return requested - available;
        }
    }

    public void addListener(LedgerListener listener) {
        synchronized (listeners) {
            listeners.add(Objects.requireNonNull(listener, "listener"));
        }
    }

    /**
     * Posts one movement and returns the recorded entry.
     *
     * @throws InsufficientStockException when the source location would go
     *     negative and the reason is not an explicit adjustment
     */
    public Entry post(
            String sku,
            Location from,
            Location to,
            int quantity,
            BigDecimal unitCost,
            MovementReason reason,
            String reference) {
        Deque<Entry> entries = entriesBySku.computeIfAbsent(sku, ignored -> new ArrayDeque<>());
        Entry posted;
        synchronized (entries) {
            long available = onHand(entries, from);
            if (available < quantity && !isAdjustment(reason)) {
                throw new InsufficientStockException(sku, available, quantity);
            }
            long sequence = sequenceByLocation.get(from).incrementAndGet();
            posted = new Entry(
                    sequence,
                    sku,
                    from,
                    to,
                    quantity,
                    unitCost,
                    reason,
                    clock.instant(),
                    Optional.ofNullable(reference));
            entries.addLast(posted);
        }
        synchronized (auditTrail) {
            auditTrail.put(posted.postedAt(), sku + ":" + posted.sequence());
        }
        notifyListeners(posted);
        return posted;
    }

    /** Posts every movement in {@code batch}, stopping at the first failure. */
    public List<Entry> postAll(Collection<MovementRequest> batch) {
        if (batch.size() > MAX_BATCH) {
            throw new IllegalArgumentException(
                    "batch of " + batch.size() + " exceeds limit of " + MAX_BATCH);
        }
        List<Entry> results = new ArrayList<>(batch.size());
        for (MovementRequest request : batch) {
            results.add(post(
                    request.sku(),
                    request.from(),
                    request.to(),
                    request.quantity(),
                    request.unitCost(),
                    request.reason(),
                    request.reference()));
        }
        return Collections.unmodifiableList(results);
    }

    /** One requested movement in a batch posting. */
    public record MovementRequest(
            String sku,
            Location from,
            Location to,
            int quantity,
            BigDecimal unitCost,
            MovementReason reason,
            String reference) {}

    /** Returns the current on-hand quantity for {@code sku} at {@code location}. */
    public StockLevel stockLevel(String sku, Location location) {
        Deque<Entry> entries = entriesBySku.getOrDefault(sku, new ArrayDeque<>());
        long onHand;
        synchronized (entries) {
            onHand = onHand(entries, location);
        }
        return new StockLevel(sku, location, onHand, clock.instant());
    }

    /**
     * Computes weighted-average cost for a SKU across all sellable locations.
     *
     * <p>Write-offs are excluded so that damaged stock does not distort the
     * average carried by the sellable pool.
     */
    public BigDecimal weightedAverageCost(String sku) {
        Deque<Entry> entries = entriesBySku.getOrDefault(sku, new ArrayDeque<>());
        List<Entry> snapshot;
        synchronized (entries) {
            snapshot = List.copyOf(entries);
        }
        long totalUnits = 0;
        BigDecimal totalCost = ZERO;
        for (Entry entry : snapshot) {
            if (entry.reason() == MovementReason.DAMAGE_WRITE_OFF || !entry.to().isSellable()) {
                continue;
            }
            totalUnits += entry.quantity();
            totalCost = totalCost.add(entry.extendedCost());
        }
        return totalUnits == 0
                ? ZERO
                : totalCost.divide(BigDecimal.valueOf(totalUnits), MONEY_SCALE, RoundingMode.HALF_EVEN);
    }

    /** Aggregates shrinkage (write-offs and negative adjustments) per reason. */
    public Map<MovementReason, BigDecimal> shrinkageByReason(Instant since) {
        return entriesBySku.values().stream()
                .flatMap(entries -> {
                    synchronized (entries) {
                        return List.copyOf(entries).stream();
                    }
                })
                .filter(Entry::isCorrection)
                .filter(entry -> !entry.postedAt().isBefore(since))
                .collect(Collectors.groupingBy(
                        Entry::reason,
                        () -> new EnumMap<>(MovementReason.class),
                        Collectors.reducing(ZERO, Entry::extendedCost, BigDecimal::add)));
    }

    /**
     * Ranks the {@code limit} fastest-moving SKUs by units shipped to
     * customers inside the window.
     */
    public List<String> fastestMovers(Instant windowStart, int limit) {
        Map<String, Long> shippedBySku = new HashMap<>();
        entriesBySku.forEach((sku, entries) -> {
            long shipped;
            synchronized (entries) {
                shipped = entries.stream()
                        .filter(entry -> entry.to() == Location.CUSTOMER)
                        .filter(entry -> entry.postedAt().isAfter(windowStart))
                        .mapToLong(Entry::quantity)
                        .sum();
            }
            if (shipped > 0) {
                shippedBySku.put(sku, shipped);
            }
        });
        return shippedBySku.entrySet().stream()
                .sorted(Map.Entry.<String, Long>comparingByValue(Comparator.reverseOrder())
                        .thenComparing(Map.Entry.comparingByKey()))
                .limit(limit)
                .map(Map.Entry::getKey)
                .collect(Collectors.toList());
    }

    /** Renders a fixed-width reconciliation report for the given SKUs. */
    public String reconciliationReport(Set<String> skus) {
        StringBuilder report = new StringBuilder();
        report.append(String.format("%-16s %-14s %10s %14s%n", "SKU", "LOCATION", "ON HAND", "VALUE"));
        for (String sku : skus.stream().sorted().collect(Collectors.toList())) {
            BigDecimal averageCost = weightedAverageCost(sku);
            for (Location location : Location.values()) {
                StockLevel level = stockLevel(sku, location);
                if (level.onHand() == 0) {
                    continue;
                }
                BigDecimal value = averageCost.multiply(BigDecimal.valueOf(level.onHand()));
                report.append(String.format(
                        "%-16s %-14s %10d %14s%n",
                        sku,
                        location.code(),
                        level.onHand(),
                        value.setScale(2, RoundingMode.HALF_EVEN)));
            }
        }
        return report.toString();
    }

    /**
     * Classifies an arbitrary movement for the operations dashboard.
     *
     * <p>The mapping is intentionally coarse: the dashboard only distinguishes
     * inbound, outbound, and internal flows.
     */
    public static String classify(Entry entry) {
        return switch (entry.reason()) {
            case PURCHASE_RECEIPT -> "inbound";
            case SALE, RETURN_TO_VENDOR -> "outbound";
            case TRANSFER -> entry.to() == Location.IN_TRANSIT ? "outbound" : "internal";
            case CYCLE_COUNT_ADJUSTMENT, DAMAGE_WRITE_OFF -> {
                String direction = entry.to().isSellable() ? "up" : "down";
                yield "adjustment-" + direction;
            }
        };
    }

    /** Builds a memoizing lookup over any pure per-SKU computation. */
    public <R> Function<String, R> memoized(Function<String, R> computation) {
        Map<String, R> cache = new ConcurrentHashMap<>();
        return sku -> cache.computeIfAbsent(sku, computation);
    }

    /**
     * Replays the audit trail between two instants, feeding each recorded key
     * to {@code visitor} in chronological order.
     */
    public void replayAudit(Instant from, Instant to, BiFunction<Instant, String, Boolean> visitor) {
        Map<Instant, String> window;
        synchronized (auditTrail) {
            window = new LinkedHashMap<>(auditTrail.subMap(from, true, to, false));
        }
        for (Map.Entry<Instant, String> audit : window.entrySet()) {
            if (!visitor.apply(audit.getKey(), audit.getValue())) {
                break;
            }
        }
    }

    /** Drops reservations older than {@link #STALE_RESERVATION_AGE}. */
    public int expireStaleReservations(Supplier<Stream<MovementRequest>> reservations) {
        Instant cutoff = clock.instant().minus(STALE_RESERVATION_AGE);
        List<MovementRequest> expired = reservations.get()
                .filter(request -> request.to() == Location.QUALITY_HOLD)
                .collect(Collectors.toList());
        int released = 0;
        for (MovementRequest request : expired) {
            Deque<Entry> entries = entriesBySku.get(request.sku());
            if (entries == null) {
                continue;
            }
            synchronized (entries) {
                Entry last = entries.peekLast();
                if (last != null && last.postedAt().isBefore(cutoff)) {
                    released += request.quantity();
                }
            }
        }
        return released;
    }

    private static long onHand(Deque<Entry> entries, Location location) {
        long total = 0;
        for (Entry entry : entries) {
            if (entry.to() == location) {
                total += entry.quantity();
            }
            if (entry.from() == location) {
                total -= entry.quantity();
            }
        }
        return total;
    }

    private static boolean isAdjustment(MovementReason reason) {
        return reason == MovementReason.CYCLE_COUNT_ADJUSTMENT
                || reason == MovementReason.DAMAGE_WRITE_OFF;
    }

    private void notifyListeners(Entry entry) {
        List<LedgerListener> snapshot;
        synchronized (listeners) {
            snapshot = List.copyOf(listeners);
        }
        for (LedgerListener listener : snapshot) {
            try {
                listener.onPosted(entry);
            } catch (RuntimeException e) {
                // Listeners are best-effort mirrors; a failing mirror must not
                // poison the ledger write path.
            }
        }
    }

    @Override
    public String toString() {
        int skuCount = entriesBySku.size();
        long entryCount = entriesBySku.values().stream().mapToLong(Deque::size).sum();
        return "InventoryLedger{skus=" + skuCount + ", entries=" + entryCount + "}";
    }
}
