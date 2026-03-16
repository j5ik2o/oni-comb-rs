use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserInfo<'a> {
  pub(crate) user_name: &'a str,
  pub(crate) password: Option<&'a str>,
}

impl<'a> UserInfo<'a> {
  pub fn new(user_name: &'a str, password: Option<&'a str>) -> Self {
    Self { user_name, password }
  }

  pub fn user_name(&self) -> &'a str {
    self.user_name
  }

  pub fn password(&self) -> Option<&'a str> {
    self.password
  }
}

impl fmt::Display for UserInfo<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.user_name)?;
    if let Some(pw) = self.password {
      write!(f, ":{}", pw)?;
    }
    Ok(())
  }
}
