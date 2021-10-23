use std::fmt::Debug;
use std::rc::Rc;

use crate::core::{ParseResult, ParseState};
use crate::extension::{
  BasicCombinator, ConversionCombinator, ConversionCombinators, OffsetCombinator, OffsetCombinators, SkipCombinator,
  SkipCombinators,
};
use crate::extension::{BasicCombinators, RepeatCombinator, RepeatCombinators};
use crate::internal::ParsersImpl;
use crate::utils::RangeArgument;

mod add_parser;
mod bitor_parser;
mod mul_parser;
mod not_parser;
mod parser_functor;
mod parser_monad;
mod parser_pure;
mod parser_runner;
mod sub_parser;

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
    ParsersImpl::repeat(self, range)
  }

  fn of_many0(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::many0(self)
  }

  fn of_many1(self) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::many1(self)
  }

  fn of_many_n_m(self, n: usize, m: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::many_n_m(self, n, m)
  }

  fn of_count(self, n: usize) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a, {
    ParsersImpl::count(self, n)
  }

  fn of_rep_sep<B, R>(
    self,
    range: R,
    separator: Option<Self::P<'a, Self::Input, B>>,
  ) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::repeat_sep(self, range, separator)
  }

  fn of_many0_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::many0_sep(self, separator)
  }

  fn of_many1_sep<B>(self, separator: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, Vec<Self::Output>>
  where
    Self::Output: Debug + 'a,
    B: Debug + 'a, {
    ParsersImpl::many1_sep(self, separator)
  }

  fn of_many_n_m_sep<B>(
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

  fn of_count_sep<B>(
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
