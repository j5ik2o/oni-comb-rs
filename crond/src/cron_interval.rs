use std::marker::PhantomData;
use std::rc::Rc;

use chrono::{DateTime, TimeZone};
use intervals_rs::{Interval, LimitValue};

use crate::cron_interval_iterator::CronIntervalIterator;
use crate::cron_specification::Specification;

/// A structure representing an interval of CROND date and time.<br/>
/// CROND日時の区間を表す構造体。
#[derive(Clone)]
pub struct CronInterval<Tz: TimeZone, S: Specification<DateTime<Tz>>> {
  pub(crate) underlying: Interval<i64>,
  pub(crate) cron_specification: S,
  phantom: PhantomData<Tz>,
}

impl<Tz: TimeZone, S: Specification<DateTime<Tz>>> CronInterval<Tz, S> {
  fn convert_to_long_limit_value(value: LimitValue<DateTime<Tz>>) -> LimitValue<i64> {
    match value {
      LimitValue::Limitless => LimitValue::Limitless,
      LimitValue::Limit(v) => LimitValue::Limit(v.timestamp_millis()),
    }
  }
}

impl<Tz: TimeZone, S: Specification<DateTime<Tz>>> CronInterval<Tz, S> {
  /// The Factory method.<br/>
  /// ファクトリメソッド。
  pub fn new(
    start_value: LimitValue<DateTime<Tz>>,
    end_value: LimitValue<DateTime<Tz>>,
    cron_specification: S,
  ) -> Self {
    let start = Self::convert_to_long_limit_value(start_value);
    let end = Self::convert_to_long_limit_value(end_value);
    Self {
      underlying: Interval::closed(start, end),
      cron_specification,
      phantom: PhantomData,
    }
  }

  /// Returns a CronIntervalIterator.<br/>
  /// CronIntervalIteratorを返す。
  pub fn iter(&self, timezone: Tz) -> CronIntervalIterator<Tz, S> {
    let timestamp = self.underlying.as_lower_limit().as_value().unwrap();
    let date_time = timezone.timestamp_millis_opt(*timestamp).unwrap();
    CronIntervalIterator::new(timezone, date_time.clone(), date_time, Rc::new(self.clone()))
  }
}
