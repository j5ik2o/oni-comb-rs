use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::core::BasicParsers;
use crate::core::ParseError;
use crate::core::ParseResult;
use crate::core::ParseState;
use crate::core::ParserRunner;
use crate::core::{CoreParsers, Element};
use crate::extension::{
  BasicCombinators, BasicRepeatParsers, ConversionCombinators, OffsetCombinators, RepeatCombinators, SkipCombinators,
};
use crate::utils::Set;
use crate::utils::{Bound, RangeArgument};
use crate::Parser;

pub(crate) struct ParsersImpl;

impl CoreParsers for ParsersImpl {
  type P<'p, I, A>
  where
    I: 'p,
  = Parser<'p, I, A>;

  fn parse<'a, 'b, I, A>(parser: &Self::P<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
  where
    'b: 'a, {
    let parse_state = ParseState::new(input, 0);
    parser.run(Rc::new(parse_state)).extract()
  }

  fn successful<'a, I, A, F>(value: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a, {
    Parser::new(move |_| ParseResult::Success {
      get: value(),
      length: 0,
    })
  }

  fn failed<'a, I, A>(parser_error: ParseError<'a, I>) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    Parser::new(move |_| ParseResult::Failure {
      get: parser_error.clone(),
      is_committed: false,
    })
  }

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state.clone()) {
      ParseResult::Success { get: a, length: n } => f(a)
        .run(Rc::new(parse_state.add_offset(n)))
        .map_err_is_committed_fallback(n != 0)
        .with_add_length(n),
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state.clone()) {
      ParseResult::Success { get: a, length } => ParseResult::Success { get: f(a), length },
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }
}

impl ConversionCombinators for ParsersImpl {
  fn convert<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Result<B, E> + 'a,
    E: Debug,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state.clone()) {
      ParseResult::Success { get: a, length } => match f(a) {
        Ok(get) => ParseResult::Success { get: get, length },
        Err(err) => {
          let ps = parse_state.add_offset(0);
          let msg = format!("Conversion error: {:?}", err);
          let parser_error = ParseError::of_mismatch(ps.input(), ps.last_offset().unwrap_or(0), msg);
          ParseResult::failed_with_un_commit(parser_error)
        }
      },
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }
}

impl BasicCombinators for ParsersImpl {
  fn not<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, bool>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(Rc::clone(&parse_state)) {
      ParseResult::Success { .. } => {
        let ps = parse_state.add_offset(0);
        let parser_error = ParseError::of_mismatch(
          ps.input(),
          ps.last_offset().unwrap_or(0),
          "not predicate failed".to_string(),
        );
        ParseResult::failed_with_un_commit(parser_error)
      }
      ParseResult::Failure { .. } => ParseResult::successful(true, 0),
    })
  }

  fn or<'a, I, A>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    A: 'a, {
    Parser::new(move |parse_state| {
      let result = pa.run(Rc::clone(&parse_state));
      if let Some(is_committed) = result.is_committed() {
        if !is_committed {
          return pb.run(parse_state);
        }
      }
      result
    })
  }

  fn and_then<'a, I, A, B>(pa: Self::P<'a, I, A>, pb: Self::P<'a, I, B>) -> Self::P<'a, I, (A, B)>
  where
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match pa.run(Rc::clone(&parse_state)) {
      ParseResult::Success { get: r1, length: n1 } => {
        let ps = Rc::new(parse_state.add_offset(n1));
        match pb.run(ps) {
          ParseResult::Success { get: r2, length: n2 } => ParseResult::successful((r1, r2), n1 + n2),
          ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
        }
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn collect<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, &'a [I]>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(Rc::clone(&parse_state)) {
      ParseResult::Success { length, .. } => {
        let slice = parse_state.slice(length);
        ParseResult::successful(slice, length)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn discard<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, ()>
  where
    A: Debug + 'a, {
    Parser::new(move |parse_state| match parser.run(Rc::clone(&parse_state)) {
      ParseResult::Success { length, .. } => ParseResult::successful((), length),
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> Self::P<'a, I, A> + 'a,
    A: Debug + 'a, {
    Parser::new(move |parse_state| {
      let parser = f();
      parser.run(Rc::clone(&parse_state))
    })
  }
}

impl SkipCombinators for ParsersImpl {}

impl OffsetCombinators for ParsersImpl {
  fn last_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(Rc::clone(&parse_state)) {
      ParseResult::Success { length, .. } => {
        let ps = parse_state.add_offset(length);
        ParseResult::successful(ps.last_offset().unwrap_or(0), length)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn next_offset<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, usize>
  where
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(Rc::clone(&parse_state)) {
      ParseResult::Success { length, .. } => {
        let ps = parse_state.add_offset(length);
        ParseResult::successful(ps.next_offset(), length)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }
}

impl RepeatCombinators for ParsersImpl {
  fn rep_sep<'a, I, A, B, R>(
    pa: Self::P<'a, I, A>,
    range: R,
    separator: Option<Self::P<'a, I, B>>,
  ) -> Self::P<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| {
      let mut ps = Rc::clone(&parse_state);
      let mut all_length = 0;
      let mut items = vec![];

      if let ParseResult::Success { get, length } = pa.run(Rc::clone(&ps)) {
        ps = Rc::new(ps.add_offset(length));
        items.push(get);
        all_length += length;
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
              ps = Rc::new(ps.add_offset(length));
              all_length += length;
            } else {
              break;
            }
          }
          if let ParseResult::Success { get, length } = pa.run(Rc::clone(&ps)) {
            ps = Rc::new(ps.add_offset(length));
            items.push(get);
            all_length += length;
          } else {
            break;
          }
        }
      }

      if let Bound::Included(&min_count) = range.start() {
        if items.len() < min_count {
          let ps = ps.add_offset(all_length);
          let pe = ParseError::of_mismatch(
            ps.input(),
            ps.last_offset().unwrap_or(0),
            format!(
              "expect repeat at least {} times, found {} times",
              min_count,
              items.len()
            ),
          );
          return ParseResult::failed_with_un_commit(pe);
        }
      }
      ParseResult::successful(items, all_length)
    })
  }
}

impl BasicParsers for ParsersImpl {
  fn end<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Debug + Display + 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(actual) = input.get(0) {
        let msg = format!("expect end of input, found: {}", actual);
        let ps = parse_state.add_offset(1);
        let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
        ParseResult::failed_with_un_commit(pe)
      } else {
        ParseResult::successful((), 0)
      }
    })
  }

  fn empty<'a, I>() -> Self::P<'a, I, ()> {
    Self::unit()
  }

  fn elm_pred<'a, I, F>(f: F) -> Self::P<'a, I, I>
  where
    F: Fn(&I) -> bool + 'a,
    I: Element + Clone + PartialEq + 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(actual) = input.get(0) {
        if f(actual) {
          return ParseResult::successful(actual.clone(), 1);
        }
      }
      let offset = parse_state.next_offset();
      let msg = format!("offset: {}", offset);
      let ps = parse_state.add_offset(1);
      let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
      ParseResult::failed_with_un_commit(pe)
    })
  }

  fn elm_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_space)
  }

  fn elm_multi_space<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_multi_space)
  }

  fn elm_alpha<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_alphabetic)
  }

  fn elm_alpha_num<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_alphanumeric)
  }

  fn elm_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_digit)
  }

  fn elm_hex_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_hex_digit)
  }

  fn elm_oct_digit<'a, I>() -> Self::P<'a, I, I>
  where
    I: Element + Clone + PartialEq + 'a, {
    Self::elm_pred(Element::is_ascii_oct_digit)
  }

  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, &'a [I]>
  where
    I: PartialEq + Debug + 'a,
    'b: 'a, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      let mut index = 0;
      loop {
        if index == tag.len() {
          return ParseResult::successful(tag.clone(), index);
        }
        if let Some(str) = input.get(index) {
          if tag[index] != *str {
            let msg = format!("seq {:?} expect: {:?}, found: {:?}", tag, tag[index], str);
            let ps = parse_state.add_offset(index);
            let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
            return ParseResult::failed_with_un_commit(pe);
          }
        } else {
          return ParseResult::failed_with_un_commit(ParseError::of_in_complete());
        }
        index += 1;
      }
    })
  }

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a, {
    Parser::new(move |parse_state| {
      let input: &[char] = parse_state.input();
      let mut index = 0;
      for c in tag.chars() {
        if let Some(&actual) = input.get(index) {
          if c != actual {
            let msg = format!("tag {:?} expect: {:?}, found: {}", tag, c, actual);
            let ps = parse_state.add_offset(index);
            let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
            return ParseResult::failed_with_un_commit(pe);
          }
        } else {
          return ParseResult::failed_with_un_commit(ParseError::of_in_complete());
        }
        index += 1;
      }
      ParseResult::successful(tag.clone(), index)
    })
  }

  fn tag_no_case<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a, {
    Parser::new(move |parse_state: Rc<ParseState<char>>| {
      let input = parse_state.clone().input();
      let mut index = 0;
      for c in tag.chars() {
        if let Some(&actual) = input.get(index) {
          if c.to_ascii_lowercase() != actual.to_ascii_lowercase() {
            let msg = format!("tag {:?} expect: {:?}, found: {}", tag, c, actual);
            let ps = parse_state.add_offset(index);
            let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
            return ParseResult::failed_with_un_commit(pe);
          }
        } else {
          return ParseResult::failed_with_un_commit(ParseError::of_in_complete());
        }
        index += 1;
      }
      ParseResult::successful(tag.clone(), index)
    })
  }

  fn take<'a, I>(n: usize) -> Self::P<'a, I, &'a [I]> {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if input.len() >= n {
        ParseResult::successful(parse_state.slice(n), n)
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn skip<'a, I>(n: usize) -> Self::P<'a, I, ()> {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if input.len() >= n {
        ParseResult::successful((), n)
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn one_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, I>
  where
    I: Clone + PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if set.contains(s) {
          ParseResult::successful(s.clone(), 1)
        } else {
          let msg = format!("expect one of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }

  fn none_of<'a, I, S>(set: &'a S) -> Self::P<'a, I, I>
  where
    I: Clone + PartialEq + Display + Debug + 'a,
    S: Set<I> + ?Sized, {
    Parser::new(move |parse_state| {
      let input = parse_state.input();
      if let Some(s) = input.get(0) {
        if !set.contains(s) {
          ParseResult::successful(s.clone(), 1)
        } else {
          let msg = format!("expect none of: {}, found: {}", set.to_str(), s);
          let ps = parse_state.add_offset(1);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), msg);
          ParseResult::failed_with_un_commit(pe)
        }
      } else {
        ParseResult::failed_with_un_commit(ParseError::of_in_complete())
      }
    })
  }
}

impl BasicRepeatParsers for ParsersImpl {
  fn any_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_0(Self::elm_any())
  }

  fn any_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_1(Self::elm_any())
  }

  fn any_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_n_m(Self::elm_any(), n, m)
  }

  fn any_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::count(Self::elm_any(), n)
  }

  fn space_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_0(Self::elm_space())
  }

  fn space_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_1(Self::elm_space())
  }

  fn space_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_n_m(Self::elm_space(), n, m)
  }

  fn space_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::count(Self::elm_space(), n)
  }

  fn multi_space_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_0(Self::elm_multi_space())
  }

  fn multi_space_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_1(Self::elm_multi_space())
  }

  fn multi_space_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_n_m(Self::elm_multi_space(), n, m)
  }

  fn multi_space_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::count(Self::elm_multi_space(), n)
  }

  fn alphabet_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_0(Self::elm_pred(Element::is_ascii_alphabetic))
  }

  fn alphabet_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_1(Self::elm_alpha())
  }

  fn alphabet_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_n_m(Self::elm_alpha(), n, m)
  }

  fn alphabet_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::count(Self::elm_alpha(), n)
  }

  fn digit_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_0(Self::elm_digit())
  }

  fn digit_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_1(Self::elm_digit())
  }

  fn digit_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_n_m(Self::elm_digit(), n, m)
  }

  fn digit_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::count(Self::elm_digit(), n)
  }

  fn hex_digit_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_0(Self::elm_hex_digit())
  }

  fn hex_digit_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_1(Self::elm_hex_digit())
  }

  fn hex_digit_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_n_m(Self::elm_hex_digit(), n, m)
  }

  fn hex_digit_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::count(Self::elm_hex_digit(), n)
  }

  fn oct_digit_seq_0<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_0(Self::elm_oct_digit())
  }

  fn oct_digit_seq_1<'a, I>() -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_1(Self::elm_oct_digit())
  }

  fn oct_digit_seq_n_m<'a, I>(n: usize, m: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::many_n_m(Self::elm_oct_digit(), n, m)
  }

  fn oct_digit_seq_of_n<'a, I>(n: usize) -> Self::P<'a, I, Vec<I>>
  where
    I: Element + Clone + PartialEq + Debug + 'a, {
    Self::count(Self::elm_oct_digit(), n)
  }
}
