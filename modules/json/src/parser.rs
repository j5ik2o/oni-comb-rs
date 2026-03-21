use std::borrow::Cow;
use std::collections::BTreeMap;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::Fail;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::value::JsonValue;

// ── Helpers ──────────────────────────────────────

/// Wrap a parser with surrounding optional whitespace.
fn ws<'a, P>(p: P) -> impl Parser<StrInputStream<'a>, Output = P::Output, Error = ParseError>
where
  P: Parser<StrInputStream<'a>, Error = ParseError>, {
  whitespace0().zip_right(p).zip_left(whitespace0())
}

fn comma<'a>() -> impl Parser<StrInputStream<'a>, Output = char, Error = ParseError> {
  whitespace0().zip_right(char(',')).zip_left(whitespace0())
}

fn colon<'a>() -> impl Parser<StrInputStream<'a>, Output = char, Error = ParseError> {
  whitespace0().zip_right(char(':')).zip_left(whitespace0())
}

// ── Primitives ──────────────────────────────────

fn json_null<'a>() -> impl Parser<StrInputStream<'a>, Output = JsonValue<'a>, Error = ParseError> {
  tag("null").map(|_| JsonValue::Null)
}

fn json_bool<'a>() -> impl Parser<StrInputStream<'a>, Output = JsonValue<'a>, Error = ParseError> {
  tag("true")
    .map(|_| JsonValue::Bool(true))
    .or(tag("false").map(|_| JsonValue::Bool(false)))
}

fn json_number<'a>() -> impl Parser<StrInputStream<'a>, Output = JsonValue<'a>, Error = ParseError> {
  float().map(JsonValue::Number)
}

fn json_string_value<'a>() -> impl Parser<StrInputStream<'a>, Output = JsonValue<'a>, Error = ParseError> {
  quoted_string().map(JsonValue::String)
}

// ── Compound (built via recursive) ──────────────

/// Build the complete JSON value parser using `recursive()` for self-referential
/// array/object definitions. All parsing is expressed as pure combinator pipelines
/// with no manual `parse_next` calls or mutable input manipulation.
fn build_json_value_parser<'a>() -> impl Parser<StrInputStream<'a>, Output = JsonValue<'a>, Error = ParseError> {
  recursive(|value| {
    // ── array: '[' ws (value (',' ws value)*)? ws ']' ──
    let array = char('[')
      .zip_right(whitespace0())
      .zip_right(value.clone().sep_by0(comma()))
      .zip_left(whitespace0())
      .zip_left(char(']').cut())
      .map(JsonValue::Array);

    // ── member: string ws ':' ws value ──
    let member = quoted_string().zip_left(colon().cut()).zip(value.clone());

    // ── object: '{' ws (member (',' ws member)*)? ws '}' ──
    let object = char('{')
      .zip_right(whitespace0())
      .zip_right(member.sep_by0(comma()))
      .zip_left(whitespace0())
      .zip_left(char('}').cut())
      .map(|pairs: Vec<(Cow<'a, str>, JsonValue<'a>)>| {
        JsonValue::Object(pairs.into_iter().collect::<BTreeMap<_, _>>())
      });

    // ── value: ws (null | bool | number | string | array | object) ──
    json_null()
      .or(json_bool())
      .or(json_number())
      .or(json_string_value())
      .or(array)
      .or(object)
      .context("JSON value")
  })
}

// ── Public API ──────────────────────────────────

/// JSON value parser (does not require EOF).
pub fn json_value<'a>() -> impl Parser<StrInputStream<'a>, Output = JsonValue<'a>, Error = ParseError> {
  ws(build_json_value_parser())
}

/// Complete JSON parser (value + optional whitespace + EOF).
pub fn json<'a>() -> impl Parser<StrInputStream<'a>, Output = JsonValue<'a>, Error = ParseError> {
  ws(build_json_value_parser()).zip_left(eof())
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
  let mut input = StrInputStream::new(src);
  json()
    .parse_next(&mut input)
    .map_err(|e| fail_to_error(e).fill_location_from_src(src))
}

/// Parse a JSON string, returning the value without requiring EOF.
pub fn parse_value(src: &str) -> Result<JsonValue<'_>, ParseError> {
  let mut input = StrInputStream::new(src);
  json_value()
    .parse_next(&mut input)
    .map_err(|e| fail_to_error(e).fill_location_from_src(src))
}
