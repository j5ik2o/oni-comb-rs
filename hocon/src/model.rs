use std::env;
use std::fs::File;
use std::io::Read;

use oni_comb_parser_rs::prelude::ParserRunner;

use crate::model::config_value::ConfigValue;
use crate::model::config_values::ConfigValues;
use crate::parsers::hocon;

pub mod config_array_value;
pub mod config_duration_value;
pub mod config_number_value;
pub mod config_object_value;
pub mod config_value;
pub mod config_values;
pub mod time_unit;

#[derive(Debug, Clone)]
pub struct Config {
  config: ConfigValue,
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
      .map(|configs| {
        let mut cur = configs[0].clone();
        for cv in &configs[1..] {
          cur.with_fallback(cv.clone());
        }
        cur
      })
      .map(|config| Self { config })
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
        cvs.prev_latest().map(Clone::clone)
      }
    } else {
      if ref_value.is_none() {
        panic!("Cannot resolve the reference: {}", ref_name)
      }
      ref_value
    }
  }

  pub fn get_value(&self, path: &str) -> Option<ConfigValue> {
    let keys = path.split(".").collect::<Vec<_>>();
    let key = keys[0];
    let child_count = keys.len() - 1;
    let config_value = self.config.get_value(key);
    match config_value {
      Some(cv) if child_count > 0 => {
        let next_key = &path[(key.len() + 1) as usize..];
        cv.get_values(next_key).and_then(|cvs| match cvs.latest() {
          ConfigValue::Reference(ref_name, missing) => self.eval_reference(cvs, &ref_name, *missing),
          _ => Some(cvs.latest().clone()),
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
      bar: "baz"
      bar: "biz"
      test {
        a: "aaaa"
        a: ${foo.bar} 
      }
    }
    foo {
      test {
        b: "bbbb"
      }
    }
    "#;
    let config = Config::parse_from_string(input).unwrap();
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, Some(ConfigValue::String("biz".to_string())));
    let b_value = config.get_value("foo.test.b");
    assert_eq!(b_value, Some(ConfigValue::String("bbbb".to_string())));
  }

  #[test]
  #[serial]
  fn test_environment_value_exists() {
    let input = r#"
    foo {
      bar = "baz",
      bar = "biz",
      test {
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
  #[should_panic]
  #[serial]
  fn test_environment_value_not_exists() {
    let input = r#"
    foo {
      bar : "baz",
      bar : "biz",
      test {
        a: "aaaa",
        a: ${TEST_VAR} 
      }
    }
    "#;
    let config = Config::parse_from_string(input).unwrap();
    let _ = config.get_value("foo.test.a");
  }

  #[test]
  #[serial]
  fn test_environment_value_exists_fallback() {
    let input = r#"
    foo {
      bar : "baz",
      bar : "biz",
      test {
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
      test {
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
