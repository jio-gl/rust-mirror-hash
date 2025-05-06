use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mirror_hash::Mirror256;
use rand::{Rng, thread_rng};

fn random_alphanumeric_string(length: usize) -> String {
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut rng = thread_rng();
    
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

fn hash_short_string(c: &mut Criterion) {
    let input = "This is a short test message.";
    c.bench_function("hash short string", |b| {
        b.iter(|| {
            let hasher = Mirror256::new(Some(black_box(input)), None, None, true);
            black_box(hasher.hexdigest())
        })
    });
}

fn hash_medium_string(c: &mut Criterion) {
    let input = "This is a medium length test message with some additional content to make it longer.";
    c.bench_function("hash medium string", |b| {
        b.iter(|| {
            let hasher = Mirror256::new(Some(black_box(input)), None, None, true);
            black_box(hasher.hexdigest())
        })
    });
}

fn hash_long_string(c: &mut Criterion) {
    let input = random_alphanumeric_string(1024);
    c.bench_function("hash long string (1KB)", |b| {
        b.iter(|| {
            let hasher = Mirror256::new(Some(black_box(&input)), None, None, true);
            black_box(hasher.hexdigest())
        })
    });
}

fn hash_empty_string(c: &mut Criterion) {
    c.bench_function("hash empty string", |b| {
        b.iter(|| {
            let hasher = Mirror256::new(Some(black_box("")), None, None, true);
            black_box(hasher.hexdigest())
        })
    });
}

fn hash_update_multiple(c: &mut Criterion) {
    c.bench_function("hash update multiple", |b| {
        b.iter(|| {
            let mut hasher = Mirror256::new(None, None, None, true);
            hasher.update(black_box("part1"));
            hasher.update(black_box("part2"));
            hasher.update(black_box("part3"));
            black_box(hasher.hexdigest())
        })
    });
}

criterion_group!(
    benches,
    hash_short_string,
    hash_medium_string,
    hash_long_string,
    hash_empty_string,
    hash_update_multiple
);
criterion_main!(benches); 