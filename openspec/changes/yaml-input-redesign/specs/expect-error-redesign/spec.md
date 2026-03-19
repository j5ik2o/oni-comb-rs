## ADDED Requirements

### Requirement: ExpectError に from_expected_at メソッドを追加する
`ExpectError` トレイトに `fn from_expected_at<I: Input>(input: &I, expected: Expected) -> Self` メソッドを追加する。`input.offset()`, `input.line()`, `input.column()` を自動取得してエラーを生成する。

#### Scenario: ParseError が行/列を自動で含む
- **WHEN** パーサーが `from_expected_at(input, Expected::Char('x'))` でエラーを生成する
- **THEN** `ParseError` の `line`, `column` フィールドに input の現在行/列が設定される

#### Scenario: MinimalError は position のみ
- **WHEN** `MinimalError::from_expected_at(input, expected)` を呼ぶ
- **THEN** `position` に `input.offset()` が設定される（MinimalError に line/column フィールドはない）

### Requirement: 全コンビネータのエラー生成が from_expected_at を使用する
`satisfy`, `sym`, `one_of`, `none_of`, `eof`, `not`, `repeat`, `seq` 等の全プリミティブ・コンビネータが `from_expected` の代わりに `from_expected_at` を使用する。

#### Scenario: sym の Backtrack エラーに行/列が含まれる
- **WHEN** `sym('a')` が `StrInput::new("b\nx")` の2行目1列目で失敗する
- **THEN** エラーの `line` は 2、`column` は 1

### Requirement: from_expected は deprecated になる
既存の `from_expected(position, expected)` は `#[deprecated]` アノテーション付きで維持する。

#### Scenario: from_expected 使用時に deprecation 警告
- **WHEN** `from_expected(0, Expected::Eof)` を使用するコードをコンパイルする
- **THEN** deprecated 警告が出力される

### Requirement: fill_location_from_src は削除する
`ParseError::fill_location_from_src` は `from_expected_at` の導入により不要になるため削除する。

#### Scenario: fill_location_from_src が存在しない
- **WHEN** `ParseError` のメソッド一覧を確認する
- **THEN** `fill_location_from_src` が存在しない

### Requirement: line_start フィールドの用途をドキュメント化する
`StrCheckpoint` / `ByteCheckpoint` の `line_start` フィールドに doc comment を追加し、「バイトオフセット。エラー時の行テキスト切り出し用。column (char 単位) とは異なる単位」と明示する。

#### Scenario: line_start に doc comment がある
- **WHEN** `StrCheckpoint` の `line_start` フィールドを確認する
- **THEN** 用途を説明する doc comment が存在する
