## Why

MS3 (Combinators) の完了条件は「expression parser と CSV/JSON subset の骨格が書ける」こと。現在 map, zip, or, optional, many0, flat_map/and_then は実装済みだが、残り 8 コンビネータ（zip_left, zip_right, many1, sep_by0/1, chainl1/r1, between）が未実装で、JSON の配列/オブジェクトや四則演算式を簡潔に書けない。

ユーザーは `zip().map()` で回避可能だが、冗長で可読性が低い。winnow/nom と同等の速度を維持しつつ、chumsky 並みのメソッドチェーン体験を提供する。

## What Changes

`ParserExt` に 7 メソッドを追加し、コンストラクタ関数 `between` を 1 つ追加する。すべて具象コンビネータ型で実装し、ヒープアロケーションはゼロ（繰り返し系の Vec を除く）。

### zip ファミリー拡張: シーケンスで一方を捨てる

- `.zip_left(rhs)` → `ZipLeft<P1, P2>` — 両方実行、左の値を返す（= terminated）
- `.zip_right(rhs)` → `ZipRight<P1, P2>` — 両方実行、右の値を返す（= preceded）
- `between(left, parser, right)` → `ZipRight<L, ZipLeft<P, R>>` — コンストラクタ関数

### 繰り返し: 1 個以上、区切り付き

- `.many1()` → `Many1<P>` — 1 個以上の繰り返し（Vec）
- `.sep_by0(sep)` → `SepBy0<P, S>` — 区切り付き 0 個以上（Vec）
- `.sep_by1(sep)` → `SepBy1<P, S>` — 区切り付き 1 個以上（Vec）

### 二項演算子チェーン

- `.chainl1(op)` → `ChainL1<P, Op>` — 左結合 fold（alloc なし）
- `.chainr1(op)` → `ChainR1<P, Op>` — 右結合 fold（Vec で collect → 右から畳む）

## Capabilities

### New Capabilities
- `zip-left-right`: シーケンスの片方を捨てるコンビネータ（zip_left, zip_right, between）
- `many1`: 1 個以上の繰り返し
- `sep-by`: 区切り付き繰り返し（sep_by0, sep_by1）
- `chain`: 二項演算子の結合（chainl1, chainr1）

### Modified Capabilities
- なし（既存 API への変更なし、完全に後方互換）

## Impact

- `modules/parser/src/parser_ext.rs`: 7 メソッド追加
- `modules/parser/src/combinator/`: 具象型ファイル 7 つ追加（zip_left, zip_right, many1, sep_by0, sep_by1, chainl1, chainr1）
- `modules/parser/src/prelude.rs`: `between` 関数をエクスポート
- `modules/parser/tests/`: 各コンビネータのテストファイル追加
- 既存コードへの影響なし
