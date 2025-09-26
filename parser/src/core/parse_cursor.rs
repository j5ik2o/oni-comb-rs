//! 内部コンビネータで共通利用する軽量なパースループ支援ユーティリティ。
//!
//! `ParseCursor` は `Parser::run` を直接呼び出す代わりに、ループ内での状態管理と
//! コミットステータスの伝播を効率的に行うための薄いラッパーを提供する。
//! `many` 系や `*_sep` 系のように同一パーサーを繰り返し評価する場面で利用し、
//! 余分な `Rc` クローンなしに状態遷移を共有できる。

use crate::core::{CommittedStatus, ParseError, ParseResult, ParseState, Parser};

#[derive(Copy, Clone)]
pub struct ParseCursor<'a, I> {
    input: &'a [I],
    state: ParseState<'a, I>,
}

impl<'a, I> ParseCursor<'a, I> {
    /// 共有の入力スライスと初期状態からカーソルを構築する。
    pub fn new(input: &'a [I], state: ParseState<'a, I>) -> Self {
        Self { input, state }
    }

    /// 現在の `ParseState` を取得する。
    pub fn state(&self) -> ParseState<'a, I> {
        self.state
    }

    /// カーソルの状態を任意のチェックポイントに巻き戻す。
    pub fn set_state(&mut self, state: ParseState<'a, I>) {
        self.state = state;
    }

    /// カーソルを消費して最新の `ParseState` を返す。
    pub fn into_state(self) -> ParseState<'a, I> {
        self.state
    }

    /// 指定したパーサーを実行し、成功時には値と消費バイト数を返す。
    ///
    /// 失敗した場合は `ParseFailure` を返し、コミット済みであれば
    /// 呼び出し元で即座に失敗を伝播させる必要がある。
    pub fn consume<A>(
        &mut self,
        parser: &Parser<'a, I, A>,
    ) -> Result<(A, usize), ParseFailure<'a, I>> {
        match parser.run(self.input, self.state) {
            ParseResult::Success {
                value,
                length,
                state: Some(next_state),
            } => {
                self.state = next_state;
                Ok((value, length))
            }
            ParseResult::Success { .. } => Err(ParseFailure::internal(
                self.state,
                "parser did not return state",
            )),
            ParseResult::Failure {
                error,
                committed_status,
            } => Err(ParseFailure {
                error,
                committed_status,
            }),
        }
    }
}

/// `ParseCursor::consume` からの失敗結果をラップする補助構造体。
pub struct ParseFailure<'a, I> {
    pub error: ParseError<'a, I>,
    pub committed_status: CommittedStatus,
}

impl<'a, I> ParseFailure<'a, I> {
    /// エラーとコミット情報から失敗を生成する。
    pub fn new(error: ParseError<'a, I>, committed_status: CommittedStatus) -> Self {
        Self {
            error,
            committed_status,
        }
    }

    /// `ParseResult` へ変換し、既存の API との互換性を保つ。
    pub fn into_result<A>(self) -> ParseResult<'a, I, A> {
        ParseResult::Failure {
            error: self.error,
            committed_status: self.committed_status,
        }
    }

    /// 失敗がコミットされたかどうかを返す。
    pub fn is_committed(&self) -> bool {
        self.committed_status.is_committed()
    }

    /// 内部整合性エラーを生成するためのユーティリティ。
    fn internal(state: ParseState<'a, I>, message: &'static str) -> Self {
        Self {
            error: ParseError::of_custom(state.current_offset(), Some(state.input()), message),
            committed_status: CommittedStatus::Uncommitted,
        }
    }
}
