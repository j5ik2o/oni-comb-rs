use crate::core::{Element, ElementParsers, ParseError, ParseResult, Parser};
use crate::internal::ParsersImpl;

impl ElementParsers for ParsersImpl {
  fn elm_pred<'a, I, F>(f: F) -> Self::P<'a, I, &'a I>
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
      let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
      ParseResult::failed_with_un_commit(pe)
    })
  }

  fn elm_space<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_space)
  }

  fn elm_multi_space<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_multi_space)
  }

  fn elm_alpha<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_alpha)
  }

  fn elm_alpha_digit<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_alpha_digit)
  }

  fn elm_digit<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_digit)
  }

  fn elm_hex_digit<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_hex_digit)
  }

  fn elm_oct_digit<'a, I>() -> Self::P<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_oct_digit)
  }
}
