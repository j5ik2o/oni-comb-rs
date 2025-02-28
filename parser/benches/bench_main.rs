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
// use pprof::criterion::{Output, PProfProfiler};

mod nom_json;
mod oni_comb_json;
mod pom_json;

/// 異なる複雑さのJSONデータを用意
fn get_test_data() -> Vec<(&'static str, &'static str)> {
  vec![
    ("bool", r#"true"#),
    ("number", r#"42.5"#),
    ("string", r#""hello world""#),
    ("simple_array", r#"[1, 2, 3, 4, 5]"#),
    ("simple_object", r#"{"a": 1, "b": 2, "c": 3}"#),
    (
      "nested_object",
      r#"{ "a" : 42, "b" : [ "x", "y", 12 ], "c": { "hello" : "world" } }"#,
    ),
    (
      "complex",
      r#"{
        "id": "0001",
        "type": "donut",
        "name": "Cake",
        "ppu": 0.55,
        "batters": {
          "batter": [
            { "id": "1001", "type": "Regular" },
            { "id": "1002", "type": "Chocolate" },
            { "id": "1003", "type": "Blueberry" },
            { "id": "1004", "type": "Devil's Food" }
          ]
        },
        "topping": [
          { "id": "5001", "type": "None" },
          { "id": "5002", "type": "Glazed" },
          { "id": "5005", "type": "Sugar" },
          { "id": "5007", "type": "Powdered Sugar" },
          { "id": "5006", "type": "Chocolate with Sprinkles" },
          { "id": "5003", "type": "Chocolate" },
          { "id": "5004", "type": "Maple" }
        ]
      }"#,
    ),
  ]
}

fn criterion_benchmark(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("json");
  
  // 各テストデータに対してベンチマークを実行
  for (name, data) in get_test_data() {
    group.bench_with_input(BenchmarkId::new("oni-comb-rs", name), data, |b, i| {
      b.iter(|| oni_comb_parse_json(i))
    });
    
    group.bench_with_input(BenchmarkId::new("nom", name), data, |b, i| {
      b.iter(|| nom_parse_json(i))
    });
    
    group.bench_with_input(BenchmarkId::new("pom", name), data, |b, i| {
      b.iter(|| pom_parse_json(i))
    });
  }
  
  group.finish();
}

criterion_group! {
  name = benches;
  config = Criterion::default();
  targets = criterion_benchmark
}

criterion_main! {
  benches,
}
