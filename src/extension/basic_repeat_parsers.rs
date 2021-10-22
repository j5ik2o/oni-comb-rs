use std::fmt::Debug;

use crate::core::Element;
use crate::extension::RepeatCombinators;

pub trait BasicRepeatParsers: RepeatCombinators {
  fn any_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn any_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn any_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn any_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn space_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn space_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn space_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn space_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn multi_space_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn multi_space_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn multi_space_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn multi_space_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn alpha_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn alpha_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn alpha_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn alpha_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn digit_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn digit_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn digit_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn digit_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn hex_digit_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn hex_digit_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn hex_digit_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn hex_digit_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn oct_digit_many0<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn oct_digit_many1<'a, I>() -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn oct_digit_many_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;

  fn oct_digit_count<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]>
  where
    I: Element + PartialEq + Debug + 'a;
}
