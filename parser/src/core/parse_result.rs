use crate::core::parse_error::ParseError;

#[derive(Debug, Clone)]
pub enum ParseResult<'a, I, A> {
  Success { get: A, length: usize },
  Failure { get: ParseError<'a, I>, is_committed: bool },
}

impl<'a, I, A> ParseResult<'a, I, A> {
  pub fn successful(get: A, length: usize) -> Self {
    ParseResult::Success { get, length }
  }

  pub fn failed(get: ParseError<'a, I>, is_committed: bool) -> Self {
    ParseResult::Failure { get, is_committed }
  }

  pub fn failed_with_un_commit(get: ParseError<'a, I>) -> Self {
    ParseResult::Failure {
      get,
      is_committed: false,
    }
  }

  pub fn failed_with_commit(get: ParseError<'a, I>) -> Self {
    ParseResult::Failure {
      get,
      is_committed: true,
    }
  }

  pub fn to_result(self) -> Result<A, ParseError<'a, I>> {
    match self {
      ParseResult::Failure { get: e, .. } => Err(e),
      ParseResult::Success { get: a, .. } => Ok(a),
    }
  }

  pub fn is_success(&self) -> bool {
    match self {
      ParseResult::Failure { .. } => false,
      ParseResult::Success { .. } => true,
    }
  }

  pub fn success(self) -> Option<A> {
    match self {
      ParseResult::Failure { .. } => None,
      ParseResult::Success { get: a, .. } => Some(a),
    }
  }

  pub fn failure(self) -> Option<ParseError<'a, I>> {
    match self {
      ParseResult::Failure { get: e, .. } => Some(e),
      ParseResult::Success { .. } => None,
    }
  }

  pub fn is_failure(&self) -> bool {
    match self {
      ParseResult::Failure { .. } => true,
      ParseResult::Success { .. } => false,
    }
  }

  pub fn is_committed(&self) -> Option<bool> {
    match self {
      &ParseResult::Failure { is_committed, .. } => Some(is_committed),
      _ => None,
    }
  }

  pub fn with_un_commit(self) -> Self {
    match self {
      ParseResult::Failure {
        get: e,
        is_committed: true,
      } => ParseResult::Failure {
        get: e,
        is_committed: false,
      },
      _ => self,
    }
  }

  pub fn with_committed_fallback(self, is_committed: bool) -> Self {
    match self {
      ParseResult::Failure {
        get: e,
        is_committed: c,
      } => ParseResult::Failure {
        get: e,
        is_committed: c || is_committed,
      },
      _ => self,
    }
  }

  pub fn map_err<F>(self, f: F) -> Self
  where
    F: Fn(ParseError<'a, I>) -> ParseError<'a, I>, {
    match self {
      ParseResult::Failure {
        get: e,
        is_committed: c,
      } => ParseResult::Failure {
        get: f(e),
        is_committed: c,
      },
      _ => self,
    }
  }

  pub fn with_add_length(self, n: usize) -> Self {
    match self {
      ParseResult::Success { get: a, length: m } => ParseResult::Success { get: a, length: n + m },
      _ => self,
    }
  }
}
