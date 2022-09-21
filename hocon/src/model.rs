use crate::parsers::hocon;
use oni_comb_parser_rs::prelude::ParserRunner;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigNumberValue {
  SignedLong(i64),
  UnsignedLong(u64),
  Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
  Null,
  Bool(bool),
  String(String),
  Number(ConfigNumberValue),
  Duration(ConfigNumberValue, TimeUnit),
  Array(Vec<ConfigValue>),
  Object(HashMap<String, ConfigValues>),
  Reference(String, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValues {
  Single(ConfigValue),
  Multi(Vec<ConfigValue>),
}

impl ConfigValues {
  pub fn of_single(cv: ConfigValue) -> Self {
    ConfigValues::Single(cv)
  }

  pub fn of_multi(cvs: Vec<ConfigValue>) -> Self {
    ConfigValues::Multi(cvs)
  }

  pub fn push(&mut self, cv: ConfigValue) {
    match self {
      ConfigValues::Single(v) => *self = Self::of_multi(vec![v.clone(), cv]),
      ConfigValues::Multi(v) => v.push(cv),
    }
  }

  pub fn head(&self) -> &ConfigValue {
    match self {
      ConfigValues::Single(v) => v,
      ConfigValues::Multi(v) => v.first().unwrap(),
    }
  }

  pub fn index(&self, idx: usize) -> &ConfigValue {
    match self {
      ConfigValues::Single(v) => v,
      ConfigValues::Multi(v) => &v[idx],
    }
  }

  pub fn last(&self) -> &ConfigValue {
    match self {
      ConfigValues::Single(v) => v,
      ConfigValues::Multi(v) => v.last().unwrap(),
    }
  }

  pub fn last_index(&self) -> usize {
    match self {
      ConfigValues::Single(_v) => 0,
      ConfigValues::Multi(v) => v.len() - 1,
    }
  }

  pub fn prev_last(&self) -> Option<&ConfigValue> {
    match self {
      ConfigValues::Single(_v) => None,
      ConfigValues::Multi(v) => Some(&v[v.len() - 2]),
    }
  }
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
    self.get_values(key).map(|v| v.last())
  }

  pub fn get_values(&self, path: &str) -> Option<&ConfigValues> {
    let keys = path.split(".").collect::<Vec<_>>();
    let key = keys[0];
    let child_count = keys.len() - 1;
    match self {
      ConfigValue::Object(map) => match map.get(key) {
        Some(cv) if child_count > 0 => cv.last().get_values(&path[(key.len() + 1) as usize..]),
        Some(cv) => Some(cv),
        None => None,
      },
      _ => None,
    }
  }

  pub fn contains(&self, key: &str) -> bool {
    match self {
      ConfigValue::Object(map) => map.contains_key(key),
      _ => false,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeUnit {
  Days,
  Hours,
  Microseconds,
  Milliseconds,
  Minutes,
  Nanoseconds,
  Seconds,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigObject {
  Object(HashMap<String, ConfigValues>),
  Array(Vec<ConfigValue>),
  KeyValue(String, ConfigValue),
}

impl ConfigObject {
  pub fn get_value(&self, key: &str) -> Option<&ConfigValue> {
    match self {
      ConfigObject::Object(map) => map.get(key).map(|v| v.last()),
      ConfigObject::Array(array) => array.iter().find(|value| value.contains(key)),
      ConfigObject::KeyValue(k, value) if key == k => Some(value),
      _ => None,
    }
  }

  pub fn contains(&self, key: &str) -> bool {
    match self {
      ConfigObject::Object(map) => map.contains_key(key),
      ConfigObject::Array(array) => array.iter().any(|p| p.contains(key)),
      ConfigObject::KeyValue(k, _value) if key == k => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Config {
  configs: Vec<ConfigObject>,
}

#[derive(Debug)]
pub enum ConfigError {
  FileNotFoundError,
  FileReadError,
  ParseError(String),
}

impl Config {
  pub fn load_from_file(filename: &str) -> Result<Config, ConfigError> {
    let mut f = File::open(filename).map_err(|_| ConfigError::FileNotFoundError)?;
    let mut text = String::new();
    f.read_to_string(&mut text).map_err(|_| ConfigError::FileReadError)?;
    Self::parse_from_string(&text)
  }

  pub fn parse_from_string(text: &str) -> Result<Config, ConfigError> {
    hocon()
      .parse(text.as_bytes())
      .to_result()
      .map(|configs| Self { configs })
      .map_err(|pe| ConfigError::ParseError(pe.to_string()))
  }

  fn eval_reference(&self, cvs: &ConfigValues, ref_name: &str, missing: bool) -> Option<ConfigValue> {
    let ref_value = self
      .get_value(ref_name)
      .or_else(|| env::var(ref_name).ok().map(|s| ConfigValue::String(s)));
    if missing {
      if ref_value.is_some() {
        ref_value
      } else {
        cvs.prev_last().map(Clone::clone)
      }
    } else {
      ref_value
    }
  }

  pub fn get_value(&self, path: &str) -> Option<ConfigValue> {
    let keys = path.split(".").collect::<Vec<_>>();
    let key = keys[0];
    let child_count = keys.len() - 1;
    let config_value = self
      .configs
      .iter()
      .find(|cv| cv.contains(key))
      .and_then(|cv| cv.get_value(key));
    match config_value {
      Some(cv) if child_count > 0 => {
        let next_key = &path[(key.len() + 1) as usize..];
        cv.get_values(next_key).and_then(|cvs| match cvs.last() {
          ConfigValue::Reference(ref_name, missing) => self.eval_reference(cvs, &ref_name, *missing),
          _ => Some(cvs.last().clone()),
        })
      }
      Some(cv) => Some(cv.clone()),
      None => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eval_reference() {
    let input = r#"
    foo {
      bar : "baz",
      bar : "biz",
      test : {
        a: "aaaa",
        a: ${foo.bar} 
      }
    }
    "#;
    let config = Config::parse_from_string(input).unwrap();
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, Some(ConfigValue::String("biz".to_string())));
  }

  #[test]
  #[serial]
  fn test_environment_value_exists() {
    let input = r#"
    foo {
      bar : "baz",
      bar : "biz",
      test : {
        a: "aaaa",
        a: ${TEST_VAR} 
      }
    }
    "#;
    let s = "12345";
    env::set_var("TEST_VAR", s);
    let config = Config::parse_from_string(input).unwrap();
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, Some(ConfigValue::String(s.to_string())));
    env::remove_var("TEST_VAR");
  }

  #[test]
  #[serial]
  fn test_environment_value_not_exists() {
    let input = r#"
    foo {
      bar : "baz",
      bar : "biz",
      test : {
        a: "aaaa",
        a: ${TEST_VAR} 
      }
    }
    "#;
    let config = Config::parse_from_string(input).unwrap();
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, None);
  }

  #[test]
  #[serial]
  fn test_environment_value_exists_fallback() {
    let input = r#"
    foo {
      bar : "baz",
      bar : "biz",
      test : {
        a: "aaaa",
        a: ${?TEST_VAR} 
      }
    }
    "#;
    let s = "12345";
    env::set_var("TEST_VAR", s);
    let config = Config::parse_from_string(input).unwrap();
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, Some(ConfigValue::String(s.to_string())));
    env::remove_var("TEST_VAR");
  }

  #[test]
  #[serial]
  fn test_environment_value_not_exists_fallback() {
    let input = r#"
    foo {
      bar : "baz",
      bar : "biz",
      test : {
        a: "aaaa",
        a: ${?TEST_VAR} 
      }
    }
    "#;
    let config = Config::parse_from_string(input).unwrap();
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, Some(ConfigValue::String("aaaa".to_string())));
  }
}
