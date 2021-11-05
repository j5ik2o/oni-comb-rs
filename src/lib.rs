#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]

use std::fmt::{Debug, Display};

use crate::core::*;
use crate::internal::*;
use crate::utils::*;

pub mod core;
pub mod extension;
mod internal;
pub mod utils;

// https://github.com/com-lihaoyi/fastparse
// https://github.com/fpinscala/fpinscala/blob/first-edition/answers/src/main/scala/fpinscala/parsing
// https://github.com/Geal/nom
// https://hazm.at/mox/lang/rust/nom/index.html
// https://github.com/J-F-Liu/pom
pub mod prelude {
  use super::*;
  use crate::extension::parsers::{
    ElementParsers, ElementsParsers, LazyParsers, OperatorParsers, PrimitiveParsers, SkipParsers, TakenParsers,
  };

  pub fn regex<'a>(pattern: &str) -> Parser<'a, char, String> {
    ParsersImpl::regex(pattern)
  }

  pub fn lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
  where
    F: Fn() -> Parser<'a, I, A> + 'a,
    A: Debug + 'a, {
    ParsersImpl::lazy(f)
  }

  pub fn unit<'a, I>() -> Parser<'a, I, ()> {
    ParsersImpl::unit()
  }

  pub fn successful<'a, I, A>(value: A) -> Parser<'a, I, A>
  where
    I: 'a,
    A: Clone + 'a, {
    ParsersImpl::successful(value)
  }

  pub fn successful_lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
  where
    I: 'a,
    F: Fn() -> A + 'a,
    A: 'a, {
    ParsersImpl::successful_lazy(f)
  }

  pub fn failed<'a, I, A>(value: ParseError<'a, I>) -> Parser<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    ParsersImpl::failed(value)
  }

  pub fn failed_lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
  where
    F: Fn() -> ParseError<'a, I> + 'a,
    I: 'a,
    A: 'a, {
    ParsersImpl::failed_lazy(f)
  }

  pub fn end<'a, I>() -> Parser<'a, I, ()>
  where
    I: Debug + Display + 'a, {
    ParsersImpl::end()
  }

  pub fn empty<'a, I>() -> Parser<'a, I, ()> {
    ParsersImpl::empty()
  }

  pub fn elm_any_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_any_ref()
  }

  pub fn elm_any<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_any()
  }

  pub fn elm_ref<'a, I>(c: I) -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_ref(c)
  }

  pub fn elm<'a, I>(c: I) -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm(c)
  }

  pub fn elm_pred_ref<'a, I, F>(f: F) -> Parser<'a, I, &'a I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_pred_ref(f)
  }

  pub fn elm_pred<'a, I, F>(f: F) -> Parser<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_pred(f)
  }

  pub fn elm_space_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_space_ref()
  }

  pub fn elm_space<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_space()
  }

  pub fn elm_multi_space_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_multi_space_ref()
  }

  pub fn elm_multi_space<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_multi_space()
  }

  pub fn elm_alpha_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_alpha_ref()
  }

  pub fn elm_alpha<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_alpha()
  }

  pub fn elm_alpha_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_alpha_digit_ref()
  }

  pub fn elm_alpha_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_alpha_digit()
  }

  pub fn elm_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_digit_ref()
  }

  pub fn elm_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_digit()
  }

  pub fn elm_digit_1_9_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    elm_digit_ref().with_filter_not(|c: &&I| c.is_ascii_zero())
  }

  pub fn elm_digit_1_9<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    elm_digit_1_9_ref().map(Clone::clone)
  }

  pub fn elm_hex_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_hex_digit_ref()
  }

  pub fn elm_hex_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    ParsersImpl::elm_hex_digit()
  }

  pub fn elm_oct_digit_ref<'a, I>() -> Parser<'a, I, &'a I>
  where
    I: Element + PartialEq + 'a, {
    ParsersImpl::elm_oct_digit_ref()
  }

  pub fn elm_oct_digit<'a, I>() -> Parser<'a, I, I>
  where
    I: Element + PartialEq + Clone + 'a, {
    ParsersImpl::elm_oct_digit()
  }

  pub fn seq<'a, 'b, I>(tag: &'b [I]) -> Parser<'a, I, &'a [I]>
  where
    I: PartialEq + Debug + 'a,
    'b: 'a, {
    ParsersImpl::seq(tag)
  }

  pub fn tag<'a, 'b>(tag: &'b str) -> Parser<'a, char, &'a str>
  where
    'b: 'a, {
    ParsersImpl::tag(tag)
  }

  pub fn tag_no_case<'a, 'b>(tag: &'b str) -> Parser<'a, char, &'a str>
  where
    'b: 'a, {
    ParsersImpl::tag_no_case(tag)
  }

  pub fn take<'a, I>(n: usize) -> Parser<'a, I, &'a [I]> {
    ParsersImpl::take(n)
  }

  pub fn take_while0<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_while0(f)
  }

  pub fn take_while1<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_while1(f)
  }

  pub fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_while_n_m(n, m, f)
  }

  pub fn take_till0<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_till0(f)
  }

  pub fn take_till1<'a, I, F>(f: F) -> Parser<'a, I, &'a [I]>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Debug + 'a, {
    ParsersImpl::take_till1(f)
  }

  pub fn skip<'a, I>(n: usize) -> Parser<'a, I, ()> {
    ParsersImpl::skip(n)
  }

  pub fn elm_ref_of<'a, I, S>(set: &'a S) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    ParsersImpl::elm_ref_of(set)
  }

  pub fn elm_of<'a, I, S>(set: &'a S) -> Parser<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    ParsersImpl::elm_of(set)
  }

  pub fn elm_in_ref<'a, I>(start: I, end: I) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    ParsersImpl::elm_ref_in(start, end)
  }

  pub fn elm_in<'a, I>(start: I, end: I) -> Parser<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a, {
    ParsersImpl::elm_in(start, end)
  }

  pub fn elm_from_until_ref<'a, I>(start: I, end: I) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Debug + 'a, {
    ParsersImpl::elm_ref_from_until(start, end)
  }

  pub fn elm_from_until<'a, I>(start: I, end: I) -> Parser<'a, I, I>
  where
    I: PartialEq + PartialOrd + Display + Copy + Clone + Debug + 'a, {
    ParsersImpl::elm_from_until(start, end)
  }

  pub fn none_ref_of<'a, I, S>(set: &'a S) -> Parser<'a, I, &'a I>
  where
    I: PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    ParsersImpl::none_ref_of(set)
  }

  pub fn none_of<'a, I, S>(set: &'a S) -> Parser<'a, I, I>
  where
    I: PartialEq + Display + Clone + Debug + 'a,
    S: Set<I> + ?Sized, {
    ParsersImpl::none_of(set)
  }

  pub fn surround<'a, I, A, B, C>(
    lp: Parser<'a, I, A>,
    parser: Parser<'a, I, B>,
    rp: Parser<'a, I, C>,
  ) -> Parser<'a, I, B>
  where
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a,
    C: Clone + Debug + 'a, {
    ParsersImpl::surround(lp, parser, rp)
  }

  pub fn chain_left1<'a, I, A, BOP>(p: Parser<'a, I, A>, op: Parser<'a, I, BOP>) -> Parser<'a, I, A>
  where
    BOP: Fn(A, A) -> A + Copy + 'a,
    A: Clone + Debug + 'a, {
    ParsersImpl::chain_left1(p, op)
  }
}

#[cfg(test)]
mod tests {
  use std::env;
  use std::iter::FromIterator;

  use crate::core::{ParserFunctor, ParserMonad, ParserRunner};

  use crate::extension::parser::{
    CollectParser, ConversionParser, DiscardParser, OffsetParser, OperatorParser, RepeatParser,
  };
  use crate::*;

  use super::prelude::*;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_chain_left1() {
    let input1 = "abc".chars().collect::<Vec<char>>();

    let p1 = tag("abc").collect().map(String::from_iter);
    let p2 = successful(|a: String, b: String| format!("{}{}", a, b));
    let p3 = chain_left1(p1, p2);
    let r = p3.parse_as_result(&input1).unwrap();

    println!("{:?}", r);
  }

  #[test]
  fn test_attempt() {
    init();
    {
      let input1 = b"b";
      let p: Parser<u8, &u8> = failed(ParseError::of_in_complete()).attempt().or(elm_ref(b'b'));

      let r = p.parse_as_result(input1);
      assert!(r.is_ok());
    }

    {
      let input1 = "abra cadabra!".chars().collect::<Vec<char>>();
      let p = (tag("abra") + elm_space() + tag("abra")).attempt() | (tag("abra") + elm_space() + tag("cadabra!"));
      let r = p.parse_as_result(&input1);
      println!("result = {:?}", r);
      assert!(r.is_ok());
    }
  }

  #[test]
  fn test_end() {
    init();
    let input1 = b"";
    let input2 = b"a";

    let p = end();

    let r = p.parse_as_result(input1);
    assert!(r.is_ok());

    let r = p.parse_as_result(input2);
    assert!(r.is_err());
  }

  #[test]
  fn test_unit() {
    init();
    let input = b"a";
    let p = unit();

    let r = p.parse_as_result(input);
    assert_eq!(r.unwrap(), ());
  }

  #[test]
  fn test_successful_in_closure() {
    init();
    let input = b"a";
    let p = successful_lazy(|| 'a');

    let r = p.parse_as_result(input).unwrap();
    assert_eq!(r, 'a');
  }

  #[test]
  fn test_elem() {
    init();
    let p = elm(b'a');

    let r = p.parse_as_result(b"a").unwrap();
    assert_eq!(r, b'a');
  }

  #[test]
  fn test_regex() {
    init();
    let input1 = "abc".chars().collect::<Vec<char>>();
    let input2 = "xbc".chars().collect::<Vec<char>>();
    let p = regex(r"a.*c$");

    let r = p.parse_as_result(&input1);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "abc");

    let r = p.parse_as_result(&input2);
    assert!(r.is_err());

    {
      let input3 = "12345 to".chars().collect::<Vec<_>>();
      let p = regex(r"\d+");
      let r = p.parse_as_result(&input3);
      println!("{:?}", r);
      assert!(r.is_ok());
      // assert_eq!(r.unwrap(), "abc");
    }
  }

  #[test]
  fn test_elm_of() {
    init();
    let patterns = b'a'..=b'f';
    let e = patterns.clone();
    let b = e.enumerate().into_iter().map(|e| e.1).collect::<Vec<_>>();
    let p = elm_of(&patterns);

    for index in 0..b.len() {
      let r = p.parse_as_result(&b[index..]);
      assert!(r.is_ok());
      assert_eq!(r.unwrap(), b[index]);
    }

    let r = p.parse_as_result(b"g");
    assert!(r.is_err());
  }

  #[test]
  fn test_none_of() {
    init();
    let patterns = b'a'..=b'f';
    let e = patterns.clone();
    let b = e.enumerate().into_iter().map(|e| e.1).collect::<Vec<_>>();
    let p = none_of(&patterns);

    for index in 0..b.len() {
      let r = p.parse_as_result(&b[index..]);
      assert!(r.is_err());
    }

    let r = p.parse_as_result(b"g");
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), b'g');
  }

  #[test]
  fn test_rep() {
    init();
    let p = elm_ref(b'a').repeat(..=3).collect();

    let r = p.parse_as_result(b"");
    assert!(r.is_ok());

    let r = p.parse_as_result(b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = p.parse_as_result(b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);

    let r = p.parse_as_result(b"aaa").unwrap();
    assert_eq!(r, vec![b'a', b'a', b'a']);
  }

  #[test]
  fn test_many_0() {
    init();
    let p = elm_ref(b'a').of_many0().collect();

    // let r = p.parse(b"").unwrap();
    // assert_eq!(r, vec![]);

    let r = p.parse_as_result(b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = p.parse_as_result(b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);
  }

  #[test]
  fn test_many_1() {
    init();
    let p = elm_ref(b'a').of_many1().collect();

    let r = p.parse_as_result(b"");
    assert!(r.is_err());

    let r = p.parse_as_result(b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = p.parse_as_result(b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);
  }

  #[test]
  fn test_many_n_m() {
    init();
    let p = elm_ref(b'a').of_many_n_m(1, 2).collect() + end();

    let r = p.parse_as_result(b"");
    assert!(r.is_err());

    let (a, _) = p.parse_as_result(b"a").unwrap();
    assert_eq!(a, vec![b'a']);

    let (a, _) = p.parse_as_result(b"aa").unwrap();
    assert_eq!(a, vec![b'a', b'a']);

    let r = p.parse_as_result(b"aaa");
    assert!(r.is_err());
  }

  #[test]
  fn test_count_sep() {
    init();
    let p1 = elm_ref(b'a');
    let p2 = elm_ref(b',');
    let p = p1.map(|e| *e).of_count_sep(3, p2);

    let r = p.parse_as_result(b"a,a,a").unwrap();
    assert_eq!(r, vec![b'a', b'a', b'a']);
  }

  #[test]
  fn test_seq() {
    init();
    let p = seq(b"abc");

    let r = p.parse_as_result(b"abc").unwrap();
    assert_eq!(r, b"abc");
  }

  #[test]
  fn test_tag() {
    init();
    let input = "abe".chars().collect::<Vec<char>>();
    let p = tag("abc").attempt() | tag("abe");

    let r = p.parse(&input);
    println!("{:?}", r);
    assert_eq!(r.to_result().unwrap(), "abe");
  }

  #[test]
  fn test_tag_no_case() {
    init();
    let input = "AbC".chars().collect::<Vec<char>>();
    let p = tag_no_case("abc");

    let r = p.parse_as_result(&input).unwrap();
    assert_eq!(r, "abc");
  }

  #[test]
  fn test_opt() {
    init();
    let p = seq(b"abc").opt();

    if let Some(b) = p.parse_as_result(b"abc").unwrap() {
      assert_eq!(b, b"abc");
    } else {
      panic!()
    }
  }

  #[test]
  fn test_not() {
    init();
    let p = seq(b"abc").not();

    let _ = p.parse_as_result(b"def").unwrap();
  }

  #[test]
  fn test_take_while0() {
    init();
    let p = take_while0(|c: &u8| c.is_ascii_digit()).convert(std::str::from_utf8);

    let result = p.parse_as_result(b"a123b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "123");

    let result = p.parse_as_result(b"def");
    assert!(result.is_ok());
  }

  #[test]
  fn test_take_while1() {
    init();
    let p = take_while1(|c: &u8| c.is_ascii_digit()).convert(std::str::from_utf8);

    let result = p.parse_as_result(b"a123b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "123");

    let result = p.parse_as_result(b"def");
    assert!(result.is_err());
  }

  #[test]
  fn test_take_while_n_m() {
    init();
    let p = take_while_n_m(1, 3, |c: &u8| c.is_ascii_digit()).convert(std::str::from_utf8);

    let result = p.parse_as_result(b"a1b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1");

    let result = p.parse_as_result(b"a12b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "12");

    let result = p.parse_as_result(b"a123b");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "123");

    let result = p.parse_as_result(b"a1234b");
    assert!(result.is_err());

    let result = p.parse_as_result(b"def");
    assert!(result.is_err());
  }

  #[test]
  fn test_take_till0() {
    init();
    let p = take_till0(|c| *c == b'c').convert(std::str::from_utf8);

    let result = p.parse_as_result(b"abcd");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "abc");

    let result = p.parse_as_result(b"def");
    assert!(result.is_ok());
  }

  #[test]
  fn test_take_till1() {
    init();
    let p = take_till1(|c| *c == b'c').convert(std::str::from_utf8);

    let result = p.parse_as_result(b"abcd");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "abc");

    let result = p.parse_as_result(b"def");
    assert!(result.is_err());
  }

  #[test]
  fn test_discard() {
    init();
    let p = seq(b"abc").discard();

    let result = p.parse_as_result(b"abc");
    assert!(result.is_ok());

    let result = p.parse_as_result(b"def");
    assert!(result.is_err());
  }

  #[test]
  fn test_and_then() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p = elm_ref(pv1) + elm_ref(pv2);

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    let (a, b) = result;
    assert_eq!(*a, pv1);
    assert_eq!(*b, pv2);
  }

  #[test]
  fn test_last_offset() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p1 = elm_ref(pv1);
    let p2 = elm_ref(pv2);
    let p = (p1 + p2).last_offset();

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    //  let (a, b) = result;
    //  assert_eq!(a, pv1);
    //  assert_eq!(b, pv2);
  }

  #[test]
  fn test_take() {
    init();
    let input1 = "abcd".chars().collect::<Vec<char>>();
    let p = ((elm_ref('a') + elm_ref('b')).flat_map(|e| skip(1).map(move |_| e)) + elm_any_ref() + end())
      .collect()
      .map(|chars| String::from_iter(chars));

    let result = p.parse_as_result(&input1).unwrap();
    log::debug!("result = {:?}", result);
  }

  #[test]
  fn test_or() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p = elm_ref(pv1) | elm_ref(pv2);

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(*result, pv1);

    let result = p.parse_as_result(b"ba").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(*result, pv2);
  }

  #[test]
  fn test_skip_left() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p = elm_ref(pv1) * elm_ref(pv2);

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(*result, pv2);
  }

  #[test]
  fn test_skip_right() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p1 = elm_ref(pv1);
    let p2 = elm_ref(pv2);
    let p = p1 - p2;

    let result = p.parse_as_result(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(*result, pv1);
  }

  #[test]
  fn test_example1() {
    init();
    let input1 = "abc".chars().collect::<Vec<char>>();
    let input2 = "abd".chars().collect::<Vec<char>>();

    let pa = elm_ref('a');
    let pb = elm_ref('b');
    let pc = elm_ref('c');
    let pd = elm_ref('d');
    let p = (pa + pb + (pc | pd)).collect().map(String::from_iter);

    let result = p.parse_as_result(&input1).unwrap();
    log::debug!("result = {}", result);
    assert_eq!(result, "abc");

    let result = p.parse_as_result(&input2).unwrap();
    log::debug!("result = {}", result);
    assert_eq!(result, "abd");
  }

  #[test]
  fn test_example2() {
    init();

    let input = "aname".chars().collect::<Vec<char>>();
    let p = (elm_ref('a') + tag("name")).collect().map(String::from_iter);

    let result = p.parse_as_result(&input).unwrap();
    // let s: String = result.iter().collect();
    log::debug!("result = {:?}", result);
    // assert_eq!(s, "aname");
  }

  #[test]
  fn test_filter() {
    init();
    {
      let input: Vec<char> = "abc def".chars().collect::<Vec<char>>();
      let p1 = tag("abc") * elm_ref(' ').map(|e| *e).of_many1() - tag("def");
      let p2 = p1.with_filter(|chars| chars.len() > 1);
      let result: Result<Vec<char>, ParseError<char>> = p2.parse_as_result(&input);
      assert!(result.is_err());
    }
    {
      let input: Vec<char> = "abc  def".chars().collect::<Vec<char>>();
      let p1 = tag("abc") * elm_ref(' ').map(|e| *e).of_many1() - tag("def");
      let p2 = p1.with_filter(|chars| chars.len() > 1);
      let result: Result<Vec<char>, ParseError<char>> = p2.parse_as_result(&input);
      assert!(result.is_ok());
    }
  }

  #[test]
  fn test_filter_not() {
    init();
    {
      let input: Vec<char> = "abc def".chars().collect::<Vec<char>>();
      let p1 = tag("abc") * elm_ref(' ').map(|e| *e).of_many1() - tag("def");
      let p2 = p1.with_filter_not(|chars| chars.len() > 1);
      let result: Result<Vec<char>, ParseError<char>> = p2.parse_as_result(&input);
      assert!(result.is_ok());
    }
    {
      let input: Vec<char> = "abc  def".chars().collect::<Vec<char>>();
      let p1 = tag("abc") * elm_ref(' ').map(|e| *e).of_many1() - tag("def");
      let p2 = p1.with_filter_not(|chars| chars.len() > 1);
      let result: Result<Vec<char>, ParseError<char>> = p2.parse_as_result(&input);
      assert!(result.is_err());
    }
  }
}
