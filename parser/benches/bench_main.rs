// Copyright 2021-2025 Developers of the `oni-comb-rs` project.
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
use std::time::Duration;

use crate::nom_json::nom_parse_json;
use crate::oni_comb_json::{oni_comb_parse_json, oni_comb_parse_json_bytes};
use crate::pom_json::pom_parse_json;

mod nom_json;
mod oni_comb_json;
mod pom_json;

/// シンプルなJSONテスト用データ
fn get_simple_test_data() -> Vec<(&'static str, &'static str)> {
  vec![("bool", r#"true"#), ("string", r#""hello world""#)]
}

fn get_cpu_bound_test_data() -> Vec<(&'static str, &'static str, usize)> {
  vec![
    ("large_array", include_str!("data/large_array.json"), 5),
    ("mixed_payload", include_str!("data/mixed_payload.json"), 5),
    ("deep_nested", include_str!("data/deep_nested.json"), 20),
  ]
}

fn criterion_benchmark(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("json_small");

  // ベンチマークの実行時間を短縮するための設定
  group.sample_size(30);
  group.measurement_time(Duration::from_secs(1));

  // 各テストデータに対してベンチマークを実行
  for (name, data) in get_simple_test_data() {
    // oni-comb-rs パーサー (文字ベース) のベンチマーク
    group.bench_with_input(BenchmarkId::new("oni-comb-rs (char)", name), data, |b, i| {
      b.iter(|| oni_comb_parse_json(i))
    });

    // oni-comb-rs パーサー (バイトベース) のベンチマーク
    group.bench_with_input(BenchmarkId::new("oni-comb-rs (byte)", name), data, |b, i| {
      b.iter(|| oni_comb_parse_json_bytes(i))
    });

    // nom パーサーのベンチマーク
    group.bench_with_input(BenchmarkId::new("nom", name), data, |b, i| b.iter(|| nom_parse_json(i)));

    // pom パーサーのベンチマーク
    group.bench_with_input(BenchmarkId::new("pom", name), data, |b, i| b.iter(|| pom_parse_json(i)));
  }

  group.finish();

  // CPU バウンド向けの計測グループ
  let mut heavy_group = criterion.benchmark_group("json_large");
  heavy_group.sample_size(20);
  heavy_group.warm_up_time(Duration::from_secs(2));
  heavy_group.measurement_time(Duration::from_secs(5));

  for (name, data, repeat) in get_cpu_bound_test_data() {
    heavy_group.bench_with_input(BenchmarkId::new("oni-comb-rs (char)", name), data, |b, input| {
      b.iter(|| {
        for _ in 0..repeat {
          oni_comb_parse_json(input);
        }
      })
    });

    heavy_group.bench_with_input(BenchmarkId::new("oni-comb-rs (byte)", name), data, |b, input| {
      b.iter(|| {
        for _ in 0..repeat {
          oni_comb_parse_json_bytes(input);
        }
      })
    });

    heavy_group.bench_with_input(BenchmarkId::new("nom", name), data, |b, input| {
      b.iter(|| {
        for _ in 0..repeat {
          nom_parse_json(input);
        }
      })
    });

    heavy_group.bench_with_input(BenchmarkId::new("pom", name), data, |b, input| {
      b.iter(|| {
        for _ in 0..repeat {
          pom_parse_json(input);
        }
      })
    });
  }

  heavy_group.finish();
}

criterion_group! {
  name = benches;
  config = Criterion::default();
  targets = criterion_benchmark
}

criterion_main! {
  benches,
}
