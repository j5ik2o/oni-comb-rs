use std::collections::BTreeMap;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::char;

use crate::common::{at_document_marker, current_indent, skip_inline_ws, skip_ws_and_comments};

// NOTE: YAML block parsers use procedural style because:
// 1. YamlInput (&mut) must be threaded for anchor/alias resolution
// 2. Indent-based parsing requires runtime state (indent stack)
//    that cannot be expressed as static combinator composition
// These constraints make pure pipeline style impractical for block YAML.
use crate::flow::flow_value;
use crate::multiline::block_scalar;
use crate::scalar::yaml_scalar;
use crate::value::YamlValue;
use crate::yaml_input::YamlInput;

/// Parse an optional anchor prefix (&name) and return the anchor name.
fn parse_anchor_prefix<'a>(input: &mut YamlInput<'a>) -> PResult<Option<String>, ParseError> {
  if input.peek_byte() != Some(b'&') {
    return Ok(None);
  }
  input.next_token(); // consume '&'
  let remaining = input.remaining();
  let end = remaining
    .find([' ', '\n', '\r', '\t', ',', ']', '}', ':'])
    .unwrap_or(remaining.len());
  if end == 0 {
    return Err(Fail::Cut(ParseError::from_expected_with_location(
      input.offset(),
      input.line(),
      input.column(),
      Expected::Description("anchor name"),
    )));
  }
  let name = remaining[..end].to_string();
  input.advance(end);
  skip_inline_ws(input)?;
  Ok(Some(name))
}

/// Parse an alias (*name) and resolve it from the YamlInput anchor map.
fn parse_alias<'a>(input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
  let pos = input.offset();
  input.next_token(); // consume '*'
  let remaining = input.remaining();
  let end = remaining
    .find([' ', '\n', '\r', '\t', ',', ']', '}'])
    .unwrap_or(remaining.len());
  if end == 0 {
    return Err(Fail::Cut(ParseError::from_expected_with_location(
      pos,
      input.line(),
      input.column(),
      Expected::Description("alias name"),
    )));
  }
  let name = &remaining[..end];
  input.advance(end);
  match input.get_anchor(name) {
    Some(value) => Ok(value.clone()),
    None => Err(Fail::Cut(ParseError::from_expected_with_location(
      pos,
      input.line(),
      input.column(),
      Expected::Description("known anchor"),
    ))),
  }
}

/// Parse a block value using the indent stack in YamlInput.
pub(crate) fn block_value<'a>(input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
  let min_indent = input.current_min_indent();

  skip_ws_and_comments(input)?;

  // Check for alias
  if input.peek_byte() == Some(b'*') {
    return parse_alias(input);
  }

  // Check for anchor prefix
  let anchor = parse_anchor_prefix(input)?;

  // After consuming anchor, the value may start on the next line
  if anchor.is_some() {
    skip_ws_and_comments(input)?;
  }

  let value = match input.peek_byte() {
    Some(b'[') => flow_value(input)?,
    Some(b'{') => flow_value(input)?,
    Some(b'-') if is_block_seq_indicator(input) => block_sequence(input, min_indent)?,
    Some(b'|') | Some(b'>') => block_scalar(input)?,
    Some(b'*') => parse_alias(input)?,
    _ => {
      let cp = input.checkpoint();
      let indent = current_indent(input);
      if indent < min_indent {
        return Err(Fail::Backtrack(ParseError::from_expected_with_location(
          input.offset(),
          input.line(),
          input.column(),
          Expected::Description("indented content"),
        )));
      }

      match try_block_mapping(input, indent) {
        Ok(v) => v,
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          yaml_scalar(input)?
        }
        Err(e) => return Err(e),
      }
    }
  };

  // Save anchor if present
  if let Some(name) = anchor {
    input.set_anchor(name, value.clone());
  }

  Ok(value)
}

fn is_block_seq_indicator<'a>(input: &YamlInput<'a>) -> bool {
  let remaining = input.remaining();
  let bytes = remaining.as_bytes();
  (bytes.len() >= 2 && bytes[0] == b'-' && (bytes[1] == b' ' || bytes[1] == b'\n'))
    || (bytes.len() == 1 && bytes[0] == b'-')
}

fn block_sequence<'a>(input: &mut YamlInput<'a>, min_indent: usize) -> PResult<YamlValue, ParseError> {
  let seq_indent = current_indent(input);
  if seq_indent < min_indent {
    return Err(Fail::Backtrack(ParseError::from_expected_with_location(
      input.offset(),
      input.line(),
      input.column(),
      Expected::Description("indented sequence"),
    )));
  }

  let mut items = Vec::new();

  loop {
    let cur_indent = current_indent(input);
    if cur_indent != seq_indent {
      break;
    }

    if at_document_marker(input).is_some() || input.peek_byte() != Some(b'-') {
      break;
    }

    char('-').parse_next(input.inner_mut())?;
    if input.peek_byte() == Some(b' ') {
      input.next_token();
    }

    input.push_indent(seq_indent + 1);
    let item = block_value(input)?;
    input.pop_indent();
    items.push(item);

    skip_ws_and_comments(input)?;
    if input.is_eof() {
      break;
    }
  }

  Ok(YamlValue::Sequence(items))
}

fn try_block_mapping<'a>(input: &mut YamlInput<'a>, map_indent: usize) -> PResult<YamlValue, ParseError> {
  let mut pairs = BTreeMap::new();

  loop {
    let cur_indent = current_indent(input);
    if cur_indent != map_indent || at_document_marker(input).is_some() {
      break;
    }

    // Check for merge key (<<)
    let key = parse_mapping_key(input)?;
    skip_inline_ws(input)?;
    char(':').parse_next(input.inner_mut())?;

    skip_inline_ws(input)?;

    let val = if input.peek_byte() == Some(b'\n') || input.peek_byte() == Some(b'#') || input.is_eof() {
      skip_ws_and_comments(input)?;
      if input.is_eof() {
        YamlValue::Null
      } else {
        let next_indent = current_indent(input);
        if next_indent > map_indent {
          input.push_indent(next_indent);
          let v = block_value(input)?;
          input.pop_indent();
          v
        } else {
          YamlValue::Null
        }
      }
    } else {
      input.push_indent(map_indent + 1);
      let v = block_value(input)?;
      input.pop_indent();
      v
    };

    // Handle merge key
    if key == "<<" {
      if let YamlValue::Mapping(merge_map) = &val {
        for (mk, mv) in merge_map {
          pairs.entry(mk.clone()).or_insert_with(|| mv.clone());
        }
      }
    } else {
      pairs.insert(key, val);
    }

    skip_ws_and_comments(input)?;
    if input.is_eof() {
      break;
    }
  }

  if pairs.is_empty() {
    return Err(Fail::Backtrack(ParseError::from_expected_with_location(
      input.offset(),
      input.line(),
      input.column(),
      Expected::Description("mapping"),
    )));
  }

  Ok(YamlValue::Mapping(pairs))
}

fn parse_mapping_key<'a>(input: &mut YamlInput<'a>) -> PResult<String, ParseError> {
  match input.peek_byte() {
    Some(b'"') | Some(b'\'') => match yaml_scalar(input)? {
      YamlValue::String(s) => Ok(s),
      other => Ok(format!("{:?}", other)),
    },
    _ => {
      let remaining = input.remaining();
      let bytes = remaining.as_bytes();
      let mut end = 0;

      while end < bytes.len() {
        let b = bytes[end];
        if b == b':' && (end + 1 >= bytes.len() || bytes[end + 1] == b' ' || bytes[end + 1] == b'\n') {
          break;
        }
        if b == b'\n' || b == b'\r' {
          break;
        }
        end += 1;
      }

      if end == 0 {
        return Err(Fail::Backtrack(ParseError::from_expected_with_location(
          input.offset(),
          input.line(),
          input.column(),
          Expected::Description("mapping key"),
        )));
      }

      let key = remaining[..end].trim_end().to_string();
      input.advance(end);
      Ok(key)
    }
  }
}
