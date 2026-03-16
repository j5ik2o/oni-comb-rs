use core::fmt;

use crate::models::host::Host;
use crate::models::user_info::UserInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Authority<'a> {
  pub(crate) user_info: Option<UserInfo<'a>>,
  pub(crate) host: Host<'a>,
  pub(crate) port: Option<u16>,
}

impl<'a> Authority<'a> {
  pub fn new(user_info: Option<UserInfo<'a>>, host: Host<'a>, port: Option<u16>) -> Self {
    Self { user_info, host, port }
  }

  pub fn user_info(&self) -> Option<&UserInfo<'a>> {
    self.user_info.as_ref()
  }

  pub fn host(&self) -> &Host<'a> {
    &self.host
  }

  pub fn port(&self) -> Option<u16> {
    self.port
  }
}

impl fmt::Display for Authority<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(ref ui) = self.user_info {
      write!(f, "{}@", ui)?;
    }
    write!(f, "{}", self.host)?;
    if let Some(port) = self.port {
      write!(f, ":{}", port)?;
    }
    Ok(())
  }
}
