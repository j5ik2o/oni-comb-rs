## 1. Fold プリミティブ型の実装

- [x] 1.1 `ManyFold<P, B, F>` コンビネータ型を `combinator/many_fold.rs` に作成し、`Parser` を実装する（ループ制御: Backtrack→停止, Cut→伝播, ZeroProgress→エラー）
- [x] 1.2 `Many1Fold<P, B, F>` コンビネータ型を `combinator/many1_fold.rs` に作成する（最初の要素を必須にし、以降は ManyFold と同じループ）
- [x] 1.3 `SepByFold0<P, S, B, F>` コンビネータ型を `combinator/sep_by_fold.rs` に作成する（初回要素 + sep→element ループの fold 版）
- [x] 1.4 `SepByFold1<P, S, B, F>` を同ファイルに追加する（最初の要素を必須にする版）
- [x] 1.5 `combinator/mod.rs` に新型をエクスポートする
- [x] 1.6 fold 系コンビネータの単体テストを作成する（0個/1個/複数/Cut伝播/ZeroProgress検出）

## 2. ParserExt に fold メソッドを追加

- [x] 2.1 `parser_ext.rs` に `many0_fold` / `many1_fold` メソッドを追加する
- [x] 2.2 `parser_ext.rs` に `sep_by0_fold` / `sep_by1_fold` メソッドを追加する
- [x] 2.3 fold メソッドの統合テストを作成する（ParserExt 経由での利用）

## 3. ParserExt に Extend ベースの _into メソッドを追加

- [x] 3.1 `parser_ext.rs` に `many0_into` / `many1_into` メソッドを追加する（fold + `extend(once(item))` で実装）
- [x] 3.2 `parser_ext.rs` に `sep_by0_into` / `sep_by1_into` メソッドを追加する
- [x] 3.3 `_into` メソッドの単体テストを作成する（Vec / カスタム Extend 型での確認）

## 4. 既存 many0 / sep_by0 を fold ベースに移行

- [x] 4.1 `ParserExt::many0` の実装を `many0_into(Vec::new())` に書き換え、戻り値を `impl Parser` にする
- [x] 4.2 `ParserExt::many1` を同様に書き換える
- [x] 4.3 `ParserExt::sep_by0` / `sep_by1` を同様に書き換える
- [x] 4.4 旧 `Many<P>` / `Many1<P>` / `SepBy0<P, S>` / `SepBy1<P, S>` 型を非公開にする（または削除）
- [x] 4.5 既存テストスイートが全て通ることを確認する

### 4.1-4.4 に関する判断変更

ベンチマークの結果、fold の move-in/move-out パターンが直接 `push` より ~10% 遅い性能退行を確認。`#[inline(always)]` では解決不可。**既存の `many0`/`many1`/`sep_by0`/`sep_by1` は旧実装（直接 `push` ループの具象型）を維持**し、fold ベースの新 API（`many0_fold`, `many0_into` 等）を追加した状態で完了とする。旧型は `pub` のまま維持。

## 5. ベンチマーク検証

- [x] 5.1 既存の JSON フルベンチマークを実行し、fold ベース実装と旧実装の性能を比較する
- [x] 5.2 性能退行がある場合は `#[inline]` / `#[inline(always)]` で対応する
