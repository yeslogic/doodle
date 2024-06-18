use criterion::{black_box, criterion_group, criterion_main, Criterion};
use doodle_gencode::codegen_tests::gzip;

pub fn inflate_benchmark(c: &mut Criterion) {
    c.bench_function("gzip inflate @ test4.gz", |b| {
        b.iter(|| gzip::try_decode_gzip(black_box("test4.gz")))
    });
}

criterion_group!(benches, inflate_benchmark);
criterion_main!(benches);
