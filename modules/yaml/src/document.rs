use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::PResult;
use oni_comb_parser::input::Input;
use oni_comb_parser::prelude::*;

use crate::block::block_value;
use crate::common::skip_ws_and_comments;
use crate::context::ParseContext;
use crate::value::YamlValue;

fn at_document_marker<'a>(input: &StrInput<'a>) -> Option<&'static str> {
  let remaining = input.remaining();
  if remaining.starts_with("---")
    && (remaining.len() == 3
      || remaining.as_bytes()[3] == b'\n'
      || remaining.as_bytes()[3] == b' '
      || remaining.as_bytes()[3] == b'\r')
  {
    Some("---")
  } else if remaining.starts_with("...")
    && (remaining.len() == 3
      || remaining.as_bytes()[3] == b'\n'
      || remaining.as_bytes()[3] == b' '
      || remaining.as_bytes()[3] == b'\r')
  {
    Some("...")
  } else {
    None
  }
}

pub(crate) fn yaml_document<'a>(input: &mut StrInput<'a>, ctx: &mut ParseContext) -> PResult<YamlValue, ParseError> {
  skip_ws_and_comments(input)?;

  if at_document_marker(input) == Some("---") {
    input.advance(3);
    while input.peek_byte().is_some() && input.peek_byte() != Some(b'\n') {
      input.next_token();
    }
    if input.peek_byte() == Some(b'\n') {
      input.next_token();
    }
  }

  skip_ws_and_comments(input)?;

  if input.is_eof() {
    return Ok(YamlValue::Null);
  }

  if at_document_marker(input) == Some("...") {
    input.advance(3);
    return Ok(YamlValue::Null);
  }

  if at_document_marker(input).is_some() {
    return Ok(YamlValue::Null);
  }
  block_value(input, 0, ctx)
}

pub(crate) fn yaml_documents<'a>(
  input: &mut StrInput<'a>,
  ctx: &mut ParseContext,
) -> PResult<Vec<YamlValue>, ParseError> {
  let mut docs = Vec::new();

  loop {
    skip_ws_and_comments(input)?;
    if input.is_eof() {
      break;
    }

    if input.remaining().starts_with("...") {
      input.advance(3);
      skip_ws_and_comments(input)?;
      continue;
    }

    let doc = yaml_document(input, ctx)?;
    docs.push(doc);

    skip_ws_and_comments(input)?;
  }

  Ok(docs)
}
