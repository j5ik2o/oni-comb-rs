use crate::fail::{Fail, PResult};
use crate::parser::Parser;
use crate::str_input::StrInput;

pub struct QuotedString;

pub fn quoted_string() -> QuotedString {
    QuotedString
}

impl<'a> Parser<StrInput<'a>> for QuotedString {
    type Output = String;
    type Error = String;

    fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<String, String> {
        let remaining = input.as_str();
        let mut chars = remaining.chars();

        // opening quote
        match chars.next() {
            Some('"') => {}
            _ => {
                return Err(Fail::Backtrack(
                    "quoted_string: expected '\"'".to_string(),
                ));
            }
        }

        let mut result = String::new();
        let mut consumed = 1; // opening quote

        loop {
            match chars.next() {
                Some('"') => {
                    consumed += 1;
                    input.advance(consumed);
                    return Ok(result);
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
                            let mut hex = String::with_capacity(4);
                            for _ in 0..4 {
                                match chars.next() {
                                    Some(c) if c.is_ascii_hexdigit() => {
                                        consumed += c.len_utf8();
                                        hex.push(c);
                                    }
                                    _ => {
                                        return Err(Fail::Cut(
                                            "quoted_string: expected 4 hex digits after \\u"
                                                .to_string(),
                                        ));
                                    }
                                }
                            }
                            let code_point = u32::from_str_radix(&hex, 16).unwrap();
                            match char::from_u32(code_point) {
                                Some(c) => result.push(c),
                                None => {
                                    return Err(Fail::Cut(format!(
                                        "quoted_string: invalid unicode code point: U+{}",
                                        hex
                                    )));
                                }
                            }
                        }
                        Some(c) => {
                            return Err(Fail::Cut(format!(
                                "quoted_string: invalid escape sequence '\\{}'",
                                c
                            )));
                        }
                        None => {
                            return Err(Fail::Cut(
                                "quoted_string: unexpected EOF after '\\'".to_string(),
                            ));
                        }
                    }
                }
                Some(c) => {
                    consumed += c.len_utf8();
                    result.push(c);
                }
                None => {
                    return Err(Fail::Cut(
                        "quoted_string: unterminated string".to_string(),
                    ));
                }
            }
        }
    }
}
