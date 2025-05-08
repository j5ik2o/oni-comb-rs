use crate::core::parse_error::ParseError;
use crate::core::CommittedStatus;

/// The enum type representing the parse result.
#[derive(Debug, Clone)]
pub enum ParseResult<'a, I, A> {
  /// Success.<br/>
  Success {
    /// The value when success.
    value: A,
    /// The size of the value.
    length: usize,
  },
  /// Failure.
  Failure {
    /// The cause when failure.
    error: ParseError<'a, I>,
    /// The commit status.
    committed_status: CommittedStatus,
  },
}

impl<'a, I, A> ParseResult<'a, I, A> {
  /// Returns the parse result of success.
  ///
  /// - value: a value
  /// - length: a size of the value
  pub fn successful(value: A, length: usize) -> Self {
    ParseResult::Success { value, length }
  }

  /// Returns the parse result of failure.
  ///
  /// - error: a [ParsedError]
  /// - committed_status: a [CommittedStatus]
  pub fn failed(error: ParseError<'a, I>, committed_status: CommittedStatus) -> Self {
    ParseResult::Failure {
      error,
      committed_status,
    }
  }

  /// Returns the parse result of failure.
  ///
  /// - error: a [ParsedError]
  pub fn failed_with_uncommitted(error: ParseError<'a, I>) -> Self {
    Self::failed(error, CommittedStatus::Uncommitted)
  }

  pub fn failed_with_commit(error: ParseError<'a, I>) -> Self {
    Self::failed(error, CommittedStatus::Committed)
  }

  /// Convert [ParsedResult] to [Result].
  pub fn to_result(self) -> Result<A, ParseError<'a, I>> {
    match self {
      ParseResult::Failure { error, .. } => Err(error),
      ParseResult::Success { value, .. } => Ok(value),
    }
  }

  /// Returns whether the parsing was successful or not.
  pub fn is_success(&self) -> bool {
    match self {
      ParseResult::Failure { .. } => false,
      ParseResult::Success { .. } => true,
    }
  }

  /// Return the results of a successful parsing.
  pub fn success(self) -> Option<A> {
    match self {
      ParseResult::Failure { .. } => None,
      ParseResult::Success { value, .. } => Some(value),
    }
  }

  /// Returns whether the parsing has failed or not.
  pub fn is_failure(&self) -> bool {
    match self {
      ParseResult::Failure { .. } => true,
      ParseResult::Success { .. } => false,
    }
  }

  /// Return the result of the failed parsing.
  pub fn failure(self) -> Option<ParseError<'a, I>> {
    match self {
      ParseResult::Failure { error, .. } => Some(error),
      ParseResult::Success { .. } => None,
    }
  }

  /// Return the committed status.
  pub fn committed_status(&self) -> Option<CommittedStatus> {
    match self {
      ParseResult::Failure {
        committed_status: is_committed,
        ..
      } => Some(*is_committed),
      _ => None,
    }
  }

  /// Forces a parse result to have an uncommitted status.
  pub fn with_uncommitted(self) -> Self {
    match self {
      ParseResult::Failure {
        error,
        committed_status: CommittedStatus::Committed,
      } => ParseResult::Failure {
        error,
        committed_status: CommittedStatus::Uncommitted,
      },
      _ => self,
    }
  }

  /// Creates a new `ParseResult` with a fallback committed status.
  pub fn add_commit(self, is_committed: bool) -> Self {
    match self {
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::Failure {
        error,
        committed_status: committed_status.or(is_committed.into()),
      },
      _ => self,
    }
  }

  pub fn flat_map<B, F>(self, f: F) -> ParseResult<'a, I, B>
  where
    F: Fn(A, usize) -> ParseResult<'a, I, B>, {
    match self {
      ParseResult::Success { value, length } => f(value, length),
      ParseResult::Failure {
        error: e,
        committed_status: c,
      } => ParseResult::Failure {
        error: e,
        committed_status: c,
      },
    }
  }

  pub fn map<B, F>(self, f: F) -> ParseResult<'a, I, B>
  where
    F: Fn(A, usize) -> (B, usize), {
    self.flat_map(|value, length| {
      let (v, l) = f(value, length);
      ParseResult::successful(v, l)
    })
  }

  pub fn map_err<F>(mut self, f: F) -> Self
  where
    F: Fn(&ParseError<'a, I>) -> ParseError<'a, I>, {
    if let  ParseResult::Failure { error, .. } = &mut self {
      *error = f(error);
    }
    self

    // match self {
    //   ParseResult::Failure {
    //     error: e,
    //     committed_status: c,
    //   } => ParseResult::Failure {
    //     error: f(e),
    //     committed_status: c,
    //   },
    //   _ => self,
    // }
  }

  pub fn advance_success(mut self, n: usize) -> Self {
    if let ParseResult::Success { length, .. } = &mut self {
      *length += n;
    }
    self
  }
}
