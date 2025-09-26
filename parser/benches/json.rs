use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use serde_json::Value;

mod json_impl;

use json_impl::{nom, oni_comb, pom, read_fail_fixture, read_fixture};

fn bench_success(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_success");
    let fixture = read_fixture("heavy");

    group.bench_function(BenchmarkId::new("oni_comb", "heavy"), |b| {
        b.iter(|| oni_comb::parse_json_value(fixture).unwrap())
    });

    group.bench_function(BenchmarkId::new("nom", "heavy"), |b| {
        b.iter(|| nom::parse_json_value(fixture).unwrap())
    });

    group.bench_function(BenchmarkId::new("pom", "heavy"), |b| {
        b.iter(|| pom::parse_json_value(fixture).unwrap())
    });

    group.bench_function(BenchmarkId::new("serde_json", "heavy"), |b| {
        b.iter(|| serde_json::from_slice::<Value>(fixture).unwrap())
    });

    group.finish();
}

fn bench_failures(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_failures");
    let fixtures = ["missing_comma", "unclosed_brace"];

    for name in fixtures {
        let input = read_fail_fixture(name);
        group.bench_with_input(BenchmarkId::new("oni_comb", name), &input, |b, data| {
            b.iter(|| oni_comb::parse_json_value(data).unwrap_err())
        });
        group.bench_with_input(BenchmarkId::new("nom", name), &input, |b, data| {
            b.iter(|| nom::parse_json_value(data).unwrap_err())
        });
        group.bench_with_input(BenchmarkId::new("pom", name), &input, |b, data| {
            b.iter(|| pom::parse_json_value(data).unwrap_err())
        });
        group.bench_with_input(BenchmarkId::new("serde_json", name), &input, |b, data| {
            b.iter(|| serde_json::from_slice::<Value>(data).unwrap_err())
        });
    }

    group.finish();
}

criterion_group!(json_benches, bench_success, bench_failures);
criterion_main!(json_benches);
