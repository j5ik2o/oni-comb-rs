## Why

oni-comb-rs は `#![no_std]` + `extern crate alloc` で構成されており、`alloc` クレートが常に必要。RP2040/RP2350（Raspberry Pi Pico）や軽量 WASM 環境ではヒープアロケータが利用できない、または使いたくないケースがある。

現在、`text/` パーサー（`char`, `tag`, `identifier` 等）がエラー型を `ParseError` にハードコードしており、`ParseError` が `Vec` を使うため `alloc` への依存が `text/` 層全体に伝播している。`primitive/` パーサーも同様。fold 系コンビネータ（`many0_fold` 等）の追加でコレクション不要の繰り返し処理は可能になったが、`char('a')` や `tag("AT+")` すら使えないのでは実用性が低い。

## What Changes

- `ExpectError` trait を導入し、エラー生成を抽象化する
- `Input` trait に `type Error: ExpectError` アソシエイテッド型を追加する
- `MinimalError`（位置のみ保持）を core-only のエラー型として追加する
- `StrInput` / `ByteInput` の `Input::Error` を `#[cfg(feature = "alloc")]` で `ParseError` / `MinimalError` に切り替える
- `text/` と `primitive/` の全パーサーで `ParseError` ハードコードを `I::Error` 経由に変更する
- `alloc` 依存のモジュール・メソッドを `#[cfg(feature = "alloc")]` で分離する
- `ParseError::expected_char()` 等の旧ファクトリメソッドは削除し、`ExpectError::from_expected()` に一本化する
- 後方互換のためのフォールバックコードは残さない（破壊的変更を許容）

## Capabilities

### New Capabilities
- `expect-error-trait`: `ExpectError` trait と `MinimalError` 型。エラー生成を抽象化し、alloc なし環境でのエラー型差し替えを可能にする
- `core-only-parsers`: `alloc` feature なし（`--no-default-features`）で `char`, `tag`, `identifier`, `integer`, `whitespace`, `satisfy`, `take_while` 等の大半のパーサーと、`map`, `zip`, `or`, `attempt`, `cut`, `optional`, `many0_fold`, `sep_by0_fold` 等のコンビネータが利用可能になる

### Modified Capabilities
- `ParseError` のファクトリメソッド（`expected_char`, `expected_tag`, `expected_description`）を削除し、`ExpectError::from_expected()` に統一（破壊的変更）
- `Input` trait に `type Error` を追加（破壊的変更: 既存の `Input` 実装に `type Error` の追加が必要）
- `map_res` の `ParseError` ハードコードを `ExpectError` 化

## Impact

- `modules/parser/src/error.rs`: `ExpectError` trait、`MinimalError` 型追加。`ParseError` のファクトリメソッド削除、`ExpectError` 実装追加
- `modules/parser/src/input.rs`: `type Error: ExpectError` 追加
- `modules/parser/src/str_input.rs`, `byte_input.rs`: `type Error` を cfg で切替
- `modules/parser/src/primitive/*.rs`: 全ファイルで `ParseError` → `I::Error` に変更
- `modules/parser/src/text/*.rs`: core 組（char, tag, identifier, integer, whitespace）で `ParseError` → `I::Error` に変更。alloc 組（quoted_string, escaped, regex）を `#[cfg(feature = "alloc")]` で囲む
- `modules/parser/src/combinator/*.rs`: many, many1, sep_by, chainr1, recursive を `#[cfg(feature = "alloc")]`。map_res を `ExpectError` 化
- `modules/parser/src/parser.rs`: `Box<P>` impl を `#[cfg(feature = "alloc")]`
- `modules/parser/src/parser_ext.rs`: alloc 依存メソッドを `#[cfg(feature = "alloc")]`
- `modules/parser/src/prelude.rs`: alloc 依存の re-export を `#[cfg(feature = "alloc")]`
- `modules/parser/src/lib.rs`: `extern crate alloc` を `#[cfg(feature = "alloc")]`
- `modules/parser/Cargo.toml`: `[features] default = ["alloc"]`, `alloc = []`
- 既存テスト: 全て `alloc` feature あり（デフォルト）で通る必要がある
- 新規テスト: `--no-default-features` でのビルドと基本動作確認
