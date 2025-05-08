use crate::core::{Element, Parsers};
use crate::utils::Set;
use std::fmt::{Debug, Display};

pub trait ElementParsers: Parsers {
  fn elm_any_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static, {
    Self::elm_pred_ref(|_| true)
  }

  fn elm_any<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_any_ref(), Clone::clone)
  }

  fn elm_ref<'a, I>(element: I) -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static, {
    Self::elm_pred_ref(move |actual| *actual == element)
  }

  fn elm<'a, I>(element: I) -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_ref(element), Clone::clone)
  }

  fn elm_pred_ref<'a, I, F>(f: F) -> Self::P<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'static,
    I: Element + 'static;

  fn elm_pred<'a, I, F>(f: F) -> Self::P<'a, I, I>
  where
    F: Fn(&I) -> bool + 'static,
    I: Element + 'static, {
    Self::map(Self::elm_pred_ref(f), Clone::clone)
  }

  fn elm_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_space())
  }

  fn elm_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_space_ref(), Clone::clone)
  }

  fn elm_multi_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_multi_space())
  }

  fn elm_multi_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_multi_space_ref(), Clone::clone)
  }

  fn elm_alpha_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_alpha())
  }

  fn elm_alpha<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_alpha_ref(), Clone::clone)
  }

  fn elm_alpha_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_alpha_digit())
  }

  fn elm_alpha_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_alpha_digit_ref(), Clone::clone)
  }

  fn elm_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_digit())
  }

  fn elm_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_digit_ref(), Clone::clone)
  }

  fn elm_hex_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_hex_digit())
  }

  fn elm_hex_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_hex_digit_ref(), Clone::clone)
  }

  fn elm_oct_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_oct_digit())
  }

  fn elm_oct_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_oct_digit_ref(), Clone::clone)
  }

  fn elm_ref_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static,
    S: Set<I> + ?Sized + 'static;

  fn elm_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, I>
  where
    I: Element + 'static,
    S: Set<I> + ?Sized + 'static, {
    Self::map(Self::elm_ref_of(set), Clone::clone)
  }

  fn elm_ref_in<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static;

  fn elm_in<'a, I>(start: I, end: I) -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_ref_in(start, end), Clone::clone)
  }

  fn elm_ref_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static;

  fn elm_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, I>
  where
    I: Element + 'static, {
    Self::map(Self::elm_ref_from_until(start, end), Clone::clone)
  }

  fn none_ref_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, &'a I>
  where
    I: Element + 'static,
    S: Set<I> + ?Sized + 'static;

  fn none_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, I>
  where
    I: Element + 'static,
    S: Set<I> + ?Sized + 'static, {
    Self::map(Self::none_ref_of(set), Clone::clone)
  }
}
