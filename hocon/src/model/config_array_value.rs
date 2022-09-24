use crate::model::config_value::ConfigValue;
use crate::model::{ConfigMergeable, Monoid};

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigArrayValue(pub(crate) Vec<ConfigValue>);

impl Monoid for ConfigArrayValue {
  fn combine(&mut self, other: &Self) {
    self.0.extend(other.0.clone());
  }
}

impl ConfigMergeable for ConfigArrayValue {
  fn merge_with(&mut self, other: Self) {
    self.0.extend(other.0);
  }
}

impl ConfigArrayValue {
  pub fn new(value: Vec<ConfigValue>) -> Self {
    Self(value)
  }
}
