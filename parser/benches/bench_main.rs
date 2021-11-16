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

use std::iter::FromIterator;

use criterion::*;

use crate::nom_json::nom_parse_json;
use crate::oni_comb_json::oni_comb_parse_json;
use crate::pom_json::pom_parse_json;

mod nom_json;
mod oni_comb_json;
mod pom_json;

fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("json");
  // let data = r#"{ "a" : 42, "b" : [ "x", "y", 12 ], "c": { "hello" : "world" } }"#;
  let data = r#"true"#;

  group.bench_with_input(BenchmarkId::new("nom", "bool"), data, |b, i| {
    b.iter(|| nom_parse_json(i))
  });
  group.bench_with_input(BenchmarkId::new("pom", "bool"), data, |b, i| {
    b.iter(|| pom_parse_json(i))
  });
  group.bench_with_input(BenchmarkId::new("oni-combi-rs", "bool"), data, |b, i| {
    b.iter(|| oni_comb_parse_json(i))
  });
  group.finish();
}

criterion_group!(benches, criterion_benchmark);

criterion_main! {
benches,
}
