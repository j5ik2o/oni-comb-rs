use crate::parse_result::ParseResult;
use crate::parse_state::ParseState;
use std::rc::Rc;

pub trait Parser<'a> {
  type Input;
  type Output;
  type M<'m, X, Y>: Parser<'m, Input = X, Output = Y>
  where
    X: 'm;

  fn run(&self, param: Rc<ParseState<'a, Self::Input>>) -> ParseResult<'a, Self::Input, Self::Output>;

  fn flat_map<B, F>(self, f: F) -> Self::M<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Self::M<'a, Self::Input, B> + 'a,
    Self::Input: Clone + 'a,
    Self::Output: 'a,
    B: Clone + 'a;

  fn pure(value: Self::Output) -> Self::M<'a, Self::Input, Self::Output>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + 'a;

  fn map<B, F>(self, f: F) -> Self::M<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> B + 'a,
    Self::Input: Clone + 'a,
    Self::Output: 'a,
    B: Clone + 'a;
}
