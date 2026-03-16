## ADDED Requirements

### Requirement: Create schedule from cron string
`CronSchedule::new()` は cron 式文字列をパースしてスケジュールを生成しなければならない（SHALL）。

#### Scenario: Valid cron string
- **WHEN** `CronSchedule::new("*/5 * * * *")` が呼ばれる
- **THEN** `Ok(CronSchedule)` を返す

#### Scenario: Invalid cron string
- **WHEN** `CronSchedule::new("invalid")` が呼ばれる
- **THEN** `Err` を返す

### Requirement: Check if datetime matches
`CronSchedule::contains()` は指定された `DateTime` が cron 式にマッチするかを判定しなければならない（SHALL）。

#### Scenario: Matching datetime
- **WHEN** `"0 9 * * *"` のスケジュールに対して 2024-01-15 09:00 を検査する
- **THEN** `true` を返す

#### Scenario: Non-matching datetime
- **WHEN** `"0 9 * * *"` のスケジュールに対して 2024-01-15 10:00 を検査する
- **THEN** `false` を返す

### Requirement: Iterate upcoming matching times
`CronSchedule::upcoming()` は指定時刻以降のマッチする `DateTime` を順に返すイテレーターを生成しなければならない（SHALL）。

#### Scenario: Every 5 minutes
- **WHEN** `"*/5 * * * *"` のスケジュールに対して 2024-01-15 09:00 から `upcoming()` を呼ぶ
- **THEN** 09:00, 09:05, 09:10, ... の順で `DateTime` を返す

#### Scenario: Specific time daily
- **WHEN** `"0 9 * * *"` のスケジュールに対して 2024-01-15 10:00 から `upcoming()` を呼ぶ
- **THEN** 最初の値は 2024-01-16 09:00 を返す

#### Scenario: Day-of-week filter
- **WHEN** `"0 9 * * MON"` のスケジュールに対して `upcoming()` を呼ぶ
- **THEN** 月曜日の 09:00 のみを返す

### Requirement: Iterator advances by one minute
`upcoming()` イテレーターは内部的に1分刻みで進み、マッチする時刻を返さなければならない（SHALL）。

#### Scenario: Skip non-matching minutes
- **WHEN** `"30 * * * *"` のスケジュールに対して 09:00 から開始
- **THEN** 最初の値は 09:30、次は 10:30 を返す
