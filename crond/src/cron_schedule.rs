use std::marker::PhantomData;

use chrono::{DateTime, TimeZone};
use intervals_rs::LimitValue;

use crate::cron_expr::CronExpr;
use crate::cron_interval::CronInterval;
use crate::cron_interval_iterator::CronIntervalIterator;
use crate::cron_parser::CronParser;
use crate::cron_specification::CronSpecification;

/// Facade that returns a CronInterval or CronIntervalIterator from a CROND string.<br/>
/// CROND文字列からCronIntervalやCronIntervalIteratorを返すFacade。
pub struct CronSchedule<Tz>
where
  Tz: TimeZone, {
  expr: CronExpr,
  phantom: PhantomData<Tz>,
}

impl<Tz: TimeZone> CronSchedule<Tz> {
  /// The Factory method.
  /// ファクトリメソッド.
  ///
  /// # Arguments(引数)
  ///
  /// * crond_string - CROND形式文字列
  ///
  /// # Retun values(戻り値)
  ///
  /// * Err:
  ///   - if CrondParser::parse fails
  ///   - CrondParser::parseに失敗した場合
  /// * Ok
  ///   - if CrondParser::parse succeeds
  ///   - CrondParser::parseに成功した場合
  pub fn new(crond_string: &str) -> Result<Self, String> {
    Ok(Self {
      expr: CronParser::parse(crond_string)?,
      phantom: PhantomData,
    })
  }

  /// Returns a CronInterval with date and time candidates after the start date and time.<br/>
  /// 開始日時以降の日時候補を持つCronIntervalを返す。
  pub fn cron_interval(&self, start: DateTime<Tz>) -> CronInterval<Tz, CronSpecification> {
    let spec = CronSpecification::new(self.expr.clone());
    let start = LimitValue::Limit(start);
    let end = LimitValue::Limitless;
    CronInterval::new(start, end, spec)
  }

  /// Returns a CronIntervalIterator with the date and time candidates after the start date and time.<br/>
  /// 開始日時以降の日時候補を持つCronIntervalIteratorを返す。
  pub fn upcoming(&self, start: DateTime<Tz>) -> CronIntervalIterator<Tz, CronSpecification> {
    self.cron_interval(start.clone()).iter(start.timezone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{TimeZone, Utc};

  #[test]
  fn test_iterator() {
    let dt: chrono::DateTime<Utc> = Utc.with_ymd_and_hms(2021, 1, 1, 1, 1, 0).unwrap();

    let itr: CronIntervalIterator<Utc, CronSpecification> =
      CronSchedule::new("0-59/30 0-23/2 * * *").unwrap().upcoming(dt);

    let dt_vec: Vec<DateTime<Utc>> = itr.take(5).collect::<Vec<_>>();
    assert_eq!(dt_vec[0], Utc.with_ymd_and_hms(2021, 1, 1, 2, 0, 0).unwrap());
    assert_eq!(dt_vec[1], Utc.with_ymd_and_hms(2021, 1, 1, 2, 30, 0).unwrap());
    assert_eq!(dt_vec[2], Utc.with_ymd_and_hms(2021, 1, 1, 4, 0, 0).unwrap());
    assert_eq!(dt_vec[3], Utc.with_ymd_and_hms(2021, 1, 1, 4, 30, 0).unwrap());
    assert_eq!(dt_vec[4], Utc.with_ymd_and_hms(2021, 1, 1, 6, 0, 0).unwrap());
    // 2021-01-01T02:00:00Z
    // 2021-01-01T02:30:00Z
    // 2021-01-01T04:00:00Z
    // 2021-01-01T04:30:00Z
    // 2021-01-01T06:00:00Z
  }
}
