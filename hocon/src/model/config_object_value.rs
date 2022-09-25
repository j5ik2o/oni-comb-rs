use crate::model::config_value::ConfigValue;
use crate::model::{ConfigMergeable, Monoid};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigObjectValue(pub(crate) HashMap<String, ConfigValue>);

impl Display for ConfigObjectValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut s = String::new();
    let map = &self.0;
    for (k, v) in map {
      if !s.is_empty() {
        s.push_str(", ");
      }
      let line = format!("{} = {}", k, v);
      s.push_str(&line);
    }
    write!(f, "{{ {} }}", s)
  }
}

impl Monoid for ConfigObjectValue {
  fn combine(&mut self, other: &Self) {
    for (key, cv) in &other.0 {
      match self.0.get_mut(key) {
        None => {
          self.0.insert(key.clone(), cv.clone());
        }
        Some(entry) => {
          entry.combine(cv);
        }
      }
    }
  }
}

impl ConfigMergeable for ConfigObjectValue {
  fn merge_with(&mut self, other: Self) {
    for (key, cv) in other.0 {
      match self.0.get_mut(&key) {
        None => {
          self.0.insert(key, cv);
        }
        Some(entry) => {
          entry.merge_with(cv);
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
