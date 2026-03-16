use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use crate::impls;

const IDENTIFIERS: &[&str] = &["x", "foo", "foo_bar_123", "_private", "longIdentifierNameForTesting"];
const INTEGERS: &[&str] = &["0", "42", "9999999", "18446744073709551615"];

pub fn register(c: &mut Criterion) {
    identifier_bench(c);
    integer_bench(c);
}

fn identifier_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("token/identifier");

    for input in IDENTIFIERS {
        group.throughput(Throughput::Bytes(input.len() as u64));

        group.bench_with_input(BenchmarkId::new("oni-comb", input), input, |b, input| {
            b.iter(|| impls::oni_comb::parse_identifier(black_box(input)))
        });
        group.bench_with_input(BenchmarkId::new("winnow", input), input, |b, input| {
            b.iter(|| impls::winnow_impl::parse_identifier(black_box(input)))
        });
        group.bench_with_input(BenchmarkId::new("nom", input), input, |b, input| {
            b.iter(|| impls::nom_impl::parse_identifier(black_box(input)))
        });
        group.bench_with_input(BenchmarkId::new("chumsky", input), input, |b, input| {
            b.iter(|| impls::chumsky_impl::parse_identifier(black_box(input)))
        });
        group.bench_with_input(BenchmarkId::new("pom", input), input, |b, input| {
            b.iter(|| impls::pom_impl::parse_identifier(black_box(input)))
        });
    }

    group.finish();
}

fn integer_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("token/integer");

    for input in INTEGERS {
        group.throughput(Throughput::Bytes(input.len() as u64));

        group.bench_with_input(BenchmarkId::new("oni-comb", input), input, |b, input| {
            b.iter(|| impls::oni_comb::parse_integer(black_box(input)))
        });
        group.bench_with_input(BenchmarkId::new("winnow", input), input, |b, input| {
            b.iter(|| impls::winnow_impl::parse_integer(black_box(input)))
        });
        group.bench_with_input(BenchmarkId::new("nom", input), input, |b, input| {
            b.iter(|| impls::nom_impl::parse_integer(black_box(input)))
        });
        group.bench_with_input(BenchmarkId::new("chumsky", input), input, |b, input| {
            b.iter(|| impls::chumsky_impl::parse_integer(black_box(input)))
        });
        group.bench_with_input(BenchmarkId::new("pom", input), input, |b, input| {
            b.iter(|| impls::pom_impl::parse_integer(black_box(input)))
        });
    }

    group.finish();
}
