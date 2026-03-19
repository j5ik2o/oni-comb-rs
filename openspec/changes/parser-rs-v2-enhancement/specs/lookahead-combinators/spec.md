## ADDED Requirements

### Requirement: not は否定先読みとして動作する
`not(parser)` は内部パーサーが Backtrack で失敗した場合に `Ok(())` を返す。内部パーサーが成功した場合は Backtrack エラーを返す。入力を消費しない。内部パーサーが Cut を返した場合は Cut をそのまま伝播する。

#### Scenario: 内部パーサーが失敗すれば成功
- **WHEN** `not(sym('a'))` を `StrInput::new("bc")` に適用する
- **THEN** `Ok(())` を返し、入力位置は変わらない

#### Scenario: 内部パーサーが成功すれば Backtrack
- **WHEN** `not(sym('a'))` を `StrInput::new("abc")` に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返し、入力位置は変わらない

#### Scenario: 内部パーサーが Cut を返せば Cut を伝播
- **WHEN** `not(sym('a').cut())` を `StrInput::new("xyz")` に適用し、内部で Cut が発生する場合
- **THEN** `Err(Fail::Cut(_))` をそのまま伝播する

### Requirement: peek は正先読みとして動作する
`peek(parser)` は内部パーサーを試行し、成功すれば出力を返すが入力を消費しない。失敗時はエラーをそのまま伝播する。

#### Scenario: 成功時は出力を返すが入力を消費しない
- **WHEN** `peek(sym('a'))` を `StrInput::new("abc")` に適用する
- **THEN** `Ok('a')` を返し、入力位置は変わらない

#### Scenario: 失敗時はエラーを伝播する
- **WHEN** `peek(sym('a'))` を `StrInput::new("xyz")` に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返す

### Requirement: guard は入力状態の条件判定として動作する
`guard(pred)` は述語 `Fn(&I) -> bool` を入力に適用し、true なら `Ok(())` を返す。false なら Backtrack エラーを返す。入力を消費しない。

#### Scenario: 条件を満たせば成功
- **WHEN** `guard(|input: &StrInput| input.column() > 2)` を column=3 の状態で適用する
- **THEN** `Ok(())` を返し、入力位置は変わらない

#### Scenario: 条件を満たさなければ Backtrack
- **WHEN** `guard(|input: &StrInput| input.column() > 2)` を column=1 の状態で適用する
- **THEN** `Err(Fail::Backtrack(_))` を返す

#### Scenario: EOF でも述語に基づいて判定する
- **WHEN** `guard(|input: &StrInput| input.is_eof())` を EOF 状態で適用する
- **THEN** `Ok(())` を返す
