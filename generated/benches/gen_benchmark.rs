use criterion::{black_box, criterion_group, criterion_main, Criterion};
use doodle_gencode::api_helper::try_decode_gzip;

pub fn inflate_benchmark(c: &mut Criterion) {
    c.bench_function("gzip inflate @ test4.gz", |b| {
        b.iter(|| try_decode_gzip(black_box("test4.gz")).unwrap().len())
    });
}



criterion_group!(benches, inflate_benchmark);
criterion_main!(benches);
