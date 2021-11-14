use crate::core::{CommittedStatus, ParseError, ParseState, ParsedResult, Parser, ParserRunner, Parsers};
use crate::internal::ParsersImpl;

mod cache_parsers_impl;
mod collect_parsers_impl;
mod conversion_parsers_impl;
mod discard_parsers_impl;
mod element_parsers_impl;
mod elements_parsers_impl;
mod lazy_parsers_impl;
mod logging_parsers_impl;
mod offset_parsers_impl;
mod operator_parsers_impl;
mod peek_parsers_impl;
mod primitive_parsers_impl;
mod repeat_parsers_impl;
mod skip_parser_impl;
mod taken_parsers_impl;

impl Parsers for ParsersImpl {
  type P<'p, I, A>
  where
    I: 'p,
    A: 'p,
  = Parser<'p, I, A>;

  fn parse<'a, 'b, I, A>(parser: &Self::P<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
  where
    A: 'a,
    'b: 'a, {
    let parse_state = ParseState::new(input, 0);
    parser.run(&parse_state).to_result()
  }

  fn successful<'a, I, A>(value: A) -> Self::P<'a, I, A>
  where
    A: Clone + 'a, {
    Parser::new(move |_| ParsedResult::successful(value.clone(), 0))
  }

  fn successful_lazy<'a, I, A, F>(value: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a, {
    Parser::new(move |_| ParsedResult::successful(value(), 0))
  }

  fn failed<'a, I, A>(value: ParseError<'a, I>, committed: CommittedStatus) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    Parser::new(move |_| ParsedResult::failed(value.clone(), committed.clone()))
  }

  fn failed_lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a, {
    Parser::new(move |_| {
      let (pe, committed) = f();
      ParsedResult::failed(pe, committed)
    })
  }

  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a,
    I: 'a,
    A: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParsedResult::Success { value, length } => {
        if f(&value) {
          ParsedResult::successful(value, length)
        } else {
          let input = parse_state.input();
          let offset = parse_state.last_offset().unwrap_or(0);
          let msg = format!("no matched to predicate: last offset: {}", offset);
          let ps = parse_state.add_offset(length);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), length, msg);
          ParsedResult::failed_with_uncommitted(pe)
        }
      }
      ParsedResult::Failure { error, is_committed } => ParsedResult::failed(error, is_committed),
    })
  }

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser.run(&parse_state) {
      ParsedResult::Success { value: a, length: n } => {
        let ps = parse_state.add_offset(n);
        f(a).run(&ps).with_committed_fallback(n != 0).with_add_length(n)
      }
      ParsedResult::Failure { error, is_committed } => ParsedResult::failed(error, is_committed),
    })
  }

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a,
    A: Clone + 'a,
    B: Clone + 'a, {
    Self::flat_map(parser, move |e| Self::successful(f(e)))
  }
}
