use robusta_benchmark::{BenchmarkId, Criterion, criterion_group, criterion_main};
use robusta_lexer::tokenize;
use robusta_tasks_common::TestFiles;

fn bench_lexer(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("lexer");
    for file in TestFiles::minimal().files() {
        group.bench_function(BenchmarkId::from_parameter(file.file_name), |b| {
            b.iter(|| tokenize(file.source_text));
        });
    }
    group.finish();
}

criterion_group!(lexer, bench_lexer);
criterion_main!(lexer);
