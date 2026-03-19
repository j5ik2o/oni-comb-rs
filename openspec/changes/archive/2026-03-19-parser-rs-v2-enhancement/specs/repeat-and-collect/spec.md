## ADDED Requirements

### Requirement: repeat は Range で繰り返し回数を指定できる
`repeat(range)` はパーサーを指定回数繰り返し、結果を Vec で返す。`0..`, `1..`, `n..m`, `n..=m`, `..n`, `..=n` の Range 構文をサポートする。

#### Scenario: repeat(0..) は many0 と同等
- **WHEN** `sym('a').repeat(0..)` を `StrInput::new("aaab")` に適用する
- **THEN** `Ok(vec!['a', 'a', 'a'])` を返す

#### Scenario: repeat(1..) は many1 と同等
- **WHEN** `sym('a').repeat(1..)` を `StrInput::new("bbb")` に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返す（1つも一致しない）

#### Scenario: repeat(2..=4) は2〜4回の繰り返し
- **WHEN** `sym('a').repeat(2..=4)` を `StrInput::new("aaa")` に適用する
- **THEN** `Ok(vec!['a', 'a', 'a'])` を返す

#### Scenario: repeat(2..=4) で最小回数に満たない場合は Backtrack
- **WHEN** `sym('a').repeat(2..=4)` を `StrInput::new("ab")` に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返す

#### Scenario: repeat(..3) は0〜2回の繰り返し
- **WHEN** `sym('a').repeat(..3)` を `StrInput::new("aaaa")` に適用する
- **THEN** `Ok(vec!['a', 'a'])` を返し、入力は `"aa"` に進む

### Requirement: collect は checkpoint から現在位置までの Slice を返す
`collect()` はラップしたパーサーの出力を捨て、パース開始位置から終了位置までの入力 Slice を返す。

#### Scenario: パースした範囲の文字列を取得する
- **WHEN** `(sym('a') + sym('b') + sym('c')).collect()` を `StrInput::new("abcdef")` に適用する
- **THEN** `Ok("abc")` を返す（&str スライス）

#### Scenario: 数値文字列をそのまま取得する
- **WHEN** `satisfy(|c: char| c.is_ascii_digit()).repeat(1..).collect()` を `StrInput::new("123abc")` に適用する
- **THEN** `Ok("123")` を返す

### Requirement: discard はパーサーの出力を () に変換する
`discard()` はラップしたパーサーの出力を捨て `()` を返す。

#### Scenario: 出力を捨てる
- **WHEN** `sym('a').repeat(0..).discard()` を `StrInput::new("aaa")` に適用する
- **THEN** `Ok(())` を返す

### Requirement: position は現在の入力位置を返す
`position()` は入力を消費せず、現在の offset を返す。

#### Scenario: 現在の offset を取得する
- **WHEN** 2トークン消費後に `position()` を適用する
- **THEN** 現在の byte offset を返し、入力位置は変わらない
