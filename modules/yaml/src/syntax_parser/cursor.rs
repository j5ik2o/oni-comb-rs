use oni_comb_parser::error::{ContextError, ExpectError, Expected, ParseError};

use super::parser::SyntaxParser;

impl SyntaxParser<'_> {
  pub(super) fn skip_trivia(&mut self) {
    loop {
      self.skip_whitespace();
      if self.peek_char() == Some('#') {
        self.skip_comment();
        continue;
      }
      break;
    }
  }

  fn skip_whitespace(&mut self) {
    while matches!(self.peek_char(), Some(ch) if ch.is_whitespace()) {
      self.advance_char();
    }
  }

  fn skip_comment(&mut self) {
    while let Some(ch) = self.peek_char() {
      self.advance_char();
      if ch == '\n' {
        break;
      }
    }
  }

  pub(super) fn consume_document_marker(&mut self, marker: &str) -> bool {
    if !self.src[self.pos..].starts_with(marker) {
      return false;
    }

    let next = self.src[self.pos + marker.len()..].chars().next();
    if !match next {
      None => true,
      Some(ch) => ch.is_whitespace() || ch == '#',
    } {
      return false;
    }

    self.pos += marker.len();
    true
  }

  pub(super) fn expect_char(&mut self, expected: char) -> Result<(), ParseError> {
    match self.peek_char() {
      Some(ch) if ch == expected => {
        self.advance_char();
        Ok(())
      }
      _ => Err(self.error(Expected::Char(expected))),
    }
  }

  pub(super) fn consume_char(&mut self, expected: char) -> bool {
    match self.peek_char() {
      Some(ch) if ch == expected => {
        self.advance_char();
        true
      }
      _ => false,
    }
  }

  pub(super) fn next_char_is_whitespace(&self) -> bool {
    let mut chars = self.src[self.pos..].chars();
    let _ = chars.next();
    matches!(chars.next(), Some(ch) if ch.is_whitespace())
  }

  pub(super) fn next_char_is_whitespace_or_delimiter(&self) -> bool {
    let mut chars = self.src[self.pos..].chars();
    let _ = chars.next();
    match chars.next() {
      None => true,
      Some(ch) => ch.is_whitespace() || matches!(ch, ',' | ']' | '}' | '#'),
    }
  }

  pub(super) fn advance_char(&mut self) {
    if let Some(ch) = self.peek_char() {
      self.pos += ch.len_utf8();
    }
  }

  pub(super) fn peek_char(&self) -> Option<char> {
    self.src[self.pos..].chars().next()
  }

  pub(super) fn is_eof(&self) -> bool {
    self.pos >= self.src.len()
  }

  pub(super) fn error(&self, expected: Expected) -> ParseError {
    ParseError::from_expected(self.pos, expected).fill_location_from_src(self.src)
  }

  pub(super) fn unsupported(&self, feature: &'static str) -> ParseError {
    self
      .error(Expected::Description(feature))
      .add_context("unsupported in YAML Phase 1")
  }
}
