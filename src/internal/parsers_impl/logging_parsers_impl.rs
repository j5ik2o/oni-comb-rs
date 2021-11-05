use crate::core::{ParseResult, Parser, ParserRunner};
use crate::extension::parsers::LoggingParsers;
use crate::internal::ParsersImpl;
use std::fmt::{Debug, Display};

impl LoggingParsers for ParsersImpl {
  fn logging_map<'a, I, A, B, F>(parser: Self::P<'a, I, A>, name: &'a str, f: F) -> Self::P<'a, I, A>
  where
    F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
    A: Debug + 'a,
    B: Display + 'a, {
    Parser::new(move |parse_state| {
      let ps = parser.run(parse_state);
      log::debug!("{} = {}", name, f(&ps));
      ps
    })
  }
}
