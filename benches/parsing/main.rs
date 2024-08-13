use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rusty_systems::parser::{parse_prod_string, parse_production};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Prod String len 4", |b| b.iter(|| parse_prod_string(black_box("A B C D"))));
    c.bench_function("Prod String len 10", |b| b.iter(|| parse_prod_string(black_box("A B C D D D E A G H"))));
    c.bench_function("Prod rule len 4", |b| b.iter(|| parse_production(black_box("A -> B C D E"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);