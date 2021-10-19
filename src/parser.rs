use std::fmt::Debug;
use std::ops::{Add, BitOr, Mul, Not, Sub};
use std::rc::Rc;

use crate::core::ParseState;
use crate::core::ParserRunner;
use crate::core::{CoreParsers, ParseError};
use crate::core::{ParseResult, ParserFunctor, ParserMonad, ParserPure};
use crate::extension::{
  BasicCombinator, ConversionCombinator, ConversionCombinators, OffsetCombinator, OffsetCombinators, SkipCombinator,
  SkipCombinators,
};
use crate::extension::{BasicCombinators, RepeatCombinator, RepeatCombinators};
use crate::internal::ParsersImpl;
use crate::utils::RangeArgument;

type Parse<'a, I, A> = dyn Fn(Rc<ParseState<'a, I>>) -> ParseResult<'a, I, A> + 'a;

pub struct Parser<'a, I, A> {
  method: Box<Parse<'a, I, A>>,
}

impl<'a, I, A> Parser<'a, I, A> {
  pub fn new<F>(parse: F) -> Parser<'a, I, A>
  where
    F: Fn(Rc<ParseState<'a, I>>) -> ParseResult<'a, I, A> + 'a, {
    Parser {
      method: Box::new(parse),
    }
  }
}

impl<'a, I, A> ParserRunner<'a> for Parser<'a, I, A> {
  type Input = I;
  type Output = A;
  type P<'m, X, Y>
  where
    X: 'm,
  = Parser<'m, X, Y>;

  fn parse<'b>(&self, input: &'b [Self::Input]) -> Result<Self::Output, ParseError<'a, Self::Input>>
  where
    'b: 'a, {
    let parse_state = ParseState::new(input, 0);
    self.run(Rc::new(parse_state)).extract()
  }

  fn run(&self, param: Rc<ParseState<'a, Self::Input>>) -> ParseResult<'a, Self::Input, Self::Output> {
    (self.method)(param)
  }
}

impl<'a, I, A> ParserFunctor<'a> for Parser<'a, I, A> {
  fn map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> B + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    B: 'a, {
    ParsersImpl::map(self, f)
  }
}

impl<'a, I, A> ParserPure<'a> for Parser<'a, I, A> {
  fn pure<F>(value: F) -> Self::P<'a, Self::Input, Self::Output>
  where
    F: Fn() -> Self::Output + 'a,
    Self::Input: 'a,
    Self::Output: 'a, {
    ParsersImpl::successful(value)
  }
}

impl<'a, I, A> ParserMonad<'a> for Parser<'a, I, A> {
  fn flat_map<B, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Self::P<'a, Self::Input, B> + 'a,
    Self::Input: 'a,
    Self::Output: 'a,
    B: 'a, {
    ParsersImpl::flat_map(self, move |e| f(e))
  }
}

impl<'a, I, A> BasicCombinator<'a> for Parser<'a, I, A> {
  fn and_then<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, (Self::Output, B)>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::and_then(self, pb)
  }

  fn or(self, pb: Self::P<'a, Self::Input, Self::Output>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::or(self, pb)
  }

  fn not(self) -> Self::P<'a, Self::Input, bool>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::not(self)
  }

  fn opt(self) -> Self::P<'a, Self::Input, Option<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::opt(self)
  }

  fn collect(self) -> Self::P<'a, Self::Input, &'a [Self::Input]>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::collect(self)
  }

  fn discard(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::discard(self)
  }
}

impl<'a, I, A> ConversionCombinator<'a> for Parser<'a, I, A> {
  fn convert<B, E, F>(self, f: F) -> Self::P<'a, Self::Input, B>
  where
    F: Fn(Self::Output) -> Result<B, E> + 'a,
    E: Debug,
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::convert(self, f)
  }
}

impl<'a, I, A> OffsetCombinator<'a> for Parser<'a, I, A> {
  fn last_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::last_offset(self)
  }

  fn next_offset(self) -> Self::P<'a, Self::Input, usize>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::next_offset(self)
  }
}

impl<'a, I, A> SkipCombinator<'a> for Parser<'a, I, A> {
  fn skip_left<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, B>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::skip_left(self, pb)
  }

  fn skip_right<B>(self, pb: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::skip_right(self, pb)
  }
}

impl<'a, I, A> RepeatCombinator<'a> for Parser<'a, I, A> {
  fn repeat<R>(self, range: R) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: Debug + 'a,
    Self: Sized, {
    ParsersImpl::rep(self, range)
  }

  fn many_0(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::many_0(self)
  }

  fn many_1(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::many_1(self)
  }

  fn many_n_m(self, n: usize, m: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::many_n_m(self, n, m)
  }

  fn count(self, n: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::count(self, n)
  }

  fn rep_sep<B, R>(
    self,
    range: R,
    separator: Option<Self::P<'a, Self::Input, B>>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::rep_sep(self, range, separator)
  }

  fn many_0_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::many_0_sep(self, separator)
  }

  fn many_1_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::many_1_sep(self, separator)
  }

  fn many_n_m_sep<B>(
    self,
    n: usize,
    m: usize,
    separator: Self::P<'a, Self::Input, B>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::many_n_m_sep(self, n, m, separator)
  }

  fn count_sep<B>(
    self,
    n: usize,
    separator: Self::P<'a, Self::Input, B>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::count_sep(self, n, separator)
  }
}

impl<'a, I, A, B> Add<Parser<'a, I, B>> for Parser<'a, I, A>
where
  A: Debug + 'a,
  B: Debug + 'a,
{
  type Output = Parser<'a, I, (A, B)>;

  fn add(self, rhs: Parser<'a, I, B>) -> Self::Output {
    self.and_then(rhs)
  }
}

impl<'a, I, A, B> Sub<Parser<'a, I, B>> for Parser<'a, I, A>
where
  A: Debug + 'a,
  B: Debug + 'a,
{
  type Output = Self;

  fn sub(self, rhs: Parser<'a, I, B>) -> Self::Output {
    self.skip_right(rhs)
  }
}

impl<'a, I, A, B> Mul<Parser<'a, I, B>> for Parser<'a, I, A>
where
  A: Debug + 'a,
  B: Debug + 'a,
{
  type Output = Parser<'a, I, B>;

  fn mul(self, rhs: Parser<'a, I, B>) -> Self::Output {
    self.skip_left(rhs)
  }
}

impl<'a, I, A> BitOr for Parser<'a, I, A>
where
  A: Debug + 'a,
{
  type Output = Self;

  fn bitor(self, rhs: Parser<'a, I, A>) -> Self::Output {
    self.or(rhs)
  }
}

impl<'a, I, A> Not for Parser<'a, I, A>
where
  A: Debug + 'a,
{
  type Output = Parser<'a, I, bool>;

  fn not(self) -> Self::Output {
    ParsersImpl::not(self)
  }
}
