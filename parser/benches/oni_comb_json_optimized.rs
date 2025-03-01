use oni_comb_parser_rs::prelude::*;
use oni_comb_parser_rs::StaticParser;
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

// 静的ディスパッチを使用した最適化されたパーサー実装

fn space_optimized<'a>() -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, ()> + Clone + 'a {
  let parser = elm_of(" \t\r\n").of_many0().discard();
  move |state| parser.run(state)
}

fn number_optimized<'a>() -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, f64> + Clone + 'a {
  let integer = elm_digit_1_9_ref() - elm_digit_ref().of_many0() | elm_ref('0');
  let frac = elm_ref('.') + elm_digit_ref().of_many1();
  let exp = elm_of("eE") + elm_of("+-").opt() + elm_digit_ref().of_many1();
  let number = elm_ref('-').opt() + integer + frac.opt() + exp.opt();
  let parser = number.collect().map(String::from_iter).map_res(|s| f64::from_str(&s));
  
  move |state| parser.run(state)
}

fn string_optimized<'a>() -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, String> + Clone + 'a {
  // エスケープ文字の処理
  let special_char = elm_ref('\\')
    | elm_ref('/')
    | elm_ref('"')
    | elm_ref('b').map(|_| &'\x08')
    | elm_ref('f').map(|_| &'\x0C')
    | elm_ref('n').map(|_| &'\n')
    | elm_ref('r').map(|_| &'\r')
    | elm_ref('t').map(|_| &'\t');

  // エスケープシーケンス
  let escape_sequence = elm_ref('\\') * special_char;

  // 通常の文字列（エスケープシーケンスを含む）
  let char_string = (none_ref_of("\\\"") | escape_sequence)
    .map(|c| *c) // Clone::clone の代わりに参照外し
    .of_many1()
    .map(String::from_iter);

  // UTF-16文字の処理（ベンチマークでは使用されないが、完全性のために残す）
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

  // 文字列全体
  let string = surround(
    elm_ref('"'),
    (char_string | utf16_string).of_many0(), // キャッシュを使用しない
    elm_ref('"'),
  );

  let parser = string.map(|strings| strings.concat());
  
  move |state| parser.run(state)
}

fn boolean_optimized<'a>() -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, bool> + Clone + 'a {
  let parser = tag("true").map(|_| true) | tag("false").map(|_| false);
  move |state| parser.run(state)
}

// 遅延評価のための関数型
type JsonValueParser<'a> = Box<dyn Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + 'a>;

// 値パーサーの遅延評価（循環参照を解決するため）
fn value_lazy<'a>(value_parser: &'a JsonValueParser<'a>) -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + Clone + 'a {
  move |state| value_parser(state)
}

// 静的ディスパッチを使用した遅延評価関数
fn value_static<'a, F>(value_parser: F) -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + Clone + 'a 
where
  F: Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + Clone + 'a,
{
  move |state| value_parser(state)
}

fn array_optimized<'a>(value_parser_fn: impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + Clone + 'a) -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, Vec<JsonValue>> + Clone + 'a {
  // 空白を含むカンマ区切りのパターン
  let comma_sep = Parser::new(move |state| {
    let space_fn1 = space_optimized();
    let result1 = space_fn1(state);
    match result1 {
      ParseResult::Success { length, .. } => {
        let state2 = state.add_offset(length);
        let result2 = elm_ref(',').run(&state2);
        match result2 {
          ParseResult::Success { length: length2, .. } => {
            let state3 = state2.add_offset(length2);
            let space_fn2 = space_optimized();
            let result3 = space_fn2(&state3);
            match result3 {
              ParseResult::Success { length: length3, .. } => {
                ParseResult::successful((), length + length2 + length3)
              }
              ParseResult::Failure { error, committed_status } => {
                ParseResult::Failure { error, committed_status }
              }
            }
          }
          ParseResult::Failure { error, committed_status } => {
            ParseResult::Failure { error, committed_status }
          }
        }
      }
      ParseResult::Failure { error, committed_status } => {
        ParseResult::Failure { error, committed_status }
      }
    }
  });

  // 配列要素のパーサー
  let value_fn = value_parser_fn.clone();
  let elems_parser = Parser::new(move |state| value_fn(state)).of_many0_sep(comma_sep);

  // 配列全体のパーサー（角括弧で囲まれた要素）
  let space_fn1 = space_optimized();
  let space_fn2 = space_optimized();
  let parser = surround(elm_ref('[') - Parser::new(space_fn1), elems_parser, Parser::new(space_fn2) * elm_ref(']'));
  
  move |state| parser.run(state)
}

fn object_optimized<'a>(value_parser_fn: impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + Clone + 'a) -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, HashMap<String, JsonValue>> + Clone + 'a {
  let string_fn = string_optimized();
  let value_fn = value_parser_fn.clone();
  
  // キーと値のペアのパーサー
  let member = Parser::new(move |state| {
    let result1 = string_fn(state);
    match result1 {
      ParseResult::Success { value, length } => {
        let state2 = state.add_offset(length);
        let space_fn = space_optimized();
        let result2 = space_fn(&state2);
        match result2 {
          ParseResult::Success { length: length2, .. } => {
            let state3 = state2.add_offset(length2);
            let result3 = elm_ref(':').run(&state3);
            match result3 {
              ParseResult::Success { length: length3, .. } => {
                let state4 = state3.add_offset(length3);
                let result4 = space_fn(&state4);
                match result4 {
                  ParseResult::Success { length: length4, .. } => {
                    let state5 = state4.add_offset(length4);
                    let result5 = value_fn(&state5);
                    match result5 {
                      ParseResult::Success { value: value2, length: length5 } => {
                        ParseResult::successful((value, value2), length + length2 + length3 + length4 + length5)
                      }
                      ParseResult::Failure { error, committed_status } => {
                        ParseResult::Failure { error, committed_status }
                      }
                    }
                  }
                  ParseResult::Failure { error, committed_status } => {
                    ParseResult::Failure { error, committed_status }
                  }
                }
              }
              ParseResult::Failure { error, committed_status } => {
                ParseResult::Failure { error, committed_status }
              }
            }
          }
          ParseResult::Failure { error, committed_status } => {
            ParseResult::Failure { error, committed_status }
          }
        }
      }
      ParseResult::Failure { error, committed_status } => {
        ParseResult::Failure { error, committed_status }
      }
    }
  });

  // 空白を含むカンマ区切りのパターン
  let space_fn1 = space_optimized();
  let space_fn2 = space_optimized();
  let comma_sep = Parser::new(space_fn1) * elm_ref(',') - Parser::new(space_fn2);

  // オブジェクトメンバーのパーサー
  let members = member.of_many0_sep(comma_sep);

  // オブジェクト全体のパーサー（波括弧で囲まれたメンバー）
  let space_fn3 = space_optimized();
  let space_fn4 = space_optimized();
  let obj = surround(elm_ref('{') - Parser::new(space_fn3), members, Parser::new(space_fn4) * elm_ref('}'));

  // メンバーをHashMapに変換
  let parser = obj.map(|members| members.into_iter().collect::<HashMap<_, _>>());
  
  move |state| parser.run(state)
}

fn value_optimized<'a>() -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + Clone + 'a {
  // 循環参照を解決するための遅延評価
  let value_fn: std::rc::Rc<std::cell::RefCell<Option<Box<dyn Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + 'a>>>> = 
    std::rc::Rc::new(std::cell::RefCell::new(None));
  
  // 実際のパーサー実装
  let string_fn = string_optimized();
  let number_fn = number_optimized();
  let boolean_fn = boolean_optimized();
  let space_fn = space_optimized();
  
  // 値パーサーの実装
  let parser_impl = {
    let value_fn_clone = value_fn.clone();
    
    move |state: &ParseState<'a, char>| {
      // 文字列パース
      let result = string_fn(state);
      if let ParseResult::Success { value, length } = result {
        return ParseResult::successful(JsonValue::Str(value), length);
      }
      
      // 数値パース
      let result = number_fn(state);
      if let ParseResult::Success { value, length } = result {
        return ParseResult::successful(JsonValue::Num(value), length);
      }
      
      // 真偽値パース
      let result = boolean_fn(state);
      if let ParseResult::Success { value, length } = result {
        return ParseResult::successful(JsonValue::Bool(value), length);
      }
      
      // nullパース
      let result = tag("null").run(state);
      if let ParseResult::Success { length, .. } = result {
        return ParseResult::successful(JsonValue::Null, length);
      }
      
      // 配列パース
      let array_fn = {
        let value_fn_clone = value_fn_clone.clone();
        array_optimized(move |s| {
          if let Some(ref f) = *value_fn_clone.borrow() {
            f(s)
          } else {
            let input = s.input();
            let offset = s.next_offset();
            let pe = ParseError::of_mismatch(input, offset, 0, "value parser not initialized".to_string());
            ParseResult::failed_with_uncommitted(pe)
          }
        })
      };
      
      let result = array_fn(state);
      if let ParseResult::Success { value, length } = result {
        return ParseResult::successful(JsonValue::Array(value), length);
      }
      
      // オブジェクトパース
      let object_fn = {
        let value_fn_clone = value_fn_clone.clone();
        object_optimized(move |s| {
          if let Some(ref f) = *value_fn_clone.borrow() {
            f(s)
          } else {
            let input = s.input();
            let offset = s.next_offset();
            let pe = ParseError::of_mismatch(input, offset, 0, "value parser not initialized".to_string());
            ParseResult::failed_with_uncommitted(pe)
          }
        })
      };
      
      let result = object_fn(state);
      if let ParseResult::Success { value, length } = result {
        return ParseResult::successful(JsonValue::Object(value), length);
      }
      
      // どれにもマッチしない場合はエラー
      let input = state.input();
      let offset = state.next_offset();
      let pe = ParseError::of_mismatch(input, offset, 0, "expected json value".to_string());
      ParseResult::failed_with_uncommitted(pe)
    }
  };
  
  // パーサーを設定
  *value_fn.borrow_mut() = Some(Box::new(parser_impl.clone()));
  
  // 最終的なパーサー
  move |state| {
    let result = parser_impl(state);
    match result {
      ParseResult::Success { value, length } => {
        let state2 = state.add_offset(length);
        let space_fn2 = space_optimized();
        let result2 = space_fn2(&state2);
        match result2 {
          ParseResult::Success { length: length2, .. } => {
            ParseResult::successful(value, length + length2)
          }
          ParseResult::Failure { error, committed_status } => {
            ParseResult::Failure { error, committed_status }
          }
        }
      }
      ParseResult::Failure { error, committed_status } => {
        ParseResult::Failure { error, committed_status }
      }
    }
  }
}

fn json_optimized<'a>() -> impl Fn(&ParseState<'a, char>) -> ParseResult<'a, char, JsonValue> + Clone + 'a {
  let value_fn = value_optimized();
  
  // 先頭の空白をスキップし、値をパースし、終端を確認
  move |state| {
    let space_fn1 = space_optimized();
    let result1 = space_fn1(state);
    match result1 {
      ParseResult::Success { length, .. } => {
        let state2 = state.add_offset(length);
        let result2 = value_fn(&state2);
        match result2 {
          ParseResult::Success { value, length: length2 } => {
            let state3 = state2.add_offset(length2);
            let result3 = end().run(&state3);
            match result3 {
              ParseResult::Success { length: length3, .. } => {
                ParseResult::successful(value, length + length2 + length3)
              }
              ParseResult::Failure { error, committed_status } => {
                ParseResult::Failure { error, committed_status }
              }
            }
          }
          ParseResult::Failure { error, committed_status } => {
            ParseResult::Failure { error, committed_status }
          }
        }
      }
      ParseResult::Failure { error, committed_status } => {
        ParseResult::Failure { error, committed_status }
      }
    }
  }
}

pub fn oni_comb_parse_json_optimized(s: &str) {
  // 文字列を文字のベクターに変換
  let input: Vec<char> = s.chars().collect();
  
  // 最適化されたパーサーを使用
  let parser_fn = json_optimized();
  let parse_state = ParseState::new(&input, 0);
  
  // パース実行
  let _ = parser_fn(&parse_state).success().unwrap();
}
