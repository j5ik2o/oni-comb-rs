use crate::cron_environment::CronEnvironment;
use crate::cron_expr::CronExpr;
use crate::get_days_from_month;
use chrono::{DateTime, Datelike, TimeZone, Timelike};

pub struct CronEvaluator<'a, Tz: TimeZone> {
  instant: &'a DateTime<Tz>,
}

impl<'a, Tz: TimeZone> CronEvaluator<'a, Tz> {
  pub fn new(instant: &'a DateTime<Tz>) -> Self {
    Self { instant }
  }

  pub fn eval(&self, ast: &CronExpr) -> bool {
    match ast {
      CronExpr::CronExpr {
        box mins,
        box hours,
        box months,
        box days,
        box day_of_weeks,
      } => {
        let last_day = get_days_from_month(self.instant.date().year(), self.instant.date().month());
        let fmins = self.visit(&CronEnvironment::new(self.instant.time().minute() as u8, 59), mins);
        let fhours = self.visit(&CronEnvironment::new(self.instant.time().hour() as u8, 23), hours);
        let fdays = self.visit(
          &CronEnvironment::new(self.instant.date().day() as u8, last_day as u8),
          days,
        );
        let fmonths = self.visit(&CronEnvironment::new(self.instant.date().month() as u8, 12), months);
        let fday_of_weeks = self.visit(
          &CronEnvironment::new(self.instant.time().minute() as u8, 7),
          day_of_weeks,
        );
        fmins && fhours && fdays && fmonths && fday_of_weeks
      }
      _ => false,
    }
  }

  fn visit(&self, env: &CronEnvironment, ast: &CronExpr) -> bool {
    match ast {
      CronExpr::AnyValueExpr => true,
      CronExpr::LastValueExpr if env.now == env.max => true,
      CronExpr::ValueExpr(n) if env.now == *n => true,
      CronExpr::ListExpr(list) => list.iter().any(|e| self.visit(env, e)),
      CronExpr::RangeExpr {
        from: box CronExpr::ValueExpr(start),
        to: box CronExpr::ValueExpr(end),
        per_option,
      } => match per_option {
        box CronExpr::NoOp if *start <= env.now && env.now <= *end => true,
        box CronExpr::ValueExpr(per) => (*start as usize..=*end as usize)
          .step_by(*per as usize)
          .into_iter()
          .any(|e| e == env.now as usize),
        _ => false,
      },
      CronExpr::PerExpr {
        digit: box CronExpr::AnyValueExpr,
        option: box CronExpr::ValueExpr(per),
      } => (0usize..=(env.max as usize))
        .step_by(*per as usize)
        .into_iter()
        .any(|e| e == env.now as usize),
      _ => false,
    }
  }

  // fn visit0(&self, env: &Environment, ast: &Expr) -> bool {
  //  self.visit1(env, ast)
  //}
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::cron_parser::instruction;
  use chrono::Utc;
  use oni_comb_parser_rs::prelude::*;

  #[test]
  fn test_anytime() {
    let date_time = Utc.with_ymd_and_hms(2021, 1, 1, 1, 1, 1).unwrap();
    let evaluator = CronEvaluator::new(&date_time);
    let expr = CronExpr::CronExpr {
      mins: Box::from(CronExpr::AnyValueExpr),
      hours: Box::from(CronExpr::AnyValueExpr),
      days: Box::from(CronExpr::AnyValueExpr),
      months: Box::from(CronExpr::AnyValueExpr),
      day_of_weeks: Box::from(CronExpr::AnyValueExpr),
    };
    let result = evaluator.eval(&expr);
    assert!(result)
  }

  #[test]
  fn test_point_time() {
    let date_time = Utc.with_ymd_and_hms(2021, 1, 1, 1, 1, 1).unwrap();
    let evaluator = CronEvaluator::new(&date_time);
    let expr = CronExpr::CronExpr {
      mins: Box::from(CronExpr::ValueExpr(1)),
      hours: Box::from(CronExpr::ValueExpr(1)),
      days: Box::from(CronExpr::ValueExpr(1)),
      months: Box::from(CronExpr::ValueExpr(1)),
      day_of_weeks: Box::from(CronExpr::AnyValueExpr),
    };
    let result = evaluator.eval(&expr);
    assert!(result)
  }

  #[test]
  fn test_example() {
    let input = "* * * * *".chars().collect::<Vec<_>>();
    let expr = (instruction() - end()).parse(&input).to_result().unwrap();
    let date_time = Utc.with_ymd_and_hms(2021, 1, 1, 1, 1, 1).unwrap();
    let evaluator = CronEvaluator::new(&date_time);
    let result = evaluator.eval(&expr);
    assert!(result)
  }
}
