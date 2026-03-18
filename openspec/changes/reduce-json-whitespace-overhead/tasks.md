## 1. Full JSON parser whitespace refactor

- [ ] 1.1 `modules/parser/benches/json_full.rs` の oni-comb JSON 実装を、grammar boundary ごとに空白を一度だけ消費する構造へ整理する
- [ ] 1.2 oni-comb 側 full JSON パーサーを共通化し、`modules/parser/benches/alloc_count.rs` から同じ実装を使うように切り替える
- [ ] 1.3 whitespace を含む array/object/member 入力で、full JSON ベンチと allocation-count パーサーが同じ JSON ノード構造を受理することを確認する

## 2. JSON subset benchmark cleanup

- [ ] 2.1 `modules/parser/benches/workloads/json.rs` の `ws()` 依存構成を見直し、value / comma / colon / delimiter ごとの helper に整理する
- [ ] 2.2 JSON subset ベンチで、空白入り入力と compact 入力の両方が従来どおり成功することを確認する

## 3. Measurement and documentation

- [ ] 3.1 `cargo bench -p oni-comb-parser --bench comparison -- json` と `cargo bench -p oni-comb-parser --bench json_full` を実行し、fixed cost 改善の有無を記録する
- [ ] 3.2 `cargo bench -p oni-comb-parser --bench alloc_count` を実行し、allocation profile が意図どおり維持されることを確認する
- [ ] 3.3 `modules/parser/benches/README.md` と `modules/parser/benches/README.ja.md` を更新し、空白処理最適化の結果と分析を反映する
