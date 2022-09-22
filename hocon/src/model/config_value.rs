use crate::model::config_array_value::ConfigArrayValue;
use crate::model::config_duration_value::ConfigDurationValue;
use crate::model::config_number_value::ConfigNumberValue;
use crate::model::config_object_value::ConfigObjectValue;
use crate::model::config_values::ConfigValues;
use crate::model::{Config, ConfigFactory};
use std::env;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigIncludeValue {
  method: String,
  file_name: String,
}

impl ConfigIncludeValue {
  pub fn new(method: String, file_name: String) -> Self {
    Self { method, file_name }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
  Null,
  Bool(bool),
  String(String),
  Number(ConfigNumberValue),
  Duration(ConfigDurationValue),
  Array(ConfigArrayValue),
  Object(ConfigObjectValue),
  Reference(String, bool),
  Include(ConfigIncludeValue),
}

impl ConfigValue {
  pub fn has_child(&self) -> bool {
    match self {
      ConfigValue::Object(..) => true,
      ConfigValue::Array(..) => true,
      _ => false,
    }
  }

  pub fn is_include(&self) -> bool {
    match self {
      ConfigValue::Include(m) => true,
      _ => false,
    }
  }

  pub fn include(&self) -> Option<&ConfigIncludeValue> {
    match self {
      ConfigValue::Include(m) => Some(m),
      _ => None,
    }
  }

  pub fn resolve(self, source: Option<&Self>, prev_value: Option<Self>) -> Option<ConfigValue> {
    match (&self, source) {
      (ConfigValue::Include(m), ..) => {
        let mut config_factory = ConfigFactory::new();
        let c = config_factory.load_from_file(&m.file_name);
        c.ok().map(|c| c.to_config_value().clone())
      }
      (ConfigValue::Reference(ref_name, missing), Some(src)) => {
        let ref_value = src
          .get_value(ref_name)
          .cloned()
          .or_else(|| env::var(ref_name).ok().map(|s| ConfigValue::String(s)));
        if *missing {
          if ref_value.is_some() {
            ref_value
          } else {
            prev_value
          }
        } else {
          if ref_value.is_none() {
            panic!("Cannot resolve the reference: {}", ref_name)
          }
          ref_value
        }
      }
      _ => Some(self),
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
        Some(cv) if child_count > 0 => {
          let next_path = &path[(key.len() + 1) as usize..];
          cv.latest().get_values(next_path)
        }
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

  fn eval_reference(
    &self,
    cvs: &ConfigValues,
    source: &ConfigValue,
    ref_name: &str,
    missing: bool,
  ) -> Option<ConfigValue> {
    let ref_value = source
      .get_value(ref_name)
      .cloned()
      .or_else(|| env::var(ref_name).ok().map(|s| ConfigValue::String(s)));
    if missing {
      if ref_value.is_some() {
        ref_value
      } else {
        cvs.prev_latest().map(Clone::clone)
      }
    } else {
      if ref_value.is_none() {
        panic!("Cannot resolve the reference: {}", ref_name)
      }
      ref_value
    }
  }
}
