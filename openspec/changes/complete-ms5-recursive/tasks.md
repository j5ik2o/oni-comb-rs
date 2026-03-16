# Tasks

## Phase 1: recursive() ヘルパー

- [ ] `combinator/recursive.rs`: `Recursive` 型 + `recursive()` 関数
- [ ] `combinator/mod.rs`: モジュール登録
- [ ] `prelude.rs`: `recursive` をエクスポート
- [ ] `tests/recursive.rs`: 基本テスト（括弧のネスト、Fail 伝播）

## Phase 2: 四則演算+括弧 統合テスト

- [ ] `tests/arithmetic.rs`: 四則演算パーサーの統合テスト
  - 単一整数、加減算、乗除算、括弧、ネストした括弧、空白処理
- [ ] COMMON.md, README.md のマイルストーン状態を更新

## 完了確認

- [ ] `cargo test -p oni-comb-parser` 全テスト通過
- [ ] `cargo bench --bench comparison -- --test` 全ベンチ正常動作
