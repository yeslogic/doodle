use criterion::{black_box, criterion_group, criterion_main, Criterion};
use doodle::{
    decoder::{Compiler, Program, Value},
    read::ReadCtxt,
    FormatModule,
};
use doodle_formats::format;
use lazy_static::lazy_static;

// amortize the cost of constructing the program to avoid overhead in the inflate profile
lazy_static! {
    static ref PROGRAM: Program = {
        let mut module = FormatModule::new();
        let format = format::main(&mut module).call();
        let program = Compiler::compile_program(&module, &format).unwrap();
        program
    };
}

fn run_decoder(f: &str) -> Value {
    let input = std::fs::read(f).unwrap();
    let value = match PROGRAM.run(ReadCtxt::new(&input)) {
        Ok((value, _)) => value,
        Err(_) => unreachable!(),
    };
    value
}

pub fn inflate_benchmark(c: &mut Criterion) {
    c.bench_function("test4.gz interpreted", |b| {
        b.iter(|| black_box(run_decoder("../test4.gz")))
    });
}

criterion_group!(benches, inflate_benchmark);
criterion_main!(benches);
