use crate::expr::Expr;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct LabelledParameter {
  pub(crate) name: String,
  pub(crate) parameter: Rc<Expr>,
}

impl LabelledParameter {
  pub fn new(name: String, parameter: Rc<Expr>) -> Self {
    Self { name, parameter }
  }
}
