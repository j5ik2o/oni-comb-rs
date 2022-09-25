use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigIncludeValue {
  method: String,
  pub(crate) file_name: String,
}

impl Display for ConfigIncludeValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "include {}({})", self.method, self.file_name)
  }
}

impl ConfigIncludeValue {
  pub fn new(method: String, file_name: String) -> Self {
    Self { method, file_name }
  }
}
