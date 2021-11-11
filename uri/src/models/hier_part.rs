use crate::models::authority::Authority;
use crate::models::path::Path;

use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HierPart {
  pub(crate) authority: Option<Authority>,
  pub(crate) path: Path,
}

impl std::fmt::Display for HierPart {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}",
      self
        .authority
        .as_ref()
        .map(|e| format!("//{}", e.to_string()))
        .unwrap_or("".to_string()),
      self.path.to_string()
    )
  }
}

impl HierPart {
  pub fn new(authority: Option<Authority>, path: Path) -> HierPart {
    HierPart { authority, path }
  }

  pub fn of_path(path: Path) -> HierPart {
    HierPart { authority: None, path }
  }
}

impl Default for HierPart {
  fn default() -> HierPart {
    HierPart {
      authority: None,
      path: Path::default(),
    }
  }
}
