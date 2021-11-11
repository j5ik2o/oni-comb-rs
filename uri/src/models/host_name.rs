use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HostName(String);

impl Default for HostName {
  fn default() -> Self {
    HostName(String::default())
  }
}

impl std::fmt::Display for HostName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<&str> for HostName {
  fn from(src: &str) -> Self {
    Self(src.to_string())
  }
}

impl From<String> for HostName {
  fn from(src: String) -> Self {
    Self(src)
  }
}

impl HostName {
  pub fn new(value: String) -> Self {
    Self(value)
  }
}
