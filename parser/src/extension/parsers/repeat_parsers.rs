use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use crate::prelude::RangeArgument;
use std::fmt::Debug;

pub trait RepeatParsers: Parsers {
  fn repeat<'a, I, A, R>(parser: Self::P<'a, I, A>, range: R) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a;

  fn many0<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a;

  fn many1<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a;

  fn count<'a, I, A>(parser: Self::P<'a, I, A>, count: usize) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a;

  fn many0_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, sep: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn many1_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, sep: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn repeat_sep<'a, I, A, B, R>(
    parser: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a;
}

pub trait StaticRepeatParsers: StaticParsers {
  fn repeat<'a, I, A, R>(parser: Self::P<'a, I, A>, range: R) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static;

  fn many0<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static;

  fn many1<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static;

  fn count<'a, I, A>(parser: Self::P<'a, I, A>, count: usize) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static;

  fn many0_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, sep: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static;

  fn many1_sep<'a, I, A, B>(parser: Self::P<'a, I, A>, sep: Self::P<'a, I, B>) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static;

  fn repeat_sep<'a, I, A, B, R>(
    parser: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static;
}

// 既存のParserを使用する関数
pub fn repeat<'a, I, A, R>(parser: Parser<'a, I, A>, range: R) -> Parser<'a, I, Vec<A>>
where
  R: RangeArgument<usize> + Debug + 'a,
  I: Clone + 'a,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::repeat(parser, range)
}

pub fn many0<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, Vec<A>>
where
  I: Clone + 'a,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::many0(parser)
}

pub fn many1<'a, I, A>(parser: Parser<'a, I, A>) -> Parser<'a, I, Vec<A>>
where
  I: Clone + 'a,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::many1(parser)
}

pub fn count<'a, I, A>(parser: Parser<'a, I, A>, count: usize) -> Parser<'a, I, Vec<A>>
where
  I: Clone + 'a,
  A: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::count(parser, count)
}

pub fn many0_sep<'a, I, A, B>(parser: Parser<'a, I, A>, sep: Parser<'a, I, B>) -> Parser<'a, I, Vec<A>>
where
  I: Clone + 'a,
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::many0_sep(parser, sep)
}

pub fn many1_sep<'a, I, A, B>(parser: Parser<'a, I, A>, sep: Parser<'a, I, B>) -> Parser<'a, I, Vec<A>>
where
  I: Clone + 'a,
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::many1_sep(parser, sep)
}

pub fn repeat_sep<'a, I, A, B, R>(
  parser: Parser<'a, I, A>,
  range: R,
  separator: Option<Parser<'a, I, B>>,
) -> Parser<'a, I, Vec<A>>
where
  R: RangeArgument<usize> + Debug + 'a,
  I: Clone + 'a,
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::repeat_sep(parser, range, separator)
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn repeat<'a, I, A, R>(parser: StaticParser<'a, I, A>, range: R) -> StaticParser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::repeat(parser, range)
  }

  pub fn many0<'a, I, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::many0(parser)
  }

  pub fn many1<'a, I, A>(parser: StaticParser<'a, I, A>) -> StaticParser<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::many1(parser)
  }

  pub fn count<'a, I, A>(parser: StaticParser<'a, I, A>, count: usize) -> StaticParser<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::count(parser, count)
  }

  pub fn many0_sep<'a, I, A, B>(
    parser: StaticParser<'a, I, A>,
    sep: StaticParser<'a, I, B>,
  ) -> StaticParser<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::many0_sep(parser, sep)
  }

  pub fn many1_sep<'a, I, A, B>(
    parser: StaticParser<'a, I, A>,
    sep: StaticParser<'a, I, B>,
  ) -> StaticParser<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::many1_sep(parser, sep)
  }

  pub fn repeat_sep<'a, I, A, B, R>(
    parser: StaticParser<'a, I, A>,
    range: R,
    separator: Option<StaticParser<'a, I, B>>,
  ) -> StaticParser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    I: Clone + 'a,
    A: Clone + Debug + 'a + 'static,
    B: Clone + Debug + 'a + 'static, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::repeat_sep(parser, range, separator)
  }
}
