## 1. クレートセットアップ

- [x] 1.1 `modules/crond/Cargo.toml` 作成（`oni-comb-parser` + `chrono` 依存）
- [x] 1.2 ワークスペース `Cargo.toml` に `modules/crond` をメンバー追加
- [x] 1.3 `modules/crond/src/lib.rs` 作成（モジュール宣言）
- [x] 1.4 `cargo build -p oni-comb-crond` が通ること

## 2. AST 定義

- [x] 2.1 `cron_expr.rs`: `CronExpr` enum 定義（Value, AnyValue, LastValue, Range, AnyStep, List, Cron）
- [x] 2.2 `CronExpr` の `Debug`, `Clone`, `PartialEq` derive

## 3. パーサー実装

- [x] 3.1 `cron_parser.rs`: 数値パーサー（`take_while1` + `parse::<u8>`）
- [x] 3.2 分フィールドパーサー（0-59 バリデーション）
- [x] 3.3 時フィールドパーサー（0-23 バリデーション）
- [x] 3.4 日フィールドパーサー（1-31 バリデーション）
- [x] 3.5 月フィールドパーサー（1-12 バリデーション）
- [x] 3.6 曜日テキストパーサー（SUN-SAT → 1-7）
- [x] 3.7 曜日フィールドパーサー（数値 1-7 or テキスト or L）
- [x] 3.8 `*` (AnyValue) パーサー
- [x] 3.9 `*/N` (AnyStep) パーサー
- [x] 3.10 `N-M` / `N-M/S` (Range) パーサー
- [x] 3.11 リスト（カンマ区切り）パーサー
- [x] 3.12 フィールド式パーサー（list | range | any_step | asterisk | value）
- [x] 3.13 フル cron 式パーサー（5フィールド、スペース区切り）
- [x] 3.14 `CronParser::parse(input: &str) -> Result<CronExpr, String>` 公開 API
- [x] 3.15 パーサーユニットテスト（各式パターン）

## 4. 評価器実装

- [x] 4.1 `cron_evaluator.rs`: `CronEvaluator` 構造体
- [x] 4.2 `visit()` 関数（Value, AnyValue, LastValue, Range, AnyStep, List のパターンマッチ）
- [x] 4.3 `eval()` 関数（全5フィールドの評価）
- [x] 4.4 `get_days_from_month()` ヘルパー
- [x] 4.5 評価器ユニットテスト

## 5. スケジュール・イテレーター実装

- [x] 5.1 `cron_schedule.rs`: `CronSchedule` 構造体 + `new()` + `contains()`
- [x] 5.2 `UpcomingIterator` 構造体 + `Iterator` impl（1分刻み）
- [x] 5.3 `CronSchedule::upcoming()` メソッド
- [x] 5.4 スケジュールユニットテスト

## 6. Specification パターン（オプション）

- [x] 6.1 `cron_specification.rs`: `Specification<T>` トレイト + `CronSpecification`

## 7. 統合テスト・検証

- [x] 7.1 `tests/integration.rs`: E2E テスト（パース → 評価 → イテレーション）
- [x] 7.2 `cargo test -p oni-comb-crond` 全テスト通過
- [x] 7.3 `RUSTFLAGS="-D warnings" cargo clippy -p oni-comb-crond` 通過
