// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]

use criterion::*;
use std::iter::FromIterator;

fn oni_comb_hello_world(s: &str) {
  use oni_comb_parser_rs::prelude::*;
  let input = s.as_bytes();

  let parser: Parser<u8, &str> = surround(
    elm_ref(b'\''),
    (seq(b"hello") + elm_space() + seq(b"world")).collect(),
    elm_ref(b'\'') + elm_ref(b';'),
  )
  .map_res(std::str::from_utf8);

  let _ = parser.parse(input).to_result().unwrap();
}

fn pom_hello_world(s: &str) {
  use pom::parser::*;
  let parser =
    (sym(b'\'') * (seq(b"hello") * one_of(b" \t") + seq(b"world")).collect() - seq(b"';")).convert(std::str::from_utf8);
  let _ = parser.parse(s.as_bytes()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("hello_world");
  let op = 0u8;
  let data = "'hello world';";
  group.bench_with_input(BenchmarkId::new("oni-combi-rs", op), &op, |b, i| {
    b.iter(|| oni_comb_hello_world(data))
  });
  group.bench_with_input(BenchmarkId::new("pom", op), &op, |b, i| {
    b.iter(|| pom_hello_world(data))
  });
  group.finish();
}

criterion_group!(benches, criterion_benchmark);

criterion_main! {
benches,
}
