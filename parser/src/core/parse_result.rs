use crate::core::CommittedStatus;
use crate::core::{parse_error::ParseError, ParseState};

#[derive(Debug, Clone)]
pub enum ParseResult<'a, I, A> {
    Success {
        value: A,
        length: usize,
        state: Option<ParseState<'a, I>>,
    },
    Failure {
        error: ParseError<'a, I>,
        committed_status: CommittedStatus,
    },
}

impl<'a, I, A> ParseResult<'a, I, A> {
    pub fn successful(value: A, length: usize) -> Self {
        ParseResult::Success {
            value,
            length,
            state: None,
        }
    }

    pub fn successful_with_state(state: ParseState<'a, I>, value: A, length: usize) -> Self {
        ParseResult::Success {
            value,
            length,
            state: Some(state),
        }
    }

    pub fn failed(error: ParseError<'a, I>, committed_status: CommittedStatus) -> Self {
        ParseResult::Failure {
            error,
            committed_status,
        }
    }

    pub fn failed_with_uncommitted(error: ParseError<'a, I>) -> Self {
        Self::failed(error, CommittedStatus::Uncommitted)
    }

    pub fn failed_with_commit(error: ParseError<'a, I>) -> Self {
        Self::failed(error, CommittedStatus::Committed)
    }

    pub fn to_result(self) -> Result<A, ParseError<'a, I>> {
        match self {
            ParseResult::Success { value, .. } => Ok(value),
            ParseResult::Failure { error, .. } => Err(error),
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, ParseResult::Success { .. })
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, ParseResult::Failure { .. })
    }

    pub fn success(self) -> Option<A> {
        match self {
            ParseResult::Success { value, .. } => Some(value),
            _ => None,
        }
    }

    pub fn failure(self) -> Option<ParseError<'a, I>> {
        match self {
            ParseResult::Failure { error, .. } => Some(error),
            _ => None,
        }
    }

    pub fn committed_status(&self) -> Option<CommittedStatus> {
        match self {
            ParseResult::Failure {
                committed_status, ..
            } => Some(*committed_status),
            _ => None,
        }
    }

    pub fn with_uncommitted(self) -> Self {
        match self {
            ParseResult::Failure { error, .. } => Self::failed(error, CommittedStatus::Uncommitted),
            _ => self,
        }
    }

    pub fn add_commit(self, is_committed: bool) -> Self {
        match self {
            ParseResult::Failure {
                error,
                committed_status,
            } => Self::failed(error, committed_status.or(is_committed.into())),
            _ => self,
        }
    }

    pub fn map<B, F>(self, f: F) -> ParseResult<'a, I, B>
    where
        F: FnOnce(A) -> B,
    {
        match self {
            ParseResult::Success {
                value,
                length,
                state,
            } => ParseResult::Success {
                value: f(value),
                length,
                state,
            },
            ParseResult::Failure {
                error,
                committed_status,
            } => ParseResult::Failure {
                error,
                committed_status,
            },
        }
    }

    pub fn map_err<F>(self, f: F) -> Self
    where
        F: FnOnce(ParseError<'a, I>) -> ParseError<'a, I>,
    {
        match self {
            ParseResult::Failure {
                error,
                committed_status,
            } => Self::failed(f(error), committed_status),
            success => success,
        }
    }

    pub fn advance_success(self, n: usize) -> Self {
        match self {
            ParseResult::Success {
                value,
                length,
                state,
            } => ParseResult::Success {
                value,
                length: length + n,
                state: state.map(|s| s.advance_by(n)),
            },
            failure => failure,
        }
    }

    pub fn with_state(self, state: ParseState<'a, I>) -> Self {
        match self {
            ParseResult::Success { value, length, .. } => ParseResult::Success {
                value,
                length,
                state: Some(state),
            },
            failure => failure,
        }
    }
}
