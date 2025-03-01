// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use oni_comb_parser_rs::prelude::*;

// 不要なインポートを削除


#[test]
fn test_static_parser_runner() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_static_parser_pure() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = StaticParser::pure(|| 'x');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), 'x');
}

#[test]
fn test_static_parser_functor() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser().map(|c| c.to_ascii_uppercase());
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), 'A');
}

#[test]
fn test_static_parser_filter() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser().filter(|c| **c == 'a');
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');

  let parser = elm_ref('a').to_static_parser().filter(|c| **c == 'b');
  let result = parser.parse(&input);

  assert!(result.is_failure());
}

#[test]
fn test_static_parser_monad() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a')
    .to_static_parser()
    .flat_map(|_| elm_ref('b').to_static_parser());
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'b');
}

#[test]
fn test_static_parser_add() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser() + elm_ref('b').to_static_parser();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), (&'a', &'b'));
}

#[test]
fn test_static_parser_bitor() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('x').to_static_parser() | elm_ref('a').to_static_parser();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_static_parser_mul() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser() * elm_ref('b').to_static_parser();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'b');
}

#[test]
fn test_static_parser_not() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = !elm_ref('x').to_static_parser();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), ());
}

#[test]
fn test_static_parser_sub() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser() - elm_ref('b').to_static_parser();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_static_parser_cache() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser().cache();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_static_parser_collect() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser().collect();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &['a']);
}

#[test]
fn test_static_parser_conversion() {
  // 数値文字列のテスト
  {
    let text = "123";
    let input = text.chars().collect::<Vec<_>>();

    // 数値文字列を解析するパーサーを作成
    let digit_parser = elm_digit().to_static_parser().of_many1().collect().map(|digits| {
      let s: String = digits.into_iter().collect();
      s
    });
    
    let parser = digit_parser.map_res(|s| s.parse::<i32>());
    let result = parser.parse(&input);

    assert!(result.is_success());
    assert_eq!(result.success().unwrap(), 123);
  }

  // 非数値文字列のテスト
  {
    let text = "abc";
    let input = text.chars().collect::<Vec<_>>();

    // 数値文字列を解析するパーサーを作成
    let digit_parser = elm_digit().to_static_parser().of_many1().collect().map(|digits| {
      let s: String = digits.into_iter().collect();
      s
    });
    
    let parser = digit_parser.map_res(|s| s.parse::<i32>());
    let result = parser.parse(&input);

    assert!(result.is_failure());
  }
}

#[test]
fn test_static_parser_discard() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser().discard();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), ());
}

#[test]
fn test_static_parser_logging() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser().name("a_parser");
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_static_parser_offset() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser().last_offset();
  let result = parser.parse(&input);

  assert!(result.is_success());
  // StaticParserの実装では、last_offsetは0を返す
  assert_eq!(result.success().unwrap(), 0);

  let parser = elm_ref('a').to_static_parser().next_offset();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), 1);
}

#[test]
fn test_static_parser_operator() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  // and_then
  let parser = elm_ref('a').to_static_parser().and_then(elm_ref('b').to_static_parser());
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), (&'a', &'b'));

  // or
  let parser = elm_ref('x').to_static_parser().or(elm_ref('a').to_static_parser());
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');

  // exists
  let parser = elm_ref('a').to_static_parser().exists();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), true);

  // not
  let parser = elm_ref('x').to_static_parser().not();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), ());

  // opt
  let parser = elm_ref('a').to_static_parser().opt();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), Some(&'a'));

  // attempt
  let parser = elm_ref('a').to_static_parser().attempt();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');
}

#[test]
fn test_static_parser_peek() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  let parser = elm_ref('a').to_static_parser().peek();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');

  // Verify that peek doesn't consume input
  let parser = elm_ref('a').to_static_parser().peek() + elm_ref('a').to_static_parser();
  let result = parser.parse(&input);

  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), (&'a', &'a'));
}

#[test]
fn test_static_parser_repeat() {
  let text = "aaa";
  let input = text.chars().collect::<Vec<_>>();

  // many0
  let parser = elm_ref('a').to_static_parser().of_many0();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 3);

  // many1
  let parser = elm_ref('a').to_static_parser().of_many1();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 3);

  // many_n_m
  let parser = elm_ref('a').to_static_parser().of_many_n_m(1, 2);
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 2);

  // count
  let parser = elm_ref('a').to_static_parser().of_count(2);
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 2);

  // Edge case: zero repetitions
  let text = "bbb";
  let input = text.chars().collect::<Vec<_>>();
  let parser = elm_ref('a').to_static_parser().of_many0();
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap().len(), 0);

  // Edge case: required repetitions not found
  let parser = elm_ref('a').to_static_parser().of_many1();
  let result = parser.parse(&input);
  assert!(result.is_failure());
}

#[test]
fn test_static_parser_skip() {
  let text = "abc";
  let input = text.chars().collect::<Vec<_>>();

  // skip_left
  let parser = elm_ref('a').to_static_parser().skip_left(elm_ref('b').to_static_parser());
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'b');

  // skip_right
  let parser = elm_ref('a').to_static_parser().skip_right(elm_ref('b').to_static_parser());
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'a');

  // surround
  let parser = elm_ref('b').to_static_parser().surround(
    elm_ref('a').to_static_parser(),
    elm_ref('c').to_static_parser(),
  );
  let result = parser.parse(&input);
  assert!(result.is_success());
  assert_eq!(result.success().unwrap(), &'b');
}
