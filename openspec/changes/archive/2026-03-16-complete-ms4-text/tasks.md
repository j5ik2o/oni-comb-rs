# Tasks

## Phase 1: 単純な便利関数

- [x] `text/whitespace.rs`: `whitespace0()`, `whitespace1()`
- [x] `text/identifier.rs`: `identifier()`
- [x] `text/integer.rs`: `integer()`
- [x] `text/mod.rs`, `prelude.rs` にモジュール登録・エクスポート
- [x] `tests/text_parsers.rs`: whitespace, identifier, integer のテスト

## Phase 2: quoted_string

- [x] `text/quoted_string.rs`: `QuotedString` 具象型 + `Parser` 実装
  - JSON 準拠エスケープ全種（`\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\uXXXX`）
  - 不正エスケープ/未閉じクォートは `Fail::Cut`
- [x] `tests/quoted_string.rs`: 正常系・エスケープ・エラー系のテスト

## Phase 2.5: 追加パーサー

- [x] `text/escaped.rs`: 汎用エスケープ文字列パーサー
- [x] `text/lexeme.rs`: トークンラッパー（後続空白消費）
- [x] `tests/escaped.rs`, `tests/lexeme.rs`: テスト

## Phase 3: 統合テスト

- [x] `tests/json_subset.rs`: 1段ネストの JSON パーサーを構築・検証
  - null, true, false, integer, string, array, object
  - 空白処理の検証
- [x] `tests/uri_tokenizer.rs`: URI パーサーを構築・検証
- [x] COMMON.md, README.md のマイルストーン状態を更新

## 完了確認

- [x] `cargo test -p oni-comb-parser` 全テスト通過
- [x] `cargo bench --bench comparison -- --test` 全ベンチ正常動作
