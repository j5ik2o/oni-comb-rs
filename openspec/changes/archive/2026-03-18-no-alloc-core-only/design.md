## Context

oni-comb-rs は `#![no_std]` + `extern crate alloc` だが、`alloc` なしでは何も使えない。原因は `text/` と `primitive/` のパーサーが `ParseError`（`Vec` 使用）をハードコードしているため。

alloc 依存は2種類に分かれる:
1. **エラー型依存**: `ParseError` をエラーとして返す（char, tag, satisfy 等）→ エラー型を抽象化すれば解消
2. **データ構造依存**: `Vec`, `Box`, `Rc`, `String`, `Cow` を返り値や内部に使う（many, recursive, quoted_string 等）→ alloc 必須、cfg で分離

## Goals / Non-Goals

**Goals:**
- `alloc` なし（core のみ）で `char`, `tag`, `identifier`, `integer`, `whitespace`, `satisfy`, `take_while`, `eof` と、`map`, `zip`, `or`, `attempt`, `cut`, `optional`, `many0_fold`, `sep_by0_fold` 等のコンビネータが使えるようにする
- RP2040/RP2350/WASM（no_std + no alloc）をターゲットとする
- `default = ["alloc"]` で既存ユーザーは何も変えずにビルドできる（ただし API は破壊的変更あり）
- 後方互換フォールバックコードは残さない

**Non-Goals:**
- alloc なしでの `quoted_string`, `escaped`, `recursive` の提供（データ構造が本質的に alloc 依存）
- `ParseError` の core-only 化（`Vec<Expected>` による複数エラー集約は alloc 必須）
- `StrInput` / `ByteInput` の分離やジェネリック化（cfg 切替で対応）
- chainr1 の alloc 不要化（内部 Vec は将来課題）

## Decisions

### D1: `ExpectError` trait でエラー生成を抽象化する

**選択**: `ExpectError` trait に `from_expected(pos, Expected) -> Self` メソッドを1つ定義し、全パーサーのエラー生成をこの trait 経由にする。

**理由**: text/primitive パーサーのエラー生成パターンは `expected_char`, `expected_tag`, `expected_description` の3種だが、全て `Expected` enum の variant に対応する。`Expected` enum 自体は alloc 不要（`&'static str` と `char` のみ）なので、`from_expected(pos, Expected)` の1メソッドで統一できる。

**代替案**:
- 3メソッド（`expected_char`, `expected_tag`, `expected_description`）を trait に定義 → メソッド数が増え、将来 `expected_byte` 等の追加時に trait 変更が必要
- `ErrorAt` のような最小 trait（位置のみ）+ Expected は optional → text パーサー側で Expected 情報を生成するコードが無駄になる

### D2: `Input::Error` に `ExpectError` のみを要求する

**選択**: `Input` trait に `type Error: ExpectError` を追加。`MergeError` と `ContextError` は `Input::Error` の bound には含めず、`or()` と `context()` の where 句で要求する（既存設計を維持）。

**理由**: ユーザーが独自の `Input` を実装する場合、`ExpectError` だけ実装すれば最低限動作する。`or()` や `context()` を使う場合のみ追加 trait が必要。

### D3: `StrInput` / `ByteInput` の `Error` を cfg で切り替える

**選択**: `#[cfg(feature = "alloc")] type Error = ParseError;` / `#[cfg(not(feature = "alloc"))] type Error = MinimalError;`

**理由**: 入力型を増やさずに済む。ユーザーは feature flag だけで切替可能。同じ `StrInput::new("...")` のコードが alloc あり/なし両方で動く。

**代替案**:
- `StrInput<'a, E>` にジェネリック化 → 型パラメータが増え、既存コードの互換性が下がる。1バイナリで両エラー型が共存できるメリットはあるが、RP2040/WASM では不要
- 別の入力型（`StrInputMinimal`）を用意 → ユーザーが使い分ける必要があり、コード重複が発生

### D4: `ParseError` のファクトリメソッドを削除し `ExpectError` に一本化する

**選択**: `ParseError::expected_char()`, `expected_tag()`, `expected_description()`, `expected_eof()` を全て削除。`ParseError` には `ExpectError::from_expected()` の実装のみを残す。`ParseError::new(pos, expected)` は内部実装として残してもよいが pub にはしない。

**理由**: 後方互換フォールバックを残さない方針。2つの生成パスが並存するとコードの一貫性が損なわれる。

### D5: alloc 依存モジュールの分離方針

**選択**: 以下をモジュール/メソッド単位で `#[cfg(feature = "alloc")]` で囲む:
- `error.rs` の `ParseError` 関連コード
- `parser.rs` の `impl Parser for Box<P>`
- `combinator/`: `many.rs`, `many1.rs`, `sep_by.rs`, `chainr1.rs`, `recursive.rs`
- `text/`: `quoted_string.rs`, `escaped.rs`, `regex.rs`
- `parser_ext.rs`: `many0`, `many1`, `sep_by0/1`, `many0_into`, `sep_by0_into`, `chainl1`, `chainr1` メソッド
- `prelude.rs`: alloc 依存の re-export

**理由**: モジュール単位が最も分かりやすく、`#[cfg]` の散在を防げる。

## Risks / Trade-offs

**[破壊的変更]** `Input` trait に `type Error` が追加される
→ 独自 `Input` 実装を持つユーザーは `type Error` の追加が必要。ただし外部ユーザーは少ないと想定。

**[破壊的変更]** `ParseError::expected_char()` 等のファクトリメソッドが削除される
→ 直接使っているユーザーは `ExpectError::from_expected()` に移行が必要。

**[性能影響]** `ParseError::from_expected()` で `Expected` enum 構築が1段増える
→ `#[inline]` で消えるはず。ベンチマーク実測で確認する。

**[API 複雑性]** `#[cfg]` による条件分岐が増える
→ ユーザー視点では `default = ["alloc"]` で透過的。ライブラリ内部の保守コストは増加。

## Open Questions

- `lexeme` は `whitespace0` に依存しており `whitespace0` は core-only で動くので core-only 利用可能。ただし現在の `lexeme` の where 句が `Error = ParseError` を要求している可能性あり → 実装時に確認
- `between` は `zip_left` + `zip_right` の糖衣で alloc 不要 → core-only で利用可能
- `fn_parser` は alloc 不要 → core-only で利用可能。ただし `recursive` は alloc 必要
