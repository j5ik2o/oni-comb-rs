## 1. ExpectError trait と MinimalError の導入

- [ ] 1.1 `error.rs` に `ExpectError` trait を追加する（`fn from_expected(position: usize, expected: Expected) -> Self`）
- [ ] 1.2 `error.rs` に `MinimalError` 型を追加する（`ExpectError`, `MergeError`, `ContextError` を実装）
- [ ] 1.3 `ParseError` に `ExpectError` を実装する
- [ ] 1.4 `ParseError` の旧ファクトリメソッド（`expected_char`, `expected_tag`, `expected_description`, `expected_eof`）を削除する
- [ ] 1.5 `ParseError::new(pos, expected)` を `pub` から `pub(crate)` または削除する
- [ ] 1.6 `ParseError` と `MinimalError` のテストを作成する

## 2. Input trait に Error 型を追加

- [ ] 2.1 `input.rs` の `Input` trait に `type Error: ExpectError` を追加する
- [ ] 2.2 `str_input.rs` の `Input` impl に `type Error` を追加する（cfg で `ParseError` / `MinimalError` を切替）
- [ ] 2.3 `byte_input.rs` の `Input` impl に `type Error` を追加する（同上）
- [ ] 2.4 `Cargo.toml` に `[features] default = ["alloc"]`, `alloc = []` を追加する
- [ ] 2.5 `lib.rs` の `extern crate alloc` を `#[cfg(feature = "alloc")]` で囲む

## 3. primitive/ パーサーのエラー型を I::Error に変更

- [ ] 3.1 `satisfy.rs` の `type Error = ParseError` を `type Error = I::Error` に変更し、エラー生成を `I::Error::from_expected()` に変更する
- [ ] 3.2 `take_while0.rs`, `take_while1.rs` を同様に変更する
- [ ] 3.3 `take_while_n_m.rs` を同様に変更する
- [ ] 3.4 `eof.rs` を同様に変更する
- [ ] 3.5 `take.rs`, `one_of.rs`, `none_of.rs`, `take_till0.rs`, `take_till1.rs` を同様に変更する
- [ ] 3.6 変更後に既存テストが通ることを確認する

## 4. text/ パーサー（core 組）のエラー型を I::Error に変更

- [ ] 4.1 `char.rs` の `type Error = ParseError` を `type Error = <StrInput as Input>::Error` に変更し、エラー生成を `Self::Error::from_expected()` に変更する
- [ ] 4.2 `tag.rs` を同様に変更する
- [ ] 4.3 `identifier.rs` を同様に変更する
- [ ] 4.4 `integer.rs` を同様に変更する
- [ ] 4.5 `whitespace.rs` は `take_while` ベースのため変更不要であることを確認する
- [ ] 4.6 `lexeme.rs` の where 句を確認し、必要なら `ExpectError` 化する
- [ ] 4.7 変更後に既存テストが通ることを確認する

## 5. text/ パーサー（alloc 組）と combinator/ の cfg 分離

- [ ] 5.1 `text/quoted_string.rs`, `text/escaped.rs`, `text/regex.rs` を `#[cfg(feature = "alloc")]` で囲む
- [ ] 5.2 `text/mod.rs` の該当モジュール宣言を `#[cfg(feature = "alloc")]` で囲む
- [ ] 5.3 `combinator/many.rs`, `many1.rs`, `sep_by.rs`, `chainr1.rs`, `recursive.rs` を `#[cfg(feature = "alloc")]` で囲む
- [ ] 5.4 `combinator/mod.rs` の該当モジュール宣言を `#[cfg(feature = "alloc")]` で囲む
- [ ] 5.5 `parser.rs` の `impl Parser for Box<P>` を `#[cfg(feature = "alloc")]` で囲む
- [ ] 5.6 `error.rs` の `ParseError` 関連コードを `#[cfg(feature = "alloc")]` で囲む（`Expected` enum と `ExpectError`/`MergeError`/`ContextError` trait は残す）

## 6. parser_ext.rs と prelude.rs の cfg 分離

- [ ] 6.1 `parser_ext.rs` の alloc 依存メソッド（`many0`, `many1`, `sep_by0/1`, `many0_into`, `many1_into`, `sep_by0_into`, `sep_by1_into`, `chainl1`, `chainr1`）を `#[cfg(feature = "alloc")]` で囲む
- [ ] 6.2 `parser_ext.rs` の `map_res` の where 句を `Error = ParseError` から `Error: ExpectError` に変更する
- [ ] 6.3 `parser_ext.rs` の alloc 依存 use 文を `#[cfg(feature = "alloc")]` で囲む
- [ ] 6.4 `prelude.rs` の alloc 依存 re-export（`escaped`, `quoted_string`, `recursive`）を `#[cfg(feature = "alloc")]` で囲む

## 7. テストと検証

- [ ] 7.1 `cargo build -p oni-comb-parser --no-default-features` でビルドが通ることを確認する
- [ ] 7.2 `cargo test -p oni-comb-parser`（デフォルト feature）で全テストが通ることを確認する
- [ ] 7.3 alloc なし環境での基本動作テストを作成する（char, tag, identifier, integer, fold 系）
- [ ] 7.4 JSON フルベンチマークで性能退行がないことを確認する
- [ ] 7.5 性能退行がある場合は `#[inline]` で対応する
