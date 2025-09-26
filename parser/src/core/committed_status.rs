#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommittedStatus {
    Committed,
    Uncommitted,
}

impl CommittedStatus {
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (CommittedStatus::Committed, _) | (_, CommittedStatus::Committed) => {
                CommittedStatus::Committed
            }
            _ => CommittedStatus::Uncommitted,
        }
    }
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
