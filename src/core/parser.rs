use std::rc::Rc;

use crate::core::{ParseResult, ParseState};
use crate::extension::{
  BasicCombinator, ConversionCombinator, ConversionCombinators, OffsetCombinator, OffsetCombinators, SkipCombinator,
  SkipCombinators,
};
use crate::extension::{BasicCombinators, RepeatCombinator, RepeatCombinators};
use crate::internal::ParsersImpl;
use crate::utils::RangeArgument;

mod add_parser;
mod bitor_parser;
mod mul_parser;
mod not_parser;
mod parser_functor;
mod parser_monad;
mod parser_pure;
mod parser_runner;
mod sub_parser;
mod basic_combinator;
mod conversion_combinator;
mod offset_combinator;
mod skip_combinator;
mod repeat_combinator;

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
