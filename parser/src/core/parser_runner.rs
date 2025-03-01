use crate::core::ParserMonad;
use crate::core::{ParseError, ParseResult, ParseState};

pub trait ParserRunner<'a> {
  type Input;
  type Output;
  type P<'m, X, Y: 'm>: ParserMonad<'m, Input = X, Output = Y>
  where
    X: 'm;

  /// Analyze input value(for [ParseResult]).<br/>
  /// 入力を解析する。
  fn parse(&self, input: &'a [Self::Input]) -> ParseResult<'a, Self::Input, Self::Output>;

  /// Analyze input value(for [Result]).<br/>
  /// 入力を解析する。
  fn parse_as_result(&self, input: &'a [Self::Input]) -> Result<Self::Output, ParseError<'a, Self::Input>> {
    self.parse(input).to_result()
  }

  /// Analyze input value(for [ParseResult]).<br/>
  /// 入力を解析する。
  ///
  /// Requires [ParseState] argument.<br/>
  /// 引数に[ParseState]が必要です。
  fn run(&self, param: &ParseState<'a, Self::Input>) -> ParseResult<'a, Self::Input, Self::Output>;
}
