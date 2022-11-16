use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::collections::HashMap;

use crate::model::*;

use oni_comb_parser_rs::prelude::*;

fn comment<'a>() -> Parser<'a, u8, &'a [u8]> {
  let head = seq(b"//").collect().attempt() | elm_ref(b'#').collect();
  let tail = take_till0(|c| matches!(*c, b'\r' | b'\n'));
  (space() + (head + tail).of_many1()).collect()
}

fn space<'a>() -> Parser<'a, u8, &'a [u8]> {
  elm_ref_of(b" \t\r\n").of_many0().collect()
}

fn space_or_comment<'a>() -> Parser<'a, u8, ()> {
  (comment().attempt() | space()).discard()
}

fn include_method<'a>() -> Parser<'a, u8, String> {
  (seq(b"file") | seq(b"url"))
    .collect()
    .map_res(std::str::from_utf8)
    .map(String::from)
}

fn include_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  (seq(b"include") * elm(b' ') * include_method() + path().surround(seq(b"(\""), seq(b"\")")))
    .map(|(method, path)| ConfigValue::Include(ConfigIncludeValue::new(method, format!("\"{}\"", path))))
}

fn number<'a>() -> Parser<'a, u8, (Option<&'a u8>, String, Option<String>, Option<String>)> {
  let integer = (elm_digit_1_9_ref() + elm_digit_ref().of_many0()).collect() | elm_ref(b'0').collect();
  let frac = elm_ref(b'.') + elm_digit_ref().of_many1();
  let exp = elm_of(b"eE") + elm_of(b"+-").opt() + elm_digit_ref().of_many1();
  let number = elm_ref(b'-').opt()
    + integer.map_res(std::str::from_utf8).map(String::from)
    + frac.collect().map_res(std::str::from_utf8).map(String::from).opt()
    + exp.collect().map_res(std::str::from_utf8).map(String::from).opt();
  number
    .surround(space_or_comment(), space_or_comment())
    .map(|(((s, i), f), e)| (s, i, f, e))
}

fn duration<'a>() -> Parser<'a, u8, (ConfigNumberValue, TimeUnit)> {
  let ns = (seq(b"ns").attempt()
    | seq(b"nano").attempt()
    | seq(b"nanos").attempt()
    | seq(b"nanosecond").attempt()
    | seq(b"nanoseconds"))
  .map(|_| TimeUnit::Nanoseconds);
  let us = (seq(b"us").attempt()
    | seq(b"micro").attempt()
    | seq(b"micros").attempt()
    | seq(b"microsecond").attempt()
    | seq(b"microseconds"))
  .map(|_| TimeUnit::Microseconds);
  let ms = (seq(b"ms").attempt()
    | seq(b"milli").attempt()
    | seq(b"millis").attempt()
    | seq(b"millisecond").attempt()
    | seq(b"milliseconds"))
  .map(|_| TimeUnit::Milliseconds);
  let s = (seq(b"s").attempt() | seq(b"second").attempt() | seq(b"seconds")).map(|_| TimeUnit::Seconds);
  let m = (seq(b"m").attempt() | seq(b"minute").attempt() | seq(b"minutes")).map(|_| TimeUnit::Minutes);
  let h = (seq(b"h").attempt() | seq(b"hour").attempt() | seq(b"hours")).map(|_| TimeUnit::Hours);
  let d = (seq(b"d").attempt() | seq(b"day").attempt() | seq(b"days")).map(|_| TimeUnit::Days);

  number_value() + (ns.attempt() | us.attempt() | ms.attempt() | s.attempt() | m.attempt() | h.attempt() | d)
}

fn duration_value<'a>() -> Parser<'a, u8, ConfigValue> {
  duration().map(|(nv, u)| ConfigValue::Duration(ConfigDurationValue::new(nv, u)))
}

fn number_value<'a>() -> Parser<'a, u8, ConfigNumberValue> {
  number().map(|(s, i, f, e)| match (s, i, f, e) {
    (None, i, None, None) => ConfigNumberValue::from(i),
    (Some(_), i, None, None) => ConfigNumberValue::from(i),
    (_, i, Some(f), None) => {
      let mut s = i;
      s.push_str(&f);
      ConfigNumberValue::from(s)
    }
    (_, i, Some(f), Some(e)) => {
      let mut s = i;
      s.push_str(&f);
      s.push_str(&e);
      ConfigNumberValue::from(s)
    }
    _ => panic!("no match !!!"),
  })
}

fn string_double_quote_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'"').surround(space_or_comment(), space_or_comment())
}

fn string_single_quote_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'\'').surround(space_or_comment(), space_or_comment())
}

fn string<'a>() -> Parser<'a, u8, String> {
  let special_char = elm_ref(b'\\')
    | elm_ref(b'/')
    | elm_ref(b'"')
    | elm_ref(b'b').map(|_| &b'\x08')
    | elm_ref(b'f').map(|_| &b'\x0C')
    | elm_ref(b'n').map(|_| &b'\n')
    | elm_ref(b'r').map(|_| &b'\r')
    | elm_ref(b't').map(|_| &b'\t');
  let escape_sequence = elm_ref(b'\\') * special_char;
  // let char_string = (none_ref_of(b"\r\n\\\"'") | escape_sequence)
  let char_string = (none_ref_of(b"[]\r\n\\\"'") | escape_sequence)
    .map(Clone::clone)
    .of_many1()
    .map_res(String::from_utf8);
  let utf16_char = seq(b"\\u")
    * elm_hex_digit()
      .of_count(4)
      .map_res(String::from_utf8)
      .map_res(|digits| u16::from_str_radix(&digits, 16));
  let utf16_string = utf16_char.of_many1().map(|chars| {
    decode_utf16(chars)
      .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
      .collect::<String>()
  });
  let string_surround = || (char_string.clone().attempt() | utf16_string.clone()).of_many1();
  let string_surround1 = string_surround().surround(string_double_quote_bracket(), string_double_quote_bracket());
  let string_surround2 = string_surround().surround(string_single_quote_bracket(), string_single_quote_bracket());
  (string_surround1.attempt() | string_surround2.attempt() | string_surround()).map(|strings| strings.concat())
}

fn string_zero<'a>() -> Parser<'a, u8, String> {
  let special_char = elm_ref(b'\\')
    | elm_ref(b'/')
    | elm_ref(b'"')
    | elm_ref(b'b').map(|_| &b'\x08')
    | elm_ref(b'f').map(|_| &b'\x0C')
    | elm_ref(b'n').map(|_| &b'\n')
    | elm_ref(b'r').map(|_| &b'\r')
    | elm_ref(b't').map(|_| &b'\t');
  let escape_sequence = elm_ref(b'\\') * special_char;
  // let char_string = (none_ref_of(b"\r\n\\\"'") | escape_sequence)
  let char_string = (none_ref_of(b"[]\r\n\\\"'") | escape_sequence)
    .map(Clone::clone)
    .of_many1()
    .map_res(String::from_utf8);
  let utf16_char = seq(b"\\u")
    * elm_hex_digit()
      .of_count(4)
      .map_res(String::from_utf8)
      .map_res(|digits| u16::from_str_radix(&digits, 16));
  let utf16_string = utf16_char.of_many1().map(|chars| {
    decode_utf16(chars)
      .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
      .collect::<String>()
  });
  let string_surround = || (char_string.clone().attempt() | utf16_string.clone()).of_many0();
  let string_surround1 = string_surround().surround(string_double_quote_bracket(), string_double_quote_bracket());
  let string_surround2 = string_surround().surround(string_single_quote_bracket(), string_single_quote_bracket());
  (string_surround1.attempt() | string_surround2.attempt() | string_surround()).map(|strings| strings.concat())
}

fn string_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  string().map(ConfigValue::String)
}

fn string_config_value2<'a>() -> Parser<'a, u8, ConfigValue> {
  string().map(ConfigValue::String)
}

fn array_left_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'[').surround(space_or_comment(), space_or_comment())
}

fn array_right_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b']').surround(space_or_comment(), space_or_comment())
}

fn array<'a>() -> Parser<'a, u8, Vec<ConfigValue>> {
  let elems = lazy(config_value).of_many0_sep(comma());
  elems.surround(array_left_bracket(), array_right_bracket())
}

fn kv<'a>() -> Parser<'a, u8, ()> {
  elm_ref_of(b"=:").discard()
}

pub fn key<'a>() -> Parser<'a, u8, String> {
  (path().attempt() | string()).surround(space_or_comment(), space_or_comment())
}

fn property<'a>() -> Parser<'a, u8, (String, ConfigValue)> {
  key() + ((kv() * lazy(config_value)).attempt() | object_config_value().attempt() | array_config_value())
}

fn property_config_value<'a>() -> Parser<'a, u8, (String, ConfigValue)> {
  property().map(|(k, v)| (k, v))
}

fn object_left_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'{').surround(space_or_comment(), space_or_comment())
}

fn object_right_bracket<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b'}').surround(space_or_comment(), space_or_comment())
}

fn comma<'a>() -> Parser<'a, u8, &'a u8> {
  elm_ref(b',').surround(space_or_comment(), space_or_comment())
}

fn object<'a>() -> Parser<'a, u8, HashMap<String, ConfigValue>> {
  let properties: Parser<'a, u8, Vec<(String, ConfigValue)>> = lazy(property).of_many0_sep(comma().opt());
  let obj: Parser<'a, u8, Vec<(String, ConfigValue)>> =
    properties.surround(object_left_bracket(), object_right_bracket());
  obj.map(|properties| {
    let m: HashMap<String, ConfigValue> = HashMap::new();
    properties.into_iter().fold(m, |mut r, e| {
      match r.get_mut(&e.0) {
        Some(v) => v.push(e.1),
        None => {
          r.insert(e.0, e.1);
        }
      };
      r
    })
  })
}

fn path_element<'a>() -> Parser<'a, u8, &'a [u8]> {
  (elm_alpha() | elm_of(b"-_")).of_many1().collect()
}

fn path<'a>() -> Parser<'a, u8, String> {
  path_element()
    .of_many1_sep(elm(b'.'))
    .collect()
    .map_res(std::str::from_utf8)
    .map(String::from)
}

fn reference_left_bracket<'a>() -> Parser<'a, u8, &'a [u8]> {
  seq(b"${").collect().surround(space_or_comment(), space_or_comment())
}

fn reference_right_bracket<'a>() -> Parser<'a, u8, &'a [u8]> {
  seq(b"}").surround(space_or_comment(), space_or_comment())
}

fn reference<'a>() -> Parser<'a, u8, (bool, String)> {
  (elm_ref(b'?').opt().map(|v| v.is_some())
    + path_element()
      .of_many1_sep(elm(b'.'))
      .collect()
      .map_res(std::str::from_utf8)
      .map(String::from))
  .surround(reference_left_bracket(), reference_right_bracket())
}

fn object_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  object().map(|v| ConfigValue::Object(ConfigObjectValue::new(v)))
}

fn array_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  array().map(|v| ConfigValue::Array(ConfigArrayValue::new(v)))
}

fn reference_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  reference().map(|(missing, ref_name)| ConfigValue::Reference {
    prev: None,
    path: ref_name,
    missing,
  })
}

fn enumeration_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  (string_config_value().attempt() | reference_config_value())
    .of_many1()
    .map(|values| ConfigValue::Enumeration {
      prev: None,
      values: ConfigArrayValue(values),
    })
}

fn simple_config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  (seq(b"null").map(|_| ConfigValue::Null).attempt()
    | seq(b"true").map(|_| ConfigValue::Bool(true)).attempt()
    | seq(b"false").map(|_| ConfigValue::Bool(false)).attempt()
    | duration_value().attempt()
    | number_value().map(ConfigValue::Number).attempt()
    | string_config_value())
  .surround(space_or_comment(), space_or_comment())
}

fn config_value<'a>() -> Parser<'a, u8, ConfigValue> {
  (array_config_value().attempt()
    | object_config_value().attempt()
    | reference_config_value().attempt()
    | simple_config_value().attempt()
    | enumeration_config_value())
  .surround(space_or_comment(), space_or_comment())
}

fn config<'a>() -> Parser<'a, u8, Vec<ConfigValue>> {
  property_config_value()
    .of_many1()
    .map(|values: Vec<(String, ConfigValue)>| {
      let map = values.into_iter().fold(HashMap::new(), |mut key_values, (k, v)| {
        match key_values.get_mut(&k) {
          None => {
            key_values.insert(k, v);
          }
          Some(cv) => {
            cv.merge_with(v);
          }
        }
        key_values
      });
      vec![ConfigValue::Object(ConfigObjectValue::new(map))]
    })
    .attempt()
    | include_config_value().of_many1().attempt()
    | (object_config_value().attempt() | array_config_value()).of_many0()
}

pub fn hocon<'a>() -> Parser<'a, u8, Vec<ConfigValue>> {
  space_or_comment() * config() - end()
}

#[cfg(test)]
mod gens {
  use super::*;
  use prop_check_rs::gen::{Gen, Gens};
  use std::collections::BTreeMap;

  pub fn comment_gen() -> Gen<String> {
    let array: [(i32, Gen<char>); 2] = [(5, Gens::choose_char('a', 'z')), (5, Gens::choose_char('A', 'Z'))];
    let (map, total) = array
      .into_iter()
      .fold((BTreeMap::new(), 0), |(mut map, mut total), (weight, value)| {
        total += weight;
        map.insert(total, value);
        (map, total)
      });
    let char_gen = Gens::choose_i32(1, total).flat_map(move |n| map.range(n..).into_iter().next().unwrap().1.clone());

    let tail = Gens::choose_u8(0, 128)
      .flat_map(move |n| Gens::list_of_n(n as usize, char_gen.clone()).map(|e| e.into_iter().collect::<String>()));
    let head = Gens::choose_u8(1, 2).map(|n| match n {
      1 => "#".to_string(),
      2 => "//".to_string(),
      n => panic!("n = {}", n),
    });
    head.flat_map(move |h| tail.clone().map(move |t| format!("{}{}", h, t)))
  }

  pub fn space_gen() -> Gen<String> {
    Gens::choose_u8(1, 128).flat_map(|n| {
      Gens::list_of_n(
        n as usize,
        Gens::choose_u8(1, 4).map(|n| match n {
          1 => ' ',
          2 => '\r',
          3 => '\n',
          4 => '\t',
          n => panic!("n = {}", n),
        }),
      )
      .map(|chars| chars.into_iter().collect::<String>())
    })
  }

  pub fn comment_space_gen() -> Gen<String> {
    Gens::frequency([(1, comment_gen()), (1, space_gen())])
  }

  pub fn include_method_gen() -> Gen<String> {
    Gens::frequency_values([(1, "file".to_string()), (1, "url".to_string())])
  }

  pub fn include_config_value_gen() -> Gen<String> {
    include_method_gen().flat_map(|method| Gens::pure(format!("include {}(\"{}\")", method, "abc")))
  }

  pub fn path_element_gen() -> Gen<String> {
    Gens::choose_u32(1, 128).flat_map(|n| {
      Gens::list_of_n(
        n as usize,
        Gens::frequency([
          (1, Gens::pure('-')),
          (1, Gens::pure('_')),
          (4, Gens::choose_char('a', 'z')),
          (4, Gens::choose_char('A', 'Z')),
        ]),
      )
      .map(|chars| chars.iter().collect::<String>())
    })
  }

  pub fn path_gen() -> Gen<String> {
    Gens::choose_u32(1, 128)
      .flat_map(|n| Gens::list_of_n(n as usize, path_element_gen()).map(|elements| elements.join(".")))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parsers::gens::*;
  use anyhow::Result;
  use prop_check_rs::prop;
  use prop_check_rs::prop::{MaxSize, TestCases};
  use prop_check_rs::rng::RNG;
  use std::env;

  const MAX_SIZE: MaxSize = 5;
  const TEST_COUNT: TestCases = 100;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  fn new_rng() -> RNG {
    RNG::new()
  }

  #[test]
  fn comment_test() -> Result<()> {
    let mut counter = 0;
    let prop = prop::for_all_gen(comment_gen(), move |input| {
      counter += 1;
      log::debug!("{:>03}, comment:string = {}", counter, input);
      let input_bytes = input.as_bytes();
      let result = (comment() - end()).parse(input_bytes).to_result();
      let comment = std::str::from_utf8(result.unwrap()).unwrap();
      assert_eq!(comment.to_string(), input);
      true
    });
    prop::test_with_prop(prop, MAX_SIZE, TEST_COUNT, new_rng())
  }

  #[test]
  fn space_test() -> Result<()> {
    let prop = prop::for_all_gen(space_gen(), move |input| {
      let input_bytes = input.as_bytes();
      let result = (space() - end()).parse(input_bytes).to_result();
      let comment = std::str::from_utf8(result.unwrap()).unwrap();
      assert_eq!(comment.to_string(), input);
      true
    });
    prop::test_with_prop(prop, MAX_SIZE, TEST_COUNT, new_rng())
  }

  #[test]
  fn space_or_comment_test() -> Result<()> {
    let prop = prop::for_all_gen(comment_space_gen(), move |input| {
      let input_bytes = input.as_bytes();
      let result = (space_or_comment() - end()).parse(input_bytes).to_result();
      assert!(result.is_ok());
      true
    });
    prop::test_with_prop(prop, MAX_SIZE, TEST_COUNT, new_rng())
  }

  #[test]
  fn include_method_test() -> Result<()> {
    let prop = prop::for_all_gen(include_method_gen(), move |input| {
      let input_bytes = input.as_bytes();
      let result = (include_method() - end()).parse(input_bytes).to_result();
      assert_eq!(result.unwrap().to_string(), input);
      true
    });
    prop::test_with_prop(prop, 5, TEST_COUNT, new_rng())
  }

  #[test]
  fn include_config_value_test() -> Result<()> {
    let prop = prop::for_all_gen(include_config_value_gen(), move |input| {
      log::debug!("include_config_value:string = {}", input);
      let input_bytes = input.as_bytes();
      let result = (include_config_value() - end()).parse(input_bytes).to_result();
      assert_eq!(result.unwrap().to_string(), input);
      true
    });
    prop::test_with_prop(prop, MAX_SIZE, TEST_COUNT, new_rng())
  }

  #[test]
  fn path_test() -> Result<()> {
    let prop = prop::for_all_gen(path_gen(), move |input| {
      log::debug!("path:string = {}", input);
      let input_bytes = input.as_bytes();
      let result = (path() - end()).parse(input_bytes).to_result();
      assert_eq!(result.unwrap().to_string(), input);
      true
    });
    prop::test_with_prop(prop, MAX_SIZE, TEST_COUNT, new_rng())
  }
}
