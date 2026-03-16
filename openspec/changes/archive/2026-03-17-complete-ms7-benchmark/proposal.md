## Why

MS7 (Benchmark) の完了条件は「v1 比較でボトルネック定量化、1回最適化サイクル完了」。MS6 で `format!` ベースの `String` エラーを構造化 `ParseError` に置き換えたが、再計測していない。また JSON subset と expression parser のベンチワークロードがスタブのまま。

## What Changes

1. JSON subset ベンチを workloads/json.rs に追加（oni-comb のみ、1段ネスト）
2. Expression parser ベンチを workloads/arithmetic.rs に追加（oni-comb のみ）
3. ParseError 導入後の全ベンチ再計測
4. benches/README.md を新しい数値と考察で更新
5. 最適化サイクルの記録: format! 排除による効果の定量化

## Capabilities

### New Capabilities
- `json-bench`: JSON subset パースのベンチマーク
- `arithmetic-bench`: 四則演算式パースのベンチマーク

### Modified Capabilities
- benches/README.md の結果更新

## Impact

- `modules/parser/benches/workloads/json.rs`: スタブ → 実装
- `modules/parser/benches/workloads/arithmetic.rs`: スタブ → 実装
- `modules/parser/benches/README.md`: 再計測結果で更新
- COMMON.md, README.md: MS7 完了に更新
