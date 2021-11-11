use crate::models::host_name::HostName;
use crate::models::user_info::UserInfo;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Authority {
  host_name: HostName,
  port: Option<u16>,
  user_info: Option<UserInfo>,
}

impl Default for Authority {
  fn default() -> Self {
    Authority {
      host_name: HostName::default(),
      port: Option::default(),
      user_info: Option::default(),
    }
  }
}

impl std::fmt::Display for Authority {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}{}",
      self
        .user_info
        .iter()
        .map(|ui| format!("{}@", ui.to_string()))
        .fold("".to_string(), |mut acc, s| {
          acc.push_str(&s);
          acc
        }),
      self.host_name.to_string(),
      self.port.map(|n| format!(":{}", n)).unwrap_or("".to_string()),
    )
  }
}

impl Authority {
  pub fn new(host_name: HostName, port: Option<u16>, user_info: Option<UserInfo>) -> Self {
    Self {
      host_name,
      port,
      user_info,
    }
  }

  pub fn host_name(&self) -> &HostName {
    &self.host_name
  }

  pub fn port(&self) -> Option<u16> {
    self.port
  }

  pub fn user_info(&self) -> Option<&UserInfo> {
    self.user_info.as_ref()
  }
}
