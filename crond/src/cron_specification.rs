use chrono::{DateTime, TimeZone};

use crate::expr::Expr;
use crate::{CronEvaluator, Expr};

pub trait Specification<T>: Clone {
  fn is_satisfied_by(&self, arg: &T) -> bool;
}

#[derive(Clone)]
pub struct CronSpecification {
  expr: Expr,
}

impl CronSpecification {
  pub fn new(expr: Expr) -> Self {
    Self { expr }
  }
}

impl<Tz: TimeZone> Specification<DateTime<Tz>> for CronSpecification {
  fn is_satisfied_by(&self, datetime: &DateTime<Tz>) -> bool {
    CronEvaluator::new(datetime).eval(&self.expr)
  }
}
