use crate::core::{Parser, Parsers, StaticParser, StaticParsers};
use std::fmt::Debug;

pub trait ElementsParsers: Parsers {
  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, Vec<I>>
  where
    I: PartialEq + Debug + Clone + 'a,
    'b: 'a;

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, String>
  where
    'b: 'a;

  fn tag_no_case<'a, 'b>(tag: &'b str) -> Self::P<'a, char, String>
  where
    'b: 'a;

  fn regex<'a>(pattern: &str) -> Self::P<'a, char, String>;
}

pub trait StaticElementsParsers: StaticParsers {
  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, Vec<I>>
  where
    I: crate::core::Element + PartialEq + Debug + Clone + 'a + 'static,
    'b: 'a;

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, String>
  where
    'b: 'a;

  fn tag_no_case<'a, 'b>(tag: &'b str) -> Self::P<'a, char, String>
  where
    'b: 'a;

  fn regex<'a>(pattern: &'a str) -> Self::P<'a, char, String>;
}

// 既存のParserを使用する関数
pub fn seq<'a, 'b, I>(tag: &'b [I]) -> Parser<'a, I, Vec<I>>
where
  I: PartialEq + Debug + Clone + 'a,
  'b: 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::seq(tag)
}

pub fn tag<'a, 'b>(tag: &'b str) -> Parser<'a, char, String>
where
  'b: 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::tag(tag)
}

pub fn tag_no_case<'a, 'b>(tag: &'b str) -> Parser<'a, char, String>
where
  'b: 'a, {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::tag_no_case(tag)
}

pub fn regex<'a>(pattern: &str) -> Parser<'a, char, String> {
  use crate::internal::parsers_impl::ParsersImpl;
  ParsersImpl::regex(pattern)
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn seq<'a, 'b, I>(tag: &'b [I]) -> StaticParser<'a, I, Vec<I>>
  where
    I: crate::core::Element + PartialEq + Debug + Clone + 'a + 'static,
    'b: 'a, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::seq(tag)
  }

  pub fn tag<'a, 'b>(tag: &'b str) -> StaticParser<'a, char, String>
  where
    'b: 'a, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::tag(tag)
  }

  pub fn tag_no_case<'a, 'b>(tag: &'b str) -> StaticParser<'a, char, String>
  where
    'b: 'a, {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::tag_no_case(tag)
  }

  pub fn regex<'a>(pattern: &'a str) -> StaticParser<'a, char, String> {
    use crate::internal::static_parsers_impl::StaticParsersImpl;
    StaticParsersImpl::regex(pattern)
  }
}
