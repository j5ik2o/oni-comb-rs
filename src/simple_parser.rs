use crate::combinator::BasicCombineParser;
use crate::combinators::BasicCombinators;
use std::fmt::Debug;
use std::rc::Rc;

use crate::parse_result::ParseResult;
use crate::parse_state::ParseState;
use crate::parser::Parser;
use crate::parsers::Parsers;
use crate::range::RangeArgument;
use crate::simple_parsers::SimpleParsers;
use crate::Tuple;

type SimpleParse<'a, I, A> = dyn Fn(Rc<ParseState<'a, I>>) -> ParseResult<'a, I, A> + 'a;

pub struct SimpleParser<'a, I, A> {
  pub(crate) method: Box<SimpleParse<'a, I, A>>,
}

impl<'a, I, A> SimpleParser<'a, I, A> {
  pub fn new<P>(parse: P) -> SimpleParser<'a, I, A>
  where
    P: Fn(Rc<ParseState<'a, I>>) -> ParseResult<'a, I, A> + 'a, {
    SimpleParser {
      method: Box::new(parse),
    }
  }
}

impl<'a, I, A> Parser<'a> for SimpleParser<'a, I, A> {
  type Input = I;
  type M<'m, X, Y>
  where
    X: 'm,
  = SimpleParser<'m, X, Y>;
  type Output = A;

  fn run(&self, param: Rc<ParseState<'a, Self::Input>>) -> ParseResult<'a, Self::Input, Self::Output> {
    (self.method)(param)
  }

  fn flat_map<B, F>(self, f: F) -> Self::M<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Self::M<'a, Self::Input, B> + 'a,
    Self::Input: Clone + 'a,
    Self::Output: 'a,
    B: Clone + 'a, {
    SimpleParsers::flat_map(self, move |e| f(e))
  }

  fn pure(value: Self::Output) -> Self::M<'a, Self::Input, Self::Output>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + 'a, {
    SimpleParsers::successful(value)
  }

  fn map<B, F>(self, f: F) -> Self::M<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> B + 'a,
    Self::Input: Clone + 'a,
    Self::Output: 'a,
    B: Clone + 'a, {
    SimpleParsers::map(self, f)
  }
}

impl<'a, I, A> BasicCombineParser<'a> for SimpleParser<'a, I, A> {
  fn and<B>(self, pb: Self::M<'a, Self::Input, B>) -> Self::M<'a, Self::Input, Tuple<Self::Output, B>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    Self::M<'a, Self::Input, B>: 'a, {
    SimpleParsers::and(self, pb)
  }

  fn or(self, pb: Self::M<'a, Self::Input, Self::Output>) -> Self::M<'a, Self::Input, Self::Output>
  where
    Self::Output: 'a, {
    SimpleParsers::or(self, pb)
  }

  fn opt(self) -> Self::M<'a, Self::Input, Option<Self::Output>>
  where
    Self::Input: Clone + 'a,
    Self::Output: Clone + 'a, {
    SimpleParsers::opt(self)
  }

  // fn repeat<R>(
  //   self,
  //   range: R
  // ) -> Self::M<'a, Self::Input, Vec<Self::Output>>
  //   where
  //       Self::Input: Clone + 'a,
  //       R: RangeArgument<usize> + Debug + 'a,
  //       Self::Output: 'a;

  fn repeat_with_separator<B, R>(
    self,
    range: R,
    separator: Option<Self::M<'a, Self::Input, B>>,
  ) -> Self::M<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Input: Clone + 'a,
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: 'a,
    B: 'a, {
    SimpleParsers::repeat_with_separator(self, range, separator)
  }
}
