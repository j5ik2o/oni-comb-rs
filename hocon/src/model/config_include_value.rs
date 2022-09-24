#[derive(Clone, Debug, PartialEq)]
pub struct ConfigIncludeValue {
  method: String,
  pub(crate) file_name: String,
}

impl ConfigIncludeValue {
  pub fn new(method: String, file_name: String) -> Self {
    Self { method, file_name }
  }
}
