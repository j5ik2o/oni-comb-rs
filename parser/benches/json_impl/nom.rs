#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while},
    character::complete::{char, one_of},
    combinator::{all_consuming, cut, map, opt, value},
    error::{context, ContextError, ParseError, VerboseError},
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};
use serde_json::{Map, Number, Value};
use std::collections::HashMap;
use std::str;

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Str(String),
    Boolean(bool),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(
        take_while(|c| c != '"' && c != '\\'),
        '\\',
        one_of("\"nrtbf/\\u"),
    )(i)
}

fn boolean<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, bool, E> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));
    alt((parse_true, parse_false)).parse(input)
}

fn null<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value((), tag("null")).parse(input)
}

fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        preceded(char('"'), cut(terminated(parse_str, char('"')))),
    )
    .parse(i)
}

fn array<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<JsonValue>, E> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(sp, char(',')), json_value),
                preceded(sp, char(']')),
            )),
        ),
    )
    .parse(i)
}

fn key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, JsonValue), E> {
    separated_pair(
        preceded(sp, string),
        cut(preceded(sp, char(':'))),
        json_value,
    )
    .parse(i)
}

fn hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, HashMap<String, JsonValue>, E> {
    context(
        "map",
        preceded(
            char('{'),
            cut(terminated(
                map(
                    separated_list0(preceded(sp, char(',')), key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            .map(|(k, v)| (String::from(k), v))
                            .collect()
                    },
                ),
                preceded(sp, char('}')),
            )),
        ),
    )
    .parse(i)
}

fn json_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, JsonValue, E> {
    preceded(
        sp,
        alt((
            map(hash, JsonValue::Object),
            map(array, JsonValue::Array),
            map(string, |s| JsonValue::Str(String::from(s))),
            map(double, JsonValue::Num),
            map(boolean, JsonValue::Boolean),
            map(null, |_| JsonValue::Null),
        )),
    )
    .parse(i)
}

fn root<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, JsonValue, E> {
    delimited(sp, json_value, opt(sp)).parse(i)
}

fn into_value(value: JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::Null,
        JsonValue::Str(s) => Value::String(s),
        JsonValue::Boolean(b) => Value::Bool(b),
        JsonValue::Num(n) => Number::from_f64(n)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        JsonValue::Array(items) => Value::Array(items.into_iter().map(into_value).collect()),
        JsonValue::Object(map) => Value::Object(
            map.into_iter()
                .map(|(k, v)| (k, into_value(v)))
                .collect::<Map<_, _>>(),
        ),
    }
}

pub fn parse_json_value(input: &[u8]) -> Result<Value, String> {
    match str::from_utf8(input) {
        Ok(s) => match all_consuming(root::<VerboseError<&str>>)(s) {
            Ok((_, json)) => Ok(into_value(json)),
            Err(e) => Err(format!("nom error: {:?}", e)),
        },
        Err(_) => Err("invalid utf8".into()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_nom_json() {
        let input = br#"{"a": [1, 2, 3], "b": true}"#;
        let value = super::parse_json_value(input).unwrap();
        assert!(value.is_object());
    }
}
