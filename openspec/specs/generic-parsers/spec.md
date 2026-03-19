## ADDED Requirements

### Requirement: sym はトークン型に対してジェネリックに動作する
`sym(token)` は `I::Token` を引数に取り、入力の次のトークンが一致すれば消費して返す。StrInput では `sym('a')` (char)、ByteInput では `sym(b'a')` (u8) として使用できる。不一致時は Backtrack エラーを返す。

#### Scenario: StrInput で char を一致させる
- **WHEN** `sym('a')` を `StrInput::new("abc")` に適用する
- **THEN** `Ok('a')` を返し、入力は `"bc"` に進む

#### Scenario: ByteInput で u8 を一致させる
- **WHEN** `sym(b'a')` を `ByteInput::new(b"abc")` に適用する
- **THEN** `Ok(b'a')` を返し、入力は `b"bc"` に進む

#### Scenario: トークン不一致で Backtrack
- **WHEN** `sym('a')` を `StrInput::new("xyz")` に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返し、入力位置は変わらない

#### Scenario: EOF で Backtrack
- **WHEN** `sym('a')` を空の StrInput に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返す

### Requirement: seq はスライス型に対してジェネリックに動作する
`seq(slice)` は入力の先頭が指定スライスに一致すれば消費して Slice を返す。StrInput では `seq("hello")` (&str)、ByteInput では `seq(b"hello")` (&[u8]) として使用できる。

#### Scenario: StrInput で文字列を一致させる
- **WHEN** `seq("hello")` を `StrInput::new("hello world")` に適用する
- **THEN** `Ok("hello")` を返し、入力は `" world"` に進む

#### Scenario: ByteInput でバイト列を一致させる
- **WHEN** `seq(b"hello")` を `ByteInput::new(b"hello world")` に適用する
- **THEN** `Ok(b"hello")` を返し、入力は `b" world"` に進む

#### Scenario: 部分一致で Backtrack
- **WHEN** `seq("hello")` を `StrInput::new("help")` に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返し、入力位置は変わらない

### Requirement: any は任意の1トークンを消費する
`any()` は入力から1トークンを消費して返す。EOF の場合は Backtrack エラーを返す。

#### Scenario: 1トークンを消費する
- **WHEN** `any()` を `StrInput::new("abc")` に適用する
- **THEN** `Ok('a')` を返し、入力は `"bc"` に進む

#### Scenario: EOF で Backtrack
- **WHEN** `any()` を空の StrInput に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返す

### Requirement: not_a は述語を満たさないトークンを消費する
`not_a(pred)` は次のトークンが述語を満たさない場合に消費して返す。満たす場合は Backtrack エラーを返す。

#### Scenario: 述語を満たさないトークンを消費する
- **WHEN** `not_a(|c: char| c == '"')` を `StrInput::new("abc")` に適用する
- **THEN** `Ok('a')` を返し、入力は `"bc"` に進む

#### Scenario: 述語を満たすトークンで Backtrack
- **WHEN** `not_a(|c: char| c == '"')` を `StrInput::new("\"abc")` に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返し、入力位置は変わらない

### Requirement: 演算子オーバーロードでパーサーを合成できる
Parser トレイトを実装する全ての型に対して、以下の演算子が使用できる。

#### Scenario: + で zip (両方の出力をタプルで返す)
- **WHEN** `sym('a') + sym('b')` を `StrInput::new("ab")` に適用する
- **THEN** `Ok(('a', 'b'))` を返す

#### Scenario: - で zip_left (左の出力のみ返す)
- **WHEN** `sym('a') - sym('b')` を `StrInput::new("ab")` に適用する
- **THEN** `Ok('a')` を返す

#### Scenario: * で zip_right (右の出力のみ返す)
- **WHEN** `sym('a') * sym('b')` を `StrInput::new("ab")` に適用する
- **THEN** `Ok('b')` を返す

#### Scenario: | で or (左が Backtrack なら右を試行)
- **WHEN** `sym('a') | sym('b')` を `StrInput::new("bc")` に適用する
- **THEN** `Ok('b')` を返す

#### Scenario: ! で not (否定先読み)
- **WHEN** `!sym('a')` を `StrInput::new("bc")` に適用する
- **THEN** `Ok(())` を返し、入力位置は変わらない

#### Scenario: >> で flat_map (モナディックバインド)
- **WHEN** `sym('a') >> |_| sym('b')` を `StrInput::new("ab")` に適用する
- **THEN** `Ok('b')` を返す

### Requirement: 既存の char/tag は StrInput 固定のショートカットとして維持する
`char('x')` と `tag("hello")` は引き続き StrInput 専用パーサーとして使用できる。sym/seq と共存する。

#### Scenario: char は従来通り動作する
- **WHEN** `char('a')` を `StrInput::new("abc")` に適用する
- **THEN** `Ok('a')` を返す（既存の動作と同一）

#### Scenario: tag は従来通り動作する
- **WHEN** `tag("hello")` を `StrInput::new("hello world")` に適用する
- **THEN** `Ok("hello")` を返す（既存の動作と同一）
