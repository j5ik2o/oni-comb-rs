use crate::core::parse_error::ParseError;
use crate::core::CommittedStatus;

/// A Parse Result.<br/>
/// 解析結果。
#[derive(Debug, Clone)]
pub enum ParseResult<'a, I, A> {
  /// 成功
  Success {
    /// 成功の値
    value: A,
    /// valueのサイズ
    length: usize,
  },
  /// 失敗
  Failure {
    /// 失敗の原因
    error: ParseError<'a, I>,
    /// コミット状態
    committed_status: CommittedStatus,
  },
}

impl<'a, I, A> ParseResult<'a, I, A> {
  /// Returns the parse result of success.<br/>
  /// 成功の解析結果を返します。
  ///
  /// - value: a value
  /// - length: a size of the value
  ///
  /// - value: 値
  /// - length: 値のサイズ
  pub fn successful(value: A, length: usize) -> Self {
    ParseResult::Success { value, length }
  }

  /// Returns the parse result of failure.<br/>
  /// 失敗の解析結果を返します。
  ///
  /// - error: a [ParsedError]
  /// - committed_status: a [CommittedStatus]
  pub fn failed(error: ParseError<'a, I>, committed_status: CommittedStatus) -> Self {
    ParseResult::Failure {
      error,
      committed_status,
    }
  }

  /// Returns the parse result of failure.<br/>
  /// 失敗の解析結果を返します。
  ///
  /// - error: a [ParsedError]
  pub fn failed_with_uncommitted(error: ParseError<'a, I>) -> Self {
    Self::failed(error, CommittedStatus::Uncommitted)
  }

  pub fn failed_with_commit(error: ParseError<'a, I>) -> Self {
    Self::failed(error, CommittedStatus::Committed)
  }

  /// Convert [ParsedResult] to [Result].<br/>
  /// [ParsedResult]を[Result]に変換する。
  pub fn to_result(self) -> Result<A, ParseError<'a, I>> {
    match self {
      ParseResult::Failure { error, .. } => Err(error),
      ParseResult::Success { value, .. } => Ok(value),
    }
  }

  /// Returns whether the parsing was successful or not.<br/>
  /// 解析が成功したかどうかを返す。
  pub fn is_success(&self) -> bool {
    match self {
      ParseResult::Failure { .. } => false,
      ParseResult::Success { .. } => true,
    }
  }

  /// Return the results of a successful parsing.<br/>
  /// 成功した解析結果を返す。
  pub fn success(self) -> Option<A> {
    match self {
      ParseResult::Failure { .. } => None,
      ParseResult::Success { value, .. } => Some(value),
    }
  }

  /// Returns whether the parsing has failed or not.<br/>
  /// 解析が失敗したかどうかを返す。
  pub fn is_failure(&self) -> bool {
    match self {
      ParseResult::Failure { .. } => true,
      ParseResult::Success { .. } => false,
    }
  }

  /// Return the result of the failed parsing.<br/>
  /// 失敗した解析結果を返す。
  pub fn failure(self) -> Option<ParseError<'a, I>> {
    match self {
      ParseResult::Failure { error, .. } => Some(error),
      ParseResult::Success { .. } => None,
    }
  }

  pub fn committed_status(&self) -> Option<CommittedStatus> {
    match self {
      ParseResult::Failure {
        committed_status: is_committed,
        ..
      } => Some(is_committed.clone()),
      _ => None,
    }
  }

  /// 失敗時のコミットを解除する
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

  pub fn with_committed_fallback(self, is_committed: bool) -> Self {
    match self {
      ParseResult::Failure {
        error,
        committed_status: c,
      } => ParseResult::Failure {
        error,
        committed_status: (c.or(&is_committed.into())),
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

  pub fn map_err<F>(self, f: F) -> Self
  where
    F: Fn(ParseError<'a, I>) -> ParseError<'a, I>, {
    match self {
      ParseResult::Failure {
        error: e,
        committed_status: c,
      } => ParseResult::Failure {
        error: f(e),
        committed_status: c,
      },
      _ => self,
    }
  }

  pub fn with_add_length(self, n: usize) -> Self {
    match self {
      ParseResult::Success { value, length: m } => ParseResult::Success { value, length: n + m },
      _ => self,
    }
  }
}
