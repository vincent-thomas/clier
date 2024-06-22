use clier_parser::Argv;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sort_arr_benchmark(c: &mut Criterion) {
  c.bench_function("Argv::from", |b| {
    let shit = &["command", "--test=value", "--name", "test", "--no-value", "subcommand"][..];
    b.iter(|| Argv::from(black_box(shit)))
  });
}

criterion_group!(benches, sort_arr_benchmark);
criterion_main!(benches);
