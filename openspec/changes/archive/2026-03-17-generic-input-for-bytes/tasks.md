# Tasks

## Phase 1: Input トレイト拡張 + StrInput 適合

- [x] `input.rs`: `Token`, `Slice`(非GAT), `next_token()`, `peek_token()`, `slice_since()` を追加
- [x] `str_input.rs`: 新 Input トレイトに適合（`Slice<'s>` → `Slice = &'a str`、新メソッド実装）
- [x] `cargo build` が通ること（既存の text/ パーサーは一時的にコンパイルエラーになる可能性あり）

## Phase 2: primitive/ モジュール作成

- [x] `primitive/mod.rs`: モジュール定義
- [x] `primitive/take.rs`: `Take` — `impl<I: Input> Parser<I>`
- [x] `primitive/satisfy.rs`: `Satisfy<F>` — `impl<I: Input, F> Parser<I>`
- [x] `primitive/take_while0.rs`: `TakeWhile0<F>` — `impl<I: Input, F> Parser<I>`
- [x] `primitive/take_while1.rs`: `TakeWhile1<F>` — `impl<I: Input, F> Parser<I>`
- [x] `primitive/take_while_n_m.rs`: `TakeWhileNM<F>` — `impl<I: Input, F> Parser<I>`
- [x] `primitive/eof.rs`: `Eof` — `impl<I: Input> Parser<I>`
- [x] `lib.rs`: `pub mod primitive;` 追加

## Phase 3: text/ モジュールの移行

- [x] `text/mod.rs`: take, satisfy, take_while 系, eof の定義を削除し、primitive/ から re-export
- [x] `text/take.rs`, `text/satisfy.rs`, `text/take_while0.rs`, `text/take_while1.rs`, `text/take_while_n_m.rs`, `text/eof.rs`: 削除
- [x] `text/take_while.rs`: primitive/ の re-export に変更
- [x] `text/whitespace.rs`: primitive::take_while0/1 を使用するよう更新
- [x] `text/lexeme.rs`: primitive::take_while0 を使用するよう更新
- [x] `text/identifier.rs`: 必要に応じて primitive::satisfy を使用するよう更新
- [x] `prelude.rs`: re-export パスを primitive/ に更新
- [x] `cargo test -p oni-comb-parser` 既存テスト全通過

## Phase 4: ByteInput 追加

- [x] `byte_input.rs`: `ByteInput<'a>` — `Input` 実装（Token=u8, Slice=&'a [u8]）
- [x] `lib.rs`: `pub mod byte_input;` 追加
- [x] `prelude.rs`: `ByteInput` を re-export

## Phase 5: Recursive ジェネリック化

- [x] `combinator/recursive.rs`: `StrInput<'a>` 固定 → `I: Input` ジェネリック化
- [x] 既存の arithmetic テスト等が通ること

## Phase 6: ByteInput 用テスト

- [x] `tests/byte_input_basic.rs`: ByteInput の基本テスト（take, satisfy, take_while, eof）
- [x] `tests/byte_input_combinator.rs`: ByteInput + コンビネータの結合テスト（map, zip, or, many0 等）
- [x] `tests/byte_input_recursive.rs`: ByteInput + recursive の テスト

## 完了確認

- [x] `cargo test -p oni-comb-parser` 全テスト通過
- [x] `cargo bench --bench comparison -- --test` 全ベンチ正常動作
- [x] CLAUDE.md のアーキテクチャセクション更新
