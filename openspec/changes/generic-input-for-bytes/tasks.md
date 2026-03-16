# Tasks

## Phase 1: Input トレイト拡張 + StrInput 適合

- [ ] `input.rs`: `Token`, `Slice`(非GAT), `next_token()`, `peek_token()`, `slice_since()` を追加
- [ ] `str_input.rs`: 新 Input トレイトに適合（`Slice<'s>` → `Slice = &'a str`、新メソッド実装）
- [ ] `cargo build` が通ること（既存の text/ パーサーは一時的にコンパイルエラーになる可能性あり）

## Phase 2: primitive/ モジュール作成

- [ ] `primitive/mod.rs`: モジュール定義
- [ ] `primitive/take.rs`: `Take` — `impl<I: Input> Parser<I>`
- [ ] `primitive/satisfy.rs`: `Satisfy<F>` — `impl<I: Input, F> Parser<I>`
- [ ] `primitive/take_while0.rs`: `TakeWhile0<F>` — `impl<I: Input, F> Parser<I>`
- [ ] `primitive/take_while1.rs`: `TakeWhile1<F>` — `impl<I: Input, F> Parser<I>`
- [ ] `primitive/take_while_n_m.rs`: `TakeWhileNM<F>` — `impl<I: Input, F> Parser<I>`
- [ ] `primitive/eof.rs`: `Eof` — `impl<I: Input> Parser<I>`
- [ ] `lib.rs`: `pub mod primitive;` 追加

## Phase 3: text/ モジュールの移行

- [ ] `text/mod.rs`: take, satisfy, take_while 系, eof の定義を削除し、primitive/ から re-export
- [ ] `text/take.rs`, `text/satisfy.rs`, `text/take_while0.rs`, `text/take_while1.rs`, `text/take_while_n_m.rs`, `text/eof.rs`: 削除
- [ ] `text/take_while.rs`: primitive/ の re-export に変更
- [ ] `text/whitespace.rs`: primitive::take_while0/1 を使用するよう更新
- [ ] `text/lexeme.rs`: primitive::take_while0 を使用するよう更新
- [ ] `text/identifier.rs`: 必要に応じて primitive::satisfy を使用するよう更新
- [ ] `prelude.rs`: re-export パスを primitive/ に更新
- [ ] `cargo test -p oni-comb-parser` 既存テスト全通過

## Phase 4: ByteInput 追加

- [ ] `byte_input.rs`: `ByteInput<'a>` — `Input` 実装（Token=u8, Slice=&'a [u8]）
- [ ] `lib.rs`: `pub mod byte_input;` 追加
- [ ] `prelude.rs`: `ByteInput` を re-export

## Phase 5: Recursive ジェネリック化

- [ ] `combinator/recursive.rs`: `StrInput<'a>` 固定 → `I: Input` ジェネリック化
- [ ] 既存の arithmetic テスト等が通ること

## Phase 6: ByteInput 用テスト

- [ ] `tests/byte_input_basic.rs`: ByteInput の基本テスト（take, satisfy, take_while, eof）
- [ ] `tests/byte_input_combinator.rs`: ByteInput + コンビネータの結合テスト（map, zip, or, many0 等）
- [ ] `tests/byte_input_recursive.rs`: ByteInput + recursive の テスト

## 完了確認

- [ ] `cargo test -p oni-comb-parser` 全テスト通過
- [ ] `cargo bench --bench comparison -- --test` 全ベンチ正常動作
- [ ] CLAUDE.md のアーキテクチャセクション更新
