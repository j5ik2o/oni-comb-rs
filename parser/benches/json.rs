use std::time::Duration;

use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, SamplingMode,
};
use serde_json::Value;

mod json_impl;

use json_impl::{nom, oni_comb, pom, read_fail_fixture, read_fixture};

struct GroupParams {
    sample_size: usize,
    measurement_time: Duration,
    warm_up_time: Duration,
    sampling_mode: SamplingMode,
}

impl GroupParams {
    fn full_success() -> Self {
        Self {
            sample_size: 200,
            measurement_time: Duration::from_secs(4),
            warm_up_time: Duration::from_secs(1),
            sampling_mode: SamplingMode::Flat,
        }
    }

    fn full_failures() -> Self {
        Self {
            sample_size: 200,
            measurement_time: Duration::from_secs(3),
            warm_up_time: Duration::from_secs(1),
            sampling_mode: SamplingMode::Flat,
        }
    }

    fn quick() -> Self {
        Self {
            sample_size: 120,
            measurement_time: Duration::from_millis(1200),
            warm_up_time: Duration::from_millis(600),
            sampling_mode: SamplingMode::Flat,
        }
    }
}

fn configure_group(group: &mut BenchmarkGroup<'_, WallTime>, params: &GroupParams) {
    group.sample_size(params.sample_size);
    group.measurement_time(params.measurement_time);
    group.warm_up_time(params.warm_up_time);
    group.sampling_mode(params.sampling_mode);
}

fn bench_success_with(c: &mut Criterion, group_name: &'static str, params: GroupParams) {
    let mut group = c.benchmark_group(group_name);
    configure_group(&mut group, &params);
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

fn bench_failures_with(c: &mut Criterion, group_name: &'static str, params: GroupParams) {
    let mut group = c.benchmark_group(group_name);
    configure_group(&mut group, &params);
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

fn bench_success_full(c: &mut Criterion) {
    bench_success_with(c, "json_success", GroupParams::full_success());
}

fn bench_failures_full(c: &mut Criterion) {
    bench_failures_with(c, "json_failures", GroupParams::full_failures());
}

fn bench_success_quick(c: &mut Criterion) {
    bench_success_with(c, "json_success_quick", GroupParams::quick());
}

fn bench_failures_quick(c: &mut Criterion) {
    bench_failures_with(c, "json_failures_quick", GroupParams::quick());
}

criterion_group! {
    name = json_benches_full;
    config = Criterion::default();
    targets = bench_success_full, bench_failures_full
}

criterion_group! {
    name = json_benches_quick;
    config = Criterion::default();
    targets = bench_success_quick, bench_failures_quick
}

criterion_main!(json_benches_full, json_benches_quick);
