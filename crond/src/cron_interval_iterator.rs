use crate::cron_interval::CronInterval;
use crate::cron_specification::Specification;
use chrono::{DateTime, Duration, TimeZone};
use std::rc::Rc;

/// Iterator for The CronInterval.<br/>
/// CronIntervalのためのイテレータ。
#[derive(Clone)]
pub struct CronIntervalIterator<Tz: TimeZone, S: Specification<DateTime<Tz>>> {
  timezone: Tz,
  curr: DateTime<Tz>,
  next: DateTime<Tz>,
  cron_interval: Rc<CronInterval<Tz, S>>,
}

impl<Tz: TimeZone, S: Specification<DateTime<Tz>>> CronIntervalIterator<Tz, S> {
  /// The factory method.
  /// ファクトリメソッド。
  pub fn new(timezone: Tz, curr: DateTime<Tz>, next: DateTime<Tz>, cron_interval: Rc<CronInterval<Tz, S>>) -> Self {
    Self {
      timezone,
      curr,
      next,
      cron_interval,
    }
  }
}

impl<Tz: TimeZone, S: Specification<DateTime<Tz>>> Iterator for CronIntervalIterator<Tz, S> {
  type Item = DateTime<Tz>;

  fn next(&mut self) -> Option<Self::Item> {
    self.curr = self.next.clone();
    self.next = self.next.clone() + Duration::minutes(1);
    match self.end_value() {
      None => {
        self.proceed_next();
        let curr: DateTime<Tz> = self.curr.clone();
        Some(curr)
      }
      Some(end) => {
        if end >= self.curr {
          self.proceed_next();
          let curr: DateTime<Tz> = self.curr.clone();
          Some(curr)
        } else {
          None
        }
      }
    }
  }
}

impl<Tz: TimeZone, S: Specification<DateTime<Tz>>> CronIntervalIterator<Tz, S> {
  /// Returns the timezone of CronIntervalIterator.<br/>
  /// CronIntervalIteratorのタイムゾーンを返す。
  pub fn timezone(&self) -> &Tz {
    &self.timezone
  }

  /// Returns the CronInterval.<br/>
  /// CronIntervalを返す。
  pub fn cron_interval(&self) -> Rc<CronInterval<Tz, S>> {
    self.cron_interval.clone()
  }

  fn end_value(&self) -> Option<DateTime<Tz>> {
    if self.cron_interval.underlying.has_upper_limit() {
      let timestamp = self.cron_interval.underlying.as_upper_limit().as_value().unwrap();
      let date_time = self.timezone.timestamp_millis_opt(*timestamp).unwrap();
      Some(date_time)
    } else {
      None
    }
  }

  fn proceed_next(&mut self) {
    while !self.cron_interval.cron_specification.is_satisfied_by(&self.curr) {
      self.curr = self.next.clone();
      self.next = self.next.clone() + Duration::minutes(1);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::cron_parser::CronParser;
  use crate::cron_specification::CronSpecification;
  use chrono::{TimeZone, Utc};
  use intervals_rs::LimitValue;

  #[test]
  fn test_iterator() {
    let dt = Utc.with_ymd_and_hms(2021, 1, 1, 1, 1, 0).unwrap();

    let expr = CronParser::parse("0-59/30 0-23/2 * * *").unwrap();
    let interval = CronInterval::new(
      LimitValue::Limit(dt),
      LimitValue::Limitless,
      CronSpecification::new(expr),
    );
    let itr = interval.iter(Utc);
    itr.take(5).for_each(|e| println!("{:?}", e));
  }
}
