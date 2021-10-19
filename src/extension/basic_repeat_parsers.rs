use std::fmt::Debug;

use crate::core::Element;
use crate::extension::RepeatCombinators;

pub trait BasicRepeatParsers: RepeatCombinators {
  fn any_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn any_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn any_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn any_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn space_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn space_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn space_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn space_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn multi_space_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn multi_space_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn multi_space_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn multi_space_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn alphabet_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn alphabet_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn alphabet_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn alphabet_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn digit_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn digit_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn digit_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn digit_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn hex_digit_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn hex_digit_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn hex_digit_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn hex_digit_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn oct_digit_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn oct_digit_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn oct_digit_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;

  fn oct_digit_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a;
}
