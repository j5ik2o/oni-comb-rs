use crate::parsers::Parsers;
use std::fmt::{Debug, Display};

pub trait ElementParsers: Parsers {
  fn eof<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Clone + Debug + Display + 'a;

  fn empty<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Clone + 'a;

  fn any_elem<'a, I>() -> Self::P<'a, I, I>
  where
    I: Clone + PartialEq + 'a, {
    Self::elem_in(|_| true)
  }

  fn elem_in<'a, I, F>(f: F) -> Self::P<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + PartialEq + 'a;

  fn elem<'a, I>(c: I) -> Self::P<'a, I, I>
  where
    I: Clone + PartialEq + 'a, {
    Self::elem_in(move |actual| *actual == c)
  }

  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, &'a [I]>
  where
    I: Clone + PartialEq + Debug + 'a,
    'b: 'a;

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a;
}
