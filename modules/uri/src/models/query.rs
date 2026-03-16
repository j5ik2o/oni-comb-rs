use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Query<'a> {
  pub(crate) raw: &'a str,
  pub(crate) params: Vec<(&'a str, Option<&'a str>)>,
}

impl<'a> Query<'a> {
  pub fn new(raw: &'a str, params: Vec<(&'a str, Option<&'a str>)>) -> Self {
    Self { raw, params }
  }

  pub fn raw(&self) -> &'a str {
    self.raw
  }

  pub fn params(&self) -> &[(&'a str, Option<&'a str>)] {
    &self.params
  }

  pub fn get_param(&self, key: &str) -> Vec<&'a str> {
    self
      .params
      .iter()
      .filter(|(k, _)| *k == key)
      .filter_map(|(_, v)| *v)
      .collect()
  }
}

impl fmt::Display for Query<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.raw)
  }
}
