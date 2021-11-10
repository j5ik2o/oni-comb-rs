use crate::core::{Parser, ParserRunner};
use crate::extension::parsers::CacheParsers;
use crate::internal::ParsersImpl;
use std::cell::RefCell;

use std::collections::HashMap;
use std::fmt::Debug;

impl CacheParsers for ParsersImpl {
  fn cache<'a, I, A>(parser: Self::P<'a, I, A>) -> Self::P<'a, I, A>
  where
    I: Clone + 'a,
    A: Clone + Debug + 'a, {
    let results = RefCell::new(HashMap::new());
    Parser::new(move |parser_state| {
      let key = format!("{:p}:{}:{:p}", parser_state, parser_state.last_offset().unwrap_or(0), &parser.method);
      let parse_result = results
        .borrow_mut()
        .entry(key)
        .or_insert_with(|| parser.run(parser_state))
        .clone();
      parse_result
    })
  }
}
