use crate::core::ParserMonad;
use crate::core::{ParseState, ParsedError, ParsedResult};

pub trait ParserRunner<'a> {
  type Input;
  type Output;
  type P<'m, X, Y>: ParserMonad<'m, Input = X, Output = Y>
  where
    X: 'm;

  fn parse(&self, input: &'a [Self::Input]) -> ParsedResult<'a, Self::Input, Self::Output>;

  fn parse_as_result(&self, input: &'a [Self::Input]) -> Result<Self::Output, ParsedError<'a, Self::Input>> {
    self.parse(input).to_result()
  }

  fn run(&self, param: &ParseState<'a, Self::Input>) -> ParsedResult<'a, Self::Input, Self::Output>;
}
