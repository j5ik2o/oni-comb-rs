use crate::core::{Element, Parser, StaticParser};
use crate::extension::parsers::element_parsers::{ElementParsers, StaticElementParsers};
use crate::internal::parsers_impl;
use std::fmt::Debug;

pub trait TakenParsers: ElementParsers {
  fn take<'a, I: Clone>(n: usize) -> Self::P<'a, I, &'a [I]>;

  fn take_while0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;

  fn take_while1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;

  fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;

  fn take_till0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;

  fn take_till1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;
}

pub trait StaticTakenParsers: StaticElementParsers {
  fn take<'a, I: Clone>(n: usize) -> Self::P<'a, I, &'a [I]>;

  fn take_while0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;

  fn take_while1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;

  fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;

  fn take_till0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;

  fn take_till1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + Element + Debug + 'a;
}

// 既存のParserを使用する関数
pub fn take<'a, I: Clone>(n: usize) -> Parser<'a, I, &'a [I]> {
  parsers_impl::ParsersImpl::take(n)
}

pub fn take_while0<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + 'a,
  I: Clone + Element + Debug + 'a, {
  parsers_impl::ParsersImpl::take_while0(f)
}

pub fn take_while1<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + 'a,
  I: Clone + Element + Debug + 'a, {
  parsers_impl::ParsersImpl::take_while1(f)
}

pub fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + 'a,
  I: Clone + Element + Debug + 'a, {
  parsers_impl::ParsersImpl::take_while_n_m(n, m, f)
}

pub fn take_till0<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + 'a,
  I: Clone + Element + Debug + 'a, {
  parsers_impl::ParsersImpl::take_till0(f)
}

pub fn take_till1<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + 'a,
  I: Clone + Element + Debug + 'a, {
  parsers_impl::ParsersImpl::take_till1(f)
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn take<'a, I>(n: usize) -> StaticParser<'a, I, &'a [I]>
  where
    I: Element + Clone + PartialEq + Debug + 'a + 'static, {
    crate::internal::static_parsers_impl::StaticParsersImpl::take(n)
  }

  pub fn take_while0<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + Clone + PartialEq + 'a + 'static, {
    crate::internal::static_parsers_impl::StaticParsersImpl::take_while0(f)
  }

  pub fn take_while1<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + Clone + PartialEq + 'a + 'static, {
    crate::internal::static_parsers_impl::StaticParsersImpl::take_while1(f)
  }

  pub fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + Clone + PartialEq + 'a + 'static, {
    crate::internal::static_parsers_impl::StaticParsersImpl::take_while_n_m(n, m, f)
  }

  pub fn take_till0<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + Clone + PartialEq + 'a + 'static, {
    crate::internal::static_parsers_impl::StaticParsersImpl::take_till0(f)
  }

  pub fn take_till1<'a, I, F>(f: F) -> StaticParser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + Clone + PartialEq + 'a + 'static, {
    crate::internal::static_parsers_impl::StaticParsersImpl::take_till1(f)
  }
}
