#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum CommittedStatus {
  Committed,
  Uncommitted,
}

impl From<bool> for CommittedStatus {
  fn from(value: bool) -> Self {
    if value {
      CommittedStatus::Committed
    } else {
      CommittedStatus::Uncommitted
    }
  }
}

impl CommittedStatus {
  pub fn to_bool(self) -> bool {
    match self {
      CommittedStatus::Committed => true,
      CommittedStatus::Uncommitted => false,
    }
  }

  pub fn or(&self, other: &Self) -> Self {
    match (self, other) {
      (CommittedStatus::Committed, _) => CommittedStatus::Committed,
      (_, CommittedStatus::Committed) => CommittedStatus::Committed,
      _ => CommittedStatus::Uncommitted,
    }
  }
}
