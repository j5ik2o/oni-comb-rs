use crate::model::config_value::ConfigValue;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigObjectValue(pub(crate) HashMap<String, ConfigValue>);

impl From<HashMap<String, ConfigValue>> for ConfigObjectValue {
  fn from(value: HashMap<String, ConfigValue>) -> Self {
    Self::new(value)
  }
}

impl From<(String, ConfigValue)> for ConfigObjectValue {
  fn from((key, value): (String, ConfigValue)) -> Self {
    let mut m = HashMap::new();
    m.insert(key, value);
    Self::new(m)
  }
}

impl ConfigObjectValue {
  pub fn new(value: HashMap<String, ConfigValue>) -> Self {
    Self(value)
  }

  pub fn combine(&mut self, other: &Self) {
    for (k, v) in &other.0 {
      match self.0.get_mut(k) {
        None => {
          self.0.insert(k.clone(), v.clone());
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
