use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::PResult;
use oni_comb_parser::input::Input;

use crate::block::block_value;
use crate::common::{at_document_marker, skip_ws_and_comments};
use crate::value::YamlValue;
use crate::yaml_input::YamlInput;

pub(crate) fn yaml_document<'a>(input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
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
  block_value(input)
}

pub(crate) fn yaml_documents<'a>(input: &mut YamlInput<'a>) -> PResult<Vec<YamlValue>, ParseError> {
  let mut docs = Vec::new();

  loop {
    skip_ws_and_comments(input)?;
    if input.is_eof() {
      break;
    }

    if at_document_marker(input) == Some("...") {
      input.advance(3);
      skip_ws_and_comments(input)?;
      continue;
    }

    let before = input.offset();
    let doc = yaml_document(input)?;
    docs.push(doc);

    skip_ws_and_comments(input)?;

    // Guard against infinite loop if no progress was made
    if input.offset() == before {
      break;
    }
  }

  Ok(docs)
}
