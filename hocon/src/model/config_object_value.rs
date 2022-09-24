use crate::model::config_value::ConfigValue;
use crate::model::{ConfigMergeable, Monoid};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigObjectValue(pub(crate) HashMap<String, ConfigValue>);

impl Monoid for ConfigObjectValue {
  fn combine(&mut self, other: &Self) {
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
}

impl ConfigMergeable for ConfigObjectValue {
  fn merge_with(&mut self, other: Self) {
    for (k, v) in other.0 {
      match self.0.get_mut(&k) {
        None => {
          self.0.insert(k, v);
        }
        Some(m) => {
          m.merge_with(v);
        }
      }
    }
  }
}

impl From<HashMap<String, ConfigValue>> for ConfigObjectValue {
  fn from(value: HashMap<String, ConfigValue>) -> Self {
    Self::new(value)
  }
}

impl From<(String, ConfigValue)> for ConfigObjectValue {
  fn from((key, value): (String, ConfigValue)) -> Self {
    let mut key_values = HashMap::new();
    key_values.insert(key, value);
    Self::new(key_values)
  }
}

impl ConfigObjectValue {
  pub fn new(values: HashMap<String, ConfigValue>) -> Self {
    Self(values)
  }
}
