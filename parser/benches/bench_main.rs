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

use crate::nom_json::nom_parse_json;
use crate::oni_comb_json::oni_comb_parse_json;
use crate::pom_json::pom_parse_json;
use pprof::criterion::{Output, PProfProfiler};

mod nom_json;
mod oni_comb_json;
mod pom_json;

fn criterion_benchmark(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("json");
  // let data = r#"{ "a" : 42, "b" : [ "x", "y", 12 ], "c": { "hello" : "world" } }"#;
  let data = r#"true"#;

  group.bench_function(BenchmarkId::new("oni-comb-rs", "bool"), |b| {
    b.iter(|| oni_comb_parse_json(data))
  });

  // group.bench_with_input(BenchmarkId::new("nom", "bool"), data, |b, i| {
  //   b.iter(|| nom_parse_json(i))
  // });
  // group.bench_with_input(BenchmarkId::new("pom", "bool"), data, |b, i| {
  //   b.iter(|| pom_parse_json(i))
  // });
  // group.bench_with_input(BenchmarkId::new("oni-comb-rs", "bool"), data, |b, i| {
  //   b.iter(|| oni_comb_parse_json(i))
  // });
  group.finish();
}

criterion_group! {
name = benches;
config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
targets = criterion_benchmark
}

// criterion_group! {
//   benches,
//   criterion_benchmark
// }

criterion_main! {
benches,
}
