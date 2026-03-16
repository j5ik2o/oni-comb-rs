use winnow::ascii::digit1;
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::token::{one_of, take_while};

type WErr = winnow::error::ErrMode<ContextError>;

pub fn parse_identifier(s: &str) -> Option<String> {
  let mut input = s;
  let head: Result<char, WErr> = one_of(|c: char| c.is_ascii_alphabetic() || c == '_').parse_next(&mut input);
  head.ok().map(|h| {
    let tail: Result<&str, WErr> =
      take_while(0.., |c: char| c.is_ascii_alphanumeric() || c == '_').parse_next(&mut input);
    let tail = tail.expect("take_while(0..) never fails");
    let mut result = String::with_capacity(1 + tail.len());
    result.push(h);
    result.push_str(tail);
    result
  })
}

pub fn parse_integer(s: &str) -> Option<u64> {
  let mut input = s;
  let digits: Result<&str, WErr> = digit1.parse_next(&mut input);
  digits.ok().and_then(|d| d.parse::<u64>().ok())
}

/// flat_map 同一型分岐: digit → tag
pub fn parse_flat_map_same_type(s: &str) -> Option<&str> {
  let mut input = s;
  let result: Result<&str, WErr> = one_of(|c: char| c.is_ascii_digit())
    .flat_map(|c: char| match c {
      '1' => "one",
      '2' => "two",
      '3' => "three",
      _ => "",
    })
    .parse_next(&mut input);
  result.ok()
}

/// flat_map 異種型分岐: 先頭文字に応じて異なる型のパーサーを選択
pub fn parse_flat_map_boxed(s: &str) -> Option<(&str, &str)> {
  let mut input = s;
  let result: Result<(&str, &str), WErr> = one_of(|c: char| c == 'c' || c == 'i')
    .flat_map(|t: char| -> Box<dyn winnow::Parser<&str, (&str, &str), ContextError>> {
      match t {
        'c' => Box::new((":", take_while(1.., |c: char| c.is_ascii_alphabetic()))),
        _ => Box::new((":", take_while(1.., |c: char| c.is_ascii_digit()))),
      }
    })
    .parse_next(&mut input);
  result.ok()
}
