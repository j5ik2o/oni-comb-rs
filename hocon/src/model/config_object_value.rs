use crate::model::config_value::ConfigValue;
use std::collections::HashMap;

use crate::model::config_values::ConfigValues;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigObjectValue(pub(crate) HashMap<String, ConfigValues>);

impl From<HashMap<String, ConfigValues>> for ConfigObjectValue {
  fn from(values: HashMap<String, ConfigValues>) -> Self {
    Self::new(values)
  }
}

impl From<(String, ConfigValues)> for ConfigObjectValue {
  fn from((key, values): (String, ConfigValues)) -> Self {
    let mut m = HashMap::new();
    m.insert(key, values);
    Self::new(m)
  }
}

impl From<(String, ConfigValue)> for ConfigObjectValue {
  fn from((key, value): (String, ConfigValue)) -> Self {
    Self::from((key, ConfigValues::of_single(value)))
  }
}

impl ConfigObjectValue {
  pub fn new(values: HashMap<String, ConfigValues>) -> Self {
    Self(values)
  }

  pub fn combine(&mut self, other: Self) {
    for (k, v) in other.0 {
      match self.0.get_mut(&k) {
        None => {
          self.0.insert(k, v);
        }
        Some(m) => {
          m.combine(v);
        }
      }
    }
  }

  pub fn with_fallback(&mut self, other: Self) {
    for (k, v) in other.0 {
      match self.0.get_mut(&k) {
        None => {
          self.0.insert(k, v);
        }
        Some(m) => {
          m.with_fallback(v);
        }
      }
    }
  }
}
