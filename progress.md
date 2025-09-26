## 2025-09-26 21:29 JST
- 作業開始。PLAN.mdと現状コードを確認。

## 2025-09-26 21:39 JST
- ParseResultにflat_map/and_then/rest/state/lengthを追加しAPI拡充を開始。
- Parserにmap/flat_map/filter/attempt/many0/many1/runを実装。

## 2025-09-26 21:49 JST
- Parserのコンビネータ(map/flat_map/filter/attempt/many系)とPrelude関数を実装。
- ParseStateのCopy/Clone実装を調整し、結合時のコミット制御を整備。
- 結合テスト(combinators)とParseResult単体テストを追加し `cargo test` を実行。

## 2025-09-26 22:00 JST
- .gitignoreを追加し.idea/Cargo.lock/targetを無視設定。
- criterion/nom/pomをdev-depsに追加しベンチマーク雛形を作成。

## 2025-09-26 22:10 JST
- CSV形式の整数列を対象にした新規CPUバウンドベンチ(csv_numbers)を追加。
- oni-comb/nom/pomで比較計測を実施 (warm-up 1s, measure 2s)。
- sum_digitsベンチと合わせて `cargo bench --bench compare -- --warm-up-time 1 --measurement-time 2` を確認。

## 2025-09-26 22:31 JST
- take_while1/byte/separated_list1/separated_fold1 を追加し、ホットパス最適化の土台を実装。
- sum_digits ベンチで約350nsまで短縮し、nomとの差は約10%まで接近。
- CSVベンチは専用Parser復活＋fold系APIで維持し、nom比1.4〜2倍程度の優位を再確認。
- `cargo bench --bench compare -- --warm-up-time 1 --measurement-time 2` と `cargo test` を再実行。

## 2025-09-26 22:48 JST
- `elm`/`seq`/`take`/`take_while`/`take_while0` を追加し、基本トークン読み取り API を補強。
- `take_while1` を内部ヘルパー化して同系関数の実装を統一。
- プリミティブの挙動を確認するテストを拡充 (`elem_and_seq_primitives_work`, `take_and_take_while_variants_cover_cases`)。
- `cargo fmt` と `cargo test` を実行し、新規 API の整合性を確認。

## 2025-09-26 23:07 JST
- `peek`/`chain_left1`/`chain_right1`/`repeat`/`repeat_sep` を実装し、演算子結合と固定回数繰り返し系の API を補完。
- 右結合チェーン用の内部ヘルパー `chain_right1_internal` を導入し、再帰処理でコミット判定を整備。
- 新規コンビネータを対象にしたテスト (`chain_left1_applies_left_associative_operations` など) を追加。
- `cargo fmt` と `cargo test` を実行して全テスト21件を通過。

## 2025-09-26 23:28 JST
- `optional`/`choice`/`one_of`/`take_until`/`take_until1` を追加し、選択系 & 先読みユーティリティを強化。
- `skip_many0`/`skip_many1`/`separated_list0` を実装し、結果を保持しない繰り返しや空許容の区切りリストに対応。
- 追加 API のユニットテストを `parser/tests/combinators.rs` に拡充（テスト総数 26）。
- `cargo fmt` と `cargo test` を実行し、新規機能の整合性を確認。

## 2025-09-26 23:42 JST
- `Parser::or` / `Parser::or_else` を実装し、バックトラック可能な論理和評価を提供。
- `prelude` に `or` / `or_else` を再公開し、`choice` をメソッド合成ベースに刷新。
- 新規コンビネータのテスト (`or_combines_two_parsers` など) を追加し、テスト総数を 29 に拡充。
- `cargo fmt` / `cargo test` を実施し、機能の安定性を確認。

## 2025-09-26 23:58 JST
- `chain_left0` / `chain_right0` を追加し、オプション返却の零回許容チェーンを実装。
- 終端条件付き繰り返し (`many_till` / `skip_till`) を導入し、パース終了条件を柔軟化。
- 新テスト (`many_till_collects_until_terminator` など) を追加してテスト総数を32に拡張。
- `cargo fmt` と `cargo test` を実行し、全テスト成功を確認。

## 2025-09-27 00:15 JST
- `Parser` に `optional`/`skip_many*`/`many_till`/`skip_till` メソッドを追加し、メソッドチェーンで利用できるように改修。
- `prelude` の `optional`/`skip_many*`/`many_till`/`skip_till`/`chain_*0` をメソッド呼び出しベースへリファクタリングし、重複ロジックを削減。
- メソッド版 API を検証するテスト (`parser_optional_method_behaves_like_function` など) を追加し、総テスト件数を35に更新。
- `cargo fmt` / `cargo test` を実行し、新規 API の安定動作を確認。

## 2025-09-27 00:33 JST
- `Parser` に `map_err` / `expect` / `unwrap_or*` / `peek` / `or_list` を追加し、エラー処理や先読み、複数候補の結合をメソッドチェーンで記述可能に改善。
- `prelude` をリファクタリングして新メソッドを再公開、`choice` を `IntoIterator` 対応に拡張。
- `parser/tests/combinators.rs` に `map_err_changes_failure_message` や `parser_peek_method_preserves_input` などのテストを追加し、総テスト件数を42に拡充。
- `README.md` と `API_SPEC.md` を更新し、新 API とコード例 (`Parser::or_list` ほか) をドキュメント化。
- `cargo fmt` / `cargo test` を実行し、全テスト成功を確認。

## 2025-09-27 00:51 JST
- JSON ベンチマーク計画を PLAN.md / benchmarks/JSON_PLAN.md に整理し、測定方針と実装タスクを明文化。
- `parser/benches/json/` に `oni_comb.rs` / `nom.rs` / `pom.rs` / `mod.rs` を追加し、各ライブラリで `serde_json::Value` を返すパーサーを実装。
- `parser/benches/json.rs` でベンチマークハーネスを作成し、成功/失敗ケースで `oni-comb`・`nom`・`pom`・`serde_json` を比較できるように準備。
- ベンチ入力データ（`heavy.json`・失敗ケース）を追加し、`README.md` / `API_SPEC.md` に JSON ベンチ案内を追記。
- `cargo fmt` / `cargo test` を実行し、新規コードの整合性を確認。

## 2025-09-27 02:15 JST
- `nom` 版 JSON パーサーを参照実装ベースで整備し、`serde_json::Value` へ段階的に変換。`parser/tests/json_nom.rs` の `heavy`/`simple`/`number` テストを通過。
- `pom` 実装も combinator 指向に刷新し、同テスト群で成功。
- `cargo bench --bench json -- --warm-up-time 1 --measurement-time 1` を再実行し、`oni_comb` の `heavy.json` 成功ケースが ~129µs、失敗入力 (`missing_comma`/`unclosed_brace`) も ~2.6µs / ~6.0µs と `pom` を大きく上回る性能を確認。
- ベンチ結果の最新値を `benchmarks/JSON_PLAN.md` に反映し、警告削減のためベンチファイルへ `#![allow(dead_code)]` 等の調整を実施。

## 2025-09-27 03:06 JST
- `repeat` / `repeat_sep` を `ParseCursor` ベースに再実装し、ループ内の `Parser::run` 呼び出しと `Rc` クローンを削減。
- `parser/src/core/parse_cursor.rs` にドキュメントコメントを追加し、API の意図と利用方法を明文化。
- `parser/tests/combinators.rs` に零進行検出テスト (`repeat_detects_zero_length_inner_parser` / `repeat_sep_detects_zero_length_separator`) を追加し、境界ケースの退行を防止。
- `cargo fmt` / `cargo test -p oni-comb-parser --test combinators` を実行し、整形と 48 件のテスト成功を確認。

## 2025-09-27 03:22 JST
- `cargo bench --bench json -- --warm-up-time 1 --measurement-time 1` を実行し、`oni_comb` / `nom` / `pom` / `serde_json` の最新性能を取得。
- `benchmarks/JSON_PLAN.md` の測定結果を 2025-09-27 時点の値（`oni_comb` 成功ケース ~131 µs など）へ更新。
- `missing_comma` 失敗ケースで `oni_comb` が +約2% の遅延を示すものの、ノイズ範囲内と判断。

## 2025-09-27 03:36 JST
- ノイズ評価として `cargo bench --bench json -- --warm-up-time 1 --measurement-time 2` を実施し、統計的安定度を確認。
- 再計測結果を `benchmarks/JSON_PLAN.md` に追記（成功ケース中央値 131 µs、失敗ケースでも大きな変動なし）。
- `nom` の失敗ケースで +~3% の回帰警告が出たが、拡張測定により `oni_comb` の値は安定と確認。

## 2025-09-27 03:52 JST
- `CRITERION_SAMPLING_MODE=flat`（サンプルサイズ 200 指定は現状反映されず）で追加ベンチを実行し、`nom` 失敗ケースのブレが改善することを確認。
- `benchmarks/JSON_PLAN.md` に flat sampling の結果（`nom` missing_comma 約 0.74 µs など）と観察メモを追記。
- sample-size を増やすにはコード側で `Criterion::default().sample_size(200)` を設定する必要がある点を記録。

## 2025-09-27 04:12 JST
- `parser/benches/json.rs` をリファクタリングし、共同ヘルパー `GroupParams` / `configure_group` でベンチ構成を一元化。
- フル計測 (`json_success` / `json_failures`) は `sample_size=200`・`measurement_time=4s/3s`・`warm_up_time=1s`・`SamplingMode::Flat` に設定、クイック計測 (`json_success_quick` / `json_failures_quick`) は `sample_size=120`・`measurement_time=1.2s`・`warm_up_time=600ms`・`SamplingMode::Flat`。
- `cargo bench --bench json`, `cargo bench --bench json json_success_quick`, `cargo bench --bench json json_failures_quick` を実行し、用途別プリセットの挙動と外れ値率を確認。`benchmarks/JSON_PLAN.md` に結果と利用方法を追記。
