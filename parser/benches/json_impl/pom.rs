#![allow(dead_code)]

use pom::parser::*;
use serde_json::{Map, Number, Value};
use std::char::REPLACEMENT_CHARACTER;
use std::str::{self, FromStr};

fn spaces<'a>() -> Parser<'a, char, ()> {
    is_a::<char, _>(|c: char| matches!(c, ' ' | '\t' | '\n' | '\r'))
        .repeat(0..)
        .discard()
}

fn lexeme<'a, O>(parser: Parser<'a, char, O>) -> Parser<'a, char, O>
where
    O: 'a,
{
    spaces() * parser - spaces()
}

fn json_null<'a>() -> Parser<'a, char, Value> {
    tag("null").map(|_| Value::Null)
}

fn json_bool<'a>() -> Parser<'a, char, Value> {
    tag("true").map(|_| Value::Bool(true)) | tag("false").map(|_| Value::Bool(false))
}

fn json_number<'a>() -> Parser<'a, char, Value> {
    let digit = || is_a::<char, _>(|c: char| c.is_ascii_digit());
    let non_zero_digit = || is_a::<char, _>(|c: char| matches!(c, '1'..='9'));

    let integer = tag("0").map(|_| ()) | (non_zero_digit() + digit().repeat(0..)).map(|_| ());

    let fraction = (sym('.') + digit().repeat(1..)).map(|_| ()).opt();

    let exponent = ((sym('e') | sym('E')) + (sym('+') | sym('-')).opt() + digit().repeat(1..))
        .map(|_| ())
        .opt();

    (sym('-').opt() + integer + fraction + exponent)
        .collect()
        .convert(|chars: &[char]| {
            let text: String = chars.iter().copied().collect();
            Number::from_str(&text).map(Value::Number)
        })
}

fn hex_digits<'a>() -> Parser<'a, char, Vec<char>> {
    is_a::<char, _>(|c: char| c.is_ascii_hexdigit()).repeat(4)
}

fn unicode_escape<'a>() -> Parser<'a, char, String> {
    sym('u')
        * hex_digits().map(|digits| {
            let hex: String = digits.into_iter().collect();
            u16::from_str_radix(&hex, 16)
                .ok()
                .and_then(|code| char::from_u32(code as u32))
                .unwrap_or(REPLACEMENT_CHARACTER)
                .to_string()
        })
}

fn escape_sequence<'a>() -> Parser<'a, char, String> {
    sym('\\')
        * (sym('"').map(|_| "\"".to_string())
            | sym('\\').map(|_| "\\".to_string())
            | sym('/').map(|_| "/".to_string())
            | sym('b').map(|_| "\u{0008}".to_string())
            | sym('f').map(|_| "\u{000C}".to_string())
            | sym('n').map(|_| "\n".to_string())
            | sym('r').map(|_| "\r".to_string())
            | sym('t').map(|_| "\t".to_string())
            | unicode_escape())
}

fn string_literal<'a>() -> Parser<'a, char, String> {
    let regular = is_a::<char, _>(|c: char| c >= ' ' && c != '"' && c != '\\')
        .repeat(1..)
        .map(|chars| chars.into_iter().collect::<String>());

    let segment = (regular | escape_sequence())
        .repeat(0..)
        .map(|parts| parts.concat());

    sym('"') * segment - sym('"')
}

fn json_string<'a>() -> Parser<'a, char, Value> {
    string_literal().map(Value::String)
}

fn json_member<'a>() -> Parser<'a, char, (String, Value)> {
    lexeme(string_literal()) + (lexeme(sym(':')) * call(json_value))
}

fn json_array<'a>() -> Parser<'a, char, Value> {
    let elements = list(call(json_value), lexeme(sym(',')));
    (lexeme(sym('[')) * elements - lexeme(sym(']'))).map(Value::Array)
}

fn json_object<'a>() -> Parser<'a, char, Value> {
    let members = list(json_member(), lexeme(sym(',')));
    (lexeme(sym('{')) * members - lexeme(sym('}')))
        .map(|entries| Value::Object(entries.into_iter().collect::<Map<_, _>>()))
}

fn json_value_inner<'a>() -> Parser<'a, char, Value> {
    json_null() | json_bool() | json_number() | json_string() | json_array() | json_object()
}

fn json_value<'a>() -> Parser<'a, char, Value> {
    spaces() * call(json_value_inner) - spaces()
}

fn json_document<'a>() -> Parser<'a, char, Value> {
    json_value() - end()
}

pub fn parse_json_value(input: &[u8]) -> Result<Value, String> {
    match str::from_utf8(input) {
        Ok(s) => {
            let chars: Vec<char> = s.chars().collect();
            let parser = json_document();
            parser
                .parse(&chars)
                .map_err(|e| format!("pom error: {:?}", e))
        }
        Err(_) => Err("invalid utf8".into()),
    }
}
