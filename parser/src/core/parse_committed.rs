#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ParseCommittedStatus {
  Committed,
  Uncommitted,
}

impl From<bool> for ParseCommittedStatus {
  fn from(value: bool) -> Self {
    if value {
      ParseCommittedStatus::Committed
    } else {
      ParseCommittedStatus::Uncommitted
    }
  }
}

impl ParseCommittedStatus {
  pub fn to_bool(self) -> bool {
    match self {
      ParseCommittedStatus::Committed => true,
      ParseCommittedStatus::Uncommitted => false,
    }
  }

  pub fn or(&self, other: &Self) -> Self {
    match (self, other) {
      (ParseCommittedStatus::Committed, _) => ParseCommittedStatus::Committed,
      (_, ParseCommittedStatus::Committed) => ParseCommittedStatus::Committed,
      _ => ParseCommittedStatus::Uncommitted,
    }
  }
}
