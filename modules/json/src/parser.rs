use std::borrow::Cow;
use std::collections::BTreeMap;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::value::JsonValue;

// ── Helpers ──────────────────────────────────────

#[inline]
fn skip_ws<'a>(input: &mut StrInput<'a>) -> PResult<(), ParseError> {
  whitespace0().parse_next(input).map(|_| ())
}

// ── Primitives ──────────────────────────────────

fn json_null<'a>(input: &mut StrInput<'a>) -> PResult<JsonValue<'a>, ParseError> {
  tag("null").map(|_| JsonValue::Null).parse_next(input)
}

fn json_bool<'a>(input: &mut StrInput<'a>) -> PResult<JsonValue<'a>, ParseError> {
  tag("true")
    .map(|_| JsonValue::Bool(true))
    .or(tag("false").map(|_| JsonValue::Bool(false)))
    .parse_next(input)
}

fn json_number<'a>(input: &mut StrInput<'a>) -> PResult<JsonValue<'a>, ParseError> {
  float().map(JsonValue::Number).parse_next(input)
}

fn json_string<'a>(input: &mut StrInput<'a>) -> PResult<JsonValue<'a>, ParseError> {
  quoted_string().map(JsonValue::String).parse_next(input)
}

// ── Compound ────────────────────────────────────

fn json_array<'a>(input: &mut StrInput<'a>) -> PResult<JsonValue<'a>, ParseError> {
  char('[').parse_next(input)?;
  skip_ws(input)?;

  if input.peek_byte() == Some(b']') {
    char(']').parse_next(input)?;
    return Ok(JsonValue::Array(Vec::new()));
  }

  let mut items = Vec::new();
  items.push(json_value_inner(input)?);

  loop {
    skip_ws(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input)?;
        skip_ws(input)?;
        items.push(json_value_inner(input)?);
      }
      _ => break,
    }
  }

  skip_ws(input)?;
  char(']').cut().parse_next(input)?;
  Ok(JsonValue::Array(items))
}

fn json_object<'a>(input: &mut StrInput<'a>) -> PResult<JsonValue<'a>, ParseError> {
  char('{').parse_next(input)?;
  skip_ws(input)?;

  if input.peek_byte() == Some(b'}') {
    char('}').parse_next(input)?;
    return Ok(JsonValue::Object(BTreeMap::new()));
  }

  let mut pairs = BTreeMap::new();
  let (key, val) = json_member(input)?;
  pairs.insert(key, val);

  loop {
    skip_ws(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input)?;
        skip_ws(input)?;
        let (key, val) = json_member(input)?;
        pairs.insert(key, val);
      }
      _ => break,
    }
  }

  skip_ws(input)?;
  char('}').cut().parse_next(input)?;
  Ok(JsonValue::Object(pairs))
}

fn json_member<'a>(input: &mut StrInput<'a>) -> PResult<(Cow<'a, str>, JsonValue<'a>), ParseError> {
  let key = quoted_string().parse_next(input)?;
  skip_ws(input)?;
  char(':').cut().parse_next(input)?;
  skip_ws(input)?;
  let val = json_value_inner(input)?;
  Ok((key, val))
}

// ── Value dispatch ──────────────────────────────

fn json_value_inner<'a>(input: &mut StrInput<'a>) -> PResult<JsonValue<'a>, ParseError> {
  skip_ws(input)?;
  match input.peek_byte() {
    Some(b'n') => json_null(input),
    Some(b't') | Some(b'f') => json_bool(input),
    Some(b'"') => json_string(input),
    Some(b'[') => json_array(input),
    Some(b'{') => json_object(input),
    Some(c) if c == b'-' || c.is_ascii_digit() => json_number(input),
    _ => {
      let pos = input.offset();
      Err(Fail::Backtrack(
        ParseError::from_expected(pos, Expected::Description("JSON value")).with_location(input.line(), input.column()),
      ))
    }
  }
}

/// JSON value parser (does not require EOF).
pub fn json_value<'a>() -> impl Parser<StrInput<'a>, Output = JsonValue<'a>, Error = ParseError> {
  fn_parser(json_value_inner)
}

/// Complete JSON parser (value + optional whitespace + EOF).
pub fn json<'a>() -> impl Parser<StrInput<'a>, Output = JsonValue<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let value = json_value_inner(input)?;
    skip_ws(input)?;
    eof().parse_next(input)?;
    Ok(value)
  })
}

fn fail_to_error(e: Fail<ParseError>) -> ParseError {
  match e {
    Fail::Backtrack(e) | Fail::Cut(e) => e,
    Fail::Incomplete => ParseError::from_expected(0, Expected::Description("incomplete input")),
    Fail::ZeroProgress => ParseError::from_expected(0, Expected::Description("zero progress")),
  }
}

/// Parse a JSON string, returning the parsed value or an error.
pub fn parse(src: &str) -> Result<JsonValue<'_>, ParseError> {
  let mut input = StrInput::new(src);
  let value = json_value_inner(&mut input).map_err(fail_to_error)?;
  skip_ws(&mut input).map_err(fail_to_error)?;
  if input.is_eof() {
    Ok(value)
  } else {
    let pos = input.offset();
    Err(ParseError::from_expected(pos, Expected::Eof).with_location(input.line(), input.column()))
  }
}

/// Parse a JSON string, returning the value without requiring EOF.
pub fn parse_value(src: &str) -> Result<JsonValue<'_>, ParseError> {
  let mut input = StrInput::new(src);
  json_value_inner(&mut input).map_err(fail_to_error)
}
