use crate::core::{ParseError, ParseResult, ParseState, Parser, ParserRunner, Parsers};
use crate::internal::ParsersImpl;

mod operator_parsers_impl;
mod conversion_parsers_impl;
mod element_parsers_impl;
mod elements_parsers;
mod repeat_parsers_impl;
mod skip_parser_impl;
mod lazy_parsers_impl;
mod offset_parsers_impl;
mod primitive_parsers_impl;

impl Parsers for ParsersImpl {
  type P<'p, I, A>
  where
    I: 'p,
  = Parser<'p, I, A>;

  fn parse<'a, 'b, I, A>(parser: &Self::P<'a, I, A>, input: &'b [I]) -> Result<A, ParseError<'a, I>>
  where
    'b: 'a, {
    let parse_state = ParseState::new(input, 0);
    parser.run(&parse_state).extract()
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

  fn failed<'a, I, A, F>(f: F) -> Self::P<'a, I, A>
  where
    F: Fn() -> ParseError<'a, I> + 'a,
    I: 'a,
    A: 'a, {
    Parser::new(move |_| ParseResult::Failure {
      get: f(),
      is_committed: false,
    })
  }

  fn flat_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Self::P<'a, I, B> + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser.run(&parse_state) {
      ParseResult::Success { get: a, length: n } => {
        let ps = parse_state.add_offset(n);
        f(a).run(&ps).map_err_is_committed_fallback(n != 0).with_add_length(n)
      }
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }

  fn map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> B + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { get: a, length } => ParseResult::Success { get: f(a), length },
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }
}
