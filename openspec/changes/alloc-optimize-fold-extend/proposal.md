## Why

コレクター系コンビネータ（`many0`, `sep_by0` 等）は常に `Vec` にアロケーションするが、ユーザーが収集先を選べない。組み込み環境や `SmallVec`/`ArrayVec` を使いたいケース、あるいはそもそもコレクション不要で fold だけしたいケースに対応できていない。fold をプリミティブ層として導入し、その上に Extend ベースの API を積むことで、ゼロアロケーションから任意コンテナまでスケーラブルに対応する。

## What Changes

- `ManyFold<P, B, F>` コンビネータ型を新規追加（fold がプリミティブ層）
- `ParserExt` に `many0_fold` / `many1_fold` / `sep_by0_fold` / `sep_by1_fold` メソッドを追加
- `ParserExt` に `many0_into` / `many1_into` / `sep_by0_into` / `sep_by1_into` メソッドを追加（`Extend` トレイトベース）
- 既存の `Many<P>` / `Many1<P>` / `SepBy0<P, S>` / `SepBy1<P, S>` 型を廃止し、内部的に `ManyFold` で実装。戻り値型は `impl Parser` で隠す
- `chainl1` は元々 fold 的なので変更なし。`chainr1` の内部 Vec は将来課題として残す

## Capabilities

### New Capabilities
- `fold-combinators`: fold ベースの畳み込みコンビネータ群（`many0_fold`, `many1_fold`, `sep_by0_fold`, `sep_by1_fold`）。core-only 層でゼロアロケーション動作
- `extend-combinators`: Extend トレイトベースの収集コンビネータ群（`many0_into`, `many1_into`, `sep_by0_into`, `sep_by1_into`）。ユーザーが任意のコンテナ（`SmallVec`, `ArrayVec` 等）を持ち込める

### Modified Capabilities

（既存の spec に対する要件変更なし。既存 `many0` 等の公開 API は維持し、内部実装のみ変更）

## Impact

- `modules/parser/src/combinator/` 配下: `many.rs`, `many1.rs`, `sep_by.rs` の実装を `ManyFold` ベースに書き換え。新規ファイル `many_fold.rs` 等を追加
- `modules/parser/src/parser_ext.rs`: 新メソッド追加（`many0_fold`, `many0_into` 等）
- `modules/parser/src/combinator/mod.rs`: 新型のエクスポート
- 既存テスト: `many0` / `sep_by0` 等の既存テストはそのまま通る（振る舞い変更なし）
- RPITIT（Rust 1.75+）を使用。MSRV 制約なし（最新 Rust を前提）
- `extend(std::iter::once(item))` を使用し stable Rust 対応（nightly の `extend_one` は不使用）
