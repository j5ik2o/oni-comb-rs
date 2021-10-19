#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]

use std::fmt::{Debug, Display};

use crate::core::BasicParsers;
use crate::core::{CoreParsers, Element};
use crate::extension::{BasicCombinators, BasicRepeatParsers};
use crate::internal::ParsersImpl;
pub use crate::parser::Parser;
use crate::utils::Set;

pub mod core;
pub mod extension;
mod internal;
mod parser;
pub mod utils;

// https://github.com/com-lihaoyi/fastparse
// https://github.com/fpinscala/fpinscala/blob/first-edition/answers/src/main/scala/fpinscala/parsing
// https://github.com/Geal/nom
// https://hazm.at/mox/lang/rust/nom/index.html
// https://github.com/J-F-Liu/pom

// pub fn parse<'a, 'b, I, A>(parser: &Parser<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
// where
//   'b: 'a, {
//   ParsersImpl::parse(parser, input)
// }

pub fn lazy<'a, I, A, F>(f: F) -> Parser<'a, I, A>
where
  F: Fn() -> Parser<'a, I, A> + 'a,
  A: Debug + 'a, {
  ParsersImpl::lazy(f)
}

pub fn unit<'a, I>() -> Parser<'a, I, ()> {
  ParsersImpl::unit()
}

pub fn successful<'a, I, A, F>(value: F) -> Parser<'a, I, A>
where
  I: 'a,
  F: Fn() -> A + 'a,
  A: 'a, {
  ParsersImpl::successful(value)
}

pub fn end<'a, I>() -> Parser<'a, I, ()>
where
  I: Debug + Display + 'a, {
  ParsersImpl::end()
}

pub fn empty<'a, I>() -> Parser<'a, I, ()> {
  ParsersImpl::empty()
}

pub fn elm_any<'a, I>() -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a, {
  ParsersImpl::elm_any()
}

pub fn elm<'a, I>(c: I) -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a, {
  ParsersImpl::elm(c)
}

pub fn elm_pred<'a, I, F>(f: F) -> Parser<'a, I, I>
where
  F: Fn(&I) -> bool + 'a,
  I: Element + Clone + PartialEq + 'a, {
  ParsersImpl::elm_pred(f)
}

pub fn elm_space<'a, I>() -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a, {
  ParsersImpl::elm_space()
}

pub fn elm_multi_space<'a, I>() -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a, {
  ParsersImpl::elm_multi_space()
}

pub fn elm_alpha<'a, I>() -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a, {
  ParsersImpl::elm_alpha()
}

pub fn elm_alpha_num<'a, I>() -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a, {
  ParsersImpl::elm_alpha_num()
}

pub fn elm_digit<'a, I>() -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a, {
  ParsersImpl::elm_digit()
}

pub fn elm_hex_digit<'a, I>() -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a, {
  ParsersImpl::elm_hex_digit()
}

pub fn elm_oct_digit<'a, I>() -> Parser<'a, I, I>
where
  I: Element + Clone + PartialEq + 'a, {
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

pub fn skip<'a, I>(n: usize) -> Parser<'a, I, ()> {
  ParsersImpl::skip(n)
}

pub fn one_of<'a, I, S>(set: &'a S) -> Parser<'a, I, I>
where
  I: Clone + PartialEq + Display + Debug + 'a,
  S: Set<I> + ?Sized, {
  ParsersImpl::one_of(set)
}

pub fn none_of<'a, I, S>(set: &'a S) -> Parser<'a, I, I>
where
  I: Clone + PartialEq + Display + Debug + 'a,
  S: Set<I> + ?Sized, {
  ParsersImpl::none_of(set)
}

pub fn space_seq_0<'a, I>() -> Parser<'a, I, Vec<I>>
where
  I: Element + Clone + PartialEq + Debug + 'a, {
  ParsersImpl::space_seq_0()
}

pub fn space_seq_1<'a, I>() -> Parser<'a, I, Vec<I>>
where
  I: Element + Clone + PartialEq + Debug + 'a, {
  ParsersImpl::space_seq_1()
}

pub fn space_seq_n_m<'a, I>(n: usize, m: usize) -> Parser<'a, I, Vec<I>>
where
  I: Element + Clone + PartialEq + Debug + 'a, {
  ParsersImpl::space_seq_n_m(n, m)
}

pub fn space_seq_of_n<'a, I>(n: usize) -> Parser<'a, I, Vec<I>>
where
  I: Element + Clone + PartialEq + Debug + 'a, {
  ParsersImpl::space_seq_of_n(n)
}

#[cfg(test)]
mod tests {
  use std::env;
  use std::iter::FromIterator;

  use crate::core::{ParserFunctor, ParserMonad, ParserRunner};
  use crate::extension::*;
  use crate::*;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_space_0() {
    init();

    let p = space_seq_0();
    let r: usize = p.parse(b"").map(|e| e.len()).unwrap();
    assert_eq!(r, 0);

    let p = space_seq_0();
    let r: usize = p.parse(b"     ").map(|e| e.len()).unwrap();
    assert_eq!(r, 5);
  }

  #[test]
  fn test_space_1() {
    init();

    let p = space_seq_1();
    let r = p.parse(b"").map(|e| e.len());
    assert!(r.is_err());

    let p = space_seq_1();
    let r: usize = p.parse(b"     ").map(|e| e.len()).unwrap();
    assert_eq!(r, 5);
  }

  #[test]
  fn test_end() {
    init();
    let input1 = b"";
    let input2 = b"a";

    let p = end();

    let r = p.parse(input1);
    assert!(r.is_ok());

    let r = p.parse(input2);
    assert!(r.is_err());
  }

  #[test]
  fn test_unit() {
    init();
    let input = b"a";
    let p = unit();

    let r = p.parse(input);
    assert_eq!(r.unwrap(), ());
  }

  #[test]
  fn test_successful() {
    init();
    let input = b"a";
    let p = successful(|| 'a');

    let r = p.parse(input).unwrap();
    assert_eq!(r, 'a');
  }

  #[test]
  fn test_elem() {
    init();
    let p = elm(b'a');

    let r = p.parse(b"a").unwrap();
    assert_eq!(r, b'a');
  }

  #[test]
  fn test_one_of() {
    init();
    let patterns = b'a'..=b'f';
    let e = patterns.clone();
    let b = e.enumerate().into_iter().map(|e| e.1).collect::<Vec<_>>();
    let p = one_of(&patterns);

    for index in 0..b.len() {
      let r = p.parse(&b[index..]);
      assert!(r.is_ok());
      assert_eq!(r.unwrap(), b[index]);
    }

    let r = p.parse(b"g");
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
      let r = p.parse(&b[index..]);
      assert!(r.is_err());
    }

    let r = p.parse(b"g");
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), b'g');
  }

  #[test]
  fn test_rep() {
    init();
    let p = elm(b'a').repeat(..=3);

    let r = p.parse(b"");
    assert!(r.is_ok());

    let r = p.parse(b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = p.parse(b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);

    let r = p.parse(b"aaa").unwrap();
    assert_eq!(r, vec![b'a', b'a', b'a']);
  }

  #[test]
  fn test_many_0() {
    init();
    let p = elm(b'a').many_0();

    let r = p.parse(b"").unwrap();
    assert_eq!(r, vec![]);

    let r = p.parse(b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = p.parse(b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);
  }

  #[test]
  fn test_many_1() {
    init();
    let p = elm(b'a').many_1();

    let r = p.parse(b"");
    assert!(r.is_err());

    let r = p.parse(b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = p.parse(b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);
  }

  #[test]
  fn test_many_n_m() {
    init();
    let p = elm(b'a').many_n_m(1, 2) + end();

    let r = p.parse(b"");
    assert!(r.is_err());

    let (a, _) = p.parse(b"a").unwrap();
    assert_eq!(a, vec![b'a']);

    let (a, _) = p.parse(b"aa").unwrap();
    assert_eq!(a, vec![b'a', b'a']);

    let r = p.parse(b"aaa");
    assert!(r.is_err());
  }

  #[test]
  fn test_count_sep() {
    init();
    let p1 = elm(b'a');
    let p2 = elm(b',');
    let p = p1.count_sep(3, p2);

    let r = p.parse(b"a,a,a").unwrap();
    assert_eq!(r, vec![b'a', b'a', b'a']);
  }

  #[test]
  fn test_seq() {
    init();
    let p = seq(b"abc");

    let r = p.parse(b"abc").unwrap();
    assert_eq!(r, b"abc");
  }

  #[test]
  fn test_tag() {
    init();
    let input = "abc".chars().collect::<Vec<char>>();
    let p = tag("abc");

    let r = p.parse(&input).unwrap();
    assert_eq!(r, "abc");
  }

  #[test]
  fn test_tag_no_case() {
    init();
    let input = "AbC".chars().collect::<Vec<char>>();
    let p = tag_no_case("abc");

    let r = p.parse(&input).unwrap();
    assert_eq!(r, "abc");
  }

  #[test]
  fn test_opt() {
    init();
    let p = seq(b"abc").opt();

    if let Some(b) = p.parse(b"abc").unwrap() {
      assert_eq!(b, b"abc");
    } else {
      panic!()
    }
  }

  #[test]
  fn test_not() {
    init();
    let p = !seq(b"abc");

    let b = p.parse(b"def").unwrap();
    assert!(b);
  }

  #[test]
  fn test_discard() {
    init();
    let p = seq(b"abc").discard();

    let result = p.parse(b"abc");
    assert!(result.is_ok());

    let result = p.parse(b"def");
    assert!(result.is_err());
  }

  #[test]
  fn test_and_then() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p = elm(pv1) + elm(pv2);

    let result = p.parse(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    let (a, b) = result;
    assert_eq!(a, pv1);
    assert_eq!(b, pv2);
  }

  #[test]
  fn test_last_offset() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p1 = elm(pv1);
    let p2 = elm(pv2);
    let p = (p1 + p2).last_offset();

    let result = p.parse(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    //  let (a, b) = result;
    //  assert_eq!(a, pv1);
    //  assert_eq!(b, pv2);
  }

  #[test]
  fn test_take() {
    init();
    let input1 = "abcd".chars().collect::<Vec<char>>();
    let pv1 = 'a';
    let pv2 = 'b';
    let p = ((elm(pv1) + elm(pv2)).flat_map(|e| skip(1).map(move |_| e)) + elm_any() + end())
      .collect()
      .map(|e| e.iter().collect::<String>());

    let result = p.parse(&input1).unwrap();
    log::debug!("result = {:?}", result);
  }

  #[test]
  fn test_or() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p = elm(pv1) | elm(pv2);

    let result = p.parse(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(result, pv1);

    let result = p.parse(b"ba").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(result, pv2);
  }

  #[test]
  fn test_skip_left() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p = elm(pv1) * elm(pv2);

    let result = p.parse(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(result, pv2);
  }

  #[test]
  fn test_skip_right() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p1 = elm(pv1);
    let p2 = elm(pv2);
    let p = p1 - p2;

    let result = p.parse(b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(result, pv1);
  }

  #[test]
  fn test_example1() {
    init();
    let input1 = "abc".chars().collect::<Vec<char>>();
    let input2 = "abd".chars().collect::<Vec<char>>();

    let pa = elm('a');
    let pb = elm('b');
    let pc = elm('c');
    let pd = elm('d');
    let p = (pa + pb + (pc | pd)).collect().map(String::from_iter);

    let result = p.parse(&input1).unwrap();
    log::debug!("result = {}", result);
    assert_eq!(result, "abc");

    let result = p.parse(&input2).unwrap();
    log::debug!("result = {}", result);
    assert_eq!(result, "abd");
  }

  #[test]
  fn test_example2() {
    init();

    let input = "aname".chars().collect::<Vec<char>>();
    let p = (elm('a') + tag("name")).collect().map(String::from_iter);

    let result = p.parse(&input).unwrap();
    // let s: String = result.iter().collect();
    log::debug!("result = {:?}", result);
    // assert_eq!(s, "aname");
  }
}
