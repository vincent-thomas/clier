use clier_parser::Argv;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sort_arr_benchmark(c: &mut Criterion) {
  c.bench_function("Argv::from", |b| {
    b.iter(|| {
      Argv::from(black_box(
        [
          "command".to_string(),
          "--test=value".to_string(),
          "--name".to_string(),
          "test".to_string(),
          "--no-value".to_string(),
          "subcommand".to_string(),
        ]
        .as_slice(),
      ))
    })
  });
}

criterion_group!(benches, sort_arr_benchmark);
criterion_main!(benches);
