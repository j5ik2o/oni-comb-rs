use alloc::borrow::Cow;
use alloc::string::String;

use crate::error::ParseError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;
use crate::str_input::StrInput;

/// エスケープなし文字列はゼロコピー (`&'a str`) で返し、
/// エスケープありの場合のみ `String` にフォールバックする `quoted_string` パーサー。
pub struct QuotedStringCow;

pub fn quoted_string_cow() -> QuotedStringCow {
    QuotedStringCow
}

impl<'a> Parser<StrInput<'a>> for QuotedStringCow {
    type Output = Cow<'a, str>;
    type Error = ParseError;

    #[inline]
    fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<Cow<'a, str>, ParseError> {
        let pos = input.offset();
        let remaining = input.as_str();
        let bytes = remaining.as_bytes();

        if bytes.is_empty() || bytes[0] != b'"' {
            return Err(Fail::Backtrack(ParseError::expected_char(pos, '"')));
        }

        // Fast path: scan for closing quote without escape
        let mut i = 1; // skip opening quote
        loop {
            if i >= bytes.len() {
                // No closing quote found
                return Err(Fail::Cut(ParseError::expected_char(pos + i, '"')));
            }
            match bytes[i] {
                b'"' => {
                    // No escape encountered — zero-copy slice
                    let s = &remaining[1..i];
                    input.advance(i + 1);
                    return Ok(Cow::Borrowed(s));
                }
                b'\\' => {
                    // Escape found — fall through to slow path
                    break;
                }
                _ => {
                    i += 1;
                }
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
                            let mut code: u32 = 0;
                            for _ in 0..4 {
                                match chars.next() {
                                    Some(c) if c.is_ascii_hexdigit() => {
                                        consumed += 1;
                                        code = code * 16 + c.to_digit(16).unwrap();
                                    }
                                    _ => {
                                        return Err(Fail::Cut(ParseError::expected_description(
                                            pos + consumed,
                                            "4 hex digits after \\u",
                                        )));
                                    }
                                }
                            }
                            match char::from_u32(code) {
                                Some(c) => result.push(c),
                                None => {
                                    return Err(Fail::Cut(ParseError::expected_description(
                                        pos + consumed - 4,
                                        "valid unicode code point",
                                    )));
                                }
                            }
                        }
                        Some(_) => {
                            return Err(Fail::Cut(ParseError::expected_description(
                                pos + consumed,
                                "valid escape sequence",
                            )));
                        }
                        None => {
                            return Err(Fail::Cut(ParseError::expected_description(
                                pos + consumed,
                                "escape character after '\\'",
                            )));
                        }
                    }
                }
                Some(c) => {
                    consumed += c.len_utf8();
                    result.push(c);
                }
                None => {
                    return Err(Fail::Cut(ParseError::expected_char(pos + consumed, '"')));
                }
            }
        }
    }
}
