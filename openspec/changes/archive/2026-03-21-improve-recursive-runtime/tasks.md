## 1. Runtime Design Validation

- [x] 1.1 現行 `recursive()` の API、`Clone` 利用パターン、既存 recursive grammar（JSON / YAML / arithmetic）の成立条件を固定する
- [x] 1.2 owner handle と non-owning self-reference handle を分離する内部表現を設計し、強参照サイクルを作らない lifetime 条件を明文化する
- [x] 1.3 typed storage + thunk dispatch により steady-state から `Box<dyn Parser>` と `Option` unwrap を外す runtime shape を確定する

## 2. Recursive Runtime Refactor

- [x] 2.1 `modules/parser/src/combinator/recursive.rs` を owner/ref 分離構造へ置き換える
- [x] 2.2 concrete parser storage と `parse_fn` / `drop_fn` thunk を実装し、public API を変えずに steady-state dispatch を切り替える
- [x] 2.3 構築時のみ未初期化状態を閉じ込め、steady-state parse path に初期化チェックが残らないようにする

## 3. Semantic Regression Coverage

- [x] 3.1 `modules/parser/tests/recursive.rs` を更新し、root owner clone、graph 内 clone、nested recursion、failure propagation を回帰確認する
- [x] 3.2 `modules/parser/tests/arithmetic.rs` と recursive 利用 downstream parser が API 変更なしで動作することを確認する
- [x] 3.3 必要なら JSON / YAML の recursive grammar に対して compile/runtime 回帰テストを追加する

## 4. Benchmark and Documentation

- [x] 4.1 recursive-heavy workload（arithmetic、必要なら JSON benchmark）を再計測し、`recursive()` の改善効果を確認する
- [x] 4.2 `modules/parser/benches/README.md` などのボトルネック説明を更新し、`recursive()` 改善後の制約と残課題を記録する
