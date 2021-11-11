use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct UserInfo {
  user_name: String,
  password: Option<String>,
}

impl Default for UserInfo {
  fn default() -> Self {
    UserInfo {
      user_name: String::default(),
      password: Option::default(),
    }
  }
}

impl std::fmt::Display for UserInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}",
      self.user_name(),
      self
        .password
        .iter()
        .map(|s| format!(":{}", s))
        .fold("".to_string(), |mut acc, s| {
          acc.push_str(&s);
          acc
        })
    )
  }
}

impl From<(&str, Option<&str>)> for UserInfo {
  fn from((user_name, password): (&str, Option<&str>)) -> Self {
    Self::new(user_name.to_string(), password.map(|s| s.to_string()))
  }
}

impl UserInfo {
  pub fn new(user_name: String, password: Option<String>) -> Self {
    Self { user_name, password }
  }

  pub fn user_name(&self) -> &str {
    &self.user_name
  }

  pub fn password(&self) -> Option<&String> {
    self.password.as_ref()
  }
}
