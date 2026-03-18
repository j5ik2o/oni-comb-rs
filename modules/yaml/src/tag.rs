use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::prelude::*;

use crate::value::YamlValue;

/// Parse a tag prefix (!! or !) and return the tag string.
#[allow(dead_code)]
pub(crate) fn parse_tag<'a>(input: &mut StrInput<'a>) -> PResult<String, ParseError> {
  let pos = input.offset();

  if input.peek_byte() != Some(b'!') {
    return Err(Fail::Backtrack(ParseError::from_expected(pos, Expected::Char('!'))));
  }

  input.next_token(); // consume first '!'

  if input.peek_byte() == Some(b'!') {
    // !! tag (core schema shorthand)
    input.next_token();
    let mut tag = String::from("!!");
    while let Some(b) = input.peek_byte() {
      if b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' {
        break;
      }
      tag.push(input.next_token().unwrap());
    }
    Ok(tag)
  } else {
    // ! tag (local/custom)
    let mut tag = String::from("!");
    while let Some(b) = input.peek_byte() {
      if b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' {
        break;
      }
      tag.push(input.next_token().unwrap());
    }
    Ok(tag)
  }
}

/// Apply a Core Schema tag to force type conversion.
pub fn apply_tag(tag: &str, value: YamlValue) -> YamlValue {
  match tag {
    "!!str" => match value {
      YamlValue::String(s) => YamlValue::String(s),
      YamlValue::Integer(n) => YamlValue::String(n.to_string()),
      YamlValue::Float(f) => YamlValue::String(f.to_string()),
      YamlValue::Bool(b) => YamlValue::String(b.to_string()),
      YamlValue::Null => YamlValue::String("null".to_string()),
      other => other,
    },
    "!!int" => match value {
      YamlValue::String(s) => {
        if let Ok(n) = s.parse::<i64>() {
          YamlValue::Integer(n)
        } else {
          YamlValue::String(s)
        }
      }
      other => other,
    },
    "!!float" => match value {
      YamlValue::String(s) => {
        if let Ok(f) = s.parse::<f64>() {
          YamlValue::Float(f)
        } else {
          YamlValue::String(s)
        }
      }
      other => other,
    },
    "!!bool" => match value {
      YamlValue::String(s) => match s.as_str() {
        "true" | "True" | "TRUE" => YamlValue::Bool(true),
        "false" | "False" | "FALSE" => YamlValue::Bool(false),
        _ => YamlValue::String(s),
      },
      other => other,
    },
    "!!null" => YamlValue::Null,
    _ => YamlValue::Tagged {
      tag: tag.to_string(),
      value: Box::new(value),
    },
  }
}
