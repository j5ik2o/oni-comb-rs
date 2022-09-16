use crate::models::authority::Authority;
use crate::models::hier_part::HierPart;
use crate::models::path::Path;
use crate::models::query::Query;
use crate::models::scheme::Scheme;
use crate::parsers::uri_parsers;
use oni_comb_parser_rs::prelude::{ParseError, ParserRunner};
use std::fmt::Formatter;
use crate::models::host_name::HostName;

pub type Fragment = String;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Uri {
  schema: Option<Scheme>,
  hier_path: Option<HierPart>,
  query: Option<Query>,
  fragment: Option<String>,
}

impl Default for Uri {
  fn default() -> Self {
    Uri {
      schema: Option::default(),
      hier_path: Option::default(),
      query: Option::default(),
      fragment: Option::default(),
    }
  }
}

impl std::fmt::Display for Uri {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}:{}{}{}{}",
      self.schema.as_ref().map(|s| s.to_string()).unwrap_or("".to_string()),
      self
        .hier_path
        .as_ref()
        .map(|hp| hp
          .authority
          .as_ref()
          .map(|a| format!("//{}", a.to_string()))
          .unwrap_or("".to_string()),)
        .unwrap_or("".to_string()),
      self
        .hier_path
        .as_ref()
        .map(|hp| hp.path.to_string())
        .unwrap_or("".to_string()),
      self
        .query
        .as_ref()
        .map(|q| format!("?{}", q.to_string()))
        .unwrap_or("".to_string()),
      self
        .fragment
        .as_ref()
        .map(|s| format!("#{}", s))
        .unwrap_or("".to_string())
    )
  }
}

impl Uri {
  pub fn parse(text: &str) -> Result<Uri, ParseError<u8>> {
    uri_parsers::uri().parse(text.as_bytes()).to_result()
  }

  pub fn new(
    schema: Option<Scheme>,
    hier_path: Option<HierPart>,
    query: Option<Query>,
    fragment: Option<Fragment>,
  ) -> Self {
    Self {
      schema,
      hier_path,
      query,
      fragment,
    }
  }

  pub fn schema(&self) -> Option<&Scheme> {
    self.schema.as_ref()
  }

  pub fn authority(&self) -> Option<&Authority> {
    match self.hier_path {
      Some(ref hp) => hp.authority.as_ref(),
      None => None,
    }
  }

  pub fn host_name(&self) -> Option<&HostName> {
    self.authority().map(|a| a.host_name())
  }

  pub fn port(&self) -> Option<u16> {
    self.authority().and_then(|a| a.port())
  }

  pub fn path(&self) -> Option<&Path> {
    match self.hier_path {
      Some(ref hp) => {
        if hp.path.is_empty() {
          None
        } else {
          Some(&hp.path)
        }
      }
      None => None,
    }
  }

  pub fn query(&self) -> Option<&Query> {
    self.query.as_ref()
  }

  pub fn fragment(&self) -> Option<&Fragment> {
    self.fragment.as_ref()
  }

  pub fn is_absolute(&self) -> bool {
    self.fragment.is_none()
  }
}

#[cfg(test)]
mod test {
  use std::env;

  use crate::models::uri::Uri;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_parse() {
    init();
    let s = "://user1:pass1@localhost:8080/example?key1=value1&key2=value2&key1=value2#f1";
    match Uri::parse(s) {
      Ok(uri) => println!("{:?}", uri),
      Err(e) => println!("{:?}", e),
    }
  }
}
