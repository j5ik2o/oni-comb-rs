use crate::core::{Element, Parsers};

pub trait ElementParsers: Parsers {
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
}
