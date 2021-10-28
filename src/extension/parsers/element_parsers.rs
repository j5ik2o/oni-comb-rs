use crate::core::{Element, Parsers};
use crate::utils::Set;
use std::fmt::{Debug, Display};

pub trait ElementParsers: Parsers {
  fn elm_any_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred_ref(|_| true)
  }

  fn elm_any<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::map(Self::elm_any_ref(), Clone::clone)
  }

  fn elm_ref<'a, I>(c: I) -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred_ref(move |actual| *actual == c)
  }

  fn elm<'a, I>(c: I) -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::map(Self::elm_ref(c), Clone::clone)
  }

  fn elm_pred_ref<'a, I, F>(f: F) -> Self::P<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + PartialEq + 'a;

  fn elm_pred<'a, I, F>(f: F) -> Self::P<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + 'a, {
    Self::map(Self::elm_pred_ref(f), Clone::clone)
  }

  fn elm_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::map(Self::elm_space_ref(), Clone::clone)
  }

  fn elm_multi_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_multi_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::map(Self::elm_multi_space_ref(), Clone::clone)
  }

  fn elm_alpha_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_alpha<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::map(Self::elm_alpha_ref(), Clone::clone)
  }

  fn elm_alpha_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_alpha_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::map(Self::elm_alpha_digit_ref(), Clone::clone)
  }

  fn elm_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::map(Self::elm_digit_ref(), Clone::clone)
  }

  fn elm_hex_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_hex_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::map(Self::elm_hex_digit_ref(), Clone::clone)
  }

  fn elm_oct_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a;

  fn elm_oct_digit<'a, I>() -> Self::P<'a, I, I>
    where
        I: Element + Clone + PartialEq + 'a {
    Self::map(Self::elm_oct_digit_ref(), Clone::clone)
  }

  fn elm_ref_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized;

  fn elm_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, I>
    where
        I: PartialEq + Clone + Display + Debug + 'a,
        S: Set<I> + ?Sized {
    Self::map(Self::elm_ref_of(set), Clone::clone)
  }

  fn elm_ref_in<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a;

  fn elm_in<'a, I>(start: I, end: I) -> Self::P<'a, I, I>
    where
        I: PartialEq + PartialOrd + Display + Debug + Copy + Clone + 'a {
    Self::map(Self::elm_ref_in(start, end), Clone::clone)
  }

  fn elm_ref_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a;

  fn elm_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, I>
    where
        I: PartialEq + PartialOrd + Display + Debug + Copy + Clone + 'a {
    Self::map(Self::elm_ref_from_until(start, end), Clone::clone)
  }

  fn none_ref_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized;

  fn none_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, I>
    where
        I: PartialEq + Display + Clone + Debug + 'a,
        S: Set<I> + ?Sized {
    Self::map(Self::none_ref_of(set), Clone::clone)
  }
}
