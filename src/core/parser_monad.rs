use std::rc::Rc;

use crate::core::parse_result::ParseResult;
use crate::core::parse_state::ParseState;
use crate::core::ParseError;

pub trait ParserPure<'a>: ParserRunner<'a> {
  fn pure<F>(value: F) -> Self::P<'a, Self::Input, Self::Output>
  where
    F: Fn() -> Self::Output + 'a,
    Self::Input: 'a,
    Self::Output: 'a;
}

pub trait ParserFunctor<'a>: ParserPure<'a> {
  fn map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> B + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    B: 'a;
}

pub trait ParserMonad<'a>: ParserFunctor<'a> {
  fn flat_map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Self::P<'a, Self::Input, B> + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    B: 'a;
}

pub trait ParserRunner<'a> {
  type Input;
  type Output;
  type P<'m, X, Y>: ParserMonad<'m, Input = X, Output = Y>
  where
    X: 'm;

  fn parse<'b>(&self, input: &'b [Self::Input]) -> Result<Self::Output, ParseError<'a, Self::Input>>
  where
    'b: 'a;

  fn run(&self, param: Rc<ParseState<'a, Self::Input>>) -> ParseResult<'a, Self::Input, Self::Output>;
}
