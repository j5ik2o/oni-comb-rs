# oni-comb-crond-rs

A rust crate for CROND parser library.

## Usage

Returns an iterator that retrieve the calculation of the corresponding date and time from a CROND format string.

```rust
let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2021, 1, 1, 1, 1, 0).unwrap();

let itr: CronIntervalIterator<Utc, CronSpecification> = CronSchedule::new("0-59/30 0-23/2 * * *").unwrap().upcoming(dt);

let dt_vec: Vec<DateTime<Utc>> = itr.take(5).collect::<Vec<_>>();

// 2021-01-01T02:00:00Z
// 2021-01-01T02:30:00Z
// 2021-01-01T04:00:00Z
// 2021-01-01T04:30:00Z
// 2021-01-01T06:00:00Z
```
