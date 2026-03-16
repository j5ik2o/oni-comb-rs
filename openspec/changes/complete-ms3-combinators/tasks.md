# Tasks

## Phase 1: シーケンス (zip_left, zip_right, between)

- [ ] `combinator/zip_left.rs`: `ZipLeft<P1, P2>` 具象型 + `Parser` 実装
- [ ] `combinator/zip_right.rs`: `ZipRight<P1, P2>` 具象型 + `Parser` 実装
- [ ] `parser_ext.rs`: `.zip_left()`, `.zip_right()` メソッド追加
- [ ] `prelude.rs`: `between` 関数をエクスポート
- [ ] `tests/sequence.rs`: zip_left, zip_right, between のテスト追加
- [ ] ベンチマーク: zip_left/zip_right が zip+map と同等以上であることを確認

## Phase 2: 繰り返し (many1, sep_by0, sep_by1)

- [ ] `combinator/many1.rs`: `Many1<P>` 具象型 + `Parser` 実装
- [ ] `combinator/sep_by.rs`: `SepBy0<P, S>`, `SepBy1<P, S>` 具象型 + `Parser` 実装
- [ ] `parser_ext.rs`: `.many1()`, `.sep_by0()`, `.sep_by1()` メソッド追加
- [ ] `tests/repeat.rs`: many1, sep_by0, sep_by1 のテスト追加（空入力、1要素、複数要素、trailing separator 拒否、Cut 伝播）
- [ ] alloc_count に sep_by0 追加（Vec 以外の追加アロケーションがないことを確認）

## Phase 3: 二項演算子チェーン (chainl1, chainr1)

- [ ] `combinator/chainl1.rs`: `ChainL1<P, Op>` 具象型 + `Parser` 実装
- [ ] `combinator/chainr1.rs`: `ChainR1<P, Op>` 具象型 + `Parser` 実装
- [ ] `parser_ext.rs`: `.chainl1()`, `.chainr1()` メソッド追加
- [ ] `tests/chain.rs`: chainl1, chainr1 のテスト追加（結合方向の検証、Cut 伝播）
- [ ] 四則演算パーサーの統合テスト

## 完了確認

- [ ] `cargo test -p oni-comb-parser` 全テスト通過
- [ ] `cargo bench --bench comparison -- --test` 全ベンチ正常動作
- [ ] `cargo bench --bench alloc_count` 追加アロケーション確認
- [ ] COMMON.md, README.md のマイルストーン状態を更新
