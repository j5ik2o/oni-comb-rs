use std::fmt::{Debug, Display};

use crate::core::{Parser, Parsers, StaticParser, StaticParsers};

pub trait PrimitiveParsers: Parsers {
  fn begin<'a, I>() -> Self::P<'a, I, ()> {
    Self::empty()
  }

  fn end<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Debug + Display + 'a;

  fn empty<'a, I>() -> Self::P<'a, I, ()>;
}

pub trait StaticPrimitiveParsers: StaticParsers {
  fn begin<'a, I>() -> Self::P<'a, I, ()> {
    Self::empty()
  }

  fn end<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Debug + Display + 'a;

  fn empty<'a, I>() -> Self::P<'a, I, ()>;
}

// 既存のParserを使用する関数
pub fn begin<'a, I>() -> Parser<'a, I, ()> {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::begin()
}

pub fn end<'a, I>() -> Parser<'a, I, ()>
where
  I: Debug + Display + 'a, {
  use crate::core::CommittedStatus;
  Parser::new(move |state| {
    let input = state.input();
    if input.is_empty() {
      crate::core::ParseResult::successful((), 0)
    } else {
      crate::core::ParseResult::failed(
        crate::core::ParseError::of_custom(state.next_offset(), None, format!("Unexpected input: {:?}", input[0])),
        CommittedStatus::Uncommitted,
      )
    }
  })
}

pub fn empty<'a, I>() -> Parser<'a, I, ()> {
  Parser::new(move |_| crate::core::ParseResult::successful((), 0))
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn begin<'a, I>() -> StaticParser<'a, I, ()> {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::begin()
  }

  pub fn end<'a, I>() -> StaticParser<'a, I, ()>
  where
    I: Debug + Display + 'a, {
    use crate::core::CommittedStatus;
    StaticParser::new(move |state| {
      let input = state.input();
      if input.is_empty() {
        crate::core::ParseResult::successful((), 0)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, format!("Unexpected input: {:?}", input[0])),
          CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn empty<'a, I>() -> StaticParser<'a, I, ()> {
    StaticParser::new(move |_| crate::core::ParseResult::successful((), 0))
  }
}
