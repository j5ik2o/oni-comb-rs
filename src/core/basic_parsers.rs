use std::fmt::{Debug, Display};

use crate::core::element::Element;
use crate::core::CoreParsers;
use crate::utils::Set;

pub trait BasicParsers: CoreParsers {
  fn end<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Debug + Display + 'a;

  fn empty<'a, I>() -> Self::P<'a, I, ()>;

  fn elm_any<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred(|_| true)
  }

  fn elm<'a, I>(c: I) -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred(move |actual| *actual == c)
  }

  fn elm_pred<'a, I, F>(f: F) -> Self::P<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + PartialEq + 'a;

  fn elm_space<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_multi_space<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_alpha<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_alpha_digit<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_digit<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_hex_digit<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_oct_digit<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, &'a [I]>
  where
    I: PartialEq + Debug + 'a,
    'b: 'a;

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a;

  fn tag_no_case<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a;

  fn take<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>;

  fn take_while0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a;

  fn take_while1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a;

  fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a;

  fn take_till0<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a;

  fn take_till1<'a, I, F>(f: F) -> Self::P<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a;

  fn skip<'a, I>(n: usize) -> Self::P<'a, I, ()>;

  fn elm_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized;

  fn elm_in<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a;

  fn elm_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a;

  fn not_elm_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized;
}
