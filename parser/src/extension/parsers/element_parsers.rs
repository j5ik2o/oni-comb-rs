use crate::core::{Element, Parser, Parsers, StaticParser, StaticParsers};
use crate::utils::Set;
use std::fmt::{Debug, Display};

pub trait ElementParsers: Parsers {
  fn elm_any_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|_| true)
  }

  fn elm_any<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_any_ref(), Clone::clone)
  }

  fn elm_ref<'a, I>(element: I) -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(move |actual| *actual == element)
  }

  fn elm<'a, I>(element: I) -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_ref(element), Clone::clone)
  }

  fn elm_pred_ref<'a, I, F>(f: F) -> Self::P<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a + 'static,
    I: Element + PartialEq + 'a + 'static;

  fn elm_pred<'a, I, F>(f: F) -> Self::P<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a + 'static,
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_pred_ref(f), Clone::clone)
  }

  fn elm_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_space())
  }

  fn elm_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_space_ref(), Clone::clone)
  }

  fn elm_multi_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_multi_space())
  }

  fn elm_multi_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_multi_space_ref(), Clone::clone)
  }

  fn elm_alpha_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_alpha())
  }

  fn elm_alpha<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_alpha_ref(), Clone::clone)
  }

  fn elm_alpha_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_alpha_digit())
  }

  fn elm_alpha_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_alpha_digit_ref(), Clone::clone)
  }

  fn elm_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_digit())
  }

  fn elm_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_digit_ref(), Clone::clone)
  }

  fn elm_hex_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_hex_digit())
  }

  fn elm_hex_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_hex_digit_ref(), Clone::clone)
  }

  fn elm_oct_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_oct_digit())
  }

  fn elm_oct_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    Self::map(Self::elm_oct_digit_ref(), Clone::clone)
  }

  fn elm_ref_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static;

  fn elm_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, I>
  where
    I: PartialEq + Clone + Display + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static, {
    Self::map(Self::elm_ref_of(set), Clone::clone)
  }

  fn elm_ref_in<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a + 'static;

  fn elm_in<'a, I>(start: I, end: I) -> Self::P<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + Clone + 'a + 'static, {
    Self::map(Self::elm_ref_in(start, end), Clone::clone)
  }

  fn elm_ref_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a + 'static;

  fn elm_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + Clone + 'a + 'static, {
    Self::map(Self::elm_ref_from_until(start, end), Clone::clone)
  }

  fn none_ref_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static;

  fn none_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static, {
    Self::map(Self::none_ref_of(set), Clone::clone)
  }
}

pub trait StaticElementParsers: StaticParsers {
  fn elm_any_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|_| true)
  }

  fn elm_any<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_ref<'a, I>(element: I) -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(move |actual| *actual == element)
  }

  fn elm<'a, I>(element: I) -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_pred_ref<'a, I, F>(f: F) -> Self::P<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a + 'static,
    I: Element + PartialEq + 'a;

  fn elm_pred<'a, I, F>(f: F) -> Self::P<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_space())
  }

  fn elm_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_multi_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_multi_space())
  }

  fn elm_multi_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_alpha_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_alpha())
  }

  fn elm_alpha<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_alpha_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_alpha_digit())
  }

  fn elm_alpha_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_digit())
  }

  fn elm_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_hex_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_hex_digit())
  }

  fn elm_hex_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_oct_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    Self::elm_pred_ref(|e: &I| e.is_ascii_oct_digit())
  }

  fn elm_oct_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static;

  fn elm_ref_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized + 'static;

  fn elm_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, I>
  where
    I: PartialEq + Clone + Display + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static;

  fn elm_ref_in<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a;

  fn elm_in<'a, I>(start: I, end: I) -> Self::P<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + Clone + 'a + 'static;

  fn elm_ref_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a;

  fn elm_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + Clone + 'a + 'static;

  fn none_ref_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized + 'static;

  fn none_of<'a, I, S>(set: &'static S) -> Self::P<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static;
}

// 既存のParserを使用する関数
pub fn elm_any_ref<'a, I>() -> Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a + 'static, {
  crate::internal::parsers_impl::ParsersImpl::elm_any_ref()
}

pub fn elm_any<'a, I>() -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a + 'static, {
  crate::internal::parsers_impl::ParsersImpl::elm_any()
}

pub fn elm_ref<'a, I>(element: I) -> Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a + 'static, {
  crate::internal::parsers_impl::ParsersImpl::elm_ref(element)
}

pub fn elm<'a, I>(element: I) -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a + 'static, {
  crate::internal::parsers_impl::ParsersImpl::elm(element)
}

pub fn elm_pred_ref<'a, I, F>(f: F) -> Parser<'a, I, &'a I>
where
  F: Fn(&I) -> bool + 'a + 'static,
  I: Element + PartialEq + 'a + 'static, {
  crate::internal::parsers_impl::ParsersImpl::elm_pred_ref(f)
}

pub fn elm_pred<'a, I, F>(f: F) -> Parser<'a, I, I>
where
  F: Fn(&I) -> bool + 'a + 'static,
  I: Element + Clone + PartialEq + 'a + 'static, {
  crate::internal::parsers_impl::ParsersImpl::elm_pred(f)
}

// StaticParserを使用する関数のモジュール
pub mod static_parsers {
  use super::*;

  // StaticParserを使用する関数
  pub fn elm_any_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a + 'static, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else {
        crate::core::ParseResult::successful(&input[0], 1)
      }
    })
  }

  pub fn elm_any<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else {
        crate::core::ParseResult::successful(input[0].clone(), 1)
      }
    })
  }

  pub fn elm_ref<'a, I>(element: I) -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Clone + 'a + 'static, {
    let element_clone = element.clone();
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if input[0] == element_clone {
        crate::core::ParseResult::successful(&input[0], 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected {:?}, but got {:?}", element_clone, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn elm<'a, I>(element: I) -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    let element_clone = element.clone();
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if input[0] == element_clone {
        crate::core::ParseResult::successful(input[0].clone(), 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected {:?}, but got {:?}", element_clone, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn elm_pred_ref<'a, I, F>(f: F) -> StaticParser<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a + 'static,
    I: Element + PartialEq + Clone + 'a + 'static, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if f(&input[0]) {
        crate::core::ParseResult::successful(&input[0], 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Predicate failed for {:?}", input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn elm_pred<'a, I, F>(f: F) -> StaticParser<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a + 'static,
    I: Element + Clone + PartialEq + 'a + 'static, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if f(&input[0]) {
        crate::core::ParseResult::successful(input[0].clone(), 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Predicate failed for {:?}", input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn elm_space_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Clone + 'a + 'static, {
    elm_pred_ref(|e: &I| e.is_ascii_space())
  }

  pub fn elm_space<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    elm_pred(|e: &I| e.is_ascii_space())
  }

  pub fn elm_multi_space_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Clone + 'a + 'static, {
    elm_pred_ref(|e: &I| e.is_ascii_multi_space())
  }

  pub fn elm_multi_space<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    elm_pred(|e: &I| e.is_ascii_multi_space())
  }

  pub fn elm_alpha_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Clone + 'a + 'static, {
    elm_pred_ref(|e: &I| e.is_ascii_alpha())
  }

  pub fn elm_alpha<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    elm_pred(|e: &I| e.is_ascii_alpha())
  }

  pub fn elm_alpha_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Clone + 'a + 'static, {
    elm_pred_ref(|e: &I| e.is_ascii_alpha_digit())
  }

  pub fn elm_alpha_digit<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    elm_pred(|e: &I| e.is_ascii_alpha_digit())
  }

  pub fn elm_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Clone + 'a + 'static, {
    elm_pred_ref(|e: &I| e.is_ascii_digit())
  }

  pub fn elm_digit<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    elm_pred(|e: &I| e.is_ascii_digit())
  }

  pub fn elm_hex_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Clone + 'a + 'static, {
    elm_pred_ref(|e: &I| e.is_ascii_hex_digit())
  }

  pub fn elm_hex_digit<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    elm_pred(|e: &I| e.is_ascii_hex_digit())
  }

  pub fn elm_oct_digit_ref<'a, I>() -> StaticParser<'a, I, &'a I>
  where
    I: Element + PartialEq + Clone + 'a + 'static, {
    elm_pred_ref(|e: &I| e.is_ascii_oct_digit())
  }

  pub fn elm_oct_digit<'a, I>() -> StaticParser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a + 'static, {
    elm_pred(|e: &I| e.is_ascii_oct_digit())
  }

  pub fn elm_ref_of<'a, I, S>(set: &'static S) -> StaticParser<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized + 'static + std::fmt::Debug, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if set.contains(&input[0]) {
        crate::core::ParseResult::successful(&input[0], 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected one of {:?}, but got {:?}", set, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn elm_of<'a, I, S>(set: &'static S) -> StaticParser<'a, I, I>
  where
    I: PartialEq + Clone + Display + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static + std::fmt::Debug, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if set.contains(&input[0]) {
        crate::core::ParseResult::successful(input[0].clone(), 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected one of {:?}, but got {:?}", set, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn elm_ref_in<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if input[0] >= start && input[0] <= end {
        crate::core::ParseResult::successful(&input[0], 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected between {:?} and {:?}, but got {:?}", start, end, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn elm_in<'a, I>(start: I, end: I) -> StaticParser<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + Clone + 'a + 'static, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if input[0] >= start && input[0] <= end {
        crate::core::ParseResult::successful(input[0].clone(), 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected between {:?} and {:?}, but got {:?}", start, end, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn elm_ref_from_until<'a, I>(start: I, end: I) -> StaticParser<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if input[0] >= start && input[0] < end {
        crate::core::ParseResult::successful(&input[0], 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected from {:?} until {:?}, but got {:?}", start, end, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn elm_from_until<'a, I>(start: I, end: I) -> StaticParser<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Debug + Copy + Clone + 'a + 'static, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if input[0] >= start && input[0] < end {
        crate::core::ParseResult::successful(input[0].clone(), 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected from {:?} until {:?}, but got {:?}", start, end, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn none_ref_of<'a, I, S>(set: &'static S) -> StaticParser<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized + 'static + std::fmt::Debug, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if !set.contains(&input[0]) {
        crate::core::ParseResult::successful(&input[0], 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected none of {:?}, but got {:?}", set, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }

  pub fn none_of<'a, I, S>(set: &'static S) -> StaticParser<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a + 'static,
    S: Set<I> + ?Sized + 'static + std::fmt::Debug, {
    StaticParser::new(move |state| {
      let input: &[I] = state.input();
      if input.is_empty() {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(state.next_offset(), None, "Unexpected EOF".to_string()),
          crate::core::CommittedStatus::Uncommitted,
        )
      } else if !set.contains(&input[0]) {
        crate::core::ParseResult::successful(input[0].clone(), 1)
      } else {
        crate::core::ParseResult::failed(
          crate::core::ParseError::of_custom(
            state.next_offset(),
            None,
            format!("Expected none of {:?}, but got {:?}", set, input[0]),
          ),
          crate::core::CommittedStatus::Uncommitted,
        )
      }
    })
  }
}
