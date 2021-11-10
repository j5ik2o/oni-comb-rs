use crate::environment::Environment;
use crate::expr::Expr;
use crate::get_days_from_month;
use chrono::{DateTime, Datelike, TimeZone, Timelike};

pub struct Evaluator<'a, Tz: TimeZone> {
  instant: &'a DateTime<Tz>,
}

impl<'a, Tz: TimeZone> Evaluator<'a, Tz> {
  pub fn new(instant: &'a DateTime<Tz>) -> Self {
    Self { instant }
  }

  pub fn eval(&self, ast: &Expr) -> bool {
    match ast {
      Expr::CronExpr {
        box mins,
        box hours,
        box months,
        box days,
        box day_of_weeks,
      } => {
        let last_day = get_days_from_month(self.instant.date().year(), self.instant.date().month());
        let fmins = self.visit0(&Environment::new(self.instant.time().minute() as u8, 59), mins);
        let fhours = self.visit0(&Environment::new(self.instant.time().hour() as u8, 23), hours);
        let fdays = self.visit0(&Environment::new(self.instant.date().day() as u8, last_day as u8), days);
        let fmonths = self.visit0(&Environment::new(self.instant.date().month() as u8, 12), months);
        let fday_of_weeks = self.visit0(&Environment::new(self.instant.time().minute() as u8, 7), day_of_weeks);
        fmins && fhours && fdays && fmonths && fday_of_weeks
      }
      _ => false,
    }
  }

  fn visit1(&self, env: &Environment, ast: &Expr) -> bool {
    match ast {
      Expr::AnyValueExpr => true,
      Expr::LastValueExpr if env.now == env.max => true,
      Expr::ValueExpr(n) if env.now == *n => true,
      Expr::ListExpr(list) => list.iter().any(|e| self.visit0(env, e)),
      Expr::RangeExpr {
        from: box Expr::ValueExpr(start),
        to: box Expr::ValueExpr(end),
        per_option,
      } => match per_option {
        box Expr::NoOp if *start <= env.now && env.now <= *end => true,
        box Expr::ValueExpr(per) => (*start as usize..=*end as usize)
          .step_by(*per as usize)
          .into_iter()
          .any(|e| e == env.now as usize),
        _ => false,
      },
      Expr::PerExpr {
        digit: box Expr::AnyValueExpr,
        option: box Expr::ValueExpr(per),
      } => (0usize..=(env.max as usize))
        .step_by(*per as usize)
        .into_iter()
        .any(|e| e == env.now as usize),
      _ => false,
    }
  }

  fn visit0(&self, env: &Environment, ast: &Expr) -> bool {
    self.visit1(env, ast)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parsers::instruction;
  use chrono::Utc;
  use oni_comb_parser_rs::prelude::*;

  #[test]
  fn test_anytime() {
    let date_time = Utc.ymd(2021, 1, 1).and_hms(1, 1, 1);
    let evaluator = Evaluator::new(&date_time);
    let expr = Expr::CronExpr {
      mins: Box::from(Expr::AnyValueExpr),
      hours: Box::from(Expr::AnyValueExpr),
      days: Box::from(Expr::AnyValueExpr),
      months: Box::from(Expr::AnyValueExpr),
      day_of_weeks: Box::from(Expr::AnyValueExpr),
    };
    let result = evaluator.eval(&expr);
    assert!(result)
  }

  #[test]
  fn test_point_time() {
    let date_time = Utc.ymd(2021, 1, 1).and_hms(1, 1, 1);
    let evaluator = Evaluator::new(&date_time);
    let expr = Expr::CronExpr {
      mins: Box::from(Expr::ValueExpr(1)),
      hours: Box::from(Expr::ValueExpr(1)),
      days: Box::from(Expr::ValueExpr(1)),
      months: Box::from(Expr::ValueExpr(1)),
      day_of_weeks: Box::from(Expr::AnyValueExpr),
    };
    let result = evaluator.eval(&expr);
    assert!(result)
  }

  #[test]
  fn test_example() {
    let input = "* * * * *".chars().collect::<Vec<_>>();
    let expr = (instruction() - end()).parse(&input).to_result().unwrap();
    let date_time = Utc.ymd(2021, 1, 1).and_hms(1, 1, 1);
    let evaluator = Evaluator::new(&date_time);
    let result = evaluator.eval(&expr);
    assert!(result)
  }
}
