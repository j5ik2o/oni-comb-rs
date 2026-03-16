## Why

oni-comb-rs v2 は Applicative/Alternative 主体の設計で、コンビネータ合成のヒープアロケーションをゼロに抑えている。しかし、文脈依存文法（長さプレフィクス付きデータ、最初のトークンに応じた分岐など）や再帰パーサーでは、1つ目のパーサーの結果に基づいて次のパーサーを動的に選ぶモナディック合成（`flat_map` / `>>=`）が不可欠である。現状これが未提供のため、表現力に制限がある。

## What Changes

- `ParserExt` に `.flat_map(f)` メソッドを追加。`f: FnMut(Output) -> P2` で、1つ目の結果に基づいて次のパーサーを動的に選択する
- `combinator/` に `FlatMap<P, F>` concrete 型を追加。`Parser` トレイトを実装
- クロージャの戻り値が同一型なら `Box` 不要（concrete 型のまま）。異なる型を返す場合のみ `Box<dyn Parser>` で型消去
- README に flat_map の使用例と、Applicative との使い分けガイドを追記

## Capabilities

### New Capabilities
- `flat-map`: モナディックパーサー合成（`flat_map` / `>>=`）。1つ目のパーサーの結果に基づいて次のパーサーを動的に選択する機能

### Modified Capabilities

## Impact

- `modules/parser/src/parser_ext.rs`: `flat_map` メソッド追加
- `modules/parser/src/combinator/`: `FlatMap<P, F>` 型と `Parser` 実装の新規ファイル追加
- `modules/parser/tests/`: flat_map のテストファイル追加
- `README.md`: 使用例とガイド追記
- 既存の Applicative コンビネータへの影響なし（後方互換）
