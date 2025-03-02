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

// 従来のParser実装
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

// StaticParser実装
pub mod static_parsers {
  use super::{decode_utf16, JsonValue, REPLACEMENT_CHARACTER};
  use oni_comb_parser_rs::prelude::static_parsers::*;
  use oni_comb_parser_rs::prelude::{ConversionParser, DiscardParser, OperatorParser, RepeatParser};
  use oni_comb_parser_rs::StaticParser;
  use std::collections::HashMap;
  use std::str::FromStr;

  pub fn space_static<'a>() -> StaticParser<'a, char, ()> {
    elm_of(" \t\r\n").of_many0().discard()
  }

  pub fn number_static<'a>() -> StaticParser<'a, char, f64> {
    let digit = elm_pred(|c: &char| c.is_digit(10));
    let digit_1_9 = elm_pred(|c: &char| *c >= '1' && *c <= '9');

    // Parse integer part
    let int_parser = digit_1_9 * digit.clone().of_many0() | elm_ref('0').map(|c| vec![*c]);

    // Parse optional minus sign
    let minus_parser = elm_ref('-')
      .opt()
      .map(|m_opt| if let Some(m) = m_opt { vec![*m] } else { vec![] });

    // Parse optional fraction part
    let frac_parser = (elm_ref('.') * digit.clone().of_many1()).opt().map(|frac_opt| {
      if let Some(frac_digits) = frac_opt {
        let mut result = vec!['.'];
        result.extend(frac_digits);
        result
      } else {
        vec![]
      }
    });

    // Parse optional exponent part
    let exp_sign_parser = elm_of("+-")
      .opt()
      .map(|sign_opt| if let Some(sign) = sign_opt { vec![sign] } else { vec![] });

    let exp_digits_parser = digit.clone().of_many1();

    // Parse optional exponent part
    let exp_e_parser = elm_of("eE");
    let exp_parser = (exp_e_parser + exp_sign_parser + exp_digits_parser)
      .opt()
      .map(|exp_opt| {
        if let Some(((e_char, sign_chars), digits)) = exp_opt {
          let mut result = vec![e_char];
          result.extend(sign_chars);
          result.extend(digits);
          result
        } else {
          vec![]
        }
      });

    // Combine all parts into a single parser using tuple operators
    let number_parser = (minus_parser + int_parser).map(|(minus_chars, int_chars)| {
      let mut result = Vec::new();
      result.extend(minus_chars);
      result.extend(int_chars);
      result
    });

    let number_parser = (number_parser + frac_parser).map(|(num_chars, frac_chars)| {
      let mut result = num_chars;
      result.extend(frac_chars);
      result
    });

    let number_parser = (number_parser + exp_parser).map(|(num_chars, exp_chars)| {
      let mut result = num_chars;
      result.extend(exp_chars);
      result
    });

    // Convert to string and parse as f64
    number_parser
      .map(|chars| chars.iter().collect::<String>())
      .map_res(|s| f64::from_str(&s))
  }

  pub fn string_static<'a>() -> StaticParser<'a, char, String> {
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
      .map(|chars: Vec<char>| chars.iter().collect::<String>());

    let hex_digit = elm_pred(|c: &char| c.is_digit(16));
    let utf16_char = elm_ref('\\')
      * (elm_ref('u')
        * (hex_digit
          .of_count(4)
          .map(|chars: Vec<char>| chars.iter().collect::<String>())
          .map_res(|s| u16::from_str_radix(&s, 16))));

    let utf16_string = utf16_char.of_many1().map(|chars| {
      decode_utf16(chars)
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect::<String>()
    });

    let string_content = (char_string | utf16_string).of_many0();
    let string_parser = elm_ref('"') * string_content - elm_ref('"');

    string_parser.map(|strings| strings.concat())
  }

  pub fn array_static<'a>() -> StaticParser<'a, char, Vec<JsonValue>> {
    let sep = space_static() * elm_ref(',') * space_static();

    // First element followed by zero or more comma-separated elements
    let first_elem = value_static();
    let rest_elems = (sep * value_static()).of_many0();

    // Combine first element with rest elements if first element exists
    let elems = (first_elem + rest_elems)
      .map(|(first, rest): (JsonValue, Vec<JsonValue>)| {
        let mut result = vec![first];
        result.extend(rest);
        result
      })
      .opt()
      .map(|opt| opt.unwrap_or_else(Vec::new));

    let array_parser = elm_ref('[') * (space_static() * elems) - (space_static() * elm_ref(']'));
    array_parser
  }

  pub fn object_static<'a>() -> StaticParser<'a, char, HashMap<String, JsonValue>> {
    let sep = space_static() * elm_ref(',') * space_static();

    // Parse a key-value pair
    let key_parser = string_static();
    let colon_parser = space_static() * elm_ref(':') * space_static();
    let value_parser = value_static();
    let member = (key_parser + (colon_parser * value_parser)).map(|(key, value): (String, JsonValue)| (key, value));

    // First member followed by zero or more comma-separated members
    let first_member = member.clone();
    let rest_members = (sep * member).of_many0();

    // Combine first member with rest members if first member exists
    let members = (first_member + rest_members)
      .map(|(first, rest): ((String, JsonValue), Vec<(String, JsonValue)>)| {
        let mut result = vec![first];
        result.extend(rest);
        result
      })
      .opt()
      .map(|opt| opt.unwrap_or_else(Vec::new));

    let obj_parser = elm_ref('{') * (space_static() * members) - (space_static() * elm_ref('}'));
    obj_parser.map(|members| members.into_iter().collect::<HashMap<String, JsonValue>>())
  }

  pub fn value_static<'a>() -> StaticParser<'a, char, JsonValue> {
    let null_parser = elm_ref('n') * elm_ref('u') * elm_ref('l') * elm_ref('l').map(|_| JsonValue::Null);

    let true_parser = elm_ref('t') * elm_ref('r') * elm_ref('u') * elm_ref('e').map(|_| JsonValue::Bool(true));

    let false_parser =
      elm_ref('f') * elm_ref('a') * elm_ref('l') * elm_ref('s') * elm_ref('e').map(|_| JsonValue::Bool(false));

    (null_parser
      | true_parser
      | false_parser
      | number_static().map(|num| JsonValue::Num(num))
      | string_static().map(|text| JsonValue::Str(text))
      | array_static().map(|arr| JsonValue::Array(arr))
      | object_static().map(|obj| JsonValue::Object(obj)))
      - space_static()
  }

  pub fn json_static<'a>() -> StaticParser<'a, char, JsonValue> {
    space_static() * value_static() - end()
  }
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

  // 従来のParserを使用した例
  println!("Parser result: {:?}", json().parse(&input));

  // StaticParserを使用した例
  println!("StaticParser result: {:?}", static_parsers::json_static().parse(&input));
}
