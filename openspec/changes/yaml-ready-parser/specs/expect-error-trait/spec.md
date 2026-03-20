## MODIFIED Requirements

### Requirement: ExpectError trait でエラー生成を抽象化する
`ExpectError` trait は bare `position: usize` だけでなく、現在の入力状態から位置情報と文脈を構築できる形でエラー生成を抽象化しなければならない (MUST)。全パーサーはエラー生成時にこの trait を経由し、主要経路で line、column、layout 文脈を失ってはならない。

#### Scenario: ParseError が入力状態から生成される
- **WHEN** 複数行入力の途中で `Expected::Char('a')` を期待して失敗する
- **THEN** `ParseError` は position だけでなく line、column、expected、context を持って生成される

#### Scenario: MinimalError は軽量エラーとして動作する
- **WHEN** core-only 構成で `MinimalError` を用いてエラーを生成する
- **THEN** `MinimalError` は軽量性を保ちつつ、少なくとも位置比較に必要な情報を保持する

### Requirement: map_res が ExpectError を要求する（ParseError ハードコード廃止）
`map_res` は特定のエラー型に依存せず、`ExpectError` を通じて入力状態に結びついたエラーを生成しなければならない (MUST)。

#### Scenario: alloc ありで位置文脈付きエラーが返る
- **WHEN** `ParseError` をエラー型とするパーサーで `map_res` が変換に失敗する
- **THEN** 返されるエラーは失敗位置の文脈を保持する

#### Scenario: alloc なしでも map_res が使える
- **WHEN** `MinimalError` をエラー型とするパーサーで `map_res` を使う
- **THEN** コンパイルが成功し、変換失敗時に軽量エラーが返る

## ADDED Requirements

### Requirement: ContextError と MergeError は位置文脈を保ったまま合成しなければならない
error merge と context 付与は、位置情報と layout-sensitive grammar の文脈を破壊してはならない (MUST)。より深い失敗位置を採用する場合でも、その位置に対応する文脈を保持しなければならない。

#### Scenario: or のエラー合成で深い位置の文脈を保つ
- **WHEN** 左右の分岐が異なる位置で失敗し、より深い位置のエラーが採用される
- **THEN** 採用されたエラーはその位置に対応する line、column、context を保持している

#### Scenario: context 付与で位置が失われない
- **WHEN** parser が `.context("block sequence")` の内部で失敗する
- **THEN** 返されるエラーは context と line/column の両方を保持している
