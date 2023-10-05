use clier_parser::Argv;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sort_arr_benchmark(c: &mut Criterion) {
  c.bench_function("Argv::from", |b| {
    let shit = &[
      "command".to_string(),
      "--test=value".to_string(),
      "--name".to_string(),
      "test".to_string(),
      "--no-value".to_string(),
      "subcommand".to_string(),
    ][..];
    b.iter(|| Argv::from(black_box(shit)))
  });
}

criterion_group!(benches, sort_arr_benchmark);
criterion_main!(benches);
