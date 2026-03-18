## ADDED Requirements

### Requirement: ExpectError trait でエラー生成を抽象化する
`ExpectError` trait は `from_expected(position: usize, expected: Expected) -> Self` メソッドを提供し、全パーサーがエラー生成時にこの trait を経由する。

#### Scenario: ParseError が ExpectError を実装する
- **WHEN** `ParseError::from_expected(42, Expected::Char('a'))` を呼び出す
- **THEN** `ParseError { position: 42, expected: vec![Expected::Char('a')], context: vec![] }` が返される

#### Scenario: MinimalError が ExpectError を実装する
- **WHEN** `MinimalError::from_expected(42, Expected::Char('a'))` を呼び出す
- **THEN** `MinimalError { position: 42 }` が返される（Expected は破棄）

#### Scenario: MinimalError が MergeError を実装する
- **WHEN** `MinimalError { position: 10 }.merge(MinimalError { position: 20 })` を呼び出す
- **THEN** `MinimalError { position: 20 }` が返される（より後方の位置を採用）

#### Scenario: MinimalError が ContextError を実装する
- **WHEN** `MinimalError { position: 10 }.add_context("label")` を呼び出す
- **THEN** `MinimalError { position: 10 }` がそのまま返される（コンテキストは破棄）

### Requirement: Input trait に Error アソシエイテッド型を追加する
`Input` trait に `type Error: ExpectError` を追加する。全パーサーは `I::Error` を通じてエラー型にアクセスする。

#### Scenario: StrInput の Error が alloc feature で切り替わる
- **WHEN** `alloc` feature が有効
- **THEN** `<StrInput as Input>::Error` は `ParseError`

#### Scenario: StrInput の Error が alloc なしで MinimalError になる
- **WHEN** `alloc` feature が無効
- **THEN** `<StrInput as Input>::Error` は `MinimalError`

#### Scenario: ByteInput も同様に切り替わる
- **WHEN** `alloc` feature が有効/無効
- **THEN** `<ByteInput as Input>::Error` は `ParseError` / `MinimalError`

### Requirement: ParseError の旧ファクトリメソッドを削除する
`ParseError::expected_char()`, `expected_tag()`, `expected_description()`, `expected_eof()` を削除する。エラー生成は `ExpectError::from_expected()` に統一する。

#### Scenario: from_expected で全 Expected variant を生成できる
- **WHEN** `ParseError::from_expected(pos, Expected::Char('x'))` を呼ぶ
- **THEN** `expected_char(pos, 'x')` と同等の ParseError が返される

## MODIFIED Requirements

### Requirement: map_res が ExpectError を要求する（ParseError ハードコード廃止）
`map_res` の where 句を `P: Parser<I, Error = ParseError>` から `P::Error: ExpectError` に変更する。

#### Scenario: alloc なし環境で map_res が使える
- **WHEN** `MinimalError` をエラー型とするパーサーで `map_res` を使う
- **THEN** コンパイルが成功し、変換失敗時に `MinimalError` が返される
