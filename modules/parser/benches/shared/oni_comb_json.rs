use std::borrow::Cow;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) enum Json<'a> {
  Null,
  Bool(bool),
  Num(f64),
  Str(Cow<'a, str>),
  Array(Vec<Json<'a>>),
  Object(Vec<(Cow<'a, str>, Json<'a>)>),
}

#[inline]
fn skip_ws<'a>(input: &mut StrInput<'a>) -> PResult<(), ParseError> {
  whitespace0().parse_next(input).map(|_| ())
}

#[inline]
fn json_value_body<'a>(input: &mut StrInput<'a>) -> PResult<Json<'a>, ParseError> {
  match input.peek_byte() {
    Some(b'n') => tag("null").map(|_| Json::Null).parse_next(input),
    Some(b't') => tag("true").map(|_| Json::Bool(true)).parse_next(input),
    Some(b'f') => tag("false").map(|_| Json::Bool(false)).parse_next(input),
    Some(b'"') => quoted_string().map(Json::Str).parse_next(input),
    Some(b'[') => json_array(input),
    Some(b'{') => json_object(input),
    Some(c) if c == b'-' || c.is_ascii_digit() => {
      take_while1(|c: char| c.is_ascii_digit() || c == '-' || c == '.' || c == 'e' || c == 'E' || c == '+')
        .map(|s: &str| Json::Num(s.parse::<f64>().unwrap()))
        .parse_next(input)
    }
    _ => Err(Fail::Backtrack(ParseError::from_expected(
      input.offset(),
      Expected::Description("JSON value"),
    ))),
  }
}

pub(crate) fn json_value<'a>(input: &mut StrInput<'a>) -> PResult<Json<'a>, ParseError> {
  skip_ws(input)?;
  json_value_body(input)
}

fn json_array<'a>(input: &mut StrInput<'a>) -> PResult<Json<'a>, ParseError> {
  char('[').parse_next(input)?;
  let mut items = Vec::new();
  skip_ws(input)?;
  if input.peek_byte() == Some(b']') {
    char(']').parse_next(input)?;
    return Ok(Json::Array(items));
  }

  items.push(json_value_body(input)?);
  loop {
    skip_ws(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input)?;
        skip_ws(input)?;
        items.push(json_value_body(input)?);
      }
      _ => break,
    }
  }
  skip_ws(input)?;
  char(']').parse_next(input)?;
  Ok(Json::Array(items))
}

fn json_object<'a>(input: &mut StrInput<'a>) -> PResult<Json<'a>, ParseError> {
  char('{').parse_next(input)?;
  let mut pairs = Vec::new();
  skip_ws(input)?;
  if input.peek_byte() == Some(b'}') {
    char('}').parse_next(input)?;
    return Ok(Json::Object(pairs));
  }

  pairs.push(json_member(input)?);
  loop {
    skip_ws(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input)?;
        skip_ws(input)?;
        pairs.push(json_member(input)?);
      }
      _ => break,
    }
  }
  skip_ws(input)?;
  char('}').parse_next(input)?;
  Ok(Json::Object(pairs))
}

fn json_member<'a>(input: &mut StrInput<'a>) -> PResult<(Cow<'a, str>, Json<'a>), ParseError> {
  let key = quoted_string().parse_next(input)?;
  skip_ws(input)?;
  char(':').parse_next(input)?;
  skip_ws(input)?;
  let val = json_value_body(input)?;
  Ok((key, val))
}

#[allow(dead_code)]
pub(crate) fn json_parser<'a>() -> impl Parser<StrInput<'a>, Output = Json<'a>, Error = ParseError> {
  fn_parser(json_value)
}

#[allow(dead_code)]
pub(crate) fn parse_complete<'a>(src: &'a str) -> PResult<Json<'a>, ParseError> {
  let mut input = StrInput::new(src);
  let value = json_value(&mut input)?;
  skip_ws(&mut input)?;
  if input.is_eof() {
    Ok(value)
  } else {
    Err(Fail::Backtrack(ParseError::from_expected(
      input.offset(),
      Expected::Description("end of input"),
    )))
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn parses_whitespace_padded_object_like_compact_form() {
    let compact = r#"{"a":[1,2],"b":{"c":true}}"#;
    let spaced = r#" { "a" : [ 1 , 2 ] , "b" : { "c" : true } } "#;

    assert_eq!(super::parse_complete(compact).unwrap(), super::parse_complete(spaced).unwrap());
  }

  #[test]
  fn parses_whitespace_padded_array_like_compact_form() {
    let compact = r#"[1,"two",true,null]"#;
    let spaced = r#"[ 1 , "two" , true , null ]"#;

    assert_eq!(super::parse_complete(compact).unwrap(), super::parse_complete(spaced).unwrap());
  }

  #[test]
  fn preserves_member_whitespace_boundaries() {
    let compact = r#"{"name":"oni-comb","version":2}"#;
    let spaced = r#"{ "name" : "oni-comb" , "version" : 2 }"#;

    let expected = super::Json::Object(vec![
      (
        std::borrow::Cow::Borrowed("name"),
        super::Json::Str(std::borrow::Cow::Borrowed("oni-comb")),
      ),
      (std::borrow::Cow::Borrowed("version"), super::Json::Num(2.0)),
    ]);

    assert_eq!(super::parse_complete(compact).unwrap(), expected);
    assert_eq!(super::parse_complete(spaced).unwrap(), expected);
  }
}
