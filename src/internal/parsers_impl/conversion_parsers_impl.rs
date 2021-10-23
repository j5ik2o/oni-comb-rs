use crate::core::{ParseError, ParseResult, Parser, ParserRunner};
use crate::extension::parsers::ConversionParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl ConversionParsers for ParsersImpl {
  fn convert<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Result<B, E> + 'a,
    E: Debug,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParseResult::Success { get: a, length } => match f(a) {
        Ok(get) => ParseResult::Success { get: get, length },
        Err(err) => {
          let ps = parse_state.add_offset(0);
          let msg = format!("Conversion error: {:?}", err);
          let parser_error = ParseError::of_conversion(ps.input(), ps.last_offset().unwrap_or(0), msg);
          ParseResult::failed_with_un_commit(parser_error)
        }
      },
      ParseResult::Failure { get, is_committed } => ParseResult::failed(get, is_committed),
    })
  }
}
