use robusta_allocator::Allocator;
use robusta_benchmark::{BenchmarkId, Criterion, criterion_group, criterion_main};
use robusta_formatter::Options;
use robusta_tasks_common::TestFiles;

fn bench_formatter(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("formatter");
    for file in TestFiles::minimal().files() {
        let options = Options::default();
        let mut allocator = Allocator::new();
        group.bench_function(BenchmarkId::from_parameter(file.file_name), |b| {
            b.iter_with_setup_wrapper(|runner| {
                allocator.reset();
                // Parse in setup so the benchmark isolates formatting from parsing.
                let parsed = robusta_parser::parse(&allocator, file.source_text);
                runner.run(|| {
                    robusta_formatter::format_compilation_unit(
                        &allocator,
                        file.source_text,
                        &parsed.unit,
                        &options,
                    )
                });
            });
        });
    }
    group.finish();
}

criterion_group!(formatter, bench_formatter);
criterion_main!(formatter);
