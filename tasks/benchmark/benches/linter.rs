use robusta_allocator::Allocator;
use robusta_benchmark::{BenchmarkId, Criterion, criterion_group, criterion_main};
use robusta_diagnostics::Severity;
use robusta_linter::{Linter, RuleConfiguration, rules};
use robusta_semantic::SemanticBuilder;
use robusta_tasks_common::TestFiles;

fn bench_linter(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("linter");
    for file in TestFiles::minimal().files() {
        // Parse and bind outside the measurement so the benchmark isolates
        // rule execution over every registered rule.
        let allocator = Allocator::new();
        let parsed = robusta_parser::parse(&allocator, file.source_text);
        let unit = &parsed.unit;
        let semantic = SemanticBuilder::new().build(file.source_text, unit);
        let linter = Linter::new(
            rules::builtin_rules()
                .iter()
                .map(|rule| RuleConfiguration::new(*rule, Severity::Warning)),
        );
        group.bench_function(BenchmarkId::from_parameter(file.file_name), |b| {
            b.iter(|| linter.lint(&semantic));
        });
    }
    group.finish();
}

criterion_group!(linter, bench_linter);
criterion_main!(linter);
