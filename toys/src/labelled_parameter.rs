use std::rc::Rc;
use crate::expr::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct LabelledParameter {
    name: String,
    parameter: Rc<Expr>,
}

impl LabelledParameter {
    pub fn new(name: String, parameter: Rc<Expr>) -> Self {
        Self { name, parameter }
    }
}