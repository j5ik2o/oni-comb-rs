use std::collections::BTreeMap;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

use crate::common::{current_indent, skip_inline_ws, skip_ws_and_comments};
use crate::context::ParseContext;
use crate::flow::flow_value;
use crate::multiline::block_scalar;
use crate::scalar::yaml_scalar;
use crate::value::YamlValue;

fn at_document_marker<'a>(input: &StrInput<'a>) -> bool {
  let remaining = input.remaining();
  (remaining.starts_with("---") || remaining.starts_with("..."))
    && (remaining.len() == 3
      || remaining.as_bytes().get(3).copied() == Some(b'\n')
      || remaining.as_bytes().get(3).copied() == Some(b' ')
      || remaining.as_bytes().get(3).copied() == Some(b'\r'))
}

/// Parse an optional anchor prefix (&name) and return the anchor name.
fn parse_anchor_prefix<'a>(input: &mut StrInput<'a>) -> PResult<Option<String>, ParseError> {
  if input.peek_byte() != Some(b'&') {
    return Ok(None);
  }
  input.next_token(); // consume '&'
  let remaining = input.remaining();
  let end = remaining
    .find([' ', '\n', '\r', '\t', ',', ']', '}', ':'])
    .unwrap_or(remaining.len());
  if end == 0 {
    return Err(Fail::Cut(ParseError::from_expected(
      input.offset(),
      Expected::Description("anchor name"),
    )));
  }
  let name = remaining[..end].to_string();
  input.advance(end);
  skip_inline_ws(input)?;
  Ok(Some(name))
}

/// Parse an alias (*name) and resolve it from the context.
fn parse_alias<'a>(input: &mut StrInput<'a>, ctx: &ParseContext) -> PResult<YamlValue, ParseError> {
  let pos = input.offset();
  input.next_token(); // consume '*'
  let remaining = input.remaining();
  let end = remaining
    .find([' ', '\n', '\r', '\t', ',', ']', '}'])
    .unwrap_or(remaining.len());
  if end == 0 {
    return Err(Fail::Cut(ParseError::from_expected(
      pos,
      Expected::Description("alias name"),
    )));
  }
  let name = &remaining[..end];
  input.advance(end);
  match ctx.get_anchor(name) {
    Some(value) => Ok(value.clone()),
    None => Err(Fail::Cut(ParseError::from_expected(
      pos,
      Expected::Description("known anchor"),
    ))),
  }
}

/// Parse a block value at the given minimum indent level.
pub(crate) fn block_value<'a>(
  input: &mut StrInput<'a>,
  min_indent: usize,
  ctx: &mut ParseContext,
) -> PResult<YamlValue, ParseError> {
  skip_ws_and_comments(input)?;

  // Check for alias
  if input.peek_byte() == Some(b'*') {
    return parse_alias(input, ctx);
  }

  // Check for anchor prefix
  let anchor = parse_anchor_prefix(input)?;

  let value = match input.peek_byte() {
    Some(b'[') => flow_value(input, ctx)?,
    Some(b'{') => flow_value(input, ctx)?,
    Some(b'-') if is_block_seq_indicator(input) => block_sequence(input, min_indent, ctx)?,
    Some(b'|') | Some(b'>') => block_scalar(input)?,
    Some(b'*') => parse_alias(input, ctx)?,
    _ => {
      let cp = input.checkpoint();
      let indent = current_indent(input);
      if indent < min_indent {
        return Err(Fail::Backtrack(ParseError::from_expected(
          input.offset(),
          Expected::Description("indented content"),
        )));
      }

      match try_block_mapping(input, indent, ctx) {
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
    ctx.set_anchor(name, value.clone());
  }

  Ok(value)
}

fn is_block_seq_indicator<'a>(input: &StrInput<'a>) -> bool {
  let remaining = input.remaining();
  let bytes = remaining.as_bytes();
  (bytes.len() >= 2 && bytes[0] == b'-' && (bytes[1] == b' ' || bytes[1] == b'\n'))
    || (bytes.len() == 1 && bytes[0] == b'-')
}

fn block_sequence<'a>(
  input: &mut StrInput<'a>,
  min_indent: usize,
  ctx: &mut ParseContext,
) -> PResult<YamlValue, ParseError> {
  let seq_indent = current_indent(input);
  if seq_indent < min_indent {
    return Err(Fail::Backtrack(ParseError::from_expected(
      input.offset(),
      Expected::Description("indented sequence"),
    )));
  }

  let mut items = Vec::new();

  loop {
    let cur_indent = current_indent(input);
    if cur_indent != seq_indent {
      break;
    }

    if at_document_marker(input) || input.peek_byte() != Some(b'-') {
      break;
    }

    char('-').parse_next(input)?;
    if input.peek_byte() == Some(b' ') {
      input.next_token();
    }

    let item = block_value(input, seq_indent + 1, ctx)?;
    items.push(item);

    skip_ws_and_comments(input)?;
    if input.is_eof() {
      break;
    }
  }

  Ok(YamlValue::Sequence(items))
}

fn try_block_mapping<'a>(
  input: &mut StrInput<'a>,
  map_indent: usize,
  ctx: &mut ParseContext,
) -> PResult<YamlValue, ParseError> {
  let mut pairs = BTreeMap::new();

  loop {
    let cur_indent = current_indent(input);
    if cur_indent != map_indent || at_document_marker(input) {
      break;
    }

    // Check for merge key (<<)
    let key = parse_mapping_key(input)?;
    skip_inline_ws(input)?;
    char(':').parse_next(input)?;

    skip_inline_ws(input)?;

    let val = if input.peek_byte() == Some(b'\n') || input.peek_byte() == Some(b'#') || input.is_eof() {
      skip_ws_and_comments(input)?;
      if input.is_eof() {
        YamlValue::Null
      } else {
        let next_indent = current_indent(input);
        if next_indent > map_indent {
          block_value(input, next_indent, ctx)?
        } else {
          YamlValue::Null
        }
      }
    } else {
      block_value(input, map_indent + 1, ctx)?
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
    return Err(Fail::Backtrack(ParseError::from_expected(
      input.offset(),
      Expected::Description("mapping"),
    )));
  }

  Ok(YamlValue::Mapping(pairs))
}

fn parse_mapping_key<'a>(input: &mut StrInput<'a>) -> PResult<String, ParseError> {
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
        return Err(Fail::Backtrack(ParseError::from_expected(
          input.offset(),
          Expected::Description("mapping key"),
        )));
      }

      let key = remaining[..end].trim_end().to_string();
      input.advance(end);
      Ok(key)
    }
  }
}
