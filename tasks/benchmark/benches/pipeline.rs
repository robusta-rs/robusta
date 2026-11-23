use robusta_allocator::AllocatorPool;
use robusta_benchmark::{BenchmarkId, Criterion, criterion_group, criterion_main};
use robusta_formatter::Options;
use robusta_tasks_common::{TestFiles, fixtures_dir};

/// Mirrors the product file reader so source-I/O costs remain visible next to
/// the parse-and-format pipeline.
fn read_source(path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

fn bench_pipeline(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("pipeline");
    for file in TestFiles::minimal().files() {
        let options = Options::default();
        let pool = AllocatorPool::new(1);
        group.bench_function(BenchmarkId::from_parameter(file.file_name), |b| {
            // Complete parse-and-format CPU pipeline: leased arena, parse
            // (including lexing), then format. Source reading is measured
            // separately below because filesystem latency is not CPU work.
            b.iter(|| {
                let allocator = pool.get();
                let parsed = robusta_parser::parse(&allocator, file.source_text);
                let unit = &parsed.unit;
                robusta_formatter::format_compilation_unit(
                    &allocator,
                    file.source_text,
                    unit,
                    &options,
                )
            });
        });
    }
    group.finish();
}

fn bench_source_read(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("source-read");
    for file in TestFiles::minimal().files() {
        let path = fixtures_dir().join(file.file_name);
        group.bench_function(BenchmarkId::from_parameter(file.file_name), |b| {
            b.iter(|| read_source(&path).expect("benchmark fixture should be readable"));
        });
    }
    group.finish();
}

fn bench_cli_pipeline(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("cli-pipeline");
    for file in TestFiles::minimal().files() {
        let path = fixtures_dir().join(file.file_name);
        let options = Options::default();
        let pool = AllocatorPool::new(1);
        group.bench_function(BenchmarkId::from_parameter(file.file_name), |b| {
            b.iter(|| {
                let source = read_source(&path).expect("benchmark fixture should be readable");
                let allocator = pool.get();
                let parsed = robusta_parser::parse(&allocator, &source);
                robusta_formatter::format_compilation_unit(
                    &allocator,
                    &source,
                    &parsed.unit,
                    &options,
                )
            });
        });
    }
    group.finish();
}

criterion_group!(pipeline, bench_pipeline, bench_source_read, bench_cli_pipeline);
criterion_main!(pipeline);
