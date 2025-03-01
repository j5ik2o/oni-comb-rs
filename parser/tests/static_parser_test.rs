// Copyright 2021 Developers of the `oni-comb-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use oni_comb_parser_rs::prelude::*;
use std::iter::FromIterator;

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

  let parser = elm_ref('a').to_static_parser().flat_map(|_| elm_ref('b').to_static_parser());
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
