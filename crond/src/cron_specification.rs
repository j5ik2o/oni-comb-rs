use crate::cron_evaluator::CronEvaluator;
use chrono::{DateTime, TimeZone};

use crate::cron_expr::CronExpr;

pub trait Specification<T>: Clone {
  fn is_satisfied_by(&self, arg: &T) -> bool;
}

#[derive(Clone)]
pub struct CronSpecification {
  expr: CronExpr,
}

impl CronSpecification {
  pub fn new(expr: CronExpr) -> Self {
    Self { expr }
  }
}

impl<Tz: TimeZone> Specification<DateTime<Tz>> for CronSpecification {
  fn is_satisfied_by(&self, datetime: &DateTime<Tz>) -> bool {
    CronEvaluator::new(datetime).eval(&self.expr)
  }
}
