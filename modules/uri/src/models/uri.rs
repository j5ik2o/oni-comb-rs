use core::fmt;

use crate::models::authority::Authority;
use crate::models::host::Host;
use crate::models::path::Path;
use crate::models::query::Query;
use crate::models::user_info::UserInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri<'a> {
  pub(crate) scheme: Option<&'a str>,
  pub(crate) authority: Option<Authority<'a>>,
  pub(crate) path: Path<'a>,
  pub(crate) query: Option<Query<'a>>,
  pub(crate) fragment: Option<&'a str>,
}

impl<'a> Uri<'a> {
  pub fn parse(input: &'a str) -> Result<Self, String> {
    crate::parsers::uri::parse_uri(input)
  }

  pub fn scheme(&self) -> Option<&'a str> {
    self.scheme
  }

  pub fn authority(&self) -> Option<&Authority<'a>> {
    self.authority.as_ref()
  }

  pub fn host(&self) -> Option<&Host<'a>> {
    self.authority.as_ref().map(|a| &a.host)
  }

  pub fn port(&self) -> Option<u16> {
    self.authority.as_ref().and_then(|a| a.port)
  }

  pub fn user_info(&self) -> Option<&UserInfo<'a>> {
    self.authority.as_ref().and_then(|a| a.user_info.as_ref())
  }

  pub fn path(&self) -> &Path<'a> {
    &self.path
  }

  pub fn query(&self) -> Option<&Query<'a>> {
    self.query.as_ref()
  }

  pub fn query_params(&self) -> &[(&'a str, Option<&'a str>)] {
    match &self.query {
      Some(q) => q.params(),
      None => &[],
    }
  }

  pub fn fragment(&self) -> Option<&'a str> {
    self.fragment
  }

  // --- URN support ---

  pub fn is_urn(&self) -> bool {
    self.scheme.map(|s| s.eq_ignore_ascii_case("urn")).unwrap_or(false)
  }

  pub fn urn_nid(&self) -> Option<&'a str> {
    if !self.is_urn() {
      return None;
    }
    let path_str = self.path_as_str()?;
    path_str.split_once(':').map(|(nid, _)| nid)
  }

  pub fn urn_nss(&self) -> Option<&'a str> {
    if !self.is_urn() {
      return None;
    }
    let path_str = self.path_as_str()?;
    path_str.split_once(':').map(|(_, nss)| nss)
  }

  fn path_as_str(&self) -> Option<&'a str> {
    match &self.path {
      Path::Rootless(segs) if !segs.is_empty() => Some(segs[0]),
      _ => None,
    }
  }
}

impl fmt::Display for Uri<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(scheme) = self.scheme {
      write!(f, "{}:", scheme)?;
    }
    if let Some(ref auth) = self.authority {
      write!(f, "//{}", auth)?;
    }
    write!(f, "{}", self.path)?;
    if let Some(ref q) = self.query {
      write!(f, "?{}", q)?;
    }
    if let Some(frag) = self.fragment {
      write!(f, "#{}", frag)?;
    }
    Ok(())
  }
}
