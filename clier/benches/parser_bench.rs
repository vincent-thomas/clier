use clier::Clier;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sort_arr_benchmark(c: &mut Criterion) {
  c.bench_function("Only Parsing", |b| {
    b.iter(|| {
      Clier::with_args(black_box(&[
        "command".to_string(),
        "--test=value".to_string(),
        "--name".to_string(),
        "test".to_string(),
        "--no-value".to_string(),
        "subcommand".to_string(),
      ]))
    })
  });
}

criterion_group!(benches, sort_arr_benchmark);
criterion_main!(benches);
