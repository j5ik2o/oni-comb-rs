# Rc クローン削減による性能改善計画

`oni-comb-parser-rs` の各コンビネータ実装では `Parser` が内部で `Rc<dyn Fn>` を多用しており、`Rc::clone` の参照カウンタ更新がベンチマーク時の大きなオーバーヘッドになっています。本ドキュメントでは、`Rc` の複製を減らしてパーサー実行時の間接参照コストを削るためのタスクを整理します。

## TODO
- [x] `parser/src/internal/parsers_impl.rs` の各コンビネータで `method.clone()` を廃止し、`Parser` をムーブして `parser.run(state)` を直接呼び出す。
- [x] `parser/src/internal/parsers_impl` 以下のサブモジュール（`operator_parsers_impl.rs`, `repeat_parsers_impl.rs`, `skip_parser_impl.rs` など）でも同様に `Parser` をムーブして `Rc::clone` を削減する。
- [x] `Rc` が本当に必要な箇所（キャッシュや循環参照目的など）を洗い出し、必要最小限に限定する。
- [x] 最適化後に `cargo +nightly bench -p oni-comb-parser-rs -- bench_main` および関連ベンチを実行し、改善効果を定量評価する。
- [x] ベンチ結果とコード変更内容を PR 説明にまとめ、リグレッションが無いことを共有する。
- [x] CPU バウンドな比較ベンチを追加し、大規模 JSON 入力で `oni-comb-rs`, `nom`, `pom` を再比較する（ウォームアップ・測定時間を延長し、ファイルIOをベンチ外へ隔離する）。

## 計測結果メモ
- 2025-09-26: `cargo +nightly bench -p oni-comb-parser-rs -- bench_main` を実行。gnuplot 非導入環境のため Plotters にフォールバックするが、全ベンチが正常終了。
- 同日、大規模 JSON 用の `json_large` グループを追加したベンチを再実行。`include_str!` で読み込んだ 3 種のサンプル（`large_array`, `mixed_payload`, `deep_nested`）に対し、各パーサーを繰り返し実行して CPU バウンドな計測を取得。
- 同日 `cargo +nightly test --workspace` を実行し、全てのクレート・ドクトテストがパスすることを確認。
- 今後 PR 作成時は上記コマンド結果と Rc クローン削減の要点（`ParserRunner::run` へ直接委譲、`Rc::clone` 削減、キャッシュキーの調整）を説明欄に転記する。
