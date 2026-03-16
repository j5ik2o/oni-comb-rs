# oni-comb-crond

[日本語](README.ja.md)

A cron expression parser and scheduler built on [oni-comb-parser](../parser/) v2 combinator API.

Ported from the v1 `crond` crate to v2's trait + concrete combinator type design.

## Features

- **5-field cron expression parsing** — `minute hour day month day-of-week`
- **Rich expression syntax** — `*`, `N`, `N-M`, `N-M/S`, `*/N`, comma lists, day-of-week names (`SUN`-`SAT`), `L` (last)
- **DateTime evaluation** — check if a `chrono::DateTime` matches a cron expression
- **Upcoming iterator** — iterate over future matching times from a given start
- **Specification pattern** — `Specification<DateTime<Tz>>` trait for predicate-based matching

## Quickstart

```rust
use chrono::Utc;
use oni_comb_crond::CronSchedule;

// Parse a cron expression and get upcoming matching times
let schedule = CronSchedule::new("*/5 * * * *").unwrap();

// Check if a specific time matches
let now = Utc::now();
if schedule.contains(&now) {
    println!("Now matches the schedule!");
}

// Iterate over the next 5 matching times
for time in schedule.upcoming(Utc::now()).take(5) {
    println!("{}", time);
}
```

## Cron Expression Format

```
┌───────────── minute (0-59)
│ ┌───────────── hour (0-23)
│ │ ┌───────────── day of month (1-31)
│ │ │ ┌───────────── month (1-12)
│ │ │ │ ┌───────────── day of week (1-7, SUN-SAT, or L)
│ │ │ │ │
* * * * *
```

### Supported Operators

| Operator | Example | Description |
|----------|---------|-------------|
| `*` | `* * * * *` | Match any value |
| `N` | `30 * * * *` | Match a specific value |
| `N-M` | `9-17 * * *` | Match a range (inclusive) |
| `N-M/S` | `0-59/15 * * * *` | Match a range with step |
| `*/N` | `*/5 * * * *` | Match every N-th value |
| `N,M,...` | `0,15,30,45 * * * *` | Match a list of values |
| `L` | `* * * * L` | Match the last day (day-of-week field) |
| `SUN`-`SAT` | `* * * * MON` | Day-of-week names (SUN=1, ..., SAT=7) |

### Examples

| Expression | Description |
|------------|-------------|
| `*/5 * * * *` | Every 5 minutes |
| `0 9 * * *` | Every day at 9:00 |
| `0 9 * * MON` | Every Monday at 9:00 |
| `0,30 * * * *` | Every hour at :00 and :30 |
| `0-59/15 9-17 * * *` | Every 15 min during business hours |
| `0 0 25 12 *` | Midnight on December 25th |

## API

### CronSchedule

```rust
// Parse a cron expression
let schedule = CronSchedule::new("0 9 * * MON")?;

// Check if a DateTime matches
schedule.contains(&datetime) -> bool

// Iterate upcoming matching times
schedule.upcoming(start) -> impl Iterator<Item = DateTime<Tz>>
```

### CronParser

```rust
// Parse to AST (low-level)
let expr = CronParser::parse("*/5 * * * *")?;
```

### CronSpecification

```rust
// Specification pattern
let spec = CronSpecification::new("0 9 * * *")?;
spec.is_satisfied_by(&datetime) -> bool
```

## Build & Test

```bash
cargo build -p oni-comb-crond
cargo test -p oni-comb-crond
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
