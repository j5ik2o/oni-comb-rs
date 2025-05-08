/// A structure representing the commit status of the parser.<br/>
/// パーサのコミット状態を表す構造体。
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
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
  /// Returns whether committed or not.
  pub fn is_committed(&self) -> bool {
    match self {
      CommittedStatus::Committed => true,
      CommittedStatus::Uncommitted => false,
    }
  }

  /// Returns whether uncommitted or not.
  pub fn is_uncommitted(&self) -> bool {
    !self.is_committed()
  }

  /// Compose [CommittedStatus].
  ///
  /// If either one is already committed, it returns it. Otherwise, it returns uncommitted.
  pub fn or(&self, other: Self) -> Self {
    match (self, other) {
      (CommittedStatus::Committed, _) => CommittedStatus::Committed,
      (_, CommittedStatus::Committed) => CommittedStatus::Committed,
      _ => CommittedStatus::Uncommitted,
    }
  }
}
