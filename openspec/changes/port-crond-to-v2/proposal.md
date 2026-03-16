## Why

oni-comb-rs v1 には crond クレートがあり、cron 式のパース・評価・スケジューリングを提供していた。v2 ではパーサーコンビネータエンジンが完全に再設計されたが、crond はまだ移植されていない。v2 の `StrInput` + 具象コンビネータ型 API でcron パーサーを再実装し、v2 エコシステムの実用的なデモ兼テストケースとする。

## What Changes

- `crond` クレートを workspace メンバーとして新規追加（`modules/crond/`）
- cron 式の AST（`CronExpr`）を定義：`ValueExpr`, `AnyValueExpr`, `RangeExpr`, `PerExpr`, `ListExpr`, `LastValueExpr`
- v2 パーサーコンビネータ API（`satisfy`, `tag`, `take_while1`, `map`, `zip`, `or`, `sep_by` 等）で cron パーサーを実装
- `CronEvaluator`: AST を `chrono::DateTime` に対して評価
- `CronSchedule`: ファサード API（パース → 評価 → イテレーション）
- `CronIntervalIterator`: マッチする時刻を順に返すイテレーター
- `Specification<T>` トレイト + `CronSpecification` 実装

## Capabilities

### New Capabilities
- `cron-parser`: cron 式（5フィールド: min hour day month dow）のパース。`*`, `N`, `N-M`, `N-M/S`, `*/S`, `N,M,O`, `L`（曜日の最終日）をサポート
- `cron-evaluator`: パース済み AST を `DateTime` に対して評価し、マッチするかを判定
- `cron-schedule`: cron 式文字列から `upcoming()` イテレーターを生成するファサード API

### Modified Capabilities

（なし）

## Impact

### ファイル影響
- `Cargo.toml`（workspace）: `modules/crond` をメンバーに追加
- `modules/crond/`: 新規クレート一式（`Cargo.toml`, `src/`）
- 依存: `oni-comb-parser`（workspace 内）、`chrono`（外部）

### API
- `CronSchedule::new("*/5 * * * *")` → `Result<CronSchedule, ParseError>`
- `schedule.upcoming(Utc::now())` → `Iterator<Item = DateTime<Tz>>`
- `CronSpecification::is_satisfied_by(&datetime)` → `bool`
