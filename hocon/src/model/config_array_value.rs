use crate::model::config_value::ConfigValue;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigArrayValue(pub(crate) Vec<ConfigValue>);

impl ConfigArrayValue {
  pub fn new(value: Vec<ConfigValue>) -> Self {
    Self(value)
  }

  pub fn with_fallback(&mut self, other: Self) {
    self.0.extend(other.0);
  }
}
