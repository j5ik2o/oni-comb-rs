use oni_comb_parser::core::{ParseError, ParseResult, Parser};
use oni_comb_parser::prelude::*;
use serde_json::{Map, Number, Value};
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::str::{self, FromStr};

fn ws<'a>() -> Parser<'a, u8, ()> {
    take_while0(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r')).map(|_| ())
}

fn lexeme<'a, A>(parser: Parser<'a, u8, A>) -> Parser<'a, u8, A>
where
    A: Clone + 'a,
{
    let with_leading = skip_left(ws(), parser);
    skip_right(with_leading, ws())
}

fn decode_units(units: Vec<u16>) -> String {
    decode_utf16(units.into_iter())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}

fn parse_hex_unit<'a>() -> Parser<'a, u8, u16> {
    take(4).map(|digits| {
        str::from_utf8(digits)
            .ok()
            .and_then(|s| u16::from_str_radix(s, 16).ok())
            .unwrap_or(0xFFFD)
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

fn digits1_vec<'a>() -> Parser<'a, u8, Vec<u8>> {
    take_while1(|b| matches!(b, b'0'..=b'9')).map(|digits| digits.to_vec())
}

fn json_number<'a>() -> Parser<'a, u8, Value> {
    let sign = byte(b'-')
        .map(|b| vec![b])
        .optional()
        .map(|opt| opt.unwrap_or_default());

    let integer = digits1_vec().filter(|digits| digits.len() == 1 || digits.first() != Some(&b'0'));

    let fractional = byte(b'.')
        .flat_map(|dot| {
            digits1_vec().map(move |digits| {
                let mut result = Vec::with_capacity(1 + digits.len());
                result.push(dot);
                result.extend_from_slice(&digits);
                result
            })
        })
        .optional()
        .map(|opt| opt.unwrap_or_default());

    let exponent_tail = one_of(b"+-".to_vec()).optional().flat_map(|sign_opt| {
        digits1_vec().map(move |digits| {
            let mut result = Vec::new();
            if let Some(sign) = sign_opt {
                result.push(sign);
            }
            result.extend_from_slice(&digits);
            result
        })
    });

    let exponent = one_of(b"eE".to_vec())
        .flat_map(move |marker| {
            exponent_tail.clone().map(move |mut tail| {
                let mut result = Vec::with_capacity(1 + tail.len());
                result.push(marker);
                result.append(&mut tail);
                result
            })
        })
        .optional()
        .map(|opt| opt.unwrap_or_default());

    sign.flat_map(move |sign_part| {
        integer.clone().flat_map(move |int_part| {
            let mut bytes = Vec::with_capacity(sign_part.len() + int_part.len());
            bytes.extend_from_slice(&sign_part);
            bytes.extend_from_slice(&int_part);
            successful(bytes)
        })
    })
    .flat_map({
        let fractional = fractional.clone();
        move |base_bytes| {
            fractional.clone().flat_map(move |frac_part| {
                let mut bytes = base_bytes.clone();
                bytes.extend_from_slice(&frac_part);
                successful(bytes)
            })
        }
    })
    .flat_map({
        let exponent = exponent.clone();
        move |base_bytes| {
            exponent.clone().flat_map(move |exp_part| {
                let mut bytes = base_bytes.clone();
                bytes.extend_from_slice(&exp_part);
                successful(bytes)
            })
        }
    })
    .flat_map(|number_bytes| {
        let number_bytes = number_bytes.clone();
        Parser::new(move |_, state| {
            match str::from_utf8(&number_bytes)
                .ok()
                .and_then(|s| Number::from_str(s).ok())
            {
                Some(num) => ParseResult::successful_with_state(state, Value::Number(num), 0),
                None => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                    state.current_offset(),
                    Some(state.input()),
                    "invalid number",
                )),
            }
        })
    })
}

fn json_string_literal<'a>() -> Parser<'a, u8, String> {
    let regular_chunk = take_while1(|b| b >= 0x20 && b != b'"' && b != b'\\')
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned());

    let simple_escape = Parser::or_list([
        byte(b'"').map(|_| "\"".to_string()),
        byte(b'\\').map(|_| "\\".to_string()),
        byte(b'/').map(|_| "/".to_string()),
        byte(b'b').map(|_| "\u{0008}".to_string()),
        byte(b'f').map(|_| "\u{000C}".to_string()),
        byte(b'n').map(|_| "\n".to_string()),
        byte(b'r').map(|_| "\r".to_string()),
        byte(b't').map(|_| "\t".to_string()),
    ]);

    let unicode_escape = byte(b'u').flat_map(|_| {
        parse_hex_unit().flat_map(|first| {
            if (0xD800..=0xDBFF).contains(&first) {
                skip_left(byte(b'\\'), byte(b'u')).flat_map(move |_| {
                    parse_hex_unit().map(move |second| decode_units(vec![first, second]))
                })
            } else {
                successful(decode_units(vec![first]))
            }
        })
    });

    let escape_sequence = Parser::or_list([simple_escape, unicode_escape]);
    let escape_chunk = byte(b'\\').flat_map(move |_| escape_sequence.clone());

    let piece = Parser::or_list([regular_chunk.clone(), escape_chunk]);

    let content = piece.many0().map(|segments| segments.concat());

    surround(byte(b'"'), content, byte(b'"'))
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
