# oni-comb-crond

[English](README.md)

[oni-comb-parser](../parser/) v2 コンビネータ API で構築した cron 式パーサー＆スケジューラー。

v1 の `crond` クレートを v2 の trait + 具象コンビネータ型設計に移植したものです。

## 特徴

- **5フィールド cron 式のパース** — `分 時 日 月 曜日`
- **豊富な式構文** — `*`, `N`, `N-M`, `N-M/S`, `*/N`, カンマ区切りリスト, 曜日テキスト（`SUN`-`SAT`）, `L`（最終日）
- **DateTime 評価** — `chrono::DateTime` が cron 式にマッチするかを判定
- **upcoming イテレーター** — 指定時刻以降のマッチする時刻を順に返す
- **Specification パターン** — `Specification<DateTime<Tz>>` トレイトによる述語ベースのマッチング

## クイックスタート

```rust
use chrono::Utc;
use oni_comb_crond::CronSchedule;

// cron 式をパースして、マッチする時刻を取得
let schedule = CronSchedule::new("*/5 * * * *").unwrap();

// 特定の時刻がマッチするかチェック
let now = Utc::now();
if schedule.contains(&now) {
    println!("現在時刻はスケジュールにマッチ！");
}

// 次の5回のマッチ時刻をイテレーション
for time in schedule.upcoming(Utc::now()).take(5) {
    println!("{}", time);
}
```

## cron 式のフォーマット

```
┌───────────── 分 (0-59)
│ ┌───────────── 時 (0-23)
│ │ ┌───────────── 日 (1-31)
│ │ │ ┌───────────── 月 (1-12)
│ │ │ │ ┌───────────── 曜日 (1-7, SUN-SAT, または L)
│ │ │ │ │
* * * * *
```

### サポートする演算子

| 演算子 | 例 | 説明 |
|--------|-----|------|
| `*` | `* * * * *` | 任意の値にマッチ |
| `N` | `30 * * * *` | 特定の値にマッチ |
| `N-M` | `9-17 * * *` | 範囲にマッチ（両端含む） |
| `N-M/S` | `0-59/15 * * * *` | ステップ付き範囲にマッチ |
| `*/N` | `*/5 * * * *` | N ごとにマッチ |
| `N,M,...` | `0,15,30,45 * * * *` | 値のリストにマッチ |
| `L` | `* * * * L` | 最終日にマッチ（曜日フィールド） |
| `SUN`-`SAT` | `* * * * MON` | 曜日テキスト（SUN=1, ..., SAT=7） |

### 式の例

| 式 | 説明 |
|----|------|
| `*/5 * * * *` | 5分ごと |
| `0 9 * * *` | 毎日 9:00 |
| `0 9 * * MON` | 毎週月曜 9:00 |
| `0,30 * * * *` | 毎時 :00 と :30 |
| `0-59/15 9-17 * * *` | 営業時間中15分ごと |
| `0 0 25 12 *` | 12月25日の0:00 |

## API

### CronSchedule

```rust
// cron 式をパース
let schedule = CronSchedule::new("0 9 * * MON")?;

// DateTime がマッチするかチェック
schedule.contains(&datetime) -> bool

// マッチする時刻をイテレーション
schedule.upcoming(start) -> impl Iterator<Item = DateTime<Tz>>
```

### CronParser

```rust
// AST にパース（低レベル API）
let expr = CronParser::parse("*/5 * * * *")?;
```

### CronSpecification

```rust
// Specification パターン
let spec = CronSpecification::new("0 9 * * *")?;
spec.is_satisfied_by(&datetime) -> bool
```

## ビルド・テスト

```bash
cargo build -p oni-comb-crond
cargo test -p oni-comb-crond
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
