# Tasks

## Phase 1: 単純な便利関数

- [ ] `text/whitespace.rs`: `whitespace0()`, `whitespace1()`
- [ ] `text/identifier.rs`: `identifier()`
- [ ] `text/integer.rs`: `integer()`
- [ ] `text/mod.rs`, `prelude.rs` にモジュール登録・エクスポート
- [ ] `tests/text_parsers.rs`: whitespace, identifier, integer のテスト

## Phase 2: quoted_string

- [ ] `text/quoted_string.rs`: `QuotedString` 具象型 + `Parser` 実装
  - JSON 準拠エスケープ全種（`\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\uXXXX`）
  - 不正エスケープ/未閉じクォートは `Fail::Cut`
- [ ] `tests/quoted_string.rs`: 正常系・エスケープ・エラー系のテスト

## Phase 3: JSON subset 統合テスト

- [ ] `tests/json_subset.rs`: 1段ネストの JSON パーサーを構築・検証
  - null, true, false, integer, string, array, object
  - 空白処理の検証
- [ ] COMMON.md, README.md のマイルストーン状態を更新

## 完了確認

- [ ] `cargo test -p oni-comb-parser` 全テスト通過
- [ ] `cargo bench --bench comparison -- --test` 全ベンチ正常動作
