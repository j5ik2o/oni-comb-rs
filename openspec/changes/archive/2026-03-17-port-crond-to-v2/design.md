# Port crond to v2 — 設計

## Context

v1 crond は `oni-comb-parser-rs` v1 API（`Vec<char>` 入力、`+`/`-`/`|` 演算子、`.cache()` メモ化）で実装されていた。v2 では `StrInput<'a>` + 具象コンビネータ型（`Map`, `Zip`, `Or` 等）に移行済み。v1 の `.cache()` は v2 に存在しないが、cron 式は短い（最大数十文字）ためメモ化は不要。

v1 crond は `intervals-rs` クレートに依存していたが、v2 では不要な複雑さを避け、`chrono` のみに依存する簡潔な設計とする。

## Goals / Non-Goals

**Goals:**
- v1 crond と同等の cron 式パース機能を v2 API で再実装
- 5フィールド cron（min hour day month dow）の完全サポート
- `DateTime<Tz>` に対する評価とマッチング時刻のイテレーション
- v2 パーサーコンビネータの実用的デモとして機能すること

**Non-Goals:**
- 6/7フィールド cron（秒、年）のサポート
- cron 式の逆生成（AST → 文字列）
- `intervals-rs` への依存（v1 の `LimitValue`/`Interval` は使わない）
- `no_std` 対応（`chrono` が `std` 前提）

## Decisions

### 1. クレート構成: `modules/crond/` として workspace メンバーに追加

v1 と同じくトップレベルに独立クレートとして配置。`oni-comb-parser` への依存は workspace 内パス参照。

### 2. AST 設計: v1 の `CronExpr` をほぼ踏襲

```rust
pub enum CronExpr {
    Value(u8),
    AnyValue,
    LastValue,
    Range { from: u8, to: u8, step: Option<u8> },
    AnyStep(u8),          // */N
    List(Vec<CronExpr>),
    Cron { mins: Box<CronExpr>, hours: Box<CronExpr>, days: Box<CronExpr>, months: Box<CronExpr>, dow: Box<CronExpr> },
}
```

**v1 との差分**: `PerExpr` と `RangeExpr` を統合。v1 は `RangeExpr { from, to, per_option }` + `PerExpr { digit, option }` と冗長だったが、v2 では `Range { from, to, step }` と `AnyStep(n)` に簡潔化。`NoOp` は削除（不要）。

### 3. パーサー設計: v2 の `prelude` API を使用

v1 の `+`（zip）→ v2 の `.zip()` / `.zip_left()` / `.zip_right()`
v1 の `|`（or）→ v2 の `.or()`
v1 の `-`（skip）→ v2 の `.zip_left()` / `.zip_right()`
v1 の `.cache()` → 不要（cron 式は短い）
v1 の `Vec<char>` → v2 の `StrInput<'a>`

各フィールドパーサーは `fn_parser` を使い、範囲バリデーション付きで実装:

```rust
// 例: 分（0-59）
fn minute_value() -> impl Parser<StrInput<'_>, Output = u8, Error = ParseError> {
    take_while1(|c: char| c.is_ascii_digit())
        .map(|s: &str| s.parse::<u8>().unwrap())
        .and_then(|n| if n <= 59 { Ok(n) } else { Err(...) })
}
```

### 4. 評価器: v1 と同等のパターンマッチ

`CronEvaluator::eval(&self, expr: &CronExpr, dt: &DateTime<Tz>) -> bool`

各フィールド（min, hour, day, month, dow）に対して再帰的にマッチング。`CronEnvironment` は v1 と同じく `{ now: u8, max: u8 }` を保持。

### 5. スケジューラ: `intervals-rs` 依存を排除

v1 の `CronInterval` + `LimitValue` + `Interval` の構造を簡潔化:

```rust
pub struct CronSchedule {
    expr: CronExpr,
}

impl CronSchedule {
    pub fn new(input: &str) -> Result<Self, String>;
    pub fn upcoming<Tz: TimeZone>(&self, from: DateTime<Tz>) -> UpcomingIterator<Tz>;
    pub fn contains<Tz: TimeZone>(&self, dt: &DateTime<Tz>) -> bool;
}
```

`UpcomingIterator` は `from` から1分刻みで進み、マッチする `DateTime` を `Iterator::next()` で返す。

### 6. 曜日テキストパース: `tag` による直接マッチ

```rust
tag("SUN").map(|_| 1)
    .or(tag("MON").map(|_| 2))
    .or(tag("TUE").map(|_| 3))
    // ...
```

v1 と同じアプローチ。大文字のみサポート（v1 準拠）。

## Risks / Trade-offs

- **パーサーのエラーメッセージ**: v2 の `ParseError` は位置と期待トークンを報告するが、cron 式のどのフィールドでエラーが起きたかのコンテキストは `.context()` で追加する必要がある
- **1分刻みイテレーション**: 遠い未来のマッチを探す場合に遅い可能性がある。v1 と同じ制約。将来的にフィールド単位のスキップ最適化が可能だが、初期実装では v1 同等とする
- **`chrono` バージョン**: v1 は `chrono 0.4.38` を使用。v2 でも同バージョン系を使用

## モジュール構成

```
modules/crond/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── cron_expr.rs          # AST enum
    ├── cron_parser.rs        # v2 パーサーコンビネータによるパーサー
    ├── cron_evaluator.rs     # AST 評価
    ├── cron_schedule.rs      # ファサード + UpcomingIterator
    └── cron_specification.rs # Specification トレイト
```

v1 の `cron_environment.rs`, `cron_interval.rs`, `cron_interval_iterator.rs` は `cron_evaluator.rs` と `cron_schedule.rs` に統合。
