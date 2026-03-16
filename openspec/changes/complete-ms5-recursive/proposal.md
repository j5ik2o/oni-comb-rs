## Why

MS5 (Recursive) の完了条件は「四則演算+括弧の parser が動く」こと。oni-comb-rs は具象コンビネータ型ベースのため、Rust の関数再帰では型が無限にネストして使えない。再帰の結び目だけを `Box<dyn Parser>` で型消去する `recursive()` ヘルパーが必要。

## What Changes

- `recursive()` コンストラクタ関数を追加。ユーザーはクロージャ内で再帰参照を受け取り、パーサーを組み立てる
- 内部は `Rc<RefCell<Option<Box<dyn Parser>>>>` で遅延束縛
- 四則演算+括弧の統合テストで完了条件を実証

## Capabilities

### New Capabilities
- `recursive`: 再帰パーサーの構築ヘルパー

### Modified Capabilities
- なし

## Impact

- `modules/parser/src/combinator/recursive.rs`: `Recursive` 型 + `recursive()` 関数
- `modules/parser/src/combinator/mod.rs`: モジュール登録
- `modules/parser/src/prelude.rs`: `recursive` をエクスポート
- `modules/parser/tests/recursive.rs`: 再帰パーサーのテスト
- `modules/parser/tests/arithmetic.rs`: 四則演算+括弧の統合テスト
- 既存コードへの変更なし
