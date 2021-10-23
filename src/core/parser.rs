use std::rc::Rc;

use crate::core::{ParseResult, ParseState};

mod add_parser_impl;
mod basic_combinator_impl;
mod bitor_parser_impl;
mod conversion_combinator_impl;
mod mul_parser_impl;
mod not_parser_impl;
mod offset_combinator_impl;
mod parser_functor_impl;
mod parser_monad_impl;
mod parser_pure_impl;
mod parser_runner_impl;
mod repeat_combinator_impl;
mod skip_combinator_impl;
mod sub_parser_impl;

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
