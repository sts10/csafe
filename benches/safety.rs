use criterion::{criterion_group, criterion_main, Criterion};

use csafe::*;
use std::path::PathBuf;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("FindUnsafe");
    let unsafe_word_list = make_set_from_file(&PathBuf::from(
        "./tests/test-files/agile_words_first_500.txt",
    ));

    // group.sample_size(10);
    group.bench_function("Using Fx Hash", |b| {
        b.iter(|| find_unsafe_word_contenders(&unsafe_word_list, false))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
