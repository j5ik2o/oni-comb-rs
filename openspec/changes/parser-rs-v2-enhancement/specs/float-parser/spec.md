## ADDED Requirements

### Requirement: float は RFC 8259 準拠の浮動小数点数をパースする
`float()` は JSON 数値仕様 `[ "-" ] int [ frac ] [ exp ]` に従い f64 を返す。StrInput 専用。

#### Scenario: 整数をパースする
- **WHEN** `float()` を `StrInput::new("42")` に適用する
- **THEN** `Ok(42.0)` を返す

#### Scenario: 負の整数をパースする
- **WHEN** `float()` を `StrInput::new("-7")` に適用する
- **THEN** `Ok(-7.0)` を返す

#### Scenario: 小数をパースする
- **WHEN** `float()` を `StrInput::new("3.14")` に適用する
- **THEN** `Ok(3.14)` を返す

#### Scenario: 指数表記をパースする
- **WHEN** `float()` を `StrInput::new("1.5e10")` に適用する
- **THEN** `Ok(1.5e10)` を返す

#### Scenario: 負の指数をパースする
- **WHEN** `float()` を `StrInput::new("2.5E-3")` に適用する
- **THEN** `Ok(2.5e-3)` を返す

#### Scenario: ゼロをパースする
- **WHEN** `float()` を `StrInput::new("0")` に適用する
- **THEN** `Ok(0.0)` を返す

#### Scenario: 先頭ゼロは許可しない
- **WHEN** `float()` を `StrInput::new("007")` に適用する
- **THEN** `Ok(0.0)` を返し、入力は `"07"` に進む（先頭の `0` のみ消費）

#### Scenario: 小数部のみ（.5 形式）は不許可
- **WHEN** `float()` を `StrInput::new(".5")` に適用する
- **THEN** `Err(Fail::Backtrack(_))` を返す
