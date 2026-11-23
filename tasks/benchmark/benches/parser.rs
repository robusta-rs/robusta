use robusta_allocator::Allocator;
use robusta_benchmark::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use robusta_tasks_common::TestFiles;

fn bench_parser(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("parser");
    for file in TestFiles::minimal().files() {
        group.bench_function(BenchmarkId::from_parameter(file.file_name), |b| {
            // Do not include initializing the arena in the benchmark: the
            // product reuses one per worker across files, so do the same here.
            let mut allocator = Allocator::new();
            b.iter(|| {
                black_box(robusta_parser::parse(&allocator, file.source_text).status);
                allocator.reset();
            });
        });
    }
    group.finish();
}

criterion_group!(parser, bench_parser);
criterion_main!(parser);
