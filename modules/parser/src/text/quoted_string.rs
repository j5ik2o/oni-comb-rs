use alloc::borrow::Cow;
use alloc::string::String;

use crate::error::{ExpectError, Expected, ParseError};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

/// 4桁の16進数を読んで u32 を返すヘルパー。
fn parse_hex4(chars: &mut core::str::Chars<'_>, consumed: &mut usize, pos: usize, line: usize, column: usize) -> PResult<u32, ParseError> {
  let mut code: u32 = 0;
  for _ in 0..4 {
    match chars.next() {
      Some(c) if c.is_ascii_hexdigit() => {
        *consumed += 1;
        code = code * 16 + c.to_digit(16).unwrap();
      }
      _ => {
        return Err(Fail::Cut(ParseError::from_expected_with_location(
          pos + *consumed,
          line,
          column,
          Expected::Description("4 hex digits after \\u"),
        )));
      }
    }
  }
  Ok(code)
}

/// エスケープなし文字列はゼロコピー (`&'a str`) で返し、
/// エスケープありの場合のみ `String` にフォールバックする quoted string パーサー。
pub struct QuotedString;

pub fn quoted_string() -> QuotedString {
  QuotedString
}

impl<'a> Parser<StrInput<'a>> for QuotedString {
  type Error = ParseError;
  type Output = Cow<'a, str>;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<Cow<'a, str>, ParseError> {
    let pos = input.offset();
    let remaining = input.as_str();
    let bytes = remaining.as_bytes();

    if bytes.is_empty() || bytes[0] != b'"' {
      return Err(Fail::Backtrack(ParseError::from_expected_with_location(pos, input.line(), input.column(), Expected::Char('"'))));
    }

    // Fast path: scan for closing quote without escape
    let mut i = 1; // skip opening quote
    loop {
      if i >= bytes.len() {
        return Err(Fail::Cut(ParseError::from_expected_with_location(pos + i, input.line(), input.column(), Expected::Char('"'))));
      }
      match bytes[i] {
        b'"' => {
          let s = &remaining[1..i];
          input.advance(i + 1);
          return Ok(Cow::Borrowed(s));
        }
        b'\\' => break,
        _ => i += 1,
      }
    }

    // Slow path: build String, reusing the prefix before the first escape
    let mut result = String::with_capacity(i + 16);
    result.push_str(&remaining[1..i]);

    let mut chars = remaining[i..].chars();
    let mut consumed = i; // bytes consumed so far (including opening quote)

    loop {
      match chars.next() {
        Some('"') => {
          consumed += 1;
          input.advance(consumed);
          return Ok(Cow::Owned(result));
        }
        Some('\\') => {
          consumed += 1;
          match chars.next() {
            Some('"') => {
              consumed += 1;
              result.push('"');
            }
            Some('\\') => {
              consumed += 1;
              result.push('\\');
            }
            Some('/') => {
              consumed += 1;
              result.push('/');
            }
            Some('b') => {
              consumed += 1;
              result.push('\u{0008}');
            }
            Some('f') => {
              consumed += 1;
              result.push('\u{000C}');
            }
            Some('n') => {
              consumed += 1;
              result.push('\n');
            }
            Some('r') => {
              consumed += 1;
              result.push('\r');
            }
            Some('t') => {
              consumed += 1;
              result.push('\t');
            }
            Some('u') => {
              consumed += 1;
              let code = parse_hex4(&mut chars, &mut consumed, pos, input.line(), input.column())?;

              // サロゲートペア処理
              if (0xD800..=0xDBFF).contains(&code) {
                // 高サロゲート: 次の \uXXXX を読んで低サロゲートと合成。
                // c1='\\', c2='u' は JSON 仕様上必ず ASCII (1 byte)。
                let c1 = chars.next();
                if c1.is_some() {
                  consumed += 1;
                }
                let c2 = chars.next();
                if c2.is_some() {
                  consumed += 1;
                }
                match (c1, c2) {
                  (Some('\\'), Some('u')) => {
                    let low = parse_hex4(&mut chars, &mut consumed, pos, input.line(), input.column())?;
                    if !(0xDC00..=0xDFFF).contains(&low) {
                      return Err(Fail::Cut(ParseError::from_expected_with_location(
                        pos + consumed - 4,
                        input.line(),
                        input.column(),
                        Expected::Description("low surrogate (\\uDC00-\\uDFFF)"),
                      )));
                    }
                    let cp = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
                    match char::from_u32(cp) {
                      Some(c) => result.push(c),
                      None => {
                        return Err(Fail::Cut(ParseError::from_expected_with_location(
                          pos + consumed - 10,
                          input.line(),
                          input.column(),
                          Expected::Description("valid surrogate pair"),
                        )));
                      }
                    }
                  }
                  _ => {
                    return Err(Fail::Cut(ParseError::from_expected_with_location(
                      pos + consumed,
                      input.line(),
                      input.column(),
                      Expected::Description("low surrogate pair (\\uXXXX)"),
                    )));
                  }
                }
              } else if (0xDC00..=0xDFFF).contains(&code) {
                // 孤立した低サロゲートはエラー
                return Err(Fail::Cut(ParseError::from_expected_with_location(
                  pos + consumed - 4,
                  input.line(),
                  input.column(),
                  Expected::Description("high surrogate before low surrogate"),
                )));
              } else {
                match char::from_u32(code) {
                  Some(c) => result.push(c),
                  None => {
                    return Err(Fail::Cut(ParseError::from_expected_with_location(
                      pos + consumed - 4,
                      input.line(),
                      input.column(),
                      Expected::Description("valid unicode code point"),
                    )));
                  }
                }
              }
            }
            Some(_) => {
              return Err(Fail::Cut(ParseError::from_expected_with_location(
                pos + consumed,
                input.line(),
                input.column(),
                Expected::Description("valid escape sequence"),
              )));
            }
            None => {
              return Err(Fail::Cut(ParseError::from_expected_with_location(
                pos + consumed,
                input.line(),
                input.column(),
                Expected::Description("escape character after '\\'"),
              )));
            }
          }
        }
        Some(c) => {
          consumed += c.len_utf8();
          result.push(c);
        }
        None => {
          return Err(Fail::Cut(ParseError::from_expected_with_location(
            pos + consumed,
            input.line(),
            input.column(),
            Expected::Char('"'),
          )));
        }
      }
    }
  }
}
