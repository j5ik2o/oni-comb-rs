## ADDED Requirements

### Requirement: many0_into はユーザー指定のコンテナにゼロ個以上の要素を収集する
`many0_into(container)` は、`Extend<T>` を実装する任意のコンテナを受け取り、パーサーが Backtrack するまで繰り返し実行し、各成功結果をコンテナに追加する。内部的に `many0_fold` + `extend(once(item))` で実装される。

#### Scenario: Vec に収集する（既存の many0 と同等）
- **WHEN** `parser.many0_into(Vec::new())` で3要素をパースする
- **THEN** 3要素を含む `Vec` が返される

#### Scenario: SmallVec に収集する
- **WHEN** `parser.many0_into(SmallVec::<[Item; 8]>::new())` で3要素をパースする
- **THEN** 3要素を含む `SmallVec` が返される（ヒープアロケーションなし）

#### Scenario: 要素が0個の場合は空のコンテナを返す
- **WHEN** パーサーが最初の試行で Backtrack する
- **THEN** 渡されたコンテナがそのまま返される

#### Scenario: Cut / ZeroProgress の伝播は many0_fold と同一
- **WHEN** パーサーの実行中に Cut エラーが発生する
- **THEN** Cut エラーがそのまま返される

### Requirement: many1_into はユーザー指定のコンテナに1個以上の要素を収集する
`many1_into(container)` は、最初の要素は必須とし、以降は `many0_into` と同じ振る舞いをする。

#### Scenario: 要素が0個の場合はエラー
- **WHEN** パーサーが最初の試行で Backtrack する
- **THEN** Backtrack エラーがそのまま返される

#### Scenario: 要素が複数ある場合はコンテナに収集
- **WHEN** `parser.many1_into(Vec::new())` で3要素をパースする
- **THEN** 3要素を含む `Vec` が返される

### Requirement: sep_by0_into はユーザー指定のコンテナにセパレータ区切りのゼロ個以上の要素を収集する
`sep_by0_into(sep, container)` は、`Extend<T>` を実装する任意のコンテナを受け取り、セパレータで区切られた要素をゼロ個以上収集する。内部的に `sep_by0_fold` + `extend(once(item))` で実装される。

#### Scenario: Vec に収集する（既存の sep_by0 と同等）
- **WHEN** `parser.sep_by0_into(comma, Vec::new())` で `1,2,3` をパースする
- **THEN** 3要素を含む `Vec` が返される

#### Scenario: 要素が0個の場合は空のコンテナを返す
- **WHEN** 最初の要素パーサーが Backtrack する
- **THEN** 渡されたコンテナがそのまま返される

### Requirement: sep_by1_into はユーザー指定のコンテナにセパレータ区切りの1個以上の要素を収集する
`sep_by1_into(sep, container)` は、最初の要素は必須とし、以降は `sep_by0_into` と同じ振る舞いをする。

#### Scenario: 要素が0個の場合はエラー
- **WHEN** 最初の要素パーサーが Backtrack する
- **THEN** Backtrack エラーがそのまま返される

#### Scenario: 要素が複数ある場合はコンテナに収集
- **WHEN** `parser.sep_by1_into(comma, Vec::new())` で `1,2,3` をパースする
- **THEN** 3要素を含む `Vec` が返される

### Requirement: _into 系コンビネータは Extend トレイトのみを要求する
`many0_into`, `many1_into`, `sep_by0_into`, `sep_by1_into` はコンテナに対して `Extend<T>` トレイトのみを要求する。`extend(std::iter::once(item))` で要素を追加する（nightly の `extend_one` は使用しない）。

#### Scenario: Extend を実装する任意の型が使える
- **WHEN** `Extend<T>` を実装するカスタム型を渡す
- **THEN** コンパイルが成功し、`extend` が呼ばれて要素が追加される
