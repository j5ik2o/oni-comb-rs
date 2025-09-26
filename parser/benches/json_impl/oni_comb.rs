#![allow(dead_code)]

use oni_comb_parser::core::{ParseCursor, ParseError, ParseResult, Parser};
use oni_comb_parser::prelude::*;
use serde_json::{Map, Number, Value};
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::str::{self, FromStr};

fn ws<'a>() -> Parser<'a, u8, ()> {
    take_while0(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r')).map(|_| ())
}

fn lexeme<'a, A>(parser: Parser<'a, u8, A>) -> Parser<'a, u8, A>
where
    A: 'a,
{
    Parser::new(move |input, state| {
        let mut cursor = ParseCursor::new(input, state);
        let mut total_length = 0usize;
        let whitespace = ws();

        if let Ok((_, len)) = cursor.consume(&whitespace) {
            total_length += len;
        }

        let value = match cursor.consume(&parser) {
            Ok((value, len)) => {
                total_length += len;
                value
            }
            Err(failure) => return failure.into_result(),
        };

        if let Ok((_, len)) = cursor.consume(&whitespace) {
            total_length += len;
        }

        ParseResult::Success {
            value,
            length: total_length,
            state: Some(cursor.state()),
        }
    })
}

fn json_null<'a>() -> Parser<'a, u8, Value> {
    seq(b"null".to_vec()).map(|_| Value::Null)
}

fn json_bool<'a>() -> Parser<'a, u8, Value> {
    Parser::or_list([
        seq(b"true".to_vec()).map(|_| Value::Bool(true)),
        seq(b"false".to_vec()).map(|_| Value::Bool(false)),
    ])
}

fn json_number<'a>() -> Parser<'a, u8, Value> {
    Parser::new(|_, state| {
        let bytes = state.input();
        if bytes.is_empty() {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(bytes),
                "expected number",
            ));
        }

        let mut idx = 0usize;

        if bytes[idx] == b'-' {
            idx += 1;
            if idx == bytes.len() {
                return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset(),
                    Some(bytes),
                    "invalid number",
                ));
            }
        }

        if bytes[idx] == b'0' {
            idx += 1;
        } else if matches!(bytes[idx], b'1'..=b'9') {
            idx += 1;
            while idx < bytes.len() && matches!(bytes[idx], b'0'..=b'9') {
                idx += 1;
            }
        } else {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(bytes),
                "invalid number",
            ));
        }

        if idx < bytes.len() && bytes[idx] == b'.' {
            let frac_start = idx + 1;
            idx += 1;
            while idx < bytes.len() && matches!(bytes[idx], b'0'..=b'9') {
                idx += 1;
            }
            if idx == frac_start {
                return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset(),
                    Some(bytes),
                    "invalid fraction",
                ));
            }
        }

        if idx < bytes.len() && (bytes[idx] == b'e' || bytes[idx] == b'E') {
            idx += 1;
            if idx < bytes.len() && (bytes[idx] == b'+' || bytes[idx] == b'-') {
                idx += 1;
            }
            let exp_start = idx;
            while idx < bytes.len() && matches!(bytes[idx], b'0'..=b'9') {
                idx += 1;
            }
            if idx == exp_start {
                return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset(),
                    Some(bytes),
                    "invalid exponent",
                ));
            }
        }

        let slice = &bytes[..idx];
        match str::from_utf8(slice)
            .ok()
            .and_then(|s| Number::from_str(s).ok())
        {
            Some(num) => {
                let next_state = state.advance_by(idx);
                ParseResult::successful_with_state(next_state, Value::Number(num), idx)
            }
            None => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(bytes),
                "invalid number",
            )),
        }
    })
}

fn json_string_literal<'a>() -> Parser<'a, u8, String> {
    Parser::new(|_, state| {
        let bytes = state.input();
        if bytes.first() != Some(&b'"') {
            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(bytes),
                "expected string",
            ));
        }

        let mut idx = 1usize;
        let mut last = 1usize;
        let mut result = String::new();

        while idx < bytes.len() {
            let byte = bytes[idx];
            if byte == b'"' {
                if idx > last {
                    if let Ok(segment) = str::from_utf8(&bytes[last..idx]) {
                        result.push_str(segment);
                    } else {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            state.current_offset() + last,
                            Some(&bytes[last..idx]),
                            "invalid utf8 in string",
                        ));
                    }
                }
                let consumed = idx + 1;
                let next_state = state.advance_by(consumed);
                return ParseResult::successful_with_state(next_state, result, consumed);
            } else if byte == b'\\' {
                if idx > last {
                    if let Ok(segment) = str::from_utf8(&bytes[last..idx]) {
                        result.push_str(segment);
                    } else {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            state.current_offset() + last,
                            Some(&bytes[last..idx]),
                            "invalid utf8 in string",
                        ));
                    }
                }

                idx += 1;
                if idx >= bytes.len() {
                    return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                        state.current_offset() + idx,
                        Some(&bytes[idx..]),
                        "unterminated escape",
                    ));
                }

                let escaped = bytes[idx];
                match escaped {
                    b'"' => result.push('"'),
                    b'\\' => result.push('\\'),
                    b'/' => result.push('/'),
                    b'b' => result.push('\u{0008}'),
                    b'f' => result.push('\u{000C}'),
                    b'n' => result.push('\n'),
                    b'r' => result.push('\r'),
                    b't' => result.push('\t'),
                    b'u' => {
                        if idx + 4 >= bytes.len() {
                            return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                                state.current_offset() + idx,
                                Some(&bytes[idx..]),
                                "invalid unicode escape",
                            ));
                        }
                        let hex = &bytes[idx + 1..idx + 5];
                        let hex_str = match str::from_utf8(hex) {
                            Ok(s) => s,
                            Err(_) => {
                                return ParseResult::failed_with_uncommitted(
                                    ParseError::of_custom(
                                        state.current_offset() + idx,
                                        Some(&bytes[idx..idx + 5]),
                                        "invalid unicode escape",
                                    ),
                                );
                            }
                        };
                        match u16::from_str_radix(hex_str, 16) {
                            Ok(first) if (0xD800..=0xDBFF).contains(&first) => {
                                if idx + 6 >= bytes.len()
                                    || bytes[idx + 5] != b'\\'
                                    || bytes[idx + 6] != b'u'
                                {
                                    result.push('\u{FFFD}');
                                    idx += 4;
                                } else {
                                    if idx + 11 >= bytes.len() {
                                        return ParseResult::failed_with_uncommitted(
                                            ParseError::of_custom(
                                                state.current_offset() + idx,
                                                Some(&bytes[idx..]),
                                                "invalid surrogate pair",
                                            ),
                                        );
                                    }
                                    let low_hex = &bytes[idx + 7..idx + 11];
                                    let second = match str::from_utf8(low_hex) {
                                        Ok(s) => u16::from_str_radix(s, 16).unwrap_or(0xFFFD),
                                        Err(_) => 0xFFFD,
                                    };
                                    let decoded = decode_utf16([first, second])
                                        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER));
                                    for ch in decoded {
                                        result.push(ch);
                                    }
                                    idx += 10;
                                }
                            }
                            Ok(first) => {
                                let ch =
                                    char::from_u32(first as u32).unwrap_or(REPLACEMENT_CHARACTER);
                                result.push(ch);
                                idx += 4;
                            }
                            Err(_) => {
                                result.push('\u{FFFD}');
                                idx += 4;
                            }
                        }
                    }
                    _ => {
                        return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                            state.current_offset() + idx,
                            Some(&bytes[idx..idx + 1]),
                            "invalid escape",
                        ));
                    }
                }
                idx += 1;
                last = idx;
            } else if byte < 0x20 {
                return ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset() + idx,
                    Some(&bytes[idx..idx + 1]),
                    "control character in string",
                ));
            } else {
                idx += 1;
            }
        }

        ParseResult::failed_with_uncommitted(ParseError::of_custom(
            state.current_offset(),
            Some(bytes),
            "unterminated string",
        ))
    })
}

fn json_string<'a>() -> Parser<'a, u8, Value> {
    json_string_literal().map(Value::String)
}

fn json_member<'a>() -> Parser<'a, u8, (String, Value)> {
    lexeme(json_string_literal()).flat_map(|key| {
        let key_clone = key.clone();
        skip_left(lexeme(byte(b':')), json_value()).map(move |value| (key_clone.clone(), value))
    })
}

fn json_array<'a>() -> Parser<'a, u8, Value> {
    let elements = separated_list0(json_value(), lexeme(byte(b',')));
    let array = skip_left(lexeme(byte(b'[')), elements);
    let array = skip_right(array, lexeme(byte(b']')));
    array.map(Value::Array)
}

fn json_object<'a>() -> Parser<'a, u8, Value> {
    let members = separated_list0(json_member(), lexeme(byte(b',')));
    let object = skip_left(lexeme(byte(b'{')), members);
    let object = skip_right(object, lexeme(byte(b'}')));
    object.map(|entries: Vec<(String, Value)>| {
        Value::Object(entries.into_iter().collect::<Map<String, Value>>())
    })
}

fn json_value_inner<'a>() -> Parser<'a, u8, Value> {
    Parser::or_list([
        json_null(),
        json_bool(),
        json_number(),
        json_string(),
        json_array(),
        json_object(),
    ])
}

fn json_value<'a>() -> Parser<'a, u8, Value> {
    Parser::new(|input, state| {
        let parser = skip_right(skip_left(ws(), json_value_inner()), ws());
        parser.run(input, state)
    })
}

pub fn parse_json_value(input: &[u8]) -> Result<Value, String> {
    let value = skip_left(ws(), json_value());
    let value = skip_right(value, ws());
    let parser = skip_right(value, end());

    match parser.parse(input) {
        ParseResult::Success { value, .. } => Ok(value),
        ParseResult::Failure { error, .. } => Err(error.message.to_string()),
    }
}
