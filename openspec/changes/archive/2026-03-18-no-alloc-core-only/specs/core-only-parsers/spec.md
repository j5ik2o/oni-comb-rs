## ADDED Requirements

### Requirement: alloc なしで text パーサー（core 組）が使える
`char`, `tag`, `identifier`, `integer`, `whitespace0/1` は `alloc` feature なし（`--no-default-features`）でコンパイル・動作する。

#### Scenario: char パーサーが alloc なしで動作する
- **WHEN** `alloc` feature なしで `char('a')` を `StrInput::new("abc")` に適用する
- **THEN** `Ok('a')` が返され、入力が1文字進む

#### Scenario: tag パーサーが alloc なしで動作する
- **WHEN** `alloc` feature なしで `tag("AT+")` を `StrInput::new("AT+CMD")` に適用する
- **THEN** `Ok("AT+")` が返される

#### Scenario: identifier パーサーが alloc なしで動作する
- **WHEN** `alloc` feature なしで `identifier()` を `StrInput::new("foo_123 ")` に適用する
- **THEN** `Ok("foo_123")` が返される

#### Scenario: integer パーサーが alloc なしで動作する
- **WHEN** `alloc` feature なしで `integer()` を `StrInput::new("42")` に適用する
- **THEN** `Ok(42)` が返される

### Requirement: alloc なしで primitive パーサーが使える
`satisfy`, `take_while0/1`, `take_while_n_m`, `eof`, `take`, `one_of`, `none_of`, `take_till0/1` は `alloc` feature なしで動作する。

#### Scenario: satisfy が alloc なしで動作する
- **WHEN** `alloc` feature なしで `satisfy(|c: char| c.is_ascii_digit())` を適用する
- **THEN** 数字にマッチして返す

### Requirement: alloc なしでコンビネータ（core 組）が使える
`map`, `zip`, `zip_left`, `zip_right`, `or`, `attempt`, `cut`, `optional`, `context`, `map_res`, `flat_map`, `fn_parser`, `many0_fold`, `many1_fold`, `sep_by0_fold`, `sep_by1_fold` は `alloc` feature なしで動作する。

#### Scenario: or コンビネータが alloc なしで動作する
- **WHEN** `alloc` feature なしで `char('a').or(char('b'))` を適用する
- **THEN** 'a' または 'b' にマッチする

#### Scenario: many0_fold が alloc なしで動作する
- **WHEN** `alloc` feature なしで `char('a').many0_fold(|| 0, |n, _| n + 1)` を適用する
- **THEN** マッチした回数が返される

### Requirement: alloc 依存パーサーは cfg で分離される
`many0`, `many1`, `sep_by0/1`, `many0_into`, `sep_by0_into`, `chainl1`, `chainr1`, `recursive`, `quoted_string`, `escaped`, `regex` は `#[cfg(feature = "alloc")]` で分離される。

#### Scenario: alloc なしで many0 を使うとコンパイルエラー
- **WHEN** `alloc` feature なしで `char('a').many0()` を書く
- **THEN** コンパイルエラーになる（メソッドが存在しない）

#### Scenario: alloc ありで many0 は今まで通り動作する
- **WHEN** `alloc` feature あり（デフォルト）で `char('a').many0()` を書く
- **THEN** `Vec<char>` を返すパーサーとして動作する

### Requirement: alloc なしでビルドが通る
`cargo build -p oni-comb-parser --no-default-features` が成功する。

#### Scenario: no-default-features でビルド成功
- **WHEN** `cargo build -p oni-comb-parser --no-default-features` を実行する
- **THEN** コンパイルが成功する

### Requirement: alloc ありで既存テストが全て通る
`cargo test -p oni-comb-parser`（デフォルト feature）で全テストが通る。

#### Scenario: デフォルト feature で全テスト通過
- **WHEN** `cargo test -p oni-comb-parser` を実行する
- **THEN** 全テストが通過する
