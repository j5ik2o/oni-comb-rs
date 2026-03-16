## ADDED Requirements

### Requirement: Parse single value
パーサーは単一の数値を `Value(n)` としてパースできなければならない（SHALL）。

#### Scenario: Parse minute value "30"
- **WHEN** 分フィールドに `"30"` が与えられる
- **THEN** `Value(30)` を返す

#### Scenario: Parse day value "15"
- **WHEN** 日フィールドに `"15"` が与えられる
- **THEN** `Value(15)` を返す

### Requirement: Parse wildcard
パーサーは `*` を `AnyValue` としてパースできなければならない（SHALL）。

#### Scenario: Parse asterisk
- **WHEN** フィールドに `"*"` が与えられる
- **THEN** `AnyValue` を返す

### Requirement: Parse range expression
パーサーは `N-M` を `Range { from: N, to: M, step: None }` としてパースできなければならない（SHALL）。

#### Scenario: Parse range "1-10"
- **WHEN** フィールドに `"1-10"` が与えられる
- **THEN** `Range { from: 1, to: 10, step: None }` を返す

### Requirement: Parse range with step
パーサーは `N-M/S` を `Range { from: N, to: M, step: Some(S) }` としてパースできなければならない（SHALL）。

#### Scenario: Parse range with step "0-59/15"
- **WHEN** フィールドに `"0-59/15"` が与えられる
- **THEN** `Range { from: 0, to: 59, step: Some(15) }` を返す

### Requirement: Parse wildcard with step
パーサーは `*/N` を `AnyStep(N)` としてパースできなければならない（SHALL）。

#### Scenario: Parse "*/5"
- **WHEN** フィールドに `"*/5"` が与えられる
- **THEN** `AnyStep(5)` を返す

### Requirement: Parse list expression
パーサーはカンマ区切りの値リスト `N,M,O` を `List(vec![...])` としてパースできなければならない（SHALL）。

#### Scenario: Parse list "1,15,30"
- **WHEN** フィールドに `"1,15,30"` が与えられる
- **THEN** `List(vec![Value(1), Value(15), Value(30)])` を返す

#### Scenario: Parse list with ranges "1-5,10-15"
- **WHEN** フィールドに `"1-5,10-15"` が与えられる
- **THEN** `List(vec![Range{from:1,to:5,step:None}, Range{from:10,to:15,step:None}])` を返す

### Requirement: Parse day-of-week text
パーサーは曜日テキスト（SUN, MON, TUE, WED, THU, FRI, SAT）を対応する数値にパースできなければならない（SHALL）。

#### Scenario: Parse "MON"
- **WHEN** 曜日フィールドに `"MON"` が与えられる
- **THEN** `Value(2)` を返す（SUN=1, MON=2, ..., SAT=7）

#### Scenario: Parse "SUN"
- **WHEN** 曜日フィールドに `"SUN"` が与えられる
- **THEN** `Value(1)` を返す

### Requirement: Parse last value
パーサーは曜日フィールドの `L` を `LastValue` としてパースできなければならない（SHALL）。

#### Scenario: Parse "L" in day-of-week
- **WHEN** 曜日フィールドに `"L"` が与えられる
- **THEN** `LastValue` を返す

### Requirement: Parse full cron expression
パーサーはスペース区切りの5フィールド cron 式をパースできなければならない（SHALL）。

#### Scenario: Parse "*/5 * * * *"
- **WHEN** `"*/5 * * * *"` が与えられる
- **THEN** `Cron { mins: AnyStep(5), hours: AnyValue, days: AnyValue, months: AnyValue, dow: AnyValue }` を返す

#### Scenario: Parse "0 9 * * MON"
- **WHEN** `"0 9 * * MON"` が与えられる
- **THEN** `Cron { mins: Value(0), hours: Value(9), days: AnyValue, months: AnyValue, dow: Value(2) }` を返す

#### Scenario: Parse "0-59/30 0-23/2 * * *"
- **WHEN** `"0-59/30 0-23/2 * * *"` が与えられる
- **THEN** `Cron { mins: Range{0,59,Some(30)}, hours: Range{0,23,Some(2)}, days: AnyValue, months: AnyValue, dow: AnyValue }` を返す

### Requirement: Reject invalid cron expressions
パーサーは不正な cron 式に対してエラーを返さなければならない（SHALL）。

#### Scenario: Too few fields
- **WHEN** `"* * *"` が与えられる（3フィールドのみ）
- **THEN** パースエラーを返す

#### Scenario: Invalid character
- **WHEN** `"abc * * * *"` が与えられる
- **THEN** パースエラーを返す
