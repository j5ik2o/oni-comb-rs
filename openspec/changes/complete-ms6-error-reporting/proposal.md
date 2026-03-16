## Why

現在のエラー型は `String`（`format!` で構築）で、位置情報・期待トークン・コンテキストがなく、ユーザーがパース失敗の原因を特定できない。また `format!` のアロケーションがベンチマークで oni-comb を nom/winnow より遅くしている一因。

## What Changes

- `ParseError` 構造体を導入（position, expected, context）
- `MergeError` / `ContextError` trait を定義（`or` のマージと `.context()` 用）
- 全 text パーサーのエラー型を `String` → `ParseError` に変更
- `or` で左右の Backtrack エラーをマージ（expected の合成）
- `.context()` コンビネータを `ParserExt` に追加
- `ParseError` の `Display` 実装（人間向け出力）
- JSON subset のエラーテストで完了条件を実証

## Design Considerations

- `Fail<E>` の E はジェネリックのまま。将来の bytes 対応で同じ `ParseError` を使えるよう、入力型に依存しない設計
- `Expected` enum に `Byte`/`ByteTag` variant を予約（bytes 対応時に追加）
- trait ベースにより、ユーザーがカスタムエラー型を使うことも可能

## Capabilities

### New Capabilities
- `parse-error`: 構造化エラー型（位置、期待トークン、コンテキスト）
- `error-merge`: `or` での Backtrack エラー合成
- `error-context`: `.context()` でコンテキストラベルを積む

### Modified Capabilities
- 全 text パーサーの Error 型が `String` → `ParseError` に変更（破壊的変更）

## Impact

- `parser/src/error.rs`: `ParseError`, `Expected`, `MergeError`, `ContextError` (新規)
- `parser/src/text/*.rs`: 全パーサーのエラー生成を `ParseError` に変更 (10 ファイル)
- `parser/src/combinator/or.rs`: マージロジック追加
- `parser/src/parser_ext.rs`: `.context()` メソッド追加
- `parser/src/combinator/context.rs`: `Context` 具象型 (新規)
- `parser/tests/`: テスト更新 + エラーテスト追加
- 既存テストの一部で `Err(Fail::Backtrack(String))` → `Err(Fail::Backtrack(ParseError))` への対応が必要
