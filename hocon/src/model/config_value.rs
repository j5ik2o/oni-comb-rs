use crate::model::config_array_value::ConfigArrayValue;
use crate::model::config_number_value::ConfigNumberValue;
use crate::model::config_object_value::ConfigObjectValue;
use crate::model::config_values::ConfigValues;
use crate::model::time_unit::TimeUnit;

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
  Null,
  Bool(bool),
  String(String),
  Number(ConfigNumberValue),
  Duration(ConfigNumberValue, TimeUnit),
  Array(ConfigArrayValue),
  Object(ConfigObjectValue),
  Reference(String, bool),
}

impl ConfigValue {
  pub fn has_child(&self) -> bool {
    match self {
      ConfigValue::Object(..) => true,
      ConfigValue::Array(..) => true,
      _ => false,
    }
  }

  pub fn get_value(&self, key: &str) -> Option<&ConfigValue> {
    self.get_values(key).map(|v| v.latest())
  }

  pub fn get_values(&self, path: &str) -> Option<&ConfigValues> {
    let keys = path.split(".").collect::<Vec<_>>();
    let key = keys[0];
    let child_count = keys.len() - 1;
    match self {
      ConfigValue::Object(map) => match map.0.get(key) {
        Some(cv) if child_count > 0 => cv.latest().get_values(&path[(key.len() + 1) as usize..]),
        Some(cv) => Some(cv),
        None => None,
      },
      _ => None,
    }
  }

  pub fn contains(&self, key: &str) -> bool {
    match self {
      ConfigValue::Object(map) => map.0.contains_key(key),
      _ => false,
    }
  }

  pub fn with_fallback(&mut self, other: Self) {
    match (self, other) {
      (ConfigValue::Object(l), ConfigValue::Object(r)) => {
        l.with_fallback(r);
      }
      (ConfigValue::Array(l), ConfigValue::Array(r)) => {
        l.with_fallback(r);
      }
      (..) => {}
    }
  }
}
