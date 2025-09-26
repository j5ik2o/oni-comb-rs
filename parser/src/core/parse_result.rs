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

    pub fn flat_map<B, F>(self, f: F) -> ParseResult<'a, I, B>
    where
        F: FnOnce(A, usize, Option<ParseState<'a, I>>) -> ParseResult<'a, I, B>,
    {
        match self {
            ParseResult::Success {
                value,
                length,
                state,
            } => {
                let consumed = length > 0;
                let fallback_state = state;
                match f(value, length, state) {
                    ParseResult::Success {
                        value,
                        length: next_length,
                        state: next_state,
                    } => {
                        let combined_length = length + next_length;
                        let combined_state = match (next_state, fallback_state) {
                            (Some(state), _) => Some(state),
                            (None, Some(state)) if next_length > 0 => {
                                Some(state.advance_by(next_length))
                            }
                            (None, Some(state)) => Some(state),
                            (None, None) => None,
                        };
                        ParseResult::Success {
                            value,
                            length: combined_length,
                            state: combined_state,
                        }
                    }
                    ParseResult::Failure {
                        error,
                        committed_status,
                    } => ParseResult::Failure {
                        error,
                        committed_status: committed_status.or(CommittedStatus::from(consumed)),
                    },
                }
            }
            ParseResult::Failure {
                error,
                committed_status,
            } => ParseResult::Failure {
                error,
                committed_status,
            },
        }
    }

    pub fn and_then<B, F>(self, f: F) -> ParseResult<'a, I, B>
    where
        F: FnOnce(A, usize, Option<ParseState<'a, I>>) -> ParseResult<'a, I, B>,
    {
        self.flat_map(f)
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

    pub fn state(&self) -> Option<ParseState<'a, I>> {
        match self {
            ParseResult::Success { state, .. } => *state,
            _ => None,
        }
    }

    pub fn length(&self) -> Option<usize> {
        match self {
            ParseResult::Success { length, .. } => Some(*length),
            _ => None,
        }
    }

    pub fn rest(&self) -> Option<&'a [I]> {
        self.state().map(|state| state.input())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_map_combines_length_and_state() {
        let input = b"abcd";
        let base_state = ParseState::new(input, 0);
        let next_state = base_state.advance_by(2);
        let result = ParseResult::successful_with_state(next_state, 41, 2);

        let combined = result.flat_map(|value, length, state| {
            assert_eq!(value, 41);
            assert_eq!(length, 2);
            let state = state.expect("state must be present");
            assert_eq!(state.current_offset(), 2);
            let final_state = state.advance_by(1);
            ParseResult::successful_with_state(final_state, value + 1, 1)
        });

        match combined {
            ParseResult::Success {
                value,
                length,
                state,
            } => {
                assert_eq!(value, 42);
                assert_eq!(length, 3);
                assert_eq!(state.expect("state").current_offset(), 3);
            }
            _ => panic!("expected success"),
        }
    }

    #[test]
    fn flat_map_failure_marks_commit_when_consumed() {
        let input = b"xyz";
        let initial = ParseState::new(input, 0);
        let consumed = initial.advance_by(1);
        let result = ParseResult::successful_with_state(consumed, (), 1);

        let failure: ParseResult<'_, u8, ()> = result.flat_map(|_, _, _| {
            ParseResult::failed(
                ParseError::of_custom(1, Some(&input[1..]), "failure"),
                CommittedStatus::Uncommitted,
            )
        });

        match failure {
            ParseResult::Failure {
                committed_status, ..
            } => assert!(committed_status.is_committed()),
            _ => panic!("expected failure"),
        }
    }

    #[test]
    fn rest_returns_remaining_input() {
        let input = b"hello";
        let state = ParseState::new(input, 0).advance_by(2);
        let result = ParseResult::successful_with_state(state, (), 2);
        assert_eq!(result.rest(), Some(&input[2..]));
    }
}
