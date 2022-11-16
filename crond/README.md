# oni-comb-crond-rs

## Usage

```rust
let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2021, 1, 1, 1, 1, 0).unwrap();

let expr: CronExpr = CronParser::parse("0-59/30 0-23/2 * * *").unwrap();
let interval: CronInterval<Utc, CronSpecification> = CronInterval::new(
  LimitValue::Limit(dt),
  LimitValue::Limitless,
  CronSpecification::new(expr),
);
let itr: CronIntervalIterator<Utc, CronSpecification> = interval.iter(Utc);

itr.take(5).for_each(|e| println!("{:?}", e));
// 2021-01-01T02:00:00Z
// 2021-01-01T02:30:00Z
// 2021-01-01T04:00:00Z
// 2021-01-01T04:30:00Z
// 2021-01-01T06:00:00Z
```