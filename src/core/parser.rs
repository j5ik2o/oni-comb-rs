use crate::core::{ParseResult, ParseState};
use std::rc::Rc;

mod add_parser;
mod basic_combinator;
mod bitor_parser;
mod conversion_combinator;
mod mul_parser;
mod not_parser;
mod offset_combinator;
mod parser_functor;
mod parser_monad;
mod parser_pure;
mod parser_runner;
mod repeat_combinator;
mod skip_combinator;
mod sub_parser;

type Parse<'a, I, A> = dyn Fn(Rc<ParseState<'a, I>>) -> ParseResult<'a, I, A> + 'a;

pub struct Parser<'a, I, A> {
  method: Box<Parse<'a, I, A>>,
}

impl<'a, I, A> Parser<'a, I, A> {
  pub fn new<F>(parse: F) -> Parser<'a, I, A>
  where
    F: Fn(Rc<ParseState<'a, I>>) -> ParseResult<'a, I, A> + 'a, {
    Parser {
      method: Box::new(parse),
    }
  }
}
