use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::prelude::*;

use crate::value::YamlValue;

/// YAML Core Schema のスカラー値をパースする。
/// null, bool, int (10進/8進/16進), float (小数/.inf/.nan), 文字列。
pub(crate) fn yaml_scalar<'a>(input: &mut StrInput<'a>) -> PResult<YamlValue, ParseError> {
  // Quoted strings: preserve as-is
  match input.peek_byte() {
    Some(b'"') => {
      let s = oni_comb_parser::prelude::quoted_string().parse_next(input)?;
      return Ok(YamlValue::String(s.into_owned()));
    }
    Some(b'\'') => {
      return parse_single_quoted(input);
    }
    _ => {}
  }

  // Plain scalar: read until flow indicator, colon+space, or newline
  let start = input.offset();
  let remaining = input.remaining();

  // Try to identify the scalar type by content
  // First, collect the plain scalar text
  let plain = collect_plain_scalar(remaining);
  if plain.is_empty() {
    return Err(Fail::Backtrack(ParseError::from_expected(
      start,
      Expected::Description("YAML scalar"),
    )));
  }

  // NOTE: plain is trim_end()'d, so trailing whitespace between the scalar
  // and the next delimiter remains unconsumed. Callers (block_value, flow_value)
  // are expected to call skip_inline_ws / skip_ws_and_comments afterward.
  input.advance(plain.len());

  // Resolve type per Core Schema
  Ok(resolve_core_scalar(plain))
}

/// Collect a plain scalar from remaining input.
/// Stops at: newline, '#' preceded by space, ':', ',', '[', ']', '{', '}'
/// Returns a trim_end()'d slice — trailing whitespace is intentionally excluded
/// from the scalar value and left unconsumed for the caller to skip.
fn collect_plain_scalar(remaining: &str) -> &str {
  let bytes = remaining.as_bytes();
  let mut end = 0;
  let mut last_non_ws = 0;

  while end < bytes.len() {
    let b = bytes[end];
    match b {
      b'\n' | b'\r' => break,
      b'#' if end > 0 && bytes[end - 1] == b' ' => {
        // Comment start; trim trailing whitespace
        end = last_non_ws;
        break;
      }
      b':' if end + 1 < bytes.len() && (bytes[end + 1] == b' ' || bytes[end + 1] == b'\n') => break,
      b':' if end + 1 >= bytes.len() => break,
      b',' | b'[' | b']' | b'{' | b'}' => break,
      _ => {
        end += 1;
        if b != b' ' && b != b'\t' {
          last_non_ws = end;
        }
      }
    }
  }

  let result = &remaining[..end];
  // Trim trailing whitespace
  result.trim_end()
}

/// Resolve a plain scalar string to a YAML Core Schema type.
fn resolve_core_scalar(s: &str) -> YamlValue {
  match s {
    // null
    "null" | "Null" | "NULL" | "~" => YamlValue::Null,

    // bool
    "true" | "True" | "TRUE" => YamlValue::Bool(true),
    "false" | "False" | "FALSE" => YamlValue::Bool(false),

    // special float
    ".inf" | ".Inf" | ".INF" => YamlValue::Float(f64::INFINITY),
    "-.inf" | "-.Inf" | "-.INF" => YamlValue::Float(f64::NEG_INFINITY),
    ".nan" | ".NaN" | ".NAN" => YamlValue::Float(f64::NAN),

    _ => {
      // Try integer (decimal)
      if let Ok(v) = s.parse::<i64>() {
        return YamlValue::Integer(v);
      }

      // Try integer (hex)
      if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        if let Ok(v) = i64::from_str_radix(hex, 16) {
          return YamlValue::Integer(v);
        }
      }

      // Try integer (octal)
      if let Some(oct) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
        if let Ok(v) = i64::from_str_radix(oct, 8) {
          return YamlValue::Integer(v);
        }
      }

      // Try float
      if let Ok(v) = s.parse::<f64>() {
        return YamlValue::Float(v);
      }

      // Plain string
      YamlValue::String(s.to_string())
    }
  }
}

/// Parse a single-quoted string (YAML style: '' for literal ')
fn parse_single_quoted<'a>(input: &mut StrInput<'a>) -> PResult<YamlValue, ParseError> {
  let pos = input.offset();
  let remaining = input.remaining();

  if !remaining.starts_with('\'') {
    return Err(Fail::Backtrack(ParseError::from_expected(pos, Expected::Char('\''))));
  }

  let mut result = String::new();
  let mut chars = remaining[1..].chars(); // skip opening quote
  let mut consumed = 1; // bytes consumed (opening quote)

  loop {
    match chars.next() {
      None => {
        return Err(Fail::Cut(ParseError::from_expected(
          pos + consumed,
          Expected::Char('\''),
        )));
      }
      Some('\'') => {
        consumed += 1;
        // Check for escaped single quote ('')
        if chars.as_str().starts_with('\'') {
          chars.next();
          consumed += 1;
          result.push('\'');
        } else {
          // End of string
          break;
        }
      }
      Some(c) => {
        consumed += c.len_utf8();
        result.push(c);
      }
    }
  }

  input.advance(consumed);
  Ok(YamlValue::String(result))
}
