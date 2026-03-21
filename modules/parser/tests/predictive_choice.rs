use std::cell::Cell;
use std::rc::Rc;

use oni_comb_parser::byte_input_stream::ByteInputStream;
use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

fn is_number_start(byte: u8) -> bool {
  byte == b'-' || byte.is_ascii_digit()
}

#[test]
fn predictive_choice_selects_branch_from_leading_byte() {
  let mut parser = predictive_choice::<StrInputStream<'_>, &'static str>()
    .when_byte(b'n', tag("null"))
    .when_byte(b't', tag("true"));
  let mut input = StrInputStream::new("true");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "true");
  assert_eq!(input.remaining(), "");
}

#[test]
fn predictive_choice_does_not_consume_input_on_unmatched_byte() {
  let mut parser = predictive_choice::<StrInputStream<'_>, &'static str>()
    .when_byte(b'n', tag("null"))
    .when_byte(b't', tag("true"));
  let mut input = StrInputStream::new("x");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn predictive_choice_propagates_selected_branch_backtrack_without_fallback() {
  let selected_hits = Rc::new(Cell::new(0));
  let other_hits = Rc::new(Cell::new(0));
  let selected_hits_2 = Rc::clone(&selected_hits);
  let other_hits_2 = Rc::clone(&other_hits);

  let selected = fn_parser(move |input: &mut StrInputStream<'_>| {
    selected_hits_2.set(selected_hits_2.get() + 1);
    Err(Fail::Backtrack(ParseError::from_position(
      input.position(),
      Expected::Description("selected branch"),
    )))
  });
  let other = fn_parser(move |_input: &mut StrInputStream<'_>| {
    other_hits_2.set(other_hits_2.get() + 1);
    Ok::<_, Fail<ParseError>>("true")
  });

  let mut parser = predictive_choice::<StrInputStream<'_>, &'static str>()
    .when_byte(b'n', selected)
    .when_byte(b't', other);
  let mut input = StrInputStream::new("null");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
  assert_eq!(selected_hits.get(), 1);
  assert_eq!(other_hits.get(), 0);
}

#[test]
fn predictive_choice_propagates_selected_branch_cut() {
  let selected = fn_parser(move |input: &mut StrInputStream<'_>| {
    Err::<&'static str, _>(Fail::Cut(ParseError::from_position(
      input.position(),
      Expected::Description("selected cut"),
    )))
  });

  let mut parser = predictive_choice::<StrInputStream<'_>, &'static str>().when_byte(b'n', selected);
  let mut input = StrInputStream::new("null");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn predictive_choice_supports_predicate_branch_for_numbers() {
  let mut parser =
    predictive_choice::<StrInputStream<'_>, i64>().when_predicate(is_number_start, integer());
  let mut input = StrInputStream::new("-42");

  assert_eq!(parser.parse_next(&mut input).unwrap(), -42);
}

#[test]
fn predictive_choice_works_with_byte_input() {
  let mut parser = predictive_choice::<ByteInputStream<'_>, &[u8]>()
    .when_byte(b'n', seq::<ByteInputStream<'_>, [u8]>(&b"null"[..]))
    .when_predicate(is_number_start, take_while1::<ByteInputStream<'_>, _>(|b: u8| b.is_ascii_digit()));
  let mut input = ByteInputStream::new(b"null");

  assert_eq!(parser.parse_next(&mut input).unwrap(), b"null");
}
