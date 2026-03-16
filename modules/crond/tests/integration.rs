use chrono::{DateTime, TimeZone, Utc};
use oni_comb_crond::{CronSchedule, CronSpecification, Specification};

fn utc(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Utc> {
  Utc.with_ymd_and_hms(y, m, d, h, min, 0).unwrap()
}

#[test]
fn e2e_every_5_minutes() {
  let sched = CronSchedule::new("*/5 * * * *").unwrap();
  let times: Vec<_> = sched.upcoming(utc(2024, 3, 1, 0, 0)).take(5).collect();
  assert_eq!(times[0], utc(2024, 3, 1, 0, 0));
  assert_eq!(times[1], utc(2024, 3, 1, 0, 5));
  assert_eq!(times[2], utc(2024, 3, 1, 0, 10));
  assert_eq!(times[3], utc(2024, 3, 1, 0, 15));
  assert_eq!(times[4], utc(2024, 3, 1, 0, 20));
}

#[test]
fn e2e_weekday_morning() {
  // MON at 9:00
  let sched = CronSchedule::new("0 9 * * MON").unwrap();
  // 2024-03-04 is Monday
  let times: Vec<_> = sched.upcoming(utc(2024, 3, 4, 0, 0)).take(3).collect();
  assert_eq!(times[0], utc(2024, 3, 4, 9, 0));
  assert_eq!(times[1], utc(2024, 3, 11, 9, 0));
  assert_eq!(times[2], utc(2024, 3, 18, 9, 0));
}

#[test]
fn e2e_contains() {
  let sched = CronSchedule::new("30 9 * * *").unwrap();
  assert!(sched.contains(&utc(2024, 6, 15, 9, 30)));
  assert!(!sched.contains(&utc(2024, 6, 15, 9, 31)));
}

#[test]
fn e2e_specification_pattern() {
  let spec = CronSpecification::new("0,30 * * * *").unwrap();
  assert!(spec.is_satisfied_by(&utc(2024, 1, 1, 12, 0)));
  assert!(spec.is_satisfied_by(&utc(2024, 1, 1, 12, 30)));
  assert!(!spec.is_satisfied_by(&utc(2024, 1, 1, 12, 15)));
}

#[test]
fn e2e_range_with_step() {
  let sched = CronSchedule::new("0-59/15 9-17 * * *").unwrap();
  // 09:00, 09:15, 09:30, 09:45, 10:00, ...
  let times: Vec<_> = sched.upcoming(utc(2024, 1, 1, 9, 0)).take(5).collect();
  assert_eq!(times[0], utc(2024, 1, 1, 9, 0));
  assert_eq!(times[1], utc(2024, 1, 1, 9, 15));
  assert_eq!(times[2], utc(2024, 1, 1, 9, 30));
  assert_eq!(times[3], utc(2024, 1, 1, 9, 45));
  assert_eq!(times[4], utc(2024, 1, 1, 10, 0));
}

#[test]
fn e2e_specific_day_and_month() {
  let sched = CronSchedule::new("0 0 25 12 *").unwrap();
  let times: Vec<_> = sched.upcoming(utc(2024, 1, 1, 0, 0)).take(2).collect();
  assert_eq!(times[0], utc(2024, 12, 25, 0, 0));
  assert_eq!(times[1], utc(2025, 12, 25, 0, 0));
}
