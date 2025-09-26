# Rc クローン削減による性能改善計画

`oni-comb-parser-rs` の各コンビネータ実装では `Parser` が内部で `Rc<dyn Fn>` を多用しており、`Rc::clone` の参照カウンタ更新がベンチマーク時の大きなオーバーヘッドになっています。本ドキュメントでは、`Rc` の複製を減らしてパーサー実行時の間接参照コストを削るためのタスクを整理します。

## TODO
- [ ] `parser/src/internal/parsers_impl.rs` の各コンビネータで `method.clone()` を廃止し、`Parser` をムーブして `parser.run(state)` を直接呼び出す。
- [ ] `parser/src/internal/parsers_impl` 以下のサブモジュール（`operator_parsers_impl.rs`, `repeat_parsers_impl.rs`, `skip_parser_impl.rs` など）でも同様に `Parser` をムーブして `Rc::clone` を削減する。
- [ ] `Rc` が本当に必要な箇所（キャッシュや循環参照目的など）を洗い出し、必要最小限に限定する。
- [ ] 最適化後に `cargo +nightly bench -p oni-comb-parser-rs -- bench_main` および関連ベンチを実行し、改善効果を定量評価する。
- [ ] ベンチ結果とコード変更内容を PR 説明にまとめ、リグレッションが無いことを共有する。

## 補足
- `Parser::clone` 自体は API として残しつつ、使用箇所を最小化します。
- 変更後の差分は `Rc` から `Parser` 本体へのムーブが中心となるため、コンパイラのライフタイム推論に注意が必要です。
- ベンチ強化のため、`oni_comb_json` など個別ベンチの実行も推奨します。
