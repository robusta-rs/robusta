use robusta_allocator::Allocator;
use robusta_benchmark::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use robusta_semantic::SemanticBuilder;
use robusta_tasks_common::TestFiles;

fn bench_semantic(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("semantic");
    for file in TestFiles::minimal().files() {
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, file.source_text);
        let unit = &parsed.unit;
        group.bench_function(BenchmarkId::from_parameter(file.file_name), |b| {
            b.iter(|| SemanticBuilder::new().build(black_box(file.source_text), black_box(unit)));
        });
    }
    group.finish();
}

criterion_group!(semantic, bench_semantic);
criterion_main!(semantic);
