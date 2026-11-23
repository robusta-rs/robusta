// Adapted from https://github.com/oxc-project/oxc/blob/566b11b2db6b664e787e67e2199d4ffcaa31c30a/crates/oxc_linter/src/timing.rs

use std::sync::Mutex;
use std::time::{Duration, Instant};

use rustc_hash::FxHashMap;

use crate::rule::RuleId;

/// One rule's accumulated cost: total time inside its `run`, and how many
/// nodes it was dispatched to.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RuleTimingStat {
    pub duration: Duration,
    pub calls: u64,
}

impl RuleTimingStat {
    fn add(&mut self, other: Self) {
        self.duration += other.duration;
        self.calls += other.calls;
    }

    /// Run `body`, adding its wall time and one call to this rule's totals.
    ///
    /// Two clock reads and two additions against a stack slot the caller
    /// already holds: this is the whole per-invocation cost of timing, and it
    /// exists only in the timed monomorphization of the dispatch loop.
    #[inline]
    pub(crate) fn time<F>(&mut self, body: F)
    where
        F: FnOnce(),
    {
        let start = Instant::now();
        body();
        self.duration += start.elapsed();
        self.calls += 1;
    }
}

/// One rule's totals, flattened for a consumer that reports them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleTimingRecord {
    pub rule: RuleId,
    pub duration: Duration,
    pub calls: u64,
}

impl RuleTimingRecord {
    const fn new(rule: RuleId, stat: RuleTimingStat) -> Self {
        Self { rule, duration: stat.duration, calls: stat.calls }
    }

    const fn split(self) -> (RuleId, RuleTimingStat) {
        (self.rule, RuleTimingStat { duration: self.duration, calls: self.calls })
    }
}

/// The run-wide destination every traversal folds its per-rule totals into.
///
/// The lock is taken once per traversal rather than once per rule invocation,
/// which is what lets files be linted in parallel without the store becoming
/// the thing being measured.
#[derive(Debug, Default)]
pub struct RuleTimingStore {
    timings: Mutex<FxHashMap<RuleId, RuleTimingStat>>,
}

impl RuleTimingStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add one traversal's totals to the run's.
    ///
    /// # Panics
    ///
    /// Panics if the store's mutex is poisoned.
    pub(crate) fn merge<I>(&self, records: I)
    where
        I: IntoIterator<Item = RuleTimingRecord>,
    {
        let mut records = records.into_iter().peekable();
        if records.peek().is_none() {
            return;
        }
        let mut timings = self.timings.lock().expect("rule timing store mutex poisoned");
        timings.reserve(records.size_hint().0);
        for record in records {
            let (rule, stat) = record.split();
            timings.entry(rule).or_default().add(stat);
        }
    }

    /// Every rule's totals, most expensive first.
    ///
    /// # Panics
    ///
    /// Panics if the store's mutex is poisoned.
    pub fn collect(&self) -> Vec<RuleTimingRecord> {
        let timings = self.timings.lock().expect("rule timing store mutex poisoned");
        let mut records = timings
            .iter()
            .map(|(&rule, &stat)| RuleTimingRecord::new(rule, stat))
            .collect::<Vec<_>>();
        records.sort_unstable_by(|left, right| {
            right
                .duration
                .cmp(&left.duration)
                .then_with(|| left.rule.as_str().cmp(right.rule.as_str()))
        });
        records
    }
}

/// One traversal's scratch space: a [`RuleTimingStat`] per enabled rule,
/// indexed by the rule's position in the enabled set.
///
/// Indexing by position is the point — the dispatch loop already has that
/// index, so recording a measurement is an array write rather than a hash
/// lookup, and the rule's identity is resolved once at merge time.
#[derive(Debug)]
pub struct RuleTimings {
    stats: Vec<RuleTimingStat>,
}

impl RuleTimings {
    pub fn new(rules: usize) -> Self {
        Self { stats: vec![RuleTimingStat::default(); rules] }
    }

    /// The slot for the rule at `index` in the enabled set.
    #[inline]
    pub fn stat(&mut self, index: usize) -> &mut RuleTimingStat {
        &mut self.stats[index]
    }

    /// Fold these totals into `store`, naming each rule from `rules` in the
    /// same order the slots were indexed.
    pub fn merge_into<I>(self, store: &RuleTimingStore, rules: I)
    where
        I: IntoIterator<Item = RuleId>,
    {
        store.merge(
            rules.into_iter().zip(self.stats).map(|(rule, stat)| RuleTimingRecord::new(rule, stat)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST: RuleId = RuleId::new("first");
    const SECOND: RuleId = RuleId::new("second");

    fn record(rule: RuleId, millis: u64, calls: u64) -> RuleTimingRecord {
        RuleTimingRecord::new(
            rule,
            RuleTimingStat { duration: Duration::from_millis(millis), calls },
        )
    }

    /// Every traversal that ran a rule adds to the same total, which is what
    /// makes a run's table describe the run rather than its last file.
    #[test]
    fn merging_accumulates_duration_and_calls_per_rule() {
        let store = RuleTimingStore::new();
        store.merge([record(FIRST, 3, 2), record(SECOND, 1, 1)]);
        store.merge([record(FIRST, 4, 5)]);

        let collected = store.collect();
        assert_eq!(
            collected,
            vec![record(FIRST, 7, 7), record(SECOND, 1, 1)],
            "totals accumulate and sort by descending duration",
        );
    }

    /// A rule that was enabled but never dispatched is still reported: "this
    /// rule cost nothing" is an answer, and silence is not.
    #[test]
    fn rules_that_never_ran_are_reported_with_zero_calls() {
        let store = RuleTimingStore::new();
        let mut timings = RuleTimings::new(2);
        timings.stat(0).time(|| {});
        timings.merge_into(&store, [FIRST, SECOND]);

        let collected = store.collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].rule, FIRST);
        assert_eq!(collected[0].calls, 1);
        assert_eq!(collected[1].rule, SECOND);
        assert_eq!(collected[1].calls, 0);
        assert_eq!(collected[1].duration, Duration::ZERO);
    }

    /// Ties are broken by name so the table is deterministic across runs, and
    /// an empty merge never touches the lock.
    #[test]
    fn equal_durations_sort_by_rule_and_empty_merges_are_skipped() {
        let store = RuleTimingStore::new();
        store.merge(std::iter::empty());
        assert_eq!(store.collect().len(), 0);

        store.merge([record(SECOND, 2, 1), record(FIRST, 2, 1)]);
        let collected = store.collect();
        assert_eq!(collected[0].rule, FIRST);
        assert_eq!(collected[1].rule, SECOND);
    }
}
