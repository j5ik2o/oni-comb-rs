use crate::models::authority::Authority;
use crate::models::hier_part::HierPart;
use crate::models::host_name::HostName;
use crate::models::path::Path;
use crate::models::query::Query;
use crate::models::scheme::Scheme;
use crate::models::user_info::UserInfo;
use crate::parsers::uri_parsers;
use oni_comb_parser_rs::prelude::{ParseError, ParserRunner};
use std::fmt::Formatter;

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

  pub fn user_info(&self) -> Option<&UserInfo> {
    self.authority().and_then(|a| a.user_info())
  }

  pub fn path(&self) -> Option<&Path> {
    self.hier_path.as_ref().map(|h| &h.path)
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
    let s = "http://user1:pass1@localhost:8080/example?key1=value1&key2=value2&key1=value2#f1";
    match Uri::parse(s) {
      Ok(uri) => {
        uri.schema().into_iter().for_each(|s| assert_eq!(s.to_string(), "http"));
        uri
          .host_name()
          .into_iter()
          .for_each(|hn| assert_eq!(hn.to_string(), "localhost"));
        uri.port().into_iter().for_each(|p| assert_eq!(p, 8080));
        uri.user_info().into_iter().for_each(|ui| {
          assert_eq!(ui.user_name(), "user1");
          assert_eq!(ui.password(), Some("pass1"));
        });
        uri
          .path()
          .into_iter()
          .for_each(|p| assert_eq!(p.to_string(), "/example"));
        uri.query().into_iter().for_each(|q| {
          q.get_param("key1".to_string()).into_iter().for_each(|v| {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0], "value1");
            assert_eq!(v[1], "value2");
          });
          q.get_param("key2".to_string()).into_iter().for_each(|v| {
            assert_eq!(v.len(), 1);
            assert_eq!(v[0], "value2");
          });
        });
        uri.fragment().into_iter().for_each(|f| assert_eq!(f, "f1"));
        println!("{:?}", uri);
      }
      Err(e) => println!("{:?}", e),
    }
  }
}
