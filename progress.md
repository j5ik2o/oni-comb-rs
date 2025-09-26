## 2025-09-26 21:29 JST
- 作業開始。PLAN.mdと現状コードを確認。

## 2025-09-26 21:39 JST
- ParseResultにflat_map/and_then/rest/state/lengthを追加しAPI拡充を開始。
- Parserにmap/flat_map/filter/attempt/many0/many1/runを実装。

## 2025-09-26 21:49 JST
- Parserのコンビネータ(map/flat_map/filter/attempt/many系)とPrelude関数を実装。
- ParseStateのCopy/Clone実装を調整し、結合時のコミット制御を整備。
- 結合テスト(combinators)とParseResult単体テストを追加し `cargo test` を実行。

