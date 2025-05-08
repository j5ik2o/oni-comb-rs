use crate::core::{CommittedStatus, ParseError, ParseResult, ParseState, Parser, ParserRunner, Parsers};

pub struct ParsersImpl;

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
    = Parser<'p, I, A>
  where
    I: 'p,
    A: 'p;

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
    Parser::new(move |_| ParseResult::successful(value.clone(), 0))
  }

  fn successful_lazy<'a, I, A, F>(value: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> A + 'a,
    A: 'a, {
    Parser::new(move |_| ParseResult::successful(value(), 0))
  }

  fn failed<'a, I, A>(value: ParseError<'a, I>, committed: CommittedStatus) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: 'a, {
    Parser::new(move |_| ParseResult::failed(value.clone(), committed.clone()))
  }

  fn failed_lazy<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> (ParseError<'a, I>, CommittedStatus) + 'a,
    I: 'a,
    A: 'a, {
    Parser::new(move |_| {
      let (pe, committed) = f();
      ParseResult::failed(pe, committed)
    })
  }

  fn filter<'a, I, A, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&A) -> bool + 'a + Clone,
    I: 'a + Clone,
    A: 'a + Clone, {
    Parser::new(move |state| match parser.run(state) {
      ParseResult::Success { value, length } => {
        if f(&value) {
          ParseResult::successful(value, length)
        } else {
          let offset = state.current_offset() + length;
          let msg = "filter: predicate returned false".to_string();
          let pe = ParseError::of_custom(offset, None, msg);
          ParseResult::failed(pe, CommittedStatus::Uncommitted)
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a,
    A: 'a,
    B: 'a, {
    let method = parser.method.clone();
    Parser::new(move |state| match method(state) {
      ParseResult::Success { value, length } => f(value)
        .run(&state.add_offset(length))
        .add_commit(length != 0)
        .advance_success(length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser.run(&parse_state) {
      ParseResult::Success { value, length } => ParseResult::successful(f(value), 0)
        .add_commit(length != 0)
        .advance_success(length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }
}
