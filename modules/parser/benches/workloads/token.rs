use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput};

use crate::impls;

const IDENTIFIERS: &[&str] = &["x", "foo", "foo_bar_123", "_private", "longIdentifierNameForTesting"];
const INTEGERS: &[&str] = &["0", "42", "9999999", "18446744073709551615"];
const FLAT_MAP_SAME_TYPE: &[&str] = &["1one", "2two", "3three"];
const FLAT_MAP_BOXED: &[&str] = &["c:hello", "i:42"];

pub fn register(c: &mut Criterion) {
  identifier_bench(c);
  integer_bench(c);
  flat_map_same_type_bench(c);
  flat_map_boxed_bench(c);
  zip_vs_flat_map_bench(c);
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

fn flat_map_same_type_bench(c: &mut Criterion) {
  let mut group = c.benchmark_group("token/flat_map_same_type");

  for input in FLAT_MAP_SAME_TYPE {
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_with_input(BenchmarkId::new("oni-comb", input), input, |b, input| {
      b.iter(|| impls::oni_comb::parse_flat_map_same_type(black_box(input)))
    });
    group.bench_with_input(BenchmarkId::new("winnow", input), input, |b, input| {
      b.iter(|| impls::winnow_impl::parse_flat_map_same_type(black_box(input)))
    });
    group.bench_with_input(BenchmarkId::new("nom", input), input, |b, input| {
      b.iter(|| impls::nom_impl::parse_flat_map_same_type(black_box(input)))
    });
    group.bench_with_input(BenchmarkId::new("chumsky", input), input, |b, input| {
      b.iter(|| impls::chumsky_impl::parse_flat_map_same_type(black_box(input)))
    });
    group.bench_with_input(BenchmarkId::new("pom", input), input, |b, input| {
      b.iter(|| impls::pom_impl::parse_flat_map_same_type(black_box(input)))
    });
  }

  group.finish();
}

fn flat_map_boxed_bench(c: &mut Criterion) {
  let mut group = c.benchmark_group("token/flat_map_boxed");

  for input in FLAT_MAP_BOXED {
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_with_input(BenchmarkId::new("oni-comb", input), input, |b, input| {
      b.iter(|| impls::oni_comb::parse_flat_map_boxed(black_box(input)))
    });
    group.bench_with_input(BenchmarkId::new("winnow", input), input, |b, input| {
      b.iter(|| impls::winnow_impl::parse_flat_map_boxed(black_box(input)))
    });
    group.bench_with_input(BenchmarkId::new("nom", input), input, |b, input| {
      b.iter(|| impls::nom_impl::parse_flat_map_boxed(black_box(input)))
    });
    group.bench_with_input(BenchmarkId::new("chumsky", input), input, |b, input| {
      b.iter(|| impls::chumsky_impl::parse_flat_map_boxed(black_box(input)))
    });
    group.bench_with_input(BenchmarkId::new("pom", input), input, |b, input| {
      b.iter(|| impls::pom_impl::parse_flat_map_boxed(black_box(input)))
    });
  }

  group.finish();
}

fn zip_vs_flat_map_bench(c: &mut Criterion) {
  let mut group = c.benchmark_group("token/zip_vs_flat_map");

  for input in IDENTIFIERS {
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_with_input(BenchmarkId::new("zip", input), input, |b, input| {
      b.iter(|| impls::oni_comb::parse_identifier_zip(black_box(input)))
    });
    group.bench_with_input(BenchmarkId::new("flat_map", input), input, |b, input| {
      b.iter(|| impls::oni_comb::parse_identifier_flat_map(black_box(input)))
    });
  }

  group.finish();
}
