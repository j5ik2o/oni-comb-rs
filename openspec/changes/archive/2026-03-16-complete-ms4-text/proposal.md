## Why

MS4 (Text module) の完了条件は「JSON subset と URI tokenizer が動く」こと。MS3 でコンビネータが揃ったが、ユーザーが毎回 `satisfy(|c| c.is_ascii_alphabetic()).zip(take_while0(...))` のように組み立てるのは冗長。頻出パターンを便利関数として text モジュールに追加する。

## What Changes

text モジュールに 5 つの便利関数を追加する:

- `whitespace0()` — 0 個以上の ASCII 空白を消費（`take_while0` の糖衣）
- `whitespace1()` — 1 個以上の ASCII 空白を消費（`take_while1` の糖衣）
- `identifier()` — ASCII 識別子（`[a-zA-Z_][a-zA-Z0-9_]*`）を返す
- `integer()` — ASCII 整数をパースして `i64` で返す（先頭の `-` 対応）
- `quoted_string()` — ダブルクォート文字列。JSON 準拠のエスケープ（`\"`, `\\`, `\/`, `\n`, `\r`, `\t`, `\b`, `\f`, `\uXXXX`）を解釈して `String` を返す

加えて、JSON subset パーサーを統合テストとして追加し、MS4 完了を実証する。

## Capabilities

### New Capabilities
- `whitespace`: 空白消費パーサー（whitespace0, whitespace1）
- `identifier`: ASCII 識別子パーサー
- `integer`: 符号付き整数パーサー
- `quoted-string`: JSON 準拠エスケープ付きダブルクォート文字列パーサー

### Modified Capabilities
- なし（既存 API への変更なし）

## Impact

- `modules/parser/src/text/`: 4 ファイル追加（whitespace.rs, identifier.rs, integer.rs, quoted_string.rs）
- `modules/parser/src/text/mod.rs`: モジュール登録
- `modules/parser/src/prelude.rs`: 新関数をエクスポート
- `modules/parser/tests/`: 各パーサーのテスト + JSON subset 統合テスト
- 既存コードへの影響なし
