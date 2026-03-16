## Why

oni-comb-rs は現在 `&str` しか扱えない。バイト列（`&[u8]`）のパースにも対応し、バイナリプロトコルや非UTF-8データを処理できるようにしたい。v1 では `Element` トレイトで `char`/`u8` を統一していたが、v2 では `&str` ゼロコピーの性能を維持しつつ同等のジェネリシティを実現する。

未リリースのため破壊的変更は問題ない。`take()` と `byte_take()` のように関数を分けず、`take()` 自体をジェネリックにする方針。

## What Changes

- `Input` トレイトに `Token`・`Slice` associated type と `next_token()`/`peek_token()`/`slice_since()` メソッドを追加。`Slice` を GAT から通常の associated type に変更し、データライフタイムを保持
- `ByteInput<'a>` を新規追加（`Token = u8`, `Slice = &'a [u8]`）
- `take`, `satisfy`, `take_while0/1`, `take_while_n_m`, `eof` を `StrInput` 専用から `I: Input` ジェネリックに変更し、`primitive/` モジュールへ移動
- `tag`, `char_`, `identifier`, `integer`, `quoted_string` 等のテキスト専用パーサーは `text/` に残し `StrInput` 専用を維持
- `Recursive` と `Lexeme` を `I: Input` ジェネリックに拡張

## Capabilities

### New Capabilities
- `byte_input`: `ByteInput<'a>` — `&[u8]` 向け `Input` 実装
- `primitive/take`: ジェネリック `take(n)` — n トークン消費して `I::Slice` を返す
- `primitive/satisfy`: ジェネリック `satisfy(f)` — 1トークン条件付き消費
- `primitive/take_while0`: ジェネリック `take_while0(f)` — 条件を満たす間消費(0+)
- `primitive/take_while1`: ジェネリック `take_while1(f)` — 条件を満たす間消費(1+)
- `primitive/take_while_n_m`: ジェネリック `take_while_n_m(n, m, f)` — bounded take_while
- `primitive/eof`: ジェネリック `eof()` — 入力終端チェック

### Modified Capabilities
- `input`: `Input` トレイトに `Token`, `Slice`(非GAT), `next_token()`, `peek_token()`, `slice_since()` を追加
- `str_input`: `StrInput<'a>` を新しい `Input` トレイトに適合（`Token = char`, `Slice = &'a str`）
- `recursive`: `Recursive` を `I: Input` ジェネリックに拡張
- `lexeme`: `Lexeme` を `I: Input<Token = char>` に拡張

## Impact

### 破壊的変更
- `Input` トレイト: associated type 追加・`Slice` の GAT → 通常型への変更
- `StrInput`: `Input` impl の変更（`Slice<'s>` → `Slice`）
- `text/` パーサーの一部が `primitive/` に移動（re-export で互換性維持）
- `Recursive`, `Lexeme` の型パラメータ変更

### ファイル影響
- `modules/parser/src/input.rs`: `Input` トレイト拡張
- `modules/parser/src/str_input.rs`: 新トレイトに適合
- `modules/parser/src/byte_input.rs`: 新規
- `modules/parser/src/primitive/`: 新規ディレクトリ（take, satisfy, take_while0/1, take_while_n_m, eof）
- `modules/parser/src/text/`: take, satisfy, take_while, eof を削除（primitive/ へ移動）
- `modules/parser/src/combinator/recursive.rs`: ジェネリック化
- `modules/parser/src/text/lexeme.rs`: ジェネリック化
- `modules/parser/src/prelude.rs`: re-export 更新
- `modules/parser/src/lib.rs`: モジュール登録更新
- テストファイル: 既存テストの型注釈更新 + ByteInput 用テスト追加

## Design Decisions

### Slice を GAT から通常の associated type に変更
現状 `type Slice<'s> = &'s str where Self: 's` だが、Parser::Output でライフタイムを表現できない。`type Slice = &'a str`（'a は Self = StrInput<'a> から取得）に変更することで、`impl<I: Input> Parser<I> for Take { type Output = I::Slice; }` が自然に書ける。

### advance() をトレイトに含めない
バイト単位の advance は内部実装の詳細。トレイトの抽象レベルはトークン単位（`next_token()`）で統一。要素が何かで1要素の幅は自明に決まる。

### tag() はテキスト専用のまま
tag のリテラル型が入力型で異なる（`&str` vs `&[u8]`）。ジェネリック化するメリットより複雑さのコストが大きい。
