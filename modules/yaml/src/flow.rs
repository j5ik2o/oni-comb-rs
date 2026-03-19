use std::collections::BTreeMap;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

use crate::common::skip_inline_ws;
use crate::context::ParseContext;
use crate::scalar::yaml_scalar;
use crate::value::YamlValue;

// NOTE: YAML flow parsers use procedural style (explicit parse_next calls)
// because ParseContext (&mut) must be threaded through recursive calls.
// This prevents using pure combinator pipelines with sep_by0/recursive.
// JSON parser (which has no context) uses the pure pipeline style.

pub(crate) fn flow_value<'a>(input: &mut StrInputStream<'a>, ctx: &mut ParseContext) -> PResult<YamlValue, ParseError> {
  skip_inline_ws(input)?;
  match input.peek_byte() {
    Some(b'[') => flow_sequence(input, ctx),
    Some(b'{') => flow_mapping(input, ctx),
    Some(b'*') => {
      // Alias in flow context
      let pos = input.offset();
      input.next_token();
      let remaining = input.remaining();
      let end = remaining.find([' ', '\n', ',', ']', '}']).unwrap_or(remaining.len());
      let name = &remaining[..end];
      input.advance(end);
      match ctx.get_anchor(name) {
        Some(v) => Ok(v.clone()),
        None => Err(Fail::Cut(ParseError::from_expected(
          pos,
          Expected::Description("known anchor"),
        ))),
      }
    }
    _ => yaml_scalar(input),
  }
}

fn flow_sequence<'a>(input: &mut StrInputStream<'a>, ctx: &mut ParseContext) -> PResult<YamlValue, ParseError> {
  char('[').parse_next(input)?;
  skip_inline_ws(input)?;

  if input.peek_byte() == Some(b']') {
    char(']').parse_next(input)?;
    return Ok(YamlValue::Sequence(Vec::new()));
  }

  let mut items = Vec::new();
  items.push(flow_value(input, ctx)?);

  loop {
    skip_inline_ws(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input)?;
        skip_inline_ws(input)?;
        if input.peek_byte() == Some(b']') {
          break;
        }
        items.push(flow_value(input, ctx)?);
      }
      _ => break,
    }
  }

  skip_inline_ws(input)?;
  char(']').cut().parse_next(input)?;
  Ok(YamlValue::Sequence(items))
}

fn flow_mapping<'a>(input: &mut StrInputStream<'a>, ctx: &mut ParseContext) -> PResult<YamlValue, ParseError> {
  char('{').parse_next(input)?;
  skip_inline_ws(input)?;

  if input.peek_byte() == Some(b'}') {
    char('}').parse_next(input)?;
    return Ok(YamlValue::Mapping(BTreeMap::new()));
  }

  let mut pairs = BTreeMap::new();
  let (key, val) = flow_member(input, ctx)?;
  pairs.insert(key, val);

  loop {
    skip_inline_ws(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input)?;
        skip_inline_ws(input)?;
        if input.peek_byte() == Some(b'}') {
          break;
        }
        let (key, val) = flow_member(input, ctx)?;
        pairs.insert(key, val);
      }
      _ => break,
    }
  }

  skip_inline_ws(input)?;
  char('}').cut().parse_next(input)?;
  Ok(YamlValue::Mapping(pairs))
}

fn flow_member<'a>(input: &mut StrInputStream<'a>, ctx: &mut ParseContext) -> PResult<(String, YamlValue), ParseError> {
  let key = flow_key(input)?;
  skip_inline_ws(input)?;
  char(':').cut().parse_next(input)?;
  skip_inline_ws(input)?;
  let val = flow_value(input, ctx)?;
  Ok((key, val))
}

fn flow_key<'a>(input: &mut StrInputStream<'a>) -> PResult<String, ParseError> {
  match yaml_scalar(input)? {
    YamlValue::String(s) => Ok(s),
    YamlValue::Integer(n) => Ok(n.to_string()),
    YamlValue::Float(f) => Ok(f.to_string()),
    YamlValue::Bool(b) => Ok(b.to_string()),
    YamlValue::Null => Ok("null".to_string()),
    _ => Err(Fail::Backtrack(ParseError::from_expected(
      input.offset(),
      Expected::Description("scalar key"),
    ))),
  }
}
