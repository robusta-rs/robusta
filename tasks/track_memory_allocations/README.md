# Robusta allocation stats

This task keeps track of the system and arena allocations each pipeline stage makes, and the bytes they take. This is used for monitoring possible regressions in allocations and improvements in memory usage.

Update the snapshots with `cargo allocs`.

## Metrics

Recorded per file and per stage (parser, semantic, formatter):

- `file size` - size of the source text
- `sys allocs` / `sys reallocs` / `sys deallocs` - system allocator calls during the stage
- `sys alloc bytes` - bytes requested from the system allocator during the stage: allocation sizes plus reallocation growth
- `sys peak growth` - the most live heap memory during the stage, above what was live when it started; includes heap the stage frees again before it ends
- `arena allocs` / `arena reallocs` - arena allocator calls during the stage
- `arena size` - bytes used in the arena at the end of the stage. The arena is not reset between stages, so this includes the AST the parser built, which the formatter allocates beside.

Arena chunks are left out of every `sys` metric. Whether the arena needs another chunk depends on byte totals that differ between platforms, and a chunk reflects the arena's history more than the stage's own work. The arena marks its chunk allocations and deallocations so the tracking allocator can skip them.

## Cross-platform reproducibility

Allocation counts are the same on every platform: the task uses mimalloc as its global allocator and leaves arena chunks out of the counts. Byte metrics can still differ slightly, because some type layouts depend on the target, such as hashbrown's table control groups (16 bytes on x86_64, 8 on aarch64). `cargo allocs` reads the committed snapshot and keeps a byte metric's committed value when the measured one is within its tolerance: 1024 bytes for `arena size`, and 1%, at least 4 kB, for `sys alloc bytes` and `sys peak growth`. A snapshot made on one platform therefore passes CI on the others.
