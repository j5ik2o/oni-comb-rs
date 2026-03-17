## Context

コレクター系コンビネータ（`many0`, `many1`, `sep_by0`, `sep_by1`）は `Vec` に直接 push するループを個別実装している。ループ制御ロジック（Backtrack→停止、Cut→伝播、ZeroProgress→エラー）は全て同一だが、4つのファイルに重複している。

現在の型階層:
- `Many<P>` → `Vec<P::Output>`
- `Many1<P>` → `Vec<P::Output>`
- `SepBy0<P, S>` → `Vec<P::Output>`
- `SepBy1<P, S>` → `Vec<P::Output>`

これらを fold プリミティブの上に統一し、ユーザーが収集先を選択可能にする。

## Goals / Non-Goals

**Goals:**
- fold をプリミティブ層として導入し、ループロジックを一元化する
- `Extend` ベースの `_into` API でユーザーが任意のコンテナ（`SmallVec`, `ArrayVec` 等）を持ち込めるようにする
- 既存の `many0` / `many1` / `sep_by0` / `sep_by1` API を維持し、内部実装のみ変更する
- fold 系コンビネータは core-only（`alloc` 不要）で動作させる

**Non-Goals:**
- `chainl1` / `chainr1` の変更（元々 fold 的で alloc 問題が小さい）
- `no_std` core-only feature gate の分離（別 change で実施）
- ベンチマーク上のパフォーマンス改善（目標は同等性能の維持）
- `quoted_string` / `ParseError` のアロケーション最適化

## Decisions

### D1: fold をプリミティブ、Extend / Vec をその上の糖衣にする

**選択**: `ManyFold<P, B, F>` を唯一のループ型とし、`many0` / `many0_into` はその上の糖衣とする。

**理由**: ループ制御ロジック（Backtrack, Cut, ZeroProgress の処理）が1箇所に集約される。既存の `Many<P>` / `Many1<P>` 等の個別型を廃止でき、保守対象が減る。

**代替案**:
- 個別型を残して `_fold` / `_into` を追加 → ループロジックが最大12箇所に分散（4コンビネータ × 3バリアント）
- 統一型 + Strategy パターン → trait bound が複雑になりすぎる

### D2: `extend(std::iter::once(item))` で stable Rust 対応

**選択**: `Extend::extend(std::iter::once(item))` を使用。

**理由**: `extend_one` は nightly のみ。`once(item)` はコンパイラがインライン化するため実質ゼロコスト。stable Rust で `SmallVec`, `ArrayVec`, `Vec` 等すべてに対応できる。

**代替案**:
- 独自トレイト `Collect<T> { push(T) }` を定義 → 外部クレートに手動 impl が必要で負担が大きい
- `FnMut(T)` コールバック → fold と区別がつかなくなる

### D3: RPITIT で戻り値型を隠す

**選択**: `ParserExt::many0` の戻り値を `impl Parser<I, Output = Vec<...>, Error = ...>` にする。

**理由**: 内部的に `ManyFold<Self, Vec<O>, impl FnMut(...)>` になるが、クロージャの型をユーザーに露出させたくない。Rust 1.75+ の RPITIT（Return Position Impl Trait in Trait）で隠せる。MSRV 制約なし。

**代替案**:
- Named fn 型（`VecPush` 等の ZST）→ 型が増え、ユーザーにも見える
- `Many<P>` 型を維持して内部で委譲 → 二重実装になる

### D4: many0 と sep_by0 は別の fold 型

**選択**: `ManyFold<P, B, F>` と `SepByFold<P, S, B, F>` の2つの fold 型を用意する。

**理由**: `sep_by` は separator パーサーを追加で持つため、`ManyFold` に separator を Optional にするよりも、別型のほうがシンプル。`many` 系と `sep_by` 系でループ構造が異なる（`sep_by` は初回要素の処理 + sep→element の繰り返し）。

### D5: many1_fold は many0_fold の上に構築

**選択**: `many1_fold(p, init, f)` は `p.parse_next()` で最初の1要素を取得し、`f(init, first)` で畳んだ後、残りを `many0_fold` のループで処理する。

**理由**: `many1` は「最低1つは必要」という制約を追加するだけ。ループロジックの重複を避ける。

実装としては `Many1Fold<P, B, F>` 型を追加し、`parse_next` 内で最初の要素取得後に `ManyFold` と同じループを実行する（型の共有ではなくロジックの共有）。

## Risks / Trade-offs

**[性能退行]** fold + `extend(once(item))` が直接 `push` より遅い可能性
→ ベンチマークで既存 `many0` との比較を実施。インライン化で同等になることを確認する。差が出た場合は `#[inline(always)]` で対応。

**[API 互換]** `many0` の戻り値型が `Many<P>` から `impl Parser<...>` に変わる
→ ユーザーが `Many<P>` を型として明示的に参照しているケースは壊れる。ただし通常は型推論に任せるため影響は限定的。`Many<P>` は `pub` だが `#[doc(hidden)]` 等で移行を促す。

**[型の複雑化]** `ManyFold` のジェネリックパラメータが3つ（P, B, F）で、エラーメッセージが読みにくくなる
→ RPITIT で隠すため、ユーザーは通常触れない。`_fold` / `_into` を直接使うユーザーは型パラメータを意識する必要があるが、上級者向け API なので許容。

## Open Questions

- `sep_by0_fold` の separator で消費された入力を fold の関数に渡すべきか？現在の `sep_by0` は separator の出力を捨てているが、fold では separator の値も畳み込みたいケースがあるかもしれない（例: `1+2-3` の演算子を含む fold）。ただしこれは `chainl1` のユースケースと重複するため、当面は separator を捨てる現行仕様に合わせる。
