# oni-comb-rs

Rust 製パーサーコンビネータ群を再構築するプロジェクトです。`reboot` ブランチでは旧実装を一度リセットし、仕様ドキュメント (`API_SPEC.md`) を参照しながらコア機能を段階的に実装しています。

## 現在の進捗 (2025-09-26)

- `ParseResult` に `flat_map` / `and_then` / `rest` / `state` / `length` といった補助 API を追加し、結果操作を強化。
- `Parser` に以下のコンビネータを実装。
  - `map`, `flat_map`, `filter`, `attempt`
  - 多重適用系: `many0`, `many1`
- `prelude` から以下の関数を再公開。
  - `map`, `flat_map`, `filter`, `attempt`, `exists`, `not`, `skip_left`, `skip_right`, `surround`, `many0`, `many1`
- 追加プリミティブ: `byte`, `take_while1`, `separated_list1`, `separated_fold1`
- `parser/tests/combinators.rs` に統合テストを追加し、基本挙動とコミット制御を検証。

## 使い方

```bash
cargo test
```

上記で単体テストおよび統合テストが実行されます。`parser/tests/combinators.rs` にサンプルとなる `byte` パーサーが含まれているので、API の利用例として参照してください。

## ベンチマーク

`criterion` を利用して `nom` / `pom` と比較する CPU バウンドベンチマークを用意しています。

```bash
cargo bench --bench compare
```

入力サイズ 1,024 / 16,384 / 131,072 バイトの数字列に対する `sum_digits`、およびカンマ区切り整数列（要素数 256 / 4,096 / 32,768）に対する `csv_numbers` の 2 系列で、各ライブラリによるパース＋総和計算性能を比較します。結果は `target/criterion` 以下に HTML レポートとして出力されます。

## 次のステップ

- 文字列／トークン処理系ユーティリティ（`elm`, `seq`, `take_while` など）の実装
- `ParseError` のメッセージ体系ブラッシュアップとコミット制御 API の調整
- README でのクイックスタート追記（実際の DSL を構築できる段階になったタイミングで更新予定）

進捗は `progress.md` に 10 分間隔で記録しています。
