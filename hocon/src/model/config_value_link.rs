use crate::model::config_value::ConfigValue;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigValueLink {
  pub(crate) prev: Rc<ConfigValue>,
  pub(crate) value: ConfigValue,
}

impl ConfigValueLink {
  pub fn new(prev: Rc<ConfigValue>, value: ConfigValue) -> Self {
    Self { prev, value }
  }
}
