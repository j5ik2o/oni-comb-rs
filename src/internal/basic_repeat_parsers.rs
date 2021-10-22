use crate::core::{BasicParsers, Element};
use crate::extension::{BasicCombinator, BasicRepeatParsers, RepeatCombinator};
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl BasicRepeatParsers for ParsersImpl {
  fn any_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_any().many0().collect()
  }

  fn any_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_any().many1().collect()
  }

  fn any_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_any().many_n_m(n, m).collect()
  }

  fn any_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_any().count(n).collect()
  }

  fn space_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_space().many0().collect()
  }

  fn space_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_space().many1().collect()
  }

  fn space_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_space().many_n_m(n, m).collect()
  }

  fn space_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_space().count(n).collect()
  }

  fn multi_space_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_multi_space().many0().collect()
  }

  fn multi_space_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_multi_space().many1().collect()
  }

  fn multi_space_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_multi_space().many_n_m(n, m).collect()
  }

  fn multi_space_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_multi_space().count(n).collect()
  }

  fn alpha_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_alpha().many0().collect()
  }

  fn alpha_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_alpha().many1().collect()
  }

  fn alpha_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_alpha().many_n_m(n, m).collect()
  }

  fn alpha_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_alpha().count(n).collect()
  }

  fn digit_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_digit().many0().collect()
  }

  fn digit_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_digit().many1().collect()
  }

  fn digit_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_digit().many_n_m(n, m).collect()
  }

  fn digit_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_digit().count(n).collect()
  }

  fn hex_digit_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_hex_digit().many0().collect()
  }

  fn hex_digit_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_hex_digit().many1().collect()
  }

  fn hex_digit_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_hex_digit().many_n_m(n, m).collect()
  }

  fn hex_digit_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_hex_digit().count(n).collect()
  }

  fn oct_digit_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_oct_digit().many0().collect()
  }

  fn oct_digit_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_oct_digit().many1().collect()
  }

  fn oct_digit_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_oct_digit().many_n_m(n, m).collect()
  }

  fn oct_digit_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a, {
    Self::elm_oct_digit().count(n).collect()
  }
}
