//! YAML 固有コンビネータ。
//!
//! `YamlInput` のインデントスタック・アンカーマップ・タグを操作する
//! パーサーコンビネータを提供する。
//!
//! These combinators are currently used in tests and reserved for future
//! pipeline-style rewrite of YAML parsers.

#![allow(dead_code)]

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;

use crate::tag::apply_tag;
use crate::value::YamlValue;
use crate::yaml_input::YamlInput;

// ── with_indent ─────────────────────────────────

/// インデントレベルを設定して内部パーサーを実行し、完了後に復元する。
pub(crate) struct WithIndent<P> {
  indent: usize,
  parser: P,
}

pub(crate) fn with_indent<'a, P>(indent: usize, parser: P) -> WithIndent<P>
where
  P: Parser<YamlInput<'a>, Output = YamlValue, Error = ParseError>,
{
  WithIndent { indent, parser }
}

impl<'a, P> Parser<YamlInput<'a>> for WithIndent<P>
where
  P: Parser<YamlInput<'a>, Output = YamlValue, Error = ParseError>,
{
  type Output = YamlValue;
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
    input.push_indent(self.indent);
    let result = self.parser.parse_next(input);
    input.pop_indent();
    result
  }
}

// ── indent_guard ────────────────────────────────

/// 現在のインデントが最小値以上かを検査する。入力を消費しない。
pub(crate) struct IndentGuard;

pub(crate) fn indent_guard() -> IndentGuard {
  IndentGuard
}

impl<'a> Parser<YamlInput<'a>> for IndentGuard {
  type Output = ();
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlInput<'a>) -> PResult<(), ParseError> {
    let current = input.column() - 1; // 0-based indent
    let min = input.current_min_indent();
    if current >= min {
      Ok(())
    } else {
      Err(Fail::Backtrack(ParseError::from_expected_with_location(
        input.offset(),
        input.line(),
        input.column(),
        Expected::Description("sufficient indentation"),
      )))
    }
  }
}

// ── save_anchor ─────────────────────────────────

/// アンカープレフィックス (`&name`) を検出し、内部パーサーの結果をアンカーマップに保存する。
pub(crate) struct SaveAnchor<P> {
  parser: P,
}

pub(crate) fn save_anchor<'a, P>(parser: P) -> SaveAnchor<P>
where
  P: Parser<YamlInput<'a>, Output = YamlValue, Error = ParseError>,
{
  SaveAnchor { parser }
}

impl<'a, P> Parser<YamlInput<'a>> for SaveAnchor<P>
where
  P: Parser<YamlInput<'a>, Output = YamlValue, Error = ParseError>,
{
  type Output = YamlValue;
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
    // Check for anchor prefix
    let anchor_name = if input.peek_byte() == Some(b'&') {
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
      // Skip inline whitespace after anchor name
      while input.peek_byte() == Some(b' ') || input.peek_byte() == Some(b'\t') {
        input.next_token();
      }
      Some(name)
    } else {
      None
    };

    // Parse the value
    let value = self.parser.parse_next(input)?;

    // Save anchor if present
    if let Some(name) = anchor_name {
      input.set_anchor(name, value.clone());
    }

    Ok(value)
  }
}

// ── resolve_alias ───────────────────────────────

/// エイリアス (`*name`) をアンカーマップから解決する。
pub(crate) struct ResolveAlias;

pub(crate) fn resolve_alias() -> ResolveAlias {
  ResolveAlias
}

impl<'a> Parser<YamlInput<'a>> for ResolveAlias {
  type Output = YamlValue;
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
    if input.peek_byte() != Some(b'*') {
      return Err(Fail::Backtrack(ParseError::from_expected_with_location(
        input.offset(),
        input.line(),
        input.column(),
        Expected::Char('*'),
      )));
    }

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
}

// ── with_tag ────────────────────────────────────

/// タグプレフィックス (`!!tag` / `!tag`) を検出し、内部パーサーの結果に型変換を適用する。
pub(crate) struct WithTag<P> {
  parser: P,
}

pub(crate) fn with_tag<'a, P>(parser: P) -> WithTag<P>
where
  P: Parser<YamlInput<'a>, Output = YamlValue, Error = ParseError>,
{
  WithTag { parser }
}

impl<'a, P> Parser<YamlInput<'a>> for WithTag<P>
where
  P: Parser<YamlInput<'a>, Output = YamlValue, Error = ParseError>,
{
  type Output = YamlValue;
  type Error = ParseError;

  fn parse_next(&mut self, input: &mut YamlInput<'a>) -> PResult<YamlValue, ParseError> {
    let tag_str = if input.peek_byte() == Some(b'!') {
      let mut tag = String::new();
      input.next_token(); // consume first '!'
      tag.push('!');

      if input.peek_byte() == Some(b'!') {
        input.next_token(); // consume second '!'
        tag.push('!');
      }

      while let Some(b) = input.peek_byte() {
        if b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' {
          break;
        }
        tag.push(input.next_token().unwrap());
      }

      // Skip whitespace after tag
      while input.peek_byte() == Some(b' ') || input.peek_byte() == Some(b'\t') {
        input.next_token();
      }

      Some(tag)
    } else {
      None
    };

    let value = self.parser.parse_next(input)?;

    match tag_str {
      Some(tag) => Ok(apply_tag(&tag, value)),
      None => Ok(value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use oni_comb_parser::input::Input;
  use oni_comb_parser::parser::Parser;
  use oni_comb_parser::prelude::*;

  // Simple scalar parser for testing
  fn test_scalar<'a>() -> impl Parser<YamlInput<'a>, Output = YamlValue, Error = ParseError> {
    fn_parser(|input: &mut YamlInput| {
      let remaining = input.remaining();
      let end = remaining
        .find(['\n', ',', ']', '}'])
        .unwrap_or(remaining.len());
      let s = remaining[..end].trim().to_string();
      input.advance(end);
      Ok(YamlValue::String(s))
    })
  }

  #[test]
  fn with_indent_sets_and_restores() {
    let mut input = YamlInput::new("hello");
    assert_eq!(input.current_min_indent(), 0);
    let mut p = with_indent(4, test_scalar());
    let _ = p.parse_next(&mut input);
    assert_eq!(input.current_min_indent(), 0); // restored
  }

  #[test]
  fn indent_guard_passes_when_sufficient() {
    let mut input = YamlInput::new("    hello");
    // consume 4 spaces to move column to 5
    for _ in 0..4 {
      input.next_token();
    }
    input.push_indent(2);
    assert!(indent_guard().parse_next(&mut input).is_ok());
    input.pop_indent();
  }

  #[test]
  fn indent_guard_fails_when_insufficient() {
    let mut input = YamlInput::new("x");
    input.push_indent(4);
    assert!(indent_guard().parse_next(&mut input).is_err());
    input.pop_indent();
  }

  #[test]
  fn save_anchor_stores_value() {
    let mut input = YamlInput::new("&myref hello");
    let result = save_anchor(test_scalar()).parse_next(&mut input).unwrap();
    assert_eq!(result, YamlValue::String("hello".to_string()));
    assert_eq!(
      input.get_anchor("myref"),
      Some(&YamlValue::String("hello".to_string()))
    );
  }

  #[test]
  fn save_anchor_without_prefix() {
    let mut input = YamlInput::new("hello");
    let result = save_anchor(test_scalar()).parse_next(&mut input).unwrap();
    assert_eq!(result, YamlValue::String("hello".to_string()));
    assert_eq!(input.get_anchor("anything"), None);
  }

  #[test]
  fn resolve_alias_resolves() {
    let mut input = YamlInput::new("*myref");
    input.set_anchor("myref".to_string(), YamlValue::Integer(42));
    let result = resolve_alias().parse_next(&mut input).unwrap();
    assert_eq!(result, YamlValue::Integer(42));
  }

  #[test]
  fn resolve_alias_fails_unknown() {
    let mut input = YamlInput::new("*unknown");
    assert!(resolve_alias().parse_next(&mut input).is_err());
  }

  #[test]
  fn with_tag_applies_core_schema() {
    let mut input = YamlInput::new("!!str 42");
    let result = with_tag(test_scalar()).parse_next(&mut input).unwrap();
    assert_eq!(result, YamlValue::String("42".to_string()));
  }

  #[test]
  fn with_tag_without_tag() {
    let mut input = YamlInput::new("hello");
    let result = with_tag(test_scalar()).parse_next(&mut input).unwrap();
    assert_eq!(result, YamlValue::String("hello".to_string()));
  }
}
