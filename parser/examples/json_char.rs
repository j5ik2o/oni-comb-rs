use oni_comb_parser_rs::core::{Parser, ParserFunctor, ParserRunner};
use oni_comb_parser_rs::extension::parser::{
  CollectParser, ConversionParser, DiscardParser, OperatorParser, RepeatParser,
};
use oni_comb_parser_rs::prelude::*;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
  Null,
  Bool(bool),
  Str(String),
  Num(f64),
  Array(Vec<JsonValue>),
  Object(HashMap<String, JsonValue>),
}

fn space<'a>() -> Parser<'a, char, ()> {
  elm_of(" \t\r\n").of_many0().discard()
}

fn number<'a>() -> Parser<'a, char, f64> {
  let integer = elm_digit_1_9_ref() - elm_digit_ref().of_many0() | elm_ref('0');
  let frac = elm_ref('.') + elm_digit_ref().of_many1();
  let exp = elm_of("eE") + elm_of("+-").opt() + elm_digit_ref().of_many1();
  let number = elm_ref('-').opt() + integer + frac.opt() + exp.opt();
  number.collect().map(String::from_iter).map_res(|s| f64::from_str(&s))
}

fn string<'a>() -> Parser<'a, char, String> {
  let special_char = elm_ref('\\')
    | elm_ref('/')
    | elm_ref('"')
    | elm_ref('b').map(|_| &'\x08')
    | elm_ref('f').map(|_| &'\x0C')
    | elm_ref('n').map(|_| &'\n')
    | elm_ref('r').map(|_| &'\r')
    | elm_ref('t').map(|_| &'\t');
  let escape_sequence = elm_ref('\\') * special_char;
  let char_string = (none_ref_of("\\\"") | escape_sequence)
    .map(Clone::clone)
    .of_many1()
    .map(String::from_iter);
  let utf16_char: Parser<char, u16> = tag("\\u")
    * elm_pred(|c: &char| c.is_digit(16))
      .of_count(4)
      .map(String::from_iter)
      .map_res(|digits| u16::from_str_radix(&digits, 16));
  let utf16_string = utf16_char.of_many1().map(|chars| {
    decode_utf16(chars)
      .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
      .collect::<String>()
  });
  let string = surround(elm_ref('"'), (char_string | utf16_string).of_many0(), elm_ref('"'));
  string.map(|strings| strings.concat())
}

fn array<'a>() -> Parser<'a, char, Vec<JsonValue>> {
  let elems = lazy(value).of_many0_sep(space() * elm_ref(',') - space());
  surround(elm_ref('[') - space(), elems, space() * elm_ref(']'))
}

fn object<'a>() -> Parser<'a, char, HashMap<String, JsonValue>> {
  let member = string() - space() - elm_ref(':') - space() + lazy(value);
  let members = member.of_many0_sep(space() + elm_ref(',') + space());
  let obj = surround(elm_ref('{') + space(), members, space() + elm_ref('}'));
  obj.map(|members| members.into_iter().collect::<HashMap<_, _>>())
}

fn value<'a>() -> Parser<'a, char, JsonValue> {
  (tag("null").map(|_| JsonValue::Null)
    | tag("true").map(|_| JsonValue::Bool(true))
    | tag("false").map(|_| JsonValue::Bool(false))
    | number().map(|num| JsonValue::Num(num))
    | string().map(|text| JsonValue::Str(text))
    | array().map(|arr| JsonValue::Array(arr))
    | object().map(|obj| JsonValue::Object(obj)))
    - space()
}

pub fn json<'a>() -> Parser<'a, char, JsonValue> {
  space() * value() - end()
}

#[allow(dead_code)]
fn main() {
  let test = r#"
	{
        "Image": {
            "Width":  800,
            "Height": 600,
            "Title":  "View from 15th Floor",
            "Thumbnail": {
                "Url":    "http://www.example.com/image/481989943",
                "Height": 125,
                "Width":  100
            },
            "Animated" : false,
            "IDs": [116, 943, 234, 38793]
        },
        "escaped characters": "\u2192\uD83D\uDE00\"\t\uD834\uDD1E"
    }"#;

  let input: Vec<char> = test.chars().collect();
  println!("{:?}", json().parse(&input));
}
