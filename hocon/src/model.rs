use std::fs::File;
use std::io::Read;

use oni_comb_parser_rs::prelude::ParserRunner;

use crate::model::config_value::ConfigValue;
use crate::parsers::hocon;

pub mod config_array_value;
pub mod config_duration_value;
pub mod config_number_value;
pub mod config_object_value;
pub mod config_value;
pub mod config_values;
pub mod time_unit;

#[derive(Debug)]
pub enum ConfigError {
  FileNotFoundError,
  FileReadError,
  ParseError(String),
}

pub trait FileReader {
  fn read_to_string(&mut self, filename: &str, text: &mut String) -> Result<(), ConfigError>;
}

pub struct DefaultFileReader;

impl FileReader for DefaultFileReader {
  fn read_to_string(&mut self, filename: &str, text: &mut String) -> Result<(), ConfigError> {
    let mut file = File::open(filename).map_err(|_| ConfigError::FileNotFoundError)?;
    file.read_to_string(text).map_err(|_| ConfigError::FileReadError)?;
    Ok(())
  }
}

pub struct ConfigFactory {
  file_reader: Box<dyn FileReader>,
}

impl ConfigFactory {
  pub fn new() -> Self {
    Self {
      file_reader: Box::new(DefaultFileReader),
    }
  }

  pub fn load_from_file(&mut self, filename: &str) -> Result<Config, ConfigError> {
    let mut text = String::new();
    let _ = self.file_reader.read_to_string(filename, &mut text);
    Self::parse_from_string(&text)
  }

  pub fn parse_from_string(text: &str) -> Result<Config, ConfigError> {
    hocon()
      .parse(text.as_bytes())
      .to_result()
      .map(|configs| {
        let mut cur = configs[0].clone();
        cur.resolve(None, None);
        for cv in &configs[1..] {
          let mut t = cv.clone();
          t.resolve(None, None);
          cur.with_fallback(t);
        }
        cur
      })
      .map(|config| {
        let mut c = config.clone();
        c.resolve(Some(&config), None);
        c
      })
      .map(|config| Config { config })
      .map_err(|pe| ConfigError::ParseError(pe.to_string()))
  }
}

#[derive(Debug, Clone)]
pub struct Config {
  config: ConfigValue,
}

impl Config {
  pub fn to_config_value(&self) -> &ConfigValue {
    &self.config
  }

  pub fn get_value(&self, path: &str) -> Option<ConfigValue> {
    let keys = path.split(".").collect::<Vec<_>>();
    let key = keys[0];
    let child_count = keys.len() - 1;
    let config_value = self.config.get_value(key);
    match config_value {
      Some(cv) if child_count > 0 => {
        let next_key = &path[(key.len() + 1) as usize..];
        cv.get_value(next_key).cloned()
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
  fn test_simple() {
    let input = r#"
    foo {
      bar = "baz"
      test {
        a = "aaaa"
      }
    }
    foo {
      test {
        b = "bbbb"
        b = "xxxx"
      }
    }
    "#;
    let config = ConfigFactory::parse_from_string(input).unwrap();
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, Some(ConfigValue::String("aaaa".to_string())));
    let b_value = config.get_value("foo.test.b");
    assert_eq!(b_value, Some(ConfigValue::String("xxxx".to_string())));
  }

  #[test]
  fn test_eval_reference() {
    let input = r#"
    foo {
      bar = "baz"
      bar = "biz"
      test {
        a = "aaaa"
        a = ${foo.bar} 
      }
    }
    foo {
      test {
        b = "bbbb"
      }
    }
    "#;
    let config = ConfigFactory::parse_from_string(input).unwrap();
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
        a = "aaaa",
        a = ${TEST_VAR} 
      }
    }
    "#;
    let s = "12345";
    env::set_var("TEST_VAR", s);
    let config = ConfigFactory::parse_from_string(input).unwrap();
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
      bar = "baz",
      bar = "biz",
      test {
        a = "aaaa",
        a = ${TEST_VAR} 
      }
    }
    "#;
    let config = ConfigFactory::parse_from_string(input).unwrap();
    let _ = config.get_value("foo.test.a");
  }

  #[test]
  #[serial]
  fn test_environment_value_exists_fallback() {
    let input = r#"
    foo {
      bar = "baz"
      bar = "biz"
      test {
        a = "aaaa"
        a = ${?TEST_VAR} 
      }
    }
    "#;
    let s = "12345";
    env::set_var("TEST_VAR", s);
    let config = ConfigFactory::parse_from_string(input).unwrap();
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, Some(ConfigValue::String(s.to_string())));
    env::remove_var("TEST_VAR");
  }

  #[test]
  #[serial]
  fn test_environment_value_not_exists_fallback() {
    let input = r#"
    foo {
      bar = "baz"
      bar = "biz"
      test {
        a = "aaaa"
        a = ${?TEST_VAR} 
      }
    }
    "#;
    let config = ConfigFactory::parse_from_string(input).unwrap();
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, Some(ConfigValue::String("aaaa".to_string())));
  }
}
