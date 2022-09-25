use crate::model::config_value::ConfigValue;
use crate::model::{ConfigMergeable, Monoid};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigArrayValue(pub(crate) Vec<ConfigValue>);

impl Display for ConfigArrayValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let s = self.0.iter().map(|cv| cv.to_string()).collect::<Vec<_>>().join(", ");
    write!(f, "[{}]", s)
  }
}

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
