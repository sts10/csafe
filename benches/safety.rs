use criterion::{criterion_group, criterion_main, Criterion};
use csafe::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("FindUnsafe");
    let unsafe_word_list = make_vec_from_file("./tests/test-files/agile_words_letter_a.txt");
    // let safe_word_list = make_vec_from_file("./tests/test-files/agile_words_letter_a.txt.csafe");

    group.sample_size(10);
    group.bench_function("As implemented", |b| {
        b.iter(|| find_unsafe_words(&unsafe_word_list))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
