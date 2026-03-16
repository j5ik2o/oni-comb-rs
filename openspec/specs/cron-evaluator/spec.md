## ADDED Requirements

### Requirement: Evaluate single value match
評価器は `Value(n)` がフィールドの現在値と一致するかを判定しなければならない（SHALL）。

#### Scenario: Minute matches
- **WHEN** 現在時刻の分が 30 で、分フィールドが `Value(30)`
- **THEN** `true` を返す

#### Scenario: Minute does not match
- **WHEN** 現在時刻の分が 15 で、分フィールドが `Value(30)`
- **THEN** `false` を返す

### Requirement: Evaluate wildcard
評価器は `AnyValue` に対して常に `true` を返さなければならない（SHALL）。

#### Scenario: Wildcard always matches
- **WHEN** 任意の時刻で、フィールドが `AnyValue`
- **THEN** `true` を返す

### Requirement: Evaluate range
評価器は `Range { from, to, step }` に対して、現在値が範囲内かを判定しなければならない（SHALL）。

#### Scenario: Value in range without step
- **WHEN** 現在値が 5 で、フィールドが `Range { from: 1, to: 10, step: None }`
- **THEN** `true` を返す

#### Scenario: Value in range with step
- **WHEN** 現在値が 4 で、フィールドが `Range { from: 0, to: 10, step: Some(2) }`
- **THEN** `true` を返す（0, 2, 4, 6, 8, 10 にマッチ）

#### Scenario: Value in range but not on step
- **WHEN** 現在値が 3 で、フィールドが `Range { from: 0, to: 10, step: Some(2) }`
- **THEN** `false` を返す

#### Scenario: Value outside range
- **WHEN** 現在値が 15 で、フィールドが `Range { from: 1, to: 10, step: None }`
- **THEN** `false` を返す

### Requirement: Evaluate wildcard with step
評価器は `AnyStep(n)` に対して、現在値が n の倍数かを判定しなければならない（SHALL）。

#### Scenario: Value is multiple of step
- **WHEN** 現在値が 15 で、フィールドが `AnyStep(5)`
- **THEN** `true` を返す

#### Scenario: Value is not multiple of step
- **WHEN** 現在値が 7 で、フィールドが `AnyStep(5)`
- **THEN** `false` を返す

### Requirement: Evaluate list
評価器は `List` のいずれかの要素がマッチすれば `true` を返さなければならない（SHALL）。

#### Scenario: Value matches one element in list
- **WHEN** 現在値が 15 で、フィールドが `List(vec![Value(0), Value(15), Value(30)])`
- **THEN** `true` を返す

#### Scenario: Value matches no element in list
- **WHEN** 現在値が 7 で、フィールドが `List(vec![Value(0), Value(15), Value(30)])`
- **THEN** `false` を返す

### Requirement: Evaluate last value
評価器は `LastValue` に対して、現在値がフィールドの最大値と一致するかを判定しなければならない（SHALL）。

#### Scenario: Last day of month
- **WHEN** 2024年2月29日で、日フィールドが `LastValue`
- **THEN** `true` を返す（2月の最終日）

### Requirement: Evaluate full cron expression
評価器は全5フィールドが全てマッチする場合のみ `true` を返さなければならない（SHALL）。

#### Scenario: All fields match
- **WHEN** 2024-01-15 09:00 (月曜) で、式が `"0 9 * * MON"`
- **THEN** `true` を返す

#### Scenario: One field does not match
- **WHEN** 2024-01-15 10:00 (月曜) で、式が `"0 9 * * MON"`
- **THEN** `false` を返す（時が不一致）
