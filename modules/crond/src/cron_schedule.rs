use chrono::{DateTime, Duration, TimeZone};

use crate::cron_evaluator::CronEvaluator;
use crate::cron_expr::CronExpr;
use crate::cron_parser::CronParser;

pub struct CronSchedule {
  expr: CronExpr,
}

impl CronSchedule {
  pub fn new(input: &str) -> Result<Self, String> {
    let expr = CronParser::parse(input)?;
    Ok(Self { expr })
  }

  pub fn contains<Tz: TimeZone>(&self, dt: &DateTime<Tz>) -> bool {
    CronEvaluator::eval(&self.expr, dt)
  }

  pub fn upcoming<Tz: TimeZone>(&self, from: DateTime<Tz>) -> UpcomingIterator<Tz> {
    UpcomingIterator {
      expr: self.expr.clone(),
      current: from,
    }
  }
}

pub struct UpcomingIterator<Tz: TimeZone> {
  expr: CronExpr,
  current: DateTime<Tz>,
}

impl<Tz: TimeZone> Iterator for UpcomingIterator<Tz> {
  type Item = DateTime<Tz>;

  fn next(&mut self) -> Option<Self::Item> {
    // 最大で2年分（約105万分）探索。見つからなければ None
    for _ in 0..1_051_200 {
      if CronEvaluator::eval(&self.expr, &self.current) {
        let result = self.current.clone();
        self.current = self.current.clone() + Duration::minutes(1);
        return Some(result);
      }
      self.current = self.current.clone() + Duration::minutes(1);
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Utc;

  fn utc(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(y, m, d, h, min, 0).unwrap()
  }

  #[test]
  fn schedule_new_valid() {
    assert!(CronSchedule::new("*/5 * * * *").is_ok());
  }

  #[test]
  fn schedule_new_invalid() {
    assert!(CronSchedule::new("invalid").is_err());
  }

  #[test]
  fn schedule_contains() {
    let s = CronSchedule::new("0 9 * * *").unwrap();
    assert!(s.contains(&utc(2024, 1, 15, 9, 0)));
    assert!(!s.contains(&utc(2024, 1, 15, 10, 0)));
  }

  #[test]
  fn schedule_upcoming_every_5_min() {
    let s = CronSchedule::new("*/5 * * * *").unwrap();
    let times: Vec<_> = s.upcoming(utc(2024, 1, 15, 9, 0)).take(4).collect();
    assert_eq!(times[0], utc(2024, 1, 15, 9, 0));
    assert_eq!(times[1], utc(2024, 1, 15, 9, 5));
    assert_eq!(times[2], utc(2024, 1, 15, 9, 10));
    assert_eq!(times[3], utc(2024, 1, 15, 9, 15));
  }

  #[test]
  fn schedule_upcoming_daily_9am() {
    let s = CronSchedule::new("0 9 * * *").unwrap();
    let times: Vec<_> = s.upcoming(utc(2024, 1, 15, 10, 0)).take(2).collect();
    assert_eq!(times[0], utc(2024, 1, 16, 9, 0));
    assert_eq!(times[1], utc(2024, 1, 17, 9, 0));
  }

  #[test]
  fn schedule_upcoming_half_hour() {
    let s = CronSchedule::new("30 * * * *").unwrap();
    let times: Vec<_> = s.upcoming(utc(2024, 1, 15, 9, 0)).take(2).collect();
    assert_eq!(times[0], utc(2024, 1, 15, 9, 30));
    assert_eq!(times[1], utc(2024, 1, 15, 10, 30));
  }
}
