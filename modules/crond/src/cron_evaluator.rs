use chrono::{Datelike, TimeZone, Timelike};

use crate::cron_expr::CronExpr;

struct CronEnvironment {
  now: u8,
  max: u8,
}

pub fn get_days_from_month(year: i32, month: u32) -> u8 {
  let (y, m) = if month == 12 {
    (year + 1, 1)
  } else {
    (year, month + 1)
  };
  chrono::NaiveDate::from_ymd_opt(y, m, 1)
    .unwrap()
    .signed_duration_since(chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days() as u8
}

fn visit(env: &CronEnvironment, expr: &CronExpr) -> bool {
  match expr {
    CronExpr::AnyValue => true,
    CronExpr::Value(n) => env.now == *n,
    CronExpr::LastValue => env.now == env.max,
    CronExpr::AnyStep(step) => {
      if *step == 0 {
        return false;
      }
      env.now.is_multiple_of(*step)
    }
    CronExpr::Range { from, to, step } => {
      if env.now < *from || env.now > *to {
        return false;
      }
      match step {
        Some(s) if *s > 0 => (env.now - from).is_multiple_of(*s),
        _ => true,
      }
    }
    CronExpr::List(items) => items.iter().any(|item| visit(env, item)),
    CronExpr::Cron { .. } => false,
  }
}

pub struct CronEvaluator;

impl CronEvaluator {
  pub fn eval<Tz: TimeZone>(expr: &CronExpr, dt: &chrono::DateTime<Tz>) -> bool {
    let CronExpr::Cron {
      mins,
      hours,
      days,
      months,
      dow,
    } = expr
    else {
      return false;
    };

    let year = dt.year();
    let month = dt.month();
    let day = dt.day();
    let hour = dt.hour();
    let minute = dt.minute();
    let weekday = dt.weekday().num_days_from_sunday() as u8 + 1; // SUN=1..SAT=7
    let max_day = get_days_from_month(year, month);

    let min_env = CronEnvironment {
      now: minute as u8,
      max: 59,
    };
    let hour_env = CronEnvironment {
      now: hour as u8,
      max: 23,
    };
    let day_env = CronEnvironment {
      now: day as u8,
      max: max_day,
    };
    let month_env = CronEnvironment {
      now: month as u8,
      max: 12,
    };
    let dow_env = CronEnvironment {
      now: weekday,
      max: 7,
    };

    visit(&min_env, mins)
      && visit(&hour_env, hours)
      && visit(&day_env, days)
      && visit(&month_env, months)
      && visit(&dow_env, dow)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::TimeZone;

  fn utc(y: i32, m: u32, d: u32, h: u32, min: u32) -> chrono::DateTime<chrono::Utc> {
    chrono::Utc
      .with_ymd_and_hms(y, m, d, h, min, 0)
      .unwrap()
  }

  #[test]
  fn eval_any_value() {
    let expr = CronExpr::Cron {
      mins: Box::new(CronExpr::AnyValue),
      hours: Box::new(CronExpr::AnyValue),
      days: Box::new(CronExpr::AnyValue),
      months: Box::new(CronExpr::AnyValue),
      dow: Box::new(CronExpr::AnyValue),
    };
    assert!(CronEvaluator::eval(&expr, &utc(2024, 1, 15, 9, 30)));
  }

  #[test]
  fn eval_value_match() {
    let expr = CronExpr::Cron {
      mins: Box::new(CronExpr::Value(30)),
      hours: Box::new(CronExpr::Value(9)),
      days: Box::new(CronExpr::AnyValue),
      months: Box::new(CronExpr::AnyValue),
      dow: Box::new(CronExpr::AnyValue),
    };
    assert!(CronEvaluator::eval(&expr, &utc(2024, 1, 15, 9, 30)));
    assert!(!CronEvaluator::eval(&expr, &utc(2024, 1, 15, 10, 30)));
  }

  #[test]
  fn eval_range() {
    let expr = CronExpr::Cron {
      mins: Box::new(CronExpr::Range {
        from: 0,
        to: 30,
        step: None,
      }),
      hours: Box::new(CronExpr::AnyValue),
      days: Box::new(CronExpr::AnyValue),
      months: Box::new(CronExpr::AnyValue),
      dow: Box::new(CronExpr::AnyValue),
    };
    assert!(CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 15)));
    assert!(!CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 45)));
  }

  #[test]
  fn eval_range_with_step() {
    let expr = CronExpr::Cron {
      mins: Box::new(CronExpr::Range {
        from: 0,
        to: 59,
        step: Some(15),
      }),
      hours: Box::new(CronExpr::AnyValue),
      days: Box::new(CronExpr::AnyValue),
      months: Box::new(CronExpr::AnyValue),
      dow: Box::new(CronExpr::AnyValue),
    };
    assert!(CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 0)));
    assert!(CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 15)));
    assert!(CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 30)));
    assert!(!CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 7)));
  }

  #[test]
  fn eval_any_step() {
    let expr = CronExpr::Cron {
      mins: Box::new(CronExpr::AnyStep(5)),
      hours: Box::new(CronExpr::AnyValue),
      days: Box::new(CronExpr::AnyValue),
      months: Box::new(CronExpr::AnyValue),
      dow: Box::new(CronExpr::AnyValue),
    };
    assert!(CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 0)));
    assert!(CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 15)));
    assert!(!CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 7)));
  }

  #[test]
  fn eval_list() {
    let expr = CronExpr::Cron {
      mins: Box::new(CronExpr::List(vec![
        CronExpr::Value(0),
        CronExpr::Value(15),
        CronExpr::Value(30),
      ])),
      hours: Box::new(CronExpr::AnyValue),
      days: Box::new(CronExpr::AnyValue),
      months: Box::new(CronExpr::AnyValue),
      dow: Box::new(CronExpr::AnyValue),
    };
    assert!(CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 15)));
    assert!(!CronEvaluator::eval(&expr, &utc(2024, 1, 1, 0, 7)));
  }

  #[test]
  fn eval_days_in_month() {
    assert_eq!(get_days_from_month(2024, 2), 29);
    assert_eq!(get_days_from_month(2023, 2), 28);
    assert_eq!(get_days_from_month(2024, 1), 31);
  }
}
