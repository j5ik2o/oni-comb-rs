use crate::core::{Element, ParseError, ParseResult, Parser};
use crate::extension::parsers::ElementParsers;
use crate::internal::ParsersImpl;
use crate::utils::Set;
use std::fmt::{Debug, Display};

impl ElementParsers for ParsersImpl {
  fn elm_pred_ref<'a, I, F>(f: F) -> Self::P<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + PartialEq + 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(actual) = input.get(0) {
        if f(actual) {
          return ParseResult::successful(actual, 1);
        }
      }
      let offset = parse_state.next_offset();
      let msg = format!("offset: {}", offset);
      let ps = parse_state.add_offset(1);
      let pe = ParseError::of_mismatch(input, ps.next_offset(), 1, msg);
      ParseResult::failed_with_un_commit(pe)
    })
  }

  fn elm_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred_ref(Element::is_ascii_space)
  }

  fn elm_multi_space_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred_ref(Element::is_ascii_multi_space)
  }

  fn elm_alpha_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred_ref(Element::is_ascii_alpha)
  }

  fn elm_alpha_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred_ref(Element::is_ascii_alpha_digit)
  }

  fn elm_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred_ref(Element::is_ascii_digit)
  }

  fn elm_hex_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred_ref(Element::is_ascii_hex_digit)
  }

  fn elm_oct_digit_ref<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred_ref(Element::is_ascii_oct_digit)
  }

  fn elm_ref_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if set.contains(s) {
          ParseResult::successful(s, 1)
        } else {
          let msg = format!("expect one of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), 1, msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn elm_ref_in<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    Parser::new(move |parse_state| {
      let set = start..=end;
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if set.contains(s) {
          ParseResult::successful(s, 1)
        } else {
          let msg = format!("expect elm of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), 1, msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn elm_ref_from_until<'a, I>(start: I, end: I) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    Parser::new(move |parse_state| {
      let set = start..end;
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if set.contains(s) {
          ParseResult::successful(s, 1)
        } else {
          let msg = format!("expect elm of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), 1, msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn none_ref_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if !set.contains(s) {
          ParseResult::successful(s, 1)
        } else {
          let msg = format!("expect none of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), 1, msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }
}
