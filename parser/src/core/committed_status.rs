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
  /// コミット済みかどうかを返す。
  pub fn is_committed(&self) -> bool {
    match self {
      CommittedStatus::Committed => true,
      CommittedStatus::Uncommitted => false,
    }
  }

  /// アンコミット済みかどうかを返す。
  pub fn is_uncommitted(&self) -> bool {
    !self.is_committed()
  }

  /// [CommittedStatus]を合成します。
  ///
  /// どちらか一方がコミット済みであれば、それを返します。
  /// そうでなければ、アンコミット済みを返します。
  pub fn or(&self, other: &Self) -> Self {
    match (self, other) {
      (CommittedStatus::Committed, _) => CommittedStatus::Committed,
      (_, CommittedStatus::Committed) => CommittedStatus::Committed,
      _ => CommittedStatus::Uncommitted,
    }
  }
}
