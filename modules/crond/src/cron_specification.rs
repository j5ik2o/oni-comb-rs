use chrono::{DateTime, TimeZone};

use crate::cron_evaluator::CronEvaluator;
use crate::cron_expr::CronExpr;
use crate::cron_parser::CronParser;

pub trait Specification<T> {
  fn is_satisfied_by(&self, arg: &T) -> bool;
}

#[derive(Clone)]
pub struct CronSpecification {
  expr: CronExpr,
}

impl CronSpecification {
  pub fn new(input: &str) -> Result<Self, String> {
    let expr = CronParser::parse(input)?;
    Ok(Self { expr })
  }
}

impl<Tz: TimeZone> Specification<DateTime<Tz>> for CronSpecification {
  fn is_satisfied_by(&self, dt: &DateTime<Tz>) -> bool {
    CronEvaluator::eval(&self.expr, dt)
  }
}
