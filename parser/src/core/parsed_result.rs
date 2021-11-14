use crate::core::parsed_error::ParsedError;
use crate::core::CommittedStatus;

/// A Parsed Result.<br/>
/// 解析された結果。
#[derive(Debug, Clone)]
pub enum ParsedResult<'a, I, A> {
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
    error: ParsedError<'a, I>,
    /// コミット状態
    is_committed: CommittedStatus,
  },
}

impl<'a, I, A> ParsedResult<'a, I, A> {
  /// Returns the parsed result of success.<br/>
  /// 成功の解析結果を返します。
  ///
  /// - value: a value
  /// - length: a size of the value
  ///
  /// - value: 値
  /// - length: 値のサイズ
  pub fn successful(value: A, length: usize) -> Self {
    ParsedResult::Success { value, length }
  }

  /// Returns the parsed result of failure.<br/>
  /// 失敗の解析結果を返します。
  ///
  /// - error: a [ParsedError]
  /// - is_committed: a [CommittedStatus]
  pub fn failed(error: ParsedError<'a, I>, is_committed: CommittedStatus) -> Self {
    ParsedResult::Failure { error, is_committed }
  }

  /// Returns the parsed result of failure.<br/>
  /// 失敗の解析結果を返します。
  ///
  /// - error: a [ParsedError]
  pub fn failed_with_uncommitted(error: ParsedError<'a, I>) -> Self {
    Self::failed(error, CommittedStatus::Uncommitted)
  }

  pub fn failed_with_commit(error: ParsedError<'a, I>) -> Self {
    Self::failed(error, CommittedStatus::Committed)
  }

  /// Convert [ParsedResult] to [Result].
  ///
  /// [ParsedResult]を[Result]に変換する。
  pub fn to_result(self) -> Result<A, ParsedError<'a, I>> {
    match self {
      ParsedResult::Failure { error, .. } => Err(error),
      ParsedResult::Success { value, .. } => Ok(value),
    }
  }

  pub fn is_success(&self) -> bool {
    match self {
      ParsedResult::Failure { .. } => false,
      ParsedResult::Success { .. } => true,
    }
  }

  pub fn success(self) -> Option<A> {
    match self {
      ParsedResult::Failure { .. } => None,
      ParsedResult::Success { value, .. } => Some(value),
    }
  }

  pub fn failure(self) -> Option<ParsedError<'a, I>> {
    match self {
      ParsedResult::Failure { error, .. } => Some(error),
      ParsedResult::Success { .. } => None,
    }
  }

  pub fn is_failure(&self) -> bool {
    match self {
      ParsedResult::Failure { .. } => true,
      ParsedResult::Success { .. } => false,
    }
  }

  pub fn committed_status(&self) -> Option<CommittedStatus> {
    match self {
      ParsedResult::Failure { is_committed, .. } => Some(is_committed.clone()),
      _ => None,
    }
  }

  /// 失敗時のコミットを解除する
  pub fn with_uncommitted(self) -> Self {
    match self {
      ParsedResult::Failure {
        error,
        is_committed: CommittedStatus::Committed,
      } => ParsedResult::Failure {
        error,
        is_committed: CommittedStatus::Uncommitted,
      },
      _ => self,
    }
  }

  pub fn with_committed_fallback(self, is_committed: bool) -> Self {
    match self {
      ParsedResult::Failure { error, is_committed: c } => ParsedResult::Failure {
        error,
        is_committed: (c.or(&is_committed.into())),
      },
      _ => self,
    }
  }

  pub fn map_err<F>(self, f: F) -> Self
  where
    F: Fn(ParsedError<'a, I>) -> ParsedError<'a, I>, {
    match self {
      ParsedResult::Failure {
        error: e,
        is_committed: c,
      } => ParsedResult::Failure {
        error: f(e),
        is_committed: c,
      },
      _ => self,
    }
  }

  pub fn with_add_length(self, n: usize) -> Self {
    match self {
      ParsedResult::Success { value, length: m } => ParsedResult::Success { value, length: n + m },
      _ => self,
    }
  }
}
