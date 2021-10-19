use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::basic_parsers::ElementParsers;

use crate::combinators::BasicCombinators;
use crate::location::Location;
use crate::parse_error::ParseError;
use crate::parse_result::ParseResult;
use crate::parse_state::ParseState;
use crate::parser::Parser;
use crate::parsers::Parsers;
use crate::range::{Bound, RangeArgument};
use crate::simple_parser::SimpleParser;
use crate::Tuple;

pub struct SimpleParsers;

impl Parsers for SimpleParsers {
  type P<'p, I, A>
  where
    I: 'p,
  = SimpleParser<'p, I, A>;

  fn run<'a, I, A>(parser: Self::P<'a, I, A>, input: &'a [I]) -> Result<A, ParseError<'a, I>>
  where
    I: Clone, {
    let location = Location::new(input);
    let parse_state = ParseState::new(location);
    parser.run(Rc::new(parse_state)).extract()
  }

  fn successful<'a, I, A>(value: A) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: Clone + 'a, {
    SimpleParser::new(move |_| ParseResult::Success {
      get: value.clone(),
      length: 0,
    })
  }

  fn failed<'a, I, A>(parser_error: ParseError<'a, I>) -> Self::P<'a, I, A>
  where
    I: Clone + 'a, {
    SimpleParser::new(move |_| ParseResult::Failure {
      get: parser_error.clone(),
      is_committed: false,
    })
  }

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a,
    I: Clone + 'a,
    A: 'a,
    B: 'a, {
    SimpleParser::new(move |parse_state| match parser.run(parse_state.clone()) {
      ParseResult::Success { get: a, length: n } => f(a)
        .run(Rc::new(parse_state.with_add_offset(n)))
        .map_err_is_committed_fallback(n != 0)
        .with_add_length(n),
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a,
    I: Clone + 'a,
    A: 'a,
    B: 'a, {
    SimpleParser::new(move |parse_state| match parser.run(parse_state.clone()) {
      ParseResult::Success { get: a, length } => ParseResult::Success {
        get: f(a),
        length,
      },
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }
}

impl BasicCombinators for SimpleParsers {
  fn or<'a, I, A>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: 'a, {
    SimpleParser::new(move |s| match pa.run(Rc::clone(&s)) {
      ParseResult::Failure {
        is_committed: false, ..
      } => pb.run(s),
      r => r,
    })
  }

  fn and<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, Tuple<A, B>>
  where
    I: Clone + 'a,
    A: 'a,
    B: 'a,
    Self::P<'a, I, B>: 'a, {
    SimpleParser::new(move |parse_state| match pa.run(Rc::clone(&parse_state)) {
      ParseResult::Success { get: r1, length: n1 } => {
        let ps = Rc::new(parse_state.with_add_offset(n1));
        match pb.run(ps) {
          ParseResult::Success { get: r2, length: n2 } => ParseResult::successful(Tuple::new(r1, r2), n1 + n2),
          ParseResult::Failure { get, is_committed } => ParseResult::Failure { get, is_committed },
        }
      }
      ParseResult::Failure { get, is_committed } => ParseResult::Failure { get, is_committed },
    })
  }

  fn repeat_with_separator<'a, I, A, B, R>(
    pa: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    I: Clone + 'a,
    R: RangeArgument<usize> + Debug + 'a,
    A: 'a,
    B: 'a, {
    SimpleParser::new(move |s| {
      let mut ps = Rc::clone(&s);
      let mut pos = s.offset();
      let mut items = vec![];

      if let ParseResult::Success { get, length } = pa.run(Rc::clone(&ps)) {
        ps = Rc::new(ps.with_add_offset(length));
        items.push(get);
        pos += length;
        loop {
          match range.end() {
            Bound::Included(&max_count) => {
              if items.len() >= max_count {
                break;
              }
            }
            Bound::Excluded(&max_count) => {
              if items.len() + 1 >= max_count {
                break;
              }
            }
            _ => (),
          }

          if let Some(sep) = &separator {
            if let ParseResult::Success { length, .. } = sep.run(Rc::clone(&ps)) {
              ps = Rc::new(ps.with_add_offset(length));
              pos += length;
            } else {
              break;
            }
          }
          if let ParseResult::Success { get, length } = pa.run(Rc::clone(&ps)) {
            ps = Rc::new(ps.with_add_offset(length));
            items.push(get);
            pos += length;
          } else {
            break;
          }
        }
      }

      if let Bound::Included(&min_count) = range.start() {
        if items.len() < min_count {
          return ParseResult::failed_with_un_commit(s.location.clone().with_add_offset(pos).to_error(format!(
            "expect repeat at least {} times, found {} times",
            min_count,
            items.len()
          )));
        }
      }
      let len = items.len();
      ParseResult::successful(items,  len)
    })
  }
}

impl ElementParsers for SimpleParsers {
  fn eof<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Clone + Debug + Display + 'a, {
    SimpleParser::new(move |s| {
      let input = s.input();
      log::debug!("input = {:?}", input);
      if let Some(actual) = input.get(0) {
        log::debug!("actual = {}", actual);
        let msg = format!("expect end of input, found: {}", actual);
        ParseResult::failed_with_un_commit(s.location.clone().with_add_offset(1).to_error(msg))
      } else {
        ParseResult::successful((), 0)
      }
    })
  }

  fn empty<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Clone + 'a, {
    Self::unit()
  }

  fn elem_in<'a, I, F>(f: F) -> Self::P<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Clone + PartialEq + 'a, {
    SimpleParser::new(move |s| {
      let offset = s.offset();
      let msg = format!("offset: {}", i);
      let ss = s.input();
      if let Some(actual) = ss.get(0) {
        if f(actual) {
          return ParseResult::successful(actual.clone(), 1);
        }
      }
      ParseResult::failed_with_un_commit(s.location.clone().with_add_offset(1).to_error(msg))
    })
  }

  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, &'a [I]>
  where
    I: Clone + PartialEq + Debug + 'a,
    'b: 'a, {
    SimpleParser::new(move |s| {
      let offset = s.offset();
      let input = s.input();
      let mut index = 0;
      loop {
        if index == tag.len() {
          return ParseResult::successful(tag.clone(), index);
        }
        let pos = offset + index;
        if let Some(str) = input.get(pos) {
          if tag[index] != *str {
            return ParseResult::failed_with_un_commit(
              s.location
                .clone()
                .with_add_offset(index)
                .to_error(format!("seq {:?} expect: {:?}, found: {:?}", tag, tag[index], str)),
            );
          }
        } else {
          return ParseResult::failed_with_un_commit(
            s.location.clone().with_add_offset(index).to_error("".to_string()),
          );
        }
        index += 1;
      }
    })
  }

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a, {
    SimpleParser::new(move |s| {
      let offset = s.offset();
      let input = s.input();
      let mut index = 0;
      for c in tag.chars() {
        let pos = offset + index;
        if let Some(actual) = input.get(pos) {
          if c != *actual {
            return ParseResult::failed_with_un_commit(
              s.location
                .clone()
                .with_add_offset(pos)
                .to_error(format!("tag {:?} expect: {:?}, found: {}", tag, c, actual)),
            );
          }
        } else {
          return ParseResult::failed_with_un_commit(s.location.clone().with_add_offset(pos).to_error("".to_string()));
        }
        index += 1;
      }
      ParseResult::successful(tag.clone(), index)
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::combinator::*;
  use std::env;

  use crate::Tuple;

  use super::*;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_eof() {
    init();
    let p = || SimpleParsers::eof();
    let r = SimpleParsers::run(p(), b"");
    assert!(r.is_ok());
    let r = SimpleParsers::run(p(), b"a");
    assert!(r.is_err());
  }

  #[test]
  fn test_unit() {
    init();
    let p1 = SimpleParsers::unit();
    let p = p1.map(|_| 'a');
    let r = SimpleParsers::run(p, b"a").unwrap();
    assert_eq!(r, 'a');
  }

  #[test]
  fn test_successful() {
    init();
    let p = SimpleParsers::successful('a');
    let r = SimpleParsers::run(p, b"a").unwrap();
    assert_eq!(r, 'a');
  }

  #[test]
  fn test_elem() {
    init();
    let p = SimpleParsers::elem(b'a');
    let r = SimpleParsers::run(p, b"a").unwrap();
    assert_eq!(r, b'a');
  }

  #[test]
  fn test_repeat() {
    init();
    let p1 = || SimpleParsers::elem(b'a');
    let p = || p1().repeat(1..3);

    let r = SimpleParsers::run(p(), b"");
    assert!(r.is_err());

    let r = SimpleParsers::run(p(), b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = SimpleParsers::run(p(), b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);

    let r = SimpleParsers::run(p(), b"aaa").unwrap();
    assert_eq!(r, vec![b'a', b'a', b'a']);
  }

  #[test]
  fn test_many() {
    init();
    let p1 = || SimpleParsers::elem(b'a');
    let p = || SimpleParsers::many(p1());

    let r = SimpleParsers::run(p(), b"").unwrap();
    assert_eq!(r, vec![]);

    let r = SimpleParsers::run(p(), b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = SimpleParsers::run(p(), b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);
  }

  #[test]
  fn test_many1() {
    init();
    let p1 = || SimpleParsers::elem(b'a');
    let p = || SimpleParsers::many1(p1());

    let r = SimpleParsers::run(p(), b"");
    assert!(r.is_err());

    let r = SimpleParsers::run(p(), b"a").unwrap();
    assert_eq!(r, vec![b'a']);

    let r = SimpleParsers::run(p(), b"aa").unwrap();
    assert_eq!(r, vec![b'a', b'a']);
  }

  #[test]
  fn test_many_n_m() {
    init();
    let p1 = || SimpleParsers::elem(b'a');
    let p2 = || SimpleParsers::many_n_m(p1(), 1, 2);
    let p = || p2().and(SimpleParsers::eof());

    let r = SimpleParsers::run(p(), b"");
    assert!(r.is_err());

    let r = SimpleParsers::run(p(), b"a").unwrap();
    assert_eq!(r.a, vec![b'a']);

    let r = SimpleParsers::run(p(), b"aa").unwrap();
    assert_eq!(r.a, vec![b'a', b'a']);

    let r = SimpleParsers::run(p(), b"aaa");
    assert!(r.is_err());
  }

  #[test]
  fn test_list() {
    init();
    let p1 = || SimpleParsers::elem(b'a');
    let sep = || SimpleParsers::elem(b',');
    let p = SimpleParsers::list(p1(), sep());
    let r = SimpleParsers::run(p, b"a,a,a").unwrap();
    assert_eq!(r, vec![b'a', b'a', b'a']);
  }

  #[test]
  fn test_seq() {
    init();
    let p = SimpleParsers::seq(b"abc");
    let r = SimpleParsers::run(p, b"abc").unwrap();
    assert_eq!(r, b"abc");
  }

  #[test]
  fn test_tag() {
    init();
    let p = SimpleParsers::tag("abc");
    let input = "abc".chars().collect::<Vec<char>>();
    let r = SimpleParsers::run(p, &input).unwrap();
    assert_eq!(r, "abc");
  }

  #[test]
  fn test_opt() {
    init();
    let p1 = SimpleParsers::seq(b"abc");
    let p = p1.opt();
    //    let p = SimpleParsers::opt(p1);
    if let Some(b) = SimpleParsers::run(p, b"abc").unwrap() {
      assert_eq!(b, b"abc");
    } else {
      panic!()
    }
  }

  #[test]
  fn test_and() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p1 = SimpleParsers::elem(pv1);
    let p2 = SimpleParsers::elem(pv2);
    let p = p1.and(p2);
    // let p = SimpleParsers::and(p1, p2);
    let result = SimpleParsers::run(p, b"ab").unwrap();
    log::debug!("result = {:?}", result);
    let Tuple { a, b } = result;
    assert_eq!(a, pv1);
    assert_eq!(b, pv2);
  }

  #[test]
  fn test_or() {
    init();
    let pv1 = b'a';
    let pv2 = b'b';
    let p1 = SimpleParsers::elem(pv1);
    let p2 = SimpleParsers::elem(pv2);
    //    let p = SimpleParsers::or(p1, p2);
    let p = p1.or(p2);
    let result = SimpleParsers::run(p, b"ab").unwrap();
    log::debug!("result = {:?}", result);
    assert_eq!(result, pv1);
  }
}
