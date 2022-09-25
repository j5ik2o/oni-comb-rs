use crate::model::config_value::ConfigValue;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigValueLink {
  pub(crate) prev: Rc<ConfigValue>,
  pub(crate) value: ConfigValue,
}

impl Display for ConfigValueLink {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value.to_string())
  }
}

impl ConfigValueLink {
  pub fn new(prev: Rc<ConfigValue>, value: ConfigValue) -> Self {
    Self { prev, value }
  }
}
