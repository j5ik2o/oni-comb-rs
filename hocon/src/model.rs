use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

use oni_comb_parser_rs::prelude::ParserRunner;

use crate::model::config_value::ConfigValue;
use crate::parsers::hocon;

pub mod config_array_value;
pub mod config_duration_value;
pub mod config_include_value;
pub mod config_number_value;
pub mod config_object_value;
pub mod config_value;
pub mod config_value_link;
pub mod time_unit;

#[derive(Debug)]
pub enum ConfigError {
  FileNotFoundError,
  FileReadError,
  ParseError(String),
}

pub trait Monoid {
  fn combine(&mut self, other: &Self);
}

pub trait ConfigMergeable {
  fn merge_with(&mut self, other: Self);
}

pub trait ConfigResolver {
  fn resolve(&mut self, source: Option<&Self>);
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
      .map(|config_values| Self::resolve_stage0(&config_values))
      .map(|config_value| Self::resolve_stage1(&config_value))
      .map(|config_value| Config { config_value })
      .map_err(|pe| ConfigError::ParseError(pe.to_string()))
  }

  fn resolve_stage1(config_value: &ConfigValue) -> ConfigValue {
    let mut c = config_value.clone();
    c.resolve(Some(&config_value));
    c
  }

  fn resolve_stage0(config_values: &Vec<ConfigValue>) -> ConfigValue {
    let mut cur = config_values[0].clone();
    cur.resolve(None);
    for cv in &config_values[1..] {
      let mut t = cv.clone();
      t.resolve(None);
      cur.merge_with(t);
    }
    cur
  }
}

#[derive(Debug, Clone)]
pub struct Config {
  config_value: ConfigValue,
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.config_value.to_string())
  }
}

impl Config {
  fn new(config_value: ConfigValue) -> Self {
    Self { config_value }
  }

  pub fn to_config_value(&self) -> &ConfigValue {
    &self.config_value
  }

  pub fn get_value(&self, path: &str) -> Option<&ConfigValue> {
    self.config_value.get_value(path)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::try_init();
  }

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
        b = "aaaa"
        b = "xxxx"
      }
    }
    "#;
    let config = ConfigFactory::parse_from_string(input).unwrap();
    println!("{:?}", config);
    let a_value = config.get_value("foo.test.a");
    assert_eq!(a_value, Some(&ConfigValue::String("aaaa".to_string())));
    let b_value = config.get_value("foo.test.b");
    assert_eq!(b_value, Some(&ConfigValue::String("xxxx".to_string())));
  }

  #[test]
  fn path_as_key() {
    let input = r#"
        x.y.a=1s
        x.y {
          c=3
          c.d.e = 5
        }
        x.y.b=[2.1, 10, 30]
        x.x.x="a"
        "#;
    let config = ConfigFactory::parse_from_string(input).unwrap();
    println!("{}", config);
    let x_value = config.get_value("x.x.x").unwrap();
    assert_eq!(x_value, &ConfigValue::String("a".to_string()));
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
    assert_eq!(a_value, Some(&ConfigValue::String("biz".to_string())));
    let b_value = config.get_value("foo.test.b");
    assert_eq!(b_value, Some(&ConfigValue::String("bbbb".to_string())));
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
    assert_eq!(a_value, Some(&ConfigValue::String(s.to_string())));
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
    assert_eq!(a_value, Some(&ConfigValue::String(s.to_string())));
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
    assert_eq!(a_value, Some(&ConfigValue::String("aaaa".to_string())));
  }
}
