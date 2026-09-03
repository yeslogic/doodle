use criterion::{Criterion, black_box, criterion_group, criterion_main};
use doodle::{FormatModule, codegen::generate_code};

pub fn codegen_run_benchmark(c: &mut Criterion) {
    let mut module = FormatModule::new();
    let format = doodle_formats::format::main(&mut module).call();
    c.bench_function("cg-run (no I/O)", |b| {
        b.iter(|| black_box(generate_code(&module, &format)))
    });
}

criterion_group!(benches, codegen_run_benchmark);
criterion_main!(benches);
