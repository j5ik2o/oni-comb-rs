use std::collections::BTreeMap;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::char;

use crate::common::skip_inline_ws;
use crate::scalar::yaml_scalar;
use crate::value::YamlValue;
use crate::yaml_input::YamlInput;

// NOTE: YAML flow parsers use procedural style (explicit parse_next calls)
// because YamlInput (&mut) must be threaded for anchor/alias resolution.
// This prevents using pure combinator pipelines with sep_by0/recursive.

pub(crate) fn flow_value<'a>(input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
  skip_inline_ws(input)?;
  match input.peek_byte() {
    Some(b'[') => flow_sequence(input),
    Some(b'{') => flow_mapping(input),
    Some(b'*') => {
      // Alias in flow context
      let pos = input.offset();
      input.next_token();
      let remaining = input.remaining();
      let end = remaining.find([' ', '\n', ',', ']', '}']).unwrap_or(remaining.len());
      let name = &remaining[..end];
      input.advance(end);
      match input.get_anchor(name) {
        Some(v) => Ok(v.clone()),
        None => Err(Fail::Cut(ParseError::from_expected_with_location(
          pos,
          input.line(),
          input.column(),
          Expected::Description("known anchor"),
        ))),
      }
    }
    _ => yaml_scalar(input),
  }
}

fn flow_sequence<'a>(input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
  char('[').parse_next(input.inner_mut())?;
  skip_inline_ws(input)?;

  if input.peek_byte() == Some(b']') {
    char(']').parse_next(input.inner_mut())?;
    return Ok(YamlValue::Sequence(Vec::new()));
  }

  let mut items = Vec::new();
  items.push(flow_value(input)?);

  loop {
    skip_inline_ws(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input.inner_mut())?;
        skip_inline_ws(input)?;
        if input.peek_byte() == Some(b']') {
          break;
        }
        items.push(flow_value(input)?);
      }
      _ => break,
    }
  }

  skip_inline_ws(input)?;
  char(']').cut().parse_next(input.inner_mut())?;
  Ok(YamlValue::Sequence(items))
}

fn flow_mapping<'a>(input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
  char('{').parse_next(input.inner_mut())?;
  skip_inline_ws(input)?;

  if input.peek_byte() == Some(b'}') {
    char('}').parse_next(input.inner_mut())?;
    return Ok(YamlValue::Mapping(BTreeMap::new()));
  }

  let mut pairs = BTreeMap::new();
  let (key, val) = flow_member(input)?;
  pairs.insert(key, val);

  loop {
    skip_inline_ws(input)?;
    match input.peek_byte() {
      Some(b',') => {
        char(',').parse_next(input.inner_mut())?;
        skip_inline_ws(input)?;
        if input.peek_byte() == Some(b'}') {
          break;
        }
        let (key, val) = flow_member(input)?;
        pairs.insert(key, val);
      }
      _ => break,
    }
  }

  skip_inline_ws(input)?;
  char('}').cut().parse_next(input.inner_mut())?;
  Ok(YamlValue::Mapping(pairs))
}

fn flow_member<'a>(input: &mut YamlInput<'a>) -> PResult<(String, YamlValue), ParseError> {
  let key = flow_key(input)?;
  skip_inline_ws(input)?;
  char(':').cut().parse_next(input.inner_mut())?;
  skip_inline_ws(input)?;
  let val = flow_value(input)?;
  Ok((key, val))
}

fn flow_key<'a>(input: &mut YamlInput<'a>) -> PResult<String, ParseError> {
  match yaml_scalar(input)? {
    YamlValue::String(s) => Ok(s),
    YamlValue::Integer(n) => Ok(n.to_string()),
    YamlValue::Float(f) => Ok(f.to_string()),
    YamlValue::Bool(b) => Ok(b.to_string()),
    YamlValue::Null => Ok("null".to_string()),
    _ => Err(Fail::Backtrack(ParseError::from_expected_with_location(
      input.offset(),
      input.line(),
      input.column(),
      Expected::Description("scalar key"),
    ))),
  }
}
