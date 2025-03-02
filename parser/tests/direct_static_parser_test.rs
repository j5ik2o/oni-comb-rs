// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Import necessary traits and types for StaticParser
use oni_comb_parser_rs::prelude::static_parsers::*;
use oni_comb_parser_rs::prelude::{
  CacheParser, ConversionParser, DiscardParser, FilterParsers, LoggingParser, OffsetParser, OperatorParser, ParserPure,
  PeekParser, RepeatParser, SkipParser,
};
use oni_comb_parser_rs::StaticParser;

// 直接StaticParserを使用するテスト

#[test]
fn test_direct_static_parser_runner() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_direct_static_parser_pure() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = StaticParser::pure(|| 'x');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), 'x');
}

#[test]
fn test_direct_static_parser_functor() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').map(|c| c.to_ascii_uppercase());
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), 'A');
}

#[test]
fn test_direct_static_parser_filter() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').filter(|c| **c == 'a');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');

  let parser = elm_ref('a').filter(|c| **c == 'b');
  let result = parser.parse(&input);

  assert!(result.is_failure());
}

#[test]
fn test_direct_static_parser_monad() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').flat_map(|_| elm_ref('b'));
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'b');
}

#[test]
fn test_direct_static_parser_add() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a') + elm_ref('b');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), (&'a', &'b'));
}

#[test]
fn test_direct_static_parser_bitor() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('x') | elm_ref('a');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_direct_static_parser_mul() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a') * elm_ref('b');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'b');
}

#[test]
fn test_direct_static_parser_not() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = !elm_ref('x');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), ());
}

#[test]
fn test_direct_static_parser_sub() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a') - elm_ref('b');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_direct_static_parser_cache() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').cache();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_direct_static_parser_collect() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').collect();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), vec![&'a']);
}

#[test]
fn test_direct_static_parser_conversion() {
  // 数値文字列のテスト
  {
    // 静的な文字配列を使用
    static DIGITS: [char; 3] = ['1', '2', '3'];
    let char_refs: Vec<&char> = DIGITS.iter().collect();

    // 数値文字列を解析するパーサーを作成
    let digit_parser = elm_digit().clone().of_many1().map(|digits| {
      let s: String = digits.into_iter().map(|c: &char| *c).collect();
      s
    });

    let parser = digit_parser.map_res(|s| s.parse::<i32>());
    let result = parser.parse(&char_refs);

    assert!(result.is_success());
    assert_eq!(result.success().unwrap(), 123);
  }

  // 非数値文字列のテスト
  {
    // 静的な文字配列を使用
    static LETTERS: [char; 3] = ['a', 'b', 'c'];
    let char_refs: Vec<&char> = LETTERS.iter().collect();

    // 数値文字列を解析するパーサーを作成
    let digit_parser = elm_digit().clone().of_many1().map(|digits| {
      let s: String = digits.into_iter().map(|c: &char| *c).collect();
      s
    });

    let parser = digit_parser.map_res(|s| s.parse::<i32>());
    let result = parser.parse(&char_refs);

    assert!(result.is_failure());
  }
}

#[test]
fn test_direct_static_parser_discard() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').discard();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), ());
}

#[test]
fn test_direct_static_parser_logging() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').name("a_parser");
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_direct_static_parser_offset() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').last_offset();
  let result = parser.parse(&input);

  assert!(result.is_success());
  // StaticParserの実装では、last_offsetは0を返す
  assert_eq!(result.success().unwrap(), 0);

  let parser = elm_ref('a').next_offset();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), 1);
}

#[test]
fn test_direct_static_parser_operator() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  // and_then
  let parser = elm_ref('a').and_then(elm_ref('b'));
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), (&'a', &'b'));

  // or
  let parser = elm_ref('x').or(elm_ref('a'));
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');

  // exists
  let parser = elm_ref('a').exists();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), true);

  // not
  let parser = elm_ref('x').not();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), ());

  // opt
  let parser = elm_ref('a').opt();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), Some(&'a'));

  // attempt
  let parser = elm_ref('a').attempt();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_direct_static_parser_peek() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').peek();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');

  // Verify that peek doesn't consume input
  let parser = elm_ref('a').peek() + elm_ref('a');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), (&'a', &'a'));
}

#[test]
fn test_direct_static_parser_repeat() {
  let text = "aaa";
  let input = text.chars().collect::<Vec<_>>();

  // many0
  let parser = elm_ref('a').of_many0();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 3);

  // many1
  let parser = elm_ref('a').of_many1();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 3);

  // many_n_m
  let parser = elm_ref('a').of_many_n_m(1, 2);
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 2);

  // count
  let parser = elm_ref('a').of_count(2);
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 2);

  // Edge case: zero repetitions
  let text = "bbb";
  let input = text.chars().collect::<Vec<_>>();
  let parser = elm_ref('a').of_many0();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 0);

  // Edge case: required repetitions not found
  let parser = elm_ref('a').of_many1();
  let result = parser.parse(&input);
  assert!(result.is_failure());
}

#[test]
fn test_direct_static_parser_skip() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  // skip_left
  let parser = elm_ref('a').skip_left(elm_ref('b'));
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'b');

  // skip_right
  let parser = elm_ref('a').skip_right(elm_ref('b'));
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');

  // surround
  let parser = elm_ref('b').surround(elm_ref('a'), elm_ref('c'));
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'b');
}
