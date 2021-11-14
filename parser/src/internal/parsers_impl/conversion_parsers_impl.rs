use crate::core::{ParsedError, ParsedResult, Parser, ParserRunner};
use crate::extension::parsers::ConversionParsers;
use crate::internal::ParsersImpl;
use std::fmt::Debug;

impl ConversionParsers for ParsersImpl {
  fn map_res<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Result<B, E> + 'a,
    E: Debug,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParsedResult::Success { value: a, length } => match f(a) {
        Ok(value) => ParsedResult::successful(value, length),
        Err(err) => {
          let ps = parse_state.add_offset(0);
          let msg = format!("Conversion error: {:?}", err);
          let parser_error = ParsedError::of_conversion(ps.input(), ps.last_offset().unwrap_or(0), 0, msg);
          ParsedResult::failed_with_uncommitted(parser_error)
        }
      },
      ParsedResult::Failure { error, is_committed } => ParsedResult::failed(error, is_committed),
    })
  }

  fn map_opt<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Option<B> + 'a,
    A: Debug + 'a,
    B: Debug + 'a, {
    Parser::new(move |parse_state| match parser.run(parse_state) {
      ParsedResult::Success { value: a, length } => match f(a) {
        Some(value) => ParsedResult::successful(value, length),
        None => {
          let ps = parse_state.add_offset(0);
          let msg = format!("Conversion error");
          let parser_error = ParsedError::of_conversion(ps.input(), ps.last_offset().unwrap_or(0), 0, msg);
          ParsedResult::failed_with_uncommitted(parser_error)
        }
      },
      ParsedResult::Failure { error, is_committed } => ParsedResult::failed(error, is_committed),
    })
  }
}
