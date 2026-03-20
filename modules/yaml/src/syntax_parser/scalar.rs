use oni_comb_parser::error::{Expected, ParseError};

use crate::{YamlSyntaxNode, YamlSyntaxScalar};

use super::parser::SyntaxParser;

impl SyntaxParser<'_> {
  pub(super) fn parse_single_quoted_scalar(&mut self) -> Result<YamlSyntaxNode, ParseError> {
    self.expect_char('\'')?;
    let mut value = String::new();

    loop {
      match self.peek_char() {
        Some('\'') => {
          self.advance_char();
          if self.peek_char() == Some('\'') {
            self.advance_char();
            value.push('\'');
          } else {
            break;
          }
        }
        Some(ch) => {
          self.advance_char();
          value.push(ch);
        }
        None => return Err(self.error(Expected::Description("closing single quote"))),
      }
    }

    Ok(YamlSyntaxNode::Scalar(YamlSyntaxScalar::SingleQuoted(value)))
  }

  pub(super) fn parse_double_quoted_scalar(&mut self) -> Result<YamlSyntaxNode, ParseError> {
    self.expect_char('"')?;
    let mut value = String::new();

    loop {
      match self.peek_char() {
        Some('"') => {
          self.advance_char();
          break;
        }
        Some('\\') => {
          self.advance_char();
          let escaped = match self.peek_char() {
            Some('"') => '"',
            Some('\\') => '\\',
            Some('n') => '\n',
            Some('r') => '\r',
            Some('t') => '\t',
            Some(other) => other,
            None => return Err(self.error(Expected::Description("escaped character"))),
          };
          self.advance_char();
          value.push(escaped);
        }
        Some(ch) => {
          self.advance_char();
          value.push(ch);
        }
        None => return Err(self.error(Expected::Description("closing double quote"))),
      }
    }

    Ok(YamlSyntaxNode::Scalar(YamlSyntaxScalar::DoubleQuoted(value)))
  }

  pub(super) fn parse_plain_scalar(&mut self) -> Result<YamlSyntaxNode, ParseError> {
    let start = self.pos;
    let mut last_non_whitespace_end = self.pos;
    let mut previous = None;

    while let Some(ch) = self.peek_char() {
      if self.is_plain_scalar_terminator(ch, previous) {
        break;
      }

      self.advance_char();
      if !ch.is_whitespace() {
        last_non_whitespace_end = self.pos;
      }
      previous = Some(ch);
    }

    let value = self.src[start..last_non_whitespace_end].trim_end();
    if value.is_empty() {
      return Err(self.error(Expected::Description("plain scalar")));
    }

    Ok(YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain(value.to_string())))
  }

  fn is_plain_scalar_terminator(&self, ch: char, previous: Option<char>) -> bool {
    match ch {
      '[' | ']' | '{' | '}' | ',' | '\n' | '\r' => true,
      ':' => self.next_char_is_whitespace_or_delimiter(),
      '#' => previous.is_none_or(char::is_whitespace),
      _ => false,
    }
  }
}
