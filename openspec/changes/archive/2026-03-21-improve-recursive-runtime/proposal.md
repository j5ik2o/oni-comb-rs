## Why

`recursive()` は JSON、YAML、四則演算のような再帰文法を public combinator chain のまま記述するための要となっているが、現状の `Rc<UnsafeCell<Option<Box<dyn Parser>>>>` ベース実装は hot path に trait object dispatch と初期化チェックを持ち込み、再帰文法の性能を強く押し下げている。`fn_parser` へ逃がすのではなく declarative な文法記述を維持したままボトルネックを減らせるよう、`recursive()` の内部ランタイムを再設計する必要がある。

## What Changes

- `recursive()` の public API と combinator-chain による利用スタイルを維持したまま、内部実装を `Box<dyn Parser>` 依存から typed storage + thunk dispatch へ置き換える
- root owner と parser graph 内の自己参照 handle を分離し、再帰参照が強参照サイクルを作らないランタイム構造へ変更する
- steady-state の `Recursive::parse_next` から `Option` unwrap と trait object dispatch を外し、hot path を `ptr + parse_fn` に近い形へ単純化する
- 既存の成功/失敗意味論、`Clone` による再利用、Cut/Backtrack 伝播、ネストした再帰文法の成立性を維持する
- `recursive()` 改善の効果を確認するため、既存 recursive tests と arithmetic / JSON 系ベンチの観測点を更新する

## Capabilities

### New Capabilities

- `recursive-runtime`: `recursive()` combinator の API 互換性を保ちながら、owner/ref 分離と typed thunk dispatch による低オーバーヘッド runtime を提供する

### Modified Capabilities

<!-- None -->

## Impact

- `modules/parser/src/combinator/recursive.rs`: `recursive()` の内部表現と drop/clone/runtime 実装の主要変更対象
- `modules/parser/tests/recursive.rs`: 既存の再帰意味論と clone 再利用の回帰確認が必要
- `modules/parser/tests/arithmetic.rs`: 再帰文法の代表ケースとして意味論維持の検証対象
- `modules/json/src/parser.rs`, `modules/yaml/src/parser.rs`: public combinator chain の利用者として API 互換性の影響を受ける
- `modules/parser/benches/README.md` と関連ベンチ: `recursive()` ボトルネックの改善確認と説明更新が必要
- public API: 破壊的変更は想定しない
